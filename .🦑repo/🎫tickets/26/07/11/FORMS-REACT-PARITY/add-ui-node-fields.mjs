#!/usr/bin/env node
// 🎫formsreactparity — one-shot codemod: append new Option fields as `None` to existing UiNode struct literals.
import { readFileSync, writeFileSync } from "node:fs";
import { execSync } from "node:child_process";

const REPO = "/Users/ueli/Documents/semio";
const STRUCT_FIELDS = {
  UiFieldNode: ["description", "required", "error"],
  UiInputNode: ["min", "max", "step", "accept"],
  UiSliderNode: ["unit"],
  UiButtonNode: ["disabled"],
  UiTreeNode: ["drop_command"],
  UiStackNode: ["id", "selected", "activate", "drop_command"],
};

const files = execSync(`grep -rl --include='*.rs' -E 'Ui(Field|Input|Slider|Button|Tree|Stack)Node \\{' '${REPO}' --exclude-dir=node_modules --exclude-dir=target --exclude-dir=.git`, { encoding: "utf8" })
  .trim()
  .split("\n")
  .filter((f) => f && !f.includes("/.repo/"));

function findLiteralClose(text, openBraceIndex) {
  let depth = 0;
  let inString = false;
  let inLineComment = false;
  for (let i = openBraceIndex; i < text.length; i += 1) {
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
      if (text[i + 1] === "\\" && text[i + 3] === "'") {
        i += 3;
        continue;
      }
      if (text[i + 2] === "'") {
        i += 2;
        continue;
      }
      continue;
    }
    if (c === "{") depth += 1;
    else if (c === "}") {
      depth -= 1;
      if (depth === 0) return i;
    }
  }
  return -1;
}

let totalEdits = 0;
for (const file of files) {
  let text = readFileSync(file, "utf8");
  let changed = false;
  for (const [structName, fields] of Object.entries(STRUCT_FIELDS)) {
    const pattern = new RegExp(`${structName} \\{`, "g");
    const edits = [];
    let match;
    while ((match = pattern.exec(text)) !== null) {
      const lineStart = text.lastIndexOf("\n", match.index) + 1;
      const line = text.slice(lineStart, text.indexOf("\n", match.index));
      if (/\bstruct\b/.test(line)) continue;
      const openIndex = match.index + match[0].length - 1;
      const closeIndex = findLiteralClose(text, openIndex);
      if (closeIndex === -1) continue;
      const body = text.slice(openIndex + 1, closeIndex);
      const missing = fields.filter((f) => !new RegExp(`(^|[\\s{,(])${f}\\s*:`).test(body));
      if (missing.length === 0) continue;
      edits.push({ openIndex, closeIndex, missing });
    }
    for (const edit of edits.reverse()) {
      const { openIndex, closeIndex, missing } = edit;
      const closeLineStart = text.lastIndexOf("\n", closeIndex) + 1;
      const singleLine = closeLineStart - 1 < openIndex;
      if (singleLine) {
        const head = text.slice(0, closeIndex).replace(/\s*$/, "");
        const needsComma = !/[{,]$/.test(head);
        text = `${head}${needsComma ? "," : ""} ${missing.map((f) => `${f}: None`).join(", ")} ${text.slice(closeIndex)}`;
      } else {
        const closeIndent = text.slice(closeLineStart, closeIndex).match(/^\s*/)[0];
        const insertion = missing.map((f) => `${closeIndent}    ${f}: None,\n`).join("");
        text = text.slice(0, closeLineStart) + insertion + text.slice(closeLineStart);
      }
      totalEdits += 1;
      changed = true;
    }
  }
  if (changed) {
    writeFileSync(file, text);
    console.log(`[DEBUG] patched ${file}`);
  }
}
console.log(`[DEBUG] total literals patched: ${totalEdits}`);
