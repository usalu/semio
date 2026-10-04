/** 🎲️ Prints a stratified sample of the corpus of `hint_audit_probe.ts` by quiz, task and hint kind: `bun hint_audit_sample.ts [filter] [count]`. */
import { readFileSync } from "node:fs";

const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line));
const filter = process.argv[2] ?? "";
const count = Number(process.argv[3] ?? 12);
const groups = new Map<string, any[]>();
for (const r of records) {
  const key = `${r.quiz}/${r.task}/${r.kind}${r.under === undefined ? "" : r.under ? "/under" : "/over"}`;
  if (!key.includes(filter)) continue;
  (groups.get(key) ?? groups.set(key, []).get(key)!).push(r);
}
for (const [key, list] of [...groups].sort()) {
  const seen = new Set<string>();
  const distinct = list.filter((r) => (seen.has(r.en) ? false : (seen.add(r.en), true)));
  console.log(`\n=== ${key}: ${list.length} records, ${distinct.length} distinct`);
  const step = Math.max(1, Math.floor(distinct.length / count));
  for (let i = 0; i < distinct.length && i < count * step; i += step) {
    const r = distinct[i];
    console.log(`- (${r.flaw}) ${r.en}\n  ${r.de}${r.claim === undefined ? "" : `\n  claim ${Number(r.claim.toPrecision(3))}  truth ${Number(r.truth.toPrecision(3))}`}`);
  }
}
