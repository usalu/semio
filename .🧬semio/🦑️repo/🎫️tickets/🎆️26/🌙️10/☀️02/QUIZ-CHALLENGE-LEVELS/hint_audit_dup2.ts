/** 👯️ Prints one answer with an identical question in both dimensions. */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line)).filter((r) => r.dimension);
const by = new Map<string, any[]>();
for (const r of records) (by.get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`) ?? by.set(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`, []).get(`${r.quiz}/${r.task}/${r.seed}/${r.flaw}`)!).push(r);
let n = 0;
for (const [k, list] of by) if (list.length !== new Set(list.map((r) => r.en)).size && n++ < 1) { console.log(k); for (const r of list) console.log(`  [${r.dimension}] ${r.en}\n     ${r.de}`); }
