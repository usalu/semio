import { existsSync } from "node:fs";
import { readText } from "./io.ts";
import { dirname, join, basename, resolve } from "node:path";
import { Body, FileModel, Item, UseLeaf, Vis, parseFile } from "./parse.ts";
import { Finding, Package, TargetDecl } from "./cargo.ts";
import { active, Env } from "./cfg.ts";

export const T = 1;
export const V = 2;
export const M = 4;

export type EntryKind = "mod" | "struct" | "enum" | "union" | "trait" | "fn" | "const" | "static" | "type" | "macro" | "extern_crate" | "import" | "variant";

export type Entry = {
  name: string;
  ns: number;
  kind: EntryKind;
  vis: Vis;
  cfg: string[];
  line: number;
  file: string;
  owner: Mod;
  item: Item | null;
  mod?: Mod;
  leaf?: UseLeaf;
  alias?: string;
  glob?: boolean;
};

export class Mod {
  entries: Entry[] = [];
  idx = new Map<string, Entry[]>();
  globs: Entry[] = [];
  kids: Mod[] = [];
  refs: Body["refs"] = [];
  local = new Set<string>();
  generics = new Set<string>();
  opaque = false;
  opaqueLines: number[] = [];
  cfgChain: string[];
  childDir = "";
  pathDir = "";
  decl: Item | null = null;
  suspect = false;
  constructor(
    public tree: CrateTree,
    public parent: Mod | null,
    public name: string,
    public file: string,
    public vis: Vis,
    cfg: string[],
    public isEnum = false,
  ) {
    this.cfgChain = parent ? [...parent.cfgChain, ...cfg] : cfg;
  }
  add(e: Entry): void {
    this.entries.push(e);
    if (e.glob) {
      this.globs.push(e);
      return;
    }
    const l = this.idx.get(e.name);
    if (l) l.push(e);
    else this.idx.set(e.name, [e]);
  }
  depth(): number {
    let d = 0;
    for (let m: Mod | null = this; m; m = m.parent) d++;
    return d;
  }
  path(): string {
    const parts: string[] = [];
    for (let m: Mod | null = this; m && m.parent; m = m.parent) parts.push(m.name);
    return "crate" + parts.reverse().map((p) => "::" + p).join("");
  }
}

export const allTrees: CrateTree[] = [];

export class CrateTree {
  root!: Mod;
  files = new Set<string>();
  findings: Finding[] = [];
  mods: Mod[] = [];
  exportedMacros: Entry[] = [];
  constructor(
    public pkg: Package,
    public target: TargetDecl,
    public id: string,
  ) {}
}

const fileCache = new Map<string, FileModel | null>();
export const parseStats = { files: 0, bytes: 0, errors: 0 };

/** 📥 Reads and parses one Rust file with caching; null when it does not exist. */
export function loadFile(path: string): FileModel | null {
  const hit = fileCache.get(path);
  if (hit !== undefined) return hit;
  let m: FileModel | null = null;
  if (existsSync(path)) {
    try {
      const src = readText(path);
      m = parseFile(path, src);
      parseStats.files++;
      parseStats.bytes += src.length;
      if (m.errors.length) parseStats.errors++;
    } catch {
      m = null;
    }
  }
  fileCache.set(path, m);
  return m;
}

export function fileModels(): IterableIterator<FileModel | null> {
  return fileCache.values();
}

const ANY_ENVS: Env[] = [
  { test: false, wasm: false },
  { test: true, wasm: false },
  { test: false, wasm: true },
];

function anyActive(chain: readonly string[]): boolean {
  return ANY_ENVS.some((e) => active(chain, e));
}

function nsOf(kind: Item["kind"], it: Item): number {
  switch (kind) {
    case "mod":
    case "enum":
    case "union":
    case "trait":
    case "type":
    case "extern_crate":
      return T;
    case "struct":
      return it.ctor ? T | V : T;
    case "fn":
    case "const":
    case "static":
      return V;
    case "macro":
      return M;
    default:
      return 0;
  }
}

function resolveRel(base: string, rel: string): string {
  return resolve(base, rel);
}

type Dirs = { childDir: string; pathDir: string } | null;

function splice(mod: Mod, body: Body, tree: CrateTree, file: string, depth: number, extraCfg: string[], dirs: Dirs = null): void {
  for (const r of body.refs) mod.refs.push({ ...r, src: file, cfg: extraCfg.length ? [...extraCfg, ...r.cfg] : r.cfg });
  for (const n of body.local) mod.local.add(n);
  for (const g of body.generics) mod.generics.add(g);
  if (body.opaque) {
    mod.opaque = true;
    mod.opaqueLines.push(...body.opaqueLines);
  }
  for (const it of body.items) addItem(mod, it, tree, file, depth, extraCfg, dirs);
}

function addItem(mod: Mod, it: Item, tree: CrateTree, file: string, depth: number, extraCfg: string[], dirs: Dirs): void {
  const cfg = extraCfg.length ? [...extraCfg, ...it.cfg] : it.cfg;
  const base = { vis: it.vis, cfg, line: it.line, file, owner: mod, item: it };
  if (it.kind === "use") {
    for (const l of it.leaves ?? []) {
      if (l.glob) mod.add({ ...base, name: "*", ns: 0, kind: "import", leaf: l, glob: true });
      else {
        const nm = l.alias ?? l.segs[l.segs.length - 1] ?? "";
        if (nm === "_") continue;
        mod.add({ ...base, name: nm, ns: 0, kind: "import", leaf: l });
      }
    }
    return;
  }
  if (it.kind === "extern_crate") {
    const nm = it.alias ?? it.name;
    if (nm !== "_") mod.add({ ...base, name: nm, ns: T, kind: "extern_crate", leaf: { segs: [it.name], alias: null, glob: false, self: false, line: it.line } });
    return;
  }
  if (it.kind === "macro_call") {
    const target = resolveRel(dirname(file), it.path ?? "");
    const fm = loadFile(target);
    if (!fm) {
      tree.findings.push(missing(tree, file, it.line, `include!(${JSON.stringify(it.path)}) target does not exist`, "include-missing"));
      return;
    }
    tree.files.add(target);
    if (depth < 48) splice(mod, fm.body, tree, target, depth + 1, cfg, { childDir: dirname(target), pathDir: dirname(target) });
    return;
  }
  if (it.kind === "impl") {
    mod.add({ ...base, name: "<impl>", ns: 0, kind: "type" as EntryKind });
    mod.entries[mod.entries.length - 1].ns = 0;
    return;
  }
  if (it.kind === "mod") {
    loadMod(mod, it, tree, file, depth, cfg, dirs);
    return;
  }
  if (it.kind === "macro") {
    const e: Entry = { ...base, name: it.name, ns: M, kind: "macro" };
    mod.add(e);
    if (it.macroExport) tree.exportedMacros.push({ ...e, vis: { k: "pub" }, owner: tree.root });
    return;
  }
  if (it.kind === "enum") {
    const em = new Mod(tree, mod, it.name, file, it.vis, cfg, true);
    em.cfgChain = [...mod.cfgChain, ...cfg];
    if (it.fromMacro) em.opaque = true;
    for (const v of it.variants ?? []) em.add({ name: v, ns: T | V, kind: "variant", vis: { k: "pub" }, cfg: [], line: it.line, file, owner: em, item: it });
    mod.add({ ...base, name: it.name, ns: T, kind: "enum", mod: em });
    return;
  }
  const kind = it.kind as EntryKind;
  const nm = it.name;
  if (nm === "_" || nm === "") return;
  mod.add({ ...base, name: nm, ns: nsOf(it.kind, it), kind });
  if (it.proc) {
    for (const d of it.proc.derive ?? []) mod.add({ ...base, name: d, ns: M, kind: "macro" });
    if (it.proc.attr || it.proc.bang) mod.add({ ...base, name: nm, ns: M, kind: "macro" });
  }
}

