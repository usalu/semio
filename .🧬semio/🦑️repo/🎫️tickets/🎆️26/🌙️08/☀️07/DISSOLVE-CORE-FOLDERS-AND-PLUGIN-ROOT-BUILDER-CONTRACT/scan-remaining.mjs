import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = process.env.TICKET;
const SKIP = new Set(["node_modules", "target", ".git", ".nx", "dist", "build", ".repo-cache", ".venv"]);

function walk(dir, acc = []) {
  let entries;
  try { entries = readdirSync(dir); } catch { return acc; }
  for (const name of entries) {
    if (SKIP.has(name) || name.startsWith(".")) continue;
    const p = join(dir, name);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p, acc);
    else acc.push(p);
  }
  return acc;
}

const files = walk(ROOT);
const cargoCore = [];
const cargoFramework = [];
const tsCore = [];
for (const f of files) {
  if (f.includes("🎫️tickets")) continue;
  const rel = relative(ROOT, f);
  if (f.endsWith("Cargo.toml")) {
    const t = readFileSync(f, "utf8");
    if (t.includes("semio-framework-core")) cargoCore.push(rel);
    if (/semio-framework\s*=/.test(t) || t.includes('package = "semio-framework"') || t.includes('name = "semio-framework"')) {
      if (t.includes("semio-framework") && !t.includes("semio-framework-core") && (t.includes('package = "semio-framework"') || /^semio-framework\s*=/m.test(t) || t.includes('name = "semio-framework"'))) {
        // count later
      }
    }
  }
  if (/\.(ts|tsx|json|mjs|cjs)$/.test(f)) {
    try {
      const t = readFileSync(f, "utf8");
      if (t.includes("@semio-tech/framework-core")) tsCore.push(rel);
    } catch {}
  }
}
console.log("Cargo still semio-framework-core:", cargoCore.length);
for (const c of cargoCore.slice(0, 40)) console.log(" ", c);
console.log("TS still @semio-tech/framework-core:", tsCore.length);
for (const c of tsCore.slice(0, 40)) console.log(" ", c);

// sample root cargo
const root = readFileSync(join(ROOT, "Cargo.toml"), "utf8");
for (const line of root.split("\n")) {
  if (line.includes("framework-core") || line.includes("semio-framework =") || line.includes('semio-framework"')) {
    if (line.includes("framework")) console.log("ROOT:", line.trim());
  }
}
