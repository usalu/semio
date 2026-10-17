// 🧮️ Read-only census of every first-party Rust crate in every workspace against the compiler-ingress source boundary, measuring the lib and the cfg(test) closures with the canonical reader.
import {readFileSync, readdirSync, existsSync, writeFileSync} from "node:fs";
import {basename, dirname, join, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated";
const PREFIX = "cgc7";
const SCOPE = (process.env.SCOPE ?? "").split(",").filter(Boolean).map(s => resolve(ROOT, s));
const TAG = process.env.TAG ?? "scoped";
const LIMIT = {files: 32, bytes: 262144, lines: 4000};

type Limit = {files: number; bytes: number; lines: number};
type Plain = {kind: string; name: string; entry: string};
type Target = Plain & {files: number; bytes: number; lines: number; embeddedBytes: number; ratio: number; binding: string; error: string};
type Node = {dir: string; manifest: string; name: string; parsed: any; ws: string | undefined; member: boolean; normal: string[]; dev: string[]; features: Set<string>; limit: Limit; lib: Target[]; test: Target[]};

const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostCfg = execFileSync(rustc, ["--print", "cfg"], {encoding: "utf8"}).trim().split("\n")
  .map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));

const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};

const tomlCache = new Map<string, any>();
const manifestOf = (path: string): any => {
  const hit = tomlCache.get(path);
  if (hit) return hit;
  const parsed = Bun.TOML.parse(readFileSync(path, "utf8")) as any;
  tomlCache.set(path, parsed);
  return parsed;
};
const rel = (path: string) => relative(ROOT, path) || ".";

const manifestPaths = readFileSync(join(OUT, `${PREFIX}-manifests.txt`), "utf8").split("\n").filter(Boolean).map(p => resolve(ROOT, p)).filter(p => existsSync(p));
const workspaceDirs = new Set<string>();
for (const path of manifestPaths) if (manifestOf(path).workspace) workspaceDirs.add(dirname(path));

const expandPattern = (base: string, pattern: string): string[] => {
  if (/[*?[]/u.test(pattern)) return [...new Bun.Glob(pattern + "/Cargo.toml").scanSync({cwd: base})].map(file => resolve(base, dirname(file)));
  return [resolve(base, pattern)];
};
const globToRegex = (pattern: string): RegExp => {
  let source = "";
  for (let index = 0; index < pattern.length; index++) {
    const char = pattern[index]!;
    if (pattern.startsWith("**/", index)) { source += "(?:.*/)?"; index += 2; }
    else if (pattern.startsWith("/**", index) && index + 3 === pattern.length) { source += "(?:/.*)?"; index += 2; }
    else if (pattern.startsWith("**", index)) { source += ".*"; index += 1; }
    else if (char === "*") source += "[^/]*";
    else if (char === "?") source += "[^/]";
    else source += char.replace(/[.+^${}()|[\]\\]/gu, "\\$&");
  }
  return new RegExp("^" + source + "$", "u");
};
const memberOf = new Map<string, string>();
for (const ws of workspaceDirs) {
  const table = manifestOf(join(ws, "Cargo.toml")).workspace;
  const excludes = ((table.exclude ?? []) as string[]).map(globToRegex);
  const isExcluded = (dir: string) => { const rel = relative(ws, dir); return excludes.some(regex => regex.test(rel)); };
  for (const pattern of table.members ?? []) for (const dir of expandPattern(ws, pattern)) if (!isExcluded(dir) && !memberOf.has(dir)) memberOf.set(dir, ws);
}
const wsOf = (dir: string): string | undefined => {
  if (memberOf.has(dir)) return memberOf.get(dir);
  let current = dir;
  while (!workspaceDirs.has(current)) {
    const up = dirname(current);
    if (up === current) return undefined;
    current = up;
  }
  return current;
};

const featureSet = (parsed: any): Set<string> => {
  const table: Record<string, string[]> = parsed.features ?? {};
  const out = new Set<string>(["default"]), queue = [...(table.default ?? [])];
  while (queue.length) {
    const token = queue.pop()!;
    if (out.has(token)) continue;
    out.add(token);
    if (token.startsWith("dep:")) continue;
    const [head, rest] = token.split("/") as [string, string | undefined];
    const name = head.replace(/\?$/u, "");
    if (table[name]) queue.push(...table[name]);
    if (rest !== undefined && !head.endsWith("?")) out.add(name);
  }
  return out;
};

const depDirsOf = (parsed: any, dir: string, ws: string | undefined, enabled: Set<string>): {normal: string[]; dev: string[]} => {
  const wsDeps: Record<string, any> = ws ? (manifestOf(join(ws, "Cargo.toml")).workspace?.dependencies ?? {}) : {};
  const targets = Object.values(parsed.target ?? {}) as any[];
  const collect = (tables: any[], normal: boolean): string[] => {
    const out: string[] = [];
    for (const table of tables) for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const inherited = spec.workspace === true && ws !== undefined;
      const resolved = inherited ? (wsDeps[spec.package ?? key] ?? {}) : spec;
      if (typeof resolved.path !== "string") continue;
      if (normal && resolved.optional === true && !enabled.has(key) && !enabled.has("dep:" + key)) continue;
      out.push(resolve(inherited ? ws! : dir, resolved.path));
    }
    return [...new Set(out)];
  };
  return {
    normal: collect([parsed.dependencies, parsed["build-dependencies"], ...targets.flatMap(t => [t.dependencies, t["build-dependencies"]])], true),
    dev: collect([parsed["dev-dependencies"], ...targets.map(t => t["dev-dependencies"])], false),
  };
};

