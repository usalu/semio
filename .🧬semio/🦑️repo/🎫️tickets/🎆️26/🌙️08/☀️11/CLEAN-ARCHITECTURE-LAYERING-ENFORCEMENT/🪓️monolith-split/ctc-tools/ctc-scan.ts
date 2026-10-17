// 🔎️ Cross-crate cfg(test) scanner: finds `#[cfg(test)]` pub items that another crate (or an integration target of the same crate) references. Usage: bun ctc-scan.ts [--root DIR] [--manifests FILE] [--out PREFIX] [--only crate,crate] [--json]
import {existsSync, readFileSync, readdirSync, statSync, writeFileSync} from "node:fs";
import {basename, dirname, join, relative, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {Cfg, Tok, isClose, isOpen, lex, matchClose, parseCfg} from "./ctc-lex.ts";

const args = process.argv.slice(2);
const flag = (name: string, fallback: string) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1]! : fallback; };
const ROOT = resolve(flag("--root", "/Users/ueli/Documents/semio"));
const OUT = flag("--out", "");
const ONLY = flag("--only", "").split(",").filter(Boolean);
const MANIFESTS = flag("--manifests", "");

type Gate = {kind: "test" | "partial"; cfgs: Cfg[]} | null;
type Rec = {hidden: boolean; crate: string; file: string; line: number; kind: string; name: string; modpath: string[]; owner?: string; gate: NonNullable<Gate>; features: string[]};
type Target = {req?: string[]; kind: "lib" | "bin" | "test" | "bench" | "example" | "build"; name: string; root: string};
type Crate = {dir: string; name: string; lib: string | undefined; libRoot: string | undefined; deps: Map<string, {dir: string; features: Set<string>}>; targets: Target[]; ws: string | undefined};
type UseLeaf = {path: string[]; alias?: string; glob: boolean; line: number};
type Fact = {crate: string; target: string; file: string; ext: string; rest: string[]; line: number; via: "use" | "path" | "glob"; mac: boolean};
type FileFacts = {types: Set<string>; crate: string; target: string; file: string; facts: Fact[]; bare: Set<string>; methods: Set<string>; localDefs: Set<string>; roots: Set<string>};

const rel = (p: string) => relative(ROOT, p) || ".";
const tomlCache = new Map<string, any>();
const toml = (path: string): any => {
  if (tomlCache.has(path)) return tomlCache.get(path);
  let parsed: any = {};
  try { parsed = Bun.TOML.parse(readFileSync(path, "utf8")); } catch { parsed = {}; }
  tomlCache.set(path, parsed);
  return parsed;
};

const git = (extra: string[]): string[] => {
  try { return execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", ...extra], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\0").filter(Boolean); } catch { return []; }
};
let manifestFiles: string[];
if (MANIFESTS) manifestFiles = readFileSync(MANIFESTS, "utf8").split("\n").filter(Boolean).map(p => resolve(ROOT, p));
else if (existsSync(join(ROOT, ".git"))) {
  const tracked = git(["--", "Cargo.toml", "*/Cargo.toml"]);
  const others = git(["--others", "--exclude-standard", "--", "Cargo.toml", "*/Cargo.toml"]);
  manifestFiles = [...new Set([...tracked, ...others])].map(p => resolve(ROOT, p));
} else {
  const walk = (dir: string, out: string[]) => { for (const e of readdirSync(dir, {withFileTypes: true})) { if (e.isDirectory()) walk(join(dir, e.name), out); else if (e.name === "Cargo.toml") out.push(join(dir, e.name)); } };
  manifestFiles = []; walk(ROOT, manifestFiles);
}
manifestFiles = manifestFiles.filter(p => existsSync(p) && !rel(p).includes("ctc-tools/"));

const wsCache = new Map<string, string | undefined>();
const wsOf = (dir: string): string | undefined => {
  if (wsCache.has(dir)) return wsCache.get(dir);
  let result: string | undefined;
  let cur = dir;
  while (true) {
    const m = join(cur, "Cargo.toml");
    if (existsSync(m) && toml(m).workspace) { result = cur; break; }
    const up = dirname(cur);
    if (up === cur || !up.startsWith(ROOT)) break;
    cur = up;
  }
  wsCache.set(dir, result);
  return result;
};

const crates = new Map<string, Crate>();
for (const manifest of manifestFiles) {
  const parsed = toml(manifest);
  if (!parsed.package) continue;
  const dir = dirname(manifest);
  const name = parsed.package.name ?? basename(dir);
  const libRoot = parsed.lib?.path ? resolve(dir, parsed.lib.path) : join(dir, "src/lib.rs");
  const hasLib = existsSync(libRoot) && parsed.lib !== undefined || existsSync(libRoot);
  crates.set(dir, {dir, name, lib: hasLib ? (parsed.lib?.name ?? name.replaceAll("-", "_")) : undefined, libRoot: hasLib ? libRoot : undefined, deps: new Map(), targets: [], ws: wsOf(dir)});
}

const listRs = (dir: string): {name: string; path: string}[] => {
  if (!existsSync(dir)) return [];
  const out: {name: string; path: string}[] = [];
  for (const e of readdirSync(dir, {withFileTypes: true})) {
    if (e.isFile() && e.name.endsWith(".rs")) out.push({name: basename(e.name, ".rs"), path: join(dir, e.name)});
    else if (e.isDirectory() && existsSync(join(dir, e.name, "main.rs"))) out.push({name: e.name, path: join(dir, e.name, "main.rs")});
  }
  return out;
};

