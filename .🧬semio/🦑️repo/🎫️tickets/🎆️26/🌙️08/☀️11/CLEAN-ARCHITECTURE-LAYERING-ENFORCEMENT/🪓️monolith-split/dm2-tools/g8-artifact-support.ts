// ⚖️ Artifact test-support crates: lowpoly mutation laws, remodeling mutation vectors, and the framework transient-preparation law.
import {readFileSync} from "node:fs";
import {relative} from "node:path";
import {log, read, put, replaceOnce, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, workspaceDependencyTable, depLine, abs} from "./lib.ts";
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
const TSV = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/dm2-modowners-0.tsv";
const rows = readFileSync(TSV, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const consumersOf = (file: string): string[] => rows.find(r => r[0] === file)![2]!.split(" ; ");
const mountFilesOf = (file: string): string[] => [...new Set(rows.find(r => r[0] === file)![3]!.split(" ; ").flatMap(entry => entry.split("=").slice(1).join("=").split(",")).map(site => site.replace(/:\d+.*$/u, "")).filter(f => f.endsWith(".rs")))];
const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
const KERNEL_FEATURES = `semio-framework-os-kernel = { workspace = true, features = ["protocol-laws", "mutation-testing"] }`;

type Support = {artifact: string; shared: string; name: string; description: string; deps: string[]; depNames: string[]; head?: [string, string]; mounts: {file: string; find: RegExp | string; replace: string}[]; alias: string};
function supportCrate(s: Support): void {
  const dir = s.shared.replace(/\/🦀️\.rs$/u, "") + "/📦️packages/🦀️rust";
  createCrate({dir, name: s.name, description: s.description, role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(dir, s.artifact), dependencies: s.deps, framework: false});
  membersInsert(`${s.artifact}/Cargo.toml`, rel(s.artifact, dir));
  workspaceDependencyTable(`${s.artifact}/Cargo.toml`, s.name, rel(s.artifact, dir));
  created.push({name: s.name, version: "0.1.0", deps: s.depNames});
  if (s.head) {
    const text = read(s.shared);
    if (text.split(s.head[0]).length !== 2) throw new Error(`head anchor in ${s.shared}`);
    put(s.shared, text.replace(s.head[0], s.head[1]));
  }
  for (const dir2 of consumersOf(s.shared)) {
    addToSection(`${dir2}/Cargo.toml`, "[dev-dependencies]", `${s.name} = { workspace = true }`);
    edges.push({consumer: packageNameOf(`${dir2}/Cargo.toml`), dep: s.name, dev: true});
  }
  for (const mount of s.mounts) {
    const text = read(mount.file);
    if (typeof mount.find === "string") replaceOnce(mount.file, mount.find, mount.replace);
    else {
      if (!mount.find.test(text)) throw new Error(`mount not found in ${mount.file}`);
      put(mount.file, text.replace(mount.find, mount.replace));
    }
  }
}

const LOW = "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly";
const LOW_M = `${LOW}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`;
supportCrate({
  artifact: LOW, shared: `${LOW_M}/🧪️tests/⚖️laws/🦀️.rs`, name: "semio-s-artifact-lowpoly-lowpoly-mut-laws",
  description: "The laws every committed quintet of a single-row-inverse lowpoly leaf holds, written once and called by each scenario.",
  deps: [KERNEL_FEATURES, "semio-framework-value = { workspace = true }", "semio-s-artifact-lowpoly-lowpoly-model = { workspace = true }", "semio-s-artifact-lowpoly-lowpoly-model-diff = { workspace = true }", "semio-s-artifact-lowpoly-lowpoly-mutations = { workspace = true }", "serde_json = { workspace = true }"],
  depNames: ["semio-framework-os-kernel", "semio-framework-value", "semio-s-artifact-lowpoly-lowpoly-model", "semio-s-artifact-lowpoly-lowpoly-model-diff", "semio-s-artifact-lowpoly-lowpoly-mutations", "serde_json"],
  head: ["use semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff;", "use semio_framework_os_kernel as protocol;\nuse semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff;"],
  alias: "laws",
  mounts: ["🎨️paint", "🕸️geometry", "🧊️objects"].map(group => ({file: `${LOW_M}/${group}/🧪️tests/🦀️.rs`, find: `#[path = "../../🧪️tests/⚖️laws/🦀️.rs"]\npub mod laws;`, replace: "use semio_s_artifact_lowpoly_lowpoly_mut_laws as laws;"})),
});

const REM = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling";
const REM_M = `${REM}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`;
supportCrate({
  artifact: REM, shared: `${REM_M}/🧪️tests/🔬️vectors/🦀️.rs`, name: "semio-s-artifact-remodel-remodeling-mut-vectors",
  description: "The law library every committed remodeling mutation vector is checked with.",
  deps: [KERNEL_FEATURES, "semio-framework-diagnostic = { workspace = true }", "semio-framework-pack-json = { workspace = true }", "semio-framework-value = { workspace = true }", "semio-s-artifact-remodel-remodeling-model = { workspace = true }", "semio-s-artifact-remodel-remodeling-model-diff = { workspace = true }", "semio-s-artifact-remodel-remodeling-mutations = { workspace = true }"],
  depNames: ["semio-framework-diagnostic", "semio-framework-os-kernel", "semio-framework-pack-json", "semio-framework-value", "semio-s-artifact-remodel-remodeling-model", "semio-s-artifact-remodel-remodeling-model-diff", "semio-s-artifact-remodel-remodeling-mutations"],
  head: ["use semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};", "use semio_framework_os_kernel as protocol;\nuse semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};"],
  alias: "vectors",
  mounts: mountFilesOf(`${REM_M}/🧪️tests/🔬️vectors/🦀️.rs`).map(file => ({file, find: /#\[path = "[^"]*🧪️tests\/🔬️vectors\/🦀️\.rs"\]\nmod vectors;/u, replace: "use semio_s_artifact_remodel_remodeling_mut_vectors as vectors;"})),
});
replaceOnce(`${REM_M}/🧪️tests/🔬️vectors/🦀️.rs`, `.join("../../../../../🧪️tests/🧫️fixtures/🧬️mutations")`, `.join("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations")`);

const FW_LAW = "🧰️framework/🔨️modules/🧪️test/⚖️transient-preparation-law";
const LAW_DIR = `${FW_LAW}/📦️packages/🦀️rust`;
const lawName = "semio-framework-test-transient-preparation-law";
createCrate({dir: LAW_DIR, name: lawName, description: "Window transient preparation law: a transient owner's own bundle prepares a mutation, its inverse and the mutation again, then retires its state through the bundle's own controls.", role: "test", libPath: "../../🦀️.rs", workspaceRel: rel(LAW_DIR, "🧰️framework"), id: "test-transient-preparation-law", dependencies: [depLine("semio-framework-os-kernel", LAW_DIR, "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", `, features = ["protocol-laws", "mutation-testing"]`), depLine("semio-framework-plugin", LAW_DIR, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", `, features = ["artifact-app-testing"]`), depLine("semio-framework-value", LAW_DIR, "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust")], framework: true, ownerName: "@semio-tech/test-transient-preparation-law-rs"});
membersInsert("🧰️framework/Cargo.toml", LAW_DIR.slice("🧰️framework/".length));
created.push({name: lawName, version: "0.1.0", deps: ["semio-framework-os-kernel", "semio-framework-plugin", "semio-framework-value"]});
const lawConsumers = consumersOf(`${FW_LAW}/🦀️.rs`);
for (const dir of lawConsumers) {
  addToSection(`${dir}/Cargo.toml`, "[dev-dependencies]", depLine(lawName, dir, LAW_DIR));
  edges.push({consumer: packageNameOf(`${dir}/Cargo.toml`), dep: lawName, dev: true});
}
const lawMount = /#\[path = "[^"]*⚖️transient-preparation-law\/🦀️\.rs"\]\nmod transient_preparation_law;/u;
const lawFiles = (rows.find(r => r[0] === `${FW_LAW}/🦀️.rs`)![3]!).split(" ; ").flatMap(entry => entry.split("=").slice(1).join("=").split(",")).map(s => s.replace(/:\d+.*$/u, "")).filter(f => f.endsWith(".rs"));
for (const file of [...new Set(lawFiles)]) {
  const text = read(file);
  if (!lawMount.test(text)) throw new Error(`no law mount in ${file}`);
  put(file, text.replace(lawMount, "use semio_framework_test_transient_preparation_law as transient_preparation_law;"));
}
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.changed) log(`  ${l}`);
