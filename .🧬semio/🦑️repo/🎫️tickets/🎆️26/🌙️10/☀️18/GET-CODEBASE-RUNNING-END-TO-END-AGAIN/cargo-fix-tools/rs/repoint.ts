import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { relative } from "node:path";
import { Env } from "./cfg.ts";
import { CrateTree, Entry, Mod, M, T, V, allTrees, loadFile } from "./tree.ts";
import { ErrInfo, ROOT, rel } from "./checks.ts";
import { active } from "./cfg.ts";
import { Ctx, Def, externs, libTree, lookup, resolvePath } from "./resolve.ts";
import { depTarget, loadPackage, Package } from "./cargo.ts";
import { Item } from "./parse.ts";

const C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const X = `${C}/🗑️generated/cargo-fix-x`;

type Edit = { file: string; start: number; end: number; text: string; why: string; key: string; hash: string };
export const edits = new Map<string, Edit[]>();
export const stats = { relocated: 0, imported: 0, widened: 0, dropped: 0, skipped: new Map<string, number>(), sample: [] as string[] };
const seenKeys = new Set<string>();

function skip(reason: string, detail: string): void {
  stats.skipped.set(reason, (stats.skipped.get(reason) ?? 0) + 1);
  if (stats.sample.length < 3000) stats.sample.push(`${reason}\t${detail}`);
}

function addEdit(file: string, start: number, end: number, text: string, why: string, key: string): boolean {
  if (seenKeys.has(key)) return true;
  const fm = loadFile(file);
  if (!fm) return false;
  seenKeys.add(key);
  const l = edits.get(file) ?? [];
  l.push({ file, start, end, text, why, key, hash: fm.hash });
  edits.set(file, l);
  return true;
}

type Cand = { mod: Mod; entry: Entry };
const indexCache = new WeakMap<CrateTree, Map<string, Cand[]>>();

function defIndex(tree: CrateTree): Map<string, Cand[]> {
  let ix = indexCache.get(tree);
  if (ix) return ix;
  ix = new Map();
  for (const m of tree.mods) {
    if (m.isEnum) continue;
    for (const e of m.entries) {
      if (e.glob || e.kind === "variant" || e.name === "<impl>" || e.kind === "extern_crate" || e.item?.fromMacro) continue;
      if (e.kind === "import" && e.vis.k === "priv") continue;
      const l = ix.get(e.name);
      if (l) l.push({ mod: m, entry: e });
      else ix.set(e.name, [{ mod: m, entry: e }]);
    }
  }
  indexCache.set(tree, ix);
  return ix;
}

function pathOf(c: Cand, from: CrateTree): string[] {
  const mp = c.mod.path().split("::").filter((seg, i) => c.mod.tree === from || !(i > 0 && (seg === "mirror" || seg === "component")));
  if (c.mod.tree === from) return [...mp, c.entry.name];
  const lib = c.mod.tree.pkg.libName ?? c.mod.tree.pkg.name.replace(/-/g, "_");
  return [lib, ...mp.slice(1), c.entry.name];
}

function entityOf(c: Cand, ctx: Ctx): Item | Entry | null {
  if (c.entry.kind !== "import") return c.entry.item ?? c.entry;
  const r = resolvePath(c.entry.owner, c.entry.leaf!.segs, "use", { env: ctx.env, home: c.entry.owner.tree });
  const d = r.defs.find((x) => x.entry);
  return d?.entry ? d.entry.item ?? d.entry : null;
}

function siteTestOnly(info: ErrInfo): boolean {
  if (info.tree.target.kind !== "lib" && info.tree.target.kind !== "bin") return true;
  return [...info.mod.cfgChain, ...(info.ref?.cfg ?? [])].some((c) => /\btest\b/.test(c));
}

function domain(info: ErrInfo): CrateTree[] {
  const out: CrateTree[] = [info.tree];
  const denv = siteTestOnly(info) ? info.env : { test: false, wasm: info.env.wasm };
  for (const x of externs(info.tree, denv).values()) {
    if (!x.pkg) continue;
    const lt = x.pkg === info.tree.pkg && info.tree.target.kind === "lib" ? info.tree : libTree(x.pkg);
    if (lt && !out.includes(lt)) out.push(lt);
  }
  return out;
}

type Found = { segs: string[]; entity: Item | Entry; def: Def };

function strongName(name: string, oldSegs: string[], newSegs: string[]): boolean {
  if (/^[A-Z]/.test(name)) return name.length >= 5;
  if (name.length >= 8 && name.includes("_")) return true;
  const shared = oldSegs.slice(1, -1).filter((s) => !["crate", "self", "super"].includes(s) && newSegs.includes(s));
  return shared.length > 0;
}

