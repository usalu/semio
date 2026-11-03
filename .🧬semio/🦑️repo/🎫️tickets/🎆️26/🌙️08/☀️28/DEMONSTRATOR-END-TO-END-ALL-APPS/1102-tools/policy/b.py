OS="🧰️framework/🛍️products/💻️os/🔨️modules"
VE="semio_framework_value::ValueError"
def iv(m,v=VE+"::invariant"):return f'.ok_or_else(||{v}("{m}"))?'
def ls(var,expr,m):return f'let Some({var})={expr} else {{ return Err({VE}::invariant("{m}")); }};'
B=[]
def add(rel,edits,note):B.append((f"{OS}/{rel}",edits,note))
add("🏪️store/👥️presence/♻️retirement/🦀️.rs",[
 ('self.local_factory.as_ref().expect("observed original local factory")','self.local_factory.as_ref()'+iv("original local factory is absent")),
 ("let factory = slot.as_mut().unwrap();",ls("factory","slot.as_mut()","original factory close slot is absent"))],"ValueError::invariant; close_step -> Result<_,ValueError>; 2 sites")
add("🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs",[
 ("let owner=slot.as_mut().unwrap();",ls("owner","slot.as_mut()","handoff cursor slot is absent")),
 ('self.snapshot_retirement.as_ref().expect("observed original snapshot factory")','self.snapshot_retirement.as_ref()'+iv("original snapshot factory is absent")),
 ("let factory = slot.as_mut().unwrap();",ls("factory","slot.as_mut()","original factory close slot is absent"))],"ValueError::invariant; 3 sites")
add("🏪️store/🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs",[
 ('let factory = self.factory_close.as_mut().expect("retained transient factory ticket");','let factory = self.factory_close.as_mut()'+iv("retained transient factory ticket is absent")+";")],"ValueError::invariant; 1 site")
add("🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs",[
 ('return match self.mutation_retirement.as_ref().expect("original mutation factory")','return match self.mutation_retirement.as_ref()'+iv("original mutation factory is absent")),
 ("let factory = slot.as_mut().unwrap();",ls("factory","slot.as_mut()","original factory close slot is absent"))],"ValueError::invariant; 2 sites")
add("🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🦀️.rs",[
 ("let factory=self.closing_factory[index].as_mut().unwrap();",ls("factory","self.closing_factory[index].as_mut()","history closing factory slot is absent").replace("; };","};"))],"ValueError::invariant; 1 site")
add("🏪️store/🔁️replay/🎮️operation/🧬️native/🦀️.rs",[
 ("Stage::SourceCapture=>{let Some((source,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().unwrap(),grant)","Stage::SourceCapture=>{let Some((source,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref()"+iv("replay snapshot issuer is absent")+",grant)"),
 ("let Some((authority,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref().unwrap(),grant)","let Some((authority,p))=CapturedSnapshotSource::admit(&mut self.base,self.snapshots.as_ref()"+iv("replay snapshot issuer is absent")+",grant)"),
 ("Stage::Inverse=>{let pages=self.pages.as_mut().unwrap();","Stage::Inverse=>{let pages=self.pages.as_mut()"+iv("replay inverse pages are absent")+";"),
 ("let original=self.pending.take().unwrap();","let original=self.pending.take()"+iv("replay pending inverse is absent")+";"),
 ("let Some((row,p))=self.rows.as_mut().unwrap().take_front(grant)","let Some((row,p))=self.rows.as_mut()"+iv("replay inverse rows are absent")+".take_front(grant)"),
 ("{self.rows.as_mut().unwrap().step(grant).result?.progress()}","{self.rows.as_mut()"+iv("replay inverse rows are absent")+".step(grant).result?.progress()}"),
 ("admit_original(&mut self.post,self.snapshots.as_ref().unwrap(),grant)","admit_original(&mut self.post,self.snapshots.as_ref()"+iv("replay snapshot issuer is absent")+",grant)"),
 ("let owner=self.factory_close[index].as_mut().unwrap();",ls("owner","self.factory_close[index].as_mut()","replay factory close slot is absent").replace("; };","};"))],"ValueError::invariant; advance/close_step -> Result<_,ValueError>; 7 sites")
