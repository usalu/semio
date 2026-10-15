import { readFileSync } from "node:fs";
import { join } from "node:path";
import { mcpCredentialSourceOrderConforms } from "../../../../../../../../../🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";
import { readModuleTreeSource } from "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const MCP = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/";
const rd = (p: string) => readFileSync(join(repoRoot, p), "utf8");
const base = {
  entrypoint: rd(MCP + "🏗️bootstrap/🦀️.rs"),
  remote: rd(MCP + "🏠️workspace/🔗️remote/🦀️.rs"),
  directory: rd("🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"),
  runner: rd(MCP + "📦️packages/🦀️rust/📜️script.ts"),
  project: rd("🌎️hub/📦️packages/🦀️rust/📋️project.json"),
};
const old = readFileSync(new URL("./orig-workspace.rs.txt", import.meta.url), "utf8");
const tree = readModuleTreeSource(join(repoRoot, MCP + "🏠️workspace/🦀️.rs"));
const single = rd(MCP + "🏠️workspace/🦀️.rs");
console.log("old-monolith conforms:", mcpCredentialSourceOrderConforms({ ...base, workspace: old }));
console.log("parent-file-only conforms (reader as it was):", mcpCredentialSourceOrderConforms({ ...base, workspace: single }));
console.log("module-tree conforms:", mcpCredentialSourceOrderConforms({ ...base, workspace: tree }));
console.log("tree bytes", tree.length, "old bytes", old.length);

const hostRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs");
const hostTree = readModuleTreeSource(hostRoot);
const hostParent = readFileSync(hostRoot, "utf8");
const hostTests = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs"), "utf8");
const law = "svg_path_extraction_preserves_transformed_geometry";
console.log("space reader: tree+test includes law fn:", `${hostTree}\n${hostTests}`.includes(`fn ${law}()`), "| parent-only includes:", hostParent.includes(`fn ${law}()`));
for (const m of ["host", "backbone", "instance", "workflow", "registry"]) {
  console.log("stdio reader", m, "new-regex:", new RegExp(`^pub mod ${m};`, "mu").test(hostParent), "old-regex:", new RegExp(`^pub mod ${m} \\{`, "mu").test(hostParent));
}
const monolith = readFileSync(new URL("./orig-host.rs.txt", import.meta.url), "utf8");
console.log("host tree has every monolith module:", ["host", "backbone", "instance", "workflow", "registry", "media_export_raster", "media_export_simple", "codec_abi"].every((m) => hostTree.includes(`pub mod ${m}`) || monolith.includes(`pub mod ${m} {`)));
const need = ["fn registry_export_media_via_composer_registry", "fn negotiate_wire_format", "pub fn rasterize_svg_to_png_base64"];
console.log("host tree contains:", need.map((s) => hostTree.includes(s)));

import { sliceModuleTreeSource } from "../../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const wsRoot = join(repoRoot, MCP + "🏠️workspace/🦀️.rs");
console.log("snapshot reader: parent-only indexOf(impl ... ProbeSnapshot) =", single.indexOf("impl store::ArtifactPackReceiving for ProbeSnapshot"), "(old reader would slice an empty string)");
const slice = sliceModuleTreeSource(wsRoot, "impl store::ArtifactPackReceiving for ProbeSnapshot", "/// 🌉️ Projects the first-party tree");
console.log("snapshot reader: tree slice bytes", slice.length, "has probe_json::receive_snapshot:", slice.includes("probe_json::receive_snapshot(text,owner)"), "has checkpoint:", slice.includes("owner.native().checkpoint()?"));
console.log("old monolith slice has same markers:", old.slice(old.indexOf("impl store::ArtifactPackReceiving for ProbeSnapshot"), old.indexOf("/// 🌉️ Projects the first-party tree")) === slice);
