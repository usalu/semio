import { resolve } from "node:path";

const ticket = resolve(import.meta.dir, "../..");
const plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
type Region = { name: string; before: string; after: string; count?: number };
type Pair = { path: string; before: string; after: string; regions: Region[] };
const pairs = new Map<string, Pair>();
async function region(path: string, name: string, before: string, after: string, count = 1) {
  let pair = pairs.get(path);
  if (!pair) { const text = await Bun.file(path).text(); pair = { path, before: text, after: text, regions: [] }; pairs.set(path, pair); }
  if (pair.after.split(before).length !== count + 1) throw Error(`Exact region guard refused ${path}: ${name}`);
  pair.after = pair.after.split(before).join(after);
  pair.regions.push({ name, before, after, count });
}
const source = await Bun.file(plugin).text();
const closeStart = source.indexOf("        pub(crate) fn close_one", source.indexOf("    impl ChildEmit {"));
const ofDoc = source.indexOf("        /// 🏭️ The one sanctioned constructor", closeStart);
const openDoc = source.indexOf("        /// 🫙️", ofDoc);
if (closeStart < 0 || ofDoc < closeStart || openDoc < ofDoc) throw Error("ChildEmit function span guards refused");
await region(plugin, "complete-prefix-owned-allocation-close", source.slice(closeStart, ofDoc), await Bun.file(resolve(import.meta.dir, "prefix-close.rs")).text() + "\n");
await region(plugin, "remove-eager-borrowed-child-constructor", source.slice(ofDoc, openDoc), "");
await region(plugin, "owned-child-preparation-fifo-lane", "        pub child_emits: Vec<ChildEmit>,", "        pub child_emits: Vec<ChildEmit>,\n        pub child_preparations: std::collections::VecDeque<ChildEmitPreparation>,");
await region(plugin, "owned-child-preparation-default", "                child_emits: Vec::new(),", "                child_emits: Vec::new(),\n                child_preparations: std::collections::VecDeque::new(),");
await region(plugin, "owned-child-preparation-module", "#[path = \"🧵️retained-command/🦀️.rs\"]", "#[path=\"🧩️composition/📨️emission/📦️preparation/🦀️.rs\"]\nmod child_emit_preparation;\n\n#[path = \"🧵️retained-command/🦀️.rs\"]");
await region(plugin, "owned-child-preparation-app-export", "    pub use super::transient_publication::{", "    pub use super::child_emit_preparation::{ChildEmitPreparation,ChildEmitPreparationStep};\n    pub use super::transient_publication::{");
const pushStart = source.indexOf("        pub fn push<S, M>", openDoc);
const pushEnd = source.indexOf("\n    }\n", pushStart);
if (pushStart < 0 || pushEnd < pushStart) throw Error("ChildEmit push span refused");
const push = `        pub fn push<S,M>(&mut self,op:&M)->Result<Option<SchemaId>,::protocol::ProtocolError>
        where M:protocol::SemanticMutation<S>+::protocol::OpBinary{
            let encoded=::protocol::OpBinary::encode_op(op)?;
            let label=protocol::SemanticMutation::label(op);
            let retired_schema=if self.ops.is_empty(){
                let semantics=protocol::SemanticMutation::semantics(op);
                Some(std::mem::replace(&mut self.op_schema,SchemaId(format!("{}.{}",semantics.entity,semantics.kind))))
            }else{None};
            self.labels.push(label);self.ops.push(encoded);Ok(retired_schema)
        }`;
await region(plugin, "typed-encoder-refusal-before-prefix-mutation", source.slice(pushStart, pushEnd), push);
await region(plugin, "owning-emission-preparation-step", "        /// 🧹️ Retires at most one bounded child-emission step", await Bun.file(resolve(import.meta.dir, "emit-prepare.rs")).text() + "        /// 🧹️ Retires at most one bounded child-emission step");

