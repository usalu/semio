import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";

const scratch = resolve(import.meta.dir, "🗑️generated");
const build = join(scratch, "native-mcp/debug/build");
const suffix = process.platform === "win32" ? ".exe" : "";
const args = process.argv.slice(2);
const prepare = args.includes("--audit-prepare");
const direct = prepare || args.includes("--audit-core");
const frames = args.includes("--trace-frames");
const optimized = args.includes("--optimized-core");
if ((frames || optimized) && !direct) throw Error("Core auditing mode required for frame tracing or an optimized interpreter");
const purpose = (prepare ? `mcp-prepare${frames ? "-frames" : ""}` : frames ? "mcp-core-frames" : direct ? "mcp-core-bootstrap" : "mcp-codec-bootstrap") + (optimized ? "-o3" : "");
const output = join(scratch, purpose + suffix);
const input = join(scratch, `${purpose}.rs`);
const paths = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(entry =>
  readdirSync(join(build, entry.name)).map(variant => join(entry.name, variant)).filter(variant => existsSync(join(build, variant, "out"))));
const interpreter = "semio_plugin_interpreter_runtime";
const fingerprintFile = (path: string, crate: string) => join(build, path, "fingerprint", `lib-${crate}`);
const packages = (crate: string) => paths.filter(path => path.startsWith(crate.replaceAll("_", "-") + sep));
const variants = packages(interpreter).filter(path => existsSync(fingerprintFile(path, interpreter)))
  .sort((a, b) => statSync(fingerprintFile(b, interpreter)).mtimeMs - statSync(fingerprintFile(a, interpreter)).mtimeMs);