for (const c of crates.values()) {
  const parsed = toml(join(c.dir, "Cargo.toml"));
  const seen = new Set<string>();
  const add = (t: Target) => { if (!seen.has(t.root) && existsSync(t.root)) { seen.add(t.root); c.targets.push(t); } };
  if (c.libRoot) add({kind: "lib", name: c.lib!, root: c.libRoot});
  for (const b of parsed.bin ?? []) add({kind: "bin", name: b.name, root: resolve(c.dir, b.path ?? `src/bin/${b.name}.rs`), req: b["required-features"]});
  if (parsed.package.autobins !== false) { add({kind: "bin", name: c.name, root: join(c.dir, "src/main.rs")}); for (const f of listRs(join(c.dir, "src/bin"))) add({kind: "bin", name: f.name, root: f.path}); }
  for (const t of parsed.test ?? []) add({kind: "test", name: t.name, root: resolve(c.dir, t.path ?? `tests/${t.name}.rs`), req: t["required-features"]});
  if (parsed.package.autotests !== false) for (const f of listRs(join(c.dir, "tests"))) add({kind: "test", name: f.name, root: f.path});
  for (const t of parsed.bench ?? []) add({kind: "bench", name: t.name, root: resolve(c.dir, t.path ?? `benches/${t.name}.rs`), req: t["required-features"]});
  if (parsed.package.autobenches !== false) for (const f of listRs(join(c.dir, "benches"))) add({kind: "bench", name: f.name, root: f.path});
  for (const t of parsed.example ?? []) add({kind: "example", name: t.name, root: resolve(c.dir, t.path ?? `examples/${t.name}.rs`), req: t["required-features"]});
  if (parsed.package.autoexamples !== false) for (const f of listRs(join(c.dir, "examples"))) add({kind: "example", name: f.name, root: f.path});
  if (parsed.package.build !== false && parsed.package.build !== undefined) add({kind: "build", name: "build", root: resolve(c.dir, String(parsed.package.build))});
  else if (existsSync(join(c.dir, "build.rs"))) add({kind: "build", name: "build", root: join(c.dir, "build.rs")});
}

for (const c of crates.values()) {
  const parsed = toml(join(c.dir, "Cargo.toml"));
  const wsDeps: Record<string, any> = c.ws ? (toml(join(c.ws, "Cargo.toml")).workspace?.dependencies ?? {}) : {};
  const tables: any[] = [parsed.dependencies, parsed["dev-dependencies"], parsed["build-dependencies"]];
  for (const t of Object.values(parsed.target ?? {}) as any[]) tables.push(t.dependencies, t["dev-dependencies"], t["build-dependencies"]);
  for (const table of tables) for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
    const spec = typeof raw === "string" ? {} : raw;
    const inherited = spec.workspace === true && c.ws !== undefined;
    const base = inherited ? (typeof wsDeps[key] === "string" ? {} : (wsDeps[key] ?? {})) : spec;
    if (typeof base.path !== "string") continue;
    const depDir = resolve(inherited ? c.ws! : c.dir, base.path);
    const target = crates.get(depDir);
    if (!target?.lib) continue;
    const renamed = base.package !== undefined || spec.package !== undefined;
    const ext = renamed ? key.replaceAll("-", "_") : target.lib;
    const features = new Set<string>([...(spec.features ?? []), ...(inherited ? (base.features ?? []) : [])]);
    const prior = c.deps.get(ext);
    if (prior) for (const f of features) prior.features.add(f); else c.deps.set(ext, {dir: depDir, features});
  }
}

const recs: Rec[] = [];
const ungatedNames = new Map<string, Set<string>>();
const ungatedMethods = new Map<string, Set<string>>();
const ungatedPaths = new Map<string, Set<string>>();
const globReexport = new Set<string>();
const fileFacts: FileFacts[] = [];
const declared = new Map<string, Set<string>>();
const modNames = new Map<string, Set<string>>();
const aliasRoots = new Map<string, Map<string, Set<string>>>();
const noteAlias = (crate: string, alias: string, root: string) => { let m = aliasRoots.get(crate); if (!m) aliasRoots.set(crate, m = new Map()); let r = m.get(alias); if (!r) m.set(alias, r = new Set()); r.add(root); };
let PHASE = 1;
type Cand = {names: Map<string, Rec[]>; methods: Map<string, Rec[]>};
const candByCrate = new Map<string, Cand>();
type Event = {head: string; strength: string; lib: boolean; crate: string; file: string; line: number; key: string; method: boolean};
const events: Event[] = [];
const missingMods: string[] = [];
const KEYWORDS = new Set(["fn", "let", "mut", "struct", "enum", "const", "static", "type", "mod", "trait", "impl", "use", "as", "for", "in", "if", "else", "match", "return", "where", "pub", "crate", "self", "super", "ref", "move", "unsafe", "async", "await", "loop", "while", "dyn", "true", "false"]);

const joinGate = (a: Gate, b: Cfg[]): Gate => {
  let cur = a;
  for (const c of b) {
    if (c.gated) cur = {kind: "test", cfgs: [...(cur?.cfgs ?? []), c]};
    else if (c.partial && (!cur || cur.kind === "partial")) cur = {kind: "partial", cfgs: [...(cur?.cfgs ?? []), c]};
  }
  return cur;
};

const useTree = (toks: Tok[], start: number, prefix: string[], out: UseLeaf[]): number => {
  let i = start;
  const path = [...prefix];
  if (toks[i]?.t === "::") i++;
  while (i < toks.length) {
    const t = toks[i]!;
    if (t.t === "{") {
      i++;
      while (i < toks.length && toks[i]!.t !== "}") { i = useTree(toks, i, path, out); if (toks[i]?.t === ",") i++; }
      return i + 1;
    }
    if (t.t === "*") { out.push({path, glob: true, line: t.line}); return i + 1; }
    path.push(t.t); i++;
    if (toks[i]?.t === "::") { i++; continue; }
    let alias: string | undefined;
    if (toks[i]?.t === "as") { alias = toks[i + 1]?.t; i += 2; }
    out.push({path: path.slice(), alias, glob: false, line: t.line});
    return i;
  }
  return i;
};

const normaliseLeaf = (leaf: UseLeaf): UseLeaf => {
  if (!leaf.glob && leaf.path[leaf.path.length - 1] === "self") {
    const path = leaf.path.slice(0, -1);
    return {...leaf, path, alias: leaf.alias ?? path[path.length - 1]};
  }
  return leaf;
};

type Walk = {hiddenMod: boolean; c: Crate; target: Target; externs: Map<string, string>; owner: boolean; file: string; dir: string; inline: number; modpath: string[]; gate: Gate};

const skipHeader = (toks: Tok[], i: number, end: number): number => {
  let j = i;
  while (j < end) {
    const t = toks[j]!;
    if (t.t === "{") return matchClose(toks, j) + 1;
    if (t.t === ";" && t.k === "p") return j + 1;
    if (isOpen(t)) { j = matchClose(toks, j) + 1; continue; }
    j++;
  }
  return end;
};

const skipToSemi = (toks: Tok[], i: number, end: number): number => {
  let j = i;
  while (j < end) {
    const t = toks[j]!;
    if (t.k === "p" && t.t === ";") return j + 1;
    if (isOpen(t)) { j = matchClose(toks, j) + 1; continue; }
    j++;
  }
  return end;
};

