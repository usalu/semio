/** ↩️ Counts reversed over-hints by shown size and by reading hazard. */
import { readFileSync } from "node:fs";
const OUT = new URL("./🗑️generated/hint-audit/", import.meta.url);
const records = readFileSync(new URL("hints.jsonl", OUT), "utf8").split("\n").map((line) => JSON.parse(line)).filter((r) => r.kind === "compare");
const shown = (r: any): number => { const m = r.en.match(/Are you sure (?:it takes )?([\d.,]+) ×/u) ?? r.en.match(/(?:only|as much as) ([\d.,]+) times/u); return Number(m[1].replace(/,/gu, "")); };
const rev = records.filter((r) => !r.under && (r.claim >= 1) !== (r.truth >= 1));
const additive = (r: any) => / ×/u.test(r.en);
console.log("reversed over, ratio form (not additive):", rev.filter((r) => !additive(r)).length, "of which shown < 2:", rev.filter((r) => !additive(r) && shown(r) < 2).length, "< 5:", rev.filter((r) => !additive(r) && shown(r) < 5).length);
console.log("reversed over, additive:", rev.filter(additive).length);
console.log("reversed AND truth/claim extreme (>100x error):", rev.filter((r) => Math.max(r.claim / r.truth, r.truth / r.claim) > 100).length);
const big = records.filter((r) => /\d[\d.,]{12,}/u.test(r.en));
console.log("counts >= 13 chars:", big.length);
const mism = records.filter((r) => r.under && r.claim < 1);
console.log("under with claim<1 (swapped roles):", mism.length);
