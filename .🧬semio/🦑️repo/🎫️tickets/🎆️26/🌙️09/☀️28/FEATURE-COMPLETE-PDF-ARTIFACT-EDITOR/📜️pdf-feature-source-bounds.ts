/** 📐 Measure the combined editor features through the repository's Cargo closure reader. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const { measureRustCrate } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const { CRATE_HARD_BUDGET } = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts"));
type Candidate = [string, string | null, string];
const generated = join(import.meta.dir, "🗑️generated");
const source = await Bun.file(join(generated, "pdf-registered-feature-overlay.json")).text();
const rows = JSON.parse(source) as Candidate[];
const overlay = new Map(rows.map(([path, ,body]) => [resolve(path), body]));
const base = process.argv[2];
assert.ok(base?.endsWith("/🧱️base"));
const graph = await Bun.file(join(generated, "wD", "unit-graph-prototype-00-retry.json")).json();
const root = process.cwd();
const crate = "semio-s-artifact-stdio-pdf-native-save";
const reader = (path: string) => overlay.get(path) ?? readFileSync(path, "utf8");
const saveManifest = resolve(base, "✏️editor/💾️native-save/📦️packages/🦀️rust/Cargo.toml");
const save = Bun.TOML.parse(reader(saveManifest)) as { package: {name:string; workspace:string}; lib: {path:string}; dependencies: Record<string,{path:string}> };
assert.equal(save.package.name, crate);
const exactManifest = (p: string) => Bun.TOML.parse(reader(p)) as {package?:{name:string}};
const workspaceMembers = join(dirname(saveManifest), save.package.workspace);
const workspace = Bun.TOML.parse(reader(join(workspaceMembers, "Cargo.toml"))) as unknown as {workspace: {members: string[]; dependencies: Record<string,{path:string}>}};
assert.ok(workspace.workspace.members.some(member => resolve(workspaceMembers, member, "Cargo.toml") === saveManifest));
for (const [name, link] of Object.entries(save.dependencies)) {
    const resolved = link.path ? resolve(dirname(saveManifest), link.path) : resolve(workspaceMembers, workspace.workspace.dependencies[name].path);
    assert.equal(exactManifest(join(resolved, "Cargo.toml")).package?.name, name);
}
for (const [path, ,payload] of rows.filter(([path]) => path.endsWith("Cargo.lock"))) {
    const lock = Bun.TOML.parse(payload) as unknown as {package: {name:string; dependencies?:string[]}[]};
    assert.equal(lock.package.filter(row => row.name === crate).length, 1, path);
    const controller = lock.package.find(row => row.name === "semio-s-artifact-stdio-pdf-schema-5");
    assert.ok(controller?.dependencies?.includes(crate), path);
}
const scopes = new Map<string, {src:string; dir:string; features:Set<string>}>();
const add = (name:string, src:string, dir:string, features:readonly string[] = []) => {
    const scope = scopes.get(name) ?? {src, dir, features: new Set<string>()};
    assert.equal(scope.src, src);
    for (const feature of features) scope.features.add(feature);
    scopes.set(name, scope);
};
for (const unit of graph.units) {
    const { target, pkg_id: id, features } = unit;
    if (!target.kind.includes("lib") || !id.startsWith("path+file:")) continue;
    if (!/^(semio_s_artifact_stdio_.*(?:pdf|contract_editing)|semio_plugin_(?:instance|document_media|decl_media)|semio_framework_pack_json$)/u.test(target.name)) continue;
    add(target.name, resolve(target.src_path), fileURLToPath(id.slice("path+".length).split("#")[0]), features);
}
add(crate, resolve(dirname(saveManifest), save.lib.path), dirname(saveManifest));
for (const [name, testId] of [
    ["semio_s_artifact_stdio_pdf_testkit", "original_mutation_archive"],
    ["semio_framework_pack_json", "original-json-owner"],
    ["semio_s_artifact_stdio_pdf_schema_4", "profile_windows"],
]) {
    const dir = scopes.get(name)!.dir;
    const manifest = Bun.TOML.parse(reader(join(dir, "Cargo.toml"))) as unknown as {test: {name:string; path:string}[]};
    const testPath = manifest.test.find(test => test.name === testId)?.path;
    assert.ok(testPath, testId);
    add(testId, resolve(dir,testPath), dir);
}

const out = [];
const claimed = new Set<string>();
let outside = false;
for (const os of ["linux", "macos", "windows", "wasi"]) {
    const cfg = [`target_os="${os}"`, os === "wasi" ? 'target_arch="wasm32"' : os === "macos" ? 'target_arch="aarch64"' : 'target_arch="x86_64"', os === "wasi" ? 'target_family="wasm"' : os === "windows" ? "windows" : "unix"];
    for (const [name, scope] of scopes) {
        for (const test of (name.startsWith("original") || name === "profile_windows") ? [true] : [false,true]) {
            const row = await measureRustCrate(root, scope.src, test, overlay, scope.dir, [...cfg, ...[...scope.features].map(value => `feature="${value}"`)]);
            const affected = row.paths.filter((path:string) => overlay.has(resolve(path)));
            if (!affected.length) continue;
            for (const path of affected) claimed.add(resolve(path));
            const limits = Bun.TOML.parse(reader(join(scope.dir, "Cargo.toml"))) as unknown as {package: { metadata?: { semio?: { "rust-source-boundary"?: Record<string,number> } } } };
            const local = limits.package.metadata?.semio?.["rust-source-boundary"] ?? {};
            const admitted = row.files <= Math.min(CRATE_HARD_BUDGET.files, local.maxSourceFiles ?? Infinity)
                && row.lines <= Math.min(CRATE_HARD_BUDGET.lines, local.maxActiveLines ?? Infinity)
                && row.bytes <= Math.min(CRATE_HARD_BUDGET.bytes, local.maxSourceBytes ?? Infinity);
            outside ||= !admitted;
            const result = { os, name, test, files:row.files, lines:row.lines, bytes:row.bytes, admitted };
            out.push(result);
            console.log("[DEBUG]", os, name, test ? "test" : "lib", row.files, row.lines, row.bytes, admitted ? "admitted" : "EXCEEDS");
        }
    }
}
for (const path of overlay.keys()) if (path.endsWith(".rs")) assert.ok(claimed.has(path), `No measured Cargo owner contains ${path}`);
await Bun.write(join(generated, "pdf-feature-source-bounds.json"), JSON.stringify({ overlay: new Bun.CryptoHasher("sha256").update(source).digest("hex"), checks: out }, null, 2) + "\n");
if (outside) process.exitCode = 1;
