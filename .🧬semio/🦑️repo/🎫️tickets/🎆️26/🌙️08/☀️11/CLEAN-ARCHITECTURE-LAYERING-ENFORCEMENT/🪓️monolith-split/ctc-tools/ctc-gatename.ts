// 🏷️ Re-gates named test-only declarations of an owner crate: bun ctc-gatename.ts GATED_PREFIX OWNER FEATURE NAME... (NAME = item name, or Type::name for methods; "*" = every pure cfg(test) pub record). Edits the attached #[cfg(test)] or, for methods, the one on the enclosing impl.
import {readFileSync, writeFileSync} from "node:fs";
const [prefix, owner, feature, ...names] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio/";
const gated = JSON.parse(readFileSync(prefix + "-gated.json", "utf8")) as any[];
const files = new Map<string, string[]>();
const done = new Set<string>();
const NEW = `#[cfg(any(test, feature = "${feature}"))]`;
const attrAbove = (lines: string[], i: number): number => {
  for (let j = i - 1; j >= Math.max(0, i - 30); j--) {
    const l = lines[j]!.trim();
    if (l === "#[cfg(test)]") return j;
    if (l.startsWith("#[cfg(any(test, feature =") && l.includes(`"${feature}"`)) return -2;
    if (l.startsWith("#[") || l.startsWith("///") || l.startsWith("//") || l.startsWith("#![") || l === "") continue;
    return -1;
  }
  return -1;
};
let n = 0;
for (const r of gated) {
  if (r.crate !== owner || r.gate !== "test") continue;
  const key = (r.owner ? r.owner + "::" : "") + r.name;
  if (!names.includes("*") && !names.includes(key) && !names.includes(r.name)) continue;
  const path = ROOT + r.file;
  let lines = files.get(path);
  if (!lines) files.set(path, lines = readFileSync(path, "utf8").split("\n"));
  const indexLine = r.line - 1;
  let hit = attrAbove(lines, indexLine);
  if (hit === -1) {
    const indent = lines[indexLine]!.match(/^\s*/)![0].length;
    for (let j = indexLine - 1; j >= 0; j--) {
      const l = lines[j]!;
      if (/^\s*(pub(\([^)]*\))?\s+)?(unsafe\s+)?(impl|mod)\b/.test(l) && l.match(/^\s*/)![0].length < indent) { hit = attrAbove(lines, j); break; }
    }
  }
  if (hit === -2) { done.add(path + r.line); continue; }
  if (hit < 0) { console.log(`SKIP no gate found: ${r.file}:${r.line} ${key}`); continue; }
  lines[hit] = lines[hit]!.replace("#[cfg(test)]", NEW);
  n++;
}
for (const [path, lines] of files) writeFileSync(path, lines.join("\n"));
console.log(`re-gated ${n} declaration(s) of ${owner}`);
