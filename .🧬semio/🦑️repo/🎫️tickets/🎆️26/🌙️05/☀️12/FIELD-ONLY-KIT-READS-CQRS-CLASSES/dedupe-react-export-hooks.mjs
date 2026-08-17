/**
 * Renames the *second* `export function useX` in compose/react/index.tsx when the name collides,
 * using suffix `KitHostBinding` (schema / host-store lane vs CQRS hooks earlier in the file).
 */
import fs from "node:fs";
import path from "node:path";

const target = path.join(process.cwd(), "compose/react/index.tsx");
const src = fs.readFileSync(target, "utf8");
const lines = src.split(/\n/);

const firstLine = new Map();
const renames = [];

for (let i = 0; i < lines.length; i++) {
  const m = lines[i].match(/^export function (use[A-Za-z0-9]+)\s*\(/);
  if (!m) continue;
  const name = m[1];
  const lineNo = i + 1;
  if (!firstLine.has(name)) {
    firstLine.set(name, lineNo);
    continue;
  }
  const newName = `${name}KitHostBinding`;
  if (lines[i].includes(newName)) continue;
  renames.push({ lineNo, name, newName, index: i });
}

for (const { name, newName, index } of [...renames].sort((a, b) => b.index - a.index)) {
  const line = lines[index];
  if (!line.startsWith(`export function ${name}(`)) continue;
  lines[index] = line.replace(`export function ${name}(`, `export function ${newName}(`);
}

fs.writeFileSync(target, lines.join("\n"), "utf8");
console.log("Renamed", renames.length, "duplicate export hooks to *KitHostBinding");
for (const r of renames.slice(0, 20)) console.log(r.lineNo, r.name, "->", r.newName);
if (renames.length > 20) console.log("...");
