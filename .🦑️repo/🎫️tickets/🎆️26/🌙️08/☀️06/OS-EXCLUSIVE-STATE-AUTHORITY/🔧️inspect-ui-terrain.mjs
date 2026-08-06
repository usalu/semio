import fs from "fs";
import path from "path";

function find(root, pred, max = 80, depth = 0, out = []) {
  if (out.length >= max || depth > 14) return out;
  let ents;
  try {
    ents = fs.readdirSync(root, { withFileTypes: true });
  } catch {
    return out;
  }
  for (const e of ents) {
    if (e.name === "target" || e.name === "node_modules" || e.name.startsWith(".")) continue;
    const p = path.join(root, e.name);
    try {
      if (pred(p, e)) out.push(p);
    } catch {}
    if (e.isDirectory()) find(p, pred, max, depth + 1, out);
  }
  return out;
}

const uiRoot = "🧰️framework/🔨️modules/🖱️ui";
console.log("ui exists", fs.existsSync(uiRoot));
const glues = find(uiRoot, (p, e) => e.isFile() && e.name === "📦️glue.rs");
console.log("UI glues:", glues);
for (const g of glues.slice(0, 5)) {
  const t = fs.readFileSync(g, "utf8");
  console.log("\n====", g);
  t.split("\n").forEach((l, i) => {
    if (/pub mod |extern crate|^mod |wgpu|GpuContext|draw_text|widgets|HitKind|mesh_content|paint_selection|feature/.test(l))
      console.log(i + 1 + ":", l.slice(0, 160));
  });
}

const cargos = find(uiRoot, (p, e) => e.isFile() && e.name === "Cargo.toml", 30);
for (const c of cargos) {
  const t = fs.readFileSync(c, "utf8");
  const name = t.match(/name\s*=\s*"([^"]+)"/);
  console.log("CARGO", c, "->", name && name[1], "features:", (t.match(/\[features\][\s\S]*?(?=\n\[|$)/) || [""])[0].slice(0, 200).replace(/\n/g, " | "));
}

console.log("\n--- search TerrainSessionCore ---");
const rsHits = find(
  "🧰️framework",
  (p, e) => e.isFile() && e.name.endsWith(".rs") && fs.readFileSync(p, "utf8").includes("TerrainSessionCore"),
  20
);
console.log(rsHits);

console.log("\n--- search framework_surface ---");
const cargoHits = find(
  "🧰️framework",
  (p, e) => e.isFile() && e.name === "Cargo.toml" && /surface.?terrain|framework.surface/i.test(fs.readFileSync(p, "utf8")),
  20
);
console.log(cargoHits);

// math graph WireNode
console.log("\n--- WireNode in math ---");
const wireHits = find(
  "🧰️framework/🔨️modules/� Combined",
  () => false,
  1
);
