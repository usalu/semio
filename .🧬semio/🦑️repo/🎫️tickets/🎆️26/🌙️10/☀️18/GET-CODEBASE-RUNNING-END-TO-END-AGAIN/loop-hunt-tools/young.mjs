import fs from "node:fs";import {brotliDecompressSync} from "node:zlib";
const f=process.argv[2];
const text=brotliDecompressSync(fs.readFileSync(f)).toString("utf8"),nl=text.indexOf("\n"),h=JSON.parse(text.slice(0,nl));
const rows=h.observations.filter(r=>r.kind==="file"||r.kind==="analysis").map(r=>{const s=r.kind==="file"?r.fact:r.fact.source;return [Math.max(s?.mtimeMs??0,s?.ctimeMs??0),r.kind,r.path]}).sort((a,b)=>b[0]-a[0]);
console.log(fs.statSync(f).mtime.toISOString());
for(const [t,k,p] of rows.slice(0,8))console.log(new Date(t).toISOString(),k,p.slice(-110));
const auth=h.authorityObservations?.length;console.log("authority",auth);
