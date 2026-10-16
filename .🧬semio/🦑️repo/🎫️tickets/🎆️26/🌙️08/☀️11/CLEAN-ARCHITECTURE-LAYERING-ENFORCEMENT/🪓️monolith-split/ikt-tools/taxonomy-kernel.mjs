import fs from "node:fs";
const TX = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const [closureFile, mode] = process.argv.slice(2);
const add = fs.readFileSync(closureFile, "utf8").split("\n").filter(Boolean);
const lines = fs.readFileSync(TX, "utf8").split("\n");
const OLD = "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";
let changed = 0;
const out = [];
for (let i = 0; i < lines.length; i++) {
  const m = /^(\s*)"(🧰️framework\/🔨️modules\/🎠️kernel\/[^"]*)",?$/.exec(lines[i]);
  if (!m) { out.push(lines[i]); continue; }
  let j = i;
  const block = [];
  while (j < lines.length) {
    const mm = /^(\s*)"(🧰️framework\/🔨️modules\/🎠️kernel\/[^"]*)"(,?)$/.exec(lines[j]);
    if (!mm) break;
    block.push({ indent: mm[1], path: mm[2], comma: mm[3] });
    j++;
  }
  if (!block.some((b) => b.path === OLD)) { for (const b of block) out.push(`${b.indent}"${b.path}"${b.comma}`); i = j - 1; continue; }
  const last = block[block.length - 1];
  const set = new Set(block.map((b) => b.path).filter((p) => p !== OLD));
  for (const p of add) set.add(p);
  const sorted = [...set].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  sorted.forEach((p, k) => out.push(`${block[0].indent}"${p}"${k === sorted.length - 1 ? last.comma : ","}`));
  changed++;
  i = j - 1;
}
if (mode === "write") fs.writeFileSync(TX, out.join("\n"));
console.log("blocks changed", changed, mode);
