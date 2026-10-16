// 🧰️ Library copy of tcc-census.ts (unchanged logic, exported) for the RENDERER-TEST-DEPS tools; original: ../tcc-tools/tcc-census.ts. Original description: each first-party crate's own test compilation units (lib under cfg(test), bin and integration test targets, examples for --all-targets) and its first-party dev-dependency closures, measured through the canonical readRustSourceClosureV1 counter for native, wasm32-unknown-unknown and wasm32-wasip2.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync} from "node:fs";
import {basename, dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1, rustCfgMatchesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

export const ROOT = "/Users/ueli/Documents/semio";
export const GEN = resolve(import.meta.dir, "..", "🗑️generated");
export const CSV = join(GEN, "crate-census.csv");
export const HARD = {files: 32, bytes: 262144, lines: 4000};
export const LABELS = ["native", "wasm32-unknown-unknown", "wasm32-wasip2"];
const TRIPLE: Record<string, string | undefined> = {native: undefined, "wasm32-unknown-unknown": "wasm32-unknown-unknown", "wasm32-wasip2": "wasm32-wasip2"};
export const ONLY = process.env.ONLY ? new Set(process.env.ONLY.split(",")) : undefined;

const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
export const hostTriple = execFileSync(rustc, ["-vV"], {encoding: "utf8"}).split("\n").find(line => line.startsWith("host: "))!.slice(6).trim();
const cfgOf = (triple?: string): string[] => execFileSync(rustc, ["--print", "cfg", ...(triple ? ["--target", triple] : [])], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
export const CFG = new Map(LABELS.map(label => [label, cfgOf(TRIPLE[label])]));
export const tripleOf = (label: string) => TRIPLE[label] ?? hostTriple;

const wsRoot = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const wsDeps: Record<string, any> = wsRoot.workspace?.dependencies ?? {};

type Limit = {files: number; bytes: number; lines: number};
type Spec = {key: string; kind: "normal" | "build" | "dev"; target?: string; dir: string; optional: boolean; features: string[]; defaults: boolean};
type Node = {dir: string; name: string; parsed: any; table: Record<string, string[]>; specs: Spec[]; limit: Limit; libEntry?: string};
type Input = {path: string; bytes: number; lines: number; kind: string};
type Measured = {files: number; bytes: number; lines: number; ratio: number; binding: string; error: string; inputs: Input[]};
type Target = {kind: "lib-test" | "bin-test" | "integration" | "example"; name: string; entry: string; test: boolean};
type State = {requested: Set<string>; defaults: boolean; viaDev: boolean; viaNormal: boolean};

export const specsOf = (parsed: any, dir: string): Spec[] => {
  const out: Spec[] = [];
  const add = (table: any, kind: Spec["kind"], target?: string) => {
    for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const inherited = spec.workspace === true;
      const ws = inherited ? (wsDeps[key] ?? wsDeps[spec.package ?? key] ?? {}) : {};
      const path = spec.path ?? ws.path;
      if (typeof path !== "string") continue;
      out.push({key, kind, target, dir: resolve(spec.path === undefined && inherited ? ROOT : dir, path), optional: (spec.optional ?? ws.optional) === true, features: [...(ws.features ?? []), ...(spec.features ?? [])], defaults: (spec["default-features"] ?? spec.default_features ?? ws["default-features"] ?? ws.default_features ?? true) !== false});
    }
  };
  add(parsed.dependencies, "normal");
  add(parsed["build-dependencies"], "build");
  add(parsed["dev-dependencies"], "dev");
  for (const [target, body] of Object.entries(parsed.target ?? {}) as [string, any][]) {
    add(body.dependencies, "normal", target);
    add(body["build-dependencies"], "build", target);
    add(body["dev-dependencies"], "dev", target);
  }
  return out;
};

const nodeCache = new Map<string, Node | null>();
export const loadNode = (dir: string): Node | null => {
  if (nodeCache.has(dir)) return nodeCache.get(dir)!;
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { nodeCache.set(dir, null); return null; }
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const declared = parsed.package?.metadata?.semio?.["rust-source-boundary"];
  const limit: Limit = declared ? {files: Math.min(HARD.files, declared.maxSourceFiles), bytes: Math.min(HARD.bytes, declared.maxSourceBytes), lines: Math.min(HARD.lines, declared.maxActiveLines)} : {...HARD};
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  const node: Node = {dir, name: parsed.package?.name ?? basename(dir), parsed, table: parsed.features ?? {}, specs: specsOf(parsed, dir), limit, libEntry: existsSync(libEntry) ? libEntry : undefined};
  nodeCache.set(dir, node);
  return node;
};

export const closureOf = (table: Record<string, string[]>, roots: string[]): Set<string> => {
  const out = new Set<string>(), queue = [...roots];
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

export const applies = (spec: Spec, label: string): boolean => {
  if (spec.target === undefined) return true;
  if (!spec.target.startsWith("cfg(")) return spec.target === tripleOf(label);
  try { return rustCfgMatchesV1(spec.target.slice(4, -1), CFG.get(label)!); } catch { return true; }
};

export const resolveGraph = (rootDir: string, label: string) => {
  const states = new Map<string, State>(), enabled = new Map<string, Set<string>>(), queue: string[] = [];
  const touch = (dir: string, requested: string[], defaults: boolean, viaDev: boolean, viaNormal: boolean) => {
    let state = states.get(dir), changed = false;
    if (!state) { state = {requested: new Set(), defaults: false, viaDev: false, viaNormal: false}; states.set(dir, state); changed = true; }
    for (const r of requested) if (!state.requested.has(r)) { state.requested.add(r); changed = true; }
    if (defaults && !state.defaults) { state.defaults = true; changed = true; }
    if (viaDev && !state.viaDev) { state.viaDev = true; changed = true; }
    if (viaNormal && !state.viaNormal) { state.viaNormal = true; changed = true; }
    if (changed) queue.push(dir);
  };
  touch(rootDir, [], true, false, true);
  while (queue.length) {
    const dir = queue.shift()!, state = states.get(dir)!, node = loadNode(dir);
    if (!node) continue;
    const seen = closureOf(node.table, [...(state.defaults ? ["default"] : []), ...state.requested]);
    enabled.set(dir, seen);
    const activated = new Set([...seen].map(token => token.startsWith("dep:") ? token.slice(4) : token));
    const pairs = [...seen].filter(token => token.includes("/")).map(token => {
      const slash = token.indexOf("/"), head = token.slice(0, slash);
      return {dep: head.replace(/\?$/u, ""), optional: head.endsWith("?"), feat: token.slice(slash + 1)};
    });
    for (const spec of node.specs) {
      if (!applies(spec, label)) continue;
      if (spec.kind === "dev" && dir !== rootDir) continue;
      if (spec.optional && !activated.has(spec.key)) continue;
      const req = [...spec.features, ...pairs.filter(pair => pair.dep === spec.key && (!pair.optional || activated.has(pair.dep))).map(pair => pair.feat)];
      touch(spec.dir, req, spec.defaults, state.viaDev || spec.kind === "dev", state.viaNormal && spec.kind !== "dev");
    }
  }
  return {states, enabled};
};

export const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};

export const cfgList = (label: string, features: Set<string>, test: boolean): string[] => [
  ...CFG.get(label)!, "debug_assertions",
  ...[...features].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f),
  ...(test ? ["test"] : []),
];

