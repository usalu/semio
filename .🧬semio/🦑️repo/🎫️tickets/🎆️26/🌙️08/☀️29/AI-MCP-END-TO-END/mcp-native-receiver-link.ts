import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const generated = resolve(import.meta.dirname, "🗑️generated");
const build = join(generated, "native-host", "debug", "build");
const originalTest = "semio-plugin-actor-input/16449bd215be69a1/fingerprint/test-lib-semio_plugin_actor_input.json";
const library = new Map<string, { directory: string; fingerprint: string; artifact: string }>();
const native = new Set<string>();
for (const packageName of readdirSync(build)) {
  const root = join(build, packageName);
  for (const variant of readdirSync(root, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
    const unit = join(root, variant.name);
    const directory = join(unit, "out");
    const fingerprintDirectory = join(unit, "fingerprint");
    if (!existsSync(directory) || !existsSync(fingerprintDirectory)) continue;
    const files = readdirSync(directory);
    if (files.some(file => /\.(a|lib)$/u.test(file))) native.add(directory);
    for (const file of files) {
      const match = /^lib(.+)-[a-f0-9]+\.(rlib|dylib|so|dll)$/u.exec(file);
      if (!match) continue;
      const crate = match[1];
      const fingerprint = join(fingerprintDirectory, `lib-${crate}`);
      if (!existsSync(fingerprint)) continue;
      library.set(readFileSync(fingerprint, "utf8").trim(), { directory, fingerprint, artifact: join(directory, file) });
    }
  }
}
function dependencies(fingerprint: string) {
  const entries: { name: string; directory: string; fingerprint: string; artifact: string }[] = [];
  for (const [, name, number] of readFileSync(fingerprint, "utf8").matchAll(/"([^"]+)",false,(\d+)\]/gu)) {
    const id = Buffer.alloc(8);
    id.writeBigUInt64LE(BigInt(number));
    const row = library.get(id.toString("hex"));
    if (row) entries.push({ name, ...row });
    else if (name !== "build_script_build") throw Error(`Exact compiled dependency missing for ${name} in ${fingerprint}`);
  }
  return entries;
}
const direct = dependencies(join(build, originalTest));
const visited = new Set();
const graph = [];
const search = new Set();
const pending = [...direct];
while (pending.length) {
  const row = pending.pop()!;
  if (visited.has(row.fingerprint)) continue;
  visited.add(row.fingerprint);
  graph.push(row);
  search.add(row.directory);
  pending.push(...dependencies(`${row.fingerprint}.json`));
}
const manifest = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/📦️packages/🦀️rust/Cargo.toml");
let source = resolve(dirname(manifest), "../../🦀️.rs");
const tools = Bun.spawnSync(["repo", "tools", "list", "--json"]);
if (tools.exitCode) throw Error("Pinned tool registry is unavailable");
const rustup = JSON.parse(tools.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !channel) throw Error("Pinned Rust is unavailable");
const optLevel = process.env.SEMIO_NATIVE_RECEIVER_OPT_LEVEL ?? "0";
const replacementDirectory = join(generated, "receiver-link");
mkdirSync(replacementDirectory, { recursive: true });
if (process.env.SEMIO_NATIVE_RECEIVER_RETAIN_DISPATCH === "1" || process.env.SEMIO_NATIVE_RECEIVER_SMALL_TRACE === "1" || process.env.SEMIO_NATIVE_RECEIVER_SMALL_FUNDED === "1") {
  const testModule = resolve(dirname(source), "🧪️tests/🦀️.rs");
  const scratch = join(replacementDirectory, "actor-input-test.rs");
  let tests = readFileSync(testModule, "utf8")
    .replace(/include_str!\("([^"]+)"/gu, (_, path) => `include_str!(${JSON.stringify(resolve(dirname(testModule), path))}`);
  if (process.env.SEMIO_NATIVE_RECEIVER_RETAIN_DISPATCH === "1") {
    tests = tests.replace(/(let mut caller\s*=\s*OriginalJobFixtureCaller::admit\([^;]+;)(?!\s*caller\.retain_dispatch\(\))/gu, "$1caller.retain_dispatch().unwrap();");
  }
  if (process.env.SEMIO_NATIVE_RECEIVER_SMALL_TRACE === "1") {
    const debug = `if step > 0 && step & (step - 1) == 0 {eprintln!("[DEBUG] Actor Input: step {step}, decoder={:?}, pending={:?}, demand={:?}, capture={}, bound={}, outcome={}, returning={}, progress={:?}, stock={:?}",receiver.decoder.receiver().map(|decoder|decoder.phase()),receiver.decoder.receiver().map(|decoder|decoder.pending()),receiver.decoder.receiver().map(|decoder|decoder.demands(&wire,131072,u64::MAX)),receiver.capture.is_admitted(),receiver.binding.is_some(),receiver.outcome().is_some(),receiver.returning,caller.receipt(),caller.remaining_grant());}`;
    tests = tests.replace(/assert!\(step\s*<\s*99_999\)/gu, `${debug}assert!(step<99_999)`);
    tests = tests.replace("            admission == InputAdmission::Ready", `let step = *step;${debug}admission == InputAdmission::Ready`);
  }
  if (process.env.SEMIO_NATIVE_RECEIVER_SMALL_FUNDED === "1") {
    const begin = tests.indexOf("fn borrowed_actor_input_keeps_identity");
    const end = tests.indexOf("fn input_failure_projection", begin);
    const small = tests.slice(begin, end).replaceAll("receiver.advance(Some(&wire),cx)", "receiver.advance((!receiver.returned).then_some(wire.as_slice()),cx)")
      .replaceAll("if receiver.input_returned(){None}else{Some(wire.as_slice())}", "(!receiver.returned).then_some(wire.as_slice())")
      .replaceAll("caller.retain_dispatch().unwrap();", `caller.retain_dispatch().unwrap();
      let table=OriginalJobFixtureCaller::table_birth();
      eprintln!("[DEBUG] original return table standalone floor {table:?}");
      let short=RetainedCloneGrant{maximum_items:table.maximum_items-1,..table};
      for _ in 0..4 {assert!(!funded(&mut caller,short,|cx|cx.admit_original_return_table().map_err(Into::into)));}
      assert_eq!(caller.receipt(), Default::default());
      caller.admit_return_table();`);
    tests = tests.slice(0, begin) + small + tests.slice(end);
  }
  writeFileSync(scratch, tests);
  const root = readFileSync(source, "utf8").replace("🧪️tests/🦀️.rs", scratch);
  source = join(replacementDirectory, "actor-input-root.rs");
  writeFileSync(source, root);
}
const replacementManifests: Record<string, string> = {
  semio_framework_value_native_receiving_original: "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/📦️packages/🦀️rust/Cargo.toml",
  semio_framework_pack_json: "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml",
  semio_framework_job_native_decode: "🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/📦️packages/🦀️rust/Cargo.toml",
  semio_framework_pack_json_output_receiving: "🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🫴️unique/📦️packages/🦀️rust/Cargo.toml",
};
const replacements = new Map<string, string>();
const addExterns = (args: string[], rows: ReturnType<typeof dependencies>) => {
  for (const row of rows) {
    const artifact = replacements.get(row.fingerprint) ?? row.artifact;
    const metadata = artifact.replace(/\.rlib$/u, ".rmeta");
    if (metadata !== artifact && existsSync(metadata)) args.push("--extern", `${row.name}=${metadata}`);
    args.push("--extern", `${row.name}=${artifact}`);
  }
  for (const directory of search) args.push("-L", `dependency=${directory}`);
  for (const directory of native) args.push("-L", `native=${directory}`);
};
const output = direct.find(row => row.name === "semio_framework_pack_json_output_receiving")!;
const outputDependencies = dependencies(`${output.fingerprint}.json`);
const original = outputDependencies.find(row => row.name === "semio_framework_value_native_receiving_original")!;
const json = outputDependencies.find(row => row.name === "semio_framework_pack_json")!;
const job = outputDependencies.find(row => row.name === "semio_framework_job_native_decode")!;
const overrides = process.env.SEMIO_NATIVE_RECEIVER_OLD_DEPS === "1" ? [] : process.env.SEMIO_NATIVE_RECEIVER_ORIGINAL === "1" ? [original, json, job, output] : [job, output];
for (const row of overrides) {
  const manifest = resolve(replacementManifests[row.name]);
  const libPath = readFileSync(manifest, "utf8").match(/^path\s*=\s*"([^"]+)"/mu)?.[1];
  if (!libPath) throw Error(`No library root in ${manifest}`);
  let crateSource = resolve(dirname(manifest), libPath);
  if (row === original && process.env.SEMIO_NATIVE_RECEIVER_TRACE === "1") {
    const owner = resolve(dirname(crateSource), "../../🦀️.rs");
    const scratch = join(replacementDirectory, "original-source.rs");
    const code = readFileSync(owner, "utf8")
      .replace(/#\[path="([^"]+)"/gu, (_, path) => `#[path=${JSON.stringify(resolve(dirname(owner), path))}`)
      .replace("authority.drive(&mut NativeSlotOperation::Advance(input,receiver,body).with_witness(witness),effect,action)", `{let result=authority.drive(&mut NativeSlotOperation::Advance(input,receiver,body).with_witness(witness),effect,action);if !matches!(result,Ok(true)){eprintln!("[DEBUG] native original local drive={:?}, performed={}, child_pending={}, quote={d:?}", result.as_ref().map_err(|e| &e.message),effect.performed_work(),receiver.advance_pending());}result}`);
    writeFileSync(scratch, code);
    const root = readFileSync(crateSource, "utf8").replace("../../🦀️.rs", scratch);
    crateSource = join(replacementDirectory, "original-root.rs");
    writeFileSync(crateSource, root);
  }
  const artifact = join(replacementDirectory, `lib${row.name}.rlib`);
  const args = ["run", channel, "rustc", "--crate-type=rlib", "--crate-name", row.name, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0", `-Copt-level=${optLevel}`, "-Cmetadata=receiver_link", crateSource, "-o", artifact, "-L", `dependency=${replacementDirectory}`];
  addExterns(args, dependencies(`${row.fingerprint}.json`));
  const child = Bun.spawn([rustup, ...args], { env: { ...process.env, CARGO_MANIFEST_DIR: dirname(manifest), CARGO_PKG_NAME: row.name.replaceAll("_", "-") }, stdout: "inherit", stderr: "inherit" });
  if (await child.exited) throw Error(`Differential compilation failed for ${row.name}`);
  replacements.set(row.fingerprint, artifact);
}
const uniqueTest = process.env.SEMIO_NATIVE_RECEIVER_UNIQUE_TEST === "1";
const uniqueManifest = resolve(replacementManifests[output.name]);
const testManifest = uniqueTest ? uniqueManifest : manifest;
const testSource = uniqueTest ? resolve(dirname(uniqueManifest), "../../🦀️.rs") : source;
const testName = uniqueTest ? output.name : "semio_plugin_actor_input";
const executable = join(generated, (uniqueTest ? "mcp-unique-json-test" : "mcp-receiver-differential-test") + (process.platform === "win32" ? ".exe" : ""));
const args = ["run", channel, "rustc", "--test", "--crate-name", testName, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0", `-Copt-level=${optLevel}`, testSource, "-o", executable, "-L", `dependency=${replacementDirectory}`];
if (uniqueTest) {
  const jsonSerde = direct.find(row => row.name === "serde_json")!;
  const core = dependencies(`${jsonSerde.fingerprint}.json`).find(row => row.name === "serde_core");
  const serde = graph.find(row => row.name === "serde" && dependencies(`${row.fingerprint}.json`).some(dep => dep.name === "serde_derive")
    && dependencies(`${row.fingerprint}.json`).find(dep => dep.name === "serde_core")?.fingerprint === core?.fingerprint);
  const testDeps = [jsonSerde, serde, ...direct.filter(row => ["semio_framework_job_testing_caller", "semio_framework_value_testing_allocator"].includes(row.name))];
  if (testDeps.length !== 4 || testDeps.some(row => !row)) throw Error("Exact unique JSON test dependencies unavailable");
  addExterns(args, [...outputDependencies, ...testDeps as ReturnType<typeof dependencies>]);
} else {
  addExterns(args, direct);
}
console.log("[DEBUG] Relinking the native receiver regression against", visited.size, "exact Rust dependencies");
const compiled = Bun.spawn([rustup, ...args], { env: { ...process.env, CARGO_MANIFEST_DIR: dirname(testManifest), CARGO_PKG_NAME: testName.replaceAll("_", "-") }, stdout: "inherit", stderr: "inherit" });
if (await compiled.exited) throw Error("Exact Actor Input relink failed");
process.exitCode = await Bun.spawn([executable, ...process.argv.slice(2)], { env: process.env, stdout: "inherit", stderr: "inherit" }).exited;

