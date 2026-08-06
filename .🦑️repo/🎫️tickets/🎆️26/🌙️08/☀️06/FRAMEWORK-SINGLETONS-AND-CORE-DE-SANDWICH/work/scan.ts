#!/usr/bin/env bun
/** 🔍️ Rust-aware brace scanner: prints the byte/line span of every `pub mod X {` block (with nesting depth). */
import { readFileSync } from "node:fs";

const src = readFileSync(process.argv[2], "utf8");

type Ev = { name: string; depth: number; open: number; close: number };
const events: Ev[] = [];
const stack: { name: string; open: number; braceDepth: number }[] = [];

let i = 0;
let braceDepth = 0;
const n = src.length;
const modRe = /(?:^|\n)[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\{/;

function lineOf(idx: number): number {
  let l = 1;
  for (let k = 0; k < idx; k++) if (src[k] === "\n") l++;
  return l;
}

while (i < n) {
  const c = src[i];
  // line comment
  if (c === "/" && src[i + 1] === "/") {
    while (i < n && src[i] !== "\n") i++;
    continue;
  }
  // block comment (nesting)
  if (c === "/" && src[i + 1] === "*") {
    let d = 0;
    while (i < n) {
      if (src[i] === "/" && src[i + 1] === "*") { d++; i += 2; continue; }
      if (src[i] === "*" && src[i + 1] === "/") { d--; i += 2; if (d === 0) break; continue; }
      i++;
    }
    continue;
  }
  // raw string
  if (c === "r" && (src[i + 1] === '"' || src[i + 1] === "#")) {
    let j = i + 1;
    let hashes = 0;
    while (src[j] === "#") { hashes++; j++; }
    if (src[j] === '"') {
      j++;
      const term = '"' + "#".repeat(hashes);
      const end = src.indexOf(term, j);
      i = end === -1 ? n : end + term.length;
      continue;
    }
  }
  // byte string b"..."
  if (c === "b" && src[i + 1] === '"') { i++; continue; }
  // normal string
  if (c === '"') {
    i++;
    while (i < n) {
      if (src[i] === "\\") { i += 2; continue; }
      if (src[i] === '"') { i++; break; }
      i++;
    }
    continue;
  }
  // char literal / lifetime
  if (c === "'") {
    if (src[i + 1] === "\\") {
      let j = i + 2;
      while (j < n && src[j] !== "'") j++;
      i = j + 1;
      continue;
    }
    if (src[i + 2] === "'") { i += 3; continue; }
    i++; // lifetime
    continue;
  }
  if (c === "{") {
    // look back for a `mod NAME {` header ending here
    const start = Math.max(0, i - 200);
    const chunk = src.slice(start, i + 1);
    const m = chunk.match(new RegExp(modRe.source + "$"));
    if (m) {
      stack.push({ name: m[1], open: i, braceDepth });
    }
    braceDepth++;
    i++;
    continue;
  }
  if (c === "}") {
    braceDepth--;
    if (stack.length && stack[stack.length - 1].braceDepth === braceDepth) {
      const top = stack.pop()!;
      events.push({ name: top.name, depth: stack.length, open: top.open, close: i });
    }
    i++;
    continue;
  }
  i++;
}

events.sort((a, b) => a.open - b.open);
for (const e of events) {
  console.log(`${"  ".repeat(e.depth)}${e.name}  openLine=${lineOf(e.open)} closeLine=${lineOf(e.close)} depth=${e.depth}`);
}
console.log(`finalBraceDepth=${braceDepth} unclosed=${stack.length}`);
