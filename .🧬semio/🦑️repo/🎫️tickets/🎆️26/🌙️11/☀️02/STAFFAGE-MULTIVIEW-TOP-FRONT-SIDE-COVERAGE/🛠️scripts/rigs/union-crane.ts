import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const L = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
(globalThis as any).__dump = [];
const a = JSON.parse(readFileSync(join(L, "🗂️catalog/🏗️machinery/tower-crane/🔣️.json"), "utf8")).asset;
try { evaluate(a, { kind: "pose", pose: "parked-weathervane" }, { kind: "scale", scale: { denominator: 500 } }); } catch (e) { console.log(String(e).slice(0, 80)); }
writeFileSync(process.argv[2]!, JSON.stringify((globalThis as any).__dump.slice(0, 1)));
