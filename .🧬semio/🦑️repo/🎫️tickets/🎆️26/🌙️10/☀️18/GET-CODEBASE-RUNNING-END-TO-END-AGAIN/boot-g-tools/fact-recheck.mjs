// fact-recheck.mjs <generations-dir> <factIndexDigest>: classify rows into journal-vouchable vs recheck (excluded/outside mention) and time the engine's observationsHold on the recheck subset (read-only on the live tree)
import {readFileSync} from "node:fs";
import {join} from "node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const {openPackedFactIndexV1}=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const obs=await import(L+"👁️observation/🟨️.mjs");
const [dir,digest]=process.argv.slice(2);
const index=openPackedFactIndexV1(readFileSync(join(dir,"x",digest+".facts.br")),1<<30);
const root="/Users/ueli/Documents/semio";
const exclRe=/\/(\.git|\.cache|node_modules|target|\.nx|dist|build|out)(\/|$|")/;
const rows=[];let vouch=0;const byKind=new Map();
for(let i=0;i<index.rows.length;i++){
  const r=index.rows[i];
  const text=r.kind+"\0"+r.path+"\0"+(typeof r.fact==="string"?r.fact:JSON.stringify(r.fact));
  const full=(r.path.startsWith("/")?r.path:root+"/"+r.path);
  const isRe= exclRe.test(full) || (typeof r.fact==="string"&&exclRe.test(r.fact)) || (r.kind==="resolve"||r.kind==="typescriptRealpath"||r.kind==="registry"||r.kind==="closure"||r.kind==="derived") && exclRe.test(text) || (r.path.startsWith("/") && !r.path.startsWith(root) && !r.path.startsWith("["));
  if(isRe){rows.push(r);byKind.set(r.kind,(byKind.get(r.kind)??0)+1);} else vouch++;
}
console.log("vouchable",vouch,"recheck",rows.length,[...byKind].sort((a,b)=>b[1]-a[1]).slice(0,12).map(x=>x.join(":")).join(" "));
const sub=rows.filter(r=>!["closure","derived"].includes(r.kind)&&(r.kind in {}||true));
const t0=performance.now(),c0=process.cpuUsage();
let okN=0,bad=[];for(let i=0;i<sub.length;i+=500){const chunk=sub.slice(i,i+500);const good=await obs.observationsHold(chunk,64,new Set(),true,undefined,row=>bad.push(row));if(good)okN+=chunk.length;}const ok=okN+'/'+sub.length;console.log('stale chunks sample',JSON.stringify(bad.slice(0,4)).slice(0,600));
const c1=process.cpuUsage(c0);
console.log("observationsHold on",sub.length,"rows ->",ok,"wall ms",Math.round(performance.now()-t0),"cpu ms",Math.round((c1.user+c1.system)/1000));
