// 🧰 bd10: adds the parent-module imports a hoisted test used through `use super::*` (reads the original mounters from the pre-hoist snapshot).
// usage: bun hoist-fixup.ts <spec.json> <snapshot-root> [--write]
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {walkCrate} from "./rust-mounts.ts";
import {parseRustSourceTree, useLeafNames, prunedUse, referencedItemNames} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {absolutizeSuperPaths} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";

const root = "/Users/ueli/Documents/semio";
const spec = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {crates: Record<string, {ident: string; root: string}>; files: {file: string; owner: string; companion: string}[]};
const snap = process.argv[3]!;
const write = process.argv.includes("--write");
const carryUses = (testText: string, parentText: string, parentModPath: readonly string[], ownerIdent: string): string[] => {
  const tree = parseRustSourceTree(parentText);
  const referenced = referencedItemNames(testText);
  const out: string[] = [];
  for (const item of tree.items) {
    if (item.kind !== "use" || item.visibility === "pub") continue;
    const statement = parentText.slice(item.declStart, item.end);
    const leaves = useLeafNames(statement).filter((name) => referenced.has(name) && !new RegExp(`\\buse\\b[^;]*\\b${name}\\b`, "u").test(testText));
    if (!leaves.length) continue;
    let text = prunedUse(statement, leaves);
    const abso = absolutizeSuperPaths(text, parentModPath);
    if (abso === null) continue;
    text = abso.replace(/(?<![\w:])crate::/gu, `${ownerIdent}::`).replace(/^\s*pub(\([^)]*\))?\s+/u, "");
    out.push(text.trim());
  }
  return out;
};
const insertUses = (text: string, uses: readonly string[]): string => {
  if (!uses.length) return text;
  const lines = text.split("\n");
  const at = lines.findIndex((line) => /^use /u.test(line));
  const block = uses.map((u) => (u.endsWith(";") ? u : u + ";"));
  if (at < 0) { const doc = lines.findLastIndex((line) => line.startsWith("//!")); lines.splice(doc + 1, 0, "", ...block); }
  else lines.splice(at, 0, ...block);
  return lines.join("\n");
};

const walks = new Map<string, ReturnType<typeof walkCrate>>();
for (const entry of spec.files) {
  const original = join(snap, entry.file);
  if (!existsSync(original)) continue;
  if (!walks.has(entry.owner)) walks.set(entry.owner, walkCrate(join(snap, spec.crates[entry.owner]!.root)));
  const walked = walks.get(entry.owner)!;
  const site = walked.sites.find((candidate) => candidate.target === original);
  if (!site) { console.log("NO SITE", entry.file); continue; }
  const live = readFileSync(join(root, entry.file), "utf8");
  const parentPath = walked.paths.get(site.file) ?? [];
  const parentGlob = new RegExp(`use\\s+${spec.crates[entry.owner]!.ident}${parentPath.map((segment) => `::${segment}`).join("")}::\\*;`, "u");
  if (!/(^|\n)\s*use\s+super::\*/u.test(readFileSync(original, "utf8")) && !parentGlob.test(live)) continue;
  const uses = carryUses(live, readFileSync(site.file, "utf8"), walked.paths.get(site.file) ?? [], spec.crates[entry.owner]!.ident);
  console.log(entry.file.split("✳️any/")[1], uses.length ? uses.join(" | ") : "-");
  if (write && uses.length) writeFileSync(join(root, entry.file), insertUses(live, uses));
}
