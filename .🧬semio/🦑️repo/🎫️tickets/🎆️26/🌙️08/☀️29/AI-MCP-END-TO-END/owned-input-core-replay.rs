use semio_plugin_interpreter::{CoreInstance, CoreModule, CoreStepOutcome, ExportKind, HostCall, StepControl, Value, ValueType};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use std::time::Instant;

const OPERATION: &str = "semio:framework/pure@1.0.0";
const MONOTONIC: &str = "wasi:clocks/monotonic-clock@0.2.0";
const STEP_FUEL: u64 = 1_000_000;
const DEFAULT_EXPORT_FUEL: u64 = 1_000_000_000;
#[cfg(feature = "testkit")]
const SAMPLE_STRIDE: u64 = 32_749;

#[cfg(feature = "testkit")]
fn report_samples(sites: BTreeMap<(u32, usize), u64>) {
    let mut functions = BTreeMap::<u32, u64>::new();
    for (&(function, _), &hits) in &sites {
        *functions.entry(function).or_default() += hits;
    }
    let total: u64 = sites.values().sum();
    println!("[DEBUG] sampling stride={SAMPLE_STRIDE} total={total} uniqueSites={}", sites.len());
    let mut sites: Vec<_> = sites.into_iter().collect();
    sites.sort_unstable_by_key(|&(key, hits)| (std::cmp::Reverse(hits), key));
    for ((function, pc), hits) in sites.into_iter().take(10) {
        println!("[DEBUG] sample site function={function} pc={pc} hits={hits}");
    }
    let mut functions: Vec<_> = functions.into_iter().collect();
    functions.sort_unstable_by_key(|&(function, hits)| (std::cmp::Reverse(hits), function));
    for (function, hits) in functions.into_iter().take(10) {
        println!("[DEBUG] sample function={function} hits={hits}");
    }
}

fn hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value.as_bytes().chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn resume(instance: &mut CoreInstance, memory: u32, call: HostCall, record: &str, export: &str) {
    let words: Vec<_> = record.split(' ').collect();
    let result = match words.as_slice() {
        ["I", "operation-clock-us", binding, pointer, micros] => {
            let binding = binding.parse::<i64>().unwrap();
            let pointer = pointer.parse::<i32>().unwrap();
            assert_eq!((call.module.as_str(), call.name.as_str()), (OPERATION, "operation-clock-us"), "{export}");
            assert_eq!(call.arguments, [Value::I64(binding), Value::I32(pointer)], "{export}");
            assert!(call.results.is_empty(), "{export}");
            let micros = micros.parse::<u64>().unwrap();
            let offset = pointer as u32 as usize;
            let mut response = [0; 16];
            response[0] = 1;
            response[8..].copy_from_slice(&micros.to_le_bytes());
            instance.memory_mut(memory).unwrap()[offset..offset + response.len()].copy_from_slice(&response);
            Vec::new()
        }
        ["I", "now", nanos] => {
            assert_eq!((call.module.as_str(), call.name.as_str()), (MONOTONIC, "now"), "{export}");
            assert!(call.arguments.is_empty(), "{export}");
            assert_eq!(call.results, [ValueType::I64], "{export}");
            vec![Value::I64(nanos.parse().unwrap())]
        }
        _ => panic!("Invalid clock record for {export}: {record}"),
    };
    instance.resume_host(call.id, Ok(result)).unwrap();
}

fn scalar(kind: &ValueType, text: &str) -> Value {
    match kind {
        ValueType::I32 => Value::I32(text.parse().unwrap()),
        ValueType::I64 => Value::I64(text.parse().unwrap()),
        _ => panic!("Unsupported scalar type {kind:?}"),
    }
}

