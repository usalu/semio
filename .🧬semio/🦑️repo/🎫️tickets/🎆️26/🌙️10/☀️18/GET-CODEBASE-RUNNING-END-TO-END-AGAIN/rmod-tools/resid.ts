import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { C, root } from "./lib.ts";
const [word, ...dirs] = process.argv.slice(2);
const plan = JSON.parse(readFileSync(C + "/🗑️generated/rmod-plan.json", "utf8")) as Record<string, { a: number; b: number; old: string }[]>;
const covered = new Map<string, Set<number>>();
for (const [f, es] of Object.entries(plan)) { const s = readFileSync(f, "utf8"); const set = new Set<number>(); for (const e of es) { let ln = 1; for (let i = 0; i < e.a; i++) if (s.charCodeAt(i) === 10) ln++; set.add(ln); } covered.set(f, set); }
const r = spawnSync("rg", ["-n", "--glob", "*.rs", "--no-heading", `\\b${word}\\b`, ...dirs], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
let n = 0;
for (const l of r.stdout.split("\n")) {
  const m = l.match(/^(.*?):(\d+):(.*)$/); if (!m) continue;
  const f = root + "/" + m[1]; const ln = +m[2]!; const text = m[3]!;
  if (/^\s*\/\//.test(text)) continue;
  if (covered.get(f)?.has(ln)) continue;
  if (!new RegExp(`(::\\s*${word}\\b|\\b${word}\\s*::|mod\\s+${word}\\b|use\\s+${word}\\b|\\{[^}]*\\b${word}\\b)`).test(text)) continue;
  console.log(m[1]!.slice(-70) + ":" + ln + ": " + text.trim().slice(0, 190)); n++;
}
console.log("uncovered", n);
