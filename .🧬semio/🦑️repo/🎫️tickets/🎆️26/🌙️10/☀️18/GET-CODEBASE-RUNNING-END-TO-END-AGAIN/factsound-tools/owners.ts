/** 🧭️ Maps the stale recheck rows of one sealed memo record to the generation owners that read them and says whether each owner lies inside the slice the record verified (the pin's covered set). Usage: bun owners.ts <memo dir> <record key prefix> <root> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [directory,prefix,root]=process.argv.slice(2) as [string,string,string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {observationsHold}=await import(L+"/👁️observation/🟨️.mjs");
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const {decodeVerifiedScopeV1}=await import(L+"/🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs");
const record=JSON.parse(readFileSync(join(directory,prefix+".json"),"utf8"));
const pin=JSON.parse(record.pin),recheck=JSON.parse(readFileSync(join(directory,record.relevance.id+".chk"),"utf8")) as {rows:{kind:string;path:string;fact:unknown}[]};
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
const stale:typeof recheck.rows=[];
for(let index=0;index<recheck.rows.length;index+=256){const chunk=recheck.rows.slice(index,index+256);if(await observationsHold(chunk,64,new Set(),true))continue;for(const row of chunk)if(!await observationsHold([row],1,new Set(),true))stale.push(row);}
console.log("[DEBUG] stale",stale.length,"rss",rss());
const engine=generation.createGenerationStoreV1({root:join(root,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest);if(!manifest)throw Error("manifest retired");
const owners=Object.keys(manifest.owners).sort(generation.compareCodePointsV1),covered=decodeVerifiedScopeV1(owners,pin.covered);
const index=await engine.readIndexView(manifest.factIndex);
const keyOf=(row:{kind:string;path:string;fact:unknown})=>row.kind+"\u0000"+row.path+"\u0000"+JSON.stringify(row.fact);
const wanted=new Map(stale.map(row=>[keyOf(row),row])),ids=new Map<number,string>();
for(let at=0;at<index.rows.length;at++){const row=index.rows[at];const key=keyOf(row);if(wanted.has(key))ids.set(at,key);if(rss()>1500)throw Error("rss");}
console.log("[DEBUG] matched ids",ids.size,"rss",rss());
const byOwner=new Map<string,number>(),inside=new Map<string,number>(),kinds=new Map<string,{in:number;out:number}>();
const ownersOfRow=new Map<number,Set<string>>();
for(const cls of index.classes){const hit=cls.r.filter((id:number)=>ids.has(id));if(!hit.length)continue;for(const id of hit){const set=ownersOfRow.get(id)??new Set();for(const owner of cls.o)set.add(index.owners[owner]);ownersOfRow.set(id,set);}}
for(const [id,set] of ownersOfRow){const row=wanted.get(ids.get(id)!)!,inScope=[...set].some(name=>covered.has(name)),entry=kinds.get(row.kind)??{in:0,out:0};inScope?entry.in++:entry.out++;kinds.set(row.kind,entry);for(const name of set){byOwner.set(name,(byOwner.get(name)??0)+1);if(covered.has(name))inside.set(name,(inside.get(name)??0)+1);}}
console.log("[DEBUG] covered",covered.size,"rows with owners",ownersOfRow.size,"by kind in/out of slice",JSON.stringify([...kinds]));
console.log("[DEBUG] owners inside slice reading stale rows",JSON.stringify([...inside].sort((a,b)=>b[1]-a[1]).slice(0,30)));
console.log("[DEBUG] owners (any) reading stale rows",JSON.stringify([...byOwner].sort((a,b)=>b[1]-a[1]).slice(0,30)));
for(const [id,set] of [...ownersOfRow].slice(0,0))console.log(id,set);
const samples=[...ownersOfRow].filter(([,set])=>[...set].some(name=>covered.has(name))).slice(0,8);
for(const [id,set] of samples)console.log("[DEBUG] in-slice stale",JSON.stringify(index.rows[id]).slice(0,300),[...set].filter(name=>covered.has(name)).slice(0,4));
console.log("[DEBUG] rss",rss());
