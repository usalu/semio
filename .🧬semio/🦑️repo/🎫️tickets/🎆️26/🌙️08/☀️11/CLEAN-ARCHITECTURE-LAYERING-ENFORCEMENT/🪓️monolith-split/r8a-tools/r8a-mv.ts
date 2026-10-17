// 🚚️ Moves top-level items (with their docs, attributes and impl blocks) from one Rust module file to another, carrying the imports they need, and repoints every `<src module>::NAME` path below a directory.
// usage: bun r8a-mv.ts <tag> <srcFile> <srcModPath> <dstFile> <dstModPath> <repathDir> name...   (no --write: plan only)
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { parseRustSourceTree, identifierSet, useLeafNames, prunedUse, rustItemStartWithComments } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const { rustTokens } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts");
const args = process.argv.slice(2);
const write = args.includes("--write");
const [tag, srcFile, srcPath, dstFile, dstPath, dir, ...names] = args.filter((a) => a !== "--write") as string[];
const src = readFileSync(srcFile, "utf8");
const dst = readFileSync(dstFile, "utf8");
const sTree = parseRustSourceTree(src);
type Block = { start: number; end: number; name: string | null };
const blocks: Block[] = [];
const wanted = new Set(names);
for (const item of sTree.items) {
  if (item.kind === "use" || item.kind === "extern-crate" || item.kind === "macro") continue;
  if (item.kind === "impl") {
    const header = src.slice(item.declStart, item.end).split("{")[0] ?? "";
    const hit = [...wanted].some((n) => new RegExp(`\\b${n}\\b`, "u").test(header.replace(/^impl(?:<[^>]*>)?\\s*/u, "")));
    if (!hit) continue;
    blocks.push({ start: rustItemStartWithComments(src, item.start), end: item.end, name: null });
  } else if (item.name && wanted.has(item.name)) blocks.push({ start: rustItemStartWithComments(src, item.start), end: item.end, name: item.name });
}
const found = new Set(blocks.filter((b) => b.name).map((b) => b.name));
for (const n of names) if (!found.has(n)) throw new Error(`item ${n} not found in ${srcFile}`);
blocks.sort((a, b) => a.start - b.start);
const movedText = blocks.map((b) => src.slice(b.start, b.end).replace(/^(?:pub\((?:crate|super|self|in [A-Za-z_:]+)\)\s+)?(?=(?:const |async |unsafe )*(?:fn|const|static|struct|enum|type|trait)\b)/mu, "pub ")).join("\n");
const ids = identifierSet(movedText);
const srcOwn = new Map<string, boolean>();
const moved = new Set(blocks.flatMap((b) => (b.name ? [b.name] : [])));
const useLines: string[] = [];
for (const item of sTree.items) {
  if (item.kind !== "use") continue;
  const statement = src.slice(item.declStart, item.end);
  const leaves = useLeafNames(statement).filter((l) => ids.has(l) && !moved.has(l));
  if (!leaves.length) continue;
  const pruned = prunedUse(statement.replace(/^(?:\s*#\[[^\]]*\]\s*)*/u, ""), leaves).replace(/^pub(?:\([^)]*\))?\s+/u, "");
  if (pruned) useLines.push((item.attributeTexts.some((a: string) => /cfg\s*\(\s*test\s*\)/u.test(a)) ? "#[cfg(test)] " : "") + pruned);
}
const stay: string[] = [];
for (const item of sTree.items) {
  if (item.kind === "use" || !item.name || item.kind === "impl" || moved.has(item.name)) continue;
  if (ids.has(item.name) && /^[A-Za-z_]/u.test(item.name)) stay.push(item.name);
}
const widenNames = stay.filter((n) => { const it = sTree.items.find((i: any) => i.name === n); return it && it.visibility !== "pub"; });
if (stay.length) useLines.push(`use ${srcPath}::{${stay.sort().join(", ")}};`);
// destination imports: skip leaves it already binds
const dTree = parseRustSourceTree(dst);
const dstBound = new Set<string>();
for (const item of dTree.items) { if (item.kind === "use") for (const l of useLeafNames(dst.slice(item.declStart, item.end))) dstBound.add(l); else if (item.name) dstBound.add(item.name); }
const dstNeeds = useLines.filter((line) => !/^#\[cfg\(test\)\]/u.test(line) || true).filter((line) => {
  const leaves = useLeafNames(line.replace(/^#\[cfg\(test\)\]\s*/u, ""));
  return leaves.some((l) => !dstBound.has(l));
});
let newDst = dst;
const lastUse = [...dTree.items].filter((i: any) => i.kind === "use").sort((a: any, b: any) => b.end - a.end)[0];
const insertAt = lastUse ? lastUse.end : (dst.match(/^(?:\/\/![^\n]*\n)+/u)?.[0].length ?? 0);
newDst = newDst.slice(0, insertAt) + (dstNeeds.length ? "\n" + dstNeeds.join("\n") : "") + newDst.slice(insertAt);
newDst = newDst.replace(/\n*$/u, "\n") + "\n" + movedText.trim() + "\n";
let newSrc = src;
for (const b of [...blocks].sort((a, b) => b.start - a.start)) newSrc = newSrc.slice(0, b.start) + newSrc.slice(b.end).replace(/^\n/u, "");
for (const n of widenNames) {
  const re = new RegExp(`^(?:pub\\((?:crate|super|self|in [A-Za-z_:]+)\\)\\s+)?((?:const |async |unsafe )*(?:fn|const|static|struct|enum|type|trait))\\s+${n}\\b`, "mu");
  newSrc = newSrc.replace(re, (_m, kw: string) => `pub ${kw} ${n}`);
}
if (!write) console.log(dstNeeds.join("\n"));
console.log(`${write ? "move" : "plan"} ${names.join(", ")}: ${blocks.length} block(s); dst imports ${dstNeeds.length}; widened ${widenNames.join(",") || "-"}`);
const edits = new Map<string, string>([[srcFile, newSrc], [dstFile, newDst]]);
const escaped = srcPath.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
const files: string[] = [];
const walk = (d: string): void => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (p.endsWith("🦀️.rs")) files.push(p); } };
walk(dir);
const alts = names.map((n) => n.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")).join("|");
for (const f of files) {
  let t = edits.get(f) ?? readFileSync(f, "utf8");
  const o = t;
  t = t.replace(new RegExp(`use\\s+${escaped}::\\{([^;]*?)\\};`, "gu"), (whole: string, body: string) => {
    const entries: string[] = []; let depth = 0, cur = "";
    for (const ch of body) { if (ch === "{") depth += 1; if (ch === "}") depth -= 1; if (ch === "," && depth === 0) { entries.push(cur.trim()); cur = ""; } else cur += ch; }
    if (cur.trim()) entries.push(cur.trim());
    const keep: string[] = [], move: string[] = [];
    for (const e of entries) { const m = /^([A-Za-z_]\w*)(\s+as\s+\w+)?$/u.exec(e); if (m && wanted.has(m[1]!)) move.push(e); else keep.push(e); }
    if (!move.length) return whole;
    return [keep.length ? `use ${srcPath}::{${keep.join(", ")}};` : "", `use ${dstPath}::{${move.join(", ")}};`].filter(Boolean).join("\n");
  });
  t = t.replace(new RegExp(`use\\s+${escaped}::(${alts})((?:\\s+as\\s+\\w+)?);`, "gu"), (_w: string, n: string, a: string) => `use ${dstPath}::${n}${a};`);
  t = t.replace(new RegExp(`(?<![\\w:])${escaped}::(${alts})\\b`, "gu"), (_w: string, n: string) => `${dstPath}::${n}`);
  if (t !== o) edits.set(f, t);
}
console.log(`files ${edits.size}`);
if (write) {
  const undo: Record<string, string> = {};
  for (const [f] of edits) undo[f] = readFileSync(f, "utf8");
  writeFileSync(`/Users/ueli/.cache/r8a/undo-${tag}-${Date.now()}.json`, JSON.stringify(undo));
  for (const [f, t] of edits) writeFileSync(f, t);
}
