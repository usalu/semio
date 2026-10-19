import { readFileSync, writeFileSync } from "node:fs";
import { ALLNAMES, buildGraph } from "./graph.ts";
import { C, root } from "./lib.ts";
const fams = JSON.parse(readFileSync(C + "/🗑️generated/rmod-families.json", "utf8")) as Record<string, string[]>;
const crateNames = new Set(Object.values(fams).flat());
const g = buildGraph(ALLNAMES);
const files = new Set<string>();
for (const [f, mods] of g.modsByFile) if (mods.some((m) => crateNames.has(m.crate.crate.name))) files.add(f);
console.error("files", files.size);
const rx = /(?<![A-Za-z0-9_])(helpers|base|core|shared|common)(::|`)/;
const rx2 = /(?<![A-Za-z0-9_])(helpers|base|core|shared|common)::[A-Za-z_]/;
const out: string[] = [];
for (const f of files) {
  const src = readFileSync(f, "utf8");
  const lines = src.split("\n");
  let inBlock = false;
  lines.forEach((l, i) => {
    let c = "";
    if (inBlock) { c = l; if (l.includes("*/")) inBlock = false; }
    else {
      const m = l.match(/\/\/.*$/); if (m) c = m[0];
      const b = l.indexOf("/*"); if (b >= 0 && !l.includes("*/", b)) { inBlock = true; c += l.slice(b); }
    }
    if (c && rx2.test(c)) out.push(`${f.slice(root.length + 1).slice(-90)}:${i + 1}: ${c.trim().slice(0, 200)}`);
  });
}
writeFileSync(C + "/🗑️generated/rmod-doccheck.txt", out.join("\n"));
console.error("doc-ish lines", out.length);
