import {readFileSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(B+"🕸️graph/🕰️generations/🟨️.mjs");
const {relativeFactPathV1}=await import(B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs");
const {manifestJournalPortV1,snapshotDirectoriesV1}=await import(B+"🪪️graph-state/📍️cursor/🟦️.ts");
const {classifyGraphInputV1}=await import(B+"🪪️graph-state/🟦️.ts");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const index:any=G.openPackedFactIndexV1(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),512*1048576,"t");
const used=new Uint8Array(index.stringCount);for(let id=0;id<index.rows.length;id++)used[index.rowPathIdOf(id)]=1;
const rels:string[]=[];for(let i=0;i<used.length;i++)if(used[i]){const p=relativeFactPathV1(root,index.textOf(i));if(p!==undefined)rels.push(p);}
console.log("distinct workspace paths",rels.length);
const port=manifestJournalPortV1({root});const cur=await port.cursor();await port.since(cur!);const excluded=port.excluded();
let t=performance.now();const a=rels.map(p=>excluded(p));console.log("old excluded ms",Math.round(performance.now()-t),"true",a.filter(Boolean).length);
// fast: ancestor-ignored shortcut using a fresh port instance (cold memo)
const port2=manifestJournalPortV1({root});const c2=await port2.cursor();await port2.since(c2!);const ex2=port2.excluded();
const ignoredMemo=new Map<string,boolean>();
const parentOf=(p:string)=>{const c=p.lastIndexOf("/");return c<0?"":p.slice(0,c);};
// need ex2 to expose "ignored" vs "unknown": approximate by testing the parent with ex2 and requiring that parent is not in known (ex2 true implies not known)
const fast=(p:string):boolean=>{
 const parent=parentOf(p);
 if(parent!==""&&ex2(parent)&&ignoredMemo.get(parent)===true)return true;
 const v=ex2(p);ignoredMemo.set(p,v);return v;
};
t=performance.now();const b=rels.map(p=>fast(p));console.log("fast ms",Math.round(performance.now()-t));
let diff=0;for(let i=0;i<a.length;i++)if(a[i]!==b[i])diff++;console.log("diff",diff);
{
 const {readFile}=await import("node:fs/promises");
 const {compileIgnoreV1,refusePathV1,sharedV1}=await import("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚧️boundary/🟦️.ts");
 const text=await readFile(root+"/.gitignore","utf8"),ignore=compileIgnoreV1(text);
 const sample=rels.slice(0,100000);
 let t0=performance.now();let k=0;for(const p of sample)if(classifyGraphInputV1(p)==="ignored")k++;console.log("classify x",sample.length,Math.round(performance.now()-t0),"ms ignored",k);
 t0=performance.now();k=0;for(const p of sample)if(refusePathV1(p)===undefined&&sharedV1(p,true,ignore))k++;console.log("shared x",sample.length,Math.round(performance.now()-t0),"ms shared",k);
}
{
 const {readFile}=await import("node:fs/promises");
 const {compileIgnoreV1,refusePathV1,sharedV1}=await import("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🚧️boundary/🟦️.ts");
 const ignore=compileIgnoreV1(await readFile(root+"/.gitignore","utf8"));
 const shared=(path:string)=>refusePathV1(path)===undefined&&sharedV1(path,true,ignore);
 const port3=manifestJournalPortV1({root});const c3=await port3.cursor();const a3:any=await port3.since(c3!);
 // known sets from a fresh source
 const src:any=await (await import(B+"🪪️graph-state/📍️cursor/🟦️.ts")).replicaSnapshotSourceV1(root);
 const snap=(await import(B+"🪪️graph-state/📍️cursor/🟦️.ts")).snapshotOfFilesV1(src.files);
 const files=new Set(snap.paths),dirs=snapshotDirectoriesV1(snap.paths);
 const memo=new Map<string,boolean>();let shortcut=0;
 const ex=(path:string):boolean=>{
  if(path===""||files.has(path)||dirs.has(path))return false;
  let v=memo.get(path);
  if(v===undefined){
   const cut=path.lastIndexOf("/"),parent=cut<0?"":path.slice(0,cut);
   if(parent!==""&&ex(parent)){v=true;shortcut++;}
   else v=classifyGraphInputV1(path)==="ignored"||!shared(path);
   memo.set(path,v);
  }
  return v;
 };
 let t0=performance.now();const r=rels.map(p=>ex(p));console.log("parent-shortcut ms",Math.round(performance.now()-t0),"shortcuts",shortcut,"memo",memo.size);
 let d=0;for(let i=0;i<r.length;i++)if(r[i]!==a[i])d++;console.log("diff vs old",d);
}
