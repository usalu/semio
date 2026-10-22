import {closure} from "./closure.ts";import fs from "node:fs";import path from "node:path";
const [entry,...rest]=process.argv.slice(2);const c=closure(path.resolve(entry),{dynamic:rest.includes("--dynamic")});
const rows=[...c.json].map(p=>({p,b:fs.existsSync(p)?fs.statSync(p).size:0})).sort((a,b)=>b.b-a.b);
console.log(`json files=${rows.length} bytes=${rows.reduce((a,r)=>a+r.b,0)}`);for(const r of rows.slice(0,8))console.log(String(r.b).padStart(9),r.p.split("/").slice(-4).join("/"));
