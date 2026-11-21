import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated = resolve(import.meta.dirname, "🗑️generated");
const build = resolve(process.argv[2] ?? join(generated, "native-host", "debug", "build"));
const output = join(generated, "note-replay");
mkdirSync(output, { recursive: true });
const byDigest = new Map<string, { name: string; file: string; fingerprint: string }>();
const native = new Set<string>();
const roots: string[] = [];
for (const name of readdirSync(build)) {
  const packageDir = join(build, name);
  if (!statSync(packageDir).isDirectory()) continue;
  for (const variant of readdirSync(packageDir, { withFileTypes: true }).filter(row => row.isDirectory())) {
    const unit = join(packageDir, variant.name);
    const files = join(unit, "out");
    const fingerprints = join(unit, "fingerprint");
    if (name === "semio-os-mcp") {
      const root = join(fingerprints, "bin-semio-os-mcp.json");
      if (existsSync(root)) roots.push(root);
    }
    if (!existsSync(files)) continue;
    for (const file of readdirSync(files)) {
      if (/\.(a|lib)$/u.test(file)) native.add(files);
      const name = /^lib(.+)-[0-9a-f]+\.(rlib|dylib|so|dll)$/u.exec(file)?.[1];
      if (!name) continue;
      const fingerprint = join(fingerprints, "lib-" + name);
      if (!existsSync(fingerprint)) continue;
      byDigest.set(readFileSync(fingerprint, "utf8").trim(), { name, file: join(files, file), fingerprint: fingerprint + ".json" });
    }
  }
}

const paths = new Set<string>();
const createdLibraries = new Set<string>();
const reuse = new Map<string, string>();
const visited = new Set<string>();
function select(fingerprint: string) {
  if (visited.has(fingerprint)) return;
  visited.add(fingerprint);
  for (const [, alias, number] of readFileSync(fingerprint, "utf8").matchAll(/"([^"]+)",false,(\d+)\]/gu)) {
    const digest = Buffer.alloc(8);
    digest.writeBigUInt64LE(BigInt(number));
    const row = byDigest.get(digest.toString("hex"));
    if (!row) {
      if (alias === "build_script_build") continue;
      throw Error("Missing native dependency " + alias + " in " + fingerprint);
    }
    paths.add(dirname(row.file));
    reuse.set(row.name, reuse.get(row.name) ?? row.file);
    select(row.fingerprint);
  }
}
roots.sort((left, right) => statSync(right).mtimeMs - statSync(left).mtimeMs);
for (const root of roots) {
  try {
    select(root);
    break;
  }
  catch (error) {
    console.error("[DEBUG]", String(error));
    paths.clear();
    reuse.clear();
    visited.clear();
  }
}
if (!reuse.has("semio_os_kernel")) throw Error("No complete native MCP build tree");

function selectPluginUiContract() {
  const plugin = [...byDigest.values()].find(row => row.file === reuse.get("semio_plugin"));
  if (!plugin) return;
  for (const [, alias, number] of readFileSync(plugin.fingerprint, "utf8").matchAll(/"([^"]+)",false,(\d+)\]/gu)) {
    if (alias !== "semio_ui_contract") continue;
    const digest = Buffer.alloc(8);
    digest.writeBigUInt64LE(BigInt(number));
    const row = byDigest.get(digest.toString("hex"));
    if (row) reuse.set(alias, row.file);
  }
}
selectPluginUiContract();

