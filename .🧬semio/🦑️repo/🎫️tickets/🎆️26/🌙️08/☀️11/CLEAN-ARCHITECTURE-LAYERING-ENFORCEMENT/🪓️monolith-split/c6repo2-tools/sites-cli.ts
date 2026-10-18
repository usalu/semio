import { candidateFiles, findSites, ROOT } from "./sites.ts";
import { writeFileSync } from "node:fs";
const targets = new Set(process.argv.slice(3).map((p) => p.startsWith("/") ? p : `${ROOT}/${p}`));
const out = process.argv[2];
const files = candidateFiles();
console.error(`candidates=${files.length}`);
const all: any[] = [];
for (const f of files) {
  try { all.push(...findSites(f, targets)); } catch (e) { console.error("ERR", f, String(e)); }
}
writeFileSync(out, JSON.stringify(all, null, 1));
const by = new Map<string, number>();
for (const s of all) by.set(s.kind + ":" + s.node, (by.get(s.kind + ":" + s.node) ?? 0) + 1);
console.log(all.length, "sites in", new Set(all.map((s) => s.file)).size, "files", [...by]);
