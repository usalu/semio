/** 🎯️ Counts, per rule, the cruiser error lines of the 31 asset and JavaScript edges (matched by ASCII projection of importer and target). Usage: bun mine.ts <err.txt> [<err.txt>...] */
import { readFileSync } from "node:fs";
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const ascii = (s: string) => s.replace(/[^\x20-\x7e]/gu, "");
const rows = readFileSync(G + "/psc3-skipped-edges-after.tsv", "utf8").split("\n").filter(l => l.startsWith("open: unsupported asset or JavaScript target")).map(l => l.split("\t"));
const pairs = rows.map(r => ({ importer: ascii(r[2].normalize("NFC")), target: ascii(r[4].normalize("NFC")), line: r[3] }));
for (const file of process.argv.slice(2)) {
  const byRule = new Map<string, number>();
  const hit = new Set<number>();
  for (const l of readFileSync(file, "utf8").split("\n")) {
    const x = l.match(/^  error ([^:]+): (.*) → (.*)$/u);
    if (!x) continue;
    const imp = ascii(x[2]), tgt = ascii(x[3]);
    pairs.forEach((p, i) => { if (imp === p.importer && (tgt === p.target || tgt.endsWith(p.target.split("/").slice(-3).join("/")))) { hit.add(i); byRule.set(x[1], (byRule.get(x[1]) ?? 0) + 1); } });
  }
  console.log(file.split("/").pop(), "pairs matched:", hit.size, "of", pairs.length, JSON.stringify([...byRule].sort()));
  if (process.env.SHOW) pairs.forEach((p, i) => { if (!hit.has(i)) console.log("  unmatched:", p.importer.slice(-70), "->", p.target.slice(-70)); });
}