fn main() {
    let mut argv = std::env::args().skip(1);
    let component_path = argv.next().expect("Component path");
    let transcript_path = argv.next().expect("Transcript path");
    let fuel_limit = argv.next().map(|value| value.parse().unwrap()).unwrap_or(DEFAULT_EXPORT_FUEL);
    assert!(argv.next().is_none());
    assert!(fs::metadata(&transcript_path).unwrap().len() <= 8 * 1024 * 1024);
    let transcript = fs::read_to_string(transcript_path).unwrap();
    let mut records = transcript.lines();
    assert_eq!(records.next(), Some("owned-input-v1"));
    let module: Vec<_> = records.next().unwrap().split(' ').collect();
    let ["M", offset, length, component_sha256] = module.as_slice() else { panic!("Missing Core module coordinates") };
    let offset = offset.parse::<u64>().unwrap();
    let length = length.parse::<usize>().unwrap();
    assert!((8..256 * 1024 * 1024).contains(&length));
    let mut component = File::open(component_path).unwrap();
    assert!(offset.checked_add(length as u64).unwrap() <= component.metadata().unwrap().len());
    component.seek(SeekFrom::Start(offset)).unwrap();
    let mut bytes = vec![0; length];
    component.read_exact(&mut bytes).unwrap();

    let started = Instant::now();
    println!("[DEBUG] parsing Core {length} bytes from component {component_sha256}");
    let module = Arc::new(CoreModule::parse(&bytes).unwrap());
    drop(bytes);
    let Some(ExportKind::Memory(memory)) = module.export("memory") else { panic!("Missing exported memory") };
    let memory = *memory;
    let mut instance = CoreInstance::instantiate(Arc::clone(&module)).unwrap();
    assert!(!instance.active(), "Unexpected start function");
    println!("[DEBUG] instantiated Core elapsedUs={} memoryBytes={}", started.elapsed().as_micros(), instance.memory(memory).unwrap().len());

    let (mut calls, mut writes, mut reads, mut input_reads) = (0, 0, 0, 0);
    let mut input_receipts = 0;
    let mut terminal_input = false;
    let mut totals = BTreeMap::<&str, (usize, u64, usize)>::new();
    let mut last_export = "";
    while let Some(record) = records.next() {
        let words: Vec<_> = record.split(' ').collect();
        match words.as_slice() {
            ["W", pointer, bytes] => {
                let offset = pointer.parse::<usize>().unwrap();
                let wire = hex(bytes);
                assert!((1..=1024).contains(&wire.len()));
                instance.memory_mut(memory).unwrap()[offset..offset + wire.len()].copy_from_slice(&wire);
                writes += 1;
            }
            ["R", pointer, bytes] => {
                let offset = pointer.parse::<usize>().unwrap();
                let expected = hex(bytes);
                let actual = &instance.memory(memory).unwrap()[offset..offset + expected.len()];
                if let Some(index) = actual.iter().zip(&expected).position(|(a, b)| a != b) {
                    panic!("Read divergence after call {calls} ({last_export}): {offset}+{index}, got {}, expected {}", actual[index], expected[index]);
                }
                if last_export == "admit_actor_input" {
                    if expected.starts_with(b"SRE1") {
                        input_reads += 1;
                    }
                    if expected.starts_with(b"SAC3") {
                        assert_eq!(expected[4], 4);
                        assert!(matches!(expected[5], 0 | 3));
                        assert!(!terminal_input, "Input retained a receipt after returning its wire");
                        terminal_input = expected[5] == 3;
                        input_receipts += 1;
                    }
                }
                reads += 1;
            }
            ["C", name, result, size, import_count, args @ ..] => {
                calls += 1;
                last_export = name;
                let export = format!("semio_owned_{name}_v1");
                let Some(ExportKind::Function(function)) = module.export(&export) else { panic!("Missing export {export}") };
                let signature = module.function_type(*function).unwrap();
                assert_eq!(signature.parameters.len(), args.len(), "{export}");
                let params = signature.parameters.iter().zip(args).map(|(kind, arg)| scalar(kind, arg)).collect();
                let expected = match signature.results.as_slice() {
                    [] if *result == "_" => Vec::new(),
                    [kind] => vec![scalar(kind, result)],
                    _ => panic!("Unexpected result signature for {export}"),
                };
                let imports = import_count.parse::<usize>().unwrap();
                instance.begin_export(&export, params).unwrap();
                let started = Instant::now();
                let (mut fuel, mut used_imports) = (0, 0);
                let mut prior_import_fuel = 0;
                let mut longest_import_gap = (0, 0);
                #[cfg(feature = "testkit")]
                let sample = *name == "admit_actor_input" && !totals.contains_key(name);
                #[cfg(feature = "testkit")]
                let mut sites = BTreeMap::new();
                #[cfg(feature = "testkit")]
                let mut next_sample = SAMPLE_STRIDE;
                let values = loop {
                    assert!(fuel < fuel_limit, "Call {calls} ({name}) exhausted {fuel} fuel after {used_imports}/{imports} imports");
                    let window = STEP_FUEL.min(fuel_limit - fuel);
                    #[cfg(feature = "testkit")]
                    let window = if sample {
                        if fuel == next_sample {
                            if let Some(site) = instance.testkit_scalar_site() {
                                *sites.entry(site).or_default() += 1;
                            }
                            next_sample += SAMPLE_STRIDE;
                        }
                        window.min(next_sample - fuel)
                    } else {
                        window
                    };
                    match instance.step(window, StepControl::default()) {
                        CoreStepOutcome::Yield { fuel_used } => fuel += fuel_used,
                        CoreStepOutcome::HostCall { fuel_used, call } => {
                            fuel += fuel_used;
                            assert!(used_imports < imports, "Unexpected import for {export} at fuel {fuel}: {call:?}");
                            resume(&mut instance, memory, call, records.next().unwrap(), &export);
                            used_imports += 1;
                            if *name == "admit_actor_input" {
                                let gap = fuel - prior_import_fuel;
                                longest_import_gap = longest_import_gap.max((gap, used_imports));
                                prior_import_fuel = fuel;
                                if used_imports <= 4 || used_imports.is_power_of_two() || used_imports == imports {
                                    println!("[DEBUG] Input clock prefix import={used_imports}/{imports} fuel={fuel} gap={gap}");
                                }
                            }
                        }
                        CoreStepOutcome::Complete { fuel_used, values } => {
                            fuel += fuel_used;
                            break values;
                        }
                        outcome => panic!("Call {calls} ({name}) stopped after {fuel} fuel and {used_imports}/{imports} imports: {outcome:?}"),
                    }
                };
                let memory_size = instance.memory(memory).unwrap().len();
                println!("[DEBUG] call={calls} export={name} fuel={fuel} elapsedUs={} memoryBytes={memory_size} imports={used_imports}", started.elapsed().as_micros());
                assert_eq!(values, expected, "{export}");
                if *name == "admit_actor_input" {
                    let (gap, index) = longest_import_gap.max((fuel - prior_import_fuel, used_imports + 1));
                    println!("[DEBUG] Input longest clock gap import={index} fuel={gap} finalGap={}", fuel - prior_import_fuel);
                }
                assert_eq!(used_imports, imports, "{export}");
                assert_eq!(memory_size, size.parse::<usize>().unwrap(), "{export}");
                #[cfg(feature = "testkit")]
                if sample {
                    report_samples(sites);
                }
                let total = totals.entry(name).or_default();
                total.0 += 1;
                total.1 += fuel;
                total.2 += used_imports;
            }
            _ => panic!("Invalid transcript record {record}"),
        }
    }
    assert_eq!(writes, 1);
    assert!(terminal_input, "Input never reached ready and returned its wire");
    assert!((1..=64).contains(&input_receipts));
    assert_eq!(input_reads, input_receipts);
    assert_eq!(totals["admit_actor_input"].0, input_receipts);
    for (export, (calls, fuel, imports)) in totals {
        println!("[DEBUG] total export={export} calls={calls} fuel={fuel} imports={imports}");
    }
    println!("[DEBUG] complete calls={calls} exactReads={reads} exactInputReceipts={input_reads}");
}
