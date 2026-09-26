"""🧩️ W2 item 4 (rebuild convergence), one-off apply script: `describe` consumes the exact `component-dev` deliverable.

Every edit is anchored on the current text and verified before anything is written; a missing anchor aborts with no file
changed. usage: python3 w2-item4-apply.py [--apply]   (default: dry run)"""
import os, re, sys

REPO = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
os.chdir(REPO)
DESCRIBE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe"
edits: dict[str, str] = {}


def load(path: str) -> str:
    return edits.get(path) if path in edits else open(path, encoding="utf-8").read()


def replace(path: str, old: str, new: str, count: int = 1) -> None:
    text = load(path)
    found = text.count(old)
    if found != count:
        raise SystemExit(f"ANCHOR {path}: expected {count}, found {found}: {old[:120]!r}")
    edits[path] = text.replace(old, new)


def cut_block(path: str, start: str, end: str, replacement: str = "") -> None:
    text = load(path)
    first = text.find(start)
    last = text.find(end, first)
    if first < 0 or last < 0 or text.count(start) != 1:
        raise SystemExit(f"BLOCK {path}: {start[:80]!r} .. {end[:60]!r} not found exactly once")
    edits[path] = text[:first] + replacement + text[last:]


# 1. component-build: the describe build is gone; describe reads the component-dev deliverable.
build = f"{DESCRIBE}/🏗️component-build/🟦️.ts"
cut_block(build, "/** @emoji 🧩 Builds the exact `component-dev` unit", "/** @emoji 🧬 Extracts the first core module")
replace(build, 'import { pluginComponentRustcArgs } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts";\n', "")

# 2. fresh-component: one describe route over the deliverable, plus its script command.
fresh = f"{DESCRIBE}/🏭️fresh-component/🟦️.ts"
replace(fresh, 'import { isAbsolute, join, relative, resolve } from "node:path";', 'import { dirname, isAbsolute, join, relative, resolve } from "node:path";\nimport { createRequire } from "node:module";')
replace(fresh, "import { devToolingEnv, parseExtensionCargoManifest, readStableBuildFile, resolveWorkspaceBin, runExactCargoLawProcess }", "import { BundleScript, devToolingEnv, readStableBuildFile, resolveWorkspaceBin, runExactCargoLawProcess }")
replace(fresh, "CRATE_NAME, buildPluginComponent, cargoTargetRoot, extractPluginCore,", "CRATE_NAME, extractPluginCore,")
snippet = open(".tmp-ticket/wp-w2/item4/describe-deliverable.snippet.ts", encoding="utf-8").read()
script_class = '''
/** @emoji 🛂️ `describe component --manifest <Cargo.toml>`: the command the inferred Nx `describe` target of every component runs. */
export class DescribeComponentScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 2 || segments[0] !== "--manifest") throw new Error("usage: component --manifest <Cargo.toml>");
    process.exit(describeComponentDeliverable(this.repoRoot, segments[1]!));
  }
}
'''
cut_block(fresh, "/** @emoji 🛂️ Shared implementation for a plugin/extension crate's own `📜️script.ts describe` command", "/** 🧬️ The exact bag handed to `createFreshComponentTests`", snippet + script_class)

# 3. the describe package's router registers the command.
router = f"{DESCRIBE}/📦️packages/🦀️rust/📜️script.ts"
replace(router, 'import { DescribeScript } from "../../🛂️descriptor-emission/🟦️.ts";', 'import { DescribeScript } from "../../🛂️descriptor-emission/🟦️.ts";\nimport { DescribeComponentScript } from "../../🏭️fresh-component/🟦️.ts";')
replace(router, '.register("describe", DescribeScript), import.meta.url);', '.register("describe", DescribeScript).register("component", DescribeComponentScript), import.meta.url);')

# 4. nx: `describe` is an inferred component target that depends on `component-dev` and emits the owner-root pair.
mjs = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"
replace(mjs, '''  commandInputs ??= nativeCommandInputs(workspaceRoot);
  return Object.fromEntries(["dev", "release"].flatMap((profile) => [[`component-${profile}`, {''', '''  commandInputs ??= nativeCommandInputs(workspaceRoot);
  const ownerRoot = nxPath(join(root, "..", ".."));
  const describe = {
    executor: DEFAULT_EXECUTOR,
    cache: false,
    dependsOn: ["component-dev"],
    outputs: [`{workspaceRoot}/${ownerRoot}/🛂️.descriptor.semio`, `{workspaceRoot}/${ownerRoot}/🔣️.json`],
    options: { cwd: ".", command: `bun ${JSON.stringify("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts")} component --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  };
  return Object.fromEntries([["describe", describe], ...["dev", "release"].flatMap((profile) => [[`component-${profile}`, {''')
