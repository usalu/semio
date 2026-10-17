// 🧹️ Re-gates every non-module, non-#[test] `#[cfg(test)]` item of a file: bun ctc-fileall.ts FEATURE FILE [--dry]. Lists what it touches.
import {readFileSync, writeFileSync} from "node:fs";
const [feature, fileArg] = process.argv.slice(2);
const dry = process.argv.includes("--dry");
const skipArg = process.argv.find(a => a.startsWith("--skip="));
const skip = skipArg ? new RegExp(skipArg.slice(7)) : null;
const file = fileArg!.startsWith("/") ? fileArg! : "/Users/ueli/Documents/semio/" + fileArg!;
const lines = readFileSync(file, "utf8").split("\n");
let n = 0;
for (let i = 0; i < lines.length; i++) {
  if (lines[i]!.trim() !== "#[cfg(test)]") continue;
  let j = i + 1, isTest = false;
  while (j < lines.length && /^\s*(#\[|\/\/)/.test(lines[j]!)) { if (/#\[(\w+::)*test(\(|\])/.test(lines[j]!) || /tokio::test|async_test/.test(lines[j]!)) isTest = true; j++; }
  const head = lines[j]?.trim() ?? "";
  const isMod = /^(pub(\([^)]*\))?\s+)?mod\s/.test(head) || head.startsWith("include!");
  if (isMod || isTest || (skip && skip.test(head))) continue;
  console.log(`${i + 1}: ${head.slice(0, 110)}`);
  if (!dry) lines[i] = lines[i]!.replace("#[cfg(test)]", `#[cfg(any(test, feature = "${feature}"))]`);
  n++;
}
if (!dry) writeFileSync(file, lines.join("\n"));
console.log(`${dry ? "would regate" : "regated"} ${n} item(s) in ${file}`);
