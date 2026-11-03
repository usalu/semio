import { readFileSync, readdirSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const L = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const [slug, d, pose, out] = process.argv.slice(2);
(globalThis as any).__dump = [];
let a: any;
for (const g of readdirSync(join(L, "🗂️catalog"), { withFileTypes: true }).filter((e) => e.isDirectory())) { const f = join(L, "🗂️catalog", g.name, slug!, "🔣️.json"); if (existsSync(f)) a = JSON.parse(readFileSync(f, "utf8")).asset; }
try { evaluate(a, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(d) } }); console.log("no error"); } catch (e) { console.log(String(e).slice(0, 90)); }
writeFileSync(out!, JSON.stringify((globalThis as any).__dump.slice(0, 1)));
