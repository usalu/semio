import {readFileSync} from "node:fs";
const F = process.argv[2]!, thr = Number(process.argv[3] ?? 0.9);
const text = readFileSync(`${F}/🗑️generated/hdr3-crate-census.csv`, "utf8").trim().split("\n");
const parse = (l: string) => { const out: string[] = []; let cur = "", q = false; for (let i = 0; i < l.length; i++) { const c = l[i]!; if (q) { if (c === '"') { if (l[i+1] === '"') { cur += '"'; i++; } else q = false; } else cur += c; } else if (c === '"') q = true; else if (c === ",") { out.push(cur); cur = ""; } else cur += c; } out.push(cur); return out; };
const h = parse(text[0]!); const rows = text.slice(1).map(l => { const v = parse(l); return Object.fromEntries(h.map((k, i) => [k, v[i]])); });
const sel = rows.filter(r => Number(r.libRatio) >= thr || Number(r.testRatio) >= thr || r.libRatio === "refused" || r.testRatio === "refused").sort((a, b) => Math.max(Number(b.libRatio)||0, Number(b.testRatio)||0) - Math.max(Number(a.libRatio)||0, Number(a.testRatio)||0));
console.log(`n=${sel.length}`);
for (const r of sel) console.log([Math.max(Number(r.libRatio)||0, Number(r.testRatio)||0).toFixed(3), r.name, `lib ${r.libFiles}/${r.libBytes}/${r.libLines} ${r.libRatio} ${r.libBinding}`, `test ${r.testFiles}/${r.testBytes}/${r.testLines} ${r.testRatio} ${r.testBinding} [${r.testWorstTarget}]`, r.manifest].join(" | "));
