import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const {openFactIndexV1}=await import(L+"🧮️fact-codec/🟨️.mjs");const {derivedOwnerNameV1,isDerivedRowV1}=await import(L+"✂️cutoff/🟨️.mjs");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maximumOutputLength:2**31} as never);
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const name=process.argv[2]!;const o=index.owners.indexOf(name);
const ownerRows=new Set<number>();const rowsOfOwner=new Map<number,Set<number>>();
for(const cls of index.classes){for(const ow of cls.o){let s=rowsOfOwner.get(ow);if(!s)rowsOfOwner.set(ow,s=new Set());for(const id of cls.r)s.add(id);}}
const mine=rowsOfOwner.get(o)!;
const derivedMine=[...mine].filter(id=>{const r=index.rows[id];return r.kind==="derived"||r.kind==="closure"});
const pseudo=new Map<string,number>();index.owners.forEach((n:string,i:number)=>pseudo.set(n,i));
let covered=new Set<number>();const stats:any[]=[];
for(const id of derivedMine){const r=index.rows[id];const pn=derivedOwnerNameV1(r);const pi=pseudo.get(pn);const sup=pi===undefined?undefined:rowsOfOwner.get(pi);if(sup)for(const s of sup)covered.add(s);stats.push([r.kind,(r.path as string).slice(0,70),pi===undefined?"no-pseudo":sup?.size]);}
console.log(stats.slice(0,14));
const toolAn=[...mine].filter(id=>index.rows[id].kind==="analysis");
const unc=toolAn.filter(id=>!covered.has(id));
console.log("owner rows",mine.size,"analysis",toolAn.length,"covered by held derived supports",toolAn.length-unc.length,"uncovered",unc.length);
const cmd=(id:number)=>index.rows[id].fact.provider.startsWith("build:command-imports");
const dirs=new Map<string,number>();for(const id of unc.filter(cmd)){const p=index.rows[id].path.replace(root+"/","");const k=p.split("/").slice(0,9).join("/");dirs.set(k,(dirs.get(k)??0)+1);}
console.log([...dirs].sort((a,b)=>b[1]-a[1]).slice(0,12));
const byKind=new Map<string,[number,number]>();
for(const id of mine){const r=index.rows[id];const k=r.kind;const e=byKind.get(k)??[0,0];e[0]++;if(covered.has(id))e[1]++;byKind.set(k,e);}
console.log("kind [total, covered]",[...byKind].sort((a,b)=>b[1][0]-a[1][0]));
const unc2=[...mine].filter(id=>!covered.has(id));const dd=new Map<string,number>();
for(const id of unc2){const r=index.rows[id];const p=String(r.path).replace(root+"/","").slice(0,90);dd.set(r.kind+" "+p.split("/").slice(0,6).join("/"),(dd.get(r.kind+" "+p.split("/").slice(0,6).join("/"))??0)+1);}
console.log([...dd].sort((a,b)=>b[1]-a[1]).slice(0,25));
