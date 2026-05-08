import { readFileSync } from "node:fs";
const s = readFileSync(process.argv[2], "utf8");
let line = 1, col = 0, q = null;
const stack = [];
for (let i = 0; i < s.length; i++) {
  const c = s[i];
  if (c === "\n") { line++; col = 0; continue; }
  col++;
  if (q) {
    if (q === "`" && c === "$" && s[i+1] === "{") { stack.push(["${", line, col]); q = null; i++; col++; continue; }
    if (c === "\\") { i++; col++; continue; }
    if (c === q) { q = null; }
    continue;
  }
  if (c === "/" && s[i+1] === "/") { while (i < s.length && s[i] !== "\n") i++; line++; col = 0; continue; }
  if (c === "/" && s[i+1] === "*") { i += 2; while (i < s.length - 1 && !(s[i] === "*" && s[i+1] === "/")) { if (s[i] === "\n") { line++; col = 0; } i++; } i++; continue; }
  if (c === '"' || c === "'" || c === "`") { q = c; continue; }
  if (c === "(" || c === "[" || c === "{") { stack.push([c, line, col]); }
  else if (c === ")" || c === "]" || c === "}") {
    const map = { ")": "(", "]": "[", "}": "{" };
    const top = stack[stack.length - 1];
    if (top && top[0] === map[c]) { stack.pop(); if (top[0] === "${" && c === "}") { q = "`"; } }
    else { console.log("MISMATCH at", line, col, "char", c, "top", top); break; }
  }
}
console.log("remaining stack tail:", stack.slice(-10));
