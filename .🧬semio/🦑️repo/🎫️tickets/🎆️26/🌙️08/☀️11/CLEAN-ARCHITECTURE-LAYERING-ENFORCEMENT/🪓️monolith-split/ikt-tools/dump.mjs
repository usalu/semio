import fs from "node:fs";
import { analyze } from "./analyze.mjs";
const [file, src, out] = process.argv.slice(2);
if (src && src !== "-") process.env.SPLIT_SOURCE = src;
const a = analyze(file);
const rows = a.units.map((u) => ({ i: u.idx, k: u.kind.replace("Declaration", "").replace("Statement", ""), n: u.names.join(","), x: u.exported ? 1 : 0, t: u.isType ? 1 : 0, ed: u.exportDecl ? 1 : 0, s: a.lineOf(u.start), e: a.lineOf(u.end), refs: [...a.unitRefs[u.idx]], imps: [...a.importRefs[u.idx]], jsx: a.jsx[u.idx] ? 1 : 0, g: a.find(u.idx) }));
fs.writeFileSync(out, JSON.stringify(rows));
console.log("units", rows.length, "imports", a.importDecls.length);
