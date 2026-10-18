import { readFileSync, existsSync, statSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve, extname } from "node:path";
const entry = process.argv[2];
const seen = new Set(), bad = [], queue = [entry], unresolved = new Set();
const re = /(?:import|export)\s*(?:[^"';]*?\s*from\s*)?["']([^"']+)["']([^;\n]*)|import\(\s*["']([^"']+)["']\s*\)/gu;
while (queue.length) {
  const file = queue.pop(); if (seen.has(file)) continue; seen.add(file);
  if (!/\.(m?[jt]s|[jt]sx)$/u.test(file)) continue;
  const src = readFileSync(file, "utf8"); const req = createRequire(file);
  for (const m of src.matchAll(re)) {
    const spec = m[1] ?? m[3]; const tail = m[2] ?? "";
    if (!spec || spec.startsWith("node:") || spec.startsWith("bun:")) continue;
    let target;
    try { target = spec.startsWith(".") ? resolve(dirname(file), spec) : req.resolve(spec); } catch { unresolved.add(spec + "  <- " + file); continue; }
    if (!existsSync(target) || statSync(target).isDirectory()) { unresolved.add(spec + "  <- " + file); continue; }
    if (extname(target) === ".json" && !/with\s*\{\s*type\s*:\s*"json"\s*\}/u.test(tail)) bad.push(file + " -> " + spec);
    if (!target.includes("/node_modules/") && !target.includes("/.cache/")) queue.push(target);
  }
}
console.log("files", seen.size); console.log("JSON-WITHOUT-ATTRIBUTE", bad.length); for (const b of new Set(bad)) console.log(b);
console.log("UNRESOLVED", unresolved.size); for (const u of unresolved) console.log(u);
