import { readFileSync, writeFileSync } from "node:fs";
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of ["agave", "bamboo-clump", "hosta-clump", "ivy-on-trellis"]) {
  const file = `${CAT}/${d}/🔣️.json`; const t = readFileSync(file, "utf8"); const indent = t.split("\n")[1]!.match(/^ */)![0].length; const doc = JSON.parse(t); const r = doc.asset.representations; const order = ["50", "100", "200", "500"].filter((k) => r[k].status === "authored");
  for (let i = 1; i < order.length; i++) { const prev = r[order[i - 1]!].validRange.maxDenominator; const cur = r[order[i]!].validRange; if (cur.minDenominator !== prev) { console.log(d, order[i], cur.minDenominator, "->", prev); cur.minDenominator = prev; } }
  writeFileSync(file, JSON.stringify(doc, null, indent) + "\n");
}