const visited = new Set<string>();

const processFile = (w: Walk, isRoot: boolean): void => {
  let src: string;
  try { src = readFileSync(w.file, "utf8"); } catch { missingMods.push(`${rel(w.file)} (declared in ${w.c.name})`); return; }
  const toks = lex(src);
  const ff: FileFacts = {types: new Set(), crate: w.c.dir, target: `${w.target.kind}:${w.target.name}`, file: w.file, facts: [], bare: new Set(), methods: new Set(), localDefs: new Set(), roots: new Set()};
  if (PHASE === 2) {
    scanIndirect(toks, w);
    walkItemsLight(toks, 0, toks.length, w);
    return;
  }
  extractConsumer(toks, w, ff);
  if (ff.facts.length) fileFacts.push(ff);
  if (w.owner) walkItems(toks, 0, toks.length, w);
  else walkItemsLight(toks, 0, toks.length, w);
  void isRoot;
};

const attrOf = (toks: Tok[], i: number): {end: number; inner: boolean; cfg?: Cfg; path?: string; macroExport?: boolean; testAttr?: boolean} => {
  let j = i + 1;
  const inner = toks[j]?.t === "!";
  if (inner) j++;
  if (toks[j]?.t !== "[") return {end: i + 1, inner};
  const close = matchClose(toks, j);
  const first = toks[j + 1]?.t;
  const res: ReturnType<typeof attrOf> = {end: close + 1, inner};
  if (first === "cfg" && toks[j + 2]?.t === "(") res.cfg = parseCfg(toks.slice(j + 3, matchClose(toks, j + 2)));
  else if (first === "path" && toks[j + 2]?.t === "=" && toks[j + 3]?.k === "s") res.path = toks[j + 3]!.t;
  else if (first === "macro_export") res.macroExport = true;
  else if (first === "test" && close === j + 2) res.testAttr = true;
  else if (toks[close - 1]?.t === "test" && toks[close - 2]?.t === "::") res.testAttr = true;
  return res;
};

const note = (w: Walk, rec: Omit<Rec, "crate" | "file" | "hidden">) => {
  recs.push({...rec, hidden: w.hiddenMod, crate: w.c.dir, file: w.file});
};

const addUngated = (map: Map<string, Set<string>>, dir: string, name: string) => {
  let s = map.get(dir);
  if (!s) map.set(dir, s = new Set());
  s.add(name);
};

const resolveMod = (w: Walk, name: string, pathAttr: string | undefined): {file: string; dir: string} | undefined => {
  if (pathAttr !== undefined) {
    const base = w.inline === 0 ? dirname(w.file) : w.dir;
    const file = resolve(base, pathAttr);
    return {file, dir: dirname(file)};
  }
  const a = join(w.dir, name + ".rs"), b = join(w.dir, name, "mod.rs");
  if (existsSync(a)) return {file: a, dir: join(w.dir, name)};
  if (existsSync(b)) return {file: b, dir: join(w.dir, name)};
  return undefined;
};

const walkItemsLight = (toks: Tok[], from: number, end: number, w: Walk) => walkItems(toks, from, end, w, true);

const pubUseHeads = (toks: Tok[], from: number, end: number): Set<string> => {
  const heads = new Set<string>();
  for (let i = from; i < end; i++) {
    const t = toks[i]!;
    if (t.k === "i" && t.t === "pub" && toks[i + 1]?.t === "use") {
      const leaves: UseLeaf[] = [];
      const close = useTree(toks, i + 2, [], leaves);
      for (const l of leaves) { const p = l.path[0] === "self" ? l.path[1] : l.path[0]; if (p) heads.add(p); }
      i = Math.max(i, close - 1);
    }
  }
  return heads;
};

