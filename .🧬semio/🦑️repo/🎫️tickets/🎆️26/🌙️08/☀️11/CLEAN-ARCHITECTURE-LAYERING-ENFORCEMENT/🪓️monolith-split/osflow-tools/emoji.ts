// 🔣️ Checks candidate folder names: RGI emoji + U+FE0F, already used as a folder prefix in the repo, not repeated among siblings. usage: bun emoji.ts <siblingDir|-> name...
import {readFileSync,readdirSync,existsSync} from "node:fs";
const used = new Map<string, number>();
for (const n of readFileSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/osflow/dirnames.txt", "utf8").split("\n")) {
  const m = n.match(/^(.*?)️/u); if (m) used.set(m[1], (used.get(m[1]) ?? 0) + 1);
}
const [dir, ...names] = process.argv.slice(2);
const sib = new Set<string>(); if (dir && dir !== "-" && existsSync(dir)) for (const n of readdirSync(dir)) { const m = n.match(/^(.*?)️/u); if (m) sib.add(m[1]); }
for (const n of names) {
  const m = n.match(/^(.*?)️/u);
  if (!m) { console.log(n, "NO-FE0F"); continue; }
  const e = m[1];
  console.log(n, "rgi", (/^\p{RGI_Emoji}$/v.test(e) || /^\p{RGI_Emoji}$/v.test(e + "️")),"usedElsewhere", used.get(e) ?? 0, "siblingDup", sib.has(e));
}
