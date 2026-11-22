use std::collections::{BTreeMap, VecDeque};
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::time::Instant;
use wasmtime::{Caller, Config, Engine, Error, Extern, ExternType, Linker, Module, Store, StoreLimits, StoreLimitsBuilder, Val, ValType};

const MEMORY_LIMIT: usize = 256 * 1024 * 1024;
const TRANSCRIPT_LIMIT: u64 = 8 * 1024 * 1024;
const DEFAULT_EXPORT_FUEL: u64 = 1_000_000_000;
const MONOTONIC: &str = "wasi:clocks/monotonic-clock@0.2.0::now";
const OPERATION: &str = "semio:framework/pure@1.0.0::operation-clock-us";

#[derive(Debug)]
enum Clock {
    Operation { binding: i64, pointer: i32, micros: u64 },
    Monotonic(i64),
}

struct Host {
    limits: StoreLimits,
    expected: VecDeque<Clock>,
    imports: BTreeMap<String, usize>,
}

fn clock(mut caller: Caller<'_, Host>, name: &str, args: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    *caller.data_mut().imports.entry(name.to_owned()).or_default() += 1;
    let expected = caller.data_mut().expected.pop_front();
    match (name, args, expected) {
        (OPERATION, [Val::I64(binding), Val::I32(pointer)], Some(Clock::Operation { binding: want_binding, pointer: want_pointer, micros }))
            if (*binding, *pointer) == (want_binding, want_pointer) => {
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(Error::msg("Guest has no exported memory"));
                };
                let mut response = [0; 16];
                response[0] = 1;
                response[8..].copy_from_slice(&micros.to_le_bytes());
                memory.write(&mut caller, *pointer as u32 as usize, &response)?;
            }
        (MONOTONIC, [], Some(Clock::Monotonic(nanos))) => results[0] = Val::I64(nanos),
        (_, _, expected) => return Err(Error::msg(format!("Import divergence: {name} {args:?}, expected {expected:?}"))),
    }
    Ok(())
}

