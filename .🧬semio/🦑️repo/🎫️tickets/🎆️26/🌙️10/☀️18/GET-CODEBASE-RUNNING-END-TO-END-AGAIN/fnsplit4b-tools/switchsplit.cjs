// usage: node switchsplit.cjs <spec.json> [APPLY]
// Switch-case extractor: every `case "lit": { ... return ...; }` of one switch (the discriminant is `<param>.<prop>`) of a function becomes a helper function placed after it.
// spec: { file, function, minLines, noun?, suffix? }   helper(param: Extract<ParamType, { <prop>: "lit" }>, ...otherParams) with the function's own return type.
const fs = require("fs");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const fn = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === spec.function);
if (!fn) throw new Error("function not found");
const sw = fn.body.statements.find((s) => ts.isSwitchStatement(s));
if (!sw) throw new Error("no switch statement at depth 1");
const disc = sw.expression;
if (!ts.isPropertyAccessExpression(disc) || !ts.isIdentifier(disc.expression)) throw new Error("discriminant must be <param>.<prop>");
const paramName = disc.expression.text, prop = disc.name.text;
const params = fn.parameters.map((p) => ({ name: p.name.getText(sf), type: p.type.getText(sf) }));
const discParam = params.find((p) => p.name === paramName);
const retType = fn.type.getText(sf);
const usedEmoji = new Set([...text.matchAll(/^\s*(?:\/\*\*|\/\/)\s*(\S+)/gm)].map((m) => m[1].replace(/️/g, "")));
const POOL = Array.from("🧱🪵🔩⚙🧲🪛🔧🔨🪚🧰🧪🧫🧬🔬🔭📡🛰🧭🗺🗝🔑🧩🎯🎲🧮📐📏📎📌📍🔖🏷🗂🗃🗄📦📥📤📬🔔🔒🔓🛡⚖⛓🪝🪢🧵🧶🪡🎛🎚🎬🎞🎥📷📸🔍🔎💡🔦🕯⏱⏲⏳⌛🔋🔌🧯🚦🚧🏗🏭🏛🏟🏰🌱🌿🍃🌳🌲🪨🌊🔥💧❄🌀🌈☀🌙⭐✨🪐🚀🛸🛠⛏🪓🧹🧺🧴🪣🧼🛁🪒🧷🧾📜📃📄📑📊📈📉📋📒📓📔📕📖📗📘📙📚🔗🧿🎁🎀🎈🎉🎊🏁🚩🏆🥇🎖🏅🔰♻🔱⚜🔶🔷🔸🔹🔺🔻💠🔘🔲🔳🟥🟧🟨🟩🟦🟪🟫");
const pickEmoji = () => { const e = POOL.find((c) => !usedEmoji.has(c)); usedEmoji.add(e); return e + "️"; };
const camel = (s) => s.replace(/[-_ ]+(\w)/g, (_, c) => c.toUpperCase());
const edits = [], helpers = [];
let count = 0;
for (const clause of sw.caseBlock.clauses) {
  if (!ts.isCaseClause(clause) || !ts.isStringLiteral(clause.expression)) continue;
  const lit = clause.expression.text;
  const stmts = clause.statements;
  if (!stmts.length) continue;
  const startLine = L(clause.getStart(sf)), endLine = L(clause.getEnd());
  if (endLine - startLine + 1 < spec.minLines) continue;
  const body = stmts.length === 1 && ts.isBlock(stmts[0]) ? stmts[0].statements : stmts;
  const last = body[body.length - 1];
  if (!ts.isReturnStatement(last) && !ts.isThrowStatement(last)) continue;
  const first = body[0];
  const innerStart = first.getFullStart(), innerEnd = last.getEnd();
  let inner = text.slice(innerStart, innerEnd);
  const lines = inner.split("\n");
  const ind = Math.min(...lines.filter((l, i) => i > 0 && l.trim()).map((l) => l.match(/^ */)[0].length));
  inner = lines.map((l, i) => (i > 0 ? l.slice(Math.min(ind - 2, l.match(/^ */)[0].length)) : l)).join("\n").replace(/^\n+/, "");
  const name = camel(lit) + (spec.suffix || "Outcome");
  const pType = `Extract<${discParam.type}, { readonly ${prop}: ${JSON.stringify(lit)} }>`;
  const used = params.filter((p) => new RegExp(`\\b${p.name}\\b`).test(text.slice(innerStart, innerEnd)));
  const sig = used.map((p) => `${p.name}: ${p.name === paramName ? pType : p.type}`).join(", ");
  helpers.push(`/** ${pickEmoji()} ${spec.noun || "Outcome of the"} \`${lit}\` case of \`${spec.function}\`. */\nfunction ${name}(${sig}): ${retType} {\n  ${inner.trimStart()}\n}`);
  const callArgs = used.map((p) => p.name).join(", ");
  const clauseStart = stmts[0].getFullStart();
  edits.push([clauseStart, clause.getEnd(), ` return ${name}(${callArgs});`]);
  count++;
}
console.log("extracted cases:", count, "of", sw.caseBlock.clauses.length);
if (APPLY) {
  let out = text;
  for (const [s, e, rep] of edits.sort((a, b) => b[0] - a[0])) out = out.slice(0, s) + rep + out.slice(e);
  const shift = edits.reduce((n, [s, e, rep]) => n + rep.length - (e - s), 0);
  const end = fn.getEnd() + shift;
  out = out.slice(0, end) + "\n\n" + helpers.join("\n\n") + out.slice(end);
  fs.writeFileSync(file, out);
  console.log("APPLIED");
}
