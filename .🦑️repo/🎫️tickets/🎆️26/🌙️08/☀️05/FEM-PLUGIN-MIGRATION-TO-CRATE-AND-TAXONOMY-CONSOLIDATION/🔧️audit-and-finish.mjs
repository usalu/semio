import { readFileSync, writeFileSync, existsSync, readdirSync, statSync, rmSync, unlinkSync } from "fs";
import { join, relative } from "path";

const root = "/Users/ueli/Documents/semio";
const fem = join(root, "✏️s/🔌️plugins/🏗️fem");
const ticket = process.env.TICKET;

function walk(dir, acc = []) {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (n === "target" || n === "node_modules") continue;
      walk(p, acc);
    } else acc.push(p);
  }
  return acc;
}

console.log("=== core ui_wgpu lines ===");
for (const p of [
  join(root, "🧰️framework/📦️packages/🦀️rust/Cargo.toml"),
  join(root, "🧰️framework/⚡️implementations/🦀️rust/Cargo.toml"),
]) {
  if (!existsSync(p)) { console.log("MISSING", p); continue; }
  const m = readFileSync(p, "utf8").match(/^ui_wgpu = \{[^}]+\}/m);
  console.log(relative(root, p), "=>", m?.[0]);
}

console.log("\n=== fem member in root? ===");
const cargo = readFileSync(join(root, "Cargo.toml"), "utf8");
const femLines = cargo.split("\n").filter((l) => l.includes("🏗️fem"));
console.log(femLines.join("\n") || "(none)");

console.log("\n=== implementations dirs ===");
const impls = [];
function walkDirs(dir) {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    if (!statSync(p).isDirectory()) continue;
    if (n === "target" || n === "node_modules" || n === "📦️packages") continue;
    if (n === "⚡️implementations") impls.push(relative(fem, p));
    else walkDirs(p);
  }
}
walkDirs(fem);
console.log("count", impls.length);
impls.forEach((i) => console.log(" ", i));

console.log("\n=== flat non-component rs ===");
const flats = walk(fem).filter((p) => p.endsWith(".rs") && !p.endsWith("component.rs") && !p.includes("/📦️packages/") && !p.includes("/⚡️implementations/") && !p.includes("/target/"));
console.log("count", flats.length);
flats.forEach((f) => console.log(" ", relative(fem, f)));

console.log("\n=== fem cargo has overlay? ===");
const femCargo = readFileSync(join(fem, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
console.log("overlay", femCargo.includes("TEMPORARY VERIFICATION OVERLAY"));
console.log("cargo-features at top", femCargo.startsWith("cargo-features"));