const helpers = new Set(["playbook_flow_emit", "widget_leaves_emit", "flow_content_leaves_emit", "sequence_child_leaves_emit", "wires_child_emit", "procedure_child_emit", "working_child_emit", "jack_child_emit", "dag_child_emit"]);
const scan = Bun.spawnSync(["rg", "-l", "ChildEmit::of|playbook_flow_emit|widget_leaves_emit|flow_content_leaves_emit|sequence_child_leaves_emit|wires_child_emit|procedure_child_emit|working_child_emit|jack_child_emit|dag_child_emit", "✏️s", "-g", "🦀️.rs"]);
if (scan.exitCode !== 0) throw Error("Owning emission caller census refused");
function ownedCall(line: string, name: string): string {
  const pos = line.indexOf(name + "(");
  if (pos < 0 || /\bfn\s/.test(line.slice(0, pos))) return line;
  const begin = pos + name.length + 1;
  let depth = 0, quote = false, escape = false, comma = -1;
  for (let i = begin; i < line.length; i++) {
    const c = line[i];
    if (quote) { if (escape) escape = false; else if (c === "\\") escape = true; else if (c === "\"") quote = false; continue; }
    if (c === "\"") { quote = true; continue; }
    if ("([{<".includes(c)) depth++;
    else if (")]} >".replaceAll(" ", "").includes(c)) { if (depth === 0) break; depth--; }
    else if (c === "," && depth === 0) { comma = i; break; }
  }
  if (comma < 0) throw Error(`Owning helper argument guard refused ${name}: ${line}`);
  const tail = line.slice(comma + 1);
  if (!/^\s*&/.test(tail)) return line;
  const repaired = tail.replace(/^(\s*)&\[/, "$1vec![").replace(/^(\s*)&/, "$1");
  return line.slice(0, comma + 1) + repaired;
}
for (const path of new TextDecoder().decode(scan.stdout).trim().split("\n")) {
  if (path.includes("/🧪️tests/") || path.includes("/🧫️fixtures/")) continue;
  const text = await Bun.file(path).text();
  const lines = text.split("\n"); const seen = new Set<string>();
  for (let index = 0; index < lines.length; index++) {
    const before = lines[index]; if (seen.has(before)) continue; seen.add(before); let after = before;
    for (const name of helpers) after = ownedCall(after, name);
    if (/\bfn\s/.test(after) && [...helpers].some(name => after.includes(name))) after = after.replace(/leaves: &\[(SemioFlowMutation|SemioGraphMutation)\]/, "leaves: Vec<$1>");
    if (after.includes("ChildEmit::of")) {
      after = after.replaceAll("ChildEmit::of", "ChildEmitPreparation::of").replace(/, &\[/g, ", vec![");
      after = after.replaceAll("child_emits: vec![", "child_preparations: std::collections::VecDeque::from([");
      if (after.includes("child_preparations:")) after = after.replace(/\]\s*,\s*(ui_scope:|\.\.Default|\.\.semio_framework_plugin)/, "]), $1");
    }
    if (after.includes("playbook_flow_emit") && after.includes("-> semio_framework_plugin::app::ChildEmit")) after = after.replace("-> semio_framework_plugin::app::ChildEmit", "-> semio_framework_plugin::app::ChildEmitPreparation");
    if (after.includes("child_emits: vec![playbook_flow_emit")) after = after.replace("child_emits: vec![", "child_preparations: std::collections::VecDeque::from([").replace(/\], ui_scope:/, "]), ui_scope:");
    if (after.includes("child_preparations:")) after = after.replace(/\](,\s*$)/, "\])$1");
    if (/^\s*(?:pub )?use /.test(after)) after = after.replace(/\bChildEmit\b/g, "ChildEmitPreparation");
    if (before !== after) await region(path, `owning-child-emission-line-${index + 1}`, before, after, text.split(before).length - 1);
  }
}
const closeChildStart=source.indexOf("        pub fn close_child_one(");
const closeChildEnd=source.indexOf("        /// ✏️ A document-operation",closeChildStart);
if(closeChildStart<0||closeChildEnd<closeChildStart)throw Error("Owned child queue close span refused");
await region(plugin,"owning-preparation-and-prefix-close",source.slice(closeChildStart,closeChildEnd),await Bun.file(resolve(import.meta.dir,"emit-close.rs")).text()+"\n");
const retained="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs";
await region(retained,"retain-emit-before-stepped-child-publication","                let Some(completion) = self.completion.as_ref() else { return self.fault(cx, b\"retained command completion owner is absent\") };",`                if let Some(emit)=self.emit.as_mut(){
                    let bytes=emit.next_child_preparation_byte_demand().max(1);
                    match emit.prepare_child_one(1,bytes){
                        Ok(crate::app::ChildEmitPreparationStep::Ready)=>{},
                        Ok(crate::app::ChildEmitPreparationStep::Pending)=>{cx.consume_fuel(1);return self.preview(cx,br#"{"en":"Preparing child operations","de":"Kindoperationen werden vorbereitet"}"#);},
                        Ok(crate::app::ChildEmitPreparationStep::Refused(fault))|Err(fault)=>return self.reducer_fault(cx,&fault),
                    }
                }
                let Some(completion) = self.completion.as_ref() else { return self.fault(cx, b"retained command completion owner is absent") };`);
