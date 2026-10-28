/** 🕳️ Shows the freshness hole on any root: installs a record of the authored compiled fixture tree (record version of that root), edits a schema the binary includes from outside its crate roots, prints what `staleDashboard` answers. `bun holeprobe.ts <root> <scratch-dir> <record-version>` */
import {mkdirSync,mkdtempSync,writeFileSync,appendFileSync,statSync,rmSync} from "node:fs";
import {dirname,join} from "node:path";
const [rootArg,scratchArg,version]=process.argv.slice(2);
const L=join(rootArg!,"🧰️framework/🛍️products/🦑️repo/🔨️modules/");
const m=await import(join(L,"📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts"));
const fixture=(await import(join(L,"🎛️dashboard/🧪️tests/🧫️fixtures/🧊️execution/🧾️compiled.json"),{with:{type:"json"}})).default;
mkdirSync(scratchArg!,{recursive:true});const root=mkdtempSync(join(scratchArg!,"hole-"));
for(const[path,content]of Object.entries(fixture.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
Bun.sleepSync(5);const builtAtMs=Date.now();Bun.sleepSync(5);
const sources=await m.captureDashboardSources(join(root,fixture.package),root),digest="b".repeat(64),path=`.cache/b/t/m/dashboard-cli/prototype/${digest}/semio`;
mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),"x");
const record:any={version:Number(version),platform:process.platform,mode:"prototype",path,sources,digest,bytes:1,permissions:statSync(join(root,path)).mode&0o777};if(Number(version)>=4)record.compiled={builtAtMs,sources:fixture.compiled};
mkdirSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype"),{recursive:true});writeFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/prototype/installed.json"),JSON.stringify(record));
console.log("[DEBUG] before edit:",JSON.stringify(await m.staleDashboard(root,"prototype")??null));
appendFileSync(join(root,"schema/🔣️.json")," ");
console.log("[DEBUG] after editing schema/🔣️.json (included by the binary, outside every crate root):",JSON.stringify(await m.staleDashboard(root,"prototype")??null));
rmSync(root,{recursive:true,force:true});
