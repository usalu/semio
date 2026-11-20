/** 🌱️ Writes the root fixes of the round-3 japanese maple from the candidate document into the catalog asset file: only the members named on the command line, each in the file's base style (`JSON.stringify` with a one-space indent, the form of its untouched members), nothing else; the file is re-read right before writing and every other member must stay equal and in order.
 * Usage: bun r3-japanese-maple-roots.ts <candidate 🔣️.json> <catalog 🔣️.json> <member>... [--dry]
 * @see ./r3-vegetation-japanese-maple.md */
import { readFileSync, writeFileSync } from "node:fs";

const args = process.argv.slice(2).filter((a) => a !== "--dry");
const [candidatePath, catalogPath, ...members] = args as [string, string, ...string[]];
const candidate = JSON.parse(readFileSync(candidatePath, "utf8")).asset;
const text = readFileSync(catalogPath, "utf8");
const before = JSON.parse(text);
const indent = /\n( +)"id"\s*:/.exec(text)![1]!;

/** 📏️ The character range of a member of the asset object (from its indent to the end of its value). */
function range(source: string, key: string): [number, number] {
  const found = new RegExp(`\\n${indent}"${key}"\\s*:\\s*`).exec(source);
  if (!found) throw new Error(`member ${key} not found`);
  let depth = 0, i = found.index + found[0].length, quoted = false;
  for (; i < source.length; i++) {
    const c = source[i]!;
    if (quoted) {
      if (c === "\\") i++;
      else if (c === '"') quoted = false;
    } else if (c === '"') quoted = true;
    else if (c === "{" || c === "[") depth++;
    else if (c === "}" || c === "]") if (--depth === 0) return [found.index + 1, i + 1];
  }
  throw new Error("unbalanced asset file");
}
const styled = (key: string, value: unknown): string => `${indent}"${key}": ${JSON.stringify(value, null, 1).replace(/\n/g, `\n${indent}`)}`;
for (const key of ["rig", "anchors", "thermal", "poses"]) {
  const [s, e] = range(text, key);
  if (text.slice(s, e) !== styled(key, before.asset[key])) console.log(`[DEBUG] untouched member ${key} is not in the base style`);
}
let result = text;
for (const m of members) {
  const [s, e] = range(result, m);
  result = result.slice(0, s) + styled(m, candidate[m]) + result.slice(e);
}
const after = JSON.parse(result);
for (const key of Object.keys(before.asset)) {
  const same = JSON.stringify(after.asset[key]) === JSON.stringify(members.includes(key) ? candidate[key] : before.asset[key]);
  if (!same) throw new Error(`member ${key} ${members.includes(key) ? "did not round-trip" : "changed"}`);
}
if (JSON.stringify(Object.keys(after.asset)) !== JSON.stringify(Object.keys(before.asset))) throw new Error("member order changed");
if (process.argv.includes("--dry")) console.log(`dry run ok: ${members.join(", ")} ${result === text ? "unchanged" : "changed"}`);
else {
  if (readFileSync(catalogPath, "utf8") !== text) throw new Error("asset changed while writing; retry");
  writeFileSync(catalogPath, result);
  console.log(`written: ${members.join(", ")}`);
}
