#!/usr/bin/env bun
/** 🪝️ R9 item 6 codemod: moves every comment that sits inside a Rust definition (function/closure bodies,
 * const/static initializers, struct fields, enum variants, impl/trait members other than `//#region` markers)
 * into the native docstring of the definition it explains (AGENTS.md: no comments inside definitions; rationale
 * and links belong in the docstring). Parser: tree-sitter-rust (web-tree-sitter). A block of consecutive comment
 * lines becomes one docstring paragraph of its enclosing function/const/static, or of the next member/field when
 * it sits between members. A definition without a docstring gets one; if the moved text does not start with an
 * emoji the file path is reported as `needs-emoji` and nothing in that file is written. `//#region` markers inside a
 * function body carry only a fold label and are deleted, never hoisted.
 * Usage: `bun comment-hoist.ts [--apply] <root>...` (dry run prints per-file block counts). */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const Parser = (await import(`${REPO}/node_modules/web-tree-sitter/tree-sitter.js`)).default;
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(`${REPO}/node_modules/tree-sitter-wasms/out/tree-sitter-rust.wasm`));
const SKIP = new Set(["node_modules", "dist", "target", "🗑️generated", "🤖️generated", ".git"]);
const EMOJI = /^\p{Extended_Pictographic}/u;
const OUTER_DOC = /^\/\/\/(?!\/)/u;

/** 🎨️ The leading emoji chosen by hand for each definition that receives its first docstring from a moved comment. */
const EMOJI_CHOICES: Record<string, string> = {
  "🌎️hub/🏗️bootstrap/🦀️.rs:4134": "🔏️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:89": "🪞️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:148": "⚔️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:174": "🔁️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:271": "🚫️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:305": "🚧️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:327": "🕰️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:401": "↩️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs:427": "💥️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🛡️policy/🧪️tests/🔬️quick/🦀️.rs:102": "⏳️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎫️handles/🧪️tests/🔬️quick/🦀️.rs:66": "⏱️",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🧪️tests/🔬️quick/🦀️.rs:123": "✂️",
};

type Target = { node: any; key: string };
type Block = { target: Target; rows: number[]; text: string[]; trailing: Map<number, string> };

function walkFiles(root: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(root)) {
    if (SKIP.has(name)) continue;
    const path = join(root, name);
    if (statSync(path).isDirectory()) out.push(...walkFiles(path));
    else if (name.endsWith(".rs")) out.push(path);
  }
  return out;
}

function commentText(raw: string): string[] {
  if (raw.startsWith("/*")) return raw.replace(/^\/\*+!?/u, "").replace(/\*+\/$/u, "").split("\n").map((line) => line.replace(/^\s*\*? ?/u, "").trimEnd()).filter((line, index, all) => line.length > 0 || (index > 0 && index < all.length - 1));
  return [raw.replace(/^\/\/[/!]?\s?/u, "").trimEnd()];
}

function nextMember(node: any): any {
  for (let next = node.nextNamedSibling; next; next = next.nextNamedSibling) if (next.type !== "line_comment" && next.type !== "block_comment" && next.type !== "attribute_item") return next;
  return null;
}

