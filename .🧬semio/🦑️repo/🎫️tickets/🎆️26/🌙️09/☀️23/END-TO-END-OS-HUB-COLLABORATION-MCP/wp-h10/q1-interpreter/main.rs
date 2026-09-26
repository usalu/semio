//! 🔬️ H10 scratch probe (ticket 26/09/23 session 12): drives the tree's owned interpreter directly on a real catalog
//! component — parse time, instantiate time, and one owned codec call's fuel, wall time and output digest.
//! usage: h10-owned-probe parse <component.wasm> | codec <component.wasm> <pack-schema-hash|genesis> <schema> [document-id] [repeat]
#![allow(dead_code)]
#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs"]
mod interpreter;

use interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};
use std::time::Instant;

fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3))
}

fn write(instance: &mut OwnedSemioInstance, pointer: i32, bytes: &[u8]) {
    let memory = instance.memory_mut().expect("memory");
    let start = pointer as u32 as usize;
    memory[start..start + bytes.len()].copy_from_slice(bytes);
}

struct Host { context: i32, next_resource: i32, clock: u64 }

fn reply(host: &mut Host, instance: &mut OwnedSemioInstance, call: &HostCall) -> Vec<Value> {
    let arg = |index: usize| match call.arguments.get(index) { Some(Value::I32(value)) => *value, other => panic!("argument {index} of {}::{} is {other:?}", call.module, call.name) };
    match (call.module.as_str(), call.name.as_str()) {
        ("semio:framework/pure@1.0.0", "now-ms") => vec![Value::I64(1_790_000_000_000)],
        ("semio:framework/pure@1.0.0", "log") | ("semio:framework/pure@1.0.0", "trace-span") => vec![],
        ("$root", "[context-get-0]") => vec![Value::I32(host.context)],
        ("$root", "[context-set-0]") => { host.context = arg(0); vec![] }
        ("$root", "[waitable-set-new]") | ("wasi:io/streams@0.2.0", "[method]output-stream.subscribe") | ("wasi:clocks/monotonic-clock@0.2.0", "subscribe-duration") | ("wasi:cli/stdin@0.2.0", "get-stdin") | ("wasi:cli/stdout@0.2.0", "get-stdout") | ("wasi:cli/stderr@0.2.0", "get-stderr") => {
            let resource = host.next_resource; host.next_resource += 1; vec![Value::I32(resource)]
        }
        ("$root", "[waitable-set-poll]") => vec![Value::I32(0)],
        ("$root", "[waitable-join]") | ("$root", "[waitable-set-drop]") | ("[export]$root", "[task-cancel]") | ("wasi:io/poll@0.2.0", "[method]pollable.block") | ("wasi:io/error@0.2.0", "[resource-drop]error") | ("wasi:io/poll@0.2.0", "[resource-drop]pollable") | ("wasi:io/streams@0.2.0", "[resource-drop]input-stream") | ("wasi:io/streams@0.2.0", "[resource-drop]output-stream") => vec![],
        ("wasi:clocks/monotonic-clock@0.2.0", "now") => { host.clock += 1_000; vec![Value::I64(host.clock as i64)] }
        ("wasi:random/insecure-seed@0.2.9", "insecure-seed") => { write(instance, arg(0), &[7u8; 16]); vec![] }
        ("wasi:random/random@0.2.9", "get-random-u64") => vec![Value::I64(0x5eed_5eed_5eed_5eed)],
        ("wasi:cli/environment@0.2.0", "get-environment") => { write(instance, arg(0), &[0; 8]); vec![] }
        ("wasi:clocks/wall-clock@0.2.0", "now") => { let mut datetime = [0u8; 16]; datetime[..8].copy_from_slice(&1_790_000_000u64.to_le_bytes()); write(instance, arg(0), &datetime); vec![] }
        ("wasi:cli/terminal-stdin@0.2.0", "get-terminal-stdin") | ("wasi:cli/terminal-stdout@0.2.0", "get-terminal-stdout") | ("wasi:cli/terminal-stderr@0.2.0", "get-terminal-stderr") => { write(instance, arg(0), &[0; 8]); vec![] }
        ("wasi:io/poll@0.2.0", "poll") => { write(instance, arg(2), &[0; 8]); vec![] }
        ("wasi:io/streams@0.2.0", "[method]output-stream.check-write") => { let pointer = arg(1); write(instance, pointer, &[0; 16]); write(instance, pointer.wrapping_add(8), &65_536u64.to_le_bytes()); vec![] }
        ("wasi:io/streams@0.2.0", "[method]output-stream.write") => { write(instance, arg(3), &[0; 16]); vec![] }
        ("wasi:io/streams@0.2.0", "[method]output-stream.blocking-flush") => { write(instance, arg(1), &[0; 16]); vec![] }
        _ => panic!("host import {}::{} is unavailable", call.module, call.name),
    }
}

