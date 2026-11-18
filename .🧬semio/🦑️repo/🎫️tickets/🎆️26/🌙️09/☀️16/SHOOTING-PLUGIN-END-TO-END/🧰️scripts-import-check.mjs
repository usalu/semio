import {readdirSync, readFileSync, statSync, existsSync} from "node:fs";
import {join, dirname, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SCOPES = [
  "✏️s/🔌️plugins/🎥️shooting",
  "✏️s/🧑‍💻dev/🎭️variants/🎥️shooting",
  "✏️s/🧑‍💻dev/🧩️catalog",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve",
];
const SKIP = /node_modules|\/target\/|\/dist\/|\/\.git\//;
function walk(d, out = []) {
  for (const e of readdirSync(d)) {
    const p = join(d, e);
    if (SKIP.test(p + "/")) continue;
    const st = statSync(p);
    if (st.isDirectory()) walk(p, out);
    else if (/\.(ts|tsx|mts|mjs)$/.test(e)) out.push(p);
  }
  return out;
}
const files = SCOPES.flatMap(s => walk(join(ROOT, s)));
const exportCache = new Map();
function exportsOf(file) {
  if (exportCache.has(file)) return exportCache.get(file);
  const src = readFileSync(file, "utf8");
  const names = new Set(); let star = false;
  for (const m of src.matchAll(/export\s+(?:declare\s+)?(?:async\s+)?(?:function\*?|const|let|var|class|type|interface|enum)\s+([\p{L}\p{N}_$]+)/gu)) names.add(m[1]);
  for (const m of src.matchAll(/export\s*(?:type\s*)?\{([^}]*)\}/g)) for (const part of m[1].split(",")) {
    const n = part.trim().replace(/^type\s+/, "").split(/\s+as\s+/).pop().trim(); if (n) names.add(n);
  }
  for (const m of src.matchAll(/export\s*\*\s*(?:as\s+[\p{L}\p{N}_$]+\s*)?from\s*["']([^"']+)["']/gu)) star = true;
  const r = {names, star, src};
  exportCache.set(file, r); return r;
}
function resolveRel(from, spec) {
  const base = resolve(dirname(from), spec);
  for (const c of [base, base + ".ts", base + ".tsx", base + ".mts", base + ".mjs", base + ".json", join(base, "index.ts")]) {
    if (existsSync(c) && statSync(c).isFile()) return c;
  }
  return null;
}
const pkgIndex = {};
function findPkg(scope, name) {
  // workspace packages by name
  return pkgIndex[name] ?? null;
}
const findings = [];
const specRe = /(?:import|export)\s+(?:type\s+)?([\s\S]*?)\s*from\s*["']([^"']+)["']|import\s*\(\s*["']([^"']+)["']\s*\)|import\s*["']([^"']+)["']/g;
for (const f of files) {
  const src = readFileSync(f, "utf8");
  for (const m of src.matchAll(specRe)) {
    const clause = m[1], spec = m[2] ?? m[3] ?? m[4];
    if (!spec) continue;
    const rel = f.slice(ROOT.length + 1);
    if (spec.startsWith(".")) {
      const target = resolveRel(f, spec);
      if (!target) { findings.push({kind: "missing-relative", file: rel, spec}); continue; }
      if (clause && !/^\*/.test(clause.trim())) {
        const want = clause.replace(/[{}]/g, "").split(",").map(s => s.trim().replace(/^type\s+/, "").split(/\s+as\s+/)[0].trim()).filter(Boolean);
        if (want.length && target.endsWith(".ts")) {
          const ex = exportsOf(target);
          if (!ex.star) for (const w of want) if (!ex.names.has(w) && !/^default$/.test(w) && !w.startsWith("type ") ) findings.push({kind: "missing-export", file: rel, spec, name: w, target: target.slice(ROOT.length + 1)});
        }
      }
    } else if (spec.startsWith("@semio-tech/")) {
      findings.push({kind: "bare-package", file: rel, spec});
    }
  }
}
console.log(JSON.stringify({files: files.length, findings}, null, 1));
