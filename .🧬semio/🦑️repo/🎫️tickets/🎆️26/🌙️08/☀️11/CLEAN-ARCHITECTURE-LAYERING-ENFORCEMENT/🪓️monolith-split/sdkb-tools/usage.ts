/** 🧮️ Per-target usage table (names, type-only flags, defaults, namespaces, stars) from a scan2 output. Read-only. */
import { readFileSync, writeFileSync } from "node:fs";
const [scan, out] = process.argv.slice(2);
const recs = JSON.parse(readFileSync(scan, "utf8"));
const by = new Map<string, any>();
for (const r of recs) {
  const a = by.get(r.resolved) ?? { target: r.resolved, files: new Set<string>(), names: new Map<string, { value: number; type: number }>(), defaults: 0, ns: 0, star: 0, side: 0, dyn: 0, importType: 0, kinds: new Set<string>() };
  by.set(r.resolved, a);
  a.files.add(r.file);
  if (r.kind === "dynamic" || r.kind === "require") a.dyn++;
  if (r.kind === "importType") a.importType++;
  if (r.defaultName) a.defaults++;
  if (r.namespaceName) a.ns++;
  if (r.exportStar) a.star++;
  if (r.sideEffect) a.side++;
  for (const n of r.named) { const e = a.names.get(n.imported) ?? { value: 0, type: 0 }; (r.typeOnly || n.typeOnly ? e.type++ : e.value++); a.names.set(n.imported, e); }
}
const rows = [...by.values()].map(a => ({ target: a.target, files: a.files.size, defaults: a.defaults, ns: a.ns, star: a.star, side: a.side, dyn: a.dyn, importType: a.importType, names: [...a.names].map(([k, v]) => ({ name: k, ...v })) })).sort((x, y) => y.files - x.files);
writeFileSync(out, JSON.stringify(rows, null, 1));
for (const r of rows) console.log(String(r.files).padStart(4), r.target.replace("🧰️framework/", ""), `| def${r.defaults} ns${r.ns} star${r.star} side${r.side} dyn${r.dyn} it${r.importType} |`, r.names.map(n => (n.value ? "" : "t:") + n.name).join(","));
