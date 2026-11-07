import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
const root="/Users/ueli/Documents/semio",PRE=Number(process.argv[2]??80);
const files=execFileSync("git",["ls-files","-z","--","*Cargo.toml"],{cwd:root,maxBuffer:1<<28}).toString().split("\0").filter(Boolean);
const pk=new Map<string,Set<string>>(),mid=new Map<string,any>();const want=new Set<string>(JSON.parse(readFileSync(import.meta.dir+"/ids.json","utf8")));
for(const f of files){let t:any;try{t=(Bun as any).TOML.parse(readFileSync(root+"/"+f,"utf8"))}catch{continue}
 if(!t.package?.name)continue;const d=new Set<string>();
 const add=(o:any)=>{for(const[k,v]of Object.entries(o??{})){if((v as any)?.optional===true)continue;d.add((v as any)?.package??k)}};
 add(t.dependencies);add(t["build-dependencies"]);for(const[tk,tv]of Object.entries(t.target??{}) as any[]){if(/not\(.*wasm|unix|windows|macos|linux/.test(tk))continue;{add(tv.dependencies);add(tv["build-dependencies"])}}
 pk.set(t.package.name,d);mid.set(t.package.name,String(t["package"]?.metadata?.component?.package??"").replace(/^semio:/,""))}
const comps=[...pk.keys()].filter(n=>n.endsWith("-component")&&want.has(mid.get(n)));
const seen=new Map<string,string>();const st:[string,string][]=comps.map(c=>[c,c]);
while(st.length){const[n,s]=st.pop()!;if(seen.has(n)||!pk.has(n))continue;seen.set(n,s);for(const d of pk.get(n)!)st.push([d,s])}
const plen=(n:string)=>PRE+30+n.length+1+16+8+n.length+1+16+5;
const rows=[...seen].map(([n,s])=>[plen(n),n,s] as const).sort((a,b)=>b[0]-a[0]);
console.log("components",comps.length,"closure",seen.size,"prefix",PRE);
for(const r of rows.slice(0,8))console.log(...r);
console.log("over256",rows.filter(r=>r[0]>256).length);
for(const r of rows.filter(r=>r[0]>256))console.log("OVER",...r);
import {existsSync} from "node:fs";
const dirs=new Map<string,string>();
for(const f of files){let t:any;try{t=(Bun as any).TOML.parse(readFileSync(root+"/"+f,"utf8"))}catch{continue} if(t.package?.name)dirs.set(t.package.name,f.replace(/Cargo\.toml$/,""))}
const bs=[...seen.keys()].filter(n=>{const d=dirs.get(n)!;return existsSync(root+"/"+d+"build.rs")});
console.log("closure crates with build.rs",bs.length);
const bsRows=bs.map(n=>[PRE+30+n.length+1+16+8+n.length+1+16+5,n] as const).sort((a,b)=>b[0]-a[0]);
for(const r of bsRows.slice(0,6))console.log("BS",...r);
console.log("build.rs over256",bsRows.filter(r=>r[0]>256).length);
const dep=[...seen.keys()].map(n=>[PRE+24+"deps/lib".length+n.length+1+16+5,n] as const).sort((a,b)=>b[0]-a[0]);
console.log("deps rlib worst",...dep[0]);
const fp=[...seen.keys()].map(n=>[143+2*n.length,n] as const).sort((a,b)=>b[0]-a[0]);
console.log("fingerprint worst",...fp[0],"over256",fp.filter(r=>r[0]>256).length);
