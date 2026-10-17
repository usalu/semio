// usage: node exportsmap.cjs <snapshot.ts> <root.ts> [kids to skip...]  -> prints, per owner file, the names the old root exported (value / type)
const fs = require("fs"); const path = require("path");
const ts = require("/Users/ueli/Documents/semio/node_modules/typescript");
const [snap, root] = process.argv.slice(2, 4);
const kinds = (f) => { const sf = ts.createSourceFile(f, fs.readFileSync(f, "utf8"), ts.ScriptTarget.Latest, true); const out = new Map(); for (const s of sf.statements) { if (!(ts.getCombinedModifierFlags(s) & ts.ModifierFlags.Export)) continue; const isType = ts.isInterfaceDeclaration(s) || ts.isTypeAliasDeclaration(s); if (ts.isVariableStatement(s)) for (const d of s.declarationList.declarations) out.set(d.name.getText(sf), (out.get(d.name.getText(sf)) || []).concat(["value"])); else if (s.name) out.set(s.name.text, (out.get(s.name.text) || []).concat([isType ? "type" : "value"])); } return out; };
const old = kinds(snap);
const files = [root]; for (const e of fs.readdirSync(path.dirname(root), { withFileTypes: true })) if (e.isDirectory()) { const f = path.join(path.dirname(root), e.name, "🟦️.ts"); if (fs.existsSync(f)) files.push(f); }
const ownerOf = new Map(); for (const f of files) for (const [n, k] of kinds(f)) ownerOf.set(n, { f, k });
const by = new Map();
for (const [n, k] of old) { const o = ownerOf.get(n); if (!o) { console.error("MISSING", n); continue; } if (!by.has(o.f)) by.set(o.f, []); by.get(o.f).push([n, o.k.every((x) => x === "type") ? "type" : "value"]); }
console.log(JSON.stringify([...by.entries()].map(([f, l]) => ({ file: f, values: l.filter((x) => x[1] === "value").map((x) => x[0]).sort(), types: l.filter((x) => x[1] === "type").map((x) => x[0]).sort() })), null, 1));
