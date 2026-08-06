import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

const root = process.cwd();
const files = [];
function walk(d, depth = 0) {
  if (depth > 12) return;
  let ents;
  try { ents = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of ents) {
    if (["node_modules", "target", ".git"].includes(e.name)) continue;
    const p = join(d, e.name);
    if (e.isFile() && e.name.endsWith(".rs") && (p.includes("wasm") || p.includes("🌉️") || p.includes("🕸️"))) {
      const src = readFileSync(p, "utf8");
      if (src.includes("dispatch_text") && src.includes("map_err") && !src.includes(".map(|_| ())")) {
        files.push(p);
      }
    }
    if (e.isDirectory()) walk(p, depth + 1);
  }
}
walk(join(root, readdirSync(root).find((n) => n.includes("✏️"))));

const changed = [];
for (const p of files) {
  let src = readFileSync(p, "utf8");
  const before = src;
  // dispatch_text(...).map_err(...)  -> .map(|_| ()).map_err(...)
  src = src.replace(
    /(\.dispatch_text\([^)]*\))\s*\.map_err\(/g,
    "$1.map(|_| ()).map_err(",
  );
  src = src.replace(
    /(\.dispatch_binary\([^)]*\))\s*\.map_err\(/g,
    "$1.map(|_| ()).map_err(",
  );
  if (src !== before) {
    writeFileSync(p, src);
    changed.push(p);
  }
}
const result = { scanned: files.length, changed };
console.log(JSON.stringify(result, null, 2));
writeFileSync(join(process.argv[2], "fix-wasm-dispatch-receipt-result.json"), JSON.stringify(result, null, 2));
