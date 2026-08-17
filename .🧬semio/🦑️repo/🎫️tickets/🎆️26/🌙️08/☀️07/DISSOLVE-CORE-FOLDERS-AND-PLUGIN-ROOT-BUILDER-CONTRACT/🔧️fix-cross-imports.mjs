import fs from "fs";
import path from "path";

const FLOW = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const ALL = [
  ["📄️document", "document"],
  ["📚️catalogue", "catalogue"],
  ["📔️registry", "registry"],
  ["🌉️bridge", "bridge"],
  ["🖥️host", "host"],
  ["🖍️drawing", "drawing"],
  ["🌉️wasm", "wasm_session"],
  ["🌿️vcs", "vcs"],
];

const CROSS = `use crate::document::*;
use crate::catalogue::*;
use crate::registry::*;
use crate::bridge::*;
use crate::host::*;
use crate::drawing::*;
use crate::wasm_session::*;
use crate::vcs::*;
use crate::brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};
`;

for (const [dir, rust] of ALL) {
  const p = path.join(FLOW, dir, "🦀️component.rs");
  let text = fs.readFileSync(p, "utf8");
  if (text.includes("use crate::document::*;")) {
    console.log("skip", dir);
    continue;
  }
  // Insert after serde import block
  const marker = "use serde::{Deserialize, Serialize};\n";
  if (!text.includes(marker)) {
    console.error("no serde marker", dir);
    continue;
  }
  // Exclude self-glob to reduce duplicate definition conflicts
  const cross = CROSS.split("\n")
    .filter((l) => !l.includes(`use crate::${rust}::`))
    .join("\n");
  text = text.replace(marker, marker + "\n" + cross + "\n");
  fs.writeFileSync(p, text);
  console.log("cross-imports", dir);
}