export const measure = async (entry: string, node: Node, cfg: string[]): Promise<Measured> => {
  try {
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: node.dir, generated: () => undefined});
    const files = inputs.length, bytes = inputs.reduce((sum, input) => sum + input.bytes, 0), lines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
    const parts = {files: files / node.limit.files, bytes: bytes / node.limit.bytes, lines: lines / node.limit.lines};
    const binding = Object.entries(parts).sort((a, b) => b[1] - a[1])[0]![0];
    return {files, bytes, lines, ratio: Math.max(parts.files, parts.bytes, parts.lines), binding, error: "", inputs: inputs.map(input => ({path: input.path, bytes: input.bytes, lines: input.activeLines, kind: input.kind}))};
  } catch (error) {
    return {files: 0, bytes: 0, lines: 0, ratio: NaN, binding: "error", error: String((error as Error).message ?? error).replaceAll("\n", " ").slice(0, 220), inputs: []};
  }
};

const normalCache = new Map<string, Measured>();
export const normalLib = async (node: Node, label: string, features: Set<string>): Promise<Measured> => {
  const cfg = cfgList(label, features, false);
  const key = label + "|" + node.dir + "|" + cfg.filter(c => c.startsWith("feature=")).sort().join(",");
  if (!normalCache.has(key)) normalCache.set(key, await measure(node.libEntry!, node, cfg));
  return normalCache.get(key)!;
};

