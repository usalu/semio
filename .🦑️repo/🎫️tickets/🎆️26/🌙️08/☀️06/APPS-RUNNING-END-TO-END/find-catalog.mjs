import { existsSync, readdirSync, readFileSync, statSync } from "fs";
import { join } from "path";

function walk(dir, pred, out=[], depth=0) {
  if (depth > 10) return out;
  let entries;
  try { entries = readdirSync(dir, { withFileTypes: true }); } catch { return out; }
  for (const e of entries) {
    if (e.name === "node_modules" || e.name === "target" || e.name === ".git") continue;
    const p = join(dir, e.name);
    if (pred(e.name, p)) out.push(p);
    if (e.isDirectory()) walk(p, pred, out, depth+1);
  }
  return out;
}

const jsons = walk("�framework", (n) => n === "🔣️playgrounds.json" || n === "playgrounds.json");
const tss = walk("�framework", (n) => n === "🟦️playgrounds.ts" || n.includes("playgrounds.ts"));
console.log("json candidates:", jsons);
console.log("ts candidates:", tss);

const stale = "./�framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/