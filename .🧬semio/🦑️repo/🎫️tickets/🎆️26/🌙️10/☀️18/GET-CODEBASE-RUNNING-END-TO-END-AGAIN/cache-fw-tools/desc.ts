import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
const cen = JSON.parse(readFileSync(join(import.meta.dir,"../🗑️generated/cache-fw-census-before.json"),"utf8"));
const cls = new Map<string,string>(); for (const t of cen.targets) cls.set(t.file+"|"+t.name, t.classProposal);
const want = process.argv.slice(2);
for (const f of projectFiles) { const j=readJson(f); for (const [n,t] of Object.entries<any>(j.targets??{})) { const c=cls.get(f+"|"+n); if(!c||(want.length&&!want.includes(c))) continue; const d=t.description ?? t.metadata?.description; if(d) console.log(c.slice(0,6), t.cache, j.name+":"+n, "|", String(d).replace(/\s+/g," ").slice(0,220)); } }
