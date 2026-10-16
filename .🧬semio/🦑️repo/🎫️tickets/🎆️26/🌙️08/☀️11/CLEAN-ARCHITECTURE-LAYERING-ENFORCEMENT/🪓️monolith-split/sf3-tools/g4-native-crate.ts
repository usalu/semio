// 🚦️ sf3 target 1, step 3: registers the shared generation-native crate (manifest, owner row, core member, artifact workspace tables, io-text dependency edges, lock closure).
// usage: bun g4-native-crate.ts [--write]
import {relative} from "node:path";
import {log, read, put, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, workspaceDependencyTable, depLine, abs, lockEnsure} from "../dm2-tools/lib.ts";
import {DRY} from "../dm2-tools/dd.ts";

const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const P = "✏️s/🔌️plugins/🌀️procedural";
const CORE = `${P}/🫀️core`;
const FW = "🧰️framework";
const OWNER = `${CORE}/🧬️generation/🪶️sqlite/🚦️native/📦️packages/🦀️rust`;
const NAME = "semio-s-procedural-core-generation-sqlite-native";
const OSM = `${FW}/🛍️products/💻️os/🔨️modules`;
const depDirs = {
  "semio-framework-dsl-record": `${FW}/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust`,
  "semio-framework-dsl-record-derive": `${FW}/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust`,
  "semio-framework-diagnostic": `${FW}/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust`,
};
const PACK = `${FW}/🔨️modules/🎒️pack/📦️packages/🦀️rust`;
const dependencies = [
  depLine("pack", OWNER, PACK, `, package = "semio-framework-pack"`),
  depLine("semio-framework-diagnostic", OWNER, depDirs["semio-framework-diagnostic"]),
  depLine("semio-framework-dsl-record", OWNER, depDirs["semio-framework-dsl-record"]),
  depLine("semio-framework-dsl-record-derive", OWNER, depDirs["semio-framework-dsl-record-derive"]),
  "semio-framework-os-flow-contract = { workspace = true }",
  "semio-framework-os-kernel = { workspace = true }",
  "semio-framework-os-playbook-contract = { workspace = true }",
  "semio-framework-value = { workspace = true }",
];
const depNames = ["semio-framework-diagnostic", "semio-framework-dsl-record", "semio-framework-dsl-record-derive", "semio-framework-os-flow-contract", "semio-framework-os-kernel", "semio-framework-os-playbook-contract", "semio-framework-pack", "semio-framework-value"];
createCrate({dir: OWNER, name: NAME, description: "Procedural generation native codec: the seven shared DSL twins and the literal eight-field native document mirrors generic over the generation 2d and 3d snapshots", role: "library", libPath: "../../🦀️.rs", workspaceRel: rel(OWNER, CORE), dependencies, framework: false});
membersInsert(`${CORE}/Cargo.toml`, OWNER.slice(CORE.length + 1));

const created = [{name: NAME, version: "0.1.0", deps: depNames}];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
for (const [art, pkg] of [["🌀️generation2d", "semio-s-artifact-procedural-2d-io-text"], ["🧊️generation3d", "semio-s-artifact-procedural-3d-io-text"]] as const) {
  const A = `${P}/🗿️artifacts/${art}`;
  const text = `${A}/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📦️packages/🦀️rust`;
  workspaceDependencyTable(`${A}/Cargo.toml`, NAME, rel(A, OWNER));
  addToSection(`${text}/Cargo.toml`, "[dependencies]", `${NAME} = { workspace = true }`);
  edges.push({consumer: packageNameOf(`${text}/Cargo.toml`), dep: NAME, dev: false});
}
const coreLock = `${CORE}/Cargo.lock`;
lockEnsure(coreLock, NAME, "0.1.0", [], []);
const coreEdges = depNames.map(dep => ({consumer: NAME, dep, dev: false}));
const coreResult = propagateEdges(coreEdges, []);
log(`core lock edges: ${coreResult.changed.join(",")} skipped=${coreResult.skipped.length}`);
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
for (const s of result.skipped) log(`  skipped ${s}`);
log(DRY ? "dry run" : "written");
void read; void put; void OSM;
