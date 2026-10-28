/** 🗂 Read-only: over every owner fragment of the live generation, which roster chunks (`SEMIO_SOURCE_CLOSURE_*`) its node names, and how many of those rosters hold build-library / process files, scheduling-domain files, test files. Answers per class how many owners would see a roster change if a file of that class joined their command closure. Usage: bun rosters.mjs <root> */
import{readFileSync,existsSync}from"node:fs";
import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
const [root]=process.argv.slice(2);
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const table=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"),"utf8"));
const sched=table.domains.flatMap(d=>d.prefixes),native=new Set(table.domains.flatMap(d=>d.nativeKeyed??[]));
const lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",proc="🧰️framework/🔨️modules/🏃️process/";
const closures=join(root,".cache/b/g/m/build-input-closures"),rosterMemo=new Map();
const roster=key=>{if(rosterMemo.has(key))return rosterMemo.get(key);const p=join(closures,key+".json");const v=existsSync(p)?JSON.parse(readFileSync(p,"utf8")).files:undefined;const s=v&&{lib:v.filter(f=>f.startsWith(lib)).length,proc:v.filter(f=>f.startsWith(proc)).length,sched:v.filter(f=>sched.some(x=>f.startsWith(x))).length,schedFiles:v.filter(f=>sched.some(x=>f.startsWith(x))).slice(0,3),tests:v.filter(f=>(f.startsWith(lib)||f.startsWith(proc))&&f.includes("/🧪️tests/")).length,n:v.length};rosterMemo.set(key,s);return s;};
let owners=0,withLib=0,withSched=0,withLibTests=0,missing=0;const schedTargets=new Map(),examples=[];
for(const[name,row]of Object.entries(man.owners)){if(row.kind!=="project")continue;owners++;
 const node=JSON.parse(brotliDecompressSync(readFileSync(join(store,"o",row.fragment+".json.br")))).node;
 const keys=new Set();JSON.stringify(node,(k,v)=>{if(typeof v==="string"&&/^SEMIO_SOURCE_CLOSURE_[0-9a-f]{64}$/.test(v))keys.add(v.slice(21));return v;});
 let l=0,s=0,t=0;for(const k of keys){const r=roster(k);if(!r){missing++;continue;}l+=r.lib+r.proc;s+=r.sched;t+=r.tests;if(r.sched&&examples.length<6)examples.push([name,r.schedFiles]);}
 if(l)withLib++;if(s)withSched++;if(t)withLibTests++;
 if(owners%500===0&&process.memoryUsage().rss>1.5*2**30){console.error("[DEBUG] rss abort");process.exit(3);}
}
console.log(JSON.stringify({generation:man.generation,owners,ownersNamingLibraryOrProcessFiles:withLib,ownersNamingSchedulingFiles:withSched,ownersNamingLibraryTestFiles:withLibTests,missingRosters:missing,distinctRosters:rosterMemo.size,rssMB:Math.round(process.memoryUsage().rss/2**20)}));
for(const e of examples)console.log(" ",JSON.stringify(e).slice(0,400));
