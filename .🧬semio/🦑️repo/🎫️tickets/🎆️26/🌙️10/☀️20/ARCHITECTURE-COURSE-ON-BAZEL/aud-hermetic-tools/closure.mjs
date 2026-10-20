// 📦 Static import closure of the architecture course router and its verbs; read-only audit helper.
import { readFileSync, existsSync, statSync, readdirSync, realpathSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve, relative, sep, extname } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const COURSE = join(ROOT, "🎓️teaching/🏛️architecture/📚️course");
const PKG = join(COURSE, "📦️packages/🟦️typescript");
const require = createRequire(join(ROOT, "package.json"));
const picomatch = require("picomatch");
const EXT = [".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs", ".json", ".css", ".d.ts"];
const TOP = ["🧰️framework", "🎓️teaching", "🎭️", ".github", ".cache", "node_modules"];

const projectJson = JSON.parse(readFileSync(join(PKG, "📋️project.json"), "utf8"));
const defaults = projectJson.namedInputs.default;
const sub = (g) => (typeof g === "string" ? g : null);
const substitute = (g) => g.replaceAll("{projectRoot}", "📦️packages/🟦️typescript").replaceAll("{workspaceRoot}", ".");
const globs = defaults.filter((x) => typeof x === "string").map(substitute);
const positive = globs.filter((g) => !g.startsWith("!") && g !== "sharedGlobals");
const negative = globs.filter((g) => g.startsWith("!")).map((g) => g.slice(1));
const isDeclared = (rel) => positive.some((g) => picomatch(g, { dot: true })(rel)) && !negative.some((g) => picomatch(g, { dot: true })(rel));

const isFile = (p) => { try { return statSync(p).isFile(); } catch { return false; } };
function tryFile(base) {
  if (isFile(base)) return base;
  for (const e of EXT) if (isFile(base + e)) return base + e;
  for (const e of EXT) if (isFile(join(base, "index" + e))) return join(base, "index" + e);
  return undefined;
}
function pkgTarget(exportsField, sp) {
  let target;
  if (!exportsField) return undefined;
  if (typeof exportsField === "string") { if (sp === ".") target = exportsField; }
  else if (sp in exportsField) target = exportsField[sp];
  if (target && typeof target === "object") target = target.import ?? target.default ?? target.node ?? target.types;
  if (Array.isArray(target)) target = target[0];
  return typeof target === "string" ? target : undefined;
}
const external = new Set();
function resolveBare(spec, from) {
  const parts = spec.split("/");
  const name = spec.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0];
  const rest = spec.slice(name.length);
  const dir = join(ROOT, "node_modules", name);
  if (!existsSync(dir)) return undefined;
  const real = realpathSync(dir);
  if (!real.startsWith(ROOT + sep) || real.includes(`${sep}.cache${sep}`)) { external.add(name); return undefined; }
  const pj = JSON.parse(readFileSync(join(real, "package.json"), "utf8"));
  const sp = rest ? "." + rest : ".";
  const target = pkgTarget(pj.exports, sp);
  if (target) return tryFile(resolve(real, target)) ?? resolve(real, target);
  return tryFile(join(real, rest || "")) ?? (rest ? undefined : tryFile(join(real, pj.main ?? "index.js")));
}
const ALIAS = (() => {
  try {
    const raw = readFileSync(join(PKG, "tsconfig.json"), "utf8").replace(/\/\/.*$/gm, "");
    return JSON.parse(raw).compilerOptions.paths;
  } catch (e) { return {}; }
})();
function resolveSpec(spec, from) {
  if (spec.startsWith("node:")) return undefined;
  if (spec in ALIAS) return tryFile(resolve(PKG, ALIAS[spec][0]));
  if (spec.startsWith(".") || spec.startsWith("/")) return tryFile(resolve(dirname(from), spec));
  return resolveBare(spec, from);
}

