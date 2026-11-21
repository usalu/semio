import { dirname, resolve } from "node:path";
import { measureFits, measureRustCrate } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
import { CRATE_HARD_BUDGET } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts";
const manifests = [
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/〰️analytic-curves/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/📦️packages/🦀️rust/Cargo.toml",
"🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/📦️packages/🦀️rust/Cargo.toml",
"🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧪️tests/📦️targets/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✂️intersect/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/🏗️solid-transaction/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🏗️solid-topology/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🧱️mesh-support/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✏️mesh-edit/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🕸️node-graph/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🕸️graph/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔗️engine-attach/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🗺️engine-registry/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🖱️ui-engine/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/📦️packages/🦀️rust/Cargo.toml",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📦️packages/🦀️rust/Cargo.toml",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml"
];
const root = process.cwd();
const cfg = process.platform === "win32" ? ["windows", "target_os=windows"] : ["unix", "target_os=" + (process.platform === "darwin" ? "macos" : process.platform)];
cfg.push("target_arch=" + (process.arch === "arm64" ? "aarch64" : process.arch === "x64" ? "x86_64" : process.arch), "debug_assertions");
for (const manifest of manifests) {
  const directory = dirname(resolve(root, manifest));
  const text = await Bun.file(manifest).text();
  const owner = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!owner) throw Error("Package name missing in " + manifest);
  const units = text.split("[[test]]").slice(1).flatMap(section => {
    const name = section.match(/^name = "(wgpu_node_graph[^"]*|preview-channel-geometry|original-brep-booleans|original-brep-queries|original-skew-boxes|mesh-modeling)"$/m)?.[1];
    const entry = section.match(/^path = "([^"]+)"$/m)?.[1];
    return name && entry ? [{ name, entry, withTest: true }] : [];
  });
  const entry = text.split("[lib]")[1]?.split("[")[0]?.match(/^path = "([^"]+)"/m)?.[1] ?? "🦀️.rs";
  units.unshift({name: owner, entry, withTest: false});
  if (owner === "semio-s-artifact-procedural-3d-preview-eval") units.push({ name: owner + "-unit", entry, withTest: true });
  for (const unit of units) {
    const result = await measureRustCrate(root, resolve(directory, unit.entry), unit.withTest, new Map(), directory, cfg);
    const { files, bytes, lines } = result;
    const fits = measureFits(result, CRATE_HARD_BUDGET);
    console.log(JSON.stringify({owner:unit.name, files, bytes, lines, fits}));
    if (!fits) process.exitCode = 1;
  }
}
