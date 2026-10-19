import { readFileSync, writeFileSync } from "node:fs";
const [, , reportPath, outPath] = process.argv;
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const mods = new Map<string, any>(report.modules.map((m: any) => [m.source, m]));
const strip = (p: string): string => p.replace(/[^\x00-\x7f\/]/gu, "").replace(/\/+/gu, "/");
const pluginOf = (p: string): string | undefined => /^✏️s\/🔌️plugins\/([^/]+)\//u.exec(p)?.[1];
const seen = new Set<string>(), rows: any[] = [];
for (const v of report.summary.violations) {
  if (v.rule.name !== "no-circular" || pluginOf(v.from) === undefined) continue;
  const members: string[] = [v.from, ...v.cycle.map((c: any) => c.name)];
  const key = [...new Set(members)].sort().join("|");
  if (seen.has(key)) continue; seen.add(key);
  const edges = [] as any[];
  for (let i = 0; i < members.length - 1; i++) {
    const dep = mods.get(members[i]!)?.dependencies.find((d: any) => d.resolved === members[i + 1]);
    edges.push({ from: members[i], to: members[i + 1], dynamic: !!dep?.dynamic, typeOnly: !!dep?.dependencyTypes?.includes("type-only"), types: dep?.dependencyTypes });
  }
  const dirs = [...new Set(members)].map((m) => m.split("/").slice(0, -1).join("/"));
  const hubDir = dirs.find((d) => dirs.every((x) => x === d || x.startsWith(d + "/")));
  const parentChild = new Set(members).size === 2 && hubDir !== undefined;
  rows.push({ plugin: pluginOf(v.from), size: new Set(members).size, hubDir: hubDir ?? null, parentChild, dynamic: edges.filter((e) => e.dynamic).length, typeOnly: edges.filter((e) => e.typeOnly).length, tests: members.some((m) => m.includes("🧪️tests")), members, edges });
}
const count = (f: (r: any) => string) => rows.reduce((o: any, r) => (o[f(r)] = (o[f(r)] ?? 0) + 1, o), {});
const summary = {
  cycles: rows.length,
  byPlugin: count((r) => strip(r.plugin)),
  allMembersUnderOneHubDir: rows.filter((r) => r.hubDir).length,
  parentChild2: rows.filter((r) => r.parentChild).length,
  withDynamicEdge: rows.filter((r) => r.dynamic).length,
  withTypeOnlyEdge: rows.filter((r) => r.typeOnly).length,
  breakableByTypeOnlyOrDynamicAlready: rows.filter((r) => r.dynamic || r.typeOnly).length,
  allStaticValueEdges: rows.filter((r) => !r.dynamic && !r.typeOnly).length,
  involvingTests: rows.filter((r) => r.tests).length,
  bySize: count((r) => String(r.size)),
  hubDirTop: Object.entries(count((r) => strip(r.hubDir ?? "(none)"))).sort((a: any, b: any) => b[1] - a[1]).slice(0, 25),
};
writeFileSync(outPath!, JSON.stringify({ summary, rows }, null, 1));
console.log(JSON.stringify(summary, null, 1));