fn hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value.as_bytes().chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn scalar(value: &Val) -> i64 {
    match value {
        Val::I32(value) => i64::from(*value),
        Val::I64(value) => *value,
        other => panic!("Unsupported scalar {other:?}"),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let component_path = args.next().expect("Component path");
    let transcript_path = args.next().expect("Transcript path");
    let max_fuel = args.next().map(|fuel| fuel.parse().unwrap()).unwrap_or(DEFAULT_EXPORT_FUEL);
    assert!(args.next().is_none());
    assert!(fs::metadata(&transcript_path).unwrap().len() <= TRANSCRIPT_LIMIT);
    let transcript = fs::read_to_string(transcript_path).unwrap();
    let mut lines = transcript.lines();
    assert_eq!(lines.next(), Some("owned-input-v1"));
    let module: Vec<_> = lines.next().unwrap().split(' ').collect();
    let ["M", offset, length, sha256] = module.as_slice() else { panic!("Missing Core module coordinates") };
    let offset: u64 = offset.parse().unwrap();
    let length: usize = length.parse().unwrap();
    assert!((8..128 * 1024 * 1024).contains(&length));
    let mut component = File::open(component_path).unwrap();
    assert!(offset.checked_add(length as u64).unwrap() <= component.metadata().unwrap().len());
    component.seek(SeekFrom::Start(offset)).unwrap();
    let mut bytes = vec![0; length];
    component.read_exact(&mut bytes).unwrap();

    let mut config = Config::new();
    config.consume_fuel(true);
    config.parallel_compilation(false);
    let engine = Engine::new(&config).unwrap();
    println!("[DEBUG] compiling Core {length} bytes from component {sha256}");
    let compile_started = Instant::now();
    let module = Module::new(&engine, bytes).unwrap();
    println!("[DEBUG] compiled Core in {} ms", compile_started.elapsed().as_millis());
    let mut linker = Linker::new(&engine);
    for import in module.imports() {
        let ExternType::Func(ty) = import.ty() else { panic!("Unsupported non-function import {}", import.name()) };
        let qualified = format!("{}::{}", import.module(), import.name());
        linker.func_new(import.module(), import.name(), ty, move |caller, args, results| clock(caller, &qualified, args, results)).unwrap();
    }
    let limits = StoreLimitsBuilder::new().memory_size(MEMORY_LIMIT).build();
    let mut store = Store::new(&engine, Host { limits, expected: VecDeque::new(), imports: BTreeMap::new() });
    store.limiter(|host| &mut host.limits);
    store.set_fuel(max_fuel).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();
    assert!(store.data().imports.is_empty());
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    println!("[DEBUG] instantiation fuel={} memoryBytes={}", max_fuel - store.get_fuel().unwrap(), memory.data_size(&store));

    let mut calls = 0;
    let mut input_count = 0;
    let mut input_fuel = 0;
    let mut writes = 0;
    let mut checked = 0;
    let mut totals = BTreeMap::<String, usize>::new();
    while let Some(line) = lines.next() {
        let words: Vec<_> = line.split(' ').collect();
        match words.as_slice() {
            ["W", at, bytes] => {
                let bytes = hex(bytes);
                assert_eq!(bytes.len(), 601);
                memory.write(&mut store, at.parse().unwrap(), &bytes).unwrap();
                writes += 1;
            }
            ["R", at, bytes] => {
                let expected = hex(bytes);
                let mut actual = vec![0; expected.len()];
                memory.read(&store, at.parse().unwrap(), &mut actual).unwrap();
                if let Some(index) = actual.iter().zip(&expected).position(|(a, b)| a != b) {
                    panic!("Receipt divergence after call {calls}: at={at}+{index} got={} expected={}", actual[index], expected[index]);
                }
                checked += 1;
            }
            ["C", name, expected, size, import_count, args @ ..] => {
                calls += 1;
                for _ in 0..import_count.parse::<usize>().unwrap() {
                    let words: Vec<_> = lines.next().unwrap().split(' ').collect();
                    let event = match words.as_slice() {
                        ["I", "operation-clock-us", binding, pointer, micros] => Clock::Operation {
                            binding: binding.parse().unwrap(), pointer: pointer.parse().unwrap(), micros: micros.parse().unwrap(),
                        },
                        ["I", "now", nanos] => Clock::Monotonic(nanos.parse().unwrap()),
                        _ => panic!("Invalid clock event {words:?}"),
                    };
                    store.data_mut().expected.push_back(event);
                }
                let export = instance.get_func(&mut store, &format!("semio_owned_{name}_v1")).unwrap();
                let signature = export.ty(&store);
                assert_eq!(signature.params().len(), args.len());
                let params: Vec<_> = signature.params().zip(args).map(|(ty, arg)| match ty {
                    ValType::I32 => Val::I32(arg.parse().unwrap()),
                    ValType::I64 => Val::I64(arg.parse().unwrap()),
                    _ => panic!("Unsupported parameter {ty:?}"),
                }).collect();
                let mut results = vec![Val::I32(0); signature.results().len()];
                store.set_fuel(max_fuel).unwrap();
                let started = Instant::now();
                let status = export.call(&mut store, &params, &mut results);
                let fuel = max_fuel - store.get_fuel().unwrap();
                let imports = std::mem::take(&mut store.data_mut().imports);
                let actual_size = memory.data_size(&store);
                println!("[DEBUG] call={calls} export={name} fuel={fuel} elapsedUs={} memoryBytes={actual_size} imports={imports:?}", started.elapsed().as_micros());
                status.unwrap_or_else(|error| panic!("Export {name} failed after {fuel} fuel: {error:#}"));
                assert!(store.data().expected.is_empty(), "Export {name} used fewer imports than expected: {:?}", store.data().expected.front());
                for (key, count) in imports {
                    *totals.entry(key).or_default() += count;
                }
                assert_eq!(actual_size, size.parse::<usize>().unwrap(), "Memory size after {name}");
                match (results.as_slice(), *expected) {
                    ([], "_") => (),
                    ([value], expected) => assert_eq!(scalar(value), expected.parse::<i64>().unwrap(), "Result of {name}"),
                    _ => panic!("Unexpected results of {name}: {results:?}"),
                }
                if *name == "admit_actor_input" {
                    input_count += 1;
                    input_fuel += fuel;
                }
            }
            _ => panic!("Invalid transcript record {words:?}"),
        }
    }
    assert_eq!((input_count, writes, checked), (2, 1, 19));
    println!("[DEBUG] complete calls={calls} inputCalls={input_count} inputFuel={input_fuel} exactReads={checked} imports={totals:?}");
}
