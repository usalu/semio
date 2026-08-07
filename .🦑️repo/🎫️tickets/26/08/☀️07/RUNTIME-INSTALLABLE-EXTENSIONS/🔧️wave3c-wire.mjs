/**
 * Wave 3.c — wire glue, fix extension imports, update callers, clean stale refs.
 */
import fs from "fs";
import path from "path";

const REPO = "/Users/ueli/Documents/semio";
const TICKET = path.join(REPO, ".🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS");
const paths = JSON.parse(fs.readFileSync(path.join(TICKET, "wave3c-paths.json"), "utf8"));

function replaceOnce(file, oldStr, newStr, label) {
  const text = fs.readFileSync(file, "utf8");
  if (!text.includes(oldStr)) {
    console.log("SKIP (not found):", label, file);
    return false;
  }
  const next = text.replace(oldStr, newStr);
  if (next === text) {
    console.log("SKIP (unchanged):", label);
    return false;
  }
  fs.writeFileSync(file, next);
  console.log("OK:", label);
  return true;
}

function replaceAll(file, oldStr, newStr, label) {
  const text = fs.readFileSync(file, "utf8");
  if (!text.includes(oldStr)) {
    console.log("SKIP (not found):", label);
    return 0;
  }
  const next = text.split(oldStr).join(newStr);
  fs.writeFileSync(file, next);
  const count = text.split(oldStr).length - 1;
  console.log("OK:", label, "x", count);
  return count;
}