function findCandidate(info: ErrInfo, name: string, wantNs: number, mode: "use" | "code" | "macro"): Found | "none" | "ambiguous" {
  const ctx: Ctx = { env: info.env, home: info.tree };
  const doms = domain(info);
  const entities = new Map<Item | Entry, { cands: Cand[]; tree: CrateTree }>();
  for (const t of doms) {
    for (const c of defIndex(t).get(name) ?? []) {
      if (!(c.entry.kind === "import" || (c.entry.ns & wantNs))) continue;
      if (c.mod.tree !== info.tree && c.entry.vis.k !== "pub") continue;
      const ent = entityOf(c, ctx);
      if (!ent) continue;
      const e = entities.get(ent) ?? { cands: [], tree: t };
      e.cands.push(c);
      entities.set(ent, e);
    }
  }
  if (entities.size === 0) return globalCandidate(info, name, wantNs, mode, ctx);
  let pool = [...entities];
  if (pool.length > 1) {
    const here = pool.filter(([, v]) => v.tree === info.tree);
    if (here.length === 1) pool = here;
    else {
      const nonTest = pool.filter(([, v]) => v.cands.some((c) => !/(^|::)tests?(::|$)/.test(c.mod.path())));
      if (nonTest.length === 1) pool = nonTest;
    }
  }
  if (pool.length !== 1) return "ambiguous";
  const [ent, v] = pool[0];
  const order = [...v.cands].sort((a, b) => {
    const ai = a.entry.kind === "import" ? 1 : 0;
    const bi = b.entry.kind === "import" ? 1 : 0;
    if (ai !== bi) return ai - bi;
    return a.mod.depth() - b.mod.depth();
  });
  for (const c of order) {
    const full = pathOf(c, info.tree);
    const head = full.slice(0, c.mod.tree === info.tree ? 1 : 1);
    const mids = full.slice(1, -1);
    for (let k = 0; k <= mids.length; k++) {
      const segs = [...head, ...mids.slice(0, mids.length - k), full[full.length - 1]];
      const r = resolvePath(info.mod, segs, mode, ctx);
      if (r.err || !r.defs.length) continue;
      if (!r.defs.some((d) => (d.entry?.item ?? d.entry) === ent || d.kind === "ext")) continue;
      return { segs, entity: ent, def: r.defs[0] };
    }
  }
  return "none";
}


const globalIx = { built: -1, map: new Map<string, Cand[]>() };

function globalIndex(): Map<string, Cand[]> {
  if (globalIx.built === allTrees.length) return globalIx.map;
  const map = new Map<string, Cand[]>();
  for (const t of allTrees) {
    if (t.target.kind !== "lib" || !t.pkg.libName) continue;
    for (const m of t.mods) {
      if (m.isEnum) continue;
      for (const e of m.entries) {
        if (e.glob || e.kind === "import" || e.kind === "variant" || e.kind === "extern_crate" || e.name === "<impl>") continue;
        if (e.vis.k !== "pub") continue;
        const l = map.get(e.name);
        if (l) l.push({ mod: m, entry: e });
        else map.set(e.name, [{ mod: m, entry: e }]);
      }
    }
  }
  globalIx.built = allTrees.length;
  globalIx.map = map;
  return map;
}

const reachMemo = new Map<string, boolean>();

function pkgReaches(from: Package, to: Package): boolean {
  const key = `${from.dir}>${to.dir}`;
  const hit = reachMemo.get(key);
  if (hit !== undefined) return hit;
  const seen = new Set<string>();
  const st = [from.dir];
  let r = false;
  while (st.length && !r) {
    const d = st.pop()!;
    if (d === to.dir) {
      r = true;
      break;
    }
    if (seen.has(d)) continue;
    seen.add(d);
    const p = loadPackage(d);
    if (!p) continue;
    for (const dep of p.deps) if (dep.section === "dependencies" && dep.path) st.push(dep.path);
  }
  reachMemo.set(key, r);
  return r;
}

function nameableAt(tree: CrateTree, parts: string[], name: string, ns: number, ent: Item | Entry, info: ErrInfo): boolean {
  const ctx: Ctx = { env: { test: false, wasm: info.env.wasm }, home: tree };
  let cur: Mod = tree.root;
  for (const seg of parts) {
    const r = lookup(cur, seg, T, info.mod, ctx);
    const d = r.defs.find((x) => x.mod);
    if (!d) return false;
    cur = d.mod!;
  }
  const last = lookup(cur, name, ns || T | V | M, info.mod, ctx);
  return last.defs.some((d) => (d.entry?.item ?? d.entry) === ent);
}

