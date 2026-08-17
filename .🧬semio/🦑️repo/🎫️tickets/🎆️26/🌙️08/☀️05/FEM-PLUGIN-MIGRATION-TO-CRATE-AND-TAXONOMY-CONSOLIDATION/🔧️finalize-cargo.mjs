import { readFileSync, writeFileSync, existsSync, readdirSync, statSync, rmSync } from "fs";
import { join, relative } from "path";

const root = "/Users/ueli/Documents/semio";
const fem = join(root, "✏️s/🔌️plugins/🏗️fem");
const femCargoPath = join(fem, "📦️packages/🦀️rust/Cargo.toml");

// Strip overlay + cargo-features header from fem package
let femCargo = readFileSync(femCargoPath, "utf8");
femCargo = femCargo.replace(/^cargo-features = \["trim-paths"\]\n+/m, "");
const overlayAt = femCargo.search(/# ==== 🧪️ TEMPORARY VERIFICATION OVERLAY|# 🚧️ TEMPORARY VERIFICATION OVERLAY/);
if (overlayAt >= 0) femCargo = femCargo.slice(0, overlayAt).replace(/\n+$/, "\n");
writeFileSync(femCargoPath, femCargo);
console.log("stripped overlay; lines", femCargo.split("\n").length);

// Remove nested lock/target from verification overlay
for (const n of ["Cargo.lock", "target"]) {
  const p = join(fem, "📦️packages/🦀️rust", n);
  if (existsSync(p)) {
    rmSync(p, { recursive: true, force: true });
    console.log("removed nested", n);
  }
}

// Re-add fem member to root if missing
let cargo = readFileSync(join(root, "Cargo.toml"), "utf8");
const member = '    "✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust",';
if (!cargo.includes(member)) {
  // insert as first member after members = [
  if (!cargo.includes("members = [")) throw new Error("no members array");
  cargo = cargo.replace("members = [\n", `members = [\n${member}\n`);
  // sanity: no undefined or ,,
  if (cargo.includes("undefined") || cargo.includes(",,")) throw new Error("corruption detected");
  writeFileSync(join(root, "Cargo.toml"), cargo);
  console.log("re-added fem member");
} else console.log("fem member already present");

// Confirm no old fem members / workspace deps
const femMentions = cargo.split("\n").filter((l) => l.includes("🏗️fem") || /semio-s-.*fem/.test(l));
console.log("fem mentions in root Cargo.toml:");
femMentions.forEach((l) => console.log(" ", l.trim()));

// Final purity
function walk(dir, acc=[]) {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (n==="target"||n==="node_modules") continue;
      if (n==="⚡️implementations") acc.push(["IMPL", relative(fem,p)]);
      else walk(p, acc);
    } else if (p.endsWith(".rs") && !p.endsWith("component.rs") && !p.includes("/📦️packages/")) {
      acc.push(["FLAT", relative(fem,p)]);
    }
  }
  return acc;
}
const issues = walk(fem);
console.log("purity issues", issues.length);
issues.forEach((i) => console.log(" ", i));

// owner root listing
console.log("owner root:");
for (const n of readdirSync(fem)) console.log(" ", n);