replace(mjs, '''    options: { cwd: ".", command: `bun ${JSON.stringify(`${webRoot}/📜️script.ts`)} materialize ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }]]));''', '''    options: { cwd: ".", command: `bun ${JSON.stringify(`${webRoot}/📜️script.ts`)} materialize ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }]])]);''')

# 5. every runtime consumer resolves a component from its crate's deliverable, never cargo's internal target.
mcp_rs = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"
replace(mcp_rs, """    pub owner_root: PathBuf,
    pub wasm_out: String,
}""", """    pub owner_root: PathBuf,
    /// 📦️ The component crate path relative to the repo root (`cratePath`): its `dist/component-{dev,release}` holds the
    /// exact bytes `describe` read, so the committed descriptor describes the component this gateway runs.
    pub crate_path: PathBuf,
    pub wasm_out: String,
}""")
cut_block(mcp_rs, "// 🎯️ Cargo's configured deliverable root", "/// 🧭️ Locates the repo root", """/// 🎯️ A component's deliverables relative to its crate root, in the order a profile wins: `component-dev` is the build
/// `describe` reads (one build per component per rebuild), `component-release` the shipped one.
const PLUGIN_COMPONENT_PROFILE_DIRS: [&str; 2] = ["dist/component-dev", "dist/component-release"];

""")
replace(mcp_rs, "        entries.push(PluginRegistryEntry { plugin_id, owner_root, wasm_out });", "        entries.push(PluginRegistryEntry { plugin_id, owner_root, crate_path: PathBuf::from(crate_path), wasm_out });")
replace(mcp_rs, """/// 🗺️ Resolves one plugin's compiled `.wasm` under `PLUGIN_WASM_TARGET_DIR/{wasm-dev,wasm-release}` —
/// same profile-dir fallback order as `🏃️run/🏗️bootstrap/🦀️.rs::resolve_plugin_paths`.""", """/// 🗺️ Resolves one plugin's component under its crate's `dist/component-{dev,release}` — the same order as
/// `🏃️run/🏗️bootstrap/🦀️.rs::resolve_plugin_paths` and the TypeScript preflight in `🌉️mcp/🟦️.ts`.""")
replace(mcp_rs, """    for profile_dir in PLUGIN_WASM_PROFILE_DIRS {
        let path = repo_root.join(PLUGIN_WASM_TARGET_DIR).join(profile_dir).join(&entry.wasm_out);""", """    for profile_dir in PLUGIN_COMPONENT_PROFILE_DIRS {
        let path = repo_root.join(&entry.crate_path).join(profile_dir).join(&entry.wasm_out);""")
replace(mcp_rs, """build it with `bun nx run @semio-tech/framework-os-dev:build -- {}`", entry.plugin_id, tried.join(", "), entry.plugin_id)))""", """build its component-dev deliverable with `bun nx run-many -t describe --projects <its crate project>`", entry.plugin_id, tried.join(", "))))""")

mcp_ts = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts"
replace(mcp_ts, """const PLUGIN_WASM_TARGET_REL = ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2";
const PLUGIN_WASM_PROFILE_DIRS = ["wasm-dev", "wasm-release"] as const;""", """const PLUGIN_COMPONENT_PROFILE_DIRS = ["dist/component-dev", "dist/component-release"] as const;""")
replace(mcp_ts, "twin of `PLUGIN_WASM_TARGET_DIR`/`PLUGIN_WASM_PROFILE_DIRS` in `🌉️mcp/🏠️workspace/🦀️.rs`", "twin of `PLUGIN_COMPONENT_PROFILE_DIRS` in `🌉️mcp/🏠️workspace/🦀️.rs`")
replace(mcp_ts, """  for (const profile of PLUGIN_WASM_PROFILE_DIRS) {
    const candidate = posix.join(repoRoot, PLUGIN_WASM_TARGET_REL, profile, row.wasmOut);""", """  for (const profile of PLUGIN_COMPONENT_PROFILE_DIRS) {
    const candidate = posix.join(repoRoot, row.cratePath, profile, row.wasmOut);""")
replace(mcp_ts, "\\`cd ${row.cratePath} && bun ./📜️script.ts describe\\`", "\\`bun nx run-many -t describe --projects <the Nx project of ${row.cratePath}>\\`", 2)

