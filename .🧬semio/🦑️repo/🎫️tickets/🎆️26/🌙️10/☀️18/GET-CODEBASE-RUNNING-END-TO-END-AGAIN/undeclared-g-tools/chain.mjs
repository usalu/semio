// undeclared-g chain: the shortest static/dynamic-literal import chain from an entry module to a target module (bun's Transpiler scanner). usage: bun chain.mjs <entry> <target-substring>
import {readFileSync, existsSync} from "node:fs";
import {dirname, relative, resolve} from "node:path";
const W = "/Users/ueli/Documents/semio", [entry, needle] = process.argv.slice(2);
const loaders = {".ts": "ts", ".tsx": "tsx", ".mts": "ts", ".js": "js", ".mjs": "js", ".cjs": "js"};
const parent = new Map(), queue = [resolve(W, entry)], t = {};
parent.set(queue[0], null);
while (queue.length) {
  const file = queue.shift(), rel = relative(W, file);
  if (rel.includes(needle)) { const chain = []; for (let f = file; f; f = parent.get(f)) chain.push(relative(W, f)); console.log(chain.reverse().join("\n  -> ")); process.exit(0); }
  const loader = loaders[file.slice(file.lastIndexOf("."))];
  if (!loader || !existsSync(file)) continue;
  let imports; try { imports = (t[loader] ??= new Bun.Transpiler({loader})).scanImports(readFileSync(file, "utf8").replace(/^#![^\n]*/u, "")); } catch { continue; }
  for (const row of imports) { if (!row.path.startsWith(".")) continue; const next = resolve(dirname(file), row.path); if (!parent.has(next)) { parent.set(next, file); queue.push(next); } }
}
console.log("no chain");
