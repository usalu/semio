import { readFileSync, writeFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const rend = OS + "🔨️modules/📺️renderer/";
const mods = new Map<string, any>(r.modules.map((m: any) => [m.source, m]));
const seen = new Set<string>(); const rows: any[] = [];
for (const v of r.summary.violations) {
  if (v.rule.name !== "no-circular") continue;
  const members: string[] = [v.from, ...v.cycle.map((c: any) => c.name)];
  const key = [...new Set(members)].sort().join("|");
  if (seen.has(key)) continue; seen.add(key);
  const edges = [] as any[];
  for (let i = 0; i < members.length - 1; i++) {
    const dep = mods.get(members[i]!)?.dependencies.find((d: any) => d.resolved === members[i + 1]);
    edges.push({ from: members[i], to: members[i + 1], dynamic: !!dep?.dynamic, typeOnly: !!dep?.dependencyTypes?.includes("type-only"), module: dep?.module });
  }
  rows.push({ renderer: members.some((m) => m.startsWith(rend)), size: new Set(members).size, members, edges });
}
writeFileSync(process.argv[3]!, JSON.stringify(rows, null, 1));
const nr = rows.filter((x) => !x.renderer);
console.log("unique cycles", rows.length, "nonRenderer", nr.length, "renderer", rows.length - nr.length);
const s = (p: string) => p.replace(OS, "");
for (const row of nr) { console.log("--", row.size, row.edges.map((e: any) => `${s(e.from)} -> ${s(e.to)} [${e.module}${e.dynamic ? " dyn" : ""}${e.typeOnly ? " type" : ""}]`).join("\n    ")); }
