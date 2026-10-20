// usage: node autop.cjs <file> <fnName|@line> <out.json> [max=150] : proposes a psplit2 spec (one folder per phase)
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [file, name, out, max = "150"] = process.argv.slice(2);
const V = "️";
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn = null, arrowVar = false, fname = name;
const find = (n) => {
  if (fn) return;
  if (name.startsWith("@")) { if (ts.isFunctionLike(n) && n.body && ts.isBlock(n.body) && L(n.getStart(sf)) === +name.slice(1) && n.parent && (ts.isSourceFile(n.parent) || ts.isVariableDeclaration(n.parent) || ts.isFunctionDeclaration(n))) { fn = n; return; } }
  else if (ts.isFunctionDeclaration(n) && n.name && n.name.text === name) { fn = n; return; }
  else if (ts.isVariableDeclaration(n) && ts.isIdentifier(n.name) && n.name.text === name && n.initializer && (ts.isArrowFunction(n.initializer) || ts.isFunctionExpression(n.initializer))) { fn = n.initializer; arrowVar = true; return; }
  ts.forEachChild(n, find);
};
find(sf);
if (!fn) throw new Error("function not found");
if (name.startsWith("@")) { if (ts.isFunctionDeclaration(fn)) fname = fn.name.text; else if (ts.isVariableDeclaration(fn.parent)) { fname = fn.parent.name.text; arrowVar = true; } else throw new Error("anonymous function"); }
if (arrowVar) { const vs = fn.parent.parent.parent; if (!ts.isVariableStatement(vs) || vs.declarationList.declarations.length !== 1) throw new Error("arrow variable must be a single-declaration statement"); }
const st = [...fn.body.statements].map((n) => ({ n, a: L(n.getStart(sf)), b: L(n.getEnd()) }));
const last = st[st.length - 1];
const hasReturn = last && ts.isReturnStatement(last.n);
const body = hasReturn ? st.slice(0, -1) : st;
const nested = (s) => { let r = false; const w = (n, d) => { if (ts.isReturnStatement(n) && d === 0) r = true; ts.forEachChild(n, (c) => w(c, d + (ts.isFunctionLike(c) ? 1 : 0))); }; w(s.n, 0); return r; };
body.forEach((s) => { s.keep = nested(s); });
const STOP = new Set("the a an of and to is are with for in on by from as at or its it that this be into than then when without every each".split(" "));
const words = (t) => t.replace(/([a-z0-9])([A-Z])/g, "$1 $2").toLowerCase().replace(/[^a-z0-9 ]+/g, " ").split(/\s+/).filter((w) => w && !STOP.has(w));
const pascal = (ws) => ws.map((w) => w[0].toUpperCase() + w.slice(1)).join("");
const titleOf = (s) => { let t = null; const w = (n) => { if (t) return; if (ts.isCallExpression(n)) { const ex = n.expression.getText(sf); if (/^(test|it|describe)(\.\w+)*$/.test(ex) && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) { t = n.arguments[0].text; return; } } ts.forEachChild(n, w); }; w(s.n); return t; };
const declNames = (s) => { const out = []; if (ts.isVariableStatement(s.n)) for (const d of s.n.declarationList.declarations) { const b = (x) => { if (ts.isIdentifier(x)) out.push(x.text); else x.elements && x.elements.forEach((e) => !ts.isOmittedExpression(e) && b(e.name)); }; b(d.name); } else if ((ts.isFunctionDeclaration(s.n) || ts.isClassDeclaration(s.n)) && s.n.name) out.push(s.n.name.text); return out; };
const usesHook = (s) => { let r = false; const w = (n) => { if (ts.isCallExpression(n) && ts.isIdentifier(n.expression) && /^use[A-Z]/.test(n.expression.text)) r = true; if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression) && /^use[A-Z]/.test(n.expression.name.text)) r = true; ts.forEachChild(n, w); }; w(s.n); return r; };
const runs = [];
let cur = [], size = 0;
const flush = () => { if (cur.length) runs.push(cur); cur = []; size = 0; };
for (const s of body) {
  if (s.keep) { flush(); runs.push([s]); continue; }
  const len = s.b - s.a + 1;
  if (cur.length && size + len > +max) flush();
  cur.push(s); size += len;
}
flush();
const POOL = "🔶 🔷 🔸 🔹 🟠 🟡 🟢 🔵 🟣 🟤 ⭐ 🌟 💠 🔰 🔱 🎯 🎲 🧩 🧱 🪵 🪨 🧲 🪝 🔩 ⚙ 🔧 🔨 🪛 🪚 🧰 🗜 🔗 ⛓ 📌 📍 📎 🖇 📏 📐 ✂ 🗂 🗃 🗄 📂 📁 📦 📮 🏷 🔖 🧾 📜 📃 📄 📑 🔍 🔎 🔬 🔭 📡 🛰 🧭 🗺 🧳 🎒 🪁 🎈 🎀 🎁 🎐 🪅 🧸 🎨 🖌 🖍 🪄 🔮 🧿 🪬 🧫 🧬 💎 🔔 🔕 📣 📢 🎵 🎶 🎼 🥁 🪘 🎷 🎺 🎸 🪕 🎻 🪗 🏆 🥇 🎖 🏅 🚩 🏁 🏴 🏳 🪧 🚦 🚧 ⚓ ⛵ 🛶 🚀 🛸 🚁 🛩 🚂 🚃 🚲 🛴 🛹 🛼 ⛽ 🛞 🪂 ⏱ ⏲ ⏰ 🕰 ⌛ ⏳ 🌡 ☀ 🌙 ⭕ 🌈 🔥 💧 🌊 ❄ ⚡ ☔ 🍀 🌱 🌿 🌳 🌲 🌴 🌵 🌾 🍁 🍂 🌸 🌼 🌻 🌺 🪻 🪷 🐝 🐞 🦋 🐌 🐜 🪲 🕷 🦂 🐢 🦎 🐙 🦀 🦞 🦐 🐠 🐬 🦈 🐳 🦭 🦦 🦫 🐿 🦔 🦇 🐦 🦉 🦅 🦆 🦢 🦜 🪶 🐓".split(" ");
const base = path.dirname(file);
const used = new Set();
for (const e of fs.existsSync(base) ? fs.readdirSync(base) : []) used.add(e.split(V)[0]);
let pi = 0;
const pick = () => { while (pi < POOL.length && (used.has(POOL[pi]) || text.includes(POOL[pi]))) pi++; const e = POOL[pi++]; used.add(e); return e; };
const phases = [];
const names = new Set();
runs.forEach((run, i) => {
  if (run.length === 1 && run[0].keep) return;
  const tests = run.map(titleOf).find(Boolean);
  const first = run[0];
  const dn = run.flatMap(declNames);
  const hook = run.some(usesHook);
  let nm, doc, slugWords;
  if (tests && run.some((s) => titleOf(s))) { const ws = words(tests).slice(0, 4); slugWords = ws; nm = "register" + (pascal(ws) || "Part" + (i + 1)) + "Tests"; doc = `Registers the ${ws.join(" ")} tests of ${fname}.`; }
  else {
    const ws = words(dn[0] || "part" + (i + 1)).slice(0, 3); slugWords = ws;
    const extra = dn.slice(0, 4).join(", ");
    nm = (hook ? "use" : "build") + (pascal(ws) || "Part" + (i + 1)) + (dn.length > 1 ? "State" : "");
    doc = extra ? `Builds ${extra} of ${fname}.` : `Runs the effects and registrations of ${fname}.`;
  }
  let u = nm, k = 2; while (names.has(u)) u = nm + k++; names.add(u);
  const fe = pick(), pe = pick();
  const slug = (slugWords.join("-") || "part-" + (i + 1)).slice(0, 36);
  phases.push({ dir: fe + V + slug, fdoc: `/** ${fe}${V} The ${slugWords.join(" ") || "part"} phase of ${fname}. */`, ph: { name: u, emoji: pe, doc, from: first.a, to: run[run.length - 1].a } });
});
const keeps = body.filter((s) => s.keep).map((s) => s.a);
const se = pick();
const spec = { file: path.resolve(file), function: fname, keep: keeps, resolve: true, typesFromProducers: true, allowAny: true, ...(arrowVar ? { arrowVar: true } : {}), shared: { dir: se + V + "shared", doc: `/** ${se}${V} The shared declarations of ${fname}. */` }, files: phases.map((p) => ({ dir: p.dir, doc: p.fdoc, phases: [p.ph] })) };
fs.writeFileSync(out, JSON.stringify(spec, null, 1));
console.log(`${fname}${arrowVar ? " (arrow)" : ""} ${phases.length} phases, keep=${keeps.join(",") || "-"}, final return=${hasReturn}`);
console.log(phases.map((p) => `${p.ph.from}-${p.ph.to} ${p.ph.name}`).join("\n"));
