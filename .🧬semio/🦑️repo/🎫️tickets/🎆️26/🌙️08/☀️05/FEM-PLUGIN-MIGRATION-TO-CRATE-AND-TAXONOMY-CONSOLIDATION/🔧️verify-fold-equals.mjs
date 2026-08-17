import { readFileSync, existsSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const fem = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem";

function walk(dir, acc = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    const st = statSync(p);
    if (st.isDirectory()) {
      if (name === "target" || name === "node_modules" || name === "⚡️implementations") continue;
      walk(p, acc);
    } else acc.push(p);
  }
  return acc;
}

const files = walk(fem);
const byBase = new Map();
for (const p of files) {
  if (!p.endsWith(".rs") || p.includes("/📦️packages/")) continue;
  const rel = relative(fem, p);
  byBase.set(rel, p);
}

// For each flat variant (not *component.rs), find folder/*/component.rs that pkg lib uses
const flats = [...byBase.keys()].filter(r => r.endsWith(".rs") && !r.endsWith("component.rs"));

// Explicit map from packages lib paths (folder form) — derive expected folder from known naming
const expected = {
  // will fill by scanning: if flat is DIR/🦀️NAME.rs and DIR/SOMETHING/🦀️component.rs exists with equal content
};

function findEqualFolder(flatRel) {
  const flatPath = byBase.get(flatRel);
  const flatBytes = readFileSync(flatPath);
  const parts = flatRel.split("/");
  const file = parts.pop();
  const parentRel = parts.join("/");
  const parentAbs = join(fem, parentRel);
  const kids = readdirSync(parentAbs);
  for (const kid of kids) {
    const cand = join(parentAbs, kid, "🦀️component.rs");
    if (!existsSync(cand)) continue;
    const b = readFileSync(cand);
    if (b.equals(flatBytes)) return { folder: join(parentRel, kid, "🦀️component.rs"), equal: true, size: b.length };
  }
  // also report closest size match
  let best = null;
  for (const kid of kids) {
    const cand = join(parentAbs, kid, "🦀️component.rs");
    if (!existsSync(cand)) continue;
    const b = readFileSync(cand);
    const d = Math.abs(b.length - flatBytes.length);
    if (!best || d < best.d) best = { folder: join(parentRel, kid, "🦀️component.rs"), equal: false, flatSize: flatBytes.length, folderSize: b.length, d };
  }
  return best;
}

let allOk = true;
for (const f of flats) {
  if (f === "📦️lib.rs") { console.log("ROOT_LIB", f); continue; }
  const r = findEqualFolder(f);
  if (!r) { console.log("NO_CANDIDATE", f); allOk = false; continue; }
  console.log(r.equal ? "EQ" : "DIFF", f, "->", r.folder, r.equal ? r.size : `${r.flatSize} vs ${r.folderSize}`);
  if (!r.equal) allOk = false;
}
console.log(allOk ? "\nALL_FLATS_HAVE_EQUAL_FOLDERS" : "\nSOME_DIFFS");