function globalCandidate(info: ErrInfo, name: string, wantNs: number, mode: "use" | "code" | "macro", ctx: Ctx): Found | "none" | "ambiguous" {
  const all = (globalIndex().get(name) ?? []).filter((c) => c.mod.tree !== info.tree && (c.entry.ns & wantNs));
  const prod = all.filter((c) => !/(^|::)tests?(::|$)/.test(c.mod.path()) && !/-(tests?|laws|fixtures?)(-|$)/.test(c.mod.tree.pkg.name) && c.mod.cfgChain.length === 0);
  const pool = prod.length ? prod : all;
  const entities = new Map<Item | Entry, Cand[]>();
  for (const c of pool) {
    const k = c.entry.item ?? c.entry;
    const l = entities.get(k) ?? [];
    l.push(c);
    entities.set(k, l);
  }
  if (entities.size === 0) return "none";
  if (entities.size > 1) return "ambiguous";
  const [ent, cands] = [...entities][0];
  const normalSite = !siteTestOnly(info);
  for (const c of cands.sort((a, b) => a.mod.depth() - b.mod.depth())) {
    if (normalSite && pkgReaches(c.mod.tree.pkg, info.tree.pkg)) continue;
    const parts = c.mod.path().split("::").slice(1).filter((seg) => seg !== "mirror" && seg !== "component");
    for (let k = 0; k <= parts.length; k++) {
      const pre = parts.slice(0, k);
      if (!nameableAt(c.mod.tree, pre, c.entry.name, c.entry.ns, ent, info)) continue;
      const lib = c.mod.tree.pkg.libName ?? c.mod.tree.pkg.name.replace(/-/g, "_");
      return { segs: [lib, ...pre, c.entry.name], entity: ent, def: { kind: "ext", ns: c.entry.ns } };
    }
  }
  return "none";
}

const wantFor = (mode: string): number => (mode === "macro" ? M : T | V | M);

function dropLeaf(file: string, item: Item, leaf: { s: number; e: number; segs: string[] }, why: string): boolean {
  const src = readFileSync(file, "utf8");
  const leaves = item.leaves ?? [];
  if (leaves.length <= 1) {
    let e = item.end;
    while (e < src.length && (src[e] === " " || src[e] === "\t")) e++;
    if (src[e] === "\n") e++;
    let s = item.at;
    while (s > 0 && (src[s - 1] === " " || src[s - 1] === "\t")) s--;
    return addEdit(file, s, e, "", why, `${file}:${item.at}:dropdecl`);
  }
  let s = leaf.s;
  let e = leaf.e;
  let k = e;
  while (k < src.length && /\s/.test(src[k])) k++;
  if (src[k] === ",") {
    e = k + 1;
    while (e < src.length && /[ \t]/.test(src[e])) e++;
  } else {
    let q = s - 1;
    while (q >= 0 && /\s/.test(src[q])) q--;
    if (src[q] === ",") s = q;
  }
  return addEdit(file, s, e, "", why, `${file}:${leaf.s}:drop`);
}

function redundantLeaf(info: ErrInfo): boolean {
  if (info.what !== "use" || !info.leaf || !info.entry?.item || info.ref || info.leaf.glob) return false;
  const bound = info.leaf.alias ?? info.leaf.segs[info.leaf.segs.length - 1];
  const ctx: Ctx = { env: info.env, home: info.tree };
  for (const o of info.mod.idx.get(bound) ?? []) {
    if (o === info.entry) continue;
    if (o.kind === "import") {
      const r = resolvePath(o.owner, o.leaf!.segs, "use", ctx);
      if (r.err) continue;
    }
    const ok = dropLeaf(info.entry.file, info.entry.item, info.leaf, `drop redundant ${info.leaf.segs.join("::")} (name ${bound} already bound)`);
    if (ok) stats.dropped++;
    return ok;
  }
  return false;
}

function removeItemWithLeaders(file: string, item: Item, why: string): boolean {
  const src = readFileSync(file, "utf8");
  let s = src.lastIndexOf("\n", item.at - 1) + 1;
  if (src.slice(s, item.at).trim() !== "") s = item.at;
  else {
    while (s > 0) {
      const ps = src.lastIndexOf("\n", s - 2) + 1;
      const t = src.slice(ps, s - 1).trim();
      if (t.startsWith("///") || t.startsWith("#[")) s = ps;
      else break;
    }
  }
  let e = item.end;
  while (e < src.length && (src[e] === " " || src[e] === "\t")) e++;
  if (src[e] === "\n") e++;
  return addEdit(file, s, e, "", why, `${file}:${item.at}:rmitem`);
}

function emptyImpl(file: string, it: Item): boolean {
  if (it.kind !== "impl" || (it.methods ?? []).length) return false;
  const t = readFileSync(file, "utf8").slice(it.at, it.end);
  const o = t.indexOf("{");
  return o >= 0 && t.slice(o + 1, t.lastIndexOf("}")).trim() === "";
}

