// usage: node emojiused.cjs [candidate...]  : set of leading-emoji prefixes of directory names across tracked files; with candidates prints used?/count
const fs = require("fs");
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const lines = fs.readFileSync(G + "/pfs-gitfiles.txt", "utf8").split("\n").filter(Boolean);
const seg = new Intl.Segmenter("en", { granularity: "grapheme" });
const counts = new Map();
const seen = new Set();
for (const l of lines) {
  const parts = l.split("/"); parts.pop();
  let acc = "";
  for (const p of parts) { acc += "/" + p; if (seen.has(acc)) continue; seen.add(acc);
    const first = [...seg.segment(p)][0]?.segment; if (!first) continue;
    if (/\p{Extended_Pictographic}/u.test(first)) counts.set(first.replace(/️/g, ""), (counts.get(first.replace(/️/g, "")) || 0) + 1);
  }
}
const cands = process.argv.slice(2);
if (cands.length) for (const c of cands) console.log(c, counts.get(c.replace(/️/g, "")) || 0);
else console.log(counts.size, "distinct");
