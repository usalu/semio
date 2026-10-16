// 🏗️ Four single-owner crates for sources that two crates mounted: compiler resources (proc-macro helper), routing workspace identity, ui paint-policy scanner, dsl-record ownership oracle.
import {DRY, log, read, put, replaceOnce, addToSection, packageNameOf, propagateEdges, createCrate, membersInsert, depLine, abs} from "./lib.ts";
import {relative} from "node:path";
const FW = "🧰️framework";
const OSM = `${FW}/🛍️products/💻️os/🔨️modules`;
const rel = (from: string, to: string) => relative(abs(from), abs(to)).replaceAll("\\", "/");
type Consumer = {dir: string; dev: boolean; edits: {file: string; find: string; replace: string}[]};
type Spec = {dir: string; name: string; id: string; description: string; role: string; deps: (dir: string) => string[]; consumers: Consumer[]; depsOfBlock: string[]};
const specs: Spec[] = [
  {
    dir: `${FW}/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/📦️packages/🦀️rust`,
    name: "semio-framework-native-build-compiler-resources", id: "native-build-compiler-resources", role: "framework",
    description: "Proc-macro filesystem input capture: reads, directory listings and genuine compiler dependency registration for original resources.",
    deps: dir => [depLine("semio-framework-hash", dir, `${FW}/🔨️modules/🔏️hash/📦️packages/🦀️rust`)],
    depsOfBlock: ["semio-framework-hash"],
    consumers: [
      {dir: `${FW}/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust`, dev: false, edits: [{file: `${FW}/🔨️modules/🌱️value/✨️derive/🦀️.rs`, find: `#[path = "../../🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs"]\nmod compiler_resources;`, replace: `use semio_framework_native_build_compiler_resources as compiler_resources;`}]},
      {dir: `${OSM}/🗣️dsl/✨️derive/📦️packages/🦀️rust`, dev: false, edits: [{file: `${OSM}/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs`, find: `#[path = "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs"]\nmod compiler_resources;`, replace: `use semio_framework_native_build_compiler_resources as compiler_resources;`}]},
    ],
  },
  {
    dir: `${FW}/🔨️modules/🏃️process/🧭️routing/🏠️workspace/📦️packages/🦀️rust`,
    name: "semio-framework-process-routing-workspace", id: "process-routing-workspace", role: "framework",
    description: "Build-tool-neutral workspace identity with caller-owned JSON decoding.",
    deps: () => [], depsOfBlock: [],
    consumers: [
      {dir: `${OSM}/🏃️run/📦️packages/🦀️rust`, dev: false, edits: [{file: `${OSM}/🏃️run/🏗️bootstrap/🦀️.rs`, find: `#[path = "../../../../../🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs"]\nmod workspace_identity;`, replace: `use semio_framework_process_routing_workspace as workspace_identity;`}]},
      {dir: `${OSM}/🏪️store/🔄️sync/📦️packages/🦀️rust`, dev: false, edits: [{file: `${OSM}/🏪️store/🔄️sync/🦀️.rs`, find: `#[path = "../../../../../🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs"]\nmod workspace_identity;`, replace: `use semio_framework_process_routing_workspace as workspace_identity;`}]},
      {dir: `${FW}/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust`, dev: false, edits: [{file: `${FW}/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️runner/🦀️.rs`, find: `#[path = "../../../../../🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs"]\nmod workspace_identity;`, replace: `use semio_framework_process_routing_workspace as workspace_identity;`}]},
    ],
  },
  {
    dir: `${FW}/🔨️modules/🖱️ui/🧪️testing/🎨️paint-policy/📦️packages/🦀️rust`,
    name: "semio-framework-ui-testing-paint-policy", id: "ui-testing-paint-policy", role: "test",
    description: "Shared test mechanism for owned renderer paint policies: scans sources for hand-written colour constructors.",
    deps: () => [], depsOfBlock: [],
    consumers: [
      {dir: `${FW}/🔨️modules/🖱️ui/📦️packages/🦀️rust`, dev: true, edits: [{file: `${FW}/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-theme-token-parity/🦀️.rs`, find: `#[path = "../../🧪️testing/🎨️paint-policy/🦀️.rs"]\nmod paint_policy;`, replace: `use semio_framework_ui_testing_paint_policy as paint_policy;`}]},
      {dir: `${OSM}/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`, dev: true, edits: [{file: `${OSM}/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️wgpu-paint-policy/🦀️.rs`, find: `#[path = "../../../../../../../🔨️modules/🖱️ui/🧪️testing/🎨️paint-policy/🦀️.rs"]\nmod paint_policy;`, replace: `use semio_framework_ui_testing_paint_policy as paint_policy;`}]},
    ],
  },
  {
    dir: `${FW}/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🏛️ownership/🧪️tests/📦️packages/🦀️rust`,
    name: "semio-framework-dsl-record-ownership-oracle", id: "dsl-record-ownership-oracle", role: "test",
    description: "Independent serde_json observations of explicitly owned record fields and record schemas.",
    deps: dir => [depLine("semio-framework-dsl-record", dir, `${FW}/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust`), `serde_json = "1.0.140"`],
    depsOfBlock: ["semio-framework-dsl-record", "serde_json"],
    consumers: [
      {dir: `${FW}/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust`, dev: true, edits: [{file: `${FW}/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪆️binding/🦀️.rs`, find: `#[path="../../../../🗣️dsl/🧬️schema/🪆️binding/🏛️ownership/🧪️tests/🦀️.rs"]\nmod oracle;`, replace: `use semio_framework_dsl_record_ownership_oracle as oracle;`}]},
      {dir: `${FW}/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust`, dev: true, edits: [{file: `${FW}/🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🏛️ownership/🦀️.rs`, find: `#[path="../../../../../../🗣️dsl/🧬️schema/🪆️binding/🏛️ownership/🧪️tests/🦀️.rs"]\nmod oracle;`, replace: `use semio_framework_dsl_record_ownership_oracle as oracle;`}]},
    ],
  },
];
const created: {name: string; version: string; deps: string[]}[] = [];
const edges: {consumer: string; dep: string; dev: boolean}[] = [];
for (const spec of specs) {
  const wsRoot = spec.dir.startsWith(`${FW}/🔨️modules`) ? FW : "";
  createCrate({dir: spec.dir, name: spec.name, description: spec.description, role: spec.role, libPath: "../../🦀️.rs", workspaceRel: rel(spec.dir, wsRoot || "."), id: spec.id, dependencies: spec.deps(spec.dir), framework: true, ownerName: `@semio-tech/${spec.id}-rs`});
  membersInsert(`${FW}/Cargo.toml`, spec.dir.slice(FW.length + 1));
  created.push({name: spec.name, version: "0.1.0", deps: spec.depsOfBlock});
  for (const consumer of spec.consumers) {
    const section = consumer.dev ? "[dev-dependencies]" : "[dependencies]";
    addToSection(`${consumer.dir}/Cargo.toml`, section, depLine(spec.name, consumer.dir, spec.dir));
    for (const edit of consumer.edits) replaceOnce(edit.file, edit.find, edit.replace);
    edges.push({consumer: packageNameOf(`${consumer.dir}/Cargo.toml`), dep: spec.name, dev: consumer.dev});
  }
}
const result = propagateEdges(edges, created);
log(`locks changed: ${result.changed.length}; skipped: ${result.skipped.length}`);
for (const l of result.skipped.slice(0, 12)) log(`  skip ${l}`);