function planDupItems(tree: CrateTree, a: Entry, b: Entry): boolean {
  if (!a.item || !b.item || a.item === b.item || a.item.kind !== b.item.kind) return false;
  if (a.item.kind === "impl") {
    const drop = emptyImpl(b.file, b.item) ? b : emptyImpl(a.file, a.item) ? a : null;
    if (drop) {
      const ok = removeItemWithLeaders(drop.file, drop.item!, `remove empty conflicting impl ${drop.item!.name} for ${drop.item!.implKey}`);
      if (ok) stats.dropped++;
      return ok;
    }
  }
  if (a.item.kind === "impl" && false) return false;
  if (!["fn", "mod", "const", "static", "struct", "enum", "type", "trait", "impl"].includes(b.item.kind)) return false;
  const sa = readFileSync(a.file, "utf8").slice(a.item.at, a.item.end).replace(/\s+/g, " ").trim();
  const sb = readFileSync(b.file, "utf8").slice(b.item.at, b.item.end).replace(/\s+/g, " ").trim();
  if (sa !== sb) {
    skip("dup-different-items", `${tree.id} ${rel(b.file)}:${b.line} ${b.name}`);
    return false;
  }
  if (removeItemWithLeaders(b.file, b.item, `remove byte-identical duplicate ${b.item.kind} ${b.name}`)) {
    stats.dropped++;
    return true;
  }
  return false;
}

export function planDup(tree: CrateTree, mod: Mod, env: Env, a: Entry, b: Entry): void {
  if ((a.kind !== "import" && b.kind !== "import") || a.item?.kind === "impl") {
    if (planDupItems(tree, a, b)) return;
  }
  if (a.kind !== "import" || b.kind !== "import" || a.glob || b.glob || !a.leaf || !b.leaf || !b.item || b.item.kind !== "use") return;
  const ctx: Ctx = { env, home: tree };
  const ra = resolvePath(mod, a.leaf.segs, "use", ctx);
  const rb = resolvePath(mod, b.leaf.segs, "use", ctx);
  if (ra.err || rb.err) return;
  const ea = new Set(ra.defs.map((d) => d.entry?.item ?? d.entry));
  const same = rb.defs.length > 0 && rb.defs.every((d) => ea.has(d.entry?.item ?? d.entry));
  if (!same) {
    skip("dup-different-entity", `${tree.id} ${rel(b.file)}:${b.line} ${a.leaf.segs.join("::")} vs ${b.leaf.segs.join("::")}`);
    return;
  }
  const victim = a.cfg.length > b.cfg.length && a.item?.kind === "use" && a.leaf ? a : b;
  if (dropLeaf(victim.file, victim.item!, victim.leaf!, `drop duplicate import ${victim.leaf!.segs.join("::")}`)) stats.dropped++;
}