const libTargetsOf = (parsed: any, dir: string, pkg: string): Plain[] => {
  const out: Plain[] = [];
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  if (existsSync(libEntry)) out.push({kind: "lib", name: parsed.lib?.name ?? pkg.replaceAll("-", "_"), entry: libEntry});
  for (const bin of parsed.bin ?? []) out.push({kind: "bin", name: bin.name, entry: resolve(dir, bin.path ?? `src/bin/${bin.name}.rs`)});
  if (parsed.autobins !== false) {
    const main = resolve(dir, "src/main.rs");
    if (existsSync(main)) out.push({kind: "bin", name: pkg, entry: main});
    const binDir = resolve(dir, "src/bin");
    if (existsSync(binDir)) for (const row of readdirSync(binDir, {withFileTypes: true})) {
      if (row.isFile() && row.name.endsWith(".rs")) out.push({kind: "bin", name: basename(row.name, ".rs"), entry: join(binDir, row.name)});
      else if (row.isDirectory() && existsSync(join(binDir, row.name, "main.rs"))) out.push({kind: "bin", name: row.name, entry: join(binDir, row.name, "main.rs")});
    }
  }
  const build = resolve(dir, parsed.build ?? "build.rs");
  if (parsed.build !== false && existsSync(build)) out.push({kind: "build-script", name: "build-script-build", entry: build});
  return dedupe(out);
};

const testTargetsOf = (parsed: any, dir: string, pkg: string): Plain[] => {
  const out: Plain[] = [];
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  if (existsSync(libEntry) && parsed.lib?.test !== false) out.push({kind: "lib-test", name: parsed.lib?.name ?? pkg.replaceAll("-", "_"), entry: libEntry});
  for (const test of parsed.test ?? []) out.push({kind: "itest", name: test.name, entry: resolve(dir, test.path ?? `tests/${test.name}.rs`)});
  if (parsed.autotests !== false) {
    const testDir = resolve(dir, "tests");
    if (existsSync(testDir)) for (const row of readdirSync(testDir, {withFileTypes: true})) {
      if (row.isFile() && row.name.endsWith(".rs")) out.push({kind: "itest", name: basename(row.name, ".rs"), entry: join(testDir, row.name)});
      else if (row.isDirectory() && existsSync(join(testDir, row.name, "main.rs"))) out.push({kind: "itest", name: row.name, entry: join(testDir, row.name, "main.rs")});
    }
  }
  return dedupe(out.filter(target => existsSync(target.entry)));
};

const dedupe = (targets: Plain[]): Plain[] => {
  const seen = new Set<string>();
  return targets.filter(target => seen.has(target.entry) ? false : (seen.add(target.entry), true));
};

const effectiveLimit = (parsed: any): Limit => {
  const declared = parsed.package?.metadata?.semio?.["rust-source-boundary"];
  return declared
    ? {files: Math.min(LIMIT.files, declared.maxSourceFiles), bytes: Math.min(LIMIT.bytes, declared.maxSourceBytes), lines: Math.min(LIMIT.lines, declared.maxActiveLines)}
    : {...LIMIT};
};