const IMPORT_RE = /(?:import|export)\s[^"'`;]*?\sfrom\s*["']([^"']+)["']|import\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|require\(\s*["']([^"']+)["']\s*\)|createRequire\([^)]*\)\s*\(\s*["']([^"']+)["']/g;
const LITERAL_RE = /["'`]((?:🧰️framework|🎓️teaching|🎭️|🐾️pets|🧫️|🔣️|🎨️)[^"'`\s]*?\.(?:ts|tsx|js|mjs|json|css|rs|toml|feature))["'`]/g;
const seen = new Map();
const edges = [];
function visit(file, depth = 0) {
  if (seen.has(file)) return;
  seen.set(file, depth);
  if (!/\.(ts|tsx|mts|js|mjs|cjs|css)$/.test(file)) return;
  let text;
  try { text = readFileSync(file, "utf8"); } catch { return; }
  for (const m of text.matchAll(IMPORT_RE)) {
    const spec = m[1] ?? m[2] ?? m[3] ?? m[4] ?? m[5];
    if (!spec) continue;
    const target = resolveSpec(spec, file);
    if (target) { edges.push([file, target]); visit(target, depth + 1); }
  }
  if (!process.env.IMPORT_ONLY) for (const m of text.matchAll(LITERAL_RE)) {
    const literal = m[1];
    const abs = resolve(ROOT, literal);
    if (isFile(abs)) { edges.push([file, abs]); visit(abs, depth + 1); }
  }
  if (!process.env.IMPORT_ONLY) for (const m of text.matchAll(/["'`](\.{1,2}\/[^"'`\s]+?\.(?:ts|tsx|js|mjs|json|css|rs|toml|feature))["'`]/g)) {
    const abs = resolve(dirname(file), m[1]);
    if (isFile(abs)) { edges.push([file, abs]); visit(abs, depth + 1); }
  }
}

// Entries: the router, the course root and vite config, deploy verbs, e2e, stack, tests, proctor bootstrap.
const entries = [
  join(PKG, "📜️script.ts"), join(COURSE, "🟦️.ts"), join(COURSE, "🏗️builder/🌐️vite/🟦️.ts"), join(COURSE, "🧱️stack/🟦️.ts"),
  join(COURSE, "🎭️e2e/🟦️.ts"), join(COURSE, "🎭️e2e/🎚️config/🟦️.ts"), join(COURSE, "🧪️tests/🎚️config/🟦️.ts"),
  join(ROOT, "🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts"), join(ROOT, "🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts"),
];
const walk = (dir, out = []) => { for (const e of readdirSync(dir, { withFileTypes: true })) { const p = join(dir, e.name); if (e.name === "node_modules" || e.name === "dist") continue; if (e.isDirectory()) walk(p, out); else if (/\.(ts|tsx)$/.test(e.name)) out.push(p); } return out; };
for (const f of [...walk(join(COURSE, "🚀️deploy")), ...walk(join(COURSE, "🎭️e2e")), ...walk(join(COURSE, "🏗️builder")), ...walk(join(COURSE, "🧱️stack"))]) entries.push(f);
if (process.env.WITH_TESTS) for (const f of walk(join(COURSE, "🧪️tests"))) entries.push(f);
for (const e of entries) visit(e);

const reached = [...seen.keys()].map((f) => relative(ROOT, f)).sort();
const undeclared = reached.filter((rel) => !isDeclared(rel));
const out = {
  entries: entries.map((e) => relative(ROOT, e)),
  reachedCount: reached.length,
  undeclaredCount: undeclared.length,
  externalBarePackagesResolvedThroughCache: [...external].sort(),
  reached,
  undeclared,
  positiveGlobs: positive,
  negativeGlobs: negative,
};
out.edges = edges.map(([a,b]) => [relative(ROOT,a), relative(ROOT,b)]);
writeFileSync(process.argv[2], JSON.stringify(out));
console.log(JSON.stringify({ reached: reached.length, undeclared: undeclared.length, external: [...external].length }));
