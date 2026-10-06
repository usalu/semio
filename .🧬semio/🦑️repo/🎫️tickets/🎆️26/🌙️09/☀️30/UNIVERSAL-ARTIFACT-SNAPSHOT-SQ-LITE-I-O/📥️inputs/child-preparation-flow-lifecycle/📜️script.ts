const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";
const input=`${ticket}/📥️inputs/child-preparation-flow-lifecycle`;
const plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const flow="✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const changes:{path:string,before:string,after:string}[]=[];
async function edit(path:string,changesForFile:(before:string)=>Promise<string>|string){const before=await Bun.file(path).text();changes.push({path,before,after:await changesForFile(before)});}
function replace(source:string,from:string,to:string){if(source.split(from).length!==2)throw Error("exact lifecycle span guard");return source.replace(from,to);}
if(process.argv[2]==="demand"){
  await edit(`${flow}/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`,async before=>{if(before.includes("rejected_child_preparation_remains"))throw Error("demand already present");return before+await Bun.file(`${input}/law.rs`).text();});
  const path=`${flow}/🎮️commands/✏️node-graph-edit/🧫️fixtures/child-preparation/🔣️.json`;
  if(await Bun.file(path).exists())throw Error("fixture already exists");changes.push({path,before:"",after:await Bun.file(`${input}/🔣️.json`).text()});
}else if(process.argv[2]==="zero-sized-allocation-provider"){
  await edit(`${flow}/🦀️.rs`,before=>{
    let after=replace(before,"    fn accept_output(&mut self,emit:","    fn has_vector_allocation<T>(owner:&Vec<T>)->bool{std::mem::size_of::<T>()!=0&&owner.capacity()!=0}\n\n    fn accept_output(&mut self,emit:");
    for(const field of ["artifact_mutations","config_mutations","window_config_mutations","draft_mutations","effects","events","extension_invocations","interaction_writes","tasks"]){after=replace(after,`emit.${field}.capacity()!=0`,`Self::has_vector_allocation(&emit.${field})`);}
    return after;
  });
}else if(process.argv[2]==="refusal-diagnostic-join"){
  await edit(`${flow}/🧪️tests/📨️child-preparation/🦀️.rs`,before=>replace(before,"InteractiveJobCloseStep::Blocked=>panic!(\"actual first-party child retirement must make bounded progress\"),","InteractiveJobCloseStep::Blocked=>panic!(\"[DEBUG] actual child owner blocked: source={:?}, refusal={:?}, retirement={:?}, preparations={}, groups={}\",work.output.as_ref().and_then(|emit|emit.child_preparations.front()).map(|source|source.retained_operation_count()),work.output.as_ref().and_then(|emit|emit.child_preparations.front()).and_then(|source|source.refusal()),work.output.as_ref().and_then(|emit|emit.child_preparations.front()).and_then(|source|source.retirement_refusal()),work.output.as_ref().map_or(0,|emit|emit.child_preparations.len()),work.output.as_ref().map_or(0,|emit|emit.child_emits.len())),"));
}else if(process.argv[2]==="fixture-path-join"){
  await edit(`${flow}/🧪️tests/📨️child-preparation/🦀️.rs`,before=>replace(before,"../../../🎮️commands/✏️node-graph-edit/🧫️fixtures/child-preparation/🔣️.json","../../🎮️commands/✏️node-graph-edit/🧫️fixtures/child-preparation/🔣️.json"));
}else if(process.argv[2]==="test-module-join"){
  const law=await Bun.file(`${input}/law.rs`).text();
  await edit(`${flow}/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`,before=>replace(before,law,""));
  const path=`${flow}/🧪️tests/📨️child-preparation/🦀️.rs`;
  if(await Bun.file(path).exists())throw Error("owning work test module already exists");
  changes.push({path,before:"",after:"use super::*;\n"+law.replace("../../🧫️fixtures/child-preparation/🔣️.json","../../../🎮️commands/✏️node-graph-edit/🧫️fixtures/child-preparation/🔣️.json").replace("crate::editor::flow::FlowChildGroupWork","FlowChildGroupWork")});
  await edit(`${flow}/🦀️.rs`,before=>before+"\n#[cfg(test)]\n#[path=\"🧪️tests/📨️child-preparation/🦀️.rs\"]\nmod child_preparation_owner_tests;\n");
}else if(process.argv[2]==="provider"){
  await edit(`${plugin}/🧩️composition/📨️emission/📦️preparation/🦀️.rs`,before=>{
    let after=replace(before,"    fn as_any_mut(&mut self)->&mut dyn std::any::Any;","    fn as_any_mut(&mut self)->&mut dyn std::any::Any;\n    fn matches_source(&self,mutation_type:std::any::TypeId,slot:&str,child_id:&str,maximum_operations:usize)->bool;");
    after=replace(after,"        let backing_bytes=operations.capacity()","        let operation_count=operations.len();\n        let backing_bytes=operations.capacity()");
    after=replace(after,"            remaining:ManuallyDrop::new(Some(operations.into_iter())),backing_bytes,","            remaining:ManuallyDrop::new(Some(operations.into_iter())),backing_bytes,operation_count,");
    after=replace(after,"    pub fn take_retirement_provider<M:Send+'static>","    /// 🪪️ Checks the original typed operation source and literal child address without encoding or transferring owners.\n    pub fn matches_source<M:'static>(&self,slot:&str,child_id:&str,maximum_operations:usize)->bool{self.owner.matches_source(std::any::TypeId::of::<M>(),slot,child_id,maximum_operations)}\n    pub fn take_retirement_provider<M:Send+'static>");
    after=replace(after,"    backing_bytes:usize,","    backing_bytes:usize,\n    operation_count:usize,");
    after=replace(after,"    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}","    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}\n    fn matches_source(&self,mutation_type:std::any::TypeId,slot:&str,child_id:&str,maximum_operations:usize)->bool{\n        mutation_type==std::any::TypeId::of::<M>()&&!self.closing&&!self.ready&&self.operation_count!=0&&self.operation_count<=maximum_operations&&self.prefix.as_ref().is_some_and(|prefix|prefix.slot==slot&&prefix.child_id==child_id)\n    }");
    return after;
  });
  await edit(`${flow}/🦀️.rs`,async before=>{
    const start=before.indexOf("struct FlowChildGroupWork {"),end=before.indexOf("\nstruct FlowChildGroupJobFactory",start);if(start<0||end<start)throw Error("actual Flow work region");let region=before.slice(start,end);
    region=replace(region,"    completed: bool,","    output: Option<Emit<FlowMutation, NoConfigMutation, NoDraftMutation>>,\n    completed: bool,");
    region=replace(region,"Self { tool_id, instance_owner: Some(instance_owner), completed: false, closing: false }","Self { tool_id, instance_owner: Some(instance_owner), output: None, completed: false, closing: false }");
    region=replace(region,"    fn admitted_child<'a>(",await Bun.file(`${input}/accept.rs`).text()+"\n    fn admitted_child<'a>(");
    region=replace(region,"if self.closing || self.completed {","if self.closing || self.completed || self.output.is_some() {");
    const left=region.indexOf("        let exact_child = emit.child_emits.first()"),right=region.indexOf("\n    fn begin_close",left);if(left<0||right<left)throw Error("old output contract");
    region=region.slice(0,left)+"        self.accept_output(emit, &snapshot.content.child_id)\n    }\n"+region.slice(right);
    const closeLeft=region.indexOf("    fn close_step("),closeRight=region.indexOf("\n    fn terminal_is_empty",closeLeft);region=region.slice(0,closeLeft)+await Bun.file(`${input}/close.rs`).text()+region.slice(closeRight);
    region=replace(region,"self.closing && self.instance_owner.is_none()","self.closing && self.instance_owner.is_none() && self.output.is_none()");
    return before.slice(0,start)+region+before.slice(end);
  });
}else throw Error("known lifecycle command required");
await Bun.write(`${input}/${process.argv[2]}-guarded-pairs.json`,JSON.stringify({pairs:changes},null,2)+"\n");
for(const pair of changes){const actual=await Bun.file(pair.path).exists()?await Bun.file(pair.path).text():"";if(actual!==pair.before)throw Error(`fresh concurrent guard failed: ${pair.path}`);}
for(const pair of changes)await Bun.write(pair.path,pair.after);
console.log(JSON.stringify({mounted:changes.length,command:process.argv[2]}));
