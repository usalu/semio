import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import { readFileSync } from "node:fs";
const [oldFile, newFile] = process.argv.slice(2);
const oldLines = readFileSync(oldFile, "utf8").split("\n");
const start = oldLines.findIndex((l) => l.startsWith("/** All built-in ands = {"));
const end = oldLines.findIndex((l, i) => i > start && l.startsWith("// #endregion 🧬️CodegenGraphql"));
const oldText = ["export type Maybe<T> = T | null;", "export type InputMaybe<T> = Maybe<T>;", "export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };", "export type Scalars = {", ...oldLines.slice(start + 1, end)].join("\n");
const parse = (name, text) => ts.createSourceFile(name, text, ts.ScriptTarget.Latest, true);
const members = (sf, node) => {
  const lit = (t) => { let c = t; if (ts.isIntersectionTypeNode(c)) return { ifaces: c.types.filter((x) => !ts.isTypeLiteralNode(x)).map((x) => x.getText(sf)).sort(), lit: c.types.find(ts.isTypeLiteralNode) }; return { ifaces: [], lit: ts.isTypeLiteralNode(c) ? c : undefined }; };
  const { ifaces, lit: l } = lit(node.type);
  const m = new Map();
  for (const p of l?.members ?? []) if (ts.isPropertySignature(p)) m.set(p.name.getText(sf), (p.questionToken ? "?" : "") + ":" + p.type.getText(sf).replace(/\s+/g, " "));
  return { ifaces, m };
};
const index = (sf) => {
  const out = new Map();
  for (const st of sf.statements) {
    if (ts.isEnumDeclaration(st)) out.set(st.name.text, { kind: "enum", values: st.members.map((x) => x.initializer.text).sort().join(",") });
    else if (ts.isTypeAliasDeclaration(st)) { const t = st.type; if (ts.isTypeLiteralNode(t) || ts.isIntersectionTypeNode(t)) out.set(st.name.text, { kind: "object", ...members(sf, st) }); else out.set(st.name.text, { kind: "alias", text: t.getText(sf).replace(/\s+/g, " ") }); }
    else if (ts.isVariableStatement(st)) { /* documents */ }
  }
  return out;
};
const A = index(parse("old.ts", oldText)), B = index(parse("new.ts", readFileSync(newFile, "utf8")));
const isOp = (n) => /(Query|Mutation)(Variables)?$/u.test(n);
const rows = { identical: [], onlyOld: [], onlyNew: [], differing: [] };
for (const [name, a] of A) { if (!B.has(name)) { rows.onlyOld.push(name); continue; } const b = B.get(name); let same = a.kind === b.kind; const diffs = []; if (same) { if (a.kind === "enum") same = a.values === b.values; else if (a.kind === "alias") same = a.text === b.text; else { const ifc = JSON.stringify(a.ifaces) === JSON.stringify(b.ifaces); if (!ifc) diffs.push(`implements ${JSON.stringify(a.ifaces)} -> ${JSON.stringify(b.ifaces)}`); for (const [k, v] of a.m) { if (!b.m.has(k)) diffs.push(`- ${k}${v}`); else if (b.m.get(k) !== v) diffs.push(`~ ${k}: ${v} -> ${b.m.get(k)}`); } for (const [k, v] of b.m) if (!a.m.has(k)) diffs.push(`+ ${k}${v}`); same = diffs.length === 0; } } if (same) rows.identical.push(name); else rows.differing.push([name, diffs]); }
for (const name of B.keys()) if (!A.has(name)) rows.onlyNew.push(name);
console.log(`old declarations ${A.size}, new declarations ${B.size}`);
console.log(`identical in shape: ${rows.identical.length}`);
console.log(`only in the hand-kept copy (${rows.onlyOld.length}): ${rows.onlyOld.sort().join(", ")}`);
console.log(`only in the generated module (${rows.onlyNew.length}): ${rows.onlyNew.sort().join(", ")}`);
console.log(`differing (${rows.differing.length}):`);
for (const [name, diffs] of rows.differing.sort((a, b) => a[0].localeCompare(b[0]))) console.log(`  ${name}\n    ${diffs.slice(0, 40).join("\n    ")}`);
console.log("IDENTICAL: " + rows.identical.sort().join(", "));
// verify every difference against the served SDL with graphql-js
const { buildSchema, isNonNullType } = await import("/Users/ueli/Documents/semio/node_modules/graphql/index.js");
const sdl = readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/🧪️tests/🧫️fixtures/📜️served-schema/🔗️.graphql", "utf8");
const schema = buildSchema(sdl);
let explained = 0, unexplained = [];
const argsOwner = (name) => { const m = name.match(/^(\w+?)([A-Z]\w*)Args$/u); if (!m) return null; for (let i = 1; i < name.length - 4; i++) { const typeName = name.slice(0, i), rest = name.slice(i, -4); const type = schema.getType(typeName); if (type && type.getFields) { const f = Object.keys(type.getFields()).find((k) => k[0].toUpperCase() + k.slice(1) === rest); if (f) return { type, field: type.getFields()[f] }; } } return null; };
const helper = (name) => /^(Make(Optional|Maybe|Empty)|Incremental)$/u.test(name);
for (const name of rows.onlyOld) { if (isOp(name)) { explained++; continue; } if (!schema.getType(name)) explained++; else unexplained.push("onlyOld still in SDL: " + name); }
for (const name of rows.onlyNew) { if (isOp(name) || helper(name)) { explained++; continue; } if (/Args$/u.test(name)) { const o = argsOwner(name); if (o && o.field.args.length) { explained++; continue; } } if (schema.getType(name)) explained++; else unexplained.push("onlyNew not in SDL: " + name); }
for (const [name, diffs] of rows.differing) {
  if (isOp(name)) { explained++; continue; }
  if (/Args$/u.test(name)) { const o = argsOwner(name); if (o) { const keys = o.field.args.map((a) => a.name); const bad = diffs.filter((d) => { const m = d.match(/^([-+~]) (\w+)/u); return m && ((m[1] === "-" && keys.includes(m[2])) || (m[1] !== "-" && !keys.includes(m[2]))); }); if (!bad.length) { explained++; continue; } } }
  if (name === "Node" && schema.getType("Node").astNode.kind === "UnionTypeDefinition") { explained++; continue; }
  const type = schema.getType(name); const fields = type && type.getFields ? type.getFields() : null;
  let ok = !!fields;
  for (const d of diffs) { const m = d.match(/^([-+~]) (\w+)/u); if (!m) continue; const [, sign, field] = m; if (!fields) { ok = false; break; } if (sign === "-" && fields[field]) ok = false; if (sign === "+" && !fields[field]) ok = false; if (sign === "~" && !fields[field]) ok = false; }
  if (ok) explained++; else unexplained.push("differing not explained by SDL: " + name);
}
console.log(`EXPLAINED BY THE SERVED SDL: ${explained} of ${rows.onlyOld.length + rows.onlyNew.length + rows.differing.length}; unexplained: ${unexplained.length}`);
for (const u of unexplained) console.log("  UNEXPLAINED " + u);
