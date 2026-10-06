import {readFileSync,writeFileSync} from "node:fs";
import {resolve,join} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../.."),root="🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space",pairs=[];
function change(path:string,transform:(text:string)=>string){const before=readFileSync(join(repo,path),"utf8"),after=transform(before);if(after===before)throw Error("missing typed contract anchor "+path);pairs.push({path,before,after});}
for(const [leaf,type,expression]of [["🎚️config","SpaceConfig","SpaceConfigMutation::Snapshot { config: base.clone() }"],["👥️presence","SpacePresence","Self::Snapshot { presence: base.clone() }"]])change(join(root,leaf,"🦀️.rs"),text=>{
 const old="fn inverse(&self, base: &"+type+") -> Vec<Self> {\n        vec!["+expression+"]\n    }";
 if(!text.includes(old))throw Error("exact original inverse contract "+type);
 return text.replace(old,"fn inverse(&self, base: &"+type+") -> Result<Vec<Self>, semio_framework_value::ValueError> {\n        Ok(vec!["+expression+"])\n    }");
});
change(join(root,"🦀️.rs"),text=>{
 const close='fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {',topology="async fn interaction_topology(doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> InteractionTopology {",start=text.indexOf(close),end=text.indexOf("    fn terminal_is_empty",start);
 if(start<0||end<0||!text.includes(topology)||!text.includes("        InteractionTopology { domains }"))throw Error("original exact close/topology contract guard");
 const before=text.slice(start,end),after=before.replace(close,close.replace("String>","semio_framework_value::ValueError>")).replace("let bytes = space_config_mutation_bytes(mutation)?;","let bytes = space_config_mutation_bytes(mutation).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,message))?;").replace('return Err("Space Config preparation could not return its exact base root".into());','return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"Space Config preparation could not return its exact base root"));');
 if(after===before||after.includes("space_config_mutation_bytes(mutation)?"))throw Error("exact typed admitted retirement error guard");
 return text.slice(0,start)+after+text.slice(end).replace(topology,topology.replace("-> InteractionTopology","-> Result<InteractionTopology, semio_framework_value::ValueError>")).replace("        InteractionTopology { domains }","        Ok(InteractionTopology { domains })");
});
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const pair of pairs)if(readFileSync(join(repo,pair.path),"utf8")!==pair.before)throw Error("concurrent exact typed contract guard "+pair.path);
for(const pair of pairs)writeFileSync(join(repo,pair.path),pair.after);
console.log("[DEBUG] Hub Space original inverse, admitted retirement and topology typed contracts mounted paths="+pairs.length);
