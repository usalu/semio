import { writeFileSync } from "node:fs";
import { ALLNAMES, buildGraph, makeResolver, Mod } from "./graph.ts";
import { C } from "./lib.ts";
const label = process.argv[2]!;
const g = buildGraph(ALLNAMES);
const R = makeResolver(g);
const id = (m: Mod | null) => (m ? `${m.parsed.file}|${m.id}` : "-");
const out: string[] = [];
const files = [...g.parsedCache.entries()].filter(([, p]) => p).sort((a, b) => (a[0] < b[0] ? -1 : 1));
for (const [file, p] of files) {
  if (!p) continue;
  let ci = 0;
  let di = 0;
  for (const d of p.decls) {
    di++;
    const mods = g.modsByDecl.get(d) ?? [];
    let ui = 0;
    for (const u of d.uses) {
      ui++;
      if (!u.segs.some((x) => ALLNAMES.has(x))) continue;
      for (let k = 0; k < u.segs.length; k++) {
        if (!ALLNAMES.has(u.segs[k]!)) continue;
        for (const m of mods) {
          const t = k === 0 ? (u.lead ? null : R.lookup(m, u.segs[0]!)) : (() => { const pr = R.resolvePath(m, u.segs.slice(0, k), u.lead); return pr ? R.lookup(pr, u.segs[k]!) : null; })();
          out.push(`${file}#u${di}.${ui}.${k}@${m.id} => ${id(t)}`);
        }
      }
    }
  }
  for (const c of p.chains) {
    ci++;
    const mods = g.modsByDecl.get(c.mod) ?? [];
    for (let k = 0; k < c.segs.length; k++) {
      if (!ALLNAMES.has(c.segs[k]!)) continue;
      for (const m of mods) {
        const t = k === 0 ? (c.lead ? null : R.lookup(m, c.segs[0]!)) : (() => { const pr = R.resolvePath(m, c.segs.slice(0, k), c.lead); return pr ? R.lookup(pr, c.segs[k]!) : null; })();
        out.push(`${file}#c${ci}.${k}@${m.id} => ${id(t)}`);
      }
    }
  }
}
writeFileSync(`${C}/🗑️generated/rmod-verify-${label}.txt`, out.join("\n"));
console.error("lines", out.length, "mods", g.allMods.length);
