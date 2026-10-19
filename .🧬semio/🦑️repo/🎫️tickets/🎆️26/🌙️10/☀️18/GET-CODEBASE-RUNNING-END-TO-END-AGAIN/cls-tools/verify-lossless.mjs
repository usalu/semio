// usage: node verify-lossless.mjs <orig.ts> <class-name> <dir-or-file>...
// Every member of the original class must reappear, textually equal modulo the state/hub/shell indirection, in exactly one class of the new files.
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/node_modules/");
const ts = require("typescript");

const [origPath, className, ...roots] = process.argv.slice(2);
const files = [];
const walk = p => {
  const stat = fs.statSync(p);
  if (stat.isDirectory()) { for (const entry of fs.readdirSync(p)) if (!["🧪️tests", "📤️segmented-download", "🔐️activation", "💓️heartbeat", "📜️wire", "📨️messages", "🚨️quota", "🩹️ui-patch", "⚙️options", "🫀️liveness"].includes(entry)) walk(path.join(p, entry)); }
  else if (p.endsWith(".ts")) files.push(p);
};
for (const root of roots) walk(root);

const normalize = text => text
  .replace(/\/\*\*[\s\S]*?\*\//gu, "")
  .replace(/\/\/[^\n]*/gu, "")
  .replace(/\bthis\.hub\.shell\b/gu, "this")
  .replace(/\bthis\.(?:hub\.)?session\.artifact\b/gu, "this.state")
  .replace(/\bthis\.hub\.(?:state|session)\./gu, "this.")
  .replace(/\bthis\.(?:state|session|resident)\./gu, "this.")
  .replace(/\bthis\.hub\.\w+\./gu, "this.")
  .replace(/\bprivate\s+/gu, "")
  .replace(/#/gu, "")
  .replace(/\s+/gu, "");

const members = (file, wanted) => {
  const text = fs.readFileSync(file, "utf8");
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const out = [];
  for (const statement of sf.statements) {
    if (!ts.isClassDeclaration(statement) || !statement.name) continue;
    if (wanted && statement.name.text !== wanted) continue;
    for (const member of statement.members) if ((ts.isMethodDeclaration(member) || ts.isGetAccessorDeclaration(member)) && member.name) out.push({ owner: statement.name.text, name: member.name.getText(sf).replace(/^#/u, ""), text: normalize(member.getText(sf)), lines: member.getText(sf).split("\n").length });
  }
  return out;
};

const original = members(origPath, className);
const found = new Map();
for (const file of files) for (const m of members(file, null)) { if (m.owner === className && file === roots[0]) {} const key = m.name; if (!found.has(key)) found.set(key, []); found.get(key).push({ ...m, file }); }
let bad = 0, same = 0, delegated = 0;
const originalNames = new Set(original.map(m => m.name));
for (const m of original) {
  const candidates = (found.get(m.name) ?? []).filter(c => c.text === m.text);
  if (candidates.length === 1) same += 1;
  else if (candidates.length === 0) {
    const impl = (found.get(m.name) ?? []).filter(c => c.text.length > 0);
    const delegate = impl.some(c => c.lines <= 2);
    if (delegate && impl.some(c => c.lines > 2 || impl.length > 1)) delegated += 1;
    else { bad += 1; console.log("MISMATCH", m.name, "candidates:", impl.map(c => c.owner + ":" + c.lines).join(",")); }
  } else console.log("DUPLICATE", m.name, candidates.map(c => c.owner).join(","));
}
const extra = [...found.keys()].filter(name => !originalNames.has(name));
console.log(JSON.stringify({ originalMembers: original.length, identical: same, onlyDelegateDiffers: delegated, mismatches: bad, newMemberNames: extra }, null, 1));
process.exit(bad === 0 ? 0 : 1);
