import { existsSync, readdirSync, readFileSync } from "fs";
import { join } from "path";

function walk(dir, pred, out = [], depth = 0) {
  if (depth > 12) return out;
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return out;
  }
  for (const e of entries) {
    if (e.name === "node_modules" || e.name === "target" || e.name === ".git" || e.name === "storybook-static") continue;
    const p = join(dir, e.name);
    if (pred(e.name, p)) out.push(p);
    if (e.isDirectory()) walk(p, pred, out, depth + 1);
  }
  return out;
}

const jsons = walk("�framework", (n) => n.includes("playgrounds") && n.endsWith(".json"));
const tss = walk("�framework", (n) => n.includes("playgrounds") && n.endsWith(".ts"));
console.log("json candidates:", jsons);
console.log("ts candidates:", tss.slice(0, 40));

const stale =
  "./�framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/