import { resolve } from "node:path";
import { readJson } from "./lib.ts";
const cls = readJson<any>(resolve(import.meta.dir, "classification.json"));
const fin = readJson<any>(resolve(import.meta.dir, "exports-final.json"));
const out: string[] = [];
const sum = (rows: any[], pick: (r: any) => number): number => rows.reduce((n, r) => n + pick(r), 0);
const imp = (r: any): number => r.importers.repoProduct + r.importers.other;
out.push("### classification totals", "", "| class | distinct symbols | import specifiers |", "|---|---:|---:|");
for (const c of ["a", "b", "c"]) { const rows = cls.rows.filter((r: any) => r.class === c); out.push(`| ${c} | ${rows.length} | ${sum(rows, imp)} |`); }
out.push("", "### (a) destinations", "", "| framework destination | symbols | import specifiers |", "|---|---:|---:|");
const a = new Map<string, { n: number; i: number }>();
for (const r of cls.rows.filter((x: any) => x.class === "a")) { const k = r.to.replace(/ as .*/u, ""); const v = a.get(k) ?? { n: 0, i: 0 }; a.set(k, v); v.n++; v.i += imp(r); }
for (const [k, v] of [...a].sort((x, y) => y[1].i - x[1].i)) out.push(`| \`${k}\` | ${v.n} | ${v.i} |`);
out.push("", "### (b) domain entries", "", "| entry | members (library files) | exported names | import specifiers (repo product / other products) | test-only outside the repo product |", "|---|---:|---:|---|---:|");
for (const [key, d] of Object.entries<any>(fin.repoLib.domains)) {
  const rows = cls.rows.filter((r: any) => r.class === "b" && r.to === key);
  out.push(`| \`${key}\` | ${d.members.length} | ${d.names} | ${sum(rows, r => r.importers.repoProduct)} / ${sum(rows, r => r.importers.other)} | ${rows.filter((r: any) => r.testOnlyOutsideRepo).length} |`);
}
out.push("", "### (c) deleted couplings", "");
for (const r of cls.rows.filter((x: any) => x.class === "c")) out.push(`- ${r.target} :: ${r.name} -> ${r.to} (${imp(r)})`);
out.push("", "### spec coverage of repo-lib rows", "");
const cov: Record<string, number> = {};
for (const v of Object.values<any>(fin.repoLib.specCoverage)) { const k = v.disposition.startsWith("settled") ? "settled (no importer left outside the library)" : v.detail.map((x: string) => x.split(":")[0]).sort().join("+"); cov[k] = (cov[k] ?? 0) + 1; }
for (const [k, v] of Object.entries(cov)) out.push(`- ${k}: ${v}`);
out.push("", "### other packages in exports-final", "", "| package | new entries | importers |", "|---|---:|---:|");
for (const s of fin.summary) out.push(`| ${s.package} | ${s.newExportEntries} | ${s.importerEdges} |`);
console.log(out.join("\n"));
