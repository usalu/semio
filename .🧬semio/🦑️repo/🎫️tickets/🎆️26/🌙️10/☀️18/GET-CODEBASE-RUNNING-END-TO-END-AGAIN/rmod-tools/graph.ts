import { existsSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { loadCrates, parse, type Chain, type Crate, type ModDecl, type Parsed, type UseEntry } from "./rs.ts";
import { renamed, root } from "./lib.ts";

export const OLD = new Set(["helpers", "base", "core", "shared", "common"]);
export const NEWNAMES = new Set(["operator_registration", "mesh_operator_io", "part_libraries", "check_evidence", "editor_vocabulary", "sweep_machinery", "intersection_numerics", "issuance_clone", "immutable_alias", "arc_custody"]);
export const OLDOF: Record<string, string> = { operator_registration: 'helpers', mesh_operator_io: 'helpers', part_libraries: 'helpers', check_evidence: 'helpers', editor_vocabulary: 'base', sweep_machinery: 'core', intersection_numerics: 'shared', issuance_clone: 'shared', immutable_alias: 'shared', arc_custody: 'shared' };
export const ALLNAMES = new Set([...OLD, ...NEWNAMES]);

export type Spec = { file: string; old: string; new: string; dir: string };
export class Mod {
  static n = 0;
  id = Mod.n++;
  kids = new Map<string, Mod>();
  spec: Spec | null = null;
  site: ModDecl | null = null;
  extra: { decl: ModDecl; parsed: Parsed }[] = [];
  decls(): ModDecl[] { return this.extra.length ? [this.decl, ...this.extra.map((e) => e.decl)] : [this.decl]; }
  constructor(public crate: CrateNode, public parsed: Parsed, public decl: ModDecl, public parent: Mod | null, public name: string, public dir: string) {}
  path(): string { return this.parent ? `${this.parent.path()}::${this.name}` : `${this.crate.crate.ident}`; }
}
export type CrateNode = { crate: Crate; root: Mod | null; roots: Mod[]; externs: Map<string, string> };

const snake = (d: string) => d.slice(d.lastIndexOf("/") + 1).replace(/^[^a-z0-9]*/u, "").replace(/-/g, "_");

export const buildGraph = (names: Set<string> = OLD) => {
  const crates = loadCrates();
  const byIdent = new Map<string, Crate>();
  const dupIdent: string[] = [];
  for (const c of crates) {
    if (!c.roots.some((r) => r.kind === "lib")) continue;
    if (byIdent.has(c.ident)) dupIdent.push(c.ident); else byIdent.set(c.ident, c);
  }
  const dirs = renamed();
  const specByFile = new Map<string, Spec>();
  for (const d of dirs) specByFile.set(join(d, "🦀️.rs"), { file: join(d, "🦀️.rs"), old: "", new: snake(d), dir: d });
  const parsedCache = new Map<string, Parsed | null>();
  const getParsed = (f: string): Parsed | null => {
    if (parsedCache.has(f)) return parsedCache.get(f)!;
    let p: Parsed | null = null;
    if (existsSync(f) && statSync(f).isFile()) { try { p = parse(f, names); } catch (e) { console.error("parse fail", f, String(e)); } }
    parsedCache.set(f, p);
    return p;
  };
  const nodes = new Map<string, CrateNode>();
  const allMods: Mod[] = [];
  const modsByFile = new Map<string, Mod[]>();
  const modsByDecl = new Map<ModDecl, Mod[]>();
  const instantiate = (cn: CrateNode, parsed: Parsed, decl: ModDecl, parent: Mod | null, name: string, dir: string, chain: Set<string>): Mod => {
    const m = new Mod(cn, parsed, decl, parent, name, dir);
    allMods.push(m);
    { const a = modsByDecl.get(decl) ?? []; a.push(m); modsByDecl.set(decl, a); }
    if (!decl.inline || decl.isRoot) { const a = modsByFile.get(parsed.file) ?? []; a.push(m); modsByFile.set(parsed.file, a); }
    const collectIncludes = (d: ModDecl, depth: number) => {
      if (depth > 8) return;
      for (const inc of d.includes) {
        const f = resolve(dirname(d.file), inc);
        const ip = getParsed(f);
        if (!ip) continue;
        m.extra.push({ decl: ip.root, parsed: ip });
        { const a = modsByDecl.get(ip.root) ?? []; a.push(m); modsByDecl.set(ip.root, a); }
        collectIncludes(ip.root, depth + 1);
      }
    };
    collectIncludes(decl, 0);
    const allKids: { k: ModDecl; baseDir: string }[] = decl.kids.map((k) => ({ k, baseDir: dir }));
    for (const e of m.extra) for (const k of e.decl.kids) allKids.push({ k, baseDir: dirname(e.decl.file) });
    for (const { k, baseDir } of allKids) {
      const dir = baseDir;
      if (k.inline) {
        const kd = k.pathAttr !== null ? resolve(dir, k.pathAttr) : join(dir, k.name);
        const kp = getParsed(k.file) ?? parsed;
        const km = instantiate(cn, kp, k, m, k.name, kd, chain);
        if (!m.kids.has(k.name)) m.kids.set(k.name, km);
      } else {
        const cands: string[] = [];
        if (k.pathAttr !== null) cands.push(resolve(dir, k.pathAttr));
        else { cands.push(join(dir, k.name + ".rs"), join(dir, k.name, "mod.rs")); }
        const f = cands.find((c) => existsSync(c));
        if (!f) continue;
        const key = f + "|" + m.id;
        if (chain.has(f)) continue;
        const cp = getParsed(f);
        if (!cp) continue;
        const nc = new Set(chain); nc.add(f);
        const km = instantiate(cn, cp, cp.root, m, k.name, dirname(f), nc);
        km.site = k;
        const sp = specByFile.get(f);
        if (sp && names.has(k.name)) { km.spec = { ...sp, old: OLDOF[sp.new] ?? k.name }; }
        if (!m.kids.has(k.name)) m.kids.set(k.name, km);
        if (km.spec && !m.kids.has(km.spec.old)) m.kids.set(km.spec.old, km);
      }
    }
    return m;
  };
  for (const c of crates) {
    const cn: CrateNode = { crate: c, root: null, roots: [], externs: new Map() };
    nodes.set(c.manifest, cn);
  }
  const libs = new Map<string, CrateNode>();
  for (const cn of nodes.values()) {
    for (const r of cn.crate.roots) {
      const p = getParsed(r.file);
      if (!p) continue;
      const m = instantiate(cn, p, p.root, null, "", dirname(r.file), new Set([r.file]));
      cn.roots.push(m);
      if (r.kind === "lib") { cn.root = m; if (byIdent.get(cn.crate.ident) === cn.crate) libs.set(cn.crate.ident, cn); }
    }
  }
  return { crates, nodes, libs, allMods, modsByFile, modsByDecl, parsedCache, dupIdent, specByFile };
};

export type G = ReturnType<typeof buildGraph>;

export const makeResolver = (g: G) => {
  const rootOf = (m: Mod) => { let c = m; while (c.parent) c = c.parent; return c; };
  const crateRoot = (m: Mod): Mod => rootOf(m);
  const externCrate = (m: Mod, name: string): Mod | null => {
    const r = rootOf(m);
    for (const e of r.decls().flatMap((d) => d.externs)) if (e.alias === name) { const t = g.libs.get(e.crate); return t?.root ?? null; }
    for (const mm of g.modsByFile.get(r.parsed.file) ?? []) if (mm.crate === m.crate) for (const e of mm.decls().flatMap((d) => d.externs)) if (e.alias === name) { const t = g.libs.get(e.crate); return t?.root ?? null; }
    const t = g.libs.get(name);
    return t?.root ?? null;
  };
  const found = new Map<string, Mod>();
  let visited: Set<string> | null = null;
  const lookupIn = (m: Mod, name: string): Mod | null => {
    const k = m.id + ":" + name;
    const f = found.get(k);
    if (f) return f;
    if (visited!.has(k)) return null;
    visited!.add(k);
    let res: Mod | null = m.kids.get(name) ?? null;
    if (!res) for (const u of m.decls().flatMap((d) => d.uses)) {
      if (u.glob || u.bind !== name) continue;
      const r = resolveIn(m, u.segs, u.lead);
      if (r) { res = r; break; }
    }
    if (!res) for (const e of m.decls().flatMap((d) => d.externs)) if (e.alias === name) { const t = g.libs.get(e.crate); if (t?.root) { res = t.root; break; } }
    if (!res) for (const u of m.decls().flatMap((d) => d.uses)) {
      if (!u.glob) continue;
      const gm = resolveIn(m, u.segs, u.lead);
      if (gm && gm !== m) { const r = lookupIn(gm, name); if (r) { res = r; break; } }
    }
    if (res) found.set(k, res);
    return res;
  };
  const resolveIn = (m: Mod, segs: string[], lead: boolean): Mod | null => {
    let cur: Mod | null = null;
    let i = 0;
    const s0 = segs[0];
    if (s0 === undefined) return m;
    if (lead) { cur = externCrate(m, s0); i = 1; }
    else if (s0 === "crate" || s0 === "$crate") { cur = crateRoot(m); i = 1; }
    else if (s0 === "self") { cur = m; i = 1; }
    else if (s0 === "super") { cur = m.parent; i = 1; while (cur && segs[i] === "super") { cur = cur.parent; i++; } }
    else { cur = lookupIn(m, s0); if (!cur) cur = externCrate(m, s0); i = 1; }
    for (; cur && i < segs.length; i++) {
      const s = segs[i]!;
      if (s === "self") continue;
      if (s === "super") { cur = cur.parent; continue; }
      cur = lookupIn(cur, s);
    }
    return cur;
  };
  const lookup = (m: Mod, name: string): Mod | null => { visited = new Set(); try { return lookupIn(m, name); } finally { visited = null; } };
  const resolvePath = (m: Mod, segs: string[], lead: boolean): Mod | null => { visited = new Set(); try { return resolveIn(m, segs, lead); } finally { visited = null; } };
  return { lookup, resolvePath, rootOf };
};
