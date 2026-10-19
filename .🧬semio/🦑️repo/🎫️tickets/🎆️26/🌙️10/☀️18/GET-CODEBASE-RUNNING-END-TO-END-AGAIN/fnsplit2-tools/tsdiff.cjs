// 🔬 TypeScript diagnostic differential. usage: node tsdiff.cjs <rootFile> <snapshot> [childFile...]
// Program A has the original text for rootFile (no children), program B is the live tree (root + children). Compares the distinct (code, message) sets and the counts per code of the files involved.
const ts = require(process.env.TYPESCRIPT_PATH || "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
const fs = require("fs");
const path = require("path");
let [rootFile, snapshot, ...children] = process.argv.slice(2);
if (children[0] === "--spec") {
  const spec = JSON.parse(fs.readFileSync(children[1], "utf8"));
  const dirs = [];
  if (spec.kit) dirs.push(spec.kit.dir);
  if (spec.shared) dirs.push(spec.shared.dir);
  if (spec.helpers) dirs.push(spec.helpers.dir);
  for (const g of spec.groups || []) { dirs.push(g.dir); if (g.helpers) dirs.push(g.helpers.dir); for (const p of g.parts || []) dirs.push(p.dir); }
  for (const p of spec.parts || []) dirs.push(p.dir);
  for (const f of spec.files || []) dirs.push(f.dir);
  const base = path.dirname(spec.file);
  children = dirs.map((d) => path.join(base, d, "🟦️" + path.extname(spec.file)));
}
const REPO = "/Users/ueli/Documents/semio";
const abs = (p) => (path.isAbsolute(p) ? p : path.join(REPO, p));
const cfg = JSON.parse(fs.readFileSync(path.join(REPO, "tsconfig.json"), "utf8"));
const { options } = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, noEmit: true, incremental: false, types: [] }, REPO);
options.noEmit = true;
if (process.env.TSDIFF_REACT === "1") {
  const NM = process.env.TSDIFF_NM || "/Users/ueli/Documents/semio-kinan-wip/node_modules";
  options.baseUrl = REPO;
  options.paths = { react: [NM + "/@types/react"], "react/*": [NM + "/@types/react/*"], "react-dom": [NM + "/@types/react-dom"], "react-dom/*": [NM + "/@types/react-dom/*"] };
}
const root = abs(rootFile);
const origText = fs.readFileSync(abs(snapshot), "utf8");
const run = (names, override) => {
  const host = ts.createCompilerHost(options);
  const get = host.getSourceFile.bind(host);
  host.getSourceFile = (fileName, lv, ...r) => (override && path.resolve(fileName) === root ? ts.createSourceFile(fileName, origText, lv, true) : get(fileName, lv, ...r));
  const program = ts.createProgram(names, options, host);
  const rows = [];
  for (const n of names) {
    const sf = program.getSourceFile(n);
    if (!sf) { rows.push({ file: path.basename(path.dirname(n)), code: 0, text: "FILE-NOT-FOUND " + n }); continue; }
    for (const d of [...program.getSyntacticDiagnostics(sf), ...program.getSemanticDiagnostics(sf)]) rows.push({ file: path.basename(path.dirname(n)), code: d.code, text: ts.flattenDiagnosticMessageText(d.messageText, "\n").split("\n")[0].slice(0, 220) });
  }
  return rows;
};
const A = run([root], true);
const B = run([root, ...children.map(abs)], false);
const key = (r) => `${r.code}: ${r.text}`;
const setA = new Set(A.map(key)), setB = new Set(B.map(key));
const onlyNew = [...setB].filter((k) => !setA.has(k)), onlyOld = [...setA].filter((k) => !setB.has(k));
const count = (rows) => rows.reduce((m, r) => (m.set(r.code, (m.get(r.code) || 0) + 1), m), new Map());
const cA = count(A), cB = count(B);
console.log(`diagnostics orig=${A.length} new=${B.length} (files: 1 -> ${1 + children.length})`);
console.log("by code orig:", JSON.stringify([...cA]), "new:", JSON.stringify([...cB]));
for (const k of onlyNew) console.log("  +new ", k, "[" + [...new Set(B.filter((r) => key(r) === k).map((r) => r.file))].join(", ") + "]");
for (const k of onlyOld) console.log("  -orig", k);
console.log(onlyNew.length ? "TSDIFF DIFFERENCES" : "TSDIFF CLEAN");
