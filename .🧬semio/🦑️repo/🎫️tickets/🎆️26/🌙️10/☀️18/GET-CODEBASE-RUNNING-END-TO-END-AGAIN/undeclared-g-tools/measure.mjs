// keys-g measure: effective declared key size per cached target from the sealed graph generation (offline, read-only).
// usage: bun measure.mjs <out.jsonl> <projectRegex> [generationDir]
// One JSON row per cached target: files matched by the expanded positive patterns minus the negations, closure-token files, owner glob, env/runtime/external counts.
// Streams rows; aborts above 700 MB RSS.
import {readFileSync, writeFileSync, appendFileSync, existsSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {spawnSync} from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio";
process.chdir(ROOT);
const L = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {buildTaskInputsV1} = await import(L + "/🧮️task-model/🟦️.ts");
const {inputGlobRegExpV1} = await import(L + "/📥️inputs/🎯️precision/🟦️.ts");
const [out, projectRegex = ".", generations = ".cache/b/g/m/generations/"] = process.argv.slice(2);
const wanted = new RegExp(projectRegex, "u");
const only = process.env.KEYS_G_ONLY ? new Set(process.env.KEYS_G_ONLY.split(",")) : undefined;
const dumpDirectory = process.env.KEYS_G_DUMP;
const patched = (target, change) => change.__append ? {...target, inputs: [...(target.inputs ?? []).filter((input) => !(typeof input === "string" && change.__replace.includes(input))), ...change.__append]} : {...target, ...change};
const patch = process.env.KEYS_G_PATCH ? JSON.parse(readFileSync(process.env.KEYS_G_PATCH, "utf8")) : {};
const rss = () => process.memoryUsage().rss / 1048576;
const limit = Number(process.env.KEYS_G_RSS_MB ?? 900);
const guard = (where = "") => { if (rss() > limit) { Bun.gc(true); if (rss() > limit) { console.error(`[keys-g] RSS ${Math.round(rss())} MB above ${limit} MB at ${where}, aborting`); process.exit(3); } } };

const cur = JSON.parse(readFileSync(generations + "current.json", "utf8"));
const manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
const projects = {}, roots = {}, fragments = {};
let dependencies = [];
const readFragment = (row) => JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + row.fragment + ".json.br")).toString("utf8"));
const edgesCache = (process.env.KEYS_G_EDGES ?? "/private/tmp/claude-501/keys-g-edges") + `-${cur.generation}.json`;
if (existsSync(edgesCache)) {
  const cached = JSON.parse(readFileSync(edgesCache, "utf8"));
  dependencies = cached.dependencies;
  for (const name of cached.names) fragments[name] = manifest.owners[name];
} else {
  for (const [name, row] of Object.entries(manifest.owners)) {
    if (row.kind !== "project") continue;
    let fragment;
    try { fragment = readFragment(row); } catch { continue; }
    fragments[name] = row;
    roots[name] = fragment.root;
    for (const edge of fragment.edges ?? []) dependencies.push({source: edge.source, target: edge.target});
  }
  writeFileSync(edgesCache, JSON.stringify({names: Object.keys(fragments), dependencies}));
}
const adjacency = new Map();
for (const edge of dependencies) { const rows = adjacency.get(edge.source) ?? []; rows.push(edge.target); adjacency.set(edge.source, rows); }
const needed = new Set(Object.keys(fragments).filter((name) => wanted.test(name))), pending = [...needed];
const referenced = (value) => { if (typeof value === "string") { const at = value.lastIndexOf(":"); return at > 0 ? [value.slice(0, at)] : []; } return typeof value?.target === "string" ? referenced(value.target) : []; };
const admit = (name) => { if (!needed.has(name) && fragments[name]) { needed.add(name); pending.push(name); } };
for (let at = 0; at < pending.length; at++) {
  const name = pending[at];
  for (const target of adjacency.get(name) ?? []) admit(target);
  if (wanted.test(name)) {
    const fragment = readFragment(fragments[name]);
    for (const definition of Object.values(fragment.node?.targets ?? {})) for (const dependency of definition.dependsOn ?? []) for (const project of referenced(dependency)) admit(project);
  }
}
for (const name of needed) {
  const fragment = readFragment(fragments[name]);
  const changes = patch[name] ?? {};
  const rewritten = Object.fromEntries(Object.entries(fragment.node?.namedInputs ?? {}).map(([key, list]) => [key, (changes.rewrite && Array.isArray(list)) ? list.flatMap((item) => (typeof item === "string" && changes.rewrite[item]) ? changes.rewrite[item] : [item]) : list]));
  projects[name] = {name, root: fragment.root, namedInputs: {...rewritten, ...(changes.namedInputs ?? {})}, targets: wanted.test(name) ? Object.fromEntries(Object.entries(fragment.node?.targets ?? {}).map(([key, value]) => [key, changes.targets?.[key] ? patched(value, changes.targets[key]) : (changes.targets?.["*"] && value.cache === true && Array.isArray(value.inputs) && value.inputs.some((input) => changes.targets["*"].when.includes(input)) && value.metadata?.semio?.inputPrecision === undefined) ? {...value, inputs: [...value.inputs, ...changes.targets["*"].append]} : value])) : {}};
  guard("load " + name);
}
console.error(`[keys-g] loaded ${Object.keys(projects).length} projects, RSS ${Math.round(rss())} MB`);
const policy = JSON.parse(readFileSync(L + "/🔣️policy.json", "utf8")).namedInputs;
const graph = {version: 1, projects, dependencies, namedInputs: policy};

