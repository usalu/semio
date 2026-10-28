import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {pathToFileURL} from "node:url";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href),prefixes=s.buildSchedulingPrefixesV1();
const g=".cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const base=p=>{const t=p.replace(/^\^?\{workspaceRoot\}\//,"");const i=t.search(/[*?{\[]/);return i<0?t:t.slice(0,i);};
const hits=p=>{const b=base(p);const glob=/[*?{\[]/.test(p);return prefixes.some(q=>b.startsWith(q)||(glob&&q.startsWith(b)));};
const rows={};let projects=0;
for(const [name,row] of Object.entries(man.owners)){
 if(row.kind!=="project")continue;let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
 projects++;const ni=frag.node?.namedInputs??{},root=frag.node?.root;
 let symbolic=false;const sens=(input,seen=new Set())=>{ if(input&&typeof input==="object"&&typeof input.closureRef==="string"){symbolic=true;return false;} if(typeof input!=="string")return false; if(input.startsWith("!"))return false; if(input.startsWith("{workspaceRoot}/")||input.startsWith("{projectRoot}"))return input.startsWith("{workspaceRoot}/")&&hits(input); if(input==="default"||input==="^default")return false; const key=input.replace(/^\^/,""); if(seen.has(key))return false; seen.add(key); const l=ni[key]; return Array.isArray(l)&&l.some(x=>sens(x,seen));};
 for(const [t,def] of Object.entries(frag.node?.targets??{})){
  if(def.cache!==true)continue;
  const head=t.split("-")[0];const excl=(def.inputs??[]).includes("schedulingSourceExclusions");
  symbolic=false;const sensitive=!excl&&(def.inputs??[]).some(i=>sens(i));
  const k=excl?"excluded":sensitive?"sensitive":symbolic?"symbolic":"clean";
  const r=rows[head]??={excluded:0,sensitive:0,clean:0,symbolic:0};r[k]++;
 }
}
const tot={excluded:0,sensitive:0,clean:0,symbolic:0};for(const r of Object.values(rows))for(const k in tot)tot[k]+=r[k];
console.log("projects",projects,"cached targets",tot.excluded+tot.sensitive+tot.clean,JSON.stringify(tot));
for(const [h,r] of Object.entries(rows).sort((a,b)=>b[1].sensitive-a[1].sensitive).slice(0,14))console.log(h.padEnd(22),JSON.stringify(r));
console.log("rssMB",Math.round(process.memoryUsage().rss/1e6));
