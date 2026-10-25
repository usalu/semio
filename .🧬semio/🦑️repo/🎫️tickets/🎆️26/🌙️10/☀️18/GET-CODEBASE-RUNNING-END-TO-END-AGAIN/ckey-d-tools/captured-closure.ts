import {readFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {dirname,resolve,join,relative} from "node:path";
const t=new Bun.Transpiler({loader:"ts"});
const LIT=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu,DECL=/\.d\.[cm]?ts$/u;
const value=new Map<string,string[]>();
const edges=(f:string):string[]=>{let k=value.get(f);if(k)return k;k=[];value.set(f,k);if(!/\.(ts|mts|mjs|tsx|js)$/u.test(f))return k;let imports:{path:string}[]=[];try{imports=t.scanImports(readFileSync(f,"utf8").replace(/^#!.*\n/u,""));}catch{return k;}for(const r of imports){if(!r.path.startsWith("."))continue;const x=resolve(dirname(f),r.path);if(existsSync(x)&&statSync(x).isFile())k.push(x);}return k;};
const closure=(roots:string[]):Set<string>=>{const seen=new Set<string>(),p=[...roots];while(p.length){const f=p.pop()!;if(seen.has(f))continue;seen.add(f);p.push(...edges(f));}return seen;};
const schemas=(f:string):string[]=>{const d=join(dirname(f),"🧬️schema");return existsSync(d)?readdirSync(d).filter(n=>n.endsWith(".json")).sort().map(n=>join(d,n)):[];};
export const captured=(entry:string,parentEntry:string):{files:Set<string>;why:Map<string,[string,string]>}=>{
 const A=closure([resolve(parentEntry)]);
 const seen=new Set<string>(),why=new Map<string,[string,string]>(),pending=[resolve(entry)];
 while(pending.length){const p=pending.pop()!;if(seen.has(p)||!A.has(p))continue;seen.add(p);if(p.endsWith(".json"))continue;
  const text=readFileSync(p,"utf8");const add=(x:string,k:string)=>{if(!seen.has(x)&&A.has(x)){if(!why.has(x))why.set(x,[p,k]);pending.push(x);}};
  for(const s of schemas(p))add(s,"schema");for(const x of edges(p))add(x,"value");
  for(const m of text.matchAll(LIT)){if(DECL.test(m[2]!))continue;const x=resolve(dirname(p),m[2]!);if(existsSync(x))add(x,"literal");}}
 return {files:seen,why};
};
if(import.meta.main){const [entry,parent]=process.argv.slice(2);const {files,why}=captured(entry!,parent!);console.log("captured",files.size,"parent value closure",closure([resolve(parent!)]).size);
 const by=new Map<string,number>();for(const [f,[p,k]] of why){if(k==="literal"||k==="schema"){const key=relative(process.cwd(),p).slice(-85)+" "+k;by.set(key,(by.get(key)??0)+1);}}
 for(const [k,v] of [...by].sort((a,b)=>b[1]-a[1]).slice(0,12))console.log(v,k);}