await region(plugin,"mounted-publication-keeps-preparation-owner", "\n                ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {",`
                ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                    let bytes=emit.next_child_preparation_byte_demand().max(1);
                    match emit.prepare_child_one(1,bytes)?{
                        ChildEmitPreparationStep::Ready=>{},
                        ChildEmitPreparationStep::Pending=>return Ok(()),
                        ChildEmitPreparationStep::Refused(fault)=>return Err(fault),
                    }`);
const toolRun="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs";
const toolSource=await Bun.file(toolRun).text();
const emitVisitorStart=toolSource.indexOf("/// 🧾️ Decodes a member run's ops");
const emitVisitorEnd=toolSource.indexOf("/// 🔢️ The member store's generation",emitVisitorStart);
if(emitVisitorStart<0||emitVisitorEnd<emitVisitorStart)throw Error("Member emission issuer span refused");
await region(toolRun,"retained-exact-decoded-member-emission",toolSource.slice(emitVisitorStart,emitVisitorEnd),await Bun.file(resolve(import.meta.dir,"tool-run-emission.rs")).text()+"\n");
await region(toolRun,"owned-member-emission-and-retirement-queues","    emit: Option<ChildEmit>,",`    emit:Option<ChildEmit>,
    emission_owner:Option<Box<dyn ToolRunMemberEmissionOwner>>,
    retired_emits:std::collections::VecDeque<ChildEmit>,
    retired_emission_owners:std::collections::VecDeque<Box<dyn ToolRunMemberEmissionOwner>>,
    retired_ops:std::collections::VecDeque<Vec<Vec<u8>>>,`);
await region(toolRun,"member-emission-retirement-queue-initialization","            emit: None,",`            emit:None,emission_owner:None,
            retired_emits:std::collections::VecDeque::new(),retired_emission_owners:std::collections::VecDeque::new(),retired_ops:std::collections::VecDeque::new(),`);
await region(toolRun,"member-release-retains-complete-encoded-input","        self.ops.clear();", "        if !self.ops.is_empty()||self.ops.capacity()!=0{self.retired_ops.push_back(std::mem::take(&mut self.ops));}");
await region(toolRun,"member-release-retains-prefix-and-decoded-owner","        self.emit = None;", "        self.retired_emits.extend(self.emit.take());\n        self.retired_emission_owners.extend(self.emission_owner.take());");
await region(toolRun,"member-rebase-retains-prefix-and-decoded-owner","                member.emit = None;", "                member.retired_emits.extend(member.emit.take());\n                member.retired_emission_owners.extend(member.emission_owner.take());");
await region(toolRun,"member-finalize-actual-retirement-issuer-caller","ops: &member.ops, emit: &mut member.emit, deadline_us: deadline", "ops: &member.ops, emit: &mut member.emit, owner:&mut member.emission_owner, maximum_bytes:TYPED_OPERATION_RESULT_PAGE_BYTES");
await region(toolRun,"member-close-keeps-exact-byte-grant","    fn tool_run_member_retire_step(&mut self) -> Result<Option<PluginCloseStep>, Fault> {", "    fn tool_run_member_retire_step(&mut self,maximum_items:usize,maximum_bytes:usize) -> Result<Option<PluginCloseStep>, Fault> {");
await region(toolRun,"member-close-actual-owned-emission-cursors","        for (member, everything) in live.chain(retired) {", "        for (member, everything) in live.chain(retired) {\n"+await Bun.file(resolve(import.meta.dir,"tool-run-close.rs")).text());
await region(toolRun,"member-retirement-actual-budget-caller","        match self.tool_run_member_retire_step()? {", "        match self.tool_run_member_retire_step(maximum_items,maximum_bytes)? {");
const storePath="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs";
await region(storePath,"borrowed-installed-mutation-retirement-issuer","    pub fn install_snapshot_retirement_factory(&mut self, factory:",`    /// ♻️ Hands an exact mutation owner to the installed issuer after checking admission while it remains in its caller slot.
    pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>)->Result<Option<Box<dyn ErasedSnapshotRetirement>>,VcsError>{
        let Some(factory)=self.mutation_retirement_factory.as_ref()else{return Err(VcsError::ValidationFailed("owned mutation retirement requires its installed exact issuer".into()))};
        Ok(owner.take().map(|mutation|factory.retire_owned(mutation)))
    }

    pub fn install_snapshot_retirement_factory(&mut self, factory:`);
