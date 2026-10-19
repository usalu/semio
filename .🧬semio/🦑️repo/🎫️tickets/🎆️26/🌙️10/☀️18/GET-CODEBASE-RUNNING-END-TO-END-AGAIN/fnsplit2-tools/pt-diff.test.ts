import { test, expect } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const liveFile = process.env.PT_REAL_FILE!;
const oldFile = process.env.PT_OLD_FILE!;
const work = process.env.PT_WORK!;

const oldMod = await import(oldFile);
const newMod = await import(liveFile);
const norm = (v: unknown): string => JSON.stringify(v, (_k, x) => (x instanceof RegExp ? `/${x.source}/${x.flags}` : x instanceof Map ? [...x] : x instanceof Set ? [...x] : typeof x === "bigint" ? String(x) : x));
const run = (mod: any, rel: string): string => { try { return "ok:" + norm(mod.loadTaxonomy({ repoRoot, taxonomyPath: rel })); } catch (e) { return "err:" + (e instanceof Error ? e.message : String(e)); } };
let seed = 987654321;
const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
const paths = (v: any, base: (string | number)[] = [], out: (string | number)[][] = []): (string | number)[][] => { if (v && typeof v === "object") { for (const k of Object.keys(v)) { out.push([...base, k]); paths(v[k], [...base, k], out); } } return out; };
test("old and new loadTaxonomy agree on the head taxonomy and on mutations", () => {
  const base = JSON.parse(fs.readFileSync(path.join(work, "head-taxonomy.json"), "utf8"));
  const rel0 = path.relative(repoRoot, path.join(work, "variant-0.json"));
  fs.writeFileSync(path.join(work, "variant-0.json"), JSON.stringify(base));
  const a0 = run(oldMod, rel0), b0 = run(newMod, rel0);
  expect(b0).toBe(a0);
  const all = paths(base);
  let errors = 0, oks = a0.startsWith("ok:") ? 1 : 0;
  for (let i = 1; i <= 150; i += 1) {
    const copy = structuredClone(base);
    const p = all[Math.floor(rnd() * all.length)]!;
    let parent: any = copy;
    for (const k of p.slice(0, -1)) parent = parent[k];
    const key = p[p.length - 1]!;
    const mode = Math.floor(rnd() * 3);
    if (mode === 0) delete parent[key]; else if (mode === 1) parent[key] = null; else parent[key] = typeof parent[key] === "string" ? parent[key] + "x" : typeof parent[key] === "number" ? parent[key] + 1 : []; 
    const file = path.join(work, `variant-${i}.json`);
    fs.writeFileSync(file, JSON.stringify(copy));
    const rel = path.relative(repoRoot, file);
    const a = run(oldMod, rel), b = run(newMod, rel);
    expect(b).toBe(a);
    if (a.startsWith("err:")) errors += 1; else oks += 1;
  }
  expect(errors).toBeGreaterThan(20);
});
