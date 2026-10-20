// usage: node blocksplit.cjs <spec.json> [APPLY]
// Class-method if-block extractor: every `if (cond) { ... return ...; }` of one method (depth 1, no else, at least minLines lines) becomes a private method placed after it; leading local consts listed in `hoist` become module-level consts before the class.
// spec: { file, method, minLines, hoist?: [name], suffix? }
const fs = require("fs");
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const spec = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
const APPLY = process.argv[3] === "APPLY";
const file = spec.file;
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let method = null, cls = null;
const visit = (n) => { if (ts.isClassDeclaration(n)) cls = cls || n; if (ts.isMethodDeclaration(n) && n.name.getText(sf) === spec.method && !method) method = n; ts.forEachChild(n, visit); };
visit(sf);
if (!method) throw new Error("method not found");
const classNode = method.parent;
const params = method.parameters.map((p) => ({ name: p.name.getText(sf), type: p.type.getText(sf) }));
const retText = method.type.getText(sf);
const unwrap = (t) => { const m = t.match(/^Promise<([\s\S]*)>$/); return m ? m[1] : t; };
const usedEmoji = new Set([...text.matchAll(/^\s*(?:\/\*\*|\/\/)\s*(\S+)/gm)].map((m) => m[1].replace(/️/g, "")));
const POOL = Array.from("🧱🪵🔩⚙🧲🪛🔧🔨🪚🧰🧪🧫🧬🔬🔭📡🛰🧭🗺🗝🔑🧩🎯🎲🧮📐📏📎📌📍🔖🏷🗂🗃🗄📦📥📤📬🔔🔒🔓🛡⚖⛓🪝🪢🧵🧶🪡🎛🎚🎬🎞🎥📷📸🔍🔎💡🔦🕯⏱⏲⏳⌛🔋🔌🧯🚦🚧🏗🏭🏛🏟🏰🌱🌿🍃🌳🌲🪨🌊🔥💧❄🌀🌈☀🌙⭐✨🪐🚀🛸🛠⛏🪓🧹🧺🧴🪣🧼🛁🪒🧷🧾📜📃📄📑📊📈📉📋📒📓📔📕📖📗📘📙📚🔗🧿🎁🎀🎈🎉🎊🏁🚩🏆🥇🎖🏅🔰♻🔱⚜🔶🔷🔸🔹🔺🔻💠🔘🔲🔳🟥🟧🟨🟩🟦🟪🟫");
const pickEmoji = () => { const e = POOL.find((c) => !usedEmoji.has(c)); usedEmoji.add(e); return e + "️"; };
const camel = (s) => s.replace(/[^A-Za-z0-9]+(\w)/g, (_, c) => c.toUpperCase()).replace(/^\w/, (c) => c.toLowerCase());
const dedent = (s, n) => s.split("\n").map((l) => l.slice(Math.min(n, l.match(/^ */)[0].length))).join("\n");
const hoist = new Set(spec.hoist || []);
const edits = [], helpers = [], hoisted = [];
const body = method.body.statements;
const usesOf = (code) => params.filter((p) => new RegExp(`\\b${p.name}\\b`).test(code));
for (const st of body) {
  if (ts.isVariableStatement(st) && st.declarationList.declarations.length === 1 && hoist.has(st.declarationList.declarations[0].name.getText(sf))) {
    const code = dedent(st.getText(sf), 0);
    const lines = text.slice(st.getStart(sf), st.getEnd()).split("\n");
    const ind = st.getStart(sf) - text.lastIndexOf("\n", st.getStart(sf)) - 1;
    hoisted.push(dedent(text.slice(st.getStart(sf), st.getEnd()), 0).split("\n").map((l, i) => (i === 0 ? l : l.slice(Math.min(ind, l.match(/^ */)[0].length)))).join("\n"));
    edits.push([text.lastIndexOf("\n", st.getFullStart() + 1) < 0 ? st.getFullStart() : st.getFullStart(), st.getEnd(), ""]);
    continue;
  }
  if (!ts.isIfStatement(st) || st.elseStatement || !ts.isBlock(st.thenStatement)) continue;
  const size = L(st.getEnd()) - L(st.getStart(sf)) + 1;
  if (size < spec.minLines) continue;
  const stmts = st.thenStatement.statements;
  const last = stmts[stmts.length - 1];
  if (!ts.isReturnStatement(last) && !ts.isThrowStatement(last)) continue;
  const cond = st.expression.getText(sf);
  const lit = (cond.match(/"([^"]+)"/) || [])[1];
  const name = (lit ? camel(lit) : "branch" + L(st.getStart(sf))) + (spec.suffix || "Diff");
  const inner = text.slice(stmts[0].getFullStart(), last.getEnd());
  const ind = Math.min(...inner.split("\n").filter((l, i) => i > 0 && l.trim()).map((l) => l.match(/^ */)[0].length));
  const innerText = inner.split("\n").map((l, i) => (i === 0 ? l : l.slice(Math.min(ind - 4, l.match(/^ */)[0].length)))).join("\n").replace(/^\n+/, "");
  const used = usesOf(innerText);
  const isAsync = /\bawait\b/.test(innerText);
  const sig = used.map((p) => `${p.name}: ${p.type}`).join(", ");
  helpers.push(`  /** ${pickEmoji()} The \`${lit || cond}\` command of \`${spec.method}\`. */\n  private ${isAsync ? "async " : ""}${name}(${sig}): ${isAsync ? retText : unwrap(retText)} {\n    ${innerText.trimStart()}\n  }`);
  const callArgs = used.map((p) => p.name).join(", ");
  edits.push([stmts[0].getFullStart(), last.getEnd(), `\n      return this.${name}(${callArgs});`]);
}
console.log("blocks:", helpers.length, "hoisted:", hoisted.length);
if (APPLY) {
  let out = text;
  for (const [s, e, rep] of edits.sort((a, b) => b[0] - a[0])) out = out.slice(0, s) + rep + out.slice(e);
  const shift = edits.reduce((n, [s, e, rep]) => n + rep.length - (e - s), 0);
  const end = method.getEnd() + shift;
  out = out.slice(0, end) + "\n\n" + helpers.join("\n\n") + out.slice(end);
  const lead = ts.getLeadingCommentRanges(text, classNode.getFullStart()); const clsStart = lead && lead.length ? lead[0].pos : classNode.getStart(sf);
  const pre = hoisted.map((h) => h.replace(/^const /, "const ")).join("\n");
  out = out.slice(0, clsStart) + pre + (pre ? "\n\n" : "") + out.slice(clsStart);
  fs.writeFileSync(file, out);
  console.log("APPLIED");
}