fn run(instance: &mut OwnedSemioInstance, host: &mut Host, fuel: &mut u64) -> Vec<Value> {
    loop {
        match instance.step(4_096, StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } => *fuel += fuel_used,
            CoreStepOutcome::HostCall { fuel_used, call } => { *fuel += fuel_used; let values = reply(host, instance, &call); instance.resume_host(call.id, Ok(values)).expect("resume host"); }
            CoreStepOutcome::Complete { fuel_used, values } => { *fuel += fuel_used; return values; }
            CoreStepOutcome::Cancelled { .. } => panic!("cancelled"),
            CoreStepOutcome::Fault { fuel_used, error } => panic!("fault after {} fuel: {error}", *fuel + fuel_used),
        }
    }
}

fn escape(text: &str) -> String { text.replace('\\', "\\\\").replace('"', "\\\"") }

fn codec(artifact: &OwnedSemioArtifact, export: OwnedSemioExport, schema: &str, document: &str) -> (u64, Vec<u8>, f64) {
    let mut instance = artifact.instantiate().expect("instantiate");
    let mut host = Host { context: 0, next_resource: 1, clock: 0 };
    codec_on(&mut instance, &mut host, export, schema, document)
}

fn codec_on(instance: &mut OwnedSemioInstance, host: &mut Host, export: OwnedSemioExport, schema: &str, document: &str) -> (u64, Vec<u8>, f64) {
    let started = Instant::now();
    let (instance, mut host) = (instance, host);
    let mut fuel = 0;
    let input = format!("{{\"artifact_schema\":\"{}\",\"document_id\":\"{}\",\"pack\":[],\"spr\":[],\"ops\":[]}}", escape(schema), escape(document)).into_bytes();
    instance.begin(OwnedSemioExport::Allocate, vec![Value::I32(input.len() as i32)]).expect("begin allocate");
    let [Value::I32(pointer)] = run(instance, &mut host, &mut fuel)[..] else { panic!("allocate result") };
    write(instance, pointer, &input);
    instance.begin(export, vec![Value::I32(pointer), Value::I32(input.len() as i32)]).expect("begin export");
    let values = run(instance, &mut host, &mut fuel);
    let output = instance.read_bytes_result(&values, 256 << 20).expect("output");
    (fuel, output, started.elapsed().as_secs_f64() * 1000.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = std::fs::read(&args[2]).expect("read component");
    let started = Instant::now();
    let artifact = OwnedSemioArtifact::parse(&bytes).expect("parse");
    let parse_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    drop(artifact.instantiate().expect("instantiate"));
    let instantiate_ms = started.elapsed().as_secs_f64() * 1000.0;
    println!("component bytes={} parseMs={parse_ms:.1} instantiateMs={instantiate_ms:.1}", bytes.len());
    if args[1] == "warm" {
        let export = match args[3].as_str() { "pack-schema-hash" => OwnedSemioExport::PackSchemaHash, "genesis" => OwnedSemioExport::Genesis, other => panic!("unknown op {other}") };
        let mut instance = artifact.instantiate().expect("instantiate");
        let mut host = Host { context: 0, next_resource: 1, clock: 0 };
        for (round, schema) in args[4..].iter().enumerate() {
            let (fuel, output, ms) = codec_on(&mut instance, &mut host, export, schema, "doc-h10-probe");
            println!("warm round={round} op={} schema={schema} fuel={fuel} ms={ms:.1} outputBytes={} outputFnv={:016x}", args[3], output.len(), fnv(&output));
        }
    }
    if args[1] == "codec" {
        let export = match args[3].as_str() { "pack-schema-hash" => OwnedSemioExport::PackSchemaHash, "genesis" => OwnedSemioExport::Genesis, other => panic!("unknown op {other}") };
        let document = args.get(5).map_or("", String::as_str);
        let repeat: usize = args.get(6).map_or(1, |value| value.parse().unwrap());
        for round in 0..repeat {
            let (fuel, output, ms) = codec(&artifact, export, &args[4], document);
            let head: String = output.iter().take(48).map(|byte| if byte.is_ascii_graphic() { *byte as char } else { '.' }).collect();
            let text = String::from_utf8_lossy(&output);
            let ok_hex: String = text.strip_prefix("{\"Ok\":[").and_then(|rest| rest.strip_suffix("]}")).map(|list| list.split(',').filter_map(|byte| byte.trim().parse::<u8>().ok()).map(|byte| format!("{byte:02x}")).collect()).unwrap_or_default();
            println!("round={round} op={} schema={} fuel={fuel} ms={ms:.1} MIPS={:.1} outputBytes={} outputFnv={:016x} okHex={ok_hex} head={head}", args[3], args[4], fuel as f64 / ms / 1000.0, output.len(), fnv(&output));
        }
    }
}
