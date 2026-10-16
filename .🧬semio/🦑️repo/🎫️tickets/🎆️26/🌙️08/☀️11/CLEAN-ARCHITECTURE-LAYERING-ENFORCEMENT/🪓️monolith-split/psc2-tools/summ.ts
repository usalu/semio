/** 📊️ Summarises scanned edges per target owner: distinct targets, importer areas, kinds. Usage: bun summ.ts <scan.json> [owner-prefix] */
import { readFileSync } from "node:fs";
const edges = JSON.parse(readFileSync(process.argv[2], "utf8"));
const only = process.argv[3];
const by = new Map<string, any[]>();
for (const e of edges) { if (only && e.sourceOwner !== only) continue; const l = by.get(e.sourceOwner) ?? []; l.push(e); by.set(e.sourceOwner, l); }
const short = (s: string) => s.replace(/🧰️framework\/🔨️modules\//g, "F/M/").replace(/🧰️framework\/🛍️products\//g, "F/P/").replace(/✏️s\/🔌️plugins\//g, "S/");
for (const [k, l] of [...by].sort((a, b) => b[1].filter((e: any) => !e.test).length - a[1].filter((e: any) => !e.test).length)) {
  const prod = l.filter((e: any) => !e.test);
  if (!prod.length && !only) continue;
  const tg = new Map<string, number>(); for (const e of prod) tg.set(e.resolved, (tg.get(e.resolved) ?? 0) + 1);
  const im = new Map<string, number>(); for (const e of prod) { const d = e.file.split("/").slice(0, 5).join("/"); im.set(d, (im.get(d) ?? 0) + 1); }
  console.log(`\n## ${short(k)}  prod=${prod.length} all=${l.length} distinctTargets=${tg.size} importerFiles=${new Set(prod.map((e: any) => e.file)).size}`);
  console.log("  importers:", [...im].sort((a, b) => b[1] - a[1]).slice(0, only ? 40 : 6).map(([d, n]) => `${n} ${short(d)}`).join(" | "));
  console.log("  targets:", [...tg].sort((a, b) => b[1] - a[1]).slice(0, only ? 60 : 4).map(([d, n]) => `${n} ${short(d.slice(k.length + 1))}`).join(" | "));
}