const mountedCloseBefore=`                        if let Some(child) = emit.child_emits.last_mut() {
                            let step = child.close_one(maximum_items, maximum_bytes);
                            if step == PluginCloseStep::Complete {
                                emit.child_emits.pop();
                                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                            }
                            return Ok(step);
                        }`;
await region(plugin,"mounted-cancellation-retains-and-steps-child-preparations",mountedCloseBefore,"                        if let Some(step)=emit.close_child_one(maximum_items,maximum_bytes){return Ok(step);}");
const memberPublishBefore=`        if self.dispatch_emit_group(&tool_id, &[], std::slice::from_ref(&emit), None, Vec::new(), Vec::new(), UiDirtyScope::Full, &meta, Some(group_id), Some(transaction)).await.is_err() {
            return self.reject_tool_run_publication(run, generation);
        }`;
await region(toolRun,"member-publication-retains-complete-prefix-after-dispatch",memberPublishBefore,`        let publication=self.dispatch_emit_group(&tool_id,&[],std::slice::from_ref(&emit),None,Vec::new(),Vec::new(),UiDirtyScope::Full,&meta,Some(group_id),Some(transaction)).await;
        let member=selected_entry_mut!(self.tool_runs).and_then(|entry|entry.member.as_mut()).expect("member publication retains its exact run");
        member.retired_emits.push_back(emit);
        member.retired_emission_owners.extend(member.emission_owner.take());
        if publication.is_err(){return self.reject_tool_run_publication(run,generation);}`);
await region(plugin,"preview-owned-preparation-job", "    fn drive_agent_lane_preview<J:", await Bun.file(resolve(import.meta.dir,"preview-prepare.rs")).text()+"    fn drive_agent_lane_preview<J:");
const previewBefore=`            let driven = drive_agent_lane_preview(dispatch.job, params, &verb);
            lease.finish();
            let steps = driven?;
            match completion.take()? {
                Some(ArtifactToolCompletionValue::Emit(emit, ephemeral)) => emit.map(|emit| (emit, (ephemeral.presence.len(), ephemeral.transient.len(), ephemeral.window_transient.len()), steps)).map_err(ArtifactBoundedToolFault::into_fault),`;
await region(plugin,"preview-captured-job-and-child-preparation-authority",previewBefore,`            let child_params=params.clone();
            let steps=match drive_agent_lane_preview(dispatch.job,params,&verb){Ok(steps)=>steps,Err(fault)=>{lease.finish();return Err(fault)}};
            match completion.take()? {
                Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>{
                    let emit=emit.map_err(ArtifactBoundedToolFault::into_fault)?;
                    let prepared=drive_agent_lane_preview(ChildEmissionPreviewJob::<A>{emit:Some(emit),ephemeral:Some(ephemeral),completion:Some(completion.clone()),maximum_bytes:contract.max_output_bytes,closing:false},child_params,&verb);
                    lease.finish();
                    let prepared_steps=prepared?;
                    match completion.take()?{
                        Some(ArtifactToolCompletionValue::Emit(emit,ephemeral))=>emit.map(|emit|(emit,(ephemeral.presence.len(),ephemeral.transient.len(),ephemeral.window_transient.len()),steps.saturating_add(prepared_steps))).map_err(ArtifactBoundedToolFault::into_fault),
                        _=>Err(plugin_sdk_fault("preview child preparation lost its complete owning emission")),
                    }
                },`);
const builderTest="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs";
const builderSource=await Bun.file(builderTest).text();
await region(builderTest,"checked-real-test-child-wire-projection","    fn label_in(label:",`    fn test_child_emit(slot:impl Into<String>,child_id:impl Into<String>,operations:&[TestMutation])->ChildEmit{
        let mut emit=ChildEmit::open(slot,child_id,operations.len());
        for operation in operations{emit.push::<TestSnapshot,_>(operation).expect("actual fixture mutation must encode completely");}
        emit
    }

    fn label_in(label:`);