const walkItems = (toks: Tok[], from: number, end: number, w: Walk, light = false): void => {
  let i = from;
  let gate = w.gate;
  const heads = pubUseHeads(toks, from, end);
  while (i < end) {
    const itemStart = i;
    const own: Cfg[] = [];
    let pathAttr: string | undefined, macroExport = false, testAttr = false;
    while (toks[i]?.t === "#" && toks[i]!.k === "p") {
      const a = attrOf(toks, i);
      if (a.end === i + 1) { i++; break; }
      if (a.inner) { if (a.cfg) gate = joinGate(gate, [a.cfg]); }
      else {
        if (a.cfg) own.push(a.cfg);
        if (a.path !== undefined) pathAttr = a.path;
        if (a.macroExport) macroExport = true;
        if (a.testAttr) testAttr = true;
      }
      i = a.end;
    }
    if (i >= end) break;
    let eff = joinGate(gate, own);
    if (testAttr) eff = {kind: "test", cfgs: [...(eff?.cfgs ?? [])]};
    let vis: "pub" | "restricted" | "priv" = "priv";
    if (toks[i]?.t === "pub" && toks[i]!.k === "i") {
      vis = "pub"; i++;
      if (toks[i]?.t === "(") { vis = "restricted"; i = matchClose(toks, i) + 1; }
    }
    const line = toks[i]?.line ?? 0;
    let q = i;
    const QUAL = new Set(["const", "async", "unsafe", "default", "extern"]);
    while (q < end && toks[q]!.k === "i" && QUAL.has(toks[q]!.t)) {
      if (toks[q]!.t === "extern") { if (toks[q + 1]?.t === "crate") break; q++; if (toks[q]?.k === "s") q++; if (toks[q]?.t === "{") break; continue; }
      if (toks[q]!.t === "const" && !["fn", "unsafe", "async", "extern"].includes(toks[q + 1]?.t ?? "")) break;
      q++;
    }
    const kw = toks[q]?.k === "i" ? toks[q]!.t : "";
    const name = toks[q + 1]?.k === "i" ? toks[q + 1]!.t : "";
    if (PHASE === 1 && kw === "mod" && name) { let m = modNames.get(w.c.dir); if (!m) modNames.set(w.c.dir, m = new Set()); m.add(name); }
    if (PHASE === 1 && name && ["mod", "fn", "struct", "enum", "trait", "type", "const", "static", "union"].includes(kw)) addUngated(declared, w.c.dir, name === "mut" ? (toks[q + 2]?.t ?? "") : name);
    if (PHASE === 1 && kw === "macro_rules" && toks[q + 2]?.k === "i") addUngated(declared, w.c.dir, toks[q + 2]!.t);
    const record = (kind: string, nm: string, owner?: string) => {
      if (eff) { if (!light && (vis === "pub")) note(w, {line, kind, name: nm, modpath: w.modpath, owner, gate: eff, features: eff.cfgs.flatMap(c => c.features)}); }
      else if (vis === "pub" || kind === "macro") { if (!light) { addUngated(owner ? ungatedMethods : ungatedNames, w.c.dir, nm); if (!owner) addUngated(ungatedPaths, w.c.dir, [...w.modpath, nm].join("::")); } }
    };
    if (kw === "mod" && name) {
      if (toks[q + 2]?.t === ";") {
        record("mod", name);
        const r = resolveMod(w, name, pathAttr);
        if (!r) { missingMods.push(`mod ${name} in ${rel(w.file)} (${w.c.name})`); i = q + 3; continue; }
        const key = w.c.dir + "|" + w.target.name + "|" + r.file;
        if (!visited.has(key)) { visited.add(key); processFile({...w, file: r.file, dir: r.dir, inline: 0, modpath: [...w.modpath, name], gate: eff, hiddenMod: w.hiddenMod || (vis !== "pub" && !heads.has(name))}, false); }
        i = q + 3;
      } else if (toks[q + 2]?.t === "{") {
        record("mod", name);
        const close = matchClose(toks, q + 2);
        walkItems(toks, q + 3, close, {...w, dir: pathAttr !== undefined ? resolve(w.inline === 0 ? dirname(w.file) : w.dir, pathAttr) : join(w.dir, name), inline: w.inline + 1, modpath: [...w.modpath, name], gate: eff, hiddenMod: w.hiddenMod || (vis !== "pub" && !heads.has(name))}, light);
        i = close + 1;
      } else i = q + 2;
      continue;
    }
    if (kw === "fn" && name) { record("fn", name); i = skipHeader(toks, q + 2, end); continue; }
    if ((kw === "struct" || kw === "enum" || kw === "trait") && name) { record(kw, name); i = skipHeader(toks, q + 2, end); continue; }
    if (kw === "union" && name && (toks[q + 2]?.t === "{" || toks[q + 2]?.t === "<")) { record("union", name); i = skipHeader(toks, q + 2, end); continue; }
    if (kw === "type" && name) { record("type", name); i = skipToSemi(toks, q + 2, end); continue; }
    if ((kw === "const" || kw === "static") && name) {
      const nm = name === "mut" ? (toks[q + 2]?.t ?? "") : name;
      record(kw, nm); i = skipToSemi(toks, q + 2, end); continue;
    }
    if (kw === "use") {
      const leaves: UseLeaf[] = [];
      const close = useTree(toks, q + 1, [], leaves);
      for (const raw of leaves) {
        const leaf = normaliseLeaf(raw);
        if (leaf.glob) { if (!eff && vis === "pub" && !light) globReexport.add(w.c.dir); continue; }
        const nm = leaf.alias ?? leaf.path[leaf.path.length - 1] ?? "";
        if (nm && nm !== "_") record("use", nm);
      }
      i = skipToSemi(toks, close, end);
      continue;
    }
    if (kw === "impl") {
      let j = q + 1;
      if (toks[j]?.t === "<") { let d = 0; while (j < end) { if (toks[j]!.t === "<") d++; else if (toks[j]!.t === ">") { d--; if (d === 0) { j++; break; } } j++; } }
      const hdrStart = j;
      while (j < end && toks[j]!.t !== "{" && toks[j]!.t !== ";") { if (isOpen(toks[j]!) && toks[j]!.t !== "{") j = matchClose(toks, j); j++; }
      let forAt = -1;
      for (let k = hdrStart; k < j; k++) if (toks[k]!.t === "for" && toks[k]!.k === "i") forAt = k;
      let ty = "";
      for (let k = forAt >= 0 ? forAt + 1 : hdrStart; k < j; k++) { const t = toks[k]!; if (t.k === "i" && !KEYWORDS.has(t.t)) { ty = t.t; if (toks[k + 1]?.t === "::") continue; break; } }
      if (toks[j]?.t === "{") {
        const close = matchClose(toks, j);
        if (!light || true) walkImpl(toks, j + 1, close, w, eff, ty, forAt >= 0, light);
        i = close + 1;
      } else i = j + 1;
      continue;
    }
    if (kw === "macro_rules" && toks[q + 1]?.t === "!" && toks[q + 2]?.k === "i") {
      const nm = toks[q + 2]!.t;
      const open = q + 3;
      if (macroExport) { if (eff) { if (!light) note(w, {line, kind: "macro", name: nm, modpath: [], gate: eff, features: eff.cfgs.flatMap(c => c.features)}); } else if (!light) addUngated(ungatedNames, w.c.dir, nm); }
      i = isOpen(toks[open]!) ? matchClose(toks, open) + 1 : open;
      if (toks[i]?.t === ";") i++;
      continue;
    }
    if (kw === "extern" && toks[q + 1]?.t === "crate") { i = skipToSemi(toks, q, end); continue; }
    if (kw === "extern" || (toks[q]?.t === "{" )) { i = toks[q]?.t === "{" ? matchClose(toks, q) + 1 : q + 1; continue; }
    if (toks[q]?.k === "i" && toks[q + 1]?.t === "!" ) {
      if (toks[q]!.t === "include" && toks[q + 2]?.t === "(" && toks[q + 3]?.k === "s" && toks[q + 3]!.t.endsWith(".rs")) {
        const target = resolve(dirname(w.file), toks[q + 3]!.t);
        const key = w.c.dir + "|" + w.target.name + "|inc|" + target + "|" + w.modpath.join("::");
        if (!visited.has(key)) { visited.add(key); processFile({...w, file: target, gate: eff}, false); }
      }
      const open = q + 2;
      i = open < end && isOpen(toks[open]!) ? matchClose(toks, open) + 1 : q + 2;
      if (toks[i]?.t === ";") i++;
      continue;
    }
    if (toks[q]?.k === "i" && toks[q + 1]?.t === "::") { i = skipToSemi(toks, q, end); continue; }
    i = Math.max(q + 1, itemStart + 1);
  }
};

