import fs from "node:fs";
import path from "node:path";
import { analyze } from "./analyze.mjs";
const E = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph";
const [orig, rootDir, kidsFile] = process.argv.slice(2);
const kids = JSON.parse(fs.readFileSync(kidsFile, "utf8"));
process.env.SPLIT_SOURCE = path.resolve(orig);
const a = analyze(path.join(E, "🟦️.tsx"));
const base = rootDir === "-" ? E : rootDir;
const files = [path.join(base, "🟦️.tsx"), ...kids.map((k) => ["🟦️.ts", "🟦️.tsx"].map((f) => path.join(base, k, f)).find(fs.existsSync))];
const texts = files.map((f) => fs.readFileSync(f, "utf8"));
let ok = 0, missing = [], dup = [], exportDecls = 0;
const norm = (s) => s.replace(/[ \t]+$/gm, "");
for (const u of a.units) {
  if (u.exportDecl) { exportDecls++; continue; }
  const t = u.stmt.getText(a.sf);
  const n = texts.reduce((c, x) => c + norm(x).split(norm(t)).length - 1, 0);
  if (n === 1) ok++; else if (n === 0) missing.push(u.names.join(",") || "<stmt>"); else dup.push(u.names.join(",") + "x" + n);
}
console.log("units", a.units.length, "exportDecls", exportDecls, "exactly-once", ok, "missing", missing, "dup", dup);