// --- 1. Fix extension imports: wildcard + sdk helpers ---
{
  const extFile = path.join(paths.brepExtRoot, "🦀️component.rs");
  let ext = fs.readFileSync(extFile, "utf8");
  const oldUse = `use flow_extension_sdk::brep_geometry::{
    classify_number, decode_base64, dispose_geometry, domain_span, encode_base64, export_solid_json, geometry_channel,
    geometry_dict, import_solid_json, kernel, kind_label, list_channel, list_indices, map_kernel_error, mesh_cache,
    number_channel, number_dictionary, out_curve, out_face, out_solid, out_wire, point_channel, point_dictionary,
    points_to_grid, read_channel_number, read_geometry, read_geometry_list, read_list, read_nested_point_lists,
    read_optional_geometry, read_point_list, read_text, read_xyz, read_xyz_dict, retain_geometry_handles,
    tessellate_geometry, text_dictionary, vector_channel, vector_dictionary, wire_from_points, with_kernel,
    with_kernel_read,
};`;
  const newUse = `use flow_extension_sdk::brep_geometry::*;
use flow_extension_sdk::{build_manifest_json, evaluate_json};`;
  if (ext.includes(oldUse)) {
    ext = ext.replace(oldUse, newUse);
    console.log("OK: extension wildcard imports");
  } else if (!ext.includes("use flow_extension_sdk::brep_geometry::*;")) {
    // try softer match
    ext = ext.replace(/use flow_extension_sdk::brep_geometry::\{[\s\S]*?\};/, newUse);
    console.log("OK: extension imports via regex");
  } else {
    console.log("SKIP: extension imports already updated");
  }
  // Remove unused heavy imports if any cause warnings later — keep for now
  // Tests module already has `use flow_extension_sdk::{build_manifest_json, evaluate_json};` — may duplicate
  ext = ext.replace(
    /mod tests \{\n    use super::\*;\n    use flow_extension_sdk::\{build_manifest_json, evaluate_json\};/,
    "mod tests {\n    use super::*;",
  );
  fs.writeFileSync(extFile, ext);
}

// --- 2. Wire glue.rs: add brep_geometry, remove brep extension mod + pub use ---
{
  const glue = paths.glueFile;
  let text = fs.readFileSync(glue, "utf8");
  const before = text;

  // Remove extern crate self as flow_extension_brep if present — keep for compatibility? User wants re-point.
  // Keep alias for transitional? No — update callers.

  // Remove brep path mod inside extensions
  text = text.replace(/\n\s*#\[path = "\.\.\/\.\.\/\.\.\/\u{1F9E9}\u{FE0F}extensions\/\u{1F4D0}\u{FE0F}brep\/\u{1F9AB}\u{FE0F}component\.rs"\]\n\s*pub mod brep;\n/u, "\n");
  // Fallback without unicode escapes — read actual file content pattern
  if (text === before) {
    // line-based removal
    const lines = text.split("\n");
    const out = [];
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      if (line.includes("extensions/") && line.includes("brep") && line.includes("path")) {
        // skip this and next pub mod brep
        if (i + 1 < lines.length && lines[i + 1].includes("pub mod brep")) {
          i++;
          continue;
        }
      }
      if (line.trim() === "pub use extensions::brep::*;") continue;
      out.push(line);
    }
    text = out.join("\n");
  }

  // Add brep_geometry path mod near top after core
  if (!text.includes("brep_geometry")) {
    const coreUse = "pub use core::*;";
    const insert = `pub use core::*;

#[path = "../../\u{1F9E9}\u{FE0F}core/\u{1F4D0}\u{FE0F}brep-geometry/\u{1F9AB}\u{FE0F}component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};
`;
    // Use actual path from paths.geometryPath relative to glue
    const rel = path.relative(path.dirname(glue), paths.geometryPath).split(path.sep).join("/");
    const insert2 = `pub use core::*;

#[path = "${rel}"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};
`;
    text = text.replace(coreUse, insert2);
  }

  // Remove extern crate self as flow_extension_brep — callers will use crate root / flow_core
  // Keep it temporarily mapped to self so old aliases still resolve side APIs? User asked re-point.
  // procedural glue: `extern crate flow_extension_draw as flow_extension_brep` — still works for tessellate if we pub use at crate root.

  fs.writeFileSync(glue, text);
  console.log("OK: glue.rs wired brep_geometry, removed brep extension mod");
  fs.writeFileSync(path.join(TICKET, "glue-after.rs"), text);
}

// --- 3. install_builtin: remove flow_extension_brep::register ---
{
  const core = paths.coreFile;
  replaceOnce(
    core,
    "    flow_extension_list::register(registry);\n    flow_extension_brep::register(registry);\n    flow_extension_draw::register(registry);\n",
    "    flow_extension_list::register(registry);\n    flow_extension_draw::register(registry);\n",
    "install_builtin remove brep",
  );
}

// --- 4. Replace flow_extension_brep:: side API calls in flow core with crate-root / brep_geometry ---
{
  const core = paths.coreFile;
  // Within semio-framework-os-flow, flow_extension_brep is extern crate self alias
  // After removing alias, use brep_geometry:: or direct crate functions
  replaceAll(core, "flow_extension_brep::retain_geometry_handles", "crate::retain_geometry_handles", "core retain");
  replaceAll(core, "flow_extension_brep::dispose_geometry", "crate::dispose_geometry", "core dispose");
  replaceAll(core, "flow_extension_brep::tessellate_geometry", "crate::tessellate_geometry", "core tessellate");
  // register in tests — must move or use extension; for host tests that need brep ops, document blocker
  // Replace register calls in fixture tests: leave a note — search remaining
  const text = fs.readFileSync(core, "utf8");
  const hits = [];
  text.split("\n").forEach((l, i) => {
    if (l.includes("flow_extension_brep")) hits.push(`${i + 1}: ${l.trim()}`);
  });
  fs.writeFileSync(path.join(TICKET, "core-remaining-brep-refs.txt"), hits.join("\n"));
  console.log("remaining flow_extension_brep in core:", hits.length);
}

// --- 5. procedural3d caller ---
{
  const p3d = path.join(REPO, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌊️procedural3d/⚙️engine/🦀️component.rs");
  // resolve actual path
  const procArt = path.join(REPO, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts");
  const p3dDir = path.join(procArt, fs.readdirSync(procArt).find((n) => n.includes("procedural3d")));
  const eng = path.join(p3dDir, "⚙️engine", fs.readdirSync(path.join(p3dDir, "⚙️engine")).find((n) => n.includes("component")));
  replaceOnce(eng, "use flow_extension_brep::tessellate_geometry;", "use flow_extension_brep::tessellate_geometry; // crate-root re-export via flow alias", "p3d import note");
  // Actually procedural aliases flow_extension_draw as flow_extension_brep — crate root still exports tessellate_geometry after our pub use. No change needed for import path!
  // Update docstring refs
  replaceAll(eng, "`flow_extension_brep::tessellate_geometry`", "`tessellate_geometry` (flow core brep geometry session)", "p3d docstring");
  console.log("procedural3d engine:", eng);
}

// --- 6. playbook caller ---
{
  const play = paths.brepExtRoot; // wrong
  const playRoot = path.join(REPO, "✏️s/🔌️plugins/📖️playbook");
  function findFile(dir, pred) {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory() && !["node_modules", "target"].includes(e.name)) {
        const hit = findFile(p, pred);
        if (hit) return hit;
      } else if (pred(p)) return p;
    }
    return null;
  }
  const playComp = findFile(playRoot, (p) => p.includes("extensions") && p.includes("procedural") && p.endsWith("component.rs") && !p.includes("packages"));
  console.log("playbook comp", playComp);
  if (playComp) {
    // Keep import from flow_extension_brep alias — still works via pub use on flow crate root
    replaceAll(playComp, "`flow_extension_brep`", "`flow` brep geometry session", "playbook docs");
  }
}

console.log("phase A done");