const walkImpl = (toks: Tok[], from: number, end: number, w: Walk, implGate: Gate, ty: string, isTrait: boolean, light: boolean): void => {
  let i = from;
  while (i < end) {
    const own: Cfg[] = [];
    let testAttr = false;
    while (toks[i]?.t === "#" && toks[i]!.k === "p") {
      const a = attrOf(toks, i);
      if (a.end === i + 1) { i++; break; }
      if (a.cfg && !a.inner) own.push(a.cfg);
      if (a.testAttr) testAttr = true;
      i = a.end;
    }
    if (i >= end) break;
    let eff = joinGate(implGate, own);
    if (testAttr) eff = {kind: "test", cfgs: [...(eff?.cfgs ?? [])]};
    let vis = "priv";
    if (toks[i]?.t === "pub") { vis = "pub"; i++; if (toks[i]?.t === "(") { vis = "restricted"; i = matchClose(toks, i) + 1; } }
    let q = i;
    while (q < end && ["const", "async", "unsafe", "default", "extern"].includes(toks[q]!.t) && toks[q]!.k === "i") {
      if (toks[q]!.t === "const" && !["fn", "unsafe", "async", "extern"].includes(toks[q + 1]?.t ?? "")) break;
      q++;
      if (toks[q]?.k === "s") q++;
    }
    const kw = toks[q]?.t, name = toks[q + 1]?.t ?? "";
    const line = toks[q]?.line ?? 0;
    if ((kw === "fn" || kw === "const" || kw === "type") && name) {
      if (PHASE === 1) addUngated(declared, w.c.dir, name);
      const isPub = vis === "pub" || isTrait;
      if (eff && isPub && !isTrait && !light) note(w, {line, kind: kw === "fn" ? "method" : "assoc-" + kw, name, modpath: w.modpath, owner: ty, gate: eff, features: eff.cfgs.flatMap(c => c.features)});
      else if (!eff && !light) { addUngated(ungatedMethods, w.c.dir, name); addUngated(ungatedMethods, w.c.dir, ty + "::" + name); }
      i = kw === "fn" ? skipHeader(toks, q + 2, end) : skipToSemi(toks, q + 2, end);
      continue;
    }
    i = Math.max(q + 1, i + 1);
  }
};

const extractConsumer = (toks: Tok[], w: Walk, ff: FileFacts): void => {
  if (w.externs.size === 0) return;
  const aliases = new Map<string, string[]>();
  const leaves: UseLeaf[] = [];
  const useSpans: [number, number][] = [];
  for (let i = 0; i < toks.length; i++) {
    const t = toks[i]!;
    if (t.k === "i" && t.t === "use" && toks[i - 1]?.t !== "r#") {
      const start = i + 1;
      const before: UseLeaf[] = [];
      const close = useTree(toks, start, [], before);
      for (const raw of before) leaves.push(normaliseLeaf(raw));
      useSpans.push([i, close]);
      i = Math.max(i, close - 1);
    }
  }
  const externOf = (seg: string | undefined): string | undefined => {
    if (seg === undefined) return undefined;
    if (w.externs.has(seg)) return seg;
    return undefined;
  };
  for (const leaf of leaves) {
    if (!leaf.glob) { const nm = leaf.alias ?? leaf.path[leaf.path.length - 1]; if (nm) { ff.localDefs.add(nm); noteAlias(w.c.dir, nm, leaf.path[0] === "" || leaf.path[0] === "::" ? leaf.path[1] ?? "" : leaf.path[0] ?? ""); } }
    let p = leaf.path;
    if (p[0] === "::" || p[0] === "") p = p.slice(1);
    const ext = externOf(p[0]);
    if (!ext) continue;
    if (leaf.glob) ff.facts.push({crate: w.c.dir, target: ff.target, file: w.file, ext, rest: p.slice(1), line: leaf.line, via: "glob", mac: false});
    else {
      ff.facts.push({crate: w.c.dir, target: ff.target, file: w.file, ext, rest: p.slice(1), line: leaf.line, via: "use", mac: false});
      const nm = leaf.alias ?? p[p.length - 1];
      if (nm && nm !== "_") aliases.set(nm, p);
    }
  }
  const isLeafStart = (j: number) => { for (const [a, b] of useSpans) if (j >= a && j < b) return true; return false; };
  const spanStarts = new Map<number, number>();
  for (const [a, b] of useSpans) spanStarts.set(a, b);
  let hasGlob = ff.facts.some(f => f.via === "glob");
  for (let i = 0; i < toks.length; i++) {
    const skipTo = spanStarts.get(i);
    if (skipTo !== undefined) { i = skipTo - 1; continue; }
    const t = toks[i]!;
    if (t.k !== "i") continue;
    const c0 = t.t.charCodeAt(0);
    if (c0 >= 65 && c0 <= 90) ff.types.add(t.t);
    const prev = toks[i - 1];
    if (prev?.t === "." ) {
      if (toks[i + 1]?.t === "(" || (toks[i + 1]?.t === "::" && toks[i + 2]?.t === "<")) ff.methods.add(t.t);
      continue;
    }
    if (prev?.t === "::" && (toks[i - 2]?.k === "i" || toks[i - 2]?.t === ">")) continue;
    if (!KEYWORDS.has(t.t) || t.t === "crate" || t.t === "self" || t.t === "super") {
      const segs: string[] = [t.t];
      let j = i + 1;
      while (toks[j]?.t === "::" && toks[j + 1]?.k === "i") { segs.push(toks[j + 1]!.t); j += 2; }
      const head = segs[0]!;
      const mac = toks[j]?.t === "!" && toks[j + 1]?.t !== "=";
      if (segs.length >= 2 || (mac && segs.length >= 1)) {
        let full: string[] | undefined;
        if (externOf(head)) full = segs;
        else if (aliases.has(head)) full = [...aliases.get(head)!, ...segs.slice(1)];
        if (full && externOf(full[0])) ff.facts.push({crate: w.c.dir, target: ff.target, file: w.file, ext: full[0]!, rest: full.slice(1), line: t.line, via: "path", mac});
      }
      if (hasGlob && segs.length === 1 && !KEYWORDS.has(head)) {
        const nx = toks[i + 1];
        const pv = prev?.t ?? "";
        const noBare = ["fn", "let", "mut", "struct", "enum", "const", "static", "type", "mod", "trait", "|", "ref"];
        if (!noBare.includes(pv) && nx?.t !== ":") ff.bare.add(head);
        if (["fn", "struct", "enum", "const", "static", "type", "trait"].includes(pv)) ff.localDefs.add(head);
      }
      i = Math.max(i, j - 1);
    }
  }
  void isLeafStart;
};

