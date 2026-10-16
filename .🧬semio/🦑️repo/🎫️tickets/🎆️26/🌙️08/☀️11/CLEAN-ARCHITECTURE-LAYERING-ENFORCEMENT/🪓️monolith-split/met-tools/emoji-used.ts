import { readFileSync, writeFileSync } from "node:fs";
const dirs = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const counts = new Map<string, number>();
for (const d of dirs) {
  const base = d.slice(d.lastIndexOf("/") + 1);
  const m = /^(\p{Extended_Pictographic}(?:‍\p{Extended_Pictographic}️?)*)️/u.exec(base);
  if (m) counts.set(m[1]!, (counts.get(m[1]!) ?? 0) + 1);
}
writeFileSync(process.argv[3]!, [...counts].sort((a, b) => b[1] - a[1]).map(([e, c]) => `${e}\t${c}`).join("\n") + "\n");
console.log(counts.size);