const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {maxBuffer: 256 * 1024 * 1024, encoding: "utf8"});
const tracked = listing.stdout.split("\0").filter(Boolean);
const generated = spawnSync("find", ["🧰️framework", "🎓️teaching", "♻️mit-bestand", "-type", "f", "-path", "*/🤖️generated/*", "-not", "-path", "*/node_modules/*", "-not", "-path", "*/target/*"], {maxBuffer: 64 * 1024 * 1024, encoding: "utf8"}).stdout.split("\n").filter(Boolean);
const files = [...new Set([...tracked, ...generated])].sort();
const fileSet = new Set(files);
const lowerBound = (prefix) => { let lo = 0, hi = files.length; while (lo < hi) { const mid = (lo + hi) >> 1; if (files[mid] < prefix) lo = mid + 1; else hi = mid; } return lo; };
const literalPrefix = (pattern) => { const wildcard = pattern.search(/[*?{]/u); return wildcard < 0 ? pattern : pattern.slice(0, wildcard); };
const matchPattern = (pattern) => {
  const prefix = literalPrefix(pattern), expression = inputGlobRegExpV1(pattern), rows = [];
  if (prefix === pattern) return fileSet.has(pattern) ? [pattern] : [];
  for (let at = lowerBound(prefix); at < files.length && files[at].startsWith(prefix); at++) if (expression.test(files[at])) rows.push(files[at]);
  return rows;
};
const closureCache = new Map();
const closureFiles = (token) => {
  const key = token.slice("SEMIO_SOURCE_CLOSURE_".length);
  if (closureCache.has(key)) return closureCache.get(key);
  const path = ".cache/b/g/m/build-input-closures/" + key + ".json";
  let rows = [];
  try { rows = JSON.parse(readFileSync(path, "utf8")).files.filter((file) => fileSet.has(file)); } catch {}
  if (closureCache.size > 64) closureCache.clear();
  closureCache.set(key, rows);
  return rows;
};
const TEST = /(🧪|🧫|\/tests?\/|\/fixtures\/|\.test\.|\.spec\.|\.feature$)/u;
writeFileSync(out, "");
const sink = {write: (text) => appendFileSync(out, text), end: () => {}};
let count = 0;
for (const [name, project] of Object.entries(projects)) {
  if (!wanted.test(name)) continue;
  for (const [target, definition] of Object.entries(project.targets)) {
    if (only && !only.has(`${name}:${target}`)) continue;
    const cache = definition.cache;
    if (cache !== true && !(cache && typeof cache === "object")) continue;
    let entries;
    try { entries = buildTaskInputsV1(graph, {project: name, definition}); } catch (error) { sink.write(JSON.stringify({project: name, target, error: String(error.message).slice(0, 200)}) + "\n"); continue; }
    if (dumpDirectory) writeFileSync(`${dumpDirectory}/${name.replace(/[/@]/gu, "_")}__${target}.entries.json`, JSON.stringify(entries));
    const included = new Set(), negatives = [], positives = [], tokens = [], environment = [], runtime = [];
    let external = 0, dependencyOutputs = 0;
    for (const entry of entries) {
      if (entry.kind === "file") {
        if (entry.exclude) negatives.push(entry.pattern); else positives.push(entry.pattern);
      } else if (entry.kind === "environment") {
        if (entry.name.startsWith("SEMIO_SOURCE_CLOSURE_")) tokens.push(entry.name); else environment.push(entry.name);
      } else if (entry.kind === "runtime") runtime.push(entry.command);
      else if (entry.kind === "external") external += entry.names.length;
      else if (entry.kind === "dependency-outputs") dependencyOutputs++;
    }
    const unique = [...new Set(positives)], perPattern = [];
    for (const pattern of unique) {
      const rows = matchPattern(pattern);
      perPattern.push([rows.length, pattern]);
      for (const file of rows) included.add(file);
    }
    let closureCount = 0;
    for (const token of new Set(tokens)) for (const file of closureFiles(token)) { if (!included.has(file)) closureCount++; included.add(file); }
    const negative = [...new Set(negatives)].map((pattern) => inputGlobRegExpV1(pattern));
    if (negative.length) for (const file of included) if (negative.some((expression) => expression.test(file))) included.delete(file);
    perPattern.sort((left, right) => right[0] - left[0]);
    if (dumpDirectory) writeFileSync(`${dumpDirectory}/${name.replace(/[/@]/gu, "_")}__${target}.files`, [...included].sort().join("\n") + "\n");
    let tests = 0;
    for (const file of included) if (TEST.test(file)) tests++;
    console.error(`[keys-g] ${name}:${target} ${included.size} files ${Math.round(rss())} MB`);
    sink.write(JSON.stringify({project: name, target, files: included.size, tests, closureOnly: closureCount, patterns: unique.length, negatives: negative.length, tokens: new Set(tokens).size, environment: [...new Set(environment)].sort(), runtime: [...new Set(runtime)].sort(), external, dependencyOutputs, top: perPattern.slice(0, 6), inputPrecision: definition.metadata?.semio?.inputPrecision ?? null}) + "\n");
    count++;
    guard(name+":"+target);
  }
}
sink.end();
console.error(`[keys-g] ${count} targets, generation ${cur.generation}, ${files.length} files, peak RSS ${Math.round(rss())} MB`);
