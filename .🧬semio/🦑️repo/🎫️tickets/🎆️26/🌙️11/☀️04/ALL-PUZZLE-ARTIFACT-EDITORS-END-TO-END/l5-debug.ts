import { readFileSync } from "node:fs";
import { resolve } from "node:path";
const [idx,dim,needlesFile]=[Number(process.argv[2]),process.argv[3],process.argv[4]];
const root="/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle";
const f=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json"),"utf8"));
const o=f.owners[idx];const dir=resolve(root,o.source,"..");
const src=[...o.mounted.map((p:string)=>readFileSync(resolve(dir,p),"utf8")),readFileSync(resolve(root,o.source),"utf8")].join("\n");
const p=src.split("//#region 🧪️UnitTests")[0];
const needles:string[]=JSON.parse(readFileSync(needlesFile,"utf8"));
for(const n of needles) if(!p.includes(n)) console.log("MISSING:",n);
const reserved=new Set(["copy","cut","paste","import-media"]);
const routes=o.groups.flatMap((g:any)=>g.routes).filter((r:string)=>!(idx!==1&&reserved.has(r)));
const pairs=new Map([...p.matchAll(/\.action_interactive_job\("([^"]+)",\s*(?:semio_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map(m=>[m[1],m[2]]));
console.log("pairs",pairs.size,"routes",routes.length,"missing-from-pairs",routes.filter((r:string)=>!pairs.has(r)),"extra",[...pairs.keys()].filter(k=>!routes.includes(k)));
const D=dim.toUpperCase(), N="Puzzle"+dim;
const ids=p.match(new RegExp(`PUZZLE${D}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
console.log("ids",!!ids);
if(ids){const got=[...ids[1].matchAll(/"([^"]+)"/g)].map(m=>m[1]);const mig=o.groups.filter((g:any)=>g.status==="migrated").flatMap((g:any)=>g.routes).filter((r:string)=>!(idx!==1&&reserved.has(r)));console.log("ids-vs-migrated: onlyIds",got.filter((x:string)=>!mig.includes(x)),"onlyMig",mig.filter((x:string)=>!got.includes(x)));}
const fac=`${N}RetainedCommandJobFactory`;
for(const s of [`impl `,]){}
console.log("ToolJobFactory",new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ToolJobFactory for ${fac}\\b`).test(p),"Owned",new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for ${fac}\\b`).test(p));
const pb=p.split("semio_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0]??"";
console.log("proof factory",pb.includes(`factory: "${fac}"`),pb.includes(`factory_type: ${fac}`));
console.log("owner type",new RegExp(`type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<${N}PlayApp>;`).test(p));
console.log("register",p.includes(`registry.register(${fac}::new(&controller))`));
const blk=(p.split(new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for ${fac}\\b`))[1]?.split("//#endregion 🧵️RetainedCommands")[0])??"";
const contracts=new Map([...blk.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map(m=>[m[1],[...m[2].matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map(l=>l[1])]));
console.log("contracts",contracts.size);
for(const s of ["fn build_artifact_store_one_item_preparation_factory()","fn build_config_store_one_item_preparation_factory()","build_draft_store_one_item_preparation_factory","build_presence_store_one_item_preparation_factory","build_transient_store_one_item_preparation_factory",'"setLocale"',"ConfigMutation::SetLocale","ConfigMutation::SetTerminology"]) console.log(s,p.includes(s));