const registry = Bun.spawnSync(["repo", "tools", "list", "--json"]);
const rustup = JSON.parse(registry.stdout.toString()).find((row: { id: string }) => row.id === "rustup").command;
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !channel) throw Error("Pinned Rust toolchain is unavailable");
const note = resolve("✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note");
const stdio = resolve("✏️s/🔌️plugins/🗄️stdio");
const mountedWorkspaces = new Map([
  ["semio_s_artifact_stdio_dxf", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust/Cargo.toml"],
  ["semio_s_artifact_stdio_semio_text_model", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📦️packages/🦀️rust/Cargo.toml"],
  ["semio_s_artifact_stdio_semio_base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📦️packages/🦀️rust/Cargo.toml"],
  ["semio_s_artifact_stdio_contract", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml"],
]);

function dependencyLines(text: string) {
  return [...(text.match(/^\[dependencies\]\s*\n([^\[]*)/mu)?.[1] ?? "").matchAll(/^([a-z0-9_-]+)(?:\.workspace)?\s*=\s*(.*)/gmu)];
}

function compile(name: string, source: string, type: "bin" | "rlib", externs: Map<string, string>, manifest: string) {
  const binary = type === "rlib" ? "lib" + name + ".rlib" : "note-mcp-replay" + (process.platform === "win32" ? ".exe" : "");
  const file = join(output, binary);
  const args = ["run", channel!, "rustc", "--crate-type=" + type, "--edition=2021", "--crate-name", name, "-Zthreads=1", "-Cdebuginfo=0", "-Cmetadata=note_mcp_replay", source, "-o", file, "-L", "dependency=" + output];
  for (const [alias, dep] of externs) {
    const metadata = dep.replace(/\.rlib$/u, ".rmeta");
    if (metadata !== dep && existsSync(metadata)) args.push("--extern", alias + "=" + metadata);
    args.push("--extern", alias + "=" + dep);
  }
  for (const directory of paths) args.push("-L", "dependency=" + directory);
  for (const directory of native) args.push("-L", "native=" + directory);
  console.error("[DEBUG] Linking " + name + " against " + externs.size + " direct libraries");
  const child = Bun.spawnSync([rustup, ...args], {
    stdout: "inherit", stderr: "inherit", env: { ...process.env, CARGO_MANIFEST_DIR: dirname(manifest), CARGO_PKG_NAME: name.replaceAll("_", "-"), CARGO_PKG_VERSION: "0.1.0" },
  });
  if (child.exitCode) throw Error("Exact native link failed for " + name);
  if (type === "rlib") createdLibraries.add(file);
  return file;
}
function collect(manifest: string): Map<string, string> {
  const dependencies = new Map<string, string>();
  const source = readFileSync(manifest, "utf8");
  const relativeWorkspace = source.match(/^workspace\s*=\s*"([^"]+)"/mu)?.[1];
  const workspace = relativeWorkspace && resolve(dirname(manifest), relativeWorkspace, "Cargo.toml");
  const workspaceContent = workspace && existsSync(workspace) ? readFileSync(workspace, "utf8") : "";
  for (const [, alias, detail] of dependencyLines(source)) {
    if (/optional\s*=\s*true/u.test(detail)) continue;
    const packageName = detail.match(/\bpackage\s*=\s*"([^"]+)"/u)?.[1] ?? alias;
    const name = packageName.replaceAll("-", "_");
    const relative = detail.match(/\bpath\s*=\s*"([^"]+)"/u)?.[1];
    const workspacePath = workspaceContent.split("[workspace.dependencies." + alias + "]")[1]
      ?.split("\n[")[0]?.match(/^path\s*=\s*"([^"]+)"/mu)?.[1];
    const dependency = relative ? resolve(dirname(manifest), relative, "Cargo.toml")
      : workspace && workspacePath ? resolve(dirname(workspace), workspacePath, "Cargo.toml") : undefined;
    const library = dependency && existsSync(dependency) ? readFileSync(dependency, "utf8")
      .match(/^\[lib\]\s*\n([^\[]*)/mu)?.[1]?.match(/^name\s*=\s*"([^"]+)"/mu)?.[1] : undefined;
    const mounted = library ?? name;
    const toBuild = mountedWorkspaces.get(name) ?? (dependency && (dependency.startsWith(note) || (manifest.startsWith(stdio) && dependency.startsWith(stdio))) ? dependency : undefined);
    const row = reuse.get(mounted) ?? (toBuild ? buildNote(toBuild) : undefined);
    if (row) dependencies.set((packageName === alias && library ? library : alias).replaceAll("-", "_"), row);
    else if (!packageName.startsWith("semio-s-artifact-stdio-")) console.error("[DEBUG] No existing native crate for " + packageName);
  }
  return dependencies;
}
function buildNote(manifest: string) {
  manifest = resolve(manifest);
  const text = readFileSync(manifest, "utf8");
  const name = text.match(/^name\s*=\s*"([^"]+)"/mu)![1].replaceAll("-", "_");
  const lib = text.match(/^\[lib\]\s*\npath\s*=\s*"([^"]+)"/mu)![1];
  const file = compile(name, resolve(dirname(manifest), lib), "rlib", collect(manifest), manifest);
  reuse.set(name, file);
  return file;
}

const executable = compile(
  "note_mcp_replay",
  join(import.meta.dirname, "note-mcp-replay.rs"),
  "bin",
  collect(join(import.meta.dirname, "note-replay", "Cargo.toml")),
  join(import.meta.dirname, "note-replay", "Cargo.toml"),
);
const smoke = Bun.spawnSync([executable], { stdin: "ignore", stderr: "inherit" });
if (smoke.exitCode !== 0) throw Error("The native replay executable could not start");
for (const library of createdLibraries) unlinkSync(library);
console.log(executable);
