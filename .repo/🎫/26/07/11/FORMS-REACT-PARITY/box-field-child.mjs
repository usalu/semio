#!/usr/bin/env node
// 🎫formsreactparity — one-shot codemod: rewrite `child: UiControlNode::X(…)` to `child: Box::new(UiNode::X(…))`.
import { readFileSync, writeFileSync } from "node:fs";
import { execSync } from "node:child_process";

const REPO = "/Users/ueli/Documents/semio";
const files = execSync(`grep -rl --include='*.rs' -F 'child: UiControlNode::' '${REPO}' --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.git`, { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter((f) => f && !f.includes("/.repo/"));

function findCallClose(text, openParenIndex) {
  let depth = 0;
  let inString = false;
  let inLineComment = false;
  for (let i = openParenIndex; i < text.length; i += 1) {
    const c = text[i];
    if (inLineComment) {
      if (c === "\n") inLineComment = false;
      continue;
    }
    if (inString) {
      if (c === "\\") i += 1;
      else if (c === '"') inString = false;
      continue;
    }
    if (c === "/" && text[i + 1] === "/") {
      inLineComment = true;
      continue;
    }
    if (c === '"') {
      inString = true;
      continue;
    }
    if (c === "'") {
      if (text[i + 1] === "\\" && text[i + 3] === "'") i += 3;
      else if (text[i + 2] === "'") i += 2;
      continue;
    }
    if (c === "(" || c === "{" || c === "[") depth += 1;
    else if (c === ")" || c === "}" || c === "]") {
      depth -= 1;
      if (depth === 0) return i;
    }
  }
  return -1;
}

let total = 0;
for (const file of files) {
  let text = readFileSync(file, "utf8");
  const marker = "child: UiControlNode::";
  let index = text.indexOf(marker);
  while (index !== -1) {
    const openParen = text.indexOf("(", index + marker.length);
    const close = findCallClose(text, openParen);
    if (close === -1) break;
    text = `${text.slice(0, index)}child: Box::new(UiNode::${text.slice(index + marker.length, close + 1)})${text.slice(close + 1)}`;
    total += 1;
    index = text.indexOf(marker, index + 1);
  }
  writeFileSync(file, text);
  console.log(`[DEBUG] boxed field children in ${file}`);
}
console.log(`[DEBUG] total rewrites: ${total}`);
