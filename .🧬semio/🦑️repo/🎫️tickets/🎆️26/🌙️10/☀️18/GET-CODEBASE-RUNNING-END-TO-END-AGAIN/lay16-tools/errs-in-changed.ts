import { readFileSync } from "node:fs";
const [tsc, ...lists] = process.argv.slice(2) as string[];
const changed = new Set(lists.flatMap(l => readFileSync(l, "utf8").split("\n").filter(Boolean)));
const mine = new Map<string, string[]>();
for (const line of readFileSync(tsc!, "utf8").split("\n")) {
  const m = /^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$/u.exec(line);
  if (!m) continue;
  if (changed.has(m[1]!)) (mine.get(m[1]!) ?? mine.set(m[1]!, []).get(m[1]!)!).push(`${m[2]}:${m[4]} ${m[5]!.slice(0, 160)}`);
}
for (const [f, errs] of mine) { console.log(f.split("/").slice(-4).join("/")); for (const e of errs.slice(0, 5)) console.log("   " + e); }
console.log("files with errors", mine.size);
