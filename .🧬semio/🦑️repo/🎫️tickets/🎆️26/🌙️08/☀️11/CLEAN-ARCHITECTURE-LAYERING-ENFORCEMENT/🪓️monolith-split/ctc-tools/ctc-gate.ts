// 🚪️ Rewrites the `#[cfg(test)]` gate above given declaration lines to `#[cfg(any(test, feature = "FEATURE"))]`: bun ctc-gate.ts FEATURE FILE LINE [LINE...] (LINE = the declaration line; the nearest attribute line above it is edited)
import {readFileSync, writeFileSync} from "node:fs";
const [feature, fileArg, ...lineArgs] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio/";
const file = fileArg!.startsWith("/") ? fileArg! : ROOT + fileArg!;
const text = readFileSync(file, "utf8");
const lines = text.split("\n");
let edited = 0;
for (const arg of lineArgs) {
  let i = Number(arg) - 1;
  let hit = -1;
  for (let j = i; j >= Math.max(0, i - 12); j--) {
    const l = lines[j]!.trim();
    if (l === "#[cfg(test)]") { hit = j; break; }
    if (l.startsWith("#[cfg(any(test, feature =") && l.includes(`"${feature}"`)) { hit = -2; break; }
    if (j < i && !(l.startsWith("#[") || l.startsWith("///") || l.startsWith("//") || l.startsWith("#!["))) break;
  }
  if (hit === -2) { console.log(`already gated at ${arg}`); continue; }
  if (hit < 0) throw new Error(`no #[cfg(test)] above line ${arg} of ${file}`);
  lines[hit] = lines[hit]!.replace("#[cfg(test)]", `#[cfg(any(test, feature = "${feature}"))]`);
  edited++;
}
writeFileSync(file, lines.join("\n"));
console.log(`edited ${edited} gate(s) in ${file}`);