function targetOf(node: any): Target | null {
  for (let current = node.parent, child = node; current; child = current, current = current.parent) {
    if (current.type === "function_item" || current.type === "const_item" || current.type === "static_item") return { node: current, key: `${current.startIndex}` };
    if (current.type === "field_declaration_list" || current.type === "enum_variant_list") {
      const member = child === node ? nextMember(node) : null;
      return member ? { node: member, key: `${member.startIndex}` } : { node: current.parent, key: `${current.parent.startIndex}` };
    }
    if (current.type === "declaration_list" && (current.parent?.type === "impl_item" || current.parent?.type === "trait_item")) {
      if (child !== node) continue;
      if (/^\/\/\s*#(end)?region/u.test(node.text)) return null;
      const member = nextMember(node);
      return member ? { node: member, key: `${member.startIndex}` } : null;
    }
    if (current.type === "declaration_list" || current.type === "source_file") return null;
    if (current.type === "macro_definition") return null;
  }
  return null;
}

function leadingRun(item: any): { docRows: number[]; startRow: number } {
  const docRows: number[] = [];
  let startRow = item.startPosition.row;
  for (let previous = item.previousNamedSibling; previous; previous = previous.previousNamedSibling) {
    const isDoc = previous.type === "line_comment" && OUTER_DOC.test(previous.text);
    if (!(isDoc || previous.type === "attribute_item") || previous.endPosition.row < startRow - 1) break;
    if (isDoc) docRows.push(previous.startPosition.row);
    startRow = previous.startPosition.row;
  }
  return { docRows: docRows.reverse(), startRow };
}

function hoist(path: string, apply: boolean): { blocks: number; needsEmoji: string[] } {
  const source = readFileSync(path, "utf8");
  const lines = source.split("\n");
  const tree = parser.parse(source);
  const comments: any[] = [];
  const cursor = tree.walk();
  const visit = (): void => {
    do {
      const node = cursor.currentNode();
      if ((node.type === "line_comment" || node.type === "block_comment") && !/^(\/\/!|\/\*\*(?!\/)|\/\*!)/u.test(node.text)) comments.push(node);
      if (cursor.gotoFirstChild()) {
        visit();
        cursor.gotoParent();
      }
    } while (cursor.gotoNextSibling());
  };
  visit();
  const blocks: Block[] = [];
  const regionRows = new Set<number>();
  for (const node of comments) {
    if (OUTER_DOC.test(node.text) && !(node.parent?.type === "block")) continue;
    const target = targetOf(node);
    if (!target) continue;
    const row = node.startPosition.row;
    const line = lines[row]!;
    if (/^\/\/\s*#(end)?region/u.test(node.text) && line.trim() === node.text.trim()) {
      regionRows.add(row);
      continue;
    }
    const whole = line.trim() === node.text.trim() || (node.endPosition.row > row && line.trim().startsWith("/*"));
    const last = blocks.at(-1);
    const text = commentText(node.text);
    const block = last && last.target.key === target.key && whole && last.rows.at(-1) === row - 1 && !last.trailing.size ? last : { target, rows: [], text: [], trailing: new Map() };
    if (block !== last) blocks.push(block);
    if (whole) for (let r = row; r <= node.endPosition.row; r++) block.rows.push(r);
    else block.trailing.set(row, node.text);
    block.text.push(...text);
  }
  const needsEmoji: string[] = [];
  const insertBefore = new Map<number, string[]>();
  const deleted = new Set<number>(regionRows);
  const trimmed = new Map<number, string>();
  const paragraphs = new Map<string, { item: any; texts: string[][] }>();
  for (const block of blocks) {
    const entry = paragraphs.get(block.target.key) ?? { item: block.target.node, texts: [] };
    entry.texts.push(block.text);
    paragraphs.set(block.target.key, entry);
    for (const row of block.rows) deleted.add(row);
    for (const [row, raw] of block.trailing) {
      const line = trimmed.get(row) ?? lines[row]!;
      const at = line.lastIndexOf(raw);
      trimmed.set(row, line.slice(0, at).trimEnd());
    }
  }
  for (const { item, texts } of paragraphs.values()) {
    const { docRows, startRow } = leadingRun(item);
    const indent = /^\s*/u.exec(lines[startRow]!)![0];
    const first = texts[0]![0] ?? "";
    const chosen = docRows.length === 0 ? EMOJI_CHOICES[`${relative(REPO, path)}:${item.startPosition.row + 1}`] : undefined;
    if (chosen) texts[0]![0] = `${chosen} ${first.charAt(0).toUpperCase()}${first.slice(1)}`;
    const body = texts.map((text) => text.map((line) => `${indent}///${line ? ` ${line}` : ""}`).join("\n")).join(`\n${indent}///\n`).split("\n");
    if (docRows.length === 0) {
      if (!chosen && !EMOJI.test(first)) needsEmoji.push(`${relative(REPO, path)}:${item.startPosition.row + 1} ${first.slice(0, 60)}`);
      insertBefore.set(startRow, [...(insertBefore.get(startRow) ?? []), ...body]);
    } else {
      const after = docRows.at(-1)! + 1;
      insertBefore.set(after, [...(insertBefore.get(after) ?? []), `${indent}///`, ...body]);
    }
  }
  tree.delete();
  if (blocks.length === 0 && regionRows.size === 0) return { blocks: 0, needsEmoji };
  const out: string[] = [];
  for (let row = 0; row < lines.length; row++) {
    for (const inserted of insertBefore.get(row) ?? []) out.push(inserted);
    if (deleted.has(row)) continue;
    out.push(trimmed.get(row) ?? lines[row]!);
  }
  if (apply && needsEmoji.length === 0) writeFileSync(path, out.join("\n"));
  if (preview !== undefined && relative(REPO, path) === preview) writeFileSync(`${REPO}/.tmp-ticket/wp-r9/generated/comment-hoist-preview.rs`, out.join("\n"));
  return { blocks: blocks.length + regionRows.size, needsEmoji };
}

const apply = process.argv.includes("--apply");
const previewAt = process.argv.indexOf("--preview");
const preview = previewAt >= 0 ? process.argv[previewAt + 1] : undefined;
const roots = process.argv.slice(2).filter((arg, index, all) => arg !== "--apply" && arg !== "--preview" && all[index - 1] !== "--preview");
let total = 0;
const pending: string[] = [];
for (const root of roots) {
  for (const path of walkFiles(join(REPO, root))) {
    const { blocks, needsEmoji } = hoist(path, apply);
    if (blocks) console.log(`${blocks}\t${needsEmoji.length ? "NEEDS-EMOJI " : ""}${relative(REPO, path)}`);
    total += blocks;
    pending.push(...needsEmoji);
  }
}
for (const line of pending) console.log(`needs-emoji ${line}`);
console.log(`${apply ? "applied" : "dry run"}: ${total} blocks, ${pending.length} new docstrings need an emoji`);
