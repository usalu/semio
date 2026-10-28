// sweep2-g: files bun's own import scanner (bare workspace specifiers resolved) reaches from entries but a key list lacks, minus the scheduling prefixes (keyed islands stay). usage: bun static-vs-key.mjs <key-list> <entry>...
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {staticClosure} from "./static-closure.mjs";
const LIB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const {buildSchedulingPrefixesV1, buildSchedulingKeyedPathsV1} = await import(join(LIB, "🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts"));
const extra = (process.env.SWEEP2_EXTRA_PREFIXES ?? "").split(",").filter(Boolean);
const prefixes = [...buildSchedulingPrefixesV1(), ...extra], keyed = buildSchedulingKeyedPathsV1();
const [list, ...entries] = process.argv.slice(2);
const key = new Set(readFileSync(list, "utf8").split("\n").filter(Boolean));
const closure = staticClosure(entries), scheduling = (p) => prefixes.some((x) => p.startsWith(x)) && !keyed.some((k) => p.startsWith(k));
const missing = closure.filter((p) => !key.has(p) && !scheduling(p));
console.log(JSON.stringify({closure: closure.length, scheduling: closure.filter(scheduling).length, missing: missing.length}));
for (const p of missing.slice(0, Number(process.env.SHOW ?? 40))) console.log("  -", p);
