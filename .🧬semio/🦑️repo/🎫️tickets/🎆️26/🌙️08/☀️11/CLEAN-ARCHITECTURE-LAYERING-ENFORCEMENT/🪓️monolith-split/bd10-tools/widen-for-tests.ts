// 🧰 bd10: makes `pub` exactly the private items, struct fields and inherent methods of host files whose names a hoisted test file mentions.
// usage: bun widen-for-tests.ts --hosts=a.rs,b.rs --tests=t1.rs,t2.rs [--write]
import {readFileSync, writeFileSync} from "node:fs";
import {parseRustSourceTree} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import {rustTokens} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const arg = (name: string): string[] => (process.argv.find((a) => a.startsWith(`--${name}=`)) ?? "").slice(name.length + 3).split(",").filter(Boolean);
const write = process.argv.includes("--write");
const idents = new Set<string>();
for (const file of arg("tests")) for (const token of rustTokens(readFileSync(file, "utf8"))) if (token.kind === "identifier" || token.kind === "ident") idents.add(token.text);
if (!idents.size) { for (const file of arg("tests")) for (const m of readFileSync(file, "utf8").matchAll(/[A-Za-z_][A-Za-z0-9_]*/gu)) idents.add(m[0]); }
const kinds = new Set(["fn", "struct", "enum", "type", "const", "static", "trait", "union", "mod"]);
for (const file of arg("hosts")) {
  let text = readFileSync(file, "utf8");
  const tree = parseRustSourceTree(text);
  const edits: {start: number; end: number; text: string}[] = [];
  const changed: string[] = [];
  for (const item of tree.items) {
    if (kinds.has(item.kind) && item.name && idents.has(item.name) && (item.visibility === "private" || item.visibility === "pub(super)" || item.visibility === "pub(self)")) {
      const vis = /^pub(?:\([^)]*\))?\s*/u.exec(text.slice(item.declStart, item.declStart + 40));
      edits.push({start: item.declStart, end: item.declStart + (vis ? vis[0].length : 0), text: "pub "});
      changed.push(item.name);
    }
    if (item.kind === "struct" && item.name) {
      const vis = /^pub(?:\([^)]*\))?\s*/u.exec(text.slice(item.declStart, item.declStart + 40));
      const from = item.declStart + (vis ? vis[0].length : 0);
      const seg = text.slice(from, item.end);
      const widened = seg.replace(/(\{)([\s\S]*)(\}\s*)$/u, (_a, open: string, body: string, close: string) => `${(open + body).replace(/(^|[,{]\s*)((?:#\[[^\]]*\]\s*)*)(?:pub\([^)]*\)\s+)?(?!pub\b)([a-z_][a-z0-9_]*)(\s*:)/gmu, (m0: string, pre: string, attrs: string, name: string, colon: string) => (idents.has(name) ? (changed.push(`${item.name}.${name}`), `${pre}${attrs}pub ${name}${colon}`) : m0))}${close}`);
      if (widened !== seg) edits.push({start: from, end: item.end, text: widened});
    }
    if (item.kind === "impl") {
      const seg = text.slice(item.declStart, item.end);
      const head = /^\s*(?:#\[[^\n]*\]\s*)*impl(?:<[^>]*>)?\s+[^{]*\{/u.exec(seg);
      if (!head || /\bfor\b/u.test(head[0])) continue;
      const widened = seg.replace(/^(\s+)(?:pub\([^)]*\)\s+)?(?!pub\b)((?:async\s+|const\s+|unsafe\s+)*fn\s+)([a-z_][a-z0-9_]*)/gmu, (m0: string, ws: string, kw: string, name: string) => (idents.has(name) ? (changed.push(`.${name}()`), `${ws}pub ${kw}${name}`) : m0));
      if (widened !== seg) edits.push({start: item.declStart, end: item.end, text: widened});
    }
  }
  const merged = edits.sort((a, b) => b.start - a.start || b.end - a.end);
  for (const edit of merged) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
  if (changed.length) console.log(`${file.split("✳️any/")[1] ?? file}: ${[...new Set(changed)].join(", ")}`);
  if (write && changed.length) writeFileSync(file, text);
}