function tryRelocate(info: ErrInfo): boolean {
  if (redundantLeaf(info)) return true;
  const segs = info.segs;
  const n = segs.length;
  const file = info.src ?? info.mod.file;
  const mode = info.what === "macro" ? "macro" : info.what === "use" ? "use" : "code";
  const from = Math.max(info.err.seg, 1);
  for (let j = n - 1; j >= from; j--) {
    const name = segs[j];
    if (name === "*" || name === "self" || name === "super" || name === "crate") continue;
    if (j >= 1 && /^[A-Z]/.test(segs[j - 1]) && j - 1 >= info.err.seg) {
      skip("assoc-of-type", `${info.tree.id} ${rel(file)}:${info.line} ${segs.join("::")}`);
      continue;
    }
    const sub = segs.slice(0, j + 1);
    const prefixOk = j === n - 1 ? true : true;
    if (!prefixOk) continue;
    const found = findCandidate(info, name, j === n - 1 ? wantFor(mode) : T, j === n - 1 ? (mode as any) : "code");
    if (found === "none" || found === "ambiguous") {
      if (j === from) skip(found, `${info.tree.id} ${rel(file)}:${info.line} ${segs.join("::")}`);
      continue;
    }
    const newPath = found.segs.join("::");
    const oldPrefix = sub.join("::");
    if (!/^(crate|self|super|semio_|machine$|pack$|protocol$|store$|dsl$|vcs$)/.test(segs[0]) && segs[0] !== "::") {
      skip("foreign-root", `${info.tree.id} ${rel(file)}:${info.line} ${segs.join("::")} -> ${newPath}`);
      return false;
    }
    if (newPath === oldPrefix) return false;
    if (!strongName(name, sub, found.segs)) {
      skip("weak-name", `${info.tree.id} ${rel(file)}:${info.line} ${segs.join("::")} -> ${newPath}`);
      return false;
    }
    const rest = segs.slice(j + 1);
    if (info.what === "use" && info.leaf && info.entry?.item && !info.ref) {
      const leaf = info.leaf;
      if (leaf.list || leaf.plen > 0) return relocateListLeaf(info, found.segs, rest, file);
      const src = readFileSync(file, "utf8");
      const text = src.slice(leaf.s, leaf.e);
      if (!text.startsWith(oldPrefix)) {
        skip("leaf-text-mismatch", `${rel(file)}:${info.line} ${text.slice(0, 80)}`);
        return false;
      }
      const ok = addEdit(file, leaf.s, leaf.s + oldPrefix.length, newPath, `relocate ${oldPrefix} -> ${newPath}`, `${file}:${leaf.s}`);
      if (ok) stats.relocated++;
      return ok;
    }
    if (info.ref && info.ref.at !== undefined && (info.what === "path" || info.what === "macro")) {
      const src = readFileSync(file, "utf8");
      const at = info.ref.at;
      let probe = oldPrefix;
      if (info.ref.segs[0] === "::") probe = oldPrefix.replace(/^::::/, "::");
      if (!src.startsWith(probe, at)) {
        skip("ref-text-mismatch", `${rel(file)}:${info.line} ${src.slice(at, at + 80).replace(/\n/g, " ")} vs ${probe}`);
        return false;
      }
      const ok = addEdit(file, at, at + probe.length, newPath, `relocate ${probe} -> ${newPath}`, `${file}:${at}`);
      if (ok) stats.relocated++;
      return ok;
    }
    if (info.what === "use" && info.ref && info.leaf) {
      const leaf = info.leaf;
      if (leaf.list || leaf.plen > 0 || leaf.glob) {
        skip("body-use", `${rel(file)}:${info.line} ${segs.join("::")}`);
        return false;
      }
      const src = readFileSync(file, "utf8");
      if (!src.startsWith(oldPrefix, leaf.s)) {
        skip("leaf-text-mismatch", `${rel(file)}:${info.line}`);
        return false;
      }
      const ok = addEdit(file, leaf.s, leaf.s + oldPrefix.length, newPath, `relocate ${oldPrefix} -> ${newPath}`, `${file}:${leaf.s}`);
      if (ok) stats.relocated++;
      return ok;
    }
    return false;
  }
  return false;
}

type LeafPlan = { leaf: ErrInfo["leaf"] & {}; text: string };
const pendingLeaves = new Map<string, { file: string; item: Item; plans: Map<number, LeafPlan> }>();

function relocateListLeaf(info: ErrInfo, newSegs: string[], rest: string[], file: string): boolean {
  const item = info.entry!.item!;
  const leaf = info.leaf!;
  const key = `${file}:${item.at}`;
  const cur = pendingLeaves.get(key) ?? { file, item, plans: new Map() };
  cur.plans.set(leaf.s, { leaf, text: [...newSegs, ...rest].join("::") });
  pendingLeaves.set(key, cur);
  return true;
}

function flushLeaves(): void {
  for (const [key, v] of pendingLeaves) {
    const src = readFileSync(v.file, "utf8");
    const item = v.item;
    const visText = item.visEnd !== undefined ? src.slice(item.at, item.visEnd) + " " : "";
    const leaves = item.leaves ?? [];
    const stmts = [...v.plans.values()].map((p) => `${visText}use ${p.text}${p.leaf.alias ? ` as ${p.leaf.alias}` : ""};`);
    if (v.plans.size >= leaves.length) {
      if (addEdit(v.file, item.at, item.end, stmts.join("\n"), `relocate list ${stmts.length} leaves`, key)) stats.relocated += stmts.length;
      continue;
    }
    for (const p of v.plans.values()) {
      let s = p.leaf.s;
      let e = p.leaf.e;
      let k = e;
      while (k < src.length && /\s/.test(src[k])) k++;
      if (src[k] === ",") {
        e = k + 1;
        while (e < src.length && /[ \t]/.test(src[e])) e++;
      } else {
        let q = s - 1;
        while (q >= 0 && /\s/.test(src[q])) q--;
        if (src[q] === ",") s = q;
      }
      addEdit(v.file, s, e, "", `drop list leaf ${p.leaf.segs.join("::")}`, `${v.file}:${p.leaf.s}:drop`);
    }
    const nl = src.indexOf("\n", item.end);
    const at = nl < 0 ? item.end : nl + 1;
    if (addEdit(v.file, at, at, stmts.join("\n") + "\n", `relocate ${stmts.length} list leaves`, `${key}:new`)) stats.relocated += stmts.length;
  }
  pendingLeaves.clear();
}


