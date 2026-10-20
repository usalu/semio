import { build } from "./b5-debug.mjs";
import { entries } from "./lib.mjs";
import { syntaxErrors } from "./check-docs.mjs";
import { spawnSync } from "node:child_process";
const r = build();
console.log(r.count, r.notes);
let bad = 0;
for (const [p, v] of entries()) {
  if (p.endsWith(".rs")) {
    const out = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: v.after, encoding: "utf8", maxBuffer: 1 << 28 });
    const before = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: v.before, encoding: "utf8", maxBuffer: 1 << 28 });
    if (out.status !== 0 || before.status !== 0) { bad++; console.log("RS", out.status, before.status, p.slice(-60), out.stderr.slice(0, 300)); }
  } else {
    const e = syntaxErrors(p, v.after);
    if (e.length) { bad++; console.log("TS", p.slice(-60), e.join(";")); }
  }
}
console.log("files", entries().length, "bad", bad);
