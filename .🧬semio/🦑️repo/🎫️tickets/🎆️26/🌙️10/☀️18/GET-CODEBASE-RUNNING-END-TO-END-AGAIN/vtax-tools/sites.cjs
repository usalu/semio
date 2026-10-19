// push-site coverage: every `problems.push(<message>)` site of the pre-split validator becomes a pattern; reports how many sites were triggered by at least one differential variant. usage: node sites.cjs <shard.json>...
const fs = require("fs"), path = require("path");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const text = fs.readFileSync(path.join(__dirname, "orig/taxonomy-authority.ts"), "utf8");
const sf = ts.createSourceFile("orig.ts", text, ts.ScriptTarget.ESNext, true, ts.ScriptKind.TS);
const messages = new Set();
for (const f of process.argv.slice(2)) for (const m of JSON.parse(fs.readFileSync(f, "utf8")).fullMessages || []) messages.add(m);
const esc = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const sites = [];
const inScope = (n) => { for (let p = n; p; p = p.parent) if (ts.isFunctionDeclaration(p) && p.name && ["validateTaxonomy", "validateSchemaScopeVocabulary"].includes(p.name.text)) return true; return false; };
const visit = (n) => {
  if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression) && n.expression.expression.getText(sf) === "problems" && n.expression.name.text === "push" && inScope(n)) {
    for (const arg of n.arguments) {
      let re = null;
      if (ts.isStringLiteral(arg) || ts.isNoSubstitutionTemplateLiteral(arg)) re = new RegExp("^" + esc(arg.text) + "$", "u");
      else if (ts.isTemplateExpression(arg)) re = new RegExp("^" + esc(arg.head.text) + arg.templateSpans.map((sp) => "[\\s\\S]*?" + esc(sp.literal.text)).join("") + "$", "u");
      sites.push({ line: sf.getLineAndCharacterOfPosition(n.getStart(sf)).line + 1, re, dynamic: !re, src: arg.getText(sf).slice(0, 90) });
    }
  }
  ts.forEachChild(n, visit);
};
visit(sf);
let hit = 0, dyn = 0; const missed = [];
for (const s of sites) { if (s.dynamic) { dyn++; continue; } if ([...messages].some((m) => s.re.test(m))) hit++; else missed.push(s); }
console.log(`push sites ${sites.length}: literal ${sites.length - dyn} (triggered ${hit}, not triggered ${missed.length}), non-literal ${dyn}; distinct full messages seen ${messages.size}`);
for (const s of missed) console.log("  not triggered, orig line", s.line, s.src);
