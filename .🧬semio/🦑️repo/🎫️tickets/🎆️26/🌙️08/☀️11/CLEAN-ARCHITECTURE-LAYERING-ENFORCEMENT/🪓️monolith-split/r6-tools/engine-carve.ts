// 🪓️ One-off tool: carves the flat module tree of the wfc engine crate into producer crates behind the unchanged facade crate.
// usage: bun engine-carve.ts plan|apply [--backup=<dir>]
import {existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
import {addWorkspaceMembers, insertBuildOwnerRows, patchCargoLock, rebaseManifestRow, relativePosix, readManifestDependencyRows} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const WFC = "✏️s/🔌️plugins/🀄️wfc";
const ENGINE = `${WFC}/⚙️engine`;
const FACADE = "semio-s-plugin-wfc-engine";
const mode = process.argv[2] ?? "plan";
const backupArg = process.argv.find((arg) => arg.startsWith("--backup="))?.slice("--backup=".length);

type Spec = {id: string; node: string; emoji: string; doc: string; mods: string[]};
const SPECS: Spec[] = [
  {id: "foundation", node: "🏛️foundation", emoji: "🏛️", doc: "Foundation of the wfc engine: ids, bitsets, weights, the compiled model, domains, topology and the propagation primitives every solver builds on.", mods: ["bitset", "domain", "error", "heuristics", "ids", "model", "propagate", "sample", "soft", "topology", "weights"]},
  {id: "oracle", node: "🧿️oracle", emoji: "🧿️", doc: "Brute-force reference solver and the shared model vectors every optimized solver of the wfc engine is cross-checked against.", mods: ["model_vectors", "oracle"]},
  {id: "kernel", node: "🧠️kernel", emoji: "🧠️", doc: "Propagation kernel of the wfc engine: diagnostics, outcomes, trail, arc-consistency propagators, nogood learning, beam search and the constraint families.", mods: ["beam", "constraint", "constraints_card", "constraints_conn", "diag", "flow", "nogood", "outcome", "prop_ac3", "prop_ac4", "trail"]},
  {id: "job", node: "💼️job", emoji: "💼️", doc: "Resumable wfc job: checkpoints, commits, publication and the headless drivers.", mods: ["job"]},
  {id: "solve", node: "🧗️solve", emoji: "🧗️", doc: "Solvers of the wfc engine: the backtracking search, repair, parallel portfolios, chunked and hierarchical solving, evolution and checkpoint serialization.", mods: ["chunk", "evolve", "hierarchy", "parallel", "repair", "search", "serial", "solver_graph"]},
  {id: "lattice", node: "🧮️lattice", emoji: "🧮️", doc: "Dense lattice front ends of the wfc engine: symmetry, tiles, motifs, 2D and 3D grids, their solvers, overlapping extraction and sparse 3D.", mods: ["extract", "grid2d", "grid3d", "motif", "solver_grid2d", "solver_grid3d", "sparse3d", "symmetry", "tiled"]},
];

const abs = (path: string) => join(ROOT, path);
const rootText = readFileSync(abs(`${ENGINE}/🦀️.rs`), "utf8");
const modules = new Map<string, {path: string; cfgTest: boolean; dir: string}>();
for (const match of rootText.matchAll(/(#\[cfg\(test\)\]\s*)?#\[path = "([^"]+)"\]\s*(?:pub )?mod (\w+);/gu)) modules.set(match[3]!, {path: match[2]!, cfgTest: Boolean(match[1]), dir: dirname(match[2]!)});

const stripComments = (text: string): string => text.replace(/\/\*[\s\S]*?\*\//gu, "").split("\n").map((line) => line.replace(/\/\/.*$/u, "")).join("\n");
const walk = (dir: string, out: string[] = []): string[] => {
  for (const entry of readdirSync(dir, {withFileTypes: true})) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) { if (entry.name !== "node_modules") walk(path, out); } else out.push(path);
  }
  return out;
};

const owner = new Map<string, string>();
for (const spec of SPECS) for (const name of spec.mods) { if (!modules.has(name)) throw new Error(`unknown module ${name}`); if (owner.has(name)) throw new Error(`module ${name} assigned twice`); owner.set(name, spec.id); }
for (const name of modules.keys()) if (name !== "grid_job_drive_tests" && !owner.has(name)) throw new Error(`module ${name} unassigned`);

type Refs = {lib: Set<string>; test: Set<string>};
const refsOf = new Map<string, Refs>();
for (const [name, info] of modules) {
  if (name === "grid_job_drive_tests") continue;
  const refs: Refs = {lib: new Set(), test: new Set()};
  const files = info.cfgTest ? [abs(`${ENGINE}/${info.path}`)] : walk(abs(`${ENGINE}/${info.dir}`)).filter((file) => file.endsWith(".rs"));
  for (const file of files) {
    const isTest = file.includes("/🧪️tests/") && !info.cfgTest && !file.endsWith("/🧮️model-vectors/🦀️.rs");
    const text = stripComments(readFileSync(file, "utf8"));
    for (const match of text.matchAll(/\bcrate::(\w+)/gu)) if (modules.has(match[1]!) && match[1] !== name) (isTest ? refs.test : refs.lib).add(match[1]!);
  }
  refsOf.set(name, refs);
}

type Plan = {spec: Spec; libImports: Map<string, string[]>; testImports: Map<string, string[]>; deps: Set<string>; devDeps: Set<string>};
const plans = new Map<string, Plan>();
for (const spec of SPECS) {
  const lib = new Set<string>(), test = new Set<string>();
  for (const name of spec.mods) {
    const refs = refsOf.get(name)!;
    for (const ref of refs.lib) if (owner.get(ref) !== spec.id) lib.add(ref);
    for (const ref of refs.test) if (owner.get(ref) !== spec.id) test.add(ref);
  }
  for (const ref of lib) test.delete(ref);
  const group = (set: Set<string>) => { const map = new Map<string, string[]>(); for (const ref of [...set].sort()) { const id = owner.get(ref)!; map.set(id, [...(map.get(id) ?? []), ref]); } return map; };
  const libImports = group(lib), testImports = group(test);
  plans.set(spec.id, {spec, libImports, testImports, deps: new Set(libImports.keys()), devDeps: new Set([...testImports.keys()].filter((id) => !libImports.has(id)))});
}
const order: string[] = [];
const visit = (id: string, stack: string[] = []) => {
  if (order.includes(id)) return;
  if (stack.includes(id)) throw new Error(`crate cycle ${[...stack, id].join(" -> ")}`);
  const plan = plans.get(id)!;
  for (const dep of [...plan.deps, ...plan.devDeps]) visit(dep, [...stack, id]);
  order.push(id);
};
for (const spec of SPECS) visit(spec.id);

const crateName = (id: string) => `${FACADE}-${id}`;
const crateIdent = (id: string) => crateName(id).replaceAll("-", "_");
const packageDir = (spec: Spec) => `${ENGINE}/${spec.node}/📦️packages/🦀️rust`;
const facadeDir = `${ENGINE}/📦️packages/🦀️rust`;

if (mode === "plan") {
  for (const id of order) {
    const plan = plans.get(id)!;
    console.log(`${crateName(id)} mods=[${plan.spec.mods.join(",")}] deps=[${[...plan.deps].join(",")}] dev=[${[...plan.devDeps].join(",")}]`);
    for (const [dep, mods] of plan.libImports) console.log(`   lib use ${dep}: ${mods.join(",")}`);
    for (const [dep, mods] of plan.testImports) console.log(`   test use ${dep}: ${mods.join(",")}`);
  }
  process.exit(0);
}

const originalManifest = readFileSync(abs(`${facadeDir}/Cargo.toml`), "utf8");
const rows = readManifestDependencyRows(originalManifest);
const workspaceRel = relativePosix(abs(`${ENGINE}/x/📦️packages/🦀️rust`), abs(WFC));
const backup = (path: string) => {
  if (!backupArg) return;
  const target = join(ROOT, backupArg, path);
  if (existsSync(abs(path)) && !existsSync(target)) { mkdirSync(dirname(target), {recursive: true}); writeFileSync(target, readFileSync(abs(path))); }
};
const write = (path: string, text: string) => { backup(path); mkdirSync(dirname(abs(path)), {recursive: true}); writeFileSync(abs(path), text); console.log(`wrote ${path}`); };

const rootFor = (plan: Plan): string => {
  const spec = plan.spec;
  const depth = relativePosix(abs(`${packageDir(spec)}`), abs(ENGINE));
  const lines: string[] = [`//! ${spec.emoji} ${spec.doc}`, "", "#![allow(clippy::result_large_err)]", ""];
  for (const name of [...spec.mods].sort()) {
    const info = modules.get(name)!;
    const rel = `${depth}/${info.path}`;
    lines.push(`#[path = "${rel}"]`, `pub mod ${name};`);
  }
  const imports = (map: Map<string, string[]>, gate: string) => {
    for (const id of order) {
      const names = map.get(id);
      if (!names) continue;
      if (gate) lines.push(gate);
      lines.push(`use ${crateIdent(id)}::{${names.join(", ")}};`);
    }
  };
  if (plan.libImports.size || plan.testImports.size) lines.push("");
  imports(plan.libImports, "");
  imports(plan.testImports, "#[cfg(test)]");
  return lines.join("\n") + "\n";
};

const manifestFor = (plan: Plan): string => {
  const spec = plan.spec;
  const dir = abs(packageDir(spec));
  const from = abs(facadeDir);
  const sibling = (id: string, extra = "") => `${crateName(id)} = { path = "${relativePosix(dir, abs(packageDir(plans.get(id)!.spec)))}"${extra} }`;
  const dependencies = [...rows.dependencies.map((row) => rebaseManifestRow(row, from, dir)), ...[...plan.deps].map((id) => sibling(id))];
  const dev = [...rows.devDependencies.map((row) => rebaseManifestRow(row, from, dir)), ...[...plan.devDeps].map((id) => sibling(id))];
  if (spec.id === "job") { /* testkit feature declared below */ }
  const sort = (list: string[]) => [...list].sort((a, b) => Buffer.compare(Buffer.from(a.split("=")[0]!.trim()), Buffer.from(b.split("=")[0]!.trim())));
  const lines = ["[package]", `workspace = "${workspaceRel}"`, `name = "${crateName(spec.id)}"`, "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", `description = ${JSON.stringify(spec.doc.replace(/\.$/u, ""))}`, "", "[package.metadata.semio]", 'role = "s-module"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "🦀️.rs"', ""];
  if (spec.id === "job") lines.push("[features]", "testkit = []", "");
  lines.push("[dependencies]", ...sort(dependencies), "", "[dev-dependencies]", ...sort(dev), "");
  return lines.join("\n");
};

for (const id of order) {
  const plan = plans.get(id)!;
  write(`${packageDir(plan.spec)}/Cargo.toml`, manifestFor(plan));
  write(`${packageDir(plan.spec)}/🦀️.rs`, rootFor(plan));
}

const reexports = SPECS.filter((spec) => spec.id !== "oracle").flatMap((spec) => [spec.id, ...[]].slice(0, 1).map(() => ({spec, mods: spec.mods})));
const facadeLines: string[] = [];
const original = rootText.split("\n");
const header = original.slice(0, original.findIndex((line) => line.startsWith("#[path")) ).join("\n").replace(/\n+$/u, "");
facadeLines.push(header.replace(/ Every module\nis production code[\s\S]*$/u, "").replace(/\n#!\[allow\(clippy::result_large_err\)\]/u, ""));
const facadeBody: string[] = [];
for (const id of order) {
  if (id === "oracle") continue;
  const spec = plans.get(id)!.spec;
  facadeBody.push(`pub use ${crateIdent(id)}::{${[...spec.mods].sort().join(", ")}};`);
}
const facadeRoot = `${header.trimEnd()}\n\n${facadeBody.join("\n")}\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️grid-job-drive/🦀️.rs"]\nmod grid_job_drive_tests;\n`;
write(`${ENGINE}/🦀️.rs`, facadeRoot);
void reexports;

const carvedRows = order.filter((id) => id !== "oracle").map((id) => `${crateName(id)} = { path = "${relativePosix(abs(facadeDir), abs(packageDir(plans.get(id)!.spec)))}" }`);
const jobRow = `${crateName("job")} = { path = "${relativePosix(abs(facadeDir), abs(packageDir(plans.get("job")!.spec)))}", features = ["testkit"] }`;
const dependenciesAt = originalManifest.search(/^\[dependencies\]\s*$/mu);
const lineEnd = originalManifest.indexOf("\n", dependenciesAt) + 1;
let manifest = originalManifest.slice(0, lineEnd) + carvedRows.join("\n") + "\n" + originalManifest.slice(lineEnd);
manifest = manifest.replace(/^\[dev-dependencies\]\s*$/mu, (head) => `${head}\n${jobRow}`);
write(`${facadeDir}/Cargo.toml`, manifest);

const members = SPECS.map((spec) => relativePosix(abs(WFC), abs(packageDir(spec))));
write(`${WFC}/Cargo.toml`, addWorkspaceMembers(readFileSync(abs(`${WFC}/Cargo.toml`), "utf8"), members));

const lockPaths = ["Cargo.lock", "🏭️bridge/Cargo.lock", "🗿️artifacts/◻️2d/Cargo.lock", "🗿️artifacts/🔲️grid2d/Cargo.lock", "🗿️artifacts/🖼️bitmap/Cargo.lock", "🗿️artifacts/🧊️3d/Cargo.lock", "🗿️artifacts/🧱️grid3d/Cargo.lock"].map((path) => `${WFC}/${path}`).concat(["🌎️hub/Cargo.lock"]);
const carved = SPECS.map((spec) => ({name: crateName(spec.id), dependsOn: [...plans.get(spec.id)!.deps, ...plans.get(spec.id)!.devDeps].map(crateName)}));
for (const path of lockPaths) write(path, patchCargoLock(readFileSync(abs(path), "utf8"), FACADE, carved));

const ownersFile = abs("📋️build-owners.json");
backup("📋️build-owners.json");
const {text, added} = insertBuildOwnerRows(readFileSync(ownersFile, "utf8"), SPECS.map((spec) => ({name: crateName(spec.id), root: packageDir(spec)})));
writeFileSync(ownersFile, text);
console.log(`owner rows added: ${added.join(", ")}`);

const scriptPath = `${facadeDir}/📜️script.ts`;
let script = readFileSync(abs(scriptPath), "utf8");
const names = [FACADE, ...SPECS.map((spec) => crateName(spec.id))].sort();
script = script.replace('class CheckScript', `const PACKAGES = [${names.map((name) => `"${name}"`).join(", ")}].flatMap(name => ["-p", name]);\n\nclass CheckScript`);
script = script.replace('["check", "-p", "semio-s-plugin-wfc-engine", "--lib"', '["check", ...PACKAGES, "--lib"').replace('["test", "-p", "semio-s-plugin-wfc-engine", "--lib"', '["test", ...PACKAGES, "--lib"');
write(scriptPath, script);
void statSync; void relative; void resolve;
