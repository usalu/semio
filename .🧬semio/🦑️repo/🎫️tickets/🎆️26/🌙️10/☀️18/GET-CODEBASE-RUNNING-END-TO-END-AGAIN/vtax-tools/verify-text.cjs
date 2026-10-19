// lossless check: the multiset of depth-1 statements of the pre-split validateTaxonomy and validateSchemaScopeVocabulary equals the union of the statements of the new factory, group and scope functions (modulo the destructuring header, the dropped region markers and the one call-site argument)
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const DISCOVERY = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/";
const parse = (file, text) => ts.createSourceFile(file, text, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
const norm = (s) => s.replace(/\s+/g, " ").trim();
const fnOf = (sf, name) => sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === name);
const orig = parse("orig.ts", fs.readFileSync(path.join(__dirname, "orig/taxonomy-authority.ts"), "utf8"));
const origStatements = [...fnOf(orig, "validateTaxonomy").body.statements.slice(0, -1).map((s) => norm(s.getText(orig))), ...fnOf(orig, "validateSchemaScopeVocabulary").body.statements.map((s) => norm(s.getText(orig)))];
const newStatements = [];
for (const dir of fs.readdirSync(DISCOVERY).filter((n) => /audit/.test(n))) {
  const file = DISCOVERY + dir + "/🟦️.ts", sf = parse(file, fs.readFileSync(file, "utf8"));
  for (const s of sf.statements) {
    if (!ts.isFunctionDeclaration(s) || !s.body) continue;
    const body = s.body.statements.slice();
    if (s.name.text === "createTaxonomyAudit") body.pop();
    if (s.name.text !== "createTaxonomyAudit" || true) { const first = body[0]; if (first && ts.isVariableStatement(first) && /= audit;$/.test(first.getText(sf))) body.shift(); }
    for (const b of body) newStatements.push(norm(b.getText(sf)).replace("validateSchemaScopeVocabulary(audit)", "validateSchemaScopeVocabulary(taxonomy)"));
  }
}
const count = (arr) => arr.reduce((m, s) => m.set(s, (m.get(s) || 0) + 1), new Map());
const a = count(origStatements), b = count(newStatements);
let missing = 0, extra = 0;
for (const [k, v] of a) if ((b.get(k) || 0) !== v) { missing++; console.log("MISSING/DIFF", v, b.get(k) || 0, k.slice(0, 140)); }
for (const [k, v] of b) if (!a.has(k)) { extra++; console.log("EXTRA", v, k.slice(0, 140)); }
console.log(`original statements ${origStatements.length} (distinct ${a.size}); new statements ${newStatements.length} (distinct ${b.size}); missing/different ${missing}; extra ${extra}; ${missing === 0 && extra === 0 && origStatements.length === newStatements.length ? "LOSSLESS OK" : "MISMATCH"}`);