const externsOf = (c: Crate, target: Target): Map<string, string> => {
  const m = new Map<string, string>();
  for (const [ext, d] of c.deps) m.set(ext, d.dir);
  if (target.kind !== "lib" && c.lib) m.set(c.lib, c.dir);
  return m;
};

const scanIndirect = (toks: Tok[], w: Walk): void => {
  const cand = candByCrate.get(w.c.dir);
  if (!cand) return;
  const types = new Set<string>(), calls = new Set<string>();
  const declKw = new Set(["fn", "let", "mut", "struct", "enum", "const", "static", "type", "mod", "trait", "ref", "union"]);
  const useStart = new Map<number, number>();
  let hasGlob = false;
  const push = (line: number, key: string, method: boolean, strength: string, head = "") => events.push({head, lib: w.target.kind === "lib", crate: w.c.dir, file: w.file, line, key, method, strength});
  for (let i = 0; i < toks.length; i++) {
    const t = toks[i]!;
    if (t.k === "i" && t.t === "use") {
      const leaves: UseLeaf[] = [];
      const close = useTree(toks, i + 1, [], leaves);
      for (const raw of leaves) {
        const leaf = normaliseLeaf(raw);
        if (leaf.glob) { hasGlob = true; continue; }
        const tail = leaf.path[leaf.path.length - 1];
        if (tail && cand.names.has(tail)) push(leaf.line, tail, false, "use", leaf.path[0] ?? "");
      }
      useStart.set(i, close);
      i = Math.max(i, close - 1);
    }
  }
  for (let i = 0; i < toks.length; i++) {
    const to = useStart.get(i);
    if (to !== undefined) { i = to - 1; continue; }
    const t = toks[i]!;
    if (t.k !== "i") continue;
    const prev = toks[i - 1]?.t, next = toks[i + 1]?.t;
    const c0 = t.t.charCodeAt(0);
    if (c0 >= 65 && c0 <= 90) types.add(t.t);
    if (prev === ".") { if (next === "(") calls.add(t.t); continue; }
    if (cand.names.has(t.t) && !(prev && declKw.has(prev)) && !(prev === "!" && toks[i - 2]?.t === "macro_rules") && next !== ":") {
      if (prev === "::") { let j = i; while (toks[j - 1]?.t === "::" && toks[j - 2]?.k === "i") j -= 2; push(t.line, t.t, false, "qualified", toks[j]!.t); }
      else if (hasGlob && (next === "(" || next === "!" || next === "::" || (c0 >= 65 && c0 <= 90))) push(t.line, t.t, false, "glob");
    }
    if (next === "::" && toks[i + 2]?.k === "i") { const k = t.t + "::" + toks[i + 2]!.t; if (cand.methods.has(k)) { let j = i; while (toks[j - 1]?.t === "::" && toks[j - 2]?.k === "i") j -= 2; push(t.line, k, true, "qualified", toks[j]!.t); } }
  }
  for (const k of cand.methods.keys()) { const [ty, m] = k.split("::") as [string, string]; if (calls.has(m) && types.has(ty)) push(0, k, true, "call"); }
};

const selected = [...crates.values()].filter(c => ONLY.length === 0 || ONLY.includes(c.name));
let done = 0;
for (const c of selected) {
  for (const target of c.targets) {
    if (target.kind === "build") continue;
    const externs = externsOf(c, target);
    const w: Walk = {hiddenMod: false, c, target, externs, owner: target.kind === "lib", file: target.root, dir: dirname(target.root), inline: 0, modpath: [], gate: null};
    const key = c.dir + "|" + target.name + "|" + target.root;
    if (visited.has(key)) continue;
    visited.add(key);
    processFile(w, true);
  }
  done++;
}

const crateByKey = new Map<string, Map<string, string>>();
for (const c of crates.values()) {
  const parsed = toml(join(c.dir, "Cargo.toml"));
  const wsDeps: Record<string, any> = c.ws ? (toml(join(c.ws, "Cargo.toml")).workspace?.dependencies ?? {}) : {};
  const tables: any[] = [parsed.dependencies, parsed["dev-dependencies"], parsed["build-dependencies"]];
  for (const t of Object.values(parsed.target ?? {}) as any[]) tables.push(t.dependencies, t["dev-dependencies"], t["build-dependencies"]);
  const m = new Map<string, string>();
  for (const table of tables) for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
    const spec = typeof raw === "string" ? {} : raw;
    const inherited = spec.workspace === true && c.ws !== undefined;
    const base = inherited ? (typeof wsDeps[key] === "string" ? {} : (wsDeps[key] ?? {})) : spec;
    if (typeof base.path === "string") m.set(key, resolve(inherited ? c.ws! : c.dir, base.path));
  }
  crateByKey.set(c.dir, m);
}
const enabledCache = new Map<string, Map<string, Set<string>>>();
const enabledOn = (consumer: Crate): Map<string, Set<string>> => {
  const hit = enabledCache.get(consumer.dir);
  if (hit) return hit;
  const E = new Map<string, Set<string>>();
  const add = (dir: string, f: string): boolean => { let s = E.get(dir); if (!s) E.set(dir, s = new Set()); if (s.has(f)) return false; s.add(f); return true; };
  for (const d of consumer.deps.values()) for (const f of d.features) add(d.dir, f);
  const forwards = (c: Crate, feats: Iterable<string>): [string, string][] => {
    const table: Record<string, string[]> = toml(join(c.dir, "Cargo.toml")).features ?? {};
    const out: [string, string][] = [];
    const seen = new Set<string>();
    const queue = [...feats, ...(table.default ?? [])];
    while (queue.length) {
      const f = queue.pop()!;
      if (seen.has(f)) continue;
      seen.add(f);
      for (const item of table[f] ?? []) {
        if (item.startsWith("dep:")) continue;
        const slash = item.indexOf("/");
        if (slash >= 0) { const key = item.slice(0, slash).replace(/\?$/u, ""); const dir = crateByKey.get(c.dir)?.get(key); if (dir) out.push([dir, item.slice(slash + 1)]); }
        else queue.push(item);
      }
    }
    return out;
  };
  for (const [d, f] of forwards(consumer, Object.keys(toml(join(consumer.dir, "Cargo.toml")).features ?? {}))) add(d, f);
  let changed = true, rounds = 0;
  while (changed && rounds++ < 12) {
    changed = false;
    for (const [dir, feats] of [...E]) {
      const c = crates.get(dir);
      if (!c) continue;
      for (const [d, f] of forwards(c, feats)) if (add(d, f)) changed = true;
    }
  }
  enabledCache.set(consumer.dir, E);
  return E;
};

