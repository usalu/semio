import { active, Env } from "./cfg.ts";
import { CrateTree, Entry, Mod, M, T, V } from "./tree.ts";
import { Package, DepSpec, depTarget, externName, loadPackage } from "./cargo.ts";
import { Vis } from "./parse.ts";
import { evalCfg, cfgOf } from "./cfg.ts";

export type DefKind = "mod" | "enum" | "crate" | "variant" | "struct" | "union" | "trait" | "type" | "fn" | "const" | "static" | "macro" | "ext";
export type Def = { kind: DefKind; ns: number; mod?: Mod; entry?: Entry };
export type ErrKind = "missing" | "private" | "unknown" | "not-module" | "no-lib";
export type Look = { defs: Def[]; err?: { kind: ErrKind; seg: number; name: string; at?: Mod; entry?: Entry } };
export type Ext = { name: string; pkg: Package | null; spec: DepSpec | null };

const STD_ROOTS = new Set(["std", "core", "alloc", "proc_macro", "test"]);
const PRIMS = new Set(["bool", "char", "str", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64"]);
const PRELUDE = new Set(["Option", "Some", "None", "Result", "Ok", "Err", "Vec", "Box", "String", "ToString", "ToOwned", "Clone", "Copy", "Send", "Sync", "Sized", "Unpin", "Drop", "Fn", "FnMut", "FnOnce", "Iterator", "IntoIterator", "Extend", "DoubleEndedIterator", "ExactSizeIterator", "Default", "Eq", "Ord", "PartialEq", "PartialOrd", "AsRef", "AsMut", "Into", "From", "TryFrom", "TryInto", "FromIterator", "Debug", "Hash", "drop", "Self", "Future", "IntoFuture", "AsyncFn", "AsyncFnMut", "AsyncFnOnce"]);

export type Ctx = { env: Env; home: CrateTree };

const memo = new Map<Entry, (Look | "busy" | undefined)[]>();
let modIds = new WeakMap<Mod, number>();
let nextId = 1;
const libTrees = new Map<Package, CrateTree | null>();
let treeBuilder: (pkg: Package) => CrateTree | null = () => null;

/** 🔧 Wires the lib-tree builder so cross-crate lookups can load a dependency lazily. */
export function setTreeBuilder(f: (pkg: Package) => CrateTree | null): void {
  treeBuilder = f;
}

export function libTree(pkg: Package): CrateTree | null {
  if (libTrees.has(pkg)) return libTrees.get(pkg)!;
  const t = treeBuilder(pkg);
  libTrees.set(pkg, t);
  return t;
}

function envIdx(env: Env): number {
  return (env.test ? 1 : 0) + (env.wasm ? 2 : 0);
}

function modId(m: Mod): number {
  let i = modIds.get(m);
  if (!i) {
    i = nextId++;
    modIds.set(m, i);
  }
  return i;
}

function actv(e: Entry, env: Env): boolean {
  const a = ((e as any)._a ??= [undefined, undefined, undefined, undefined]) as (boolean | undefined)[];
  const i = envIdx(env);
  let v = a[i];
  if (v === undefined) {
    v = active(e.cfg, env) && active(e.owner.cfgChain, env);
    a[i] = v;
  }
  return v;
}

export function isActive(e: Entry, env: Env): boolean {
  return actv(e, env);
}

export function modActive(m: Mod, env: Env): boolean {
  return active(m.cfgChain, env);
}

function isAncestor(a: Mod, b: Mod): boolean {
  for (let m: Mod | null = b; m; m = m.parent) if (m === a) return true;
  return false;
}

function realMod(m: Mod): Mod {
  while (m.isEnum && m.parent) m = m.parent;
  return m;
}

/** 🔐 Rust privacy: may code in `from` name an entry with `vis` declared in `owner`? */
export function visible(vis: Vis, owner: Mod, from: Mod): boolean {
  if (owner.tree !== from.tree) return vis.k === "pub";
  const o = realMod(owner);
  switch (vis.k) {
    case "pub":
    case "crate":
      return true;
    case "priv":
    case "self":
      return isAncestor(o, from);
    case "super":
      return isAncestor(o.parent ?? o, from);
    case "in": {
      let m: Mod = o;
      const p = vis.path;
      let i = 0;
      if (p[0] === "crate") {
        m = o.tree.root;
        i = 1;
      } else if (p[0] === "self") i = 1;
      else if (p[0] === "super") {
        while (p[i] === "super") {
          m = m.parent ?? m;
          i++;
        }
      }
      for (; i < p.length; i++) {
        const k = m.kids.find((x) => x.name === p[i]);
        if (!k) return true;
        m = k;
      }
      return isAncestor(m, from);
    }
  }
}

export function externs(tree: CrateTree, env: Env): Map<string, Ext> {
  const out = new Map<string, Ext>();
  const pkg = tree.pkg;
  const kind = tree.target.kind;
  const wantDev = (kind === "lib" || kind === "bin") ? env.test : true;
  const wantBuild = kind === "build";
  for (const d of pkg.deps) {
    if (d.section === "dev-dependencies" && !wantDev) continue;
    if (d.section === "build-dependencies" && !wantBuild) continue;
    if (d.section === "dependencies" && wantBuild) {
      // build scripts also see no normal dependencies
      continue;
    }
    if (d.target) {
      const m = /^cfg\((.*)\)$/s.exec(d.target.trim());
      if (m) {
        const e = cfgOf(m[1]);
        if (e && evalCfg(e, env) === false) continue;
      } else if (/^wasm32/.test(d.target) !== env.wasm) continue;
      else if (!/^wasm32/.test(d.target) && env.wasm) continue;
    }
    const t = depTarget(d);
    const name = externName(d, t.pkg);
    out.set(name, { name, pkg: t.pkg, spec: d });
  }
  if ((kind === "bin" || kind === "test" || kind === "bench" || kind === "example") && pkg.libName) out.set(pkg.libName, { name: pkg.libName, pkg, spec: null });
  for (const e of tree.root.entries) {
    if (e.kind !== "extern_crate" || !active(e.cfg, env) || !active(tree.root.cfgChain, env)) continue;
    const orig = e.leaf!.segs[0];
    const x = orig === "self" ? { name: e.name, pkg: tree.pkg.libName ? pkg : null, spec: null } : out.get(orig) ?? { name: e.name, pkg: null, spec: null };
    out.set(e.name, x);
  }
  return out;
}

const externCache = new WeakMap<CrateTree, (Map<string, Ext> | undefined)[]>();

function externsCached(tree: CrateTree, env: Env): Map<string, Ext> {
  let a = externCache.get(tree);
  if (!a) {
    a = [undefined, undefined, undefined, undefined];
    externCache.set(tree, a);
  }
  const i = envIdx(env);
  return (a[i] ??= externs(tree, env));
}

function switchCtx(m: Mod, ctx: Ctx): Ctx {
  if (m.tree === ctx.home) return ctx;
  return { env: { test: false, wasm: ctx.env.wasm }, home: m.tree };
}

function defOf(e: Entry): Def {
  switch (e.kind) {
    case "mod":
      return { kind: "mod", ns: T, mod: e.mod, entry: e };
    case "enum":
      return { kind: "enum", ns: T, mod: e.mod, entry: e };
    default:
      return { kind: e.kind as DefKind, ns: e.ns, entry: e };
  }
}

function extDef(): Def {
  return { kind: "ext", ns: T | V | M };
}

function crateDef(x: Ext, ctx: Ctx): Look {
  if (!x.pkg) return { defs: [extDef()] };
  const lt = x.pkg === ctx.home.pkg && ctx.home.target.kind === "lib" ? ctx.home : libTree(x.pkg);
  if (!lt) return { defs: [], err: { kind: "no-lib", seg: 0, name: x.name } };
  return { defs: [{ kind: "crate", ns: T, mod: lt.root }] };
}

function importDefs(e: Entry, ctx: Ctx): Look | "busy" {
  let a = memo.get(e);
  if (!a) {
    a = [undefined, undefined, undefined, undefined];
    memo.set(e, a);
  }
  const i = envIdx(ctx.env);
  const hit = a[i];
  if (hit === "busy") return "busy";
  if (hit) return hit;
  a[i] = "busy";
  let r: Look;
  const c = switchCtx(e.owner, ctx);
  if (e.kind === "extern_crate") {
    const nm = e.leaf!.segs[0];
    if (nm === "self") r = { defs: [{ kind: "crate", ns: T, mod: e.owner.tree.root }] };
    else {
      const x = externsCached(c.home, c.env).get(nm) ?? (STD_ROOTS.has(nm) ? { name: nm, pkg: null, spec: null } : undefined);
      r = x ? crateDef(x, c) : { defs: [], err: { kind: "missing", seg: 0, name: nm, at: e.owner } };
    }
  } else {
    r = resolvePath(e.owner, e.leaf!.segs, e.leaf!.glob ? "glob" : "use", c);
  }
  a[i] = r;
  return r;
}

function entryDefs(e: Entry, ctx: Ctx): Look | "busy" {
  if (e.kind === "import" || e.kind === "extern_crate") return importDefs(e, ctx);
  return { defs: [defOf(e)] };
}

type Seen = Set<string>;

/** 🔎 Looks `name` up in one module: own entries, then glob imports, honoring privacy and cfg. */
export function lookup(m: Mod, name: string, want: number, from: Mod, ctx: Ctx, seen: Seen = new Set()): Look {
  const c = switchCtx(m, ctx);
  const out: Def[] = [];
  let priv: Entry | undefined;
  let busy = false;
  for (const e of m.idx.get(name) ?? []) {
    if (!actv(e, c.env)) continue;
    const r = entryDefs(e, c);
    if (r === "busy") {
      busy = true;
      continue;
    }
    for (const d of r.defs) {
      if (!(d.ns & want)) continue;
      if (!visible(e.vis, m, from)) {
        priv ??= e;
        continue;
      }
      out.push(d);
    }
    if (r.err && r.defs.length === 0 && (r.err.kind === "unknown")) busy = true;
  }
  let covered = 0;
  for (const d of out) covered |= d.ns;
  if (out.length && (covered & want) === want) return { defs: out };
  let ext = false;
  const gwant = want & ~covered;
  for (const g of m.globs) {
    if (!actv(g, c.env)) continue;
    if (!visible(g.vis, m, from)) continue;
    const key = `${modId(m)}:${name}:${gwant}:${envIdx(c.env)}`;
    if (seen.has(key)) continue;
    seen.add(key);
    const r = entryDefs(g, c);
    if (r === "busy") {
      busy = true;
      continue;
    }
    for (const d of r.defs) {
      if (d.kind === "ext") {
        ext = true;
        continue;
      }
      if (!d.mod) continue;
      const sub = lookup(d.mod, name, gwant, m, c, seen);
      out.push(...sub.defs.map((x) => x));
      if (sub.err?.kind === "unknown") ext = true;
    }
    if (r.err && r.defs.length === 0) ext = ext || r.err.kind === "unknown" || r.err.kind === "missing";
    seen.delete(key);
  }
  if (out.length) return { defs: out };
  if (priv) return { defs: [], err: { kind: "private", seg: 0, name, at: m, entry: priv } };
  if (m.opaque || ext || busy) return { defs: [], err: { kind: "unknown", seg: 0, name, at: m } };
  return { defs: [], err: { kind: "missing", seg: 0, name, at: m } };
}

function builtinFirst(name: string, ctx: Ctx): Def[] | null {
  if (STD_ROOTS.has(name) || PRIMS.has(name) || PRELUDE.has(name)) return [extDef()];
  return null;
}

/** 🧭 Resolves the first path segment through module scope, then extern prelude, then std prelude. */
export function scopeLookup(m: Mod, name: string, want: number, ctx: Ctx, skipLocal = false): Look {
  const c = switchCtx(m, ctx);
  const own = lookup(m, name, want, m, c);
  if (own.defs.length) return own;
  if (!skipLocal && (m.local.has(name) || m.generics.has(name))) return { defs: [extDef()] };
  const x = externsCached(c.home, c.env).get(name);
  if (x) return crateDef(x, c);
  const b = builtinFirst(name, c);
  if (b) return { defs: b };
  if (own.err?.kind === "private") return own;
  if (m.opaque || own.err?.kind === "unknown") return { defs: [], err: { kind: "unknown", seg: 0, name, at: m } };
  return { defs: [], err: { kind: "missing", seg: 0, name, at: m } };
}

export type PathMode = "use" | "glob" | "code" | "macro";

/** 🛤️ Resolves a whole path from module `m`; modules and enums are walked, other items end the walk in code mode. */
export function resolvePath(m: Mod, segs: string[], mode: PathMode, ctx: Ctx): Look {
  const n = segs.length;
  if (n === 0) return { defs: [extDef()] };
  let i = 0;
  let cur: Def[];
  const c = switchCtx(m, ctx);
  const first = segs[0];
  const lastWant = mode === "macro" ? M : mode === "glob" ? T : T | V | M;
  if (first === "crate") {
    cur = [{ kind: "crate", ns: T, mod: m.tree.root }];
    i = 1;
  } else if (first === "self") {
    cur = [{ kind: "mod", ns: T, mod: realMod(m) }];
    i = 1;
  } else if (first === "super") {
    let mm: Mod = realMod(m);
    while (segs[i] === "super") {
      if (!mm.parent) return { defs: [], err: { kind: "missing", seg: i, name: "super", at: mm } };
      mm = realMod(mm.parent);
      i++;
    }
    cur = [{ kind: "mod", ns: T, mod: mm }];
  } else if (first === "::") {
    const nm = segs[1];
    const x = externsCached(c.home, c.env).get(nm) ?? (STD_ROOTS.has(nm) ? { name: nm, pkg: null, spec: null } : undefined);
    if (!x) return { defs: [], err: { kind: "missing", seg: 0, name: nm, at: m } };
    const r = crateDef(x, c);
    if (r.err) return { ...r, err: { ...r.err, seg: 0 } };
    cur = r.defs;
    i = 2;
  } else if (first === "Self") {
    return { defs: [extDef()] };
  } else {
    const want = n === 1 ? lastWant : T;
    const r = scopeLookup(m, first, want, c);
    if (r.err) return { defs: [], err: { ...r.err, seg: 0, name: first } };
    cur = r.defs;
    i = 1;
  }
  for (; i < n; i++) {
    const nm = segs[i];
    if (cur.some((d) => d.kind === "ext")) return { defs: [extDef()] };
    const mods = cur.filter((d) => d.mod);
    if (!mods.length) {
      if (mode === "code" || mode === "macro") return { defs: [extDef()] };
      return { defs: [], err: { kind: "not-module", seg: i, name: nm, at: m } };
    }
    const want = i < n - 1 ? T : lastWant;
    let next: Def[] = [];
    let err: Look["err"];
    for (const d of mods) {
      const r = lookup(d.mod!, nm, want, m, c);
      if (r.defs.length) next.push(...r.defs);
      else err ??= r.err;
    }
    if (!next.length && mode !== "use" && mode !== "glob" && mods.every((d) => d.kind === "enum")) return { defs: [extDef()] };
    if (!next.length) return { defs: [], err: { ...(err ?? { kind: "missing" as ErrKind, name: nm }), seg: i, name: nm, at: err?.at ?? mods[0].mod } };
    cur = next;
  }
  return { defs: cur };
}

export function resetMemo(): void {
  memo.clear();
  modIds = new WeakMap();
}

export function libNameIndex(pkgs: Iterable<Package>): Map<string, Package[]> {
  const out = new Map<string, Package[]>();
  for (const p of pkgs) {
    if (!p.libName) continue;
    const l = out.get(p.libName);
    if (l) l.push(p);
    else out.set(p.libName, [p]);
  }
  return out;
}

export { loadPackage };
