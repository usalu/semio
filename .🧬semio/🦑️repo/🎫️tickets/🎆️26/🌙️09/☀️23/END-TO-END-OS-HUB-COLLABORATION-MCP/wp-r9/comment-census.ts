#!/usr/bin/env bun
/** 🔎️ R9 item 6: parser-based census of comments inside Rust definitions (tree-sitter-rust via web-tree-sitter).
 * Every comment node is classified by its nearest enclosing construct: `fn-body` (function/closure bodies and
 * const/static initializers), `type-body` (struct fields, enum variants), `impl-between-items` (impl/trait blocks
 * between members), `macro-body` (macro_rules! arms and item-level macro token trees), `module-level` (allowed:
 * between items of a file or inline module). Doc comments (`///`, `//!`, `/**`, `/*!`) are counted apart.
 * Usage: `bun comment-census.ts <out.json> <root>...` — prints totals per root and the top files. */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const Parser = (await import(`${REPO}/node_modules/web-tree-sitter/tree-sitter.js`)).default;
await Parser.init();
const rust = await Parser.Language.load(`${REPO}/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm`);
const parser = new Parser();
parser.setLanguage(rust);

type Category = "fn-body" | "type-body" | "impl-between-items" | "macro-body" | "module-level";
type Hit = { line: number; category: Category; doc: boolean; text: string; owner: string };
const SKIP = new Set(["node_modules", "dist", "target", "🗑️generated", "🤖️generated", ".git"]);

function files(root: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(root)) {
    if (SKIP.has(name)) continue;
    const path = join(root, name);
    const stat = statSync(path);
    if (stat.isDirectory()) out.push(...files(path));
    else if (name.endsWith(".rs")) out.push(path);
  }
  return out;
}

function classify(node: any): { category: Category; owner: string } {
  for (let current = node.parent; current; current = current.parent) {
    switch (current.type) {
      case "block":
      case "closure_expression":
        for (let up = current; up; up = up.parent) {
          if (up.type === "function_item" || up.type === "function_signature_item") return { category: "fn-body", owner: up.childForFieldName("name")?.text ?? "fn" };
          if (up.type === "const_item" || up.type === "static_item") return { category: "fn-body", owner: up.childForFieldName("name")?.text ?? "const" };
          if (up.type === "macro_definition") return { category: "macro-body", owner: up.childForFieldName("name")?.text ?? "macro" };
        }
        return { category: "fn-body", owner: "block" };
      case "const_item":
      case "static_item":
        return { category: "fn-body", owner: current.childForFieldName("name")?.text ?? "const" };
      case "field_declaration_list":
      case "ordered_field_declaration_list":
      case "enum_variant_list":
        return { category: "type-body", owner: current.parent?.childForFieldName("name")?.text ?? current.type };
      case "macro_definition":
        return { category: "macro-body", owner: current.childForFieldName("name")?.text ?? "macro" };
      case "token_tree":
        continue;
      case "macro_invocation":
        return { category: "macro-body", owner: current.childForFieldName("macro")?.text ?? "macro!" };
      case "declaration_list": {
        const parent = current.parent;
        if (parent?.type === "impl_item") return { category: "impl-between-items", owner: `impl ${parent.childForFieldName("type")?.text ?? ""}` };
        if (parent?.type === "trait_item") return { category: "impl-between-items", owner: `trait ${parent.childForFieldName("name")?.text ?? ""}` };
        return { category: "module-level", owner: parent?.childForFieldName("name")?.text ?? "mod" };
      }
      case "source_file":
        return { category: "module-level", owner: "file" };
    }
  }
  return { category: "module-level", owner: "file" };
}

function census(path: string): Hit[] {
  const tree = parser.parse(readFileSync(path, "utf8"));
  const hits: Hit[] = [];
  const cursor = tree.walk();
  const visit = (): void => {
    do {
      const node = cursor.currentNode();
      if (node.type === "line_comment" || node.type === "block_comment") {
        const text = node.text as string;
        const doc = /^(\/\/\/(?!\/)|\/\/!|\/\*\*(?!\/)|\/\*!)/u.test(text);
        const { category, owner } = classify(node);
        hits.push({ line: node.startPosition.row + 1, category, doc, text: text.slice(0, 160), owner });
      }
      if (cursor.gotoFirstChild()) {
        visit();
        cursor.gotoParent();
      }
    } while (cursor.gotoNextSibling());
  };
  visit();
  tree.delete();
  return hits;
}

const [out, ...roots] = process.argv.slice(2);
const report: Record<string, { files: number; totals: Record<string, number>; byFile: Record<string, Record<string, number>>; hits: Record<string, Hit[]> }> = {};
for (const root of roots) {
  const entry = { files: 0, totals: {} as Record<string, number>, byFile: {} as Record<string, Record<string, number>>, hits: {} as Record<string, Hit[]> };
  for (const path of files(join(REPO, root))) {
    entry.files += 1;
    const rel = relative(REPO, path);
    for (const hit of census(path)) {
      const key = `${hit.category}${hit.doc ? ":doc" : ""}`;
      entry.totals[key] = (entry.totals[key] ?? 0) + 1;
      (entry.byFile[rel] ??= {})[key] = (entry.byFile[rel]![key] ?? 0) + 1;
      if (!hit.doc && hit.category !== "module-level") (entry.hits[rel] ??= []).push(hit);
    }
  }
  report[root] = entry;
  console.log(`${root}: files=${entry.files} ${JSON.stringify(entry.totals)}`);
  const top = Object.entries(entry.byFile).map(([file, counts]) => [file, (counts["fn-body"] ?? 0) + (counts["type-body"] ?? 0) + (counts["impl-between-items"] ?? 0) + (counts["macro-body"] ?? 0)] as const).sort((a, b) => b[1] - a[1]).slice(0, 8);
  for (const [file, count] of top) console.log(`  ${count}\t${file}`);
}
if (out) writeFileSync(out, JSON.stringify(report, null, 1));