projection = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts"
cut_block(projection, "/** @emoji 🗂️ Emits plugin wasm artifact constants for headless `semio-framework-os-run`.", "/** @emoji 🗂️ The full generated catalog", """/** @emoji 🗂️ Emits plugin component constants for headless `semio-framework-os-run`: each plugin id with its crate
 * path and component file, resolved under the crate's own `dist/component-{dev,release}` deliverable (the bytes the
 * committed descriptor describes), never under cargo's internal target directory. */
export function emitRustArtifacts(entries: PluginRegistryEntry[], _repoRoot: string): string {
  const rows = entries.map((entry) => `    (${JSON.stringify(entry.pluginId)}, ${JSON.stringify(entry.cratePath)}, ${JSON.stringify(entry.wasmOut)}),`).join("\\n");
  return `// @generated by framework/plugin/registry/script.ts — do not edit.

pub const PLUGIN_COMPONENT_PROFILE_DIRS: &[&str] = &["dist/component-dev", "dist/component-release"];
pub const PLUGIN_WASM_ARTIFACTS: &[(&str, &str, &str)] = &[
${rows}
];
`;
}


""")
replace(projection, 'import { cargoTargetDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";\n', "")

run_rs = "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs"
replace(run_rs, "registry:generate`) — `pub const PLUGIN_WASM_ARTIFACTS: &[(&str, &str)]`, each entry a plugin id\n// paired with its compiled component `.wasm` file name under `PLUGIN_WASM_TARGET_DIR` / profile dirs", "registry:generate`) — `pub const PLUGIN_WASM_ARTIFACTS: &[(&str, &str, &str)]`, each entry a plugin id\n// with its crate path and component `.wasm` file name under `PLUGIN_COMPONENT_PROFILE_DIRS`")
replace(run_rs, """/// 🗺️ Resolves every plugin id this run needs to its compiled `.wasm` under `PLUGIN_WASM_TARGET_DIR/`,
/// trying `PLUGIN_WASM_PROFILE_DIRS` in order (`wasm-dev` before `wasm-release`).""", """/// 🗺️ Resolves every plugin id this run needs to its component under its crate's deliverables, trying
/// `PLUGIN_COMPONENT_PROFILE_DIRS` in order (`dist/component-dev` before `dist/component-release`).""")
replace(run_rs, "    let artifact_by_plugin: HashMap<&str, &str> = PLUGIN_WASM_ARTIFACTS.iter().map(|(plugin_id, wasm_out)| (*plugin_id, *wasm_out)).collect();", "    let artifact_by_plugin: HashMap<&str, (&str, &str)> = PLUGIN_WASM_ARTIFACTS.iter().map(|(plugin_id, crate_path, wasm_out)| (*plugin_id, (*crate_path, *wasm_out))).collect();")
replace(run_rs, "        let wasm_out = artifact_by_plugin.get(plugin_id.as_str())", "        let (crate_path, wasm_out) = artifact_by_plugin.get(plugin_id.as_str())")
replace(run_rs, """        for profile_dir in PLUGIN_WASM_PROFILE_DIRS {
            let path = repo_root.join(PLUGIN_WASM_TARGET_DIR).join(profile_dir).join(wasm_out);""", """        for profile_dir in PLUGIN_COMPONENT_PROFILE_DIRS {
            let path = repo_root.join(crate_path).join(profile_dir).join(wasm_out);""")

root_script = "📜️script.ts"
replace(root_script, "type OsPluginArtifact = { pluginId: string; wasmOut: string };", "type OsPluginArtifact = { pluginId: string; cratePath: string; wasmOut: string };")
replace(root_script, """ * 🔍️Plugin ids from the generated plugin registry with no built `.wasm` under Cargo's own configured
 * deliverable root (`.cargo/config.toml`'s `build.target-dir`) `/wasm32-wasip2/{wasm-dev,wasm-release}/`
 * — same resolution order as `resolve_plugin_paths` in `semio-framework-os-run`.
 */
const PLUGIN_WASM_PROFILE_DIRS = ["wasm-dev", "wasm-release"] as const;

function pluginWasmArtifactExists(repoRoot: string, wasmOut: string): boolean {
  const wasmTargetDir = join(cargoTargetDirectory(repoRoot), "wasm32-wasip2");
  for (const profileDir of PLUGIN_WASM_PROFILE_DIRS) {
    if (existsSync(join(wasmTargetDir, profileDir, wasmOut))) return true;
  }
  return false;
}""", """ * 🔍️Plugin ids from the generated plugin registry with no component deliverable under their crate's
 * `dist/component-{dev,release}` — same resolution order as `resolve_plugin_paths` in `semio-framework-os-run`.
 */
const PLUGIN_COMPONENT_PROFILE_DIRS = ["dist/component-dev", "dist/component-release"] as const;

function pluginWasmArtifactExists(repoRoot: string, entry: OsPluginArtifact): boolean {
  return PLUGIN_COMPONENT_PROFILE_DIRS.some((profileDir) => existsSync(join(repoRoot, entry.cratePath, profileDir, entry.wasmOut)));
}""")
replace(root_script, "entries.filter((entry) => !pluginWasmArtifactExists(repoRoot, entry.wasmOut))", "entries.filter((entry) => !pluginWasmArtifactExists(repoRoot, entry))")
if load(root_script).count("cargoTargetDirectory") == 1:
    replace(root_script, 'import { cargoTargetDirectory } from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";\n', "")

