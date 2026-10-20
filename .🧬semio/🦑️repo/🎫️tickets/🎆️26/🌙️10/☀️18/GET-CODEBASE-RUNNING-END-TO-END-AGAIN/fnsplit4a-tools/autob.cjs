// usage: node autob.cjs <file> <scopeLine> <out.json> [max=150] : proposes a bx range spec (one folder per range) for the function-like starting at scopeLine
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [file, name, out, max = "150"] = process.argv.slice(2);
const V = "️";
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
let fn = null;
const find0 = (n) => { if (fn) return; if (ts.isFunctionLike(n) && n.body && ts.isBlock(n.body) && L(n.getStart(sf)) === +name) { fn = n; return; } ts.forEachChild(n, find0); };
find0(sf);
if (!fn) throw new Error("scope function not found");
const fname = fn.name ? fn.name.getText(sf) : (fn.parent && ts.isVariableDeclaration(fn.parent) ? fn.parent.name.getText(sf) : "the function");
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
const items = [], dirs = [];
const names = new Set();
runs.forEach((run, i) => {
  if (run.length === 1 && run[0].keep) return;
  const tests = run.map(titleOf).find(Boolean);
  const dn = run.flatMap(declNames);
  const hook = run.some(usesHook);
  let nm, doc, slugWords;
  if (tests) { const ws = words(tests).slice(0, 4); slugWords = ws; nm = "register" + (pascal(ws) || "Part" + (i + 1)) + "Tests"; doc = `Registers the ${ws.join(" ")} tests of ${fname}.`; }
  else { const ws = words(dn[0] || "step" + (i + 1)).slice(0, 3); slugWords = ws; nm = (hook ? "use" : "run") + (pascal(ws) || "Step" + (i + 1)) + "Step"; doc = dn.length ? `Runs the ${dn.slice(0, 4).join(", ")} step of ${fname}.` : `Runs one step of ${fname}.`; }
  let u = nm, kk = 2; while (names.has(u)) u = nm + kk++; names.add(u);
  const fe = pick(), pe = pick();
  const slug = (slugWords.join("-") || "step-" + (i + 1)).slice(0, 36);
  dirs.push({ id: "f" + i, dir: fe + V + slug, doc: `/** ${fe}${V} The ${slugWords.join(" ") || "step"} step of ${fname}. */` });
  items.push({ id: "i" + i, kind: "range", line: run[0].a, to: run[run.length - 1].b, name: u, emoji: pe, doc, file: "f" + i });
});
const spec = { file: path.resolve(file), scopeLine: +name, files: dirs, items };
fs.writeFileSync(out, JSON.stringify(spec, null, 1));
console.log(`${fname} ${items.length} ranges`);
console.log(items.map((p) => `${p.line}-${p.to} ${p.name}`).join("\n"));
