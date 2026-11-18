import {readdirSync, readFileSync, statSync, existsSync} from "node:fs";
import {join, dirname, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SCOPES = ["✏️s/🔌️plugins/🎥️shooting","✏️s/🧑‍💻dev/🎭️variants/🎥️shooting","✏️s/🧑‍💻dev/🧩️catalog","🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve"];
const pk = new Map();
function walk(d) { let ents; try { ents = readdirSync(d); } catch { return; }
  for (const e of ents) { if (["node_modules",".git","target","dist"].includes(e)) continue; const p = join(d, e); let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p); else if (e === "package.json") { try { const j = JSON.parse(readFileSync(p, "utf8")); if (j.name?.startsWith("@semio-tech/")) pk.set(j.name, {dir: d, exports: j.exports}); } catch {} } } }
walk(join(ROOT, "🧰️framework")); walk(join(ROOT, "✏️s"));
const strip = s => s.replace(/\/\*[\s\S]*?\*\//g, "").replace(/(^|[^:"'`\\])\/\/[^\n]*/g, "$1");
function walkTs(d, out = []) { for (const e of readdirSync(d)) { const p = join(d, e); if (["node_modules","target","dist"].includes(e)) continue; const st = statSync(p); if (st.isDirectory()) walkTs(p, out); else if (/\.(ts|tsx)$/.test(e)) out.push(p); } return out; }
function resolveFile(from, spec) {
  let base;
  if (spec.startsWith(".")) base = resolve(dirname(from), spec);
  else { const parts = spec.split("/"); const name = spec.startsWith("@") ? parts.slice(0,2).join("/") : parts[0];
    const p = pk.get(name); if (!p) return {err: "NO-PACKAGE"}; const sub = "./" + parts.slice(name.split("/").length).join("/"); const ex = p.exports;
    if (!ex) return {err: "no-exports"}; const key = sub === "./" ? "." : sub; if (!(key in ex)) return {err: "SUBPATH-NOT-EXPORTED"};
    let t = ex[key]; if (typeof t === "object") t = t.default ?? t.import ?? Object.values(t)[0]; base = join(p.dir, t); }
  for (const c of [base, base + ".ts", base + ".tsx", base + ".mts", base + ".mjs", base + ".json", join(base, "index.ts")]) if (existsSync(c) && statSync(c).isFile()) return {file: c};
  return {err: "FILE-MISSING:" + base};
}
const cache = new Map();
function exportNames(file, depth = 0) {
  if (cache.has(file)) return cache.get(file);
  const names = new Set(); let unknown = false; cache.set(file, {names, unknown: true});
  if (depth > 6 || !/\.(ts|tsx|mts|mjs)$/.test(file)) { return {names, unknown: true}; }
  const src = strip(readFileSync(file, "utf8"));
  for (const m of src.matchAll(/export\s+(?:declare\s+)?(?:async\s+)?(?:function\*?|const|let|var|class|type|interface|enum|abstract\s+class)\s+([\p{L}\p{N}_$]+)/gu)) names.add(m[1]);
  for (const m of src.matchAll(/export\s+(?:type\s+)?\{([^}]*)\}(\s*from\s*["']([^"']+)["'])?/g)) for (const part of m[1].split(",")) { const n = part.trim().replace(/^type\s+/, "").split(/\s+as\s+/).pop().trim(); if (n) names.add(n); }
  for (const m of src.matchAll(/export\s*\*\s*(as\s+([\p{L}\p{N}_$]+)\s*)?from\s*["']([^"']+)["']/gu)) { if (m[2]) names.add(m[2]); else { const r = resolveFile(file, m[3]); if (r.file) { const sub = exportNames(r.file, depth + 1); for (const n of sub.names) names.add(n); if (sub.unknown) unknown = true; } else unknown = true; } }
  if (/export\s+default/.test(src)) names.add("default");
  const res = {names, unknown}; cache.set(file, res); return res;
}
const out = [];
const files = SCOPES.flatMap(s => walkTs(join(ROOT, s)));
for (const f of files) {
  const rel = f.slice(ROOT.length + 1);
  const src = strip(readFileSync(f, "utf8"));
  for (const m of src.matchAll(/import\s+(type\s+)?([^;]*?)\s+from\s*["']([^"']+)["']/g)) {
    const clause = m[2], spec = m[3];
    if (/^\s*type\s/.test(clause)) continue;
    const r = resolveFile(f, spec);
    if (r.err) { out.push({rel, spec, problem: r.err}); continue; }
    const brace = clause.match(/\{([^}]*)\}/);
    if (!brace) continue;
    const names = brace[1].split(",").map(s => s.trim()).filter(Boolean).filter(s => !/^type\s/.test(s)).map(s => s.split(/\s+as\s+/)[0].trim());
    if (!r.file.match(/\.(ts|tsx|mts|mjs)$/)) continue;
    const ex = exportNames(r.file);
    for (const n of names) if (!ex.names.has(n) && !ex.unknown) out.push({rel, spec, missingName: n, target: r.file.slice(ROOT.length + 1)});
    if (ex.unknown && names.some(n => !ex.names.has(n))) out.push({rel, spec, unverified: names.filter(n => !ex.names.has(n)), target: r.file.slice(ROOT.length + 1)});
  }
}
console.log(JSON.stringify({files: files.length, out}, null, 1));
