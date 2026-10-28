/** 🔁️ Prints the generation's analysis rows whose path contains a needle, with every nested observation, and the owners holding them. Usage: bun rows.ts <root> <needle> [manifest] */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,needle,given]=process.argv.slice(2) as [string,string,string|undefined];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const store=join(root,".cache/b/g/m/generations"),digest=given??JSON.parse(readFileSync(join(store,"current.json"),"utf8")).manifest;
const engine=generation.createGenerationStoreV1({root:store,fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(digest);if(!manifest)throw Error("manifest retired");
const index=await engine.readIndexView(manifest.factIndex);
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
const hits=new Map<number,unknown>();
for(let at=0;at<index.rows.length;at++){const row=index.rows[at];if(String(row.path).includes(needle))hits.set(at,row);if(rss()>2000)throw Error("rss");}
const owners=new Map<number,Set<string>>();
for(const cls of index.classes)for(const id of cls.r)if(hits.has(id)){const set=owners.get(id)??new Set();for(const o of cls.o)set.add(index.owners[o]);owners.set(id,set);}
for(const [id,row] of hits)console.log("[DEBUG] row",id,JSON.stringify(row).slice(0,4000),"owners",JSON.stringify([...owners.get(id)??[]]));
console.log("[DEBUG] generation",manifest.generation??"?","rows",index.rows.length,"hits",hits.size,"rss",rss());
