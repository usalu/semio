// 🔬 Census of target-name families and script subcommands over the static graph.
const graph = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/bworkers-graph/graph.json").text());
const names = new Map<string, number>(), subs = new Map<string, number>(), cont = new Map<string, number>();
let cache = 0, nocache = 0, total = 0;
for (const project of Object.values<any>(graph.projects)) for (const [name, t] of Object.entries<any>(project.targets)) {
  total++;
  const fam = name.split("-")[0]!;
  names.set(fam, (names.get(fam) ?? 0) + 1);
  const e = t.execution ?? {}; const op = e.command ?? "";
  const m = /📜️script\.ts"?\s+(\S+)/u.exec(op); const sub = m?.[1] ?? "?";
  subs.set(sub, (subs.get(sub) ?? 0) + 1);
  if (t.continuous) cont.set(fam, (cont.get(fam) ?? 0) + 1);
  if (t.cache === false) nocache++; else cache++;
}
const top = (m: Map<string, number>, n = 45) => [...m].sort((a, b) => b[1] - a[1]).slice(0, n).map(([k, v]) => `${k}:${v}`).join("  ");
console.log("targets", total, "cache!==false", cache, "cache===false", nocache);
console.log("families:", top(names));
console.log("subcommands:", top(subs));
console.log("continuous:", top(cont));
