import { readFileSync, writeFileSync } from "node:fs";
import { C } from "./lib.ts";
const plan = JSON.parse(readFileSync(C + "/🗑️generated/rmod-plan.json", "utf8")) as Record<string, { a: number; b: number; old: string; nw: string }[]>;
let files = 0, edits = 0;
const bad: string[] = [];
const dry = process.argv[2] === "--dry";
for (const [f, es] of Object.entries(plan)) {
  const src = readFileSync(f, "utf8");
  let ok = true;
  for (const e of es) if (src.slice(e.a, e.b) !== e.old && src.slice(e.a, e.b) !== e.nw) { ok = false; bad.push(`${f}@${e.a}: expected ${e.old} found ${src.slice(e.a, e.b)}`); }
  if (!ok) continue;
  let out = src;
  for (const e of [...es].sort((x, y) => y.a - x.a)) if (src.slice(e.a, e.b) === e.old) { out = out.slice(0, e.a) + e.nw + out.slice(e.b); edits++; }
  if (!dry) writeFileSync(f, out);
  files++;
}
console.log(`${dry ? "dry " : ""}applied ${edits} edits in ${files} files; stale ${bad.length}`);
for (const b of bad.slice(0, 20)) console.log("STALE", b);
