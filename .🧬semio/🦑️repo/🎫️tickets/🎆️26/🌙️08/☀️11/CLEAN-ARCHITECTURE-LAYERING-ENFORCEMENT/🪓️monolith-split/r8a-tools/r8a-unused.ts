// 🧹 Drops unused leaves of private top-level `use` statements; a leaf stays when the file, a cfg(test) child file, or a glob-importing child names it. usage: bun r8a-unused.ts <dir> [--write] [--tag=x]
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { parseRustSourceTree, identifierSet, useLeafNames, prunedUse } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const args = process.argv.slice(2);
const write = args.includes("--write");
const tag = args.find((a) => a.startsWith("--tag="))?.slice(6) ?? "unused";
const dir = args.find((a) => !a.startsWith("--"))!;
const files: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d)) { if (e === "node_modules" || e === "target" || e === "📦️packages" || e === "🤖️generated") continue; const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (e === "🦀️.rs") files.push(p); } };
walk(dir);
const undo: Record<string, string> = {};
let touched = 0, dropped = 0;
const childTexts = (file: string, src: string): string => {
  const out: string[] = [];
  const re = /#\[cfg\(test\)\]\s*#\[path\s*=\s*"([^"]+)"\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;/gu;
  for (const m of src.matchAll(re)) { const p = resolve(dirname(file), m[1]!); if (existsSync(p) && statSync(p).isFile()) { const t = readFileSync(p, "utf8"); out.push(t); if (m[1]!.includes("🧪️tests") || true) out.push(childTexts(p, t)); } }
  return out.join("\n");
};
for (const file of files) {
  if (file.includes("/🧪️tests/")) continue;
  const src = readFileSync(file, "utf8");
  const tree = parseRustSourceTree(src);
  const edits: { start: number; end: number; text: string }[] = [];
  const kids = childTexts(file, src);
  const kidIds: Set<string> = identifierSet(kids);
  for (const item of tree.items) {
    if (item.kind !== "use" || item.visibility !== "private") continue;
    const statement = src.slice(item.declStart, item.end);
    if (/\*\s*[;,}]|\bas\s+_/u.test(statement) || /#\[(?:macro_use|allow\(unused)/u.test(statement)) continue;
    const leaves: string[] = useLeafNames(statement);
    if (!leaves.length) continue;
    const rest = src.slice(0, item.declStart) + " " + src.slice(item.end);
    const restClean = rest.replace(/\/\/[^\n]*/gu, "").replace(/"(?:[^"\\]|\\.)*"/gu, '""');
    const usedIn = (text: string, l: string): boolean => new RegExp(`(?<![\\w.:])${l}\\b`, "u").test(text);
    const used = leaves.filter((l) => usedIn(restClean, l) || usedIn(kids.replace(/\/\/[^\n]*/gu, ""), l));
    if (used.length === leaves.length) continue;
    const attrs = item.attributeTexts?.length ? "" : "";
    if (!used.length) { edits.push({ start: item.declStart, end: item.end + (src[item.end] === "\n" ? 1 : 0), text: "" }); dropped += leaves.length; }
    else { const pr = prunedUse(statement.replace(/^(?:\s*#\[[^\]]*\]\s*)*/u, ""), used); edits.push({ start: item.declStart, end: item.end, text: pr }); dropped += leaves.length - used.length; }
  }
  if (!edits.length) continue;
  let out = src;
  for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
  touched += 1; undo[file] = src;
  if (write) writeFileSync(file, out);
}
if (write) writeFileSync(`/Users/ueli/.cache/r8a/undo-${tag}-${Date.now()}.json`, JSON.stringify(undo));
console.log(`${write ? "wrote" : "plan"}: ${touched} files, ${dropped} unused leaves`);
