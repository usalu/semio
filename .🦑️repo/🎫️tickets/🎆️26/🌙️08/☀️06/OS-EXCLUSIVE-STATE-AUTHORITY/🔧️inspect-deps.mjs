import fs from "fs";
import path from "path";
import { createHash } from "crypto";

const rootComp = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs";
const worldComp = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs";
const a = fs.readFileSync(rootComp);
const b = fs.readFileSync(worldComp);
console.log("root==world?", createHash("sha256").update(a).digest("hex") === createHash("sha256").update(b).digest("hex"));

console.log("\n=== surface cargo ===");
console.log(fs.readFileSync("🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml", "utf8"));

console.log("\n=== ui features ===");
const ui = fs.readFileSync("🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml", "utf8");
const feat = ui.match(/\[features\][\s\S]*?(?=\n\[|$)/);
console.log(feat ? feat[0] : "none");

function find(root, pred, max = 40, depth = 0, out = []) {
  if (out.length >= max || depth > 14) return out;
  let ents;
  try { ents = fs.readdirSync(root, { withFileTypes: true }); } catch { return out; }
  for (const e of ents) {
    if (e.name === "target" || e.name === "node_modules" || e.name.startsWith(".")) continue;
    const p = path.join(root, e.name);
    try { if (pred(p, e)) out.push(p); } catch {}
    if (e.isDirectory()) find(p, pred, max, depth + 1, out);
  }
  return out;
}

const wire = find("🧰️framework/🔨️modules/🧮️math", (p, e) => e.isFile() && e.name.endsWith(".rs") && /pub struct WireNode|pub struct WireEdge|wire_literal_from_dag/.test(fs.readFileSync(p, "utf8")), 20);
console.log("\nWireNode hits", wire);

// How other crates depend on surface
const depHits = find(".", (p, e) => e.isFile() && e.name === "Cargo.toml" && p.includes("framework") && /framework.surface|semio-framework-surface|surface_terrain/.test(fs.readFileSync(p, "utf8")), 30);
console.log("\nsurface dependents", depHits);

// Check kernel for dsl attribute / derive
const kernelCargo = "