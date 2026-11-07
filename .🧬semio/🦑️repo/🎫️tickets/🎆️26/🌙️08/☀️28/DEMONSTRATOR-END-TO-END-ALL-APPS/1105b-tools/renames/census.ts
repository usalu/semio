import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {join,dirname,normalize} from "node:path";
const R="/Users/ueli/Documents/semio",SKIP=new Set(["node_modules","target",".git","🗑️generated","dist",".🧬semio"]);
const mans:string[]=[];
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(SKIP.has(e.name)||e.name.startsWith(".cache"))continue;walk(join(d,e.name));}else if(e.name==="Cargo.toml")mans.push(join(d,e.name));}};
walk(R);
const data=new Map<string,any>();
for(const m of mans){try{data.set(m,Bun.TOML.parse(readFileSync(m,"utf8")));}catch(e){console.log("PARSEFAIL",m,String(e).slice(0,100));}}
const pkg=new Map<string,string>();
for(const[m,t]of data){const n=t.package?.name;if(n)pkg.set(dirname(m),n);}
const roots=new Set([...data].filter(([,t])=>t.workspace).map(([m])=>m));
const memberSets=new Map<string,Set<string>>();
for(const r of roots){const s=new Set<string>();for(const x of data.get(r).workspace?.members??[])s.add(normalize(join(dirname(r),x)));memberSets.set(r,s);}
const rootOf=(m:string)=>{const ex=data.get(m)?.package?.workspace;if(typeof ex==="string")return normalize(join(dirname(m),ex,"Cargo.toml"));let d=dirname(m);const own=dirname(m);const cands:string[]=[];while(d.startsWith(R)){const c=join(d,"Cargo.toml");if(roots.has(c))cands.push(c);d=dirname(d);}
 for(const c of cands){if(c===m)continue;const ms=memberSets.get(c)!;if(ms.has(own))return c;}
 for(const c of cands){if(c===m)continue;const ms=memberSets.get(c)!;for(const x of ms)if(x.endsWith("/*")&&normalize(x.slice(0,-2))===dirname(own))return c;}
 return cands.find(c=>c!==m);};
const kinds=["dependencies","dev-dependencies","build-dependencies"];
const tables=(t:any):[string,any][]=>{const o:[string,any][]=[];for(const k of kinds)if(t[k])o.push([k,t[k]]);for(const[tg,tv]of Object.entries<any>(t.target??{}))for(const k of kinds)if(tv[k])o.push([tg+"/"+k,tv[k]]);if(t.workspace?.dependencies)o.push(["ws",t.workspace.dependencies]);return o;};
const issues:any[]=[];
for(const[m,t]of data)for(const[tn,tab]of tables(t))for(const[key,v]of Object.entries<any>(tab)){
 let real=key,path:string|undefined;
 if(v&&typeof v==="object"){real=v.package??key;path=v.path;
  if(v.workspace){const r=rootOf(m),w=r?data.get(r).workspace?.dependencies?.[key]:undefined;if(w===undefined)issues.push(["WS-MISSING-KEY",m,key,""]);continue;}}
 if(path){const tgt=normalize(join(dirname(m),path)),n=pkg.get(tgt);
  if(n===undefined)issues.push(["PATH-NO-PKG",m,key,tgt]);else if(n!==real)issues.push(["NAME-MISMATCH",m,key+"|"+real,n]);}
}
writeFileSync("census.json",JSON.stringify(issues));
const cnt=(a:string[])=>{const c=new Map<string,number>();for(const x of a)c.set(x,(c.get(x)??0)+1);return [...c].sort((a,b)=>b[1]-a[1]);};
console.log(mans.length,issues.length,cnt(issues.map(i=>i[0])));
console.log(cnt(issues.filter(i=>i[0]==="NAME-MISMATCH").map(i=>i[2]+" => "+i[3])).slice(0,60));
console.log(cnt(issues.filter(i=>i[0]==="WS-MISSING-KEY").map(i=>i[2])).slice(0,60));
console.log(issues.filter(i=>i[0]==="PATH-NO-PKG").slice(0,10));