type AliasHit = { text: string; pkg: Package; ws: string | null };
const aliasUse = { built: -1, map: new Map<string, Map<string, AliasHit[]>>() };

function aliasUseIndex(): Map<string, Map<string, AliasHit[]>> {
  if (aliasUse.built === allTrees.length) return aliasUse.map;
  const map = new Map<string, Map<string, AliasHit[]>>();
  for (const t of allTrees) {
    for (const m of t.mods) {
      if (m.isEnum) continue;
      for (const e of m.entries) {
        if (e.kind !== "import" || e.glob || !e.leaf || !e.leaf.alias) continue;
        const segs = e.leaf.segs;
        if (segs[0] === "super" || segs[0] === "self") continue;
        const text = segs.join("::");
        let byText = map.get(e.leaf.alias);
        if (!byText) map.set(e.leaf.alias, (byText = new Map()));
        const l = byText.get(text) ?? [];
        l.push({ text, pkg: t.pkg, ws: t.pkg.ws?.dir ?? null });
        byText.set(text, l);
      }
    }
  }
  aliasUse.built = allTrees.length;
  aliasUse.map = map;
  return map;
}

function aliasConvention(info: ErrInfo, name: string): string | null {
  const byText = aliasUseIndex().get(name);
  if (!byText) return null;
  const pick = (pred: (h: AliasHit) => boolean): string | null => {
    const scored = [...byText].map(([text, hits]) => [text, hits.filter(pred).length] as const).filter(([, n]) => n > 0).sort((a, b) => b[1] - a[1]);
    if (!scored.length) return null;
    const total = scored.reduce((a, [, n]) => a + n, 0);
    return scored[0][1] / total >= 0.8 ? scored[0][0] : null;
  };
  const own = pick((h) => h.pkg === info.tree.pkg);
  const text = own ?? pick((h) => !!h.ws && h.ws === info.tree.pkg.ws?.dir && !h.text.startsWith("crate")) ?? pick((h) => !h.text.startsWith("crate"));
  if (!text) return null;
  if (text.startsWith("crate") && !own) return null;
  return `${text} as ${name}`;
}

const STD: Record<string, string> = {
  HashMap: "std::collections::HashMap", HashSet: "std::collections::HashSet", BTreeMap: "std::collections::BTreeMap", BTreeSet: "std::collections::BTreeSet", VecDeque: "std::collections::VecDeque", BinaryHeap: "std::collections::BinaryHeap",
  ManuallyDrop: "std::mem::ManuallyDrop", Arc: "std::sync::Arc", Mutex: "std::sync::Mutex", RwLock: "std::sync::RwLock", Rc: "std::rc::Rc", RefCell: "std::cell::RefCell", Cell: "std::cell::Cell", PhantomData: "std::marker::PhantomData",
  Duration: "std::time::Duration", Instant: "std::time::Instant", Ordering: "std::cmp::Ordering", PathBuf: "std::path::PathBuf", Path: "std::path::Path", Cow: "std::borrow::Cow", Pin: "std::pin::Pin", OnceLock: "std::sync::OnceLock", AtomicUsize: "std::sync::atomic::AtomicUsize", AtomicU64: "std::sync::atomic::AtomicU64", AtomicBool: "std::sync::atomic::AtomicBool",
};

const pendingImports = new Map<string, { file: string; at: number; inline: boolean; text: string; envs: Set<string>; mod: Mod; name: string }>();

function domainReaches(info: ErrInfo, lib: string): boolean {
  if (lib === "crate" || lib === "self" || lib === "super") return true;
  return domain(info).some((t) => (t.pkg.libName ?? t.pkg.name.replace(/-/g, "_")) === lib);
}

