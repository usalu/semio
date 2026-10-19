import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
const [r1, r2] = process.argv.slice(2) as string[];
const all: any[] = [...JSON.parse(readFileSync(r1!, "utf8")).reverted, ...JSON.parse(readFileSync(r2!, "utf8"))];
const seen = new Set<string>(), out: string[] = [];
for (const e of all) {
  const k = e.file + "\0" + e.before + "\0" + e.after;
  if (seen.has(k)) continue; seen.add(k);
  const head = spawnSync("git", ["show", `HEAD:${e.file}`], { cwd: "/Users/ueli/Documents/semio", encoding: "utf8", maxBuffer: 1 << 26 });
  if (head.status !== 0) continue;
  const n = head.stdout.split(`"${e.after}"`).length - 1;
  if (n > 0) out.push(`${e.file}\t${e.before}\t${e.after}\tHEAD occurrences ${n}`);
}
console.log(out.join("\n")); console.log(out.length, "of", seen.size);
