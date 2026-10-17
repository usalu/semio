/** 🔍️ Real Vite resolve of sample `@semio-tech/*` subpath specifiers under the alias list a config declares: `node resolve-check.mjs <phase> <config-file...>`.
 * The alias list is read from the config source text (`alias: [` or `alias: {` block, evaluated in a stub scope), because the builders cannot be loaded without staged wasm deliveries.
 * Each aliased `@semio-tech/<pkg>` contributes its bare specifier and up to 6 sampled `exports` subpaths; a spec passes when the resolved file exists and is not a swallowed `<replacement>/<subpath>` path. */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { readFileSync, existsSync, realpathSync, writeFileSync } from "node:fs";
import { join, dirname, resolve as pathResolve, join as pathJoin, dirname as pathDirname } from "node:path";

const R = "/Users/ueli/Documents/semio";
const G = process.env.G;
const [phase, ...args] = process.argv.slice(2);
const overlay = new Map();
const files = args.map((arg) => {
  const [rel, snap] = arg.split("::");
  if (snap) overlay.set(rel.startsWith("/") ? rel : `${R}/${rel}`, snap);
  return rel;
});
const readSrcFile = (file) => (overlay.has(file) ? readFileSync(overlay.get(file), "utf8") : readFileSync(file, "utf8"));

