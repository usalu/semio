// 🔬️ Prints the consumer source lines behind hits: bun ctc-ctx.ts PREFIX OWNER ITEM_SUBSTRING [maxLines]
import {readFileSync} from "node:fs";
const [prefix, owner, item, max] = process.argv.slice(2);
const hits = JSON.parse(readFileSync(prefix + "-hits.json", "utf8")) as any[];
const sel = hits.filter(h => !h.sanctioned && h.owner === owner && h.item.includes(item!));
const seen = new Set<string>();
let n = 0;
for (const h of sel) {
  const k = h.file + ":" + h.line;
  if (seen.has(k)) continue;
  seen.add(k);
  if (n++ >= Number(max ?? 6)) break;
  let text = "";
  try {
    const all = readFileSync("/Users/ueli/Documents/semio/" + h.file, "utf8").split("\n");
    if (h.line > 0) text = all[h.line - 1] ?? "";
    else {
      const m = h.item.split(" ")[0].split("::").pop();
      const owner = h.item.split("::")[0];
      const ix = all.map((l, i) => [l, i] as const).filter(([l]) => l.includes("." + m + "(")).slice(0, 2).map(([l, i]) => `${i + 1}: ${l.trim().slice(0, 110)}`);
      const has = all.some(l => l.includes(owner));
      text = `[owner mentioned: ${has}] ` + ix.join(" || ");
    }
  } catch {}
  console.log(`${h.class}\t${h.consumerCrate}\t${h.file.split("/").slice(-3).join("/")}:${h.line}\t${text.trim().slice(0, 160)}`);
}
console.log(`-- ${sel.length} hits`);
