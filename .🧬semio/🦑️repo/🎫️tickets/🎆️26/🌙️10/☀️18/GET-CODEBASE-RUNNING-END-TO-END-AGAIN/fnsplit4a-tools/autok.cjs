// usage: node autok.cjs <file> <fnName> <out.json> [max=150] [helpers=0|1] : proposes a ksplit2 spec for a registration function
const ts = require("/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs"), path = require("path");
const [file, fname, out, max = "150", helpersFlag = "0"] = process.argv.slice(2);
const V = "️";
const text = fs.readFileSync(file, "utf8");
const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
const L = (p) => sf.getLineAndCharacterOfPosition(p).line + 1;
const F = sf.statements.find((s) => ts.isFunctionDeclaration(s) && s.name && s.name.text === fname);
if (!F) throw new Error("function declaration not found");
const paramNames = new Set(F.parameters.map((p) => p.name.getText(sf)));
const kindOf = (n) => {
  if (ts.isTypeAliasDeclaration(n) || ts.isInterfaceDeclaration(n)) return "type";
  if (ts.isVariableStatement(n)) { const d = n.declarationList.declarations; if (d.length === 1 && d[0].initializer && ts.isIdentifier(d[0].initializer) && ts.isObjectBindingPattern(d[0].name) && paramNames.has(d[0].initializer.text)) return "destruct"; return "decl"; }
  if (ts.isFunctionDeclaration(n) || ts.isClassDeclaration(n)) return "decl";
  return "reg";
};
const st = [...F.body.statements].map((n) => ({ n, a: L(n.getStart(sf)), b: L(n.getEnd()), kind: kindOf(n) }));
const STOP = new Set("the a an of and to is are with for in on by from as at or its it that this be into than then when without every each".split(" "));
const words = (t) => t.replace(/([a-z0-9])([A-Z])/g, "$1 $2").toLowerCase().replace(/[^a-z0-9 ]+/g, " ").split(/\s+/).filter((w) => w && !STOP.has(w));
const pascal = (ws) => ws.map((w) => w[0].toUpperCase() + w.slice(1)).join("");
const titleOf = (n) => { let t = null; const w = (x) => { if (t) return; if (ts.isCallExpression(x)) { const ex = x.expression.getText(sf); if (/^(test|it|describe)(\.\w+)*$/.test(ex) && x.arguments[0] && ts.isStringLiteralLike(x.arguments[0])) { t = x.arguments[0].text; return; } } ts.forEachChild(x, w); }; w(n); return t; };
const POOL = "🔶 🔷 🔸 🔹 🟠 🟡 🟢 🔵 🟣 🟤 ⭐ 🌟 💠 🔰 🔱 🎯 🎲 🧩 🧱 🪵 🪨 🧲 🪝 🔩 ⚙ 🔧 🔨 🪛 🪚 🧰 🗜 🔗 ⛓ 📌 📍 📎 🖇 📏 📐 ✂ 🗂 🗃 🗄 📂 📁 📦 📮 🏷 🔖 🧾 📜 📃 📄 📑 🔍 🔎 🔬 🔭 📡 🛰 🧭 🗺 🧳 🎒 🪁 🎈 🎀 🎁 🎐 🪅 🧸 🎨 🖌 🖍 🪄 🔮 🧿 🪬 🧫 🧬 💎 🔔 🔕 📣 📢 🎵 🎶 🎼 🥁 🪘 🎷 🎺 🎸 🪕 🎻 🪗 🏆 🥇 🎖 🏅 🚩 🏁 🏴 🏳 🪧 🚦 🚧 ⚓ ⛵ 🛶 🚀 🛸 🚁 🛩 🚂 🚃 🚲 🛴 🛹 🛼 ⛽ 🛞 🪂 ⏱ ⏲ ⏰ 🕰 ⌛ ⏳ 🌡 ☀ 🌙 ⭕ 🌈 🔥 💧 🌊 ❄ ⚡ ☔ 🍀 🌱 🌿 🌳 🌲 🌴 🌵 🌾 🍁 🍂 🌸 🌼 🌻 🌺 🪻 🪷 🐝 🐞 🦋 🐌 🐜 🪲 🕷 🦂 🐢 🦎 🐙 🦀 🦞 🦐 🐠 🐬 🦈 🐳 🦭 🦦 🦫 🐿 🦔 🦇 🐦 🦉 🦅 🦆 🦢 🦜 🪶 🐓".split(" ");
const base = path.dirname(file);
const used = new Set();
for (const e of fs.existsSync(base) ? fs.readdirSync(base) : []) used.add(e.split(V)[0]);
let pi = 0;
const pick = () => { while (pi < POOL.length && (used.has(POOL[pi]) || text.includes(POOL[pi]))) pi++; const e = POOL[pi++]; used.add(e); return e; };
const names = new Set();
const uniq = (nm) => { let u = nm, k = 2; while (names.has(u)) u = nm + k++; names.add(u); return u; };
const chunk = (items, getA, getB) => { const runs = []; let cur = [], size = 0; for (const it of items) { const len = getB(it) - getA(it) + 1; if (cur.length && size + len > +max) { runs.push(cur); cur = []; size = 0; } cur.push(it); size += len; } if (cur.length) runs.push(cur); return runs; };
const mkGroup = (run, getA, getB, getNode) => {
  const t = run.map((x) => titleOf(getNode(x))).find(Boolean) || fname;
  const ws = words(t).slice(0, 4);
  const e = pick(), fe = pick();
  const slug = (ws.join("-") || "part").slice(0, 36);
  const nm = uniq("register" + (pascal(ws) || "Part") + "Tests");
  return { dir: e + V + slug, doc: `/** ${e}${V} The ${ws.join(" ")} tests of ${fname}. */`, name: nm, emoji: fe, fdoc: `Registers the ${ws.join(" ")} tests of ${fname}.`, from: getA(run[0]), to: getA(run[run.length - 1]) };
};
const regs = st.filter((s) => s.kind === "reg");
const groups = [];
let plainRun = [];
const flushPlain = () => { for (const run of chunk(plainRun, (s) => s.a, (s) => s.b)) groups.push(mkGroup(run, (s) => s.a, (s) => s.b, (s) => s.n)); plainRun = []; };
for (const r of regs) {
  const call = ts.isExpressionStatement(r.n) && ts.isCallExpression(r.n.expression) ? r.n.expression : null;
  const cb = call ? call.arguments[call.arguments.length - 1] : null;
  const isBlockCb = cb && (ts.isArrowFunction(cb) || ts.isFunctionExpression(cb)) && ts.isBlock(cb.body) && cb.parameters.length === 0;
  if (r.b - r.a + 1 > +max && isBlockCb) {
    flushPlain();
    const inner = [...cb.body.statements].map((n) => ({ n, a: L(n.getStart(sf)), b: L(n.getEnd()) }));
    const title = titleOf(r.n) || fname;
    const ws = words(title).slice(0, 4);
    const e = pick(), fe = pick();
    const g = { dir: e + V + (ws.join("-") || "suite").slice(0, 36), doc: `/** ${e}${V} The ${ws.join(" ")} describe block of ${fname}: its wrapper and parts. */`, name: uniq("register" + (pascal(ws) || "Suite") + "Suite"), emoji: fe, fdoc: `Registers the ${ws.join(" ")} describe block from its parts.`, describeLine: r.a, parts: [] };
    if (helpersFlag === "1") { const he = pick(), hf = pick(); g.helpers = { dir: he + V + "helpers", doc: `/** ${he}${V} The callback-local helpers shared by the parts of the ${ws.join(" ")} describe block. */`, name: uniq("create" + (pascal(ws) || "Suite") + "Helpers"), emoji: hf, fdoc: `Creates the callback-local helpers shared by the parts of the ${ws.join(" ")} describe block.`, typeName: (pascal(ws) || "Suite") + "Helpers" }; }
    for (const run of chunk(inner, (s) => s.a, (s) => s.b)) g.parts.push(mkGroup(run, (s) => s.a, (s) => s.b, (s) => s.n));
    groups.push(g);
  } else plainRun.push(r);
}
flushPlain();
const ke = pick(), kf = pick();
const core = fname.replace(/^register/, "").replace(/Tests$/, "") || "Registration";
const kit = { dir: ke + V + "test-kit", doc: `/** ${ke}${V} The shared test kit of ${fname}: the destructured dependencies and the shared fixtures. */`, name: "create" + core + "Kit", emoji: kf, fdoc: `Creates the shared test kit of ${fname} from the injected dependencies.`, typeName: core + "Kit" };
fs.writeFileSync(out, JSON.stringify({ file: path.resolve(file), function: fname, kit, groups, rootKeep: [] }, null, 1));
console.log(groups.map((g) => g.describeLine ? `split ${g.describeLine} (${g.parts.length} parts) ${g.name}` : `${g.from}-${g.to} ${g.name}`).join("\n"));
