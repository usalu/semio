import { blocksOf } from "./docs.ts";
const root = "/Users/ueli/Documents/semio";
const out = process.argv[2];
const scan = JSON.parse(await Bun.file(`${out}/scan.json`).text());
const rows: any[] = [];
for (const [p, v] of Object.entries<any>(scan)) {
  if (!v.missing) continue;
  const text = await Bun.file(`${root}/${p}`).text();
  for (const b of blocksOf(p, text)) if (!b.startEmoji) rows.push({ p, line: b.line, kind: b.kind, skip: b.skip, head: b.head, decl: b.decl, mtime: v.mtime });
}
await Bun.write(`${out}/missing.json`, JSON.stringify(rows));
const by = (f: (r: any) => string) => { const m: Record<string, number> = {}; for (const r of rows) m[f(r)] = (m[f(r)] ?? 0) + 1; return Object.entries(m).sort((a, b) => b[1] - a[1]); };
console.log(rows.length, "\nSKIP", by((r) => r.skip ?? "none"), "\nKIND", by((r) => r.kind), "\nTREE", by((r) => r.p.split("/").slice(0, 2).join("/")).slice(0, 25));
