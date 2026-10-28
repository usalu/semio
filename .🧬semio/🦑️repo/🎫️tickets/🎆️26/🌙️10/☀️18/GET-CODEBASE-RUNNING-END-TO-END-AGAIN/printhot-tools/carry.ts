/** 🧳️ Read-only probe: would a graph-memo record whose generation pointer moved still describe the current generation for its slice? `bun carry.ts <root> <memo-dir>` prints, per record, the covered owners and how many of them (plus env/global/implementation/membership) differ between the record's manifest and the current one. Nothing is written. */
import {readFileSync,readdirSync,existsSync} from "node:fs";
import {join} from "node:path";
const [root,directory]=process.argv.slice(2) as [string,string];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const coverage=await import(join(L,"🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),current=JSON.parse(readFileSync(join(store,"current.json"),"utf8"));
const manifests=new Map<string,any>(),manifest=(digest:string)=>{let value=manifests.get(digest);if(value===undefined){const file=join(store,"m",digest+".json");value=existsSync(file)?JSON.parse(readFileSync(file,"utf8")):null;manifests.set(digest,value);}return value;};
const now=manifest(current.manifest);
console.log(JSON.stringify({current:{generation:current.generation,manifest:current.manifest.slice(0,12)}}));
for(const name of readdirSync(directory).filter(name=>name.endsWith(".json"))){
 const record=JSON.parse(readFileSync(join(directory,name),"utf8"));
 if(record.state!=="sealed"){console.log(JSON.stringify({record:name.slice(0,12),state:record.state}));continue;}
 const then=manifest(record.manifest);
 if(!then){console.log(JSON.stringify({record:name.slice(0,12),generation:record.pointer.generation,manifest:"retired"}));continue;}
 const pin=JSON.parse(record.pin),owners=Object.keys(then.owners).sort((a,b)=>a<b?-1:a>b?1:0),covered=[...coverage.decodeVerifiedScopeV1(owners,pin.covered)] as string[];
 const differs=covered.filter(owner=>JSON.stringify(then.owners[owner])!==JSON.stringify(now.owners[owner]));
 const tops=Object.fromEntries(["env","global","implementation","membership","identity","frontier","factIndex"].map(key=>[key,then[key]===now[key]]));
 console.log(JSON.stringify({record:name.slice(0,12),generation:record.pointer.generation,projects:record.projects,covered:covered.length,ownersNow:Object.keys(now.owners).length,ownersThen:owners.length,differs:differs.length,examples:differs.slice(0,5).map(owner=>({owner,then:then.owners[owner]?.fragment?.slice(0,8),now:now.owners[owner]?.fragment?.slice(0,8),edgesThen:then.owners[owner]?.edgesEnv?.slice(0,8),edgesNow:now.owners[owner]?.edgesEnv?.slice(0,8)})),same:tops}));
}
console.log(JSON.stringify({rssMb:Math.round(process.memoryUsage().rss/1048576)}));
