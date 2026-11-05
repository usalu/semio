import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
const root = process.cwd(), artifact = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling", subset = artifact + "/🏅️standards/🔖️1/🪆️subsets/✳️any", engine = subset + "/✏️editor/⚙️engine";
const read = path => readFileSync(resolve(root, path), "utf8");
let meshDataSources = 0;
for (const owner of ["🏭️reconstruction", "🥽️mesh"]) {
  const manifest = read(engine + "/" + owner + "/📦️packages/🦀️rust/Cargo.toml");
  assert(manifest.includes("semio-framework-mesh-engine = { workspace = true }"));
  assert(!/^semio-framework =/m.test(manifest));
  const walk = path => { for (const item of readdirSync(resolve(root, path), { withFileTypes: true })) {
    const file = path + "/" + item.name;
    if (item.isDirectory()) walk(file);
    else if (file.endsWith(".rs")) { const source = read(file); assert(!source.includes("semio_framework::"), file); if (source.includes("semio_framework_mesh_engine::MeshData")) meshDataSources++; }
  }};
  walk(engine + "/" + owner);
}
assert.equal(meshDataSources, 5);
const workspace = read(artifact + "/Cargo.toml");
assert(workspace.includes("[workspace.dependencies.semio-framework-mesh-engine]"));
assert(!read(artifact + "/📦️packages/🦀️rust/Cargo.toml").includes("semio-s-plugin-energy-engine"));
assert(!read(artifact + "/🧪️tests/🔬️editor/🦀️.rs").includes("semio_s_plugin_energy_engine"));
const descriptor = JSON.parse(read("🌎️hub/🧩️compositions/📸️remodel/🧩️component/🔣️.json"));
const editor = descriptor.manifest.apps.find(app => app.id === "s.remodel.remodeling@1/*#editor");
assert(editor);
assert.deepEqual(editor.modes.map(mode => mode.id), ["capture", "model", "analyze"]);
assert.deepEqual(editor.windowKinds.map(window => window.id), ["remodeling-main", "remodeling-frames", "remodeling-report"]);
assert.deepEqual(descriptor.manifest.examples.map(example => example.id), ["demo", "demo-session", "synthetic-orbit"]);
for (const item of [...editor.modes, ...editor.windowKinds, ...editor.panelTabs, ...descriptor.manifest.examples]) {
  assert(item.label.native.en.length > 0);
  assert(item.label.native.de.length > 0);
}
const owners = ["🎭️modes/🧊️model/🪟️windows/🧊️model", "🎭️modes/📷️capture/🪟️windows/🖼️frames", "🎭️modes/🔍️analyze/🪟️windows/📊️report"];
for (const [index, owner] of owners.entries()) {
  const source = read(subset + "/✏️editor/" + owner + "/🦀️.rs"), window = editor.windowKinds[index];
  assert(source.includes('"' + window.id + '"'));
  assert(source.includes('"' + window.bodyKey + '"'));
}
const viewSource = read(subset + "/✏️editor/🎛️artifact-editor/🦀️.rs");
for (const owner of ["RemodelingModelWindowConfigOwner", "RemodelingFramesWindowConfigOwner", "RemodelingReportWindowConfigOwner"]) assert(viewSource.includes(owner));
console.log("Integration source checks passed: direct owner dependencies, no energy aliases, descriptor modes/windows/examples/en-de labels and exact window owners.");
