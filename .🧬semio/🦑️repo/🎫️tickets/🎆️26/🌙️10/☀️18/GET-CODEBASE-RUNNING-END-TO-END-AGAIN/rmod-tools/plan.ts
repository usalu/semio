import { writeFileSync } from "node:fs";
import { ALLNAMES, NEWNAMES, buildGraph, makeResolver, Mod, OLD } from "./graph.ts";
import { C, root } from "./lib.ts";

const t0 = Date.now();
const g = buildGraph(ALLNAMES);
console.error(`graph: crates ${g.crates.length} libs ${g.libs.size} mods ${g.allMods.length} files ${g.parsedCache.size} dupIdent ${g.dupIdent.length} (${Date.now() - t0} ms)`);
const R = makeResolver(g);
const specMods = g.allMods.filter((m) => m.spec);
const specSites = new Map<string, { spec: string; sites: Set<string>; crates: Set<string> }>();
for (const m of specMods) {
  const e = specSites.get(m.spec!.dir) ?? { spec: `${m.spec!.old}->${m.spec!.new}`, sites: new Set(), crates: new Set() };
  e.sites.add(m.site!.file.slice(root.length + 1)); e.crates.add(m.crate.crate.name);
  specSites.set(m.spec!.dir, e);
}
for (const [d, e] of specSites) console.error(d.slice(root.length + 1).slice(-60), e.spec, "sites", e.sites.size, "crates", [...e.crates].join(","));

type Ed = { a: number; b: number; old: string; nw: string; why: string };
const edits = new Map<string, Map<number, Ed>>();
const conflicts: string[] = [];
const broken: string[] = [];
const other = new Map<string, Set<number>>();
const add = (file: string, off: [number, number], old: string, nw: string, why: string) => {
  const f = edits.get(file) ?? new Map(); edits.set(file, f);
  const ex = f.get(off[0]);
  if (ex && ex.nw !== nw) conflicts.push(`${file}@${off[0]} ${ex.nw} vs ${nw}`);
  f.set(off[0], { a: off[0], b: off[1], old, nw, why });
};
const evalPath = (m: Mod, segs: string[], offs: [number, number][], lead: boolean, file: string, why: string) => {
  for (let k = 0; k < segs.length; k++) {
    const s = segs[k]!;
    if (!ALLNAMES.has(s)) continue;
    let t: Mod | null = null;
    if (k === 0) {
      if (lead) continue;
      t = R.lookup(m, s);
    } else {
      let p: Mod | null;
      p = R.resolvePath(m, segs.slice(0, k), lead);
      if (!p) continue;
      t = R.lookup(p, s);
    }
    if (NEWNAMES.has(s) && (!t || !t.spec)) broken.push(`${file.slice(root.length + 1).slice(-80)} ${segs.join('::')} k=${k} ${t ? 'other' : 'unresolved'}`);
    if (!t) continue;
    if (t.spec && t.spec.old === s) add(file, offs[k]!, s, t.spec.new, why);
    else { const o = other.get(file) ?? new Set(); o.add(offs[k]![0]); other.set(file, o); }
  }
};
for (const [decl, mods] of g.modsByDecl) {
  const hasChains = false;
}
let pi = 0;
for (const [file, p] of g.parsedCache) {
  if (!p) continue;
  if (++pi % 3000 === 0) console.error('eval', pi, Date.now() - t0, 'ms edits', edits.size);
  for (const d of p.decls) {
    const mods = g.modsByDecl.get(d) ?? [];
    for (const u of d.uses) {
      if (!u.segs.some((x) => OLD.has(x))) continue;
      for (const m of mods) evalPath(m, u.segs, u.offs, u.lead, file, "use");
    }
  }
  for (const c of p.chains) {
    const mods = g.modsByDecl.get(c.mod) ?? [];
    for (const m of mods) evalPath(m, c.segs, c.offs, c.lead, file, "path");
  }
}
for (const m of specMods) if (m.site) add(m.site.file, m.site.nameOff, m.spec!.old, m.spec!.new, "decl");
for (const [f, o] of other) { const e = edits.get(f); if (!e) continue; for (const a of o) if (e.has(a)) conflicts.push(`${f}@${a}: renamed in one instance, other target in another`); }
let n = 0;
const out: Record<string, Ed[]> = {};
for (const [f, m] of edits) { out[f] = [...m.values()].sort((x, y) => x.a - y.a); n += m.size; }
writeFileSync(C + "/🗑️generated/rmod-plan.json", JSON.stringify(out, null, 1));
const fam: Record<string, Set<string>> = {};
for (const [f, es] of Object.entries(out)) for (const e of es) {
  const key = `${e.old}->${e.nw}`;
  for (const m of g.modsByFile.get(f) ?? []) (fam[key] ??= new Set()).add(m.crate.crate.name);
}
writeFileSync(C + "/🗑️generated/rmod-families.json", JSON.stringify(Object.fromEntries(Object.entries(fam).map(([k, v]) => [k, [...v].sort()])), null, 1));
console.error(`edits ${n} in ${edits.size} files; conflicts ${conflicts.length}; newname segments not resolving to their module ${broken.length}`);
writeFileSync(C + "/🗑️generated/rmod-broken.txt", [...new Set(broken)].join("\n"));
for (const c of conflicts.slice(0, 40)) console.error("CONFLICT", c);
