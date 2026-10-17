// 🔪️ Splits a snapshot kit file into the part that implements methods on the entity and value types (stays below the snapshot root) and the part whose items take the whole model snapshot (the problems part, above the root). Dry run prints the split; --apply writes the new file and rewrites the kit.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { parseRustSourceTree, identifierSet } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
const [file, newDir, modName, ...flags] = process.argv.slice(2);
const apply = flags.includes("--apply");
const text = readFileSync(file!, "utf8");
const tree = parseRustSourceTree(text);
const items = tree.items.filter((item) => item.kind !== "use");
const regionStarts = [...text.matchAll(/^\/\/#(?:end)?region[^\n]*\n/gmu)];
const textOf = (item: (typeof items)[number]) => text.slice(item.start, item.end);
const names = new Map<number, string>();
items.forEach((item, index) => { if (item.name) names.set(index, item.name); });
const mentions = (index: number, name: string) => identifierSet(textOf(items[index]!)).has(name);
const isP = items.map((item) => identifierSet(textOf(item)).has("ModelSnapshot"));
let changed = true;
while (changed) {
  changed = false;
  items.forEach((_, i) => {
    if (isP[i]) return;
    for (let j = 0; j < items.length; j += 1) if (isP[j] && names.get(j) && mentions(i, names.get(j)!)) { isP[j] = false; changed = true; }
  });
}
const report = items.map((item, i) => `${isP[i] ? "P" : "I"} ${item.kind} ${item.name ?? headerOf(textOf(item))} (${textOf(item).split("\n").length}l)`);
function headerOf(t: string): string { return t.split("{")[0]!.replace(/\s+/gu, " ").slice(0, 60); }
const pLines = items.reduce((n, item, i) => n + (isP[i] ? textOf(item).split("\n").length : 0), 0);
const iLines = items.reduce((n, item, i) => n + (!isP[i] ? textOf(item).split("\n").length : 0), 0);
console.log(file!.split("/").slice(-2).join("/"), "I lines", iLines, "P lines", pLines);
if (flags.includes("--list")) console.log(report.join("\n"));
const badImpl = items.filter((item, i) => isP[i] && item.kind === "impl");
if (badImpl.length) console.log("WARNING P-side impl blocks:", badImpl.map((item) => headerOf(textOf(item))).join(" | "));
if (apply) {
  const uses = tree.items.filter((item) => item.kind === "use").map((item) => text.slice(item.declStart, item.end)).join("\n");
  const pText = items.filter((_, i) => isP[i]).map((item) => text.slice(item.start, item.end)).join("\n\n");
  const doc = "//! 🩺️ Problems and lookups of the `" + modName + "` kit that read the whole model snapshot; the kit itself keeps the methods of the entity and value types.\n\n";
  mkdirSync(newDir!, { recursive: true });
  writeFileSync(join(newDir!, "🦀️.rs"), doc + uses + "\nuse super::" + modName + "::*;\n\n" + pText + "\n");
  let out = text;
  const cuts = items.filter((_, i) => isP[i]).sort((a, b) => b.start - a.start);
  for (const cut of cuts) out = out.slice(0, cut.start) + out.slice(cut.end);
  out = out.replace(/\n{3,}/gu, "\n\n");
  writeFileSync(file!, out);
}
