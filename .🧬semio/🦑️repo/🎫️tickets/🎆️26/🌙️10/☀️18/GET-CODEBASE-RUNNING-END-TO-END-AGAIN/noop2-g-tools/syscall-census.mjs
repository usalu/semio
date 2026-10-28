import {createRequire} from "node:module";
const req=createRequire(import.meta.url),stats=new Map();
const sites=new Map();const wrap=(m,spec)=>{for(const k of Object.keys(m)){const f=m[k];if(typeof f!=="function"||!/^(lstat|stat|readFile|readdir|realpath|readlink|open|opendir|access|copyFile|lstatSync|statSync|readFileSync|readdirSync|realpathSync|openSync|existsSync|readlinkSync)$/.test(k))continue;
 try{m[k]=function(...a){const t=performance.now();if(k==='lstat'){const st=new Error().stack.split('\n').slice(2,6).map(x=>x.trim().replace(/\(.*[\\/]/,'(').replace(/^at /,'')).join(' < ');sites.set(st,(sites.get(st)??0)+1);}const r=f.apply(this,a);const n=spec+"."+k,s=stats.get(n)??{c:0,ms:0};s.c++;if(r&&typeof r.then==="function"){return r.then(v=>{s.ms+=performance.now()-t;stats.set(n,s);return v},e=>{s.ms+=performance.now()-t;stats.set(n,s);throw e})}s.ms+=performance.now()-t;stats.set(n,s);return r}}catch{}}};
wrap(req("node:fs"),"fs");wrap(req("node:fs/promises"),"fsp");
const {resolve}=await import("node:path");
const root=resolve(import.meta.dirname,"../../../../../../../..");const entry=process.argv[2];
const mod=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"));
const t=performance.now();const cpu=process.cpuUsage();
const c=await mod.captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});
console.log("capture",Math.round(performance.now()-t),"ms files",c.manifest.sources.length,JSON.stringify(process.cpuUsage(cpu)));
for(const [k,v] of [...stats].sort((a,b)=>b[1].ms-a[1].ms))console.log(k.padEnd(14),String(v.c).padStart(7),Math.round(v.ms),"ms");

console.log('--- lstat call sites');for(const [k,v] of [...sites].sort((a,b)=>b[1]-a[1]).slice(0,14))console.log(String(v).padStart(7),k.slice(0,260));
