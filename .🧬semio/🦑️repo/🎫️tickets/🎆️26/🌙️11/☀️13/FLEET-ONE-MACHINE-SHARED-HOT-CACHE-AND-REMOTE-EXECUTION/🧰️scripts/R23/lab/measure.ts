import {readFileSync,readdirSync,writeFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {REAPI_V1,decodeProtobufV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {hostOfRootV1,neutralRecordV1} from "./ov/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
const base="/Users/Shared/semio/.cache/f/cache-server/v1/main/ac",cas=process.env.HOME+"/.cache/semio/cas/v1/o/";
const recs:any[]=[];
for(const a of readdirSync(base))for(const b of readdirSync(base+"/"+a)){
 try{
  const r=decodeProtobufV1(REAPI_V1.actionResult,readFileSync(`${base}/${a}/${b}`)) as any,note=JSON.parse(Buffer.from(r.stdout_raw).toString());
  if(note.kind!=="project-inference")continue;
  const doc=note.documents.find((d:any)=>d.name==="record.br");if(!doc)continue;
  const rec=JSON.parse(brotliDecompressSync(readFileSync(cas+doc.hash.slice(0,2)+"/"+doc.hash.slice(2))).toString());
  let img:any;try{img=JSON.parse(rec.image);}catch{continue;}
  recs.push({platform:note.platform,t:note.createdMs,name:img.name,root:img.root,rec});
 }catch{}
}
const latest=new Map<string,any>();
for(const r of recs){const k=r.platform+"|"+r.root;const p=latest.get(k);if(!p||r.t>p.t)latest.set(k,r);}
const winRecs=[...latest.values()].filter(r=>r.platform==="win32-x64"),posix=new Map<string,any>();
for(const r of latest.values()){if(r.platform==="win32-x64")continue;const p=posix.get(r.root);if(!p||r.t>p.t)posix.set(r.root,r);}
const walk=(v:any,f:(s:string)=>string):any=>typeof v==="string"?f(v):Array.isArray(v)?v.map(x=>walk(x,f)):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,x])=>[k,walk(x,f)])):v;
const winSpell=(s:string)=>s.replaceAll("⟦semio-root-json⟧","C:\\semio").replaceAll("⟦semio-root-slash⟧","C:/semio").replaceAll("⟦semio-root⟧","C:\\semio");
const posSpell=(root:string)=>(s:string)=>s.replaceAll("⟦semio-root⟧",root).replaceAll("⟦generation⟧","86415d695aa883fffc22");
const unspell=(rec:any,f:(s:string)=>string)=>({digest:rec.digest,authority:walk(rec.authority,f),observations:walk(rec.observations,f),image:f(rec.image)});
const ROOTS=["/Users/Shared/semio","/Users/ueli/Documents/semio","/home/deck/semio","/home/deck/Documents/semio","/home/deck/Dokumente/semio"];
const VOL=new Set(["mtimeMs","ctimeMs","ino"]);
const stable=(v:any):any=>Array.isArray(v)?v.map(stable):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).filter(([k])=>!VOL.has(k)).map(([k,x])=>[k,stable(x)])):v;
const rows=(rec:any)=>rec.observations.filter((r:any)=>!(r.kind==="lstat"&&r.fact==="directory"&&r.path!=="⟦semio-root⟧")).map((r:any)=>JSON.stringify(stable(r))).sort();
const ex:any={};const out:any={windows:winRecs.length,pairs:0,refusedWindows:0,refusedPosix:0,noRoot:0,equal:0,rowsDiffer:0,imageDiffer:0,kinds:{} as any,names:{} as any};
for(const w of winRecs){
 const p=posix.get(w.root);if(!p)continue;out.pairs++;
 let nw:any;try{nw=neutralRecordV1(unspell(w.rec,winSpell) as any,hostOfRootV1("C:\\semio")).record;}catch(e){out.refusedWindows++;out.names[w.name]="win refused";continue;}
 let np:any,pr="";for(const root of ROOTS){try{np=neutralRecordV1(unspell(p.rec,posSpell(root)) as any,hostOfRootV1(root)).record;pr=root;break;}catch{}}
 if(np===undefined){out.noRoot++;continue;}
 const same_img=JSON.stringify(stable(JSON.parse(nw.image)))===JSON.stringify(stable(JSON.parse(np.image)));
 const rw=rows(nw),rp=rows(np),sw=new Set(rw),sp=new Set(rp),diff=[...rw.filter(x=>!sp.has(x)),...rp.filter(x=>!sw.has(x))];
 if(same_img&&diff.length===0)out.equal++;
 else{const ks=[...new Set(diff.map(d=>JSON.parse(d).kind))].sort().join('+')||'image';out.sets??={};out.sets[ks]=(out.sets[ks]??0)+1;(out.setEx??={})[ks]??=w.name;if(!same_img)out.imageDiffer++;if(diff.length){out.rowsDiffer++;for(const d of diff){const k=JSON.parse(d).kind;if(!ex[k]||ex[k].length<3)(ex[k]??=[]).push((rw.includes(d)?'W ':'P ')+d.slice(0,260));out.kinds[k]=(out.kinds[k]??0)+1;}}out.names[w.name]=(same_img?"":"image ")+(diff.length?"rows "+diff.slice(0,1).map(d=>d.slice(0,160)):"");}
}
writeFileSync("measure-ex.json",JSON.stringify(ex,null,1));writeFileSync("measure.json",JSON.stringify(out,null,1));
console.log(JSON.stringify({...out,names:Object.keys(out.names).length}));