function tryImport(info: ErrInfo): boolean {
  const name = info.segs[0];
  if (info.what !== "path" && info.what !== "macro" && info.what !== "use") return false;
  if (name === "::" || name === "crate" || name === "self" || name === "super" || name === "Self") return false;
  let found = findCandidate(info, name, info.what === "macro" ? M : T, info.what === "macro" ? "macro" : "code");
  let aliasText: string | null = null;
  if (info.ref?.bare && name === "Value") {
    const text = readFileSync(info.src ?? info.mod.file, "utf8");
    const sj = /\bserde_json::/.test(text);
    const pj = /\bpack_json::|\bsemio_framework_pack_json::/.test(text);
    if (sj === pj) {
      skip("bare-value-ambiguous", `${info.tree.id} ${rel(info.src ?? info.mod.file)}:${info.line}`);
      return false;
    }
    const aliasValue = sj ? "serde_json::Value" : "semio_framework_pack_json::Value";
    const fileV = info.src ?? info.entry?.file ?? info.mod.file;
    const fmV = loadFile(fileV);
    if (!fmV) return false;
    const declV = info.mod.decl;
    const inlineV = !!declV?.body && declV.bodyStart !== undefined && info.mod.file === fileV;
    const atV = inlineV ? declV!.bodyStart! : fmV.useAt;
    const keyV = `${info.tree.id}|${info.mod.path()}|${info.mod.file}|Value`;
    if (!pendingImports.has(keyV)) pendingImports.set(keyV, { file: fileV, at: atV, inline: inlineV, text: aliasValue, envs: new Set([info.envs]), mod: info.mod, name: "Value" });
    return true;
  }
  if (info.ref?.bare && /^(Value|Error|Number|Map|Context|Duration|Instant|Path|Result|Item|Event|State|Output|Input|Config|Self)$/.test(name)) {
    skip("bare-registry-name", `${info.tree.id} ${rel(info.src ?? info.mod.file)}:${info.line} ${name}`);
    return false;
  }
  if (info.ref?.bare && found !== "none" && found !== "ambiguous" && !domainReaches(info, found.segs[0]) && !(name.length >= 10 && /^[A-Z][a-z0-9]+([A-Z][a-z0-9]*)+$/.test(name))) found = "none";
  if (found === "none" || found === "ambiguous") {
    aliasText = (info.what === "path" && STD[name]) || aliasConvention(info, name);
    if (!aliasText) {
      skip(`qualifier-${found}`, `${info.tree.id} ${rel(info.src ?? info.mod.file)}:${info.line} ${info.segs.join("::")}`);
      return false;
    }
    found = { segs: aliasText.split(" as ")[0].split("::"), entity: {} as any, def: { kind: "ext", ns: T } };
  }
  const file = info.src ?? info.entry?.file ?? info.mod.file;
  const fm = loadFile(file);
  if (!fm) return false;
  const decl = info.mod.decl;
  const inline = !!decl?.body && decl.bodyStart !== undefined && info.mod.file === file;
  const at = inline ? decl!.bodyStart! : fm.useAt;
  const key = `${info.tree.id}|${info.mod.path()}|${info.mod.file}|${name}`;
  const p = aliasText ?? found.segs.join("::");
  const cur = pendingImports.get(key);
  if (cur) {
    cur.envs.add(info.envs);
    return true;
  }
  pendingImports.set(key, { file, at, inline, text: p, envs: new Set([info.envs]), mod: info.mod, name });
  return true;
}

function afterInnerDocs(src: string, at: number, inline: boolean): number {
  let p = at;
  let last = -1;
  for (;;) {
    let q = p;
    while (q < src.length && /\s/.test(src[q])) q++;
    if (!src.startsWith("//!", q) && !src.startsWith("#![", q)) break;
    let e = src.indexOf("\n", q);
    if (e < 0) e = src.length;
    last = e;
    p = e;
  }
  return last < 0 ? at : inline ? last : last + 1;
}

function flushImports(): void {
  for (const [key, v] of pendingImports) {
    const onlyTest = [...v.envs].every((e) => e === "test");
    const modTestOnly = v.mod.cfgChain.length > 0;
    const prefix = onlyTest && !modTestOnly ? "#[cfg(test)] " : "";
    const text = v.inline ? `\n    ${prefix}use ${v.text};` : `${prefix}use ${v.text};\n`;
    const at = afterInnerDocs(readFileSync(v.file, "utf8"), v.at, !!v.inline);
    if (addEdit(v.file, at, at, text, `import ${v.name} from ${v.text}`, key)) stats.imported++;
  }
  pendingImports.clear();
}

