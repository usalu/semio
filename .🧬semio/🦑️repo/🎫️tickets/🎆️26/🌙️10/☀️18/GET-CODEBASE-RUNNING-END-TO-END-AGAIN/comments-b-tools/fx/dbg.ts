import { applyRemovals } from "../keep.ts";
import { planTs, tsSignature } from "../ts-engine.ts";
import { readFileSync } from "node:fs";
const t = readFileSync(import.meta.dir + "/a.tsx", "utf8");
const n = applyRemovals(t, planTs("a.tsx", t).removals);
const a = tsSignature("a.tsx", t)!, b = tsSignature("a.tsx", n)!;
for (let i = 0; i < Math.max(a.length, b.length); i++) if (a[i] !== b[i]) { console.log(i, JSON.stringify(a[i]), JSON.stringify(b[i])); break; }
