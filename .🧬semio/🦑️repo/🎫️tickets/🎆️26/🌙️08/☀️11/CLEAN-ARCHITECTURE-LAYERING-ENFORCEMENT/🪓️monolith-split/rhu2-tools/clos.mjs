import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const ts = require("typescript");
const R = "/Users/ueli/Documents/semio/";
const opts = { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, allowImportingTsExtensions: true, resolveJsonModule: true, customConditions: ["semio-source"] };
const host = ts.sys;
const cache = new Map();
export function imports(file) {
  if (cache.has(file)) return cache.get(file);
  let out = [];
  try {
    const txt = fs.readFileSync(file, "utf8");
    const info = ts.preProcessFile(txt, true, true);
    const mods = [...info.importedFiles.map(x => x.fileName)];
    for (const m of mods) {
      const r = ts.resolveModuleName(m, file, opts, host);
      const f = r.resolvedModule?.resolvedFileName;
      if (f && !f.includes("/node_modules/")) out.push({ spec: m, file: f });
      else if (f) out.push({ spec: m, file: f, ext: true });
      else out.push({ spec: m, file: null });
    }
  } catch {}
  cache.set(file, out);
  return out;
}
export function closure(roots, stop = () => false, includeTests = false) {
  const seen = new Set(); const st = roots.map(r => r.startsWith("/") ? r : R + r);
  while (st.length) { const n = st.pop(); if (seen.has(n)) continue; seen.add(n); if (stop(n) && !roots.includes(n)) continue; for (const d of imports(n)) if (d.file && !d.ext && (includeTests || !/\/🧪️tests\//.test(d.file))) st.push(d.file); }
  return seen;
}
if (process.argv[1].endsWith("clos.mjs")) {
  const [mode, ...roots] = process.argv.slice(2);
  const ENG = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/";
  const OSA = R + "🧰️framework/🛍️products/💻️os/";
  const set = closure(roots, n => !n.startsWith(OSA) || n.startsWith(ENG));
  const list = [...set].map(p => p.replace(R, "")).sort();
  const OSP = "🧰️framework/🛍️products/💻️os/";
  console.log("total", list.length);
  for (const p of list) console.log(p.replace(OSP, "OS/").replace("🧰️framework/", "F/"));
}
