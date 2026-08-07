import fs from "fs";
import path from "path";

const core = path.join("🧰️framework/🔨️modules/🧩core", "🟦️component.ts");
const lines = fs.readFileSync(core, "utf8").split("\n");
console.log("=== PluginManifest ===");
for (let i = 2240; i < 2320; i++) console.log(`${i + 1}|${lines[i]}`);

console.log("\n=== createExtensionSource region ===");
for (let i = 3380; i < 3660; i++) {
  if (i >= lines.length) break;
  console.log(`${i + 1}|${lines[i]}`);
}

const space = path.join("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space", "🦀️component.rs");
const sl = fs.readFileSync(space, "utf8").split("\n");
console.log("\n=== empty_space_projection_matches_schema ===");
for (let i = 1405; i < 1450; i++) console.log(`${i + 1}|${sl[i]}`);

// shellLabel fallback
function walk(dir, pred, acc = []) {
  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch {
    return acc;
  }
  for (const e of entries) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, pred, acc);
    else if (pred(p, e.name)) acc.push(p);
  }
  return acc;
}
const labelDef = walk("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer", (p, n) => {
  if (!/\.(ts|tsx)$/.test(n)) return false;
  try {
    return /function shellLabel|export function shellLabel|ui\.plugins\.action\.install/.test(fs.readFileSync(p, "utf8"));
  } catch {
    return false;
  }
});
console.log("\nSHELL LABEL DEFS", labelDef);
for (const f of labelDef) {
  const t = fs.readFileSync(f, "utf8");
  const idx = t.indexOf("ui.plugins");
  if (idx >= 0) console.log(f, "has ui.plugins at", idx);
  const idx2 = t.indexOf("function shellLabel");
  if (idx2 >= 0) console.log(f, "\n", t.slice(idx2, idx2 + 600));
  const idx3 = t.indexOf("\"ui.plugins");
  if (idx3 >= 0) {
    // print nearby object keys
    const start = Math.max(0, idx3 - 200);
    console.log("--- keys ---", t.slice(start, idx3 + 800));
  }
}