norm = "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts"
replace(norm, "import { FRESH_COMPONENT_MAX_BYTES, pluginWasmArtifactPath } from", "import { FRESH_COMPONENT_MAX_BYTES } from")
replace(norm, "/** ⚖️ Law: the `wasm-dev` component this plugin's `describe` reads fits", "/** ⚖️ Law: the `component-dev` deliverable this plugin's `describe` reads fits")
replace(norm, """    const component = pluginWasmArtifactPath(this.repoRoot, "semio-s-plugin-norm");
    if (!existsSync(component)) throw new Error(`norm's wasm-dev component is absent, so its size cannot be weighed: build it with \\`cargo build -p semio-s-plugin-norm --target wasm32-wasip2 --profile wasm-dev\\` (${component})`);""", """    const component = join(this.root, "dist", "component-dev", "semio_s_plugin_norm.wasm");
    if (!existsSync(component)) throw new Error(`norm's component-dev deliverable is absent, so its size cannot be weighed: build it with \\`bun nx run @semio-tech/norm-plugin:component-dev\\` (${component})`);""")

# 6. laws and tests follow the deliverable rule.
launch = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts"
replace(launch, """  it("keeps generated native, root preflight, and MCP runtime profiles identical without debug", () => {
    const root = getWorkspaceRoot();
    for (const path of [
      "📜️script.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
    ]) {
      const source = readFileSync(join(root, path), "utf8");
      const declaration = source.split("\\n").find((line) => line.includes("const PLUGIN_WASM_PROFILE_DIRS"));
      expect(declaration, path).toContain('["wasm-dev", "wasm-release"]');
      expect(declaration, path).not.toContain('"debug"');
    }
    const describe = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts"), "utf8");
    expect(describe.match(/pluginComponentRustcArgs\\(packageName, "wasm-dev"\\)/g)).toHaveLength(1);""", """  it("resolves every runtime component from its crate's deliverable, which describe reads without building", () => {
    const root = getWorkspaceRoot();
    for (const path of [
      "📜️script.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts",
    ]) {
      const source = readFileSync(join(root, path), "utf8");
      const declaration = source.split("\\n").find((line) => line.includes("const PLUGIN_COMPONENT_PROFILE_DIRS"));
      expect(declaration, path).toContain('"dist/component-dev", "dist/component-release"');
      expect(source, path).not.toContain("PLUGIN_WASM_TARGET");
    }
    const describe = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts"), "utf8");
    expect(describe).not.toContain("pluginComponentRustcArgs");
    const inferred = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"), "utf8");
    expect(inferred).toContain('dependsOn: ["component-dev"]');""")

projection_law = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/📖️generated-projection/🟦️.ts"
replace(projection_law, """/** 🔮️ Generated wasm target dir must follow .cargo/config.toml, never private uplift env. */
describe("registry rust artifacts projection", () => {
  test("PLUGIN_WASM_TARGET_DIR ignores private CARGO_TARGET_DIR overrides", () => {
    const previous = process.env.CARGO_TARGET_DIR;
    process.env.CARGO_TARGET_DIR = "/tmp/semio-private-uplift-must-not-land-in-catalog";
    try {
      const body = emitRustArtifacts([], getWorkspaceRoot());
      expect(body).toContain("PLUGIN_WASM_TARGET_DIR");
      expect(body).toContain("wasm32-wasip2");
      expect(body).toContain("cache/cargo/target");
      expect(body).not.toContain("semio-private-uplift-must-not-land-in-catalog");""", """/** 🔮️ Generated component paths are crate deliverables, never cargo's target directory or a private uplift. */
describe("registry rust artifacts projection", () => {
  test("every row resolves under its crate's dist deliverable, whatever CARGO_TARGET_DIR says", () => {
    const previous = process.env.CARGO_TARGET_DIR;
    process.env.CARGO_TARGET_DIR = "/tmp/semio-private-uplift-must-not-land-in-catalog";
    try {
      const note = { pluginId: "note", cratePath: "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust", wasmOut: "semio_s_plugin_note.wasm" } as PluginRegistryEntry;
      const body = emitRustArtifacts([note], getWorkspaceRoot());
      expect(body).toContain('pub const PLUGIN_COMPONENT_PROFILE_DIRS: &[&str] = &["dist/component-dev", "dist/component-release"];');
      expect(body).toContain('("note", "✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust", "semio_s_plugin_note.wasm"),');
      expect(body).not.toContain("cache/cargo/target");
      expect(body).not.toContain("semio-private-uplift-must-not-land-in-catalog");""")

