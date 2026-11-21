import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const generated = resolve(import.meta.dirname, "🗑️generated");
const build = join(generated, "native-host", "debug", "build");
const crate = "semio_framework_value_native_receiving_frame";
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
function latest(packageName: string, crateName: string) {
  const root = join(build, packageName);
  return [...library.values()].filter(row => row.fingerprint.startsWith(root + "/") && basename(row.fingerprint) === `lib-${crateName}`).sort((a,b) => statSync(b.fingerprint).mtimeMs - statSync(a.fingerprint).mtimeMs)[0];
}
const target = latest(crate.replaceAll("_", "-"), crate);
const direct = dependencies(`${target.fingerprint}.json`);
for (const [packageName, name] of [["serde_json", "serde_json"], ["semio-framework-test-fixtures", "semio_framework_test_fixtures"]]) direct.push({name,...latest(packageName,name)});
const visited = new Set();
const search = new Set();
const pending = [...direct];
while (pending.length) {
  const row = pending.pop()!;
  if (visited.has(row.fingerprint)) continue;
  visited.add(row.fingerprint);
  search.add(row.directory);
  pending.push(...dependencies(`${row.fingerprint}.json`));
}
const manifest = resolve("🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/📦️packages/🦀️rust/Cargo.toml");
const source = resolve(dirname(manifest), "../../🦀️.rs");
const tools = Bun.spawnSync(["repo", "tools", "list", "--json"]);
if (tools.exitCode) throw Error("Pinned tool registry is unavailable");
const rustup = JSON.parse(tools.stdout.toString()).find((row: { id: string }) => row.id === "rustup")?.command;
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !channel) throw Error("Pinned Rust is unavailable");
const executable = join(generated, "receiving-transfer-unit" + (process.platform === "win32" ? ".exe" : ""));
const args = ["run", channel, "rustc", "--test", "--crate-name", crate, "--edition=2021", "-Zthreads=1", "-Cdebuginfo=0", source, "-o", executable];
for (const row of direct) {
  const metadata = row.artifact.replace(/\.rlib$/u, ".rmeta");
  if (metadata !== row.artifact && existsSync(metadata)) args.push("--extern", `${row.name}=${metadata}`);
  args.push("--extern", `${row.name}=${row.artifact}`);
}
for (const directory of search) args.push("-L", `dependency=${directory}`);
for (const directory of native) args.push("-L", `native=${directory}`);
console.log("[DEBUG] Relinking native receiving frame test against", visited.size, "exact Rust dependencies");
const compiled = Bun.spawn([rustup, ...args], { env: { ...process.env, CARGO_MANIFEST_DIR: dirname(manifest), CARGO_PKG_NAME: basename(manifest) }, stdout: "inherit", stderr: "inherit" });
if (await compiled.exited) throw Error("Exact native receiving frame relink failed");
process.exitCode = await Bun.spawn([executable, ...process.argv.slice(2)], { env: process.env, stdout: "inherit", stderr: "inherit" }).exited;
