import { rmSync, readdirSync, statSync, existsSync } from "fs";
import { join, relative } from "path";
const fem = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem";
const deleted = [];
function walk(dir) {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    if (!statSync(p).isDirectory()) continue;
    if (n === "target" || n === "node_modules" || n === "📦️packages") continue;
    if (n === "⚡️implementations") {
      rmSync(p, { recursive: true, force: true });
      deleted.push(relative(fem, p));
    } else walk(p);
  }
}
walk(fem);
console.log(JSON.stringify({ deletedCount: deleted.length, deleted }, null, 2));

// remove empty 🔨️modules dirs and 🛂️manifest if empty of useful content
function rmEmpty(dir) {
  if (!existsSync(dir) || !statSync(dir).isDirectory()) return;
  for (const n of readdirSync(dir)) rmEmpty(join(dir, n));
  if (readdirSync(dir).length === 0) {
    rmSync(dir, { recursive: true, force: true });
    console.log("rm empty", relative(fem, dir));
  }
}
for (const n of ["🔨️modules", "🛂️manifest", "🎛️apps/◻2d/🔨️modules", "🎛️apps/🎗3d/🔨️modules"]) {
  // discover actual
}
function findNamed(dir, name, acc=[]) {
  if (!existsSync(dir)) return acc;
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    if (!statSync(p).isDirectory()) continue;
    if (n === name) acc.push(p);
    if (n !== "📦️packages" && n !== "target") findNamed(p, name, acc);
  }
  return acc;
}
for (const p of findNamed(fem, "🔨️modules")) rmEmpty(p);
for (const p of findNamed(fem, "🛂️manifest")) rmEmpty(p);
