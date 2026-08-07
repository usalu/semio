import fs from "fs";
import path from "path";

const TICKET = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/26/08/☀️07/RUNTIME-INSTALLABLE-EXTENSIONS";
const paths = JSON.parse(fs.readFileSync(path.join(TICKET, "wave3c-paths.json"), "utf8"));

const geo = fs.readFileSync(paths.geometryPath, "utf8");
const ext = fs.readFileSync(path.join(paths.brepExtRoot, "🦀️component.rs"), "utf8");
const backup = fs.readFileSync(path.join(TICKET, "brep-original-backup.rs"), "utf8");

console.log("=== geometry pub fns ===");
[...geo.matchAll(/^pub fn (\w+)/gm)].forEach((m) => console.log(m[1]));
console.log("=== geometry private fns ===");
[...geo.matchAll(/^fn (\w+)/gm)].forEach((m) => console.log(m[1]));

// Symbols used in extension that might be missing from imports
const used = new Set();
for (const m of ext.matchAll(/\b([a-z_][a-zA-Z0-9_]*)\s*\(/g)) used.add(m[1]);

const imported = new Set();
const useBlock = ext.match(/use flow_extension_sdk::brep_geometry::\{([\s\S]*?)\};/);
if (useBlock) {
  useBlock[1].split(",").forEach((s) => {
    const n = s.trim().split(/\s+/).pop();
    if (n) imported.add(n);
  });
}
console.log("\nimported count", imported.size);

// Find fn definitions in original between helpers and primitives that aren't in geo or ops as def
const helpersEnd = backup.indexOf("// #endregion 🔖️Helpers") + "// #endregion 🔖️Helpers".length;
const primStart = backup.indexOf("// #region 🔖️Primitives");
const between = backup.slice(helpersEnd, primStart);
console.log("\n=== between helpers and primitives ===");
console.log(between.slice(0, 3000));
fs.writeFileSync(path.join(TICKET, "brep-between-helpers-prims.rs"), between);

// Check if extension defines or imports: reg_geo, geometry_schema, etc.
for (const sym of [
  "reg_geo",
  "geometry_schema",
  "topology_element_schema",
  "brep_schema",
  "text_schema",
  "topology_output",
  "BrepDeconstruct",
  "BoxPrim",
  "evaluate_json",
  "build_manifest_json",
  "base64",
]) {
  const defined = new RegExp(`(fn|struct|enum|type|use .*::)${sym}\\b`).test(ext) || ext.includes(`fn ${sym}`) || ext.includes(`struct ${sym}`);
  const inGeo = geo.includes(`fn ${sym}`) || geo.includes(`pub fn ${sym}`);
  const inImport = imported.has(sym);
  console.log(sym, { definedInExt: defined || ext.includes(`struct ${sym}`), inGeo, inImport, inExtText: ext.includes(sym) });
}

// Cargo.toml of extension
console.log("\n=== ext Cargo.toml ===");
console.log(fs.readFileSync(path.join(paths.brepRust, "Cargo.toml"), "utf8"));