const cfgFor = (node: Node, test: boolean): string[] => [
  ...hostCfg, "debug_assertions", ...(test ? ["test"] : []),
  ...[...node.features].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f),
];

const measure = async (target: Plain, dir: string, cfg: string[], limit: Limit): Promise<Target> => {
  try {
    const inputs = await readRustSourceClosureV1([target.entry], ROOT, cfg, readFile, {manifestDirectory: dir, generated: () => undefined});
    const files = inputs.length;
    const bytes = inputs.reduce((sum, input) => sum + input.bytes, 0);
    const lines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
    const embedded = inputs.filter(input => input.kind === "embedded");
    const parts = {files: files / limit.files, bytes: bytes / limit.bytes, lines: lines / limit.lines};
    const binding = Object.entries(parts).sort((a, b) => b[1] - a[1])[0]![0];
    return {...target, files, bytes, lines, embeddedBytes: embedded.reduce((sum, input) => sum + input.bytes, 0), ratio: Math.max(parts.files, parts.bytes, parts.lines), binding, error: ""};
  } catch (error) {
    return {...target, files: 0, bytes: 0, lines: 0, embeddedBytes: 0, ratio: Infinity, binding: "refused", error: String((error as Error).message ?? error).replaceAll("\n", " ").slice(0, 200)};
  }
};

// Node discovery: every [package] manifest on disk plus every member and every first-party path dependency (normal and dev) reached from them.
const nodes = new Map<string, Node>();
const missing: string[] = [];
const notPackage: string[] = [];
const pending = [...new Set([...manifestPaths.map(p => dirname(p)), ...memberOf.keys()])];
const seenDirs = new Set<string>();
while (pending.length) {
  const dir = pending.shift()!;
  if (seenDirs.has(dir)) continue;
  seenDirs.add(dir);
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { missing.push(rel(dir)); continue; }
  const parsed = manifestOf(manifest);
  if (!parsed.package) { notPackage.push(rel(dir)); continue; }
  const ws = wsOf(dir);
  const features = featureSet(parsed);
  const deps = depDirsOf(parsed, dir, ws, features);
  nodes.set(dir, {dir, manifest, name: parsed.package.name ?? basename(dir), parsed, ws, member: memberOf.has(dir), normal: deps.normal, dev: deps.dev, features, limit: effectiveLimit(parsed), lib: [], test: []});
  pending.push(...deps.normal, ...deps.dev);
}

// Measurement: sequential, read-only.
let measured = 0;
for (const node of nodes.values()) {
  if (SCOPE.length && !SCOPE.some(prefix => node.dir === prefix || node.dir.startsWith(prefix + "/"))) continue;
  const cfgLib = cfgFor(node, false), cfgTest = cfgFor(node, true);
  for (const target of libTargetsOf(node.parsed, node.dir, node.name)) node.lib.push(await measure(target, node.dir, cfgLib, node.limit));
  for (const target of testTargetsOf(node.parsed, node.dir, node.name)) node.test.push(await measure(target, node.dir, cfgTest, node.limit));
  measured++;
  if (measured % 100 === 0) console.error(`[DEBUG] measured ${measured}/${nodes.size}`);
}
console.error(`[DEBUG] measured ${measured}/${nodes.size}`);


const fmt = (t: Target | undefined) => t ? `${t.files}/${t.bytes}/${t.lines}${t.ratio > 1 ? " OVER(" + t.binding + ")" : ""}${t.error ? " ERR:" + t.error : ""}` : "-";
const worst = (targets: Target[]) => [...targets].sort((a, b) => b.ratio - a.ratio)[0];
const out: string[] = ["crate\tlib(files/bytes/lines)\ttest(files/bytes/lines)\ttestTarget\tmember"];
let over = 0, count = 0;
for (const node of [...nodes.values()].sort((a, b) => a.dir.localeCompare(b.dir))) {
  if (!node.lib.length && !node.test.length) continue;
  count++;
  const l = worst(node.lib), t = worst(node.test);
  if ((l && l.ratio > 1) || (t && t.ratio > 1) || (l && l.error) || (t && t.error)) over++;
  out.push([node.name, fmt(l), fmt(t), t ? t.kind + ":" + t.name : "-", node.member].join("\t"));
}
writeFileSync(join(OUT, `r8a-census-${TAG}.tsv`), out.join("\n") + "\n");
console.log(out.join("\n"));
console.log(`crates ${count} over-or-error ${over}`);