await region(builderTest,"test-groups-through-checked-real-encoder","ChildEmit::of::<TestSnapshot, _>","test_child_emit",builderSource.split("ChildEmit::of::<TestSnapshot, _>").length-1);
const fixturePath="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs";
await region(fixturePath,"typed-fixture-owning-mutation-retirement","impl protocol::OpText for TestMutation {",`impl semio_framework_value::retirement::RetireOwned for TestMutation{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
        use semio_framework_value::retirement::RetireOwned;
        match self{Self::SetCount(operation)=>operation.value.retirement(),Self::SetLabel(operation)=>operation.value.retirement(),Self::SetSlotChildren(operation)=>operation.children.retirement()}
    }
}

impl protocol::OpText for TestMutation {`);
await region(builderTest,"actual-reducer-emits-owned-child-source","                child_emits: vec![test_child_emit(slot.clone(), child_id.clone(), &[TestMutation::SetCount(SetCount { value: *child_value })])],", "                child_preparations:std::collections::VecDeque::from([crate::app::ChildEmitPreparation::of::<TestSnapshot,_>(slot.clone(),child_id.clone(),vec![TestMutation::SetCount(SetCount{value:*child_value})])]),");
await region(builderTest,"full-allocation-prepublication-close-grant","assert!(released_items <= 1 && released_bytes <= 4);","assert!(released_items<=1&&released_bytes<=fixture[\"maximumBytes\"].as_u64().unwrap() as usize);");
const closeFixture="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧫️fixtures/🧩️child-prepublication-close.json";
await region(closeFixture,"physical-full-allocation-child-close-demand","  \"maximumBytes\": 4,","  \"maximumBytes\": 65536,\n  \"shortByteGrantPreservesExactAllocation\": true,");
const drawingTest="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️gesture-operation-owner/🦀️.rs";
await region(drawingTest,"empty-declared-child-source","semio_framework_plugin::app::ChildEmit::of::<DrawingSnapshot, DrawingMutation>(\"member\", \"drawing-child\", &[])","semio_framework_plugin::app::ChildEmit::open(\"member\",\"drawing-child\",0)");
await region(drawingTest,"whole-child-allocation-return-grant","job.close_step(1, 4)","job.close_step(1, 4_096)",3);
await region(drawingTest,"whole-child-allocation-assertion","released_bytes <= 4","released_bytes <= 4_096");
const writerTest="✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs";
await region(writerTest,"empty-declared-child-source","semio_framework_plugin::app::ChildEmit::of::<WriterSnapshot, WriterMutation>(\"member\", \"writer-child\", &[])","semio_framework_plugin::app::ChildEmit::open(\"member\",\"writer-child\",0)");
for(const filename of ["label-held-pair.json","borrowed-fault-projection-held-pair.json","closed-ownership-demand-held-pair.json","owned-native-law-held-pair.json"]){
    const held=await Bun.file(resolve(import.meta.dir,filename)).json();
    const candidates=held.pairs??[held];
    for(const pair of candidates){
        const current=await Bun.file(pair.path).text();
        if(current!==pair.before)throw Error(`Fresh prerequisite guard refused ${filename}: ${pair.path}`);
        await region(pair.path,`exact-${filename}`,pair.before,pair.after);
    }
}
const modulePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📨️emission/📦️preparation/🦀️.rs";
pairs.set(modulePath, { path: modulePath, before: "", after: await Bun.file(resolve(import.meta.dir, "🦀️.rs")).text(), regions: [] });
const output = { state: "HeldOwnedEmissionProducerAndOwnedHelperInvocationDraft", pairs: [...pairs.values()], required: ["all explicit constructor imports", "ToolRun decoded owned mutation handoff using actual Store factory", "actual retained job and mounted publication/preview pumps", "closed owner demand and tests", "typed close refusal ownership", "known builtin versus provider factory cell return"], limitations: ["Not mounted", "Not parsed or compiled", "No runtime credit", "Opaque custom transport source remains retained awaiting genuine provider handoff"] };
await Bun.write(resolve(ticket, "📥️inputs/child-emission-owned-capsule/producer-and-helpers-held-pairs.json"), JSON.stringify(output, null, 2) + "\n");
console.log(JSON.stringify({ paths: pairs.size, regions: [...pairs.values()].reduce((n, pair) => n + pair.regions.length, 0), state: output.state }));