add("🏪️store/🔁️replay/🧬️supersession/🦀️.rs",[
 ("let job=self.job.as_mut().unwrap();",'let job=self.job.as_mut().ok_or_else(||invalid("native supersession job is absent"))?;'),
 ("let owner=self.retirement.as_ref().unwrap();if owner.terminal_is_empty()",'let owner=self.retirement.as_ref().ok_or_else(||invalid("native supersession retirement is absent"))?;if owner.terminal_is_empty()'),
 ("close_original_retained_step(cx,d,|grant|{let owner=self.retirement.as_mut().unwrap();let result=owner.step(grant);",'let owner=self.retirement.as_mut().ok_or_else(||invalid("native supersession retirement is absent"))?;close_original_retained_step(cx,d,|grant|{let result=owner.step(grant);'),
 ("source:job.take_rejection().unwrap(),",'source:job.take_rejection().ok_or_else(||invalid("native supersession rejection is absent"))?,')],"ValueError via invalid() (OriginalTurnFailure: From<ValueError>); 2 sites")
add("🏪️store/🕰️history/🎮️command/🦀️.rs",[
 ("let selected=f.selection.unwrap();",'let selected=f.selection.ok_or_else(||refusal("history edit selection is absent"))?;'),
 ("OriginalHistoryEditSelection{key:f.key.unwrap(),",'OriginalHistoryEditSelection{key:f.key.ok_or_else(||refusal("history edit key is absent"))?,')],"ValueError via refusal(); 2 sites")
H=lambda v,m:f'let owner=self.{v}.as_mut()'+iv(m)+";"
add("🏪️store/🕰️history/🏁️replay/checkpoint.rs",[
 ("let owner=self.metadata.as_mut().unwrap();",H("metadata","history checkpoint metadata owner is absent")),
 ("let owner=self.batch.as_mut().unwrap();",H("batch","history checkpoint batch owner is absent"))],"ValueError::invariant -> OriginalTurnFailure; 2 sites")
add("🏪️store/🕰️history/🏁️replay/repin.rs",[
 ("let owner=self.batch.as_mut().unwrap();",H("batch","history repin batch owner is absent")),
 ("self.stage.as_mut().unwrap().close_step(store,cx)?","self.stage.as_mut()"+iv("history repin stage is absent")+".close_step(store,cx)?"),
 ("if self.checkpoint_staged{let d=store.parts_ref().log.envelope.vcs.checkpoints.replacement_settlement_demands(self.key.unwrap())?;","if self.checkpoint_staged{let key=self.key"+iv("history repin key is absent")+";let d=store.parts_ref().log.envelope.vcs.checkpoints.replacement_settlement_demands(key)?;"),
 ("settle_replacement(self.key.unwrap(),&mut self.checkpoint","settle_replacement(key,&mut self.checkpoint")],"ValueError::invariant -> OriginalTurnFailure; 3 sites")
add("🏪️store/🕰️history/🏁️replay/📬️publication/🦀️.rs",[("4=>{let owner=self.wire.as_mut().unwrap();if owner.ready()","4=>{"+H("wire","history publication wire owner is absent")+"if owner.ready()"),("5=>{let owner=self.wire.as_mut().unwrap();","5=>{"+H("wire","history publication wire owner is absent"))],"ValueError::invariant; 2 sites")
add("🏪️store/🕰️history/🏁️replay/🦀️.rs",[("let owner=self.transitions.as_mut().unwrap();",'let owner=self.transitions.as_mut().ok_or_else(||invalid("history replay transitions owner is absent"))?;')],"ValueError via invalid(); 1 site")
add("🏪️store/⚙️engine/🌿️branch/📜️transitions/🦀️.rs",[
 ("3=>{if self.branch_source.as_mut().unwrap().terminal_is_empty(){","3=>{if self.branch_source.as_mut()"+iv("branch source owner is absent")+".terminal_is_empty(){"),
 ("}else{self.branch_source.as_mut().unwrap().close_step(cx)?;}","}else{self.branch_source.as_mut()"+iv("branch source owner is absent")+".close_step(cx)?;}"),
 ("retired(cx,d,|_|{drop(self.branch_source.take());let branch=self.branch.as_ref().unwrap();","let branch=self.branch.as_ref()"+iv("branch commit owner is absent")+";retired(cx,d,|_|{drop(self.branch_source.take());")],"ValueError::invariant -> OriginalTurnFailure; 2 sites")
