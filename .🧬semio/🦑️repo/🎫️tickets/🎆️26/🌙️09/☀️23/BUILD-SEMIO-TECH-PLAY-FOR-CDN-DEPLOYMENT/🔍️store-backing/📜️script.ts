import Ajv from "ajv";
import { strict as assert } from "node:assert";
import { readFileSync, existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

function oracle(): void {
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root);
    root = parent;
  }
  const path = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/📦️backing/🧫️fixtures");
  const fixture = JSON.parse(readFileSync(join(path, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(path, "🧬️schema/🔣️.json"), "utf8")));
  assert.ok(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(validate({ ...fixture, maximumAdmissionBytes: 262145 }), false);
  for (const extent of fixture.emptyVectorExtents) {
    const owner = Buffer.allocUnsafeSlow(extent);
    assert.equal(owner.buffer.byteLength, extent);
    assert.ok(extent <= fixture.maximumAdmissionBytes);
    const denied = extent - fixture.undergrantOffset;
    assert.ok(denied < owner.buffer.byteLength);
    assert.equal(fixture.deniedReleaseBytes, 0);
    console.log("[DEBUG] Node original resident backing=" + extent + " one-below denied; exact whole receipt=" + owner.buffer.byteLength);
  }
  assert.equal(fixture.retainedSlots, 1024);
  assert.equal(fixture.maximumItems, 1);
  assert.equal(fixture.registryFinalDropBytes, 0);
  assert.equal(fixture.catalogTerminalDropBytes, 0);
  assert.equal(validate({ ...fixture, catalogTerminalDropBytes: 1 }), false);
  assert.deepEqual(fixture.registryLockStates, ["available", "busy", "poisoned"]);
  assert.equal(fixture.registryExternalAliases, 1);
}

function stringOwner(): void {
  oracle();
  let root = import.meta.dirname;
  while (!existsSync(join(root, "nx.json"))) root = dirname(root);
  const store = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
  const fixture = JSON.parse(readFileSync(join(store, "♻️retirement/📦️backing/🧫️fixtures/🔣️.json"), "utf8"));
  const source = readFileSync(join(store, "🦀️.rs"), "utf8");
  const start = source.indexOf("struct ArtifactStoreStringRetirement {");
  const end = source.indexOf("struct ArtifactStoreStringVectorRetirement {", start);
  assert.ok(start >= 0 && end > start);
  const ticket = dirname(import.meta.dirname);
  const generated = join(ticket, "🗑️generated", "string-owner");
  mkdirSync(generated, { recursive: true });
  const harness = `use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
static ACTIVE:AtomicBool=AtomicBool::new(false);
static BIRTH:AtomicUsize=AtomicUsize::new(0);
static FREE:AtomicUsize=AtomicUsize::new(0);
struct Observer;
unsafe impl GlobalAlloc for Observer {
 unsafe fn alloc(&self,layout:Layout)->*mut u8 {let ptr=unsafe{System.alloc(layout)};if ACTIVE.load(Ordering::Relaxed){BIRTH.fetch_add(layout.size(),Ordering::Relaxed);}ptr}
 unsafe fn dealloc(&self,ptr:*mut u8,layout:Layout){if ACTIVE.load(Ordering::Relaxed){FREE.fetch_add(layout.size(),Ordering::Relaxed);}unsafe{System.dealloc(ptr,layout)}}
}
#[global_allocator] static ALLOCATOR:Observer=Observer;
fn observe<T>(f:impl FnOnce()->T)->(T,usize,usize){BIRTH.store(0,Ordering::Relaxed);FREE.store(0,Ordering::Relaxed);ACTIVE.store(true,Ordering::Relaxed);let result=f();ACTIVE.store(false,Ordering::Relaxed);(result,BIRTH.load(Ordering::Relaxed),FREE.load(Ordering::Relaxed))}
mod semio_framework_value {#[derive(Debug)] pub struct ValueError;}
#[derive(Debug,PartialEq,Eq)] enum SnapshotRetirementStep {Pending{released_items:usize,released_bytes:usize},Complete,Blocked}
trait ErasedSnapshotRetirement {fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,semio_framework_value::ValueError>;fn terminal_is_empty(&self)->bool;fn next_close_byte_demand(&self)->usize{1}}
${source.slice(start, end)}
fn main(){for extent in [${fixture.emptyVectorExtents.join(",")}] {
 let mut owner=ArtifactStoreStringRetirement::new(String::from_utf8(vec![b'a';extent]).unwrap());
 let (step,birth,free)=observe(||owner.close_step(1,extent-1).unwrap());
 eprintln!("[DEBUG] exact production String extent={} undergrant={} query={} step={:?} SystemBirth={} SystemFree={}",extent,extent-1,owner.next_close_byte_demand(),step,birth,free);
 assert_eq!(step,SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});assert_eq!((birth,free),(0,0));assert_eq!(owner.next_close_byte_demand(),extent);
 let (step,birth,free)=observe(||owner.close_step(1,extent).unwrap());
 assert_eq!(step,SnapshotRetirementStep::Pending{released_items:1,released_bytes:extent});assert_eq!((birth,free),(0,extent));assert!(owner.terminal_is_empty());
 eprintln!("[DEBUG] exact production String whole original extent={} same-turn SystemFree={} receipt exact; terminalDrop0",extent,free);
 let (_,birth,free)=observe(||drop(owner));assert_eq!((birth,free),(0,0));
}}
`;
  const input = join(generated, "string-owner.rs");
  const output = join(generated, process.platform === "win32" ? "string-owner.exe" : "string-owner");
  writeFileSync(input, harness);
  const compiler = spawnSync("rustc", ["--edition=2024", input, "-o", output], { encoding: "utf8" });
  process.stdout.write(compiler.stdout ?? "");
  process.stderr.write(compiler.stderr ?? "");
  assert.equal(compiler.status, 0, "compile exact production owner with neutral observation interface");
  const runtime = spawnSync(output, [], { encoding: "utf8" });
  process.stdout.write(runtime.stdout ?? "");
  process.stderr.write(runtime.stderr ?? "");
  assert.equal(runtime.status, 0, "exact production owner must satisfy independent whole-allocation/System witness");
  for (const name of ["ArtifactStoreStringVectorRetirement", "ArtifactStoreRevisionAccumulatorRetirement", "ArtifactStoreCursorRetirement"]) {
    const begin = source.indexOf(`impl ErasedSnapshotRetirement for ${name} {`);
    const end = source.indexOf(`impl Drop for ${name} {`, begin);
    const body = source.slice(begin, end);
    assert.ok(body.includes("fn next_close_byte_demand(&self) -> usize"), `${name} must expose its retained child's whole physical demand`);
    assert.ok(body.includes("self.active.as_ref()") && body.includes("ErasedSnapshotRetirement::next_close_byte_demand"));
  }
}

if (import.meta.main) {
  const command = process.argv[2] ?? "oracle";
  assert.ok(command === "oracle" || command === "string-owner");
  if (command === "string-owner") stringOwner();
  else oracle();
}