function tryWiden(info: ErrInfo): boolean {
  const e = info.err.entry;
  if (!e || !e.item || e.item.fromMacro) return false;
  const file = e.file;
  const crossCrate = e.owner.tree !== info.mod.tree;
  if (crossCrate && e.owner.tree.pkg !== info.tree.pkg && !info.tree.pkg.deps.some((d) => depTarget(d).pkg === e.owner.tree.pkg)) {
    skip("widen-no-cross-crate-user", `${info.tree.id} ${rel(info.src ?? info.mod.file)}:${info.line} ${e.name}`);
    return false;
  }
  const vis = crossCrate ? "pub" : "pub(crate)";
  const it = e.item;
  if (e.kind === "variant") return false;
  const src = readFileSync(file, "utf8");
  let start = it.at;
  let end = it.at;
  if (it.visEnd !== undefined) end = it.visEnd;
  const cur = src.slice(start, end);
  if (cur === "pub") return true;
  if (cur.startsWith("pub") && crossCrate === false && /pub\s*\((crate|super|self|in )/.test(cur)) {
    if (cur.replace(/\s/g, "") === "pub(crate)") return true;
  }
  const text = it.visEnd !== undefined ? (/\s/.test(src[end] ?? "") ? vis : `${vis} `) : `${vis} `;
  if (e.kind !== "import" && !["struct", "enum", "union", "trait", "fn", "const", "static", "type", "mod", "macro"].includes(e.kind)) return false;
  if (e.kind === "macro") return false;
  const ok = addEdit(file, start, end, text, `widen ${e.name} -> ${vis}`, `${file}:${it.at}`);
  if (ok) stats.widened++;
  return ok;
}

const queue: ErrInfo[] = [];

/** 🩺 Collects every definite resolution error; edits are planned after all trees are loaded. */
export function plan(info: ErrInfo): void {
  queue.push(info);
}

export const suggestions: string[] = [];

function suggestFor(info: ErrInfo): void {
  const e = info.err;
  if (e.kind !== "missing") return;
  const name = e.name;
  if (!name || ["crate", "self", "super", "::"].includes(name)) return;
  const here = defIndex(info.tree).get(name) ?? [];
  const cands = [...here.map((c) => ({ c, local: true })), ...(globalIndex().get(name) ?? []).filter((c) => c.mod.tree !== info.tree).map((c) => ({ c, local: false }))];
  const direct = new Set(info.tree.pkg.deps.map((d) => depTarget(d).pkg));
  const out = cands.slice(0, 8).map(({ c, local }) => ({
    pkg: c.mod.tree.pkg.name,
    path: c.mod.path() + "::" + c.entry.name,
    kind: c.entry.kind,
    vis: c.entry.vis.k,
    local,
    direct: local || direct.has(c.mod.tree.pkg),
    cycle: !local && pkgReaches(c.mod.tree.pkg, info.tree.pkg),
  }));
  suggestions.push(JSON.stringify({ crate: info.tree.id, file: rel(info.src ?? info.mod.file), line: info.line, path: info.segs.join("::"), seg: e.seg, name, what: info.what, envs: info.envs, cands: out }));
}

function planOne(info: ErrInfo): void {
  suggestFor(info);
  const e = info.err;
  if (e.kind === "missing" && e.seg === 0) {
    if (!tryImport(info) && info.segs.length >= 2) tryRelocate(info);
    return;
  }
  if (e.kind === "missing" || e.kind === "not-module") {
    tryRelocate(info);
    return;
  }
  if (e.kind === "private") {
    tryWiden(info);
    return;
  }
}

/** 💾 Applies planned edits per file, bottom-up, skipping files that changed since parsing. */
export function applyAll(apply: boolean): { files: number; applied: number; overlaps: number; stale: number } {
  for (const i of queue) planOne(i);
  queue.length = 0;
  flushLeaves();
  flushImports();
  mkdirSync(`${X}/backup`, { recursive: true });
  const ledgerPath = `${X}/backup/ledger.json`;
  const ledger: Record<string, string> = existsSync(ledgerPath) ? JSON.parse(readFileSync(ledgerPath, "utf8")) : {};
  let applied = 0;
  let overlaps = 0;
  let stale = 0;
  const log: string[] = [];
  for (const [file, list] of edits) {
    const src = readFileSync(file, "utf8");
    const h = createHash("sha1").update(src).digest("hex");
    if (list[0].hash !== h) {
      stale++;
      continue;
    }
    list.sort((a, b) => a.start - b.start || a.end - b.end);
    const keep: Edit[] = [];
    let lastEnd = -1;
    for (const e of list) {
      if (e.start < lastEnd) {
        overlaps++;
        continue;
      }
      keep.push(e);
      lastEnd = Math.max(lastEnd, e.end);
    }
    let out = src;
    for (let i = keep.length - 1; i >= 0; i--) {
      const e = keep[i];
      out = out.slice(0, e.start) + e.text + out.slice(e.end);
      log.push(`${relative(ROOT, file)}\t${e.why}`);
    }
    applied += keep.length;
    if (apply) {
      const bh = createHash("sha1").update(file).digest("hex").slice(0, 16);
      if (!ledger[file]) {
        writeFileSync(`${X}/backup/${bh}`, src);
        ledger[file] = bh;
        writeFileSync(ledgerPath, JSON.stringify(ledger, null, 1));
      }
      const cur = readFileSync(file, "utf8");
      if (createHash("sha1").update(cur).digest("hex") === h) writeFileSync(file, out);
      else stale++;
    }
  }
  writeFileSync(`${X}/repoint-${apply ? `applied-${process.env.RS_TAG ?? Date.now()}` : "dry"}.log`, log.join("\n") + "\n");
  writeFileSync(`${X}/repoint-skipped.tsv`, stats.sample.join("\n") + "\n");
  return { files: edits.size, applied, overlaps, stale };
}
