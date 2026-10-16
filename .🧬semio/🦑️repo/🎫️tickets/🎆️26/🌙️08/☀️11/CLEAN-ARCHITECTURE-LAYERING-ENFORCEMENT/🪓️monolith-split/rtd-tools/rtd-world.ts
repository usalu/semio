// 🌐️ Static Rust name resolver over first-party crates: module trees (#[path] aware), pub use chains, globs. Answers "which crate defines the item behind <crate>::a::b::C and by which public path".
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync} from "node:fs";
import {basename, dirname, join, resolve} from "node:path";
import {expandUse, mask} from "./rtd-rust.ts";

const ROOT = "/Users/ueli/Documents/semio";
const wsRoot = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const wsDeps: Record<string, any> = wsRoot.workspace?.dependencies ?? {};

export type Crate = {dir: string; pkg: string; rootFile: string; deps: Map<string, string | null>; root?: Mod};
export type Def = {name: string; kind: string; vis: string; file: string; pos: number; mod: Mod; attrs: string};
export type UseDecl = {vis: string; leaf: {segs: string[]; kind: "item" | "glob" | "self"; alias?: string}; attrs: string; file: string};
export type Mod = {id: string; crate: Crate; path: string[]; file: string; dirBase: string; defs: Map<string, Def>; mods: Map<string, Mod>; modVis: Map<string, string>; modAttrs: Map<string, string>; uses: UseDecl[]; externs: Map<string, string>; parent?: Mod; names?: Map<string, any>};

const crates = new Map<string, Crate>();
const byPkg = new Map<string, Crate>();

export const loadCrate = (dir: string): Crate | null => {
  dir = resolve(dir);
  if (crates.has(dir)) return crates.get(dir)!;
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) return null;
  const t = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const pkg = t.package?.name ?? basename(dir);
  const rootFile = resolve(dir, t.lib?.path ?? "src/lib.rs");
  const deps = new Map<string, string | null>();
  const add = (tbl: any) => {
    for (const [k, raw] of Object.entries(tbl ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const ws = spec.workspace === true ? (wsDeps[k] ?? wsDeps[spec.package ?? k] ?? {}) : {};
      const path = spec.path ?? ws.path;
      const ident = k.replace(/-/g, "_");
      if (typeof path === "string") deps.set(ident, resolve(spec.path === undefined && spec.workspace === true ? ROOT : dir, path)); else if (!deps.has(ident)) deps.set(ident, null);
    }
  };
  add(t.dependencies); add(t["dev-dependencies"]); add(t["build-dependencies"]);
  for (const b of Object.values(t.target ?? {}) as any[]) { add(b.dependencies); add(b["dev-dependencies"]); }
  const c: Crate = {dir, pkg, rootFile, deps};
  crates.set(dir, c);
  byPkg.set(pkg, c);
  return c;
};