T="🔌️plugin/👷️job/🛬️receiving/🔀️transaction/🦀️.rs"
iT=lambda m:'.ok_or_else(||invalid("'+m+'"))?'
add(T,[
 ("let fields=self.fields.as_mut().unwrap();if fields.operations.capacity()==0","let fields=self.fields.as_mut()"+iT("transaction receiving fields are absent")+";if fields.operations.capacity()==0"),
 ("1=>{let fields=self.fields.as_mut().unwrap();","1=>{let fields=self.fields.as_mut()"+iT("transaction receiving fields are absent")+";"),
 ("let parser=self.children.as_mut().unwrap();if parser.advance(cx)?","let parser=self.children.as_mut()"+iT("transaction child parser is absent")+";if parser.advance(cx)?"),
 ("2=>{let parser=self.children.as_mut().unwrap();if self.fields.as_ref().unwrap().children.is_none(){parser.take(cx,&mut self.fields.as_mut().unwrap().children)?;","2=>{let parser=self.children.as_mut()"+iT("transaction child parser is absent")+";if self.fields.as_ref()"+iT("transaction receiving fields are absent")+".children.is_none(){parser.take(cx,&mut self.fields.as_mut()"+iT("transaction receiving fields are absent")+".children)?;"),
 ("4=>{let parser=self.origin.as_mut().unwrap();if self.fields.as_ref().unwrap().origin.is_none(){parser.take(cx,&mut self.fields.as_mut().unwrap().origin)?;","4=>{let parser=self.origin.as_mut()"+iT("transaction origin parser is absent")+";if self.fields.as_ref()"+iT("transaction receiving fields are absent")+".origin.is_none(){parser.take(cx,&mut self.fields.as_mut()"+iT("transaction receiving fields are absent")+".origin)?;"),
 ("5=>{let fields=self.fields.as_mut().unwrap();let value=","5=>{let fields=self.fields.as_mut()"+iT("transaction receiving fields are absent")+";let value=")],"ValueError via invalid(reason) -> OriginalTurnFailure; 9 sites")
iG=lambda m:'.ok_or_else(||invalid("'+m+'"))?'
add("🔌️plugin/🌱️initialization/🌱️genesis/🦀️.rs",[
 ("->Result<(),OriginalTurnFailure>{let cursor=self.cursor.as_ref().unwrap();","->Result<(),OriginalTurnFailure>{let cursor=self.cursor.as_ref()"+iG("Store genesis cursor is absent")+";"),
 ("O::initial_state_pack_source(self.sources.initial.as_ref().unwrap())?","O::initial_state_pack_source(self.sources.initial.as_ref()"+iG("Store genesis initial source is absent")+")?"),
 ("let result=self.cursor.as_mut().unwrap().advance(source,spec,target,","let result=self.cursor.as_mut()"+iG("Store genesis cursor is absent")+".advance(source,spec,target,"),
 ("if let Some(length)=self.cursor.as_ref().unwrap().output_len(){","if let Some(length)=self.cursor.as_ref()"+iG("Store genesis cursor is absent")+".output_len(){"),
 ("4=>{let cursor=self.cursor.as_mut().unwrap();","4=>{let cursor=self.cursor.as_mut()"+iG("Store genesis cursor is absent")+";"),
 ("if self.outcome.original().unwrap().kind()==","if self.outcome.original()"+iG("Store native initializer outcome is absent")+".kind()=="),
 ("let initializer=self.initializer.as_mut().unwrap();if self.ready{","let initializer=self.initializer.as_mut()"+iG("Store native initializer is absent")+";if self.ready{"),
 ("7=>{let initializer=self.initializer.as_mut().unwrap();","7=>{let initializer=self.initializer.as_mut()"+iG("Store native initializer is absent")+";")],"ValueError via invalid(message); 8 sites")
