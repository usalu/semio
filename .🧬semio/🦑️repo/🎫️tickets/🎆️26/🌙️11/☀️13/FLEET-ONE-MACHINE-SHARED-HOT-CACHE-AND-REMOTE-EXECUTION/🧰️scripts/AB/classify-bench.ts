import {readFileSync} from "node:fs";
const B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(B+"🕸️graph/🕰️generations/🟨️.mjs");
const {relativeFactPathV1,rowFilePathV1,installCoveredV1,resolveRowDoubtfulV1}=await import(B+"🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs");
const {GRAPH_HOOKS_POLICY_V1}=await import(B+"🏡️resident/🕰️graph-hooks/🟨️.mjs");
const {manifestJournalPortV1}=await import(B+"🪪️graph-state/📍️cursor/🟦️.ts");
const root="/Users/Shared/semio",dir=root+"/.cache/b/g/m/generations";
const pointer=JSON.parse(readFileSync(dir+"/current.json","utf8")),manifest=JSON.parse(readFileSync(dir+"/m/"+pointer.manifest+".json","utf8"));
const index=G.openPackedFactIndexV1(readFileSync(dir+"/x/"+manifest.factIndex+".facts.br"),512*1048576,"t");
const port=manifestJournalPortV1({root});const cur=await port.cursor();await port.since(cur!);const excluded=port.excluded();
const TRUSTED=new Set(GRAPH_HOOKS_POLICY_V1.trustedKinds),FILE=new Set(GRAPH_HOOKS_POLICY_V1.fileRowKinds);
const empty={has:(_:string)=>false};
const old=(row:any)=>{
 if(row.kind==="resolve")return resolveRowDoubtfulV1(root,row,empty,excluded,true)?1:0;
 const filed=FILE.has(row.kind);if(!filed&&!TRUSTED.has(row.kind))return 1;
 const path=filed?rowFilePathV1(root,row.path):relativeFactPathV1(root,row.path);if(path===undefined)return 1;
 if(installCoveredV1(path))return 0;if(empty.has(path))return 1;return excluded(path)?2:0;
};
const n=index.rows.length;
let t=performance.now();const a=new Uint8Array(n);for(let i=0;i<n;i++)a[i]=old(index.rows[i]);console.log("old ms",Math.round(performance.now()-t));
const prefix=root+"/";
const fast=(()=>{const memo=new Map<string,number>();return(row:any)=>{
 const k=row.kind;
 if(k==="resolve")return resolveRowDoubtfulV1(root,row,empty,excluded,true)?1:0;
 if(!FILE.has(k)&&!TRUSTED.has(k))return 1;
 let v=memo.get(row.path);
 if(v===undefined){
  const p=row.path;let rel:string|undefined;
  if(p.startsWith(prefix))rel=p.slice(prefix.length);else if(p===root)rel="";else if(FILE.has(k)&&!p.startsWith("/")&&!p.startsWith("..")&&!p.includes("\\")&&!/^[A-Za-z]:/.test(p))rel=p;else rel=FILE.has(k)?rowFilePathV1(root,p):relativeFactPathV1(root,p);
  v=rel===undefined?1:installCoveredV1(rel)?0:excluded(rel)?2:0;memo.set(p,v);
 }
 return v;};})();
const memoOnly=(()=>{const memo=new Map<string,number>(),fm=new Map<string,number>();return(row:any)=>{
 const k=row.kind;if(k==="resolve")return resolveRowDoubtfulV1(root,row,empty,excluded,true)?1:0;
 const filed=FILE.has(k);if(!filed&&!TRUSTED.has(k))return 1;
 const m=filed?fm:memo;let v=m.get(row.path);
 if(v===undefined){const rel=filed?rowFilePathV1(root,row.path):relativeFactPathV1(root,row.path);v=rel===undefined?1:installCoveredV1(rel)?0:excluded(rel)?2:0;m.set(row.path,v);}
 return v;};})();
t=performance.now();const c=new Uint8Array(n);for(let i=0;i<n;i++)c[i]=memoOnly(index.rows[i]);console.log("memo-only ms",Math.round(performance.now()-t));
t=performance.now();const b=new Uint8Array(n);for(let i=0;i<n;i++)b[i]=fast(index.rows[i]);console.log("fast ms",Math.round(performance.now()-t));
let diff=0;for(let i=0;i<n;i++)if(a[i]!==b[i])diff++;console.log("diff",diff);
t=performance.now();let x=0;for(let i=0;i<n;i++)x+=index.rows[i].path.length;console.log("materialise ms",Math.round(performance.now()-t));
