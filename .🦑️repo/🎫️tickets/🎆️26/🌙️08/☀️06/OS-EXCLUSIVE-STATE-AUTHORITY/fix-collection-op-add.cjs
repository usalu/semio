
const { readdirSync, readFileSync, writeFileSync } = require("fs");
const { join } = require("path");
const fw = readdirSync(".").find((e) => e.endsWith("framework"));
const sDir = readdirSync(".").find((e) => e.startsWith("✏️"));
function walk(d, acc = []) {
  let ents;
  try { ents = readdirSync(d, { withFileTypes: true }); } catch { return acc; }
  for (const e of ents) {
    const p = join(d, e.name);
    if (e.isDirectory()) {
      if (["node_modules", "target", ".git"].includes(e.name)) continue;
      if (/compose|bestand|hub/.test(e.name)) continue;
      walk(p, acc);
    } else if (e.name.endsWith(".rs")) acc.push(p);
  }
  return acc;
}
const files = [sDir, fw].filter(Boolean).flatMap((r) => walk(r));
let changedFiles = 0, sites = 0;
for (const file of files) {
  let s = readFileSync(file, "utf8");
  if (!s.includes("CollectionOperation::Add")) continue;
  const before = s;
  s = s.replace(/CollectionOperation::Add\s*\{\s*id:\s*[^,]+,\s*item\s*,\s*at\s*\}/g, () => { sites++; return "CollectionOperation::Add { index: at, item }"; });
  s = s.replace(/CollectionOperation::Add\s*\{([^{}]*)\}/g, (full, body) => {
    if (!/\bid\s*:/.test(body) && !/\bat\s*:/.test(body)) return full;
    if (/\bindex\s*:/.test(body) && !/\bid\s*:/.test(body) && !/\bat\s*:/.test(body)) return full;
    const parts = [];
    let depth = 0, cur = "";
    for (const ch of body) {
      if ("([{".includes(ch)) depth++;
      if (")]}".includes(ch)) depth--;
      if (ch === "," && depth === 0) { parts.push(cur); cur = ""; }
      else cur += ch;
    }
    if (cur.trim()) parts.push(cur);
    const fields = {};
    for (const part of parts) {
      const m = part.match(/^\s*(id|item|at|index)\s*:\s*([\s\S]+)$/);
      if (m) fields[m[1]] = m[2].trim();
      else return full;
    }
    if (fields.item == null) return full;
    const indexExpr = fields.index ?? fields.at ?? "0";
    sites++;
    return "CollectionOperation::Add { index: " + indexExpr + ", item: " + fields.item + " }";
  });
  if (s !== before) { writeFileSync(file, s); changedFiles++; console.log("updated", file); }
}
console.log(JSON.stringify({ changedFiles, sites }));
