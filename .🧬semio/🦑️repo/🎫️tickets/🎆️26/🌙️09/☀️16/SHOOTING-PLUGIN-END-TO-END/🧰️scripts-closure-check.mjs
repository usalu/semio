import {readdirSync, readFileSync, statSync, existsSync} from "node:fs";
import {join, dirname, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const pk = new Map();
function walk(d) { let ents; try { ents = readdirSync(d); } catch { return; }
  for (const e of ents) { if (["node_modules",".git","target","dist"].includes(e)) continue; const p = join(d, e); let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p); else if (e === "package.json") { try { const j = JSON.parse(readFileSync(p, "utf8")); if (j.name?.startsWith("@semio-tech/")) pk.set(j.name, {dir: d, exports: j.exports}); } catch {} } } }
walk(join(ROOT, "🧰️framework")); walk(join(ROOT, "✏️s"));
const strip = s => s.replace(/\/\*[\s\S]*?\*\//g, "").replace(/(^|[^:"'`\\])\/\/[^\n]*/g, "$1");
function resolveSpec(from, spec) {
  let base;
  if (spec.startsWith(".")) base = resolve(dirname(from), spec);
  else if (spec.startsWith("@semio-tech/")) { const parts = spec.split("/"); const name = parts.slice(0,2).join("/"); const p = pk.get(name); if (!p) return {err: "NO-PACKAGE " + name};
    const sub = "./" + parts.slice(2).join("/"); const ex = p.exports; if (!ex) { base = join(p.dir, parts.slice(2).join("/") || "index.ts"); } else { const key = sub === "./" ? "." : sub; if (!(key in ex)) return {err: "SUBPATH-NOT-EXPORTED " + spec}; let t = ex[key]; if (typeof t === "object") t = t.default ?? t.import ?? Object.values(t)[0]; base = join(p.dir, t); } }
  else return null; // external
  for (const c of [base, base + ".ts", base + ".tsx", base + ".mjs", base + ".js", base + ".json", join(base, "index.ts")]) if (existsSync(c) && statSync(c).isFile()) return {file: c};
  return {err: "FILE-MISSING " + spec + " -> " + base.slice(ROOT.length+1)};
}
const seen = new Set(), queue = [join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts"), join(ROOT, "✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🚀️entry/🟦️.ts"), join(ROOT, "✏️s/🧑‍💻dev/🎭️variants/🎥️shooting/🏗️builder/🌐️vite/🟦️.ts")], problems = [];
const specRe = /(?:import|export)\s[^;]*?from\s*["']([^"']+)["']|import\s*\(\s*["']([^"']+)["']\s*\)|import\s*["']([^"']+)["']/g;
while (queue.length) {
  const f = queue.shift(); if (seen.has(f)) continue; seen.add(f);
  if (!/\.(ts|tsx|mjs|js)$/.test(f)) continue;
  let src; try { src = strip(readFileSync(f, "utf8")); } catch { problems.push({from: f.slice(ROOT.length+1), err: "UNREADABLE"}); continue; }
  for (const m of src.matchAll(specRe)) { const spec = m[1] ?? m[2] ?? m[3]; if (!spec) continue; const r = resolveSpec(f, spec); if (!r) continue; if (r.err) { problems.push({from: f.slice(ROOT.length+1), err: r.err}); continue; } queue.push(r.file); }
}
const uniq = [...new Map(problems.map(p => [p.from + "|" + p.err, p])).values()];
console.log(JSON.stringify({visited: seen.size, problems: uniq}, null, 1));
