import {readFileSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(B+"🕸️graph/🕰️generations/🟨️.mjs");
const {relativeFactPathV1,rowFilePathV1,installCoveredV1}=await import(B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs");
const {GRAPH_HOOKS_POLICY_V1}=await import(B+"🏡️resident/🕰️graph-hooks/🟨️.mjs");
const {manifestJournalPortV1}=await import(B+"🪪️graph-state/📍️cursor/🟦️.ts");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const index:any=G.openPackedFactIndexV1(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),512*1048576,"t");
console.log("has rowPathIdOf",typeof index.rowPathIdOf);
const port=manifestJournalPortV1({root});const cur=await port.cursor();await port.since(cur!);const excluded=port.excluded();
const TRUSTED=new Set(GRAPH_HOOKS_POLICY_V1.trustedKinds),FILE=new Set(GRAPH_HOOKS_POLICY_V1.fileRowKinds);
const n=index.rows.length;
let t=performance.now();
const tr=new Int8Array(index.stringCount).fill(-1),fl=new Int8Array(index.stringCount).fill(-1);const out=new Uint8Array(n);let decoded=0;
for(let id=0;id<n;id++){
 const kind=index.rowKindOf(id);if(kind==="resolve"){out[id]=1;continue;}
 const filed=FILE.has(kind);if(!filed&&!TRUSTED.has(kind)){out[id]=1;continue;}
 const table=filed?fl:tr,at=index.rowPathIdOf(id);let level=table[at];
 if(level<0){decoded++;const text=index.textOf(at);const p=filed?rowFilePathV1(root,text):relativeFactPathV1(root,text);level=p===undefined?1:installCoveredV1(p)?0:excluded(p)?2:0;table[at]=level;}
 out[id]=level;
}
console.log("ids classification ms",Math.round(performance.now()-t),"decoded",decoded);
t=performance.now();const sel=[];for(let i=0;i<n;i++)if(out[i])sel.push(i);console.log("kept",sel.length,Math.round(performance.now()-t));
{
 const used=new Uint8Array(index.stringCount);for(let id=0;id<n;id++)used[index.rowPathIdOf(id)]=1;
 let c=0;for(let i=0;i<used.length;i++)c+=used[i];
 let t0=performance.now();let len=0;for(let i=0;i<used.length;i++)if(used[i])len+=index.textOf(i).length;
 console.log("textOf for",c,"used strings ms",Math.round(performance.now()-t0));
 const {relativeFactPathV1:rel}=await import(B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs");
 const texts:string[]=[];for(let i=0;i<used.length;i++)if(used[i])texts.push(index.textOf(i));
 t0=performance.now();let k=0;for(const x of texts){const p=rel(root,x);if(p!==undefined&&excluded(p))k++;}
 console.log("level compute for",texts.length,"ms",Math.round(performance.now()-t0),k);
}