export const autoEntries = (dir: string): [string, string][] => {
  if (!existsSync(dir)) return [];
  const out: [string, string][] = [];
  for (const row of readdirSync(dir, {withFileTypes: true}).sort((a, b) => a.name.localeCompare(b.name))) {
    if (row.isFile() && row.name.endsWith(".rs")) out.push([basename(row.name, ".rs"), join(dir, row.name)]);
    else if (row.isDirectory() && existsSync(join(dir, row.name, "main.rs"))) out.push([row.name, join(dir, row.name, "main.rs")]);
  }
  return out;
};

export const targetsOf = (node: Node, features: Set<string>): Target[] => {
  const p = node.parsed, out: Target[] = [], seen = new Set<string>();
  const ready = (table: any) => (table?.["required-features"] ?? []).every((f: string) => features.has(f));
  const add = (kind: Target["kind"], name: string, entry: string, test: boolean, required: boolean) => {
    if (!required || seen.has(entry) || !existsSync(entry)) return;
    seen.add(entry);
    out.push({kind, name, entry, test});
  };
  if (node.libEntry && p.lib?.test !== false) add("lib-test", node.name, node.libEntry, true, true);
  for (const bin of p.bin ?? []) add("bin-test", bin.name, resolve(node.dir, bin.path ?? `src/bin/${bin.name}.rs`), bin.test !== false, ready(bin));
  if (p.autobins !== false) {
    add("bin-test", node.name, resolve(node.dir, "src/main.rs"), true, true);
    for (const [name, entry] of autoEntries(join(node.dir, "src/bin"))) add("bin-test", name, entry, true, true);
  }
  for (const t of p.test ?? []) add("integration", t.name, resolve(node.dir, t.path ?? `tests/${t.name}.rs`), true, ready(t));
  if (p.autotests !== false) for (const [name, entry] of autoEntries(join(node.dir, "tests"))) add("integration", name, entry, true, true);
  for (const e of p.example ?? []) add("example", e.name, resolve(node.dir, e.path ?? `examples/${e.name}.rs`), false, ready(e));
  if (p.autoexamples !== false) for (const [name, entry] of autoEntries(join(node.dir, "examples"))) add("example", name, entry, false, true);
  return out;
};

export const rel = (path: string) => path.replace(ROOT + "/", "");
export const tsv = (rows: Record<string, unknown>[]): string => {
  if (!rows.length) return "";
  const header = [...new Set(rows.flatMap(row => Object.keys(row)))];
  const cell = (value: unknown) => String(value ?? "").replaceAll("\t", " ").replaceAll("\n", " ");
  return [header.join("\t"), ...rows.map(row => header.map(key => cell(row[key])).join("\t"))].join("\n") + "\n";
};
export const fixed = (value: number) => Number.isFinite(value) ? Number(value.toFixed(3)) : "";

