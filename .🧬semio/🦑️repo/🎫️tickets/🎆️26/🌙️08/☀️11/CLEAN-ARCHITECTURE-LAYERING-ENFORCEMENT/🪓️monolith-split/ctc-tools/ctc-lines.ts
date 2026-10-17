// 🎯️ Replaces `#[cfg(test)]` / `#[cfg(not(test))]` on exactly the given 1-based lines with the testkit forms: bun ctc-lines.ts FEATURE FILE LINE...
import {readFileSync, writeFileSync} from "node:fs";
const [feature, fileArg, ...ls] = process.argv.slice(2);
const file = fileArg!.startsWith("/") ? fileArg! : "/Users/ueli/Documents/semio/" + fileArg!;
const lines = readFileSync(file, "utf8").split("\n");
for (const l of ls) {
  const i = Number(l) - 1;
  const t = lines[i]!.trim();
  if (t === "#[cfg(test)]") lines[i] = lines[i]!.replace("#[cfg(test)]", `#[cfg(any(test, feature = "${feature}"))]`);
  else if (t === "#[cfg(not(test))]") lines[i] = lines[i]!.replace("#[cfg(not(test))]", `#[cfg(not(any(test, feature = "${feature}")))]`);
  else throw new Error(`line ${l} is not a test cfg: ${lines[i]}`);
}
writeFileSync(file, lines.join("\n"));
console.log(`regated ${ls.length} line(s) in ${file}`);