const ITEM_RX = /^(pub(?:\s*\([^)]*\))?\s+)?((?:(?:default|unsafe|async|extern\s*"[^"]*"|extern)\s+)*(?:const\s+(?=(?:unsafe|async|extern|fn)\b))?)(struct|enum|union|trait|type|fn|static|const|mod|use|macro_rules\s*!|impl|extern\s+crate|macro)\b\s*([A-Za-z_][A-Za-z0-9_]*)?/;

const skipAttrs = (t: string, p: number): [number, string] => {
  let attrs = "";
  for (;;) {
    while (p < t.length && /\s/.test(t[p]!)) p++;
    if (t.startsWith("#![", p) || t.startsWith("#[", p)) {
      let d = 0, j = p + (t[p + 1] === "!" ? 2 : 1);
      for (; j < t.length; j++) { if (t[j] === "[") d++; else if (t[j] === "]") { d--; if (d === 0) { j++; break; } } }
      attrs += t.slice(p, j);
      p = j;
    } else return [p, attrs];
  }
};

const itemEnd = (t: string, p: number, bodyKinds: boolean, e: number): {end: number; bodyStart: number; bodyEnd: number} => {
  let d = 0, j = p, bodyStart = -1;
  for (; j < e; j++) {
    const ch = t[j]!;
    if (ch === "(" || ch === "[") d++;
    else if (ch === ")" || ch === "]") d--;
    else if (ch === "{") { if (d === 0 && bodyKinds && bodyStart < 0) bodyStart = j; d++; }
    else if (ch === "}") { d--; if (d === 0 && bodyKinds && bodyStart >= 0) return {end: j + 1, bodyStart, bodyEnd: j}; }
    else if (ch === ";" && d === 0) return {end: j + 1, bodyStart: -1, bodyEnd: -1};
  }
  return {end: e, bodyStart, bodyEnd: e};
};

const fileCache = new Map<string, {raw: string; masked: string}>();
const readMasked = (file: string) => {
  let v = fileCache.get(file);
  if (!v) { const raw = existsSync(file) ? readFileSync(file, "utf8") : ""; v = {raw, masked: mask(raw)}; fileCache.set(file, v); }
  return v;
};

const BODY = new Set(["struct", "enum", "union", "trait", "fn", "impl", "mod", "macro_rules!", "macro"]);

const parseRegion = (m: Mod, file: string, t: string, s: number, e: number, dirBase: string) => {
  let p = s;
  while (p < e) {
    let attrs: string;
    [p, attrs] = skipAttrs(t, p);
    if (p >= e) break;
    const rest = t.slice(p, Math.min(e, p + 300));
    const im = ITEM_RX.exec(rest);
    if (!im) {
      const st = itemEnd(t, p, true, e);
      p = Math.max(st.end, p + 1);
      continue;
    }
    const vis = (im[1] ?? "").replace(/\s+/g, "") || "";
    const kind = im[3]!.replace(/\s+/g, "");
    const name = im[4];
    const hasBody = BODY.has(kind) && kind !== "mod" || kind === "mod";
    const st = itemEnd(t, p, hasBody, e);
    if (kind === "mod" && name) {
      m.modVis.set(name, vis);
      m.modAttrs.set(name, attrs);
      const pm = /#\[\s*path\s*=\s*"([^"]+)"/.exec(attrs.length ? readRawAttr(file, t, p, attrs) : "");
      let child: Mod;
      if (st.bodyStart >= 0) {
        child = {id: `${m.id}::${name}`, crate: m.crate, path: [...m.path, name], file, dirBase: join(dirBase, name), defs: new Map(), mods: new Map(), modVis: new Map(), modAttrs: new Map(), uses: [], externs: new Map(), parent: m};
        parseRegion(child, file, t, st.bodyStart + 1, st.bodyEnd, child.dirBase);
      } else {
        let target = "";
        if (pm) target = resolve(dirBase, pm[1]!);
        else for (const cand of [join(dirBase, name, "🦀️.rs"), join(dirBase, `${name}.rs`), join(dirBase, name, "mod.rs")]) if (existsSync(cand)) { target = cand; break; }
        child = {id: `${m.id}::${name}`, crate: m.crate, path: [...m.path, name], file: target || file, dirBase: target ? dirname(target) : dirBase, defs: new Map(), mods: new Map(), modVis: new Map(), modAttrs: new Map(), uses: [], externs: new Map(), parent: m};
        if (target && existsSync(target)) { const r = readMasked(target); parseRegion(child, target, r.masked, 0, r.masked.length, child.dirBase); }
      }
      m.mods.set(name, child);
    } else if (kind === "use") {
      const body = t.slice(p + im[0].length - (im[4] ? im[4].length : 0) - 0, st.end).replace(/;\s*$/, "");
      const useText = t.slice(p, st.end).replace(/^(pub(?:\s*\([^)]*\))?\s+)?use\s+/, "").replace(/;\s*$/, "");
      for (const leaf of expandUse(useText.replace(/^::/, ""))) m.uses.push({vis, leaf, attrs, file});
      void body;
    } else if (kind === "externcrate" && name) {
      const am = /^extern\s+crate\s+([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?/.exec(t.slice(p, st.end));
      if (am) m.externs.set(am[2] ?? am[1]!, am[1]!);
    } else if (kind === "macro_rules!" && name) {
      if (/macro_export/.test(attrs)) rootOf(m).defs.set(name + "!", {name, kind: "macro", vis: "pub", file, pos: p, mod: rootOf(m), attrs});
      m.defs.set(name + "!", {name, kind: "macro", vis: /macro_export/.test(attrs) ? "pub" : "", file, pos: p, mod: m, attrs});
    } else if (name && kind !== "impl") {
      const prev = m.defs.get(name);
      if (!prev || (prev.vis === "" && vis !== "")) m.defs.set(name, {name, kind, vis, file, pos: p, mod: m, attrs});
      if (kind === "enum" || kind === "struct" || kind === "union") m.defs.set(name, {name, kind, vis, file, pos: p, mod: m, attrs});
    }
    p = Math.max(st.end, p + 1);
  }
};

const readRawAttr = (file: string, masked: string, p: number, attrs: string): string => {
  const raw = readMasked(file).raw;
  const mStart = masked.lastIndexOf(attrs.slice(0, 10), p);
  void mStart;
  const idx = raw.lastIndexOf("#[path", p);
  if (idx < 0 || p - idx > 400) return "";
  const close = raw.indexOf("]", idx);
  return raw.slice(idx, close + 1);
};

const rootOf = (m: Mod): Mod => { let r = m; while (r.parent) r = r.parent; return r; };

export const rootMod = (c: Crate): Mod => {
  if (c.root) return c.root;
  const r = readMasked(c.rootFile);
  const m: Mod = {id: c.pkg, crate: c, path: [], file: c.rootFile, dirBase: dirname(c.rootFile), defs: new Map(), mods: new Map(), modVis: new Map(), modAttrs: new Map(), uses: [], externs: new Map()};
  c.root = m;
  parseRegion(m, c.rootFile, r.masked, 0, r.masked.length, m.dirBase);
  return m;
};

export type Ent = {kind: "crate" | "mod"; mod: Mod} | {kind: "item"; def: Def; rest: string[]} | {kind: "external"; name: string};

const isPub = (v: string) => v === "pub";

export class Resolver {
  private guard = new Set<string>();
  trace: string[] = [];
  externCrate(from: Crate, ident: string): Ent | null {
    if (from.deps.has(ident)) {
      const d = from.deps.get(ident);
      if (!d) return {kind: "external", name: ident};
      const c = loadCrate(d);
      if (!c) return {kind: "external", name: ident};
      this.trace.push(c.pkg);
      return {kind: "crate", mod: rootMod(c)};
    }
    if (ident === from.pkg.replace(/-/g, "_")) return {kind: "crate", mod: rootMod(from)};
    if (["std", "core", "alloc"].includes(ident)) return {kind: "external", name: ident};
    return null;
  }
  private memo = new Map<string, Ent | null>();
  private cuts = 0;
  lookup(m: Mod, name: string, publicOnly: boolean, seen?: Set<string>): Ent | null {
    const key = `${m.id}|${name}|${publicOnly}`;
    if (!seen && this.memo.has(key)) { const e = this.memo.get(key)!; if (e?.kind === "crate") this.trace.push(e.mod.crate.pkg); return e; }
    if (seen) { if (seen.has(key)) return null; seen.add(key); }
    else {
      if (this.guard.has(key)) { this.cuts++; return null; }
      this.guard.add(key);
      seen = new Set([key]);
    }
    const top = this.guard.has(key);
    const before = this.cuts;
    try {
      const r = this.lookupUncached(m, name, publicOnly, seen);
      if (top && (r || this.cuts === before)) this.memo.set(key, r);
      return r;
    } finally { if (top) this.guard.delete(key); }
  }
  private lookupUncached(m: Mod, name: string, publicOnly: boolean, seen: Set<string>): Ent | null {
    const sub = m.mods.get(name);
    if (sub && (!publicOnly || isPub(m.modVis.get(name) ?? ""))) return {kind: "mod", mod: sub};
    const def = m.defs.get(name);
    if (def && (!publicOnly || isPub(def.vis))) return {kind: "item", def, rest: []};
    for (const u of m.uses) {
      if (publicOnly && !isPub(u.vis)) continue;
      if (u.leaf.kind === "glob") continue;
      const bind = u.leaf.alias ?? u.leaf.segs[u.leaf.segs.length - 1];
      if (bind !== name || bind === "_") continue;
      const r = this.walk(m, u.leaf.segs);
      if (r) return r;
    }
    for (const u of m.uses) {
      if (publicOnly && !isPub(u.vis)) continue;
      if (u.leaf.kind !== "glob") continue;
      const r = this.walk(m, u.leaf.segs);
      if (r && (r.kind === "mod" || r.kind === "crate")) {
        const hit = this.lookup(r.mod, name, true, seen);
        if (hit) return hit;
      }
    }
    const ex = m.externs.get(name);
    if (ex) return this.externCrate(m.crate, ex);
    return null;
  }
  first(m: Mod, seg: string): Ent | null {
    if (seg === "crate") return {kind: "crate", mod: rootMod(m.crate)};
    if (seg === "self") return {kind: "mod", mod: m};
    if (seg === "super") return m.parent ? {kind: "mod", mod: m.parent} : null;
    return this.localNoGlob(m, seg) ?? this.externCrate(m.crate, seg) ?? this.lookup(m, seg, false);
  }
  private localNoGlob(m: Mod, name: string): Ent | null {
    const sub = m.mods.get(name);
    if (sub) return {kind: "mod", mod: sub};
    const def = m.defs.get(name);
    if (def) return {kind: "item", def, rest: []};
    const key = `${m.id}|${name}|local`;
    if (this.guard.has(key)) return null;
    this.guard.add(key);
    try {
      for (const u of m.uses) {
        if (u.leaf.kind === "glob") continue;
        const bind = u.leaf.alias ?? u.leaf.segs[u.leaf.segs.length - 1];
        if (bind !== name || bind === "_") continue;
        const r = this.walk(m, u.leaf.segs);
        if (r) return r;
      }
      const ex = m.externs.get(name);
      return ex ? this.externCrate(m.crate, ex) : null;
    } finally { this.guard.delete(key); }
  }
  walk(m: Mod, segs: string[]): Ent | null {
    if (!segs.length) return {kind: "mod", mod: m};
    let ent: Ent | null = this.first(m, segs[0]!);
    for (let i = 1; i < segs.length && ent; i++) {
      const s = segs[i]!;
      if (ent.kind === "external") return ent;
      if (ent.kind === "item") { ent = {...ent, rest: [...ent.rest, s]}; continue; }
      if (s === "super") { ent = ent.mod.parent ? {kind: "mod", mod: ent.mod.parent} : null; continue; }
      const crossing = ent.mod.crate !== m.crate;
      ent = this.lookup(ent.mod, s, crossing);
    }
    return ent;
  }
  resolveFrom(c: Crate, segs: string[]): {ent: Ent | null; via: string[]} {
    this.trace = [];
    const ent = this.walk(rootMod(c), ["crate", ...segs]);
    return {ent, via: [...new Set(this.trace)]};
  }
  gatesOf(def: Def, path: string[]): string[] {
    const out: string[] = [];
    const note = (a: string) => { const t = a.replace(/\s+/g, " ").trim(); if (/cfg/.test(t)) out.push(t); };
    note(def.attrs);
    let m: Mod | undefined = def.mod;
    while (m && m.parent) { const nm = m.path[m.path.length - 1]!; note(m.parent.modAttrs.get(nm) ?? ""); m = m.parent; }
    const root = rootMod(def.mod.crate);
    if (path.length) {
      let cur: Mod = root;
      for (let i = 0; i < path.length; i++) {
        const seg = path[i]!;
        for (const u of cur.uses) {
          const bind = u.leaf.alias ?? u.leaf.segs[u.leaf.segs.length - 1];
          if (u.vis === "pub" && (bind === seg || u.leaf.kind === "glob") && (u.leaf.kind === "glob" ? this.lookup(this.walk(cur, u.leaf.segs)?.kind === "mod" || this.walk(cur, u.leaf.segs)?.kind === "crate" ? (this.walk(cur, u.leaf.segs) as any).mod : cur, seg, true) !== null : true)) note(u.attrs);
        }
        const sub = cur.mods.get(seg);
        if (sub) { note(cur.modAttrs.get(seg) ?? ""); cur = sub; }
      }
    }
    return [...new Set(out)];
  }
  publicPath(def: Def): string[] | null {
    const c = def.mod.crate;
    const root = rootMod(c);
    const same = (e: Ent | null) => e?.kind === "item" && e.def.file === def.file && e.def.pos === def.pos;
    const flat = this.lookup(root, def.name, true);
    if (same(flat)) return [def.name];
    const direct = [...def.mod.path, def.name];
    let m: Mod = root, ok = true;
    for (const s of def.mod.path) {
      const n = this.lookup(m, s, true);
      if (!n || n.kind !== "mod") { ok = false; break; }
      m = n.mod;
    }
    if (ok && same(this.lookup(m, def.name, true))) return direct;
    const seen = new Set<string>();
    const bfs = (mm: Mod, prefix: string[], depth: number): string[] | null => {
      if (depth > 3 || seen.has(mm.id)) return null;
      seen.add(mm.id);
      for (const [name] of mm.mods) {
        const e = this.lookup(mm, name, true);
        if (!e || e.kind !== "mod") continue;
        const hit = this.lookup(e.mod, def.name, true);
        if (same(hit)) return [...prefix, name, def.name];
      }
      for (const [name] of mm.mods) {
        const e = this.lookup(mm, name, true);
        if (e && e.kind === "mod") { const r = bfs(e.mod, [...prefix, name], depth + 1); if (r) return r; }
      }
      return null;
    };
    return bfs(root, [], 0);
  }
}

export const defOf = (e: Ent | null): Def | null => (e && e.kind === "item" ? e.def : null);
export const crateFor = (pkg: string): Crate | undefined => byPkg.get(pkg);
export const lineAt = (d: Def): number => { const t = readMasked(d.file).masked; let n = 1; for (let i = 0; i < d.pos; i++) if (t.charCodeAt(i) === 10) n++; return n; };