add("🔌️plugin/⚛️reactor/🔄️turn/📥️receiving/staged.rs",[
 ("self.inputs.as_mut().unwrap().issue=Some(fold(error));","self.inputs.as_mut().ok_or_else(invalid)?.issue=Some(fold(error));"),
 ("take_received_wire_value(self.receiver.as_mut().unwrap(),&mut self.inputs.as_mut().unwrap().value,","take_received_wire_value(self.receiver.as_mut().ok_or_else(invalid)?,&mut self.inputs.as_mut().ok_or_else(invalid)?.value,")],"ValueError via invalid(); 3 sites")
add("🔌️plugin/⚛️reactor/🪟️surfaces/native.rs",[
 ("2=>{let source=self.source.as_ref().unwrap().borrow();","2=>{let source=self.source.as_ref().ok_or_else(invalid)?.borrow();"),
 ("let d=self.source.as_ref().unwrap().next_take_demand(body)?;","let d=self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body)?;"),
 ("match self.source.as_mut().unwrap().take_authority(g).result?{","match self.source.as_mut().ok_or_else(invalid)?.take_authority(g).result?{")],"ValueError via invalid(); 3 sites")
add("🔌️plugin/👷️job/🛬️receiving/🧾️copies/🦀️.rs",[
 ("2=>{if self.values.as_ref().unwrap().len()==self.total{","2=>{if self.values.as_ref().ok_or_else(invalid)?.len()==self.total{"),
 ("let source=self.source.as_ref().unwrap().try_borrow()?;let cursor=self.cursor.as_mut().unwrap();","let source=self.source.as_ref().ok_or_else(invalid)?.try_borrow()?;let cursor=self.cursor.as_mut().ok_or_else(invalid)?;")],"ValueError via invalid(); 3 sites")
add("🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/metadata.rs",[
 ("6=>{let d=self.receiver.as_ref().unwrap().demands(body)?;receive(cx,d,|control,g|self.receiver.as_mut().unwrap().advance(control,body,g).result.map_err(fold))?;if self.receiver.as_ref().unwrap().output_ready(){self.phase=7}}","6=>{let receiver=self.receiver.as_mut()"+iv("manifest metadata receiver is absent")+";let d=receiver.demands(body)?;receive(cx,d,|control,g|receiver.advance(control,body,g).result.map_err(fold))?;if receiver.output_ready(){self.phase=7}}"),
 ("7=>{let d=self.receiver.as_ref().unwrap().take_demand()?;let g=cx.retained_grant();built(cx,&mut *self.view,construction(d),||{let(view,progress)=self.receiver.as_mut().unwrap().take(g).result?","7=>{let receiver=self.receiver.as_mut()"+iv("manifest metadata receiver is absent")+";let d=receiver.take_demand()?;let g=cx.retained_grant();built(cx,&mut *self.view,construction(d),||{let(view,progress)=receiver.take(g).result?"),
 ("self.receiver.as_mut().unwrap().begin_close();self.phase=8;","receiver.begin_close();self.phase=8;")],"ValueError::invariant -> OriginalTurnFailure; receiver hoisted out of the closures; 3 sites")
add("🔌️plugin/👷️job/🛬️receiving/📥️pack/🦀️.rs",[
 ("if self.admission.is_some(){let admission=self.admission.as_ref().unwrap();","if let Some(admission)=self.admission.as_ref(){"),
 ("retired(cx,d,|grant|{let admission=self.admission.as_mut().unwrap();if admission.phase()","retired(cx,d,|grant|{let Some(admission)=self.admission.as_mut()else{return Err(invalid())};if admission.phase()")],"unrepresentable (if let / let-else with invalid()); 2 sites")
if __name__=="__main__":
    from mk import *
    for rel,edits,note in B:
        build("probe",{rel:edits}) 
