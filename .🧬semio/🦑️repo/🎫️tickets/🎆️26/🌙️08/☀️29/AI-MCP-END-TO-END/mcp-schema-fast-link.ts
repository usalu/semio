import assert from "node:assert/strict";
import { existsSync, mkdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated = join(import.meta.dir, "🗑️generated");
const cache = join(generated, "native-host", "debug", "build");
const output = join(generated, "inference-fast");
const gateway = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const directories: Record<string, string> = {
  semio_os_mcp_workspace: join(gateway, "🪟️workspace-host/📦️packages/🦀️rust"),
  semio_os_mcp_tools_inference: join(gateway, "🔭️inference-tools/📦️packages/🦀️rust"),
  semio_os_mcp: join(gateway, "📦️packages/🦀️rust"),
};
const patches = new Set(["semio_os_mcp_workspace", "semio_os_mcp_tools_inference"]);
const rootCandidates = [...new Bun.Glob("semio-os-mcp/*/fingerprint/bin-semio-os-mcp.json").scanSync({ cwd: cache, absolute: true })]
  .sort((a, b) => statSync(b).mtimeMs - statSync(a).mtimeMs);
type Unit = { name: string; artifact: string; fingerprint: string };
const units = new Map<string, Unit>();
const nativePaths = new Set<string>();
for (const path of new Bun.Glob("*/*/out/*").scanSync({ cwd: cache, absolute: true })) {
  if (/\.(a|lib)$/u.test(path)) nativePaths.add(dirname(path));
  const name = /\/lib(.+)-[a-f0-9]+\.(?:rlib|dylib|so|dll)$/u.exec(path.replaceAll("\\", "/"))?.[1];
  if (!name) continue;
  const fingerprint = join(dirname(path), "..", "fingerprint", `lib-${name}`);
  if (!existsSync(fingerprint)) continue;
  const digest = readFileSync(fingerprint, "utf8").trim();
  if (digest) units.set(digest, { name, artifact: path, fingerprint: fingerprint + ".json" });
}

const children = new Map<string, Map<string, Unit>>();
function dependencies(fingerprint: string): Map<string, Unit> {
  const prior = children.get(fingerprint);
  if (prior) return prior;
  const rows = new Map<string, Unit>();
  for (const [, alias, number] of readFileSync(fingerprint, "utf8").matchAll(/"([^"]+)",false,(\d+)\]/gu)) {
    if (alias === "build_script_build") continue;
    const digest = Buffer.alloc(8);
    digest.writeBigUInt64LE(BigInt(number));
    const entry = units.get(digest.toString("hex"));
    if (!entry) throw Error(`Missing exact archive ${alias} (${digest.toString("hex")})`);
    rows.set(alias, entry);
  }
  children.set(fingerprint, rows);
  return rows;
}

let base: string | undefined, paths = new Set<string>();
let changed = new Map<Unit, Map<string, Unit>>();
for (const candidate of rootCandidates.slice(0, 5)) {
  const visited = new Map<string, boolean>();
  paths = new Set();
  changed = new Map();
  function visit(unit: Unit): boolean {
    if (visited.has(unit.fingerprint)) return visited.get(unit.fingerprint)!;
    paths.add(dirname(unit.artifact));
    const deps = dependencies(unit.fingerprint);
    const transitivelyChanged = [...deps.values()].map(visit).some(Boolean);
    const rebuild = patches.has(unit.name) || transitivelyChanged;
    if (rebuild) {
      if (!directories[unit.name]) throw Error(`Extra affected crate ${unit.name}`);
      changed.set(unit, deps);
    }
    visited.set(unit.fingerprint, rebuild);
    return rebuild;
  }
  try {
    [...dependencies(candidate).values()].map(visit);
    assert(changed.size === 3, "Both fixes and the root library must be rebuilt");
    base = candidate;
    break;
  } catch (error) {
    console.log("[DEBUG] Rejected native fingerprint", candidate.slice(cache.length + 1), String(error));
  }
}
assert(base, "No coherent cached native gateway graph with a three-crate replacement");
console.log("[DEBUG] Exact native gateway basis", base.slice(cache.length + 1), paths.size, [...changed.keys()].map(unit => unit.name));
if (process.argv.includes("--inspect")) process.exit(0);

mkdirSync(output, { recursive: true });
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
assert(channel);
const replacement = new Map<Unit, string>();
async function compile(name: string, fingerprint: string, deps: Map<string, Unit>, type: "rlib" | "bin") {
  const directory = resolve(directories[name]!);
  const manifest = readFileSync(join(directory, "Cargo.toml"), "utf8");
  const section = manifest.split(type === "rlib" ? "[lib]" : "[[bin]]")[1]?.split(/^\[/mu)[0];
  const source = section?.match(/^path\s*=\s*"([^"]+)"/mu)?.[1];
  assert(source);
  const file = join(output, type === "rlib" ? `lib${name}.rlib` : process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp");
  const args = ["repo", "tools", "exec", "--light", "rustup", "--", "run", channel!, "rustc", "--crate-type=" + type,
    "--crate-name", name, "--edition=2021", "-Cdebuginfo=0", "-Zthreads=1", "-Cmetadata=inference_fast", join(directory, source), "-o", file, "-L", `dependency=${output}`];
  const features: string[] = JSON.parse(JSON.parse(readFileSync(fingerprint, "utf8")).features);
  for (const feature of features) args.push("--cfg", `feature=${JSON.stringify(feature)}`);
  for (const [alias, child] of deps) {
    const archive = replacement.get(child) ?? child.artifact;
    const metadata = archive.replace(/\.rlib$/u, ".rmeta");
    if (metadata !== archive && existsSync(metadata)) args.push("--extern", `${alias}=${metadata}`);
    args.push("--extern", `${alias}=${archive}`);
  }
  for (const path of paths) args.push("-L", `dependency=${path}`);
  for (const path of nativePaths) args.push("-L", `native=${path}`);
  console.log("[DEBUG] Compiling", name, type);
  const child = Bun.spawn(args, {
    env: { ...process.env, CARGO_MANIFEST_DIR: directory, CARGO_PKG_NAME: name.replaceAll("_", "-"), CARGO_PKG_VERSION: "0.1.0" },
    stdout: "inherit", stderr: "inherit", timeout: type === "bin" ? 60_000 : 30_000,
  });
  assert.equal(await child.exited, 0, `Native ${name} ${type} link failed`);
  return file;
}

for (const [unit, deps] of changed) replacement.set(unit, await compile(unit.name, unit.fingerprint, deps, "rlib"));
const executable = await compile("semio_os_mcp", base, dependencies(base), "bin");
console.log("[DEBUG] Linked", executable);