const featureClosure = (owner: Crate, start: Set<string>): Set<string> => {
  const table: Record<string, string[]> = toml(join(owner.dir, "Cargo.toml")).features ?? {};
  const out = new Set<string>();
  const queue = [...start, ...(table.default ?? [])];
  while (queue.length) {
    const f = queue.pop()!;
    if (out.has(f) || f.startsWith("dep:") || f.includes("/")) continue;
    out.add(f);
    queue.push(...(table[f] ?? []));
  }
  return out;
};

const enabledFor = (consumer: Crate, ownerDir: string, rec: Rec): boolean => {
  const eff = enabledOn(consumer).get(ownerDir);
  return rec.gate.kind === "partial" && rec.features.length > 0 && !!eff && rec.features.some(feat => featureClosure(crates.get(ownerDir)!, eff).has(feat));
};

const recsByCrate = new Map<string, Rec[]>();
for (const r of recs) { let a = recsByCrate.get(r.crate); if (!a) recsByCrate.set(r.crate, a = []); a.push(r); }
const typeIsGated = new Map<string, Set<string>>();
for (const r of recs) if (!r.owner && ["struct", "enum", "union", "type", "trait"].includes(r.kind)) { let s = typeIsGated.get(r.crate); if (!s) typeIsGated.set(r.crate, s = new Set()); s.add(r.name); }

const facing = new Map<string, Set<string>>();
for (const ff of fileFacts) {
  const c = crates.get(ff.crate)!;
  const target = c.targets.find(t => `${t.kind}:${t.name}` === ff.target)!;
  const ex = externsOf(c, target);
  for (const f of ff.facts) { const o = ex.get(f.ext); if (o && recsByCrate.has(o)) { let a = facing.get(ff.crate); if (!a) facing.set(ff.crate, a = new Set()); a.add(o); } }
}
for (const [cDir, owners] of facing) {
  const cand: Cand = {names: new Map(), methods: new Map()};
  const decl = declared.get(cDir) ?? new Set();
  for (const o of owners) for (const rec of recsByCrate.get(o)!) {
    if (rec.hidden) continue;
    if (rec.owner) {
      if (typeIsGated.get(o)?.has(rec.owner) || ungatedMethods.get(o)?.has(rec.owner + "::" + rec.name)) continue;
      const k = rec.owner + "::" + rec.name;
      let a = cand.methods.get(k); if (!a) cand.methods.set(k, a = []); a.push(rec);
    } else {
      if (ungatedNames.get(o)?.has(rec.name) || decl.has(rec.name)) continue;
      if (rec.gate.kind === "test" && (recsByCrate.get(o) ?? []).some(r => !r.owner && r.name === rec.name && r.gate.kind === "partial" && enabledFor(crates.get(cDir)!, o, r))) continue;
      let a = cand.names.get(rec.name); if (!a) cand.names.set(rec.name, a = []); a.push(rec);
    }
  }
  if (cand.names.size || cand.methods.size) candByCrate.set(cDir, cand);
}
PHASE = 2;
visited.clear();
for (const c of selected) {
  if (!candByCrate.has(c.dir)) continue;
  for (const target of c.targets) {
    if (target.kind === "build") continue;
    const w: Walk = {hiddenMod: false, c, target, externs: externsOf(c, target), owner: false, file: target.root, dir: dirname(target.root), inline: 0, modpath: [], gate: null};
    const key = c.dir + "|" + target.name + "|" + target.root;
    if (visited.has(key)) continue;
    visited.add(key);
    processFile(w, true);
  }
}

type Hit = {consumer: string; consumerCrate: string; target: string; file: string; line: number; owner: string; ownerCrate: string; class: string; item: string; declFile: string; declLine: number; gateRaw: string; sanctioned: boolean; features: string[]};
const hits: Hit[] = [];
const seenHit = new Set<string>();
const gatedOnlyName = (dir: string, name: string) => !ungatedNames.get(dir)?.has(name) && !globReexport.has(dir);
const gatedOnlyMethod = (dir: string, name: string) => !ungatedMethods.get(dir)?.has(name) && !ungatedNames.get(dir)?.has(name);

const push = (ff: FileFacts, f: Fact | undefined, ownerDir: string, rec: Rec, cls: string, line: number) => {
  const consumerCrate = crates.get(ff.crate)!;
  const owner = crates.get(ownerDir)!;
  const dep = ff.crate === ownerDir ? undefined : consumerCrate.deps.get(f?.ext ?? "");
  const eff0 = enabledOn(consumerCrate).get(ownerDir);
  const tgt = consumerCrate.targets.find(t => `${t.kind}:${t.name}` === ff.target);
  const eff = ff.crate === ownerDir && tgt?.req ? new Set([...(eff0 ?? []), ...tgt.req]) : eff0;
  const enabled = rec.gate.kind === "partial" && rec.features.length > 0 && !!eff && rec.features.some(feat => featureClosure(owner, eff).has(feat));
  void dep;
  const key = `${ff.file}|${line}|${rec.file}|${rec.line}|${rec.name}|${cls}`;
  if (seenHit.has(key)) return;
  seenHit.add(key);
  hits.push({consumer: rel(ff.file), consumerCrate: consumerCrate.name, target: ff.target, file: rel(ff.file), line, owner: owner.name, ownerCrate: rel(owner.dir), class: cls, item: `${rec.owner ? rec.owner + "::" : ""}${[...rec.modpath, rec.name].join("::")} (${rec.kind})`, declFile: rel(rec.file), declLine: rec.line, gateRaw: rec.gate.cfgs.map(c => c.raw).join(" & "), sanctioned: enabled, features: rec.features});
};

