import { readFileSync } from "node:fs";
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const map = JSON.parse(readFileSync(`${G}/ts3-geometry-map.json`, "utf8"));
const meta = JSON.parse(readFileSync(`${G}/ts3-geometry-meta.json`, "utf8"));
const orig: Record<string, unknown> = await import(`${G}/ts3-orig/geometry-orig.ts`);
const files = [...new Set(Object.values(map.names) as string[])];
const merged: Record<string, unknown> = {};
const dup: string[] = [];
for (const f of files) {
  const mod: Record<string, unknown> = await import(`/Users/ueli/Documents/semio/${f}`);
  for (const [k, v] of Object.entries(mod)) { if (k in merged) dup.push(k); merged[k] = v; }
}
const onlyOrig = Object.keys(orig).filter((k) => !(k in merged));
const onlyNew = Object.keys(merged).filter((k) => !(k in orig));
let same = 0; const differs: string[] = [];
for (const k of Object.keys(orig)) {
  if (!(k in merged)) continue;
  const a = orig[k], b = merged[k];
  if (typeof a === "function" && typeof b === "function") { if (a.toString() === b.toString()) same++; else differs.push(k); }
  else if (typeof a === "object" && a !== null) { try { if (JSON.stringify(a) === JSON.stringify(b)) same++; else differs.push(k + " (value)"); } catch { same++; } }
  else if (a === b) same++; else differs.push(k + " (primitive)");
}
const exportedMeta = Object.entries(meta).filter(([, m]: any) => m.exported && !m.type).map(([k]) => k);
console.log(JSON.stringify({ origExports: Object.keys(orig).length, newExports: Object.keys(merged).length, duplicatesAcrossFiles: dup, onlyOrig, onlyNewInternalNowExported: onlyNew, identicalToString: same, differs, valueExportsPerMeta: exportedMeta.length }, null, 1));
