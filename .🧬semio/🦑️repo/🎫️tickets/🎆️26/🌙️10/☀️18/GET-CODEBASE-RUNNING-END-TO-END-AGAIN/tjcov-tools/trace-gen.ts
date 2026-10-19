import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const SRC = process.argv[2]!, OUT = process.argv[3]!;
let s = readFileSync(SRC, "utf8");
const dir = dirname(SRC);
s = s.replace(/from "(\.\.?\/[^"]+)"/g, (_m, p) => `from "${resolve(dir, p)}"`);
const lines = s.split("\n");
let n = 0;
const out = lines.map((line) => {
  const m = line.match(/^(\s*)if \(!(toolJob\w+)\(/);
  if (!m) return line;
  const start = line.indexOf("(", line.indexOf(m[2]!));
  let depth = 0, i = start, q = "";
  for (; i < line.length; i++) {
    const c = line[i]!;
    if (q) { if (c === "\\") i++; else if (c === q) q = ""; continue; }
    if (c === '"' || c === "'" || c === "`") { q = c; continue; }
    if (c === "(") depth++;
    if (c === ")") { depth--; if (depth === 0) break; }
  }
  if (depth !== 0) return line;
  const args = line.slice(start + 1, i);
  n++;
  return `${m[1]}await (globalThis as any).__trace(${JSON.stringify(m[2])}, () => [${args}]);\n${line}`;
});
writeFileSync(OUT, out.join("\n"));
console.log("instrumented", n);
