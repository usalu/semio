import { readdirSync, statSync, readFileSync, existsSync } from "fs";
import { join, relative } from "path";

const fem = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem";

function walk(dir, acc = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (name === "target" || name === "node_modules") continue;
      walk(p, acc);
    } else acc.push(p);
  }
  return acc;
}

const files = walk(fem);
const flat = files.filter((p) => {
  if (!p.endsWith(".rs")) return false;
  if (p.includes("/📦️packages/")) return false;
  if (p.includes("/⚡️implementations/")) return false;
  if (p.endsWith("component.rs")) return false;
  return true;
});
console.log("FLAT VARIANTS:");
for (const p of flat) console.log(" ", relative(fem, p), statSync(p).size);

const pairs = [
  ["🙰core/🦀️analyses.rs", "🙰core/🧮️analyses/🦀️component.rs"],
];
// discover pairs: flat X.rs vs folder/component.rs sibling
console.log("\nPAIRS (flat vs folder):");
for (const p of flat) {
  const rel = relative(fem, p);
  const base = rel.split("/").pop().replace(/^🦀️/, "").replace(/\.rs$/, "");
  const parent = join(p, "..");
  // find sibling dirs that might match
  const siblings = readdirSync(parent).filter((n) => {
    try { return statSync(join(parent, n)).isDirectory(); } catch { return false; }
  });
  const match = siblings.find((n) => n.includes(base) || n.endsWith(base) || base.includes(n.replace(/^[^\w]*/g, "")));
  // softer: any sibling with component.rs whose content equals
  let found = null;
  for (const s of siblings) {
    const cand = join(parent, s, "🦀️component.rs");
    if (existsSync(cand)) {
      const a = readFileSync(p);
      const b = readFileSync(cand);
      if (a.equals(b)) { found = { s, equal: true, size: a.length }; break; }
      // check if same length
      if (!found) found = { s, equal: false, flatSize: a.length, folderSize: b.length };
    }
  }
  console.log(rel, "->", found);
}

// root lib
console.log("\nROOT LIB exists:", existsSync(join(fem, "📦️lib.rs")));
console.log("PKG LIB exists:", existsSync(join(fem, "📦️packages/🦀️rust/📦️lib.rs")));

// check pkg lib references flat or folder
const lib = readFileSync(join(fem, "📦️packages/🦀️rust/📦️lib.rs"), "utf8");
const flatRefs = [...lib.matchAll(/#\[path = "([^"]+)"\]/g)].map(m => m[1]);
console.log("\nPKG LIB PATHS:");
for (const r of flatRefs) console.log(" ", r);
const bad = flatRefs.filter(r => r.endsWith(".rs") && !r.endsWith("component.rs") && r !== ".");
console.log("\nNON-COMPONENT PATH REFS:", bad);