for (const ff of fileFacts) {
  const consumer = crates.get(ff.crate)!;
  const externs = new Map<string, string>();
  for (const f of ff.facts) externs.set(f.ext, externsOf(consumer, consumer.targets.find(t => `${t.kind}:${t.name}` === ff.target)!).get(f.ext)!);
  const ownersInFile = new Set<string>();
  for (const f of ff.facts) {
    const ownerDir = externs.get(f.ext);
    if (!ownerDir) continue;
    const list = recsByCrate.get(ownerDir);
    if (!list) continue;
    ownersInFile.add(ownerDir);
    const R = f.rest;
    if (f.via === "glob") {
      for (const rec of list) {
        if (rec.owner) continue;
        const P = rec.modpath.join("::"), G = R.join("::");
        const under = G === "" || P === G || P.startsWith(G + "::");
        if (!under) continue;
        if (!ff.bare.has(rec.name) || ff.localDefs.has(rec.name)) continue;
        push(ff, f, ownerDir, rec, P === G ? "glob-exact" : "glob-loose", f.line);
      }
      continue;
    }
    if (R.length === 0) continue;
    const last = R[R.length - 1]!;
    const exactHere = list.some(r => !r.owner && (() => { const full = [...r.modpath, r.name]; return R.length >= full.length && full.every((x, k) => R[k] === x); })());
    const upaths = ungatedPaths.get(ownerDir);
    const resolvesUngated = upaths !== undefined && R.some((_, k) => upaths.has(R.slice(0, k + 1).join("::")));
    for (const rec of list) {
      if (rec.hidden) continue;
      if (rec.owner) {
        if (typeIsGated.get(ownerDir)?.has(rec.owner) || ungatedMethods.get(ownerDir)?.has(rec.owner + "::" + rec.name)) continue;
        if (R.length >= 2 && R[R.length - 2] === rec.owner && last === rec.name) push(ff, f, ownerDir, rec, "method-path", f.line);
        continue;
      }
      const full = [...rec.modpath, rec.name];
      const exact = R.length >= full.length && full.every((s, k) => R[k] === s) && (R.length === full.length || rec.kind === "mod" || rec.kind === "enum" || rec.kind === "struct" || rec.kind === "use" || rec.kind === "trait");
      if (exact && upaths?.has(full.join("::"))) continue;
      if (exact) { push(ff, f, ownerDir, rec, "exact", f.line); continue; }
      if (rec.kind === "mod" && R.length > full.length && full.every((s, k) => R[k] === s)) { push(ff, f, ownerDir, rec, "exact", f.line); continue; }
      if (resolvesUngated || exactHere) continue;
      if (last === rec.name) push(ff, f, ownerDir, rec, gatedOnlyName(ownerDir, rec.name) ? "name" : "ambiguous", f.line);
    }
  }
  for (const ownerDir of ownersInFile) {
    for (const rec of recsByCrate.get(ownerDir) ?? []) {
      if (!rec.owner) continue;
      if (typeIsGated.get(ownerDir)?.has(rec.owner) || ungatedMethods.get(ownerDir)?.has(rec.owner + "::" + rec.name)) continue;
      if (!ff.methods.has(rec.name) || !ff.types.has(rec.owner)) continue;
      if (!gatedOnlyMethod(ownerDir, rec.name)) continue;
      push(ff, undefined, ownerDir, rec, "method-call", 0);
    }
  }
}

const directKeys = new Set(hits.map(h => `${h.file}|${h.declFile}|${h.declLine}`));
for (const ev of events) {
  const cand = candByCrate.get(ev.crate)!;
  const list = (ev.method ? cand.methods.get(ev.key) : cand.names.get(ev.key)) ?? [];
  const pseudo: FileFacts = {types: new Set(), crate: ev.crate, target: "", file: ev.file, facts: [], bare: new Set(), methods: new Set(), localDefs: new Set(), roots: new Set()};
  for (const rec of list) {
    if (ev.lib && rec.crate === ev.crate) continue;
    if (ev.head !== "" && !["crate", "self", "super"].includes(ev.head)) {
      const cc = crates.get(ev.crate)!;
      const tg = cc.targets.find(t => t.kind === (ev.lib ? "lib" : t.kind));
      const ex = externsOf(cc, tg ?? cc.targets[0]!);
      const roots = aliasRoots.get(ev.crate)?.get(ev.head);
      const viaAlias = roots !== undefined && [...roots].some(r => ["crate", "self", "super"].includes(r) || ex.get(r) === rec.crate);
      if (!viaAlias && !modNames.get(ev.crate)?.has(ev.head) && ex.get(ev.head) !== rec.crate && !(ev.head === cc.lib && rec.crate === cc.dir)) continue;
    }
    if (rec.kind === "use") continue;
    if (directKeys.has(`${rel(ev.file)}|${rel(rec.file)}|${rec.line}`)) continue;
    const base = ev.method ? "indirect-method" : rec.kind === "use" ? "indirect-reexport" : "indirect";
    push(pseudo, undefined, rec.crate, rec, ev.strength === "glob" || ev.strength === "call" ? base + "-weak" : base, ev.line);
  }
}

const gatedCounts = new Map<string, number>();
for (const r of recs) gatedCounts.set(r.crate, (gatedCounts.get(r.crate) ?? 0) + 1);
const summary = {
  crates: crates.size, scannedCrates: selected.length, filesWithExternFacts: fileFacts.length, gatedPubRecords: recs.length, ownersWithGated: gatedCounts.size,
  hits: hits.length, byClass: Object.fromEntries([...new Set(hits.map(h => h.class))].map(c => [c, hits.filter(h => h.class === c).length])),
  sanctioned: hits.filter(h => h.sanctioned).length, missingMods: missingMods.length,
};
const text = hits.filter(h => !h.sanctioned).map(h => [h.class, h.consumerCrate, h.target, h.file + ":" + h.line, "->", h.owner, h.item, h.declFile + ":" + h.declLine, h.gateRaw].join("\t")).join("\n");
if (OUT) {
  writeFileSync(OUT + "-hits.json", JSON.stringify(hits, null, 1));
  writeFileSync(OUT + "-hits.tsv", text + "\n");
  writeFileSync(OUT + "-owners.tsv", [...gatedCounts].map(([d, n]) => `${crates.get(d)!.name}\t${n}\t${rel(d)}`).sort().join("\n") + "\n");
  writeFileSync(OUT + "-gated.json", JSON.stringify(recs.map(r => ({...r, file: rel(r.file), crate: crates.get(r.crate)!.name, gate: r.gate.cfgs.map(c => c.raw).join(" & "), kindGate: r.gate.kind})), null, 1));
  writeFileSync(OUT + "-missing-mods.txt", missingMods.join("\n") + "\n");
}
console.log(JSON.stringify(summary));
if (!OUT) console.log(text);
