import { readFileSync } from "node:fs";
const [changed, ...logs] = process.argv.slice(2) as string[];
const set = new Set(readFileSync(changed!, "utf8").split("\n").filter(Boolean));
for (const l of logs) for (const line of readFileSync(l, "utf8").split("\n")) {
  const m = line.match(/^(.+?)\((\d+),(\d+)\): error (TS\d+): (.*)$/u);
  if (m && set.has(m[1]!)) console.log(`${m[1]!.slice(-80)}:${m[2]} ${m[4]} ${m[5]!.slice(0, 200)}`);
}
