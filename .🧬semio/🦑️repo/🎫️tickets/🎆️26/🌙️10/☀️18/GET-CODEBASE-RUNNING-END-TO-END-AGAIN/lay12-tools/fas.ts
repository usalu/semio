import { readFileSync } from "node:fs";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const cycles: string[][] = a.rows.filter((r: any) => r.rule === "no-circular").map((r: any) => [r.from, ...r.cycle]);
const short = (p: string) => p.replace(/🧰️framework\/🔨️modules\//u, "M:").replace(/🧰️framework\/📦️packages\//u, "P:").replace(/[️]/gu, "");
let remaining = cycles.map((c) => c.slice());
const edgesOf = (c: string[]) => c.map((n, i) => n + " => " + c[(i + 1) % c.length]);
const picks: [string, number][] = [];
while (remaining.length) {
  const cnt = new Map<string, number>();
  for (const c of remaining) for (const e of new Set(edgesOf(c))) cnt.set(e, (cnt.get(e) ?? 0) + 1);
  const [best, n] = [...cnt.entries()].sort((x, y) => y[1] - x[1])[0]!;
  picks.push([best, n]);
  remaining = remaining.filter((c) => !edgesOf(c).includes(best));
}
console.log(picks.length, "edges cover", cycles.length, "cycles");
for (const [e, n] of picks) console.log(n, short(e.split(" => ")[0]!), "=>", short(e.split(" => ")[1]!));
