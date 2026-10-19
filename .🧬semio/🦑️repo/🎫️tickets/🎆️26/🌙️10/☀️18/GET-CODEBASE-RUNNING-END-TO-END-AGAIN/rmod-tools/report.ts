import { readFileSync } from "node:fs";
import { C, root } from "./lib.ts";
const plan = JSON.parse(readFileSync(C + "/🗑️generated/rmod-plan.json", "utf8")) as Record<string, { a: number; b: number; old: string; nw: string; why: string }[]>;
const by: Record<string, { n: number; files: Set<string>; why: Record<string, number> }> = {};
for (const [f, es] of Object.entries(plan)) for (const e of es) { const k = `${e.old}->${e.nw}`; const r = (by[k] ??= { n: 0, files: new Set(), why: {} }); r.n++; r.files.add(f); r.why[e.why] = (r.why[e.why] ?? 0) + 1; }
for (const [k, r] of Object.entries(by)) console.log(k, r.n, "edits", r.files.size, "files", JSON.stringify(r.why));
const sample = process.argv[2];
if (sample) {
  for (const [f, es] of Object.entries(plan)) {
    const src = readFileSync(f, "utf8");
    for (const e of es) if (`${e.old}->${e.nw}` === sample) {
      const ls = src.lastIndexOf("\n", e.a) + 1; const le = src.indexOf("\n", e.a);
      console.log(f.slice(-60), "|", src.slice(ls, le).trim().slice(0, 170));
    }
  }
}
