// usage: node rmblock.mjs <file> <regex-of-registrar-calls> [APPLY]
// Removes `if (import.meta.vitest) { ... registerX(...) ... }` statements whose text matches the regex, drops region pairs left empty, and prunes imports no longer referenced.
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import fs from "node:fs";
import crypto from "node:crypto";
const [file, reSrc, mode] = process.argv.slice(2);
const APPLY = mode === "APPLY";
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const re = new RegExp(reSrc);
const regionRe = /^[ \t]*\/\/\s*#(end)?region\s+(.*?)\s*$/;
const removed = [];
for (const s of sf.statements) if (ts.isIfStatement(s) && s.expression.getText(sf) === "import.meta.vitest" && re.test(s.thenStatement.getText(sf))) removed.push([s.getStart(sf), s.getEnd()]);
console.log("blocks matched:", removed.length);
let out = text;
for (const [a, b] of removed.slice().sort((x, y) => y[0] - x[0])) {
  let end = b; if (out[end] === "\n") end++;
  out = out.slice(0, a) + out.slice(end);
}
let lines = out.split("\n");
let changed = true;
const droppedRegions = [];
while (changed) {
  changed = false;
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(regionRe);
    if (!m || m[1]) continue;
    let j = i + 1;
    while (j < lines.length && lines[j].trim() === "") j++;
    const e = j < lines.length ? lines[j].match(regionRe) : null;
    if (e && e[1] && e[2] === m[2]) { droppedRegions.push(m[2]); lines.splice(i, j - i + 1); if (lines[i - 1]?.trim() === "" && lines[i]?.trim() === "") lines.splice(i, 1); changed = true; break; }
  }
}
out = lines.join("\n");
const sf2 = ts.createSourceFile(file, out, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const used = new Set();
const visit = (n) => { if (ts.isImportDeclaration(n)) return; if (ts.isIdentifier(n)) used.add(n.text); ts.forEachChild(n, visit); };
visit(sf2);
const edits = [];
const pruned = [];
for (const s of sf2.statements) {
  if (!ts.isImportDeclaration(s) || !s.importClause) continue;
  const c = s.importClause;
  if (c.name || (c.namedBindings && !ts.isNamedImports(c.namedBindings))) continue;
  if (!c.namedBindings) continue;
  const keep = c.namedBindings.elements.filter((el) => used.has(el.name.text));
  if (keep.length === c.namedBindings.elements.length) continue;
  for (const el of c.namedBindings.elements) if (!used.has(el.name.text)) pruned.push(el.name.text);
  if (!keep.length) edits.push({ start: s.getStart(sf2), end: s.getEnd() + (out[s.getEnd()] === "\n" ? 1 : 0), text: "" });
  else edits.push({ start: s.getStart(sf2), end: s.getEnd(), text: `import ${c.isTypeOnly ? "type " : ""}{ ${keep.map((e) => e.getText(sf2)).join(", ")} } from ${s.moduleSpecifier.getText(sf2)};` });
}
for (const e of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, e.start) + e.text + out.slice(e.end);
console.log(JSON.stringify({ regionsDropped: droppedRegions, importNamesPruned: pruned, linesBefore: text.split("\n").length - 1, linesAfter: out.split("\n").length - 1 }));
if (APPLY) {
  const live = fs.readFileSync(file, "utf8");
  if (crypto.createHash("sha1").update(live).digest("hex") !== crypto.createHash("sha1").update(text).digest("hex")) throw new Error("file changed while processing");
  fs.writeFileSync(file, out);
  console.log("APPLIED");
}
