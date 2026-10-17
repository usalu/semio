// 📌️ Shows declarations (file:line + 3 lines of source) of gated records: bun ctc-decl.ts PREFIX OWNER [name-substring]
import {readFileSync} from "node:fs";
const [prefix, owner, sub] = process.argv.slice(2);
const g = JSON.parse(readFileSync(prefix + "-gated.json", "utf8")) as any[];
for (const r of g) {
  if (r.crate !== owner || (sub && !r.name.includes(sub))) continue;
  const all = readFileSync("/Users/ueli/Documents/semio/" + r.file, "utf8").split("\n");
  console.log(`## ${r.file.split("/").slice(-3).join("/")}:${r.line} ${r.kind} ${r.owner ?? ""}::${r.name} [${r.gate}]`);
  for (let i = Math.max(0, r.line - 3); i < Math.min(all.length, r.line + 2); i++) console.log("  " + (i + 1) + ": " + all[i]!.slice(0, 150));
}