if (!variants.length) throw Error("Missing linked interpreter runtime");
const fingerprint = readFileSync(fingerprintFile(variants[0], interpreter) + ".json", "utf8");
const crates = direct ? ["semio_plugin_interpreter"] : [interpreter, "semio_framework", "semio_framework_job", "semio_framework_value"];
const optimizedLibrary = join(scratch, "libsemio_plugin_interpreter.rlib");
if (optimized && !existsSync(optimizedLibrary)) throw Error("Missing prebuilt optimized production Core interpreter");
const dependencies = optimized ? ["--extern", `semio_plugin_interpreter=${optimizedLibrary}`] : crates.flatMap(crate => {
  const value = fingerprint.match(new RegExp(`"${crate}",false,(\\d+)`, "u"))?.[1];
  const identity = value && Buffer.from(BigInt(value).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
  const path = crate === interpreter ? variants[0] : packages(crate).find(candidate => {
    const recorded = fingerprintFile(candidate, crate);
    return existsSync(recorded) && readFileSync(recorded, "utf8").trim() === identity;
  });
  if (!path) throw Error(`Missing linked codec dependency ${crate}`);
  const library = join(path, "out", `lib${crate}-${path.split(/[\\/]/u).at(-1)}`);
  return ["--extern", `${crate}=${library}.rmeta`, "--extern", `${crate}=${library}.rlib`];
});
if (!args.length) throw Error("Pass --compile-only or one or more verified component paths");
if (!existsSync(output) || args.includes("--compile-only")) {
  console.log(`[DEBUG] selected interpreter runtime ${optimized ? optimizedLibrary : variants[0]}`);
  writeFileSync(input, direct ? `use semio_plugin_interpreter::{CoreStepOutcome, OwnedSemioArtifact, StepControl, ${prepare ? "CoreInstance, Value" : "OwnedSemioExport"}};
use std::{env, fs, path::Path, time::{Duration, Instant}};
${frames ? `struct Cursor<'a>(&'a [u8]);
impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> &'a [u8] { let (now, rest) = self.0.split_at(n); self.0 = rest; now }
    fn byte(&mut self) -> u8 { self.take(1)[0] }
    fn word(&mut self) -> u32 { u32::from_le_bytes(self.take(4).try_into().unwrap()) }
    fn size(&mut self) -> usize { usize::try_from(u64::from_le_bytes(self.take(8).try_into().unwrap())).unwrap() }
    fn sized_bytes(&mut self) { let size = self.size(); self.take(size); }
    fn option(&mut self, read: impl FnOnce(&mut Self)) { match self.byte() { 0 => {}, 1 => read(self), flag => panic!("unknown checkpoint option {flag}") } }
    fn list(&mut self, mut read: impl FnMut(&mut Self)) { for _ in 0..self.size() { read(self); } }
    fn value(&mut self) {
        match self.byte() {
            0 | 2 => { self.take(4); }, 1 | 3 => { self.take(8); },
            4 => self.option(|c| { c.take(4); }), 5 => self.option(|c| { c.take(8); }),
            tag => panic!("unknown checkpoint value {tag}"),
        }
    }
}
fn show_frames(checkpoint: &[u8]) {
    let mut cursor = Cursor(checkpoint);
    if cursor.0.starts_with(b"SEMIOACT") {
        assert_eq!(cursor.take(9), b"SEMIOACT\\x01");
        cursor.take(8);
        let size = cursor.size();
        assert_eq!(size, cursor.0.len());
    }
    assert_eq!(cursor.take(9), b"SEMIOWCP\\x01");
    cursor.take(16);
    cursor.list(Cursor::sized_bytes);
    cursor.list(|table| table.list(|element| element.option(|c| { c.take(8); })));
    cursor.list(Cursor::value);
    cursor.list(|segment| segment.option(Cursor::sized_bytes));
    cursor.list(|segment| segment.option(|elements| elements.list(|entry| entry.option(|c| { c.take(4); }))));
    assert_eq!(cursor.byte(), 1, "active Core machine required");
    cursor.list(Cursor::value);
    cursor.list(|frame| {
        let function = frame.word(); let pc = frame.size(); frame.take(8);
        frame.list(Cursor::value);
        frame.list(|control| { control.take(25); control.sized_bytes(); control.sized_bytes(); });
        println!("[DEBUG] core frame function={function} pc={pc}");
    });
    assert_eq!(cursor.byte(), 1, "pending actual import required");
    cursor.take(8);
    cursor.sized_bytes(); cursor.sized_bytes(); cursor.list(Cursor::value); cursor.sized_bytes(); cursor.take(8);
    assert!(cursor.0.is_empty(), "machine checkpoint has trailing bytes");
}
` : ""}
fn main() {
    for path in env::args_os().skip(1) {
        let path = Path::new(&path);
        let started = Instant::now();
        let bytes = fs::read(path).expect("read model");
        let artifact = OwnedSemioArtifact::parse(&bytes).expect("parse exact model");
        let mut actor = ${prepare ? "CoreInstance::instantiate(std::sync::Arc::clone(artifact.original_actor_module()))" : "artifact.instantiate()"}.expect("instantiate exact model");
        eprintln!("[DEBUG] direct core artifact file={} bytes={} elapsed_ms={}", path.display(), bytes.len(), started.elapsed().as_millis());
        drop(bytes);
        assert!(!actor.${prepare ? "active" : "startup_active"}(), "unexpected implicit guest start");
        ${prepare ? 'actor.begin_export("semio_owned_prepare_actor_v1", Vec::new()).expect("start prepared actor export");' : 'actor.begin(OwnedSemioExport::Describe, Vec::new()).expect("start Describe export");'}
        let began = Instant::now();
        let mut fuel = 0u64;
        let mut next = 100_000_000;
        while fuel < 12_000_000_000 && began.elapsed() < Duration::from_secs(480) {
            let elapsed = began.elapsed().as_millis();
            match actor.step(2_000_000, StepControl::default()) {
                CoreStepOutcome::Yield { fuel_used } if fuel_used > 0 => { fuel += fuel_used; }
                CoreStepOutcome::HostCall { fuel_used, call } => {
                    println!("[DEBUG] first import module={:?} name={:?} fuel={} elapsed_ms={elapsed}", call.module, call.name, fuel + fuel_used);
                    ${frames ? "show_frames(&actor.checkpoint());" : ""}
                    break;
                }
                CoreStepOutcome::Complete { fuel_used, values } => {
                    ${prepare ? 'assert_eq!(values, vec![Value::I32(1)], "guest preparation must acknowledge successful assembly");' : ""}
                    println!("[DEBUG] ${prepare ? "actor preparation" : "describe"} completed without an import fuel={} elapsed_ms={elapsed} result_count={}", fuel + fuel_used, values.len());
                    break;
                }
                outcome => panic!("direct Describe stopped after {fuel} fuel: {outcome:?}"),
            }
            if fuel >= next { eprintln!("[DEBUG] direct core fuel={fuel} elapsed_ms={elapsed}"); next += 100_000_000; }
        }
        if fuel >= 12_000_000_000 || began.elapsed() >= Duration::from_secs(480) {
            println!("[DEBUG] no import observed inside bounds fuel={fuel} elapsed_ms={}", began.elapsed().as_millis());
        }
    }
}
` : `use semio_framework::kernel::Budget;
use semio_framework_job::default_now_us;
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_interpreter_runtime::OwnedRuntime;
use std::{env, fs, path::Path, time::Instant};
fn main() {
    for path in env::args_os().skip(1) {
        let path = Path::new(&path);
        let started = Instant::now();
        let runtime = OwnedRuntime::new(default_now_us);
        let bytes = fs::read(path).expect("read model");
        let compiled = runtime.compile_component(&bytes).expect("compile model");
        eprintln!("[DEBUG] bootstrap compile file={} bytes={} elapsed_ms={}", path.display(), bytes.len(), started.elapsed().as_millis());
        drop(bytes);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let budget = Budget::original(grant, 20_000_000_000, default_now_us().expect("original clock"), 600_000, 0, 0, 0);
        let began = Instant::now();
        let mut next = 500_000_000;
        let mut progress = |fuel, elapsed: std::time::Duration| {
            if fuel >= next {
                eprintln!("[DEBUG] bootstrap fuel={} elapsed_ms={}", fuel, elapsed.as_millis());
                next += 500_000_000;
            }
        };
        let (_instance, fuel) = runtime.codec_instance(&compiled, budget, &mut progress, None).expect("assemble codec origin");
        println!("[DEBUG] bootstrap file={} fuel={} elapsed_ms={}", path.display(), fuel, began.elapsed().as_millis());
    }
}
`);
  const search = paths.flatMap(path => {
    const out = join(path, "out");
    return ["-L", `dependency=${out}`, ...(readdirSync(join(build, out)).some(file => /\.(a|lib)$/u.test(file)) ? ["-L", `native=${out}`] : [])];
  });
  const list = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
  if (list.exitCode) process.exit(list.exitCode);
  const rustup = JSON.parse(list.stdout.toString()).find((item: { id: string }) => item.id === "rustup")?.command;
  const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
  if (!rustup || !toolchain) throw Error("Pinned Rust compiler unavailable");
  const compiled = Bun.spawnSync([rustup, "run", toolchain, "rustc", "--edition=2024", "-Cdebuginfo=0", "-Cstrip=symbols", ...(optimized ? ["-Copt-level=3"] : []), input,
    "-o", output, ...dependencies, ...search], { cwd: build, stdout: "inherit", stderr: "inherit" });
  if (compiled.exitCode) process.exit(compiled.exitCode);
}
const models = args.filter(arg => !["--compile-only", "--audit-core", "--audit-prepare", "--trace-frames", "--optimized-core"].includes(arg)).map(arg => resolve(arg));
if (models.length) {
  if (models.some(path => !path.startsWith(scratch + sep))) throw Error("Expected verified ticket component paths");
  const measured = Bun.spawn([output, ...models], { stdout: "inherit", stderr: "inherit" });
  for (const signal of ["SIGINT", "SIGTERM"] as const) process.once(signal, () => measured.kill(signal));
  process.exitCode = await measured.exited;
}