owned = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs"
replace(owned, """const PLUGIN_WASM_CARGO_CACHE_DIR: &str = ".🧬semio/🦑️repo/⚡️cache/cargo";
const PLUGIN_WASM_PROFILE_DIRS: [&str; 2] = ["wasm-dev", "wasm-release"];""", """const PLUGIN_COMPONENT_PROFILE_DIRS: [&str; 2] = ["component-dev", "component-release"];""")
cut_block(owned, "/// 🔎️ The FRESHEST build of one plugin component anywhere in the shared cargo cache.", "/// 🖍️ `✏️s/🔌️plugins/🖍️draw/🔣️.json`", """/// 🔎️ The FRESHEST deliverable of one plugin component: `<component crate>/dist/<profile>/<file>` over every plugin
/// and extension crate under `✏️s/🔌️plugins`, newest mtime first. These are the bytes `describe`, the dev staging and the
/// trusted catalog read; cargo's own target directories are build internals no law reads.
fn plugin_wasm(file_name: &str) -> Option<PathBuf> {
    plugin_wasm_in_profiles(file_name, &PLUGIN_COMPONENT_PROFILE_DIRS)
}

/// 🎯️ The same search restricted to named profiles. A law about how LONG a component takes must
/// say which build it means: `component-dev` carries four times the code of `component-release` for the same
/// plugin (215 MB against 48 MB for `🌍️gis` on 2026-09-22), and a trusted catalog stages the
/// RELEASE component, so a timing law that silently picked up whichever profile a peer rebuilt last
/// measures a build no hub ever runs — which is exactly what happened to slice HC1 at 21:52.
fn plugin_wasm_in_profiles(file_name: &str, profiles: &[&str]) -> Option<PathBuf> {
    let plugins = repo_root().join("✏️s/🔌️plugins");
    let mut crates = Vec::new();
    for owner in std::fs::read_dir(&plugins).ok()?.flatten() {
        crates.push(owner.path().join("📦️packages/🦀️rust"));
        for extension in std::fs::read_dir(owner.path().join("🧩️extensions")).into_iter().flatten().flatten() {
            crates.push(extension.path().join("📦️packages/🦀️rust"));
        }
    }
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    for crate_root in crates {
        for profile in profiles.iter().copied() {
            let candidate = crate_root.join("dist").join(profile).join(file_name);
            let Ok(modified) = candidate.metadata().and_then(|meta| meta.modified()) else { continue };
            if newest.as_ref().is_none_or(|(seen, _)| modified > *seen) {
                newest = Some((modified, candidate));
            }
        }
    }
    newest.map(|(_, path)| path)
}

""")
replace(owned, 'plugin_wasm_in_profiles(file_name, &["wasm-release"])', 'plugin_wasm_in_profiles(file_name, &["component-release"])', 2)
replace(owned, 'plugin_wasm_in_profiles("semio_s_plugin_gis.wasm", &["wasm-release"])', 'plugin_wasm_in_profiles("semio_s_plugin_gis.wasm", &["component-release"])')

poll = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs"
replace(poll, '.join("target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm");', '.join("✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_procedural.wasm");')
run_unit = "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧪️tests/🔬️unit/🦀️.rs"
replace(run_unit, 'let candidate_wasm_paths = [repo_root.join("target/wasm32-wasip2/wasm-dev/semio_s_plugin_note.wasm"), repo_root.join("target/wasm32-wasip2/wasm-release/semio_s_plugin_note.wasm")];', 'let candidate_wasm_paths = ["component-dev", "component-release"].map(|profile| repo_root.join("✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/dist").join(profile).join("semio_s_plugin_note.wasm"));')

exec(open(".tmp-ticket/wp-w2/item4/w2-item4-part4.py", encoding="utf-8").read())

for path, text in edits.items():
    if APPLY:
        open(path, "w", encoding="utf-8").write(text)
    print(("WROTE " if APPLY else "OK    ") + path)
print(f"{'APPLIED' if APPLY else 'DRY RUN'}: {len(edits)} files")
