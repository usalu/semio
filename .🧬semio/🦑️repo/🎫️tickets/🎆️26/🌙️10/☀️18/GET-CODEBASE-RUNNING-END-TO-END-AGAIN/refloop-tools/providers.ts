/** 🧮️ Counts the current generation's analysis rows per provider, the owners holding them, and which hold in a fresh process that loaded the inference module (as the graph child does) before and after the lazy declared-import provider registers. Usage: bun providers.ts <root> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root]=process.argv.slice(2) as [string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const generation=await import(L+"/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
await import(L+"/🏗️build/🕸️graph/📥️inference/🟨️.mjs");
const {observationsHold}=await import(L+"/🏗️build/👁️observation/🟨️.mjs");
const store=join(root,".cache/b/g/m/generations"),digest=JSON.parse(readFileSync(join(store,"current.json"),"utf8")).manifest;
const engine=generation.createGenerationStoreV1({root:store,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(digest);if(!manifest)throw Error("manifest retired");
const index=await engine.readIndexView(manifest.factIndex);
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
type Row={kind:string;path:string;fact:{provider:string}};
const byProvider=new Map<string,number[]>();
for(let at=0;at<index.rows.length;at++){const row=index.rows[at] as Row;if(row.kind!=="analysis")continue;const key=row.fact.provider.replace(/:[a-f0-9]{64}$/u,":<hash>");const list=byProvider.get(key)??[];list.push(at);byProvider.set(key,list);if(rss()>2000)throw Error("rss");}
const ownersOf=new Map<number,Set<string>>(),wanted=new Set([...byProvider.values()].flat());
for(const cls of index.classes)for(const id of cls.r)if(wanted.has(id)){const set=ownersOf.get(id)??new Set();for(const o of cls.o)set.add(index.owners[o]);ownersOf.set(id,set);}
const check=async(label:string)=>{for(const [provider,ids] of byProvider){let bad=0;const owners=new Set<string>();for(const id of ids)if(!await observationsHold([index.rows[id]],1,new Set(),true)){bad++;for(const o of ownersOf.get(id)??[])owners.add(o);}console.log("[DEBUG]",label,provider,"rows",ids.length,"stale",bad,"owners",JSON.stringify([...owners].slice(0,8)),owners.size);}};
await check("before");
const {sourceRuntimeImportsV1}=await import(L+"/🕸️dependencies/🟦️typescript/🟨️.mjs");
sourceRuntimeImportsV1(join(L,"🕸️dependencies/🟦️typescript/🟨️.mjs"));
await check("after");
console.log("[DEBUG] generation",manifest.generation??"?","rss",rss());
