import fs from "node:fs";
const lines = fs.readFileSync(0, "utf8").split("\n");
const seg = new Set();
for (const l of lines) { const parts = l.split("/"); parts.pop(); for (const p of parts) seg.add(p); }
const re = /^(\p{RGI_Emoji})/v;
const emo = new Map();
for (const p of seg) { const m = re.exec(p); if (m) { const k = m[1].replace(/\uFE0F/g, ""); emo.set(k, (emo.get(k) || 0) + 1); } }
fs.writeFileSync(process.argv[2], JSON.stringify([...emo.entries()].sort((a, b) => b[1] - a[1])));
console.log("distinct emoji folder prefixes", emo.size);
