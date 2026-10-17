// Source change: moves the 17 subset sqlite snapshot test files that the semio root crate mounts into five test-law crates, so the root test unit stays inside the gate.
import {existsSync, mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {join, relative, resolve} from "node:path";
import {addWorkspaceMembers, insertBuildOwnerRows, patchCargoLock, patchRouterPackages, readManifestDependencyRows, rebaseManifestRow, renderCrateManifest, manifestRowKey} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import {measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const ART = `${ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio`;
const SS = `${ART}/🏅️standards/🔖️v1/🪆️subsets`;
const FACADE = "semio-s-artifact-stdio-semio";
const facadeDir = `${ART}/📦️packages/🦀️rust`;
const subsets = ["🎞️animation", "🔊️audio", "🧊️brep", "📐️cad", "📑️document", "🖊️drawing", "🌊️flow", "🕸️graph", "🖼️image", "🧰️kit", "🔺️mesh", "🏛️model", "📦️object", "📽️presentation", "📊️table", "🔤️text", "🔢️value", "🎬️video"];
const names: Record<string, string> = {"🎞️animation": "animation", "🔊️audio": "audio", "🧊️brep": "brep", "📐️cad": "cad", "📑️document": "document", "🖊️drawing": "drawing", "🌊️flow": "flow", "🕸️graph": "graph", "🖼️image": "image", "🧰️kit": "kit", "🔺️mesh": "mesh", "🏛️model": "model", "📦️object": "object", "📽️presentation": "presentation", "📊️table": "table", "🔤️text": "text", "🔢️value": "value", "🎬️video": "video"};
const folders = ["🧫️", "🧬️", "🧲️", "🧱️", "🧮️"];

const sized: {dir: string; bytes: number; files: number}[] = [];
for (const dir of subsets) {
  const file = `${SS}/${dir}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`;
  const m = await measureRustCrate(ROOT, file, false, new Map(), SS);
  sized.push({dir, bytes: m.bytes, files: m.files});
}
sized.sort((a, b) => b.bytes - a.bytes);
const groups: {members: typeof sized; bytes: number; files: number}[] = Array.from({length: 5}, () => ({members: [], bytes: 0, files: 0}));
for (const item of sized) {
  groups.sort((a, b) => a.bytes - b.bytes);
  groups[0]!.members.push(item);
  groups[0]!.bytes += item.bytes;
  groups[0]!.files += item.files;
}
groups.sort((a, b) => a.members.map((m) => m.dir).sort()[0]!.localeCompare(b.members.map((m) => m.dir).sort()[0]!));
console.log(groups.map((g) => `${g.members.map((m) => names[m.dir]).join("+")} ${g.files}/${g.bytes}`));

const facadeManifestPath = `${facadeDir}/Cargo.toml`;
const facadeManifest = readFileSync(facadeManifestPath, "utf8");
const facadeRows = readManifestDependencyRows(facadeManifest);
const created: {name: string; root: string; dir: string}[] = [];
const fixtureUse = new Map<string, string>();
groups.forEach((group, index) => {
  const n = index + 1;
  const name = `${FACADE}-sqlite-laws-${n}`;
  const ident = name.replaceAll("-", "_");
  const dir = `${ART}/🧪️tests/🪶️sqlite/${folders[index]}subset-laws-${n}/📦️packages/🦀️rust`;
  mkdirSync(dir, {recursive: true});
  const normal = new Map<string, string>(facadeRows.dependencies.map((row) => [manifestRowKey(row), rebaseManifestRow(row, facadeDir, dir)]));
  for (const row of facadeRows.devDependencies) normal.set(manifestRowKey(row), rebaseManifestRow(row, facadeDir, dir));
  normal.delete(name);
  normal.set(FACADE, `${FACADE} = { path = "${relative(dir, facadeDir)}" }`);
  const manifest = renderCrateManifest({
    name,
    description: `Sqlite snapshot laws of the semio root artifact for the subsets ${group.members.map((m) => names[m.dir]).join(", ")}`,
    workspaceRel: relative(dir, ART),
    libPath: "🦀️.rs",
    dependencyRows: [...normal.values()],
    devDependencyRows: [],
    targetSections: [],
  });
  writeFileSync(`${dir}/Cargo.toml`, manifest);
  const mounts = group.members.map((m) => m.dir).sort().map((subset) => `#[path = "${relative(dir, `${SS}/${subset}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`)}"]\npub mod ${names[subset]};`);
  const root = [
    `//! 🧫️ Sqlite snapshot laws of the semio root artifact for the subsets ${group.members.map((m) => names[m.dir]).join(", ")}: the original test files, mounted unchanged, and the \`fixture\` of each subset that the base union law reuses.`,
    "",
    "#![allow(async_fn_in_trait)]",
    "#![allow(long_running_const_eval)]",
    "#![allow(dead_code, unused_imports)]",
    "",
    "extern crate semio_framework_os_kernel as dsl;",
    "extern crate semio_framework_os_kernel as protocol;",
    "extern crate semio_framework_os_kernel as store;",
    "extern crate semio_framework_schema as framework_schema;",
    "extern crate semio_framework_value_derive as value_derive;",
    "",
    "pub(crate) use semio_framework_io_sqlite_snapshot_testing_backing as snapshot_test_backing;",
    "pub use semio_s_artifact_stdio_semio::*;",
    "",
    ...mounts,
    "",
  ].join("\n");
  writeFileSync(`${dir}/🦀️.rs`, root);
  for (const m of group.members) fixtureUse.set(m.dir, ident);
  created.push({name, root: relative(ROOT, dir), dir});
  for (const m of group.members) {
    const file = `${SS}/${m.dir}/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`;
    const text = readFileSync(file, "utf8");
    const next = text.replace("pub(crate) fn fixture()", "pub fn fixture()");
    if (next === text && !text.includes("pub fn fixture()")) throw new Error(`no fixture in ${file}`);
    writeFileSync(file, next);
  }
});

const baseFile = `${SS}/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`;
let base = readFileSync(baseFile, "utf8");
for (const [dir, ident] of fixtureUse) {
  const from = `crate::subset_sqlite_snapshot_tests::${names[dir]}::fixture`;
  if (!base.includes(from)) throw new Error(`base test misses ${from}`);
  base = base.replaceAll(from, `${ident}::${names[dir]}::fixture`);
}
if (base.includes("subset_sqlite_snapshot_tests")) throw new Error("base test still names subset_sqlite_snapshot_tests");
writeFileSync(baseFile, base);

const rootFile = `${ART}/🦀️.rs`;
let rootText = readFileSync(rootFile, "utf8");
const start = rootText.indexOf("#[cfg(test)]\n#[path = \".\"]\nmod subset_sqlite_snapshot_tests {");
if (start < 0) throw new Error("root mount not found");
let depth = 0;
let end = rootText.indexOf("{", start);
for (let i = end; i < rootText.length; i += 1) {
  if (rootText[i] === "{") depth += 1;
  if (rootText[i] === "}") { depth -= 1; if (depth === 0) { end = i + 1; break; } }
}
rootText = rootText.slice(0, start) + rootText.slice(end).replace(/^\n/u, "");
writeFileSync(rootFile, rootText);

const devRows = created.map((c) => `${c.name} = { path = "${relative(facadeDir, c.dir)}" }`);
writeFileSync(facadeManifestPath, facadeManifest.replace("[dev-dependencies]\n", `[dev-dependencies]\n${devRows.join("\n")}\n`));

const wsPath = `${ART}/Cargo.toml`;
writeFileSync(wsPath, addWorkspaceMembers(readFileSync(wsPath, "utf8"), created.map((c) => relative(ART, c.dir))));
const lockPath = `${ART}/Cargo.lock`;
writeFileSync(lockPath, patchCargoLock(readFileSync(lockPath, "utf8"), FACADE, created.map((c) => ({name: c.name, dependsOn: [FACADE]}))));
const scriptPath = `${facadeDir}/📜️script.ts`;
writeFileSync(scriptPath, patchRouterPackages(readFileSync(scriptPath, "utf8"), FACADE, created.map((c) => c.name)));
const ownersPath = `${ROOT}/📋️build-owners.json`;
const owners = insertBuildOwnerRows(readFileSync(ownersPath, "utf8"), created.map((c) => ({name: c.name, root: c.root})));
writeFileSync(ownersPath, owners.text);
console.log("created", created.map((c) => c.name), "owner rows", owners.added.length);