function missing(tree: CrateTree, file: string, line: number, msg: string, kind: string): Finding {
  return { crate: tree.id, kind, sev: "error", file, line, msg };
}

function loadMod(parent: Mod, it: Item, tree: CrateTree, file: string, depth: number, cfg: string[], dirs: Dirs): void {
  const baseChild = dirs?.childDir ?? parent.childDir;
  const basePath = dirs?.pathDir ?? parent.pathDir;
  const child = new Mod(tree, parent, it.name, file, it.vis, cfg);
  child.decl = it;
  tree.mods.push(child);
  parent.kids.push(child);
  parent.add({ name: it.name, ns: T, kind: "mod", vis: it.vis, cfg, line: it.line, file, owner: parent, item: it, mod: child });
  if (it.body) {
    child.childDir = join(baseChild, it.name);
    child.pathDir = child.childDir;
    if (it.path) {
      child.childDir = resolveRel(basePath, it.path);
      child.pathDir = child.childDir;
    }
    splice(child, it.body, tree, file, depth + 1, []);
    return;
  }
  let target: string | null = null;
  let modrs = false;
  if (it.path !== undefined) {
    target = resolveRel(basePath, it.path);
    modrs = true;
    if (!existsSync(target)) {
      if (anyActive(child.cfgChain)) tree.findings.push(missing(tree, file, it.line, `#[path = ${JSON.stringify(it.path)}] mod ${it.name} -> ${target} does not exist (resolved against ${basePath})`, "mod-missing"));
      return;
    }
  } else {
    const a = join(baseChild, `${it.name}.rs`);
    const b = join(baseChild, it.name, "mod.rs");
    if (existsSync(a)) target = a;
    else if (existsSync(b)) {
      target = b;
      modrs = true;
    } else {
      if (anyActive(child.cfgChain)) tree.findings.push(missing(tree, file, it.line, `mod ${it.name}; has neither ${a} nor ${b}`, "mod-missing"));
      return;
    }
  }
  const fm = loadFile(target);
  if (!fm) {
    tree.findings.push(missing(tree, file, it.line, `mod ${it.name}: cannot read ${target}`, "mod-missing"));
    return;
  }
  if (depth > 64) return;
  tree.files.add(target);
  child.file = target;
  child.pathDir = dirname(target);
  child.childDir = modrs ? dirname(target) : join(dirname(target), basename(target, ".rs"));
  if (fm.cfg.length) child.cfgChain = [...child.cfgChain, ...fm.cfg];
  for (const l of fm.errors) tree.findings.push({ crate: tree.id, kind: "parse-error", sev: "error", file: target, line: l, msg: "tree-sitter reported a syntax error" });
  if (fm.errors.length) child.opaque = child.suspect = true;
  splice(child, fm.body, tree, target, depth + 1, []);
}

/** 🌳 Builds the module tree of one crate target by following `mod`, `#[path]` and `include!`. */
export function buildTree(pkg: Package, target: TargetDecl, id: string): CrateTree {
  const tree = new CrateTree(pkg, target, id);
  allTrees.push(tree);
  const rootFile = target.path!;
  const root = new Mod(tree, null, "crate", rootFile, { k: "pub" }, []);
  tree.root = root;
  tree.mods.push(root);
  const fm = loadFile(rootFile);
  if (!fm) return tree;
  tree.files.add(rootFile);
  root.childDir = dirname(rootFile);
  root.pathDir = dirname(rootFile);
  root.cfgChain = [...fm.cfg];
  for (const l of fm.errors) tree.findings.push({ crate: id, kind: "parse-error", sev: "error", file: rootFile, line: l, msg: "tree-sitter reported a syntax error" });
  if (fm.errors.length) root.opaque = root.suspect = true;
  splice(root, fm.body, tree, rootFile, 0, []);
  for (const e of tree.exportedMacros) root.add(e);
  return tree;
}
