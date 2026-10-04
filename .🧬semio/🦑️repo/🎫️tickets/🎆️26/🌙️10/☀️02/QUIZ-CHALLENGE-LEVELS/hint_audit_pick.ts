/** 📌️ Prints chosen records of the corpus in full, for the report: `bun hint_audit_pick.ts <substring>...` matches the English question. */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
for (const needle of process.argv.slice(2)) {
  const r = records.find((x) => x.en.includes(needle) || x.de.includes(needle));
  console.log(r ? `[${r.quiz}/${r.task}/${r.flaw}/${r.dimension ?? ""}] claim=${r.claim} truth=${r.truth}\n  EN ${r.en}\n  DE ${r.de}` : `MISSING ${needle}`);
}
const compare = records.filter((x) => x.kind === "compare");
console.log("claim<1 (subject is not the hinted item):", compare.filter((x) => x.claim < 1).length, "of", compare.length);