function block(src, key) {
  const i = src.search(new RegExp(`\\b${key}\\s*:\\s*[\\[{]`, "u"));
  if (i < 0) return null;
  const open = src.indexOf(src.slice(i).match(/[\[{]/u)[0], i);
  const closeChar = src[open] === "[" ? "]" : "}";
  let depth = 0, inStr = null, inRe = false;
  for (let k = open; k < src.length; k++) {
    const c = src[k];
    if (inStr) { if (c === "\\") k++; else if (c === inStr) inStr = null; continue; }
    if (c === '"' || c === "'" || c === "`") { inStr = c; continue; }
    if (c === "/" && src[k + 1] === "/") { k = src.indexOf("\n", k); continue; }
    if (c === "[" || c === "{" || c === "(") depth++;
    if (c === "]" || c === "}" || c === ")") { depth--; if (depth === 0 && c === closeChar) return src.slice(open, k + 1); }
  }
  return null;
}

const U = new Proxy(function () {}, {
  get: (_t, p) => (p === Symbol.iterator ? function* () {} : p === Symbol.toPrimitive ? () => "<unknown>" : p === "map" || p === "flatMap" || p === "filter" ? () => [] : U),
  apply: () => U,
  has: () => true,
});
const joinStr = (...a) => a.map(String).join("/").replace(/\/+/gu, "/");

const preludeCache = new Map();
function scopeFor(file) {
  if (preludeCache.has(file)) return preludeCache.get(file);
  const known = {
    repoRoot: R, resolve: pathResolve, join: pathJoin, dirname: pathDirname, fileURLToPath: (u) => String(u).replace(/^file:\/\//u, ""), pathToFileURL: (p) => ({ href: "file://" + p }),
    path: { resolve: pathResolve, join: pathJoin, dirname: pathDirname, relative: (a, b) => b },
    exactPackage: (n) => new RegExp(`^${n.replace(/[/\\]/gu, "\\$&")}$`, "u"),
    packageTree: (n) => new RegExp(`^${n.replace(/[/\\]/gu, "\\$&")}(?:/.*)?$`, "u"),
    frameworkSourcePath: (p) => `${R}/🧰️framework/${p}`,
    MODULE_PLUGIN_ROUTE: "/🔌️plugin-modules", MODULE_EXTENSION_ROUTE: "/🧩️extension-modules",
    reactRoot: R + "/node_modules/react", reactDomRoot: R + "/node_modules/react-dom",
  };
  const scope = new Proxy(known, { has: (_t, p) => typeof p === "string" && !(p in globalThis) && p !== "undefined", get: (t, p) => (typeof p === "symbol" ? undefined : p in t ? t[p] : U), set: (t, p, v) => { t[p] = v; return true; } });
  const src = readSrcFile(file)
    .replaceAll("import.meta.url", JSON.stringify("file://" + file))
    .replaceAll("import.meta.dirname", JSON.stringify(pathDirname(file)));
  for (const m of src.matchAll(/^(?:export )?const ([A-Za-z_$][\w$]*)\s*=\s*(.*?);(?:\s*\/\/.*)?\s*$/gmu)) {
    if (m[1] in known && m[1] !== "root") continue;
    try { known[m[1]] = new Function("scope", `with (scope) { return (${m[2]}); }`)(scope); } catch { }
  }
  preludeCache.set(file, scope);
  return scope;
}

function evaluate(text, file) {
  const scope = scopeFor(file);
  return new Function("scope", `with (scope) { return (${text}); }`)(scope);
}

function objectsWithFind(src) {
  const rows = [];
  const re = /\{\s*find\s*:/gu;
  let m;
  while ((m = re.exec(src))) {
    let depth = 0, inStr = null;
    for (let k = m.index; k < src.length; k++) {
      const c = src[k];
      if (inStr) { if (c === "\\") k++; else if (c === inStr) inStr = null; continue; }
      if (c === '"' || c === "'" || c === "`") { inStr = c; continue; }
      if (c === "{" || c === "(" || c === "[") depth++;
      if (c === "}" || c === ")" || c === "]") { depth--; if (depth === 0) { rows.push(src.slice(m.index, k + 1)); break; } }
    }
  }
  return rows;
}

function readAliases(file) {
  const src = readSrcFile(file);
  const found = [];
  for (const text of objectsWithFind(src)) {
    try { found.push(evaluate(text, file)); } catch { }
  }
  const text = block(src, "alias");
  if (text && text.startsWith("{")) {
    try { for (const [find, replacement] of Object.entries(evaluate(text, file))) found.push({ find, replacement }); } catch { }
  }
  const seen = new Set();
  return found.filter((r) => r && (typeof r.find === "string" || r.find instanceof RegExp) && typeof r.replacement === "string").filter((r) => {
    const key = String(r.find) + "=>" + r.replacement;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

function packageOf(find) {
  const s = find instanceof RegExp ? find.source.replace(/\\\//gu, "/") : find;
  const m = s.match(/@semio-tech\/([a-z0-9-]+)/u);
  return m ? m[1] : null;
}

function samples(pkg) {
  const p = `${R}/node_modules/@semio-tech/${pkg}/package.json`;
  if (!existsSync(p)) return [];
  const ex = JSON.parse(readFileSync(p, "utf8")).exports;
  const subs = ex && typeof ex === "object" ? Object.keys(ex).filter((k) => k.startsWith("./") && !k.includes("*") && k !== "./package.json") : [];
  const picked = subs.length <= 6 ? subs : [0, 1, Math.floor(subs.length / 3), Math.floor(subs.length / 2), subs.length - 2, subs.length - 1].map((i) => subs[i]);
  return [`@semio-tech/${pkg}`, ...[...new Set(picked)].map((k) => `@semio-tech/${pkg}/${k.slice(2)}`)];
}

function pinned(rows, spec) {
  return rows.some((x) => x.find instanceof RegExp && x.find.test(spec) && (x.find.source.includes("(?:") || x.find.source === `^${spec.replace(/[/\\]/gu, "\\/")}$`));
}

const out = {};
for (const file of files) {
  const abs = file.startsWith("/") ? file : `${R}/${file}`;
  const aliases = readAliases(abs);
  if (!Array.isArray(aliases)) { out[file] = { error: aliases.error }; continue; }
  const sem = aliases.filter((a) => packageOf(a.find));
  const pkgs = [...new Set(sem.map((a) => packageOf(a.find)))];
  const specs = [...new Set(pkgs.flatMap(samples))];
  const importer = join(dirname(abs), "x.ts");
  const mk = async (alias) => createServer({ root: R, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [R] }, watch: null }, resolve: { alias }, optimizeDeps: { noDiscovery: true, include: [] } });
  const withAlias = await mk(sem.map((a) => ({ find: a.find, replacement: a.replacement })));
  const bare = await mk([]);
  const rows = [];
  for (const spec of specs) {
    const a = await withAlias.pluginContainer.resolveId(spec, importer, { ssr: false });
    const n = await bare.pluginContainer.resolveId(spec, importer, { ssr: false });
    const clean = (r) => (r?.id ? String(r.id).split("?")[0] : null);
    const aid = clean(a), nid = clean(n);
    const subpath = spec.includes("/", "@semio-tech/".length);
    const exists = aid && existsSync(aid);
    const covered = sem.some((x) => (x.find instanceof RegExp ? x.find.test(spec) : spec === x.find || spec.startsWith(x.find + "/")));
    let status;
    if (!aid || !exists) status = "UNRESOLVED";
    else if (subpath && !covered) status = "ok-node";
    else if (subpath && nid && realpathSync(aid) === realpathSync(nid)) status = "ok";
    else if (subpath && pinned(sem, spec)) status = "pinned";
    else if (subpath) status = "DIFFERS";
    else status = "ok-bare";
    rows.push({ spec, status, id: aid ? aid.replace(R + "/", "").slice(-90) : null, node: nid ? nid.replace(R + "/", "").slice(-90) : null });
  }
  await withAlias.close();
  await bare.close();
  out[file] = { aliases: sem.length, specs: specs.length, bad: rows.filter((r) => r.status === "UNRESOLVED" || r.status === "DIFFERS").length, rows };
}
writeFileSync(`${G}/ass-resolve-${phase}.json`, JSON.stringify(out, null, 1));
for (const [f, v] of Object.entries(out)) console.log(f.replace(R + "/", "").slice(-70).padEnd(72), v.error ? "ERROR " + v.error.slice(0, 120) : `aliases=${v.aliases} specs=${v.specs} bad=${v.bad}`);
