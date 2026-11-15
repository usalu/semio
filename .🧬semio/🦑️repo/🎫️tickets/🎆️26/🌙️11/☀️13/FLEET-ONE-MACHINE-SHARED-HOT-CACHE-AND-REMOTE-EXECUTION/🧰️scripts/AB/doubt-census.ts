import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {openFactIndexV1}=await import(B+"🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs");
const {relativeFactPathV1,resolveRowDoubtfulV1,installCoveredV1}=await import(B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs");
const {GRAPH_HOOKS_POLICY_V1}=await import(B+"🏡️resident/🕰️graph-hooks/🟨️.mjs");
const {manifestJournalPortV1}=await import(B+"🪪️graph-state/📍️cursor/🟦️.ts");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+(pointer.manifest??pointer.parent)+".json","utf8"));
const raw=brotliDecompressSync(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),{maxOutputLength:2**31});
const index:any=openFactIndexV1(raw,{maximumBytes:raw.length});
const t0=Date.now();
const port=manifestJournalPortV1({root});
const cur=await port.cursor();const ans:any=await port.since(cur!);
console.log("since",ans.complete,ans.rows?.length,"ms",Date.now()-t0);
const excluded=port.excluded(),TRUSTED=new Set(GRAPH_HOOKS_POLICY_V1.trustedKinds);
const empty={has:()=>false};
const hist=new Map<string,number>();let tEx=0,nEx=0;const exT=(p:string)=>{const a=performance.now();const r=excluded(p);tEx+=performance.now()-a;nEx++;return r;};const T0=performance.now();let tRel=0;
for(let id=0;id<index.rows.length;id++){
 const row=index.rows[id];let lvl=0,why="";
 if(row.kind===GRAPH_HOOKS_POLICY_V1.resolveKind){lvl=resolveRowDoubtfulV1(root,row,empty,excluded,true)?1:0;why="resolve";}
 else if(!TRUSTED.has(row.kind)){lvl=1;why="untrusted-kind";}
 else{const p=relativeFactPathV1(root,row.path);if(p===undefined){lvl=1;why="outside";}else if(installCoveredV1(p)){lvl=0;}else if(exT(p)){lvl=2;why="excluded";}}
 if(lvl){const k=row.kind+"/"+why+"/"+lvl;hist.set(k,(hist.get(k)??0)+1);}
}
console.log("classify total ms",Math.round(performance.now()-T0),"excluded calls",nEx,"excluded ms",Math.round(tEx));console.log("rows",index.rows.length,[...hist].sort((a,b)=>b[1]-a[1]));
const ex=new Map<string,number>();
for(let id=0;id<index.rows.length;id++){const row=index.rows[id];if(!TRUSTED.has(row.kind))continue;const p=relativeFactPathV1(root,row.path);if(p!==undefined&&!installCoveredV1(p)&&excluded(p)){const k=row.kind+" "+p.split("/").slice(0,3).join("/");ex.set(k,(ex.get(k)??0)+1);}}
console.log([...ex].sort((a,b)=>b[1]-a[1]).slice(0,25));
console.log("trusted kinds",[...TRUSTED]);
{
 const distinct=new Set<string>(),byKind=new Map<string,Set<string>>();
 for(let id=0;id<index.rows.length;id++){const r=index.rows[id];distinct.add(r.path);let s=byKind.get(r.kind);if(!s)byKind.set(r.kind,s=new Set());s.add(r.path);}
 console.log("rows",index.rows.length,"distinct paths",distinct.size);
 console.log([...byKind].map(([k,s])=>k+":"+s.size).join(" "));
 const t=performance.now();const memo=new Map<string,number>();let hits=0;
 for(let id=0;id<index.rows.length;id++){const r=index.rows[id];let v=memo.get(r.path);if(v===undefined){const p=relativeFactPathV1(root,r.path);v=p===undefined?1:(installCoveredV1(p)?0:excluded(p)?2:index.rows?0:0);memo.set(r.path,v);}else hits++;}
 console.log("memoised pass ms",Math.round(performance.now()-t),"hits",hits);
 const t2=performance.now();let n=0;for(let id=0;id<index.rows.length;id++){const r=index.rows[id];n+=r.path.length;}console.log("row materialisation only ms",Math.round(performance.now()-t2));
}
