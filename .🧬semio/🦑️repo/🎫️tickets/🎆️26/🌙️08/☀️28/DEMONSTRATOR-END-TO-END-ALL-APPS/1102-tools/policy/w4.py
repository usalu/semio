import sys,os,collections,json
sys.path.insert(0,".")
import mk
from mk import R,crate_of,build
mk.OUT=os.path.dirname(os.path.abspath(__file__))+"/patches/w4"
W=[]
def add(rel,edits,note):W.append((rel,edits,note))
def block(rel,start,end_marker):
    t=open(f"{R}/{rel}",encoding="utf8").read()
    i=t.index(start);j=t.index(end_marker,i)+len(end_marker);return t[i:j]
OS="🧰️framework/🛍️products/💻️os/🔨️modules"
M="🧰️framework/🔨️modules"
VEI="semio_framework_value::ValueError::invariant"
# 1 flow host-close
HC=f"{OS}/🌊️flow/🖥️host/🏁️host-close/🦀️.rs"
old=block(HC,"    pub fn retire_cold(self) {","\n    }\n")
new='''    pub fn retire_cold(self)->Result<(),ValueError>{
        let mut retirement=FlowHostRetirement::new(self);
        let mut idle=0usize;while !retirement.terminal_is_empty(){let copy=retirement.next_close_copy_byte_demand()?.max(4096);let demand=retirement.close_demands(copy)?;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let step=retirement.close_step(grant)?;if step.progress()==RetainedCloneProgress::default(){idle+=1;if idle>64{return Err(ValueError::invariant("cold original host close stalled at exact demand"));}}else{idle=0;}}
        Ok(())
    }
'''
add(HC,[(old,new)],"flow host-close: FlowHost::retire_cold now returns Result<(),ValueError> (option a); [DEBUG] eprintln and 5 expects removed")
# 2 flow retained push_cold
FR=f"{OS}/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs"
old=block(FR,"    pub fn push_cold(&mut self,owner:FlowOwner) {","\n    }\n")
new='''    pub fn push_cold(&mut self,owner:FlowOwner)->Result<(),ValueError>{
        if self.root.is_none(){return self.push(owner).map(|_|()).map_err(|_|ValueError::invariant("empty Flow root admission refused"));}
        while !self.queue.has_reserved_slot(){self.reserve_push(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:self.queue.next_reserve_capacity_byte_demand()?,maximum_release_bytes:0,maximum_depth:self.queue.reserve_demand()?.depth}).result?;}
        self.admit_owner(owner,RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<FlowOwner>(),maximum_release_bytes:0,maximum_depth:self.queue.len()+1}).result.map(|_|()).map_err(|(error,_)|error)
    }
'''
add(FR,[(old,new)],"flow retained: FlowRetirement::push_cold now returns Result<(),ValueError> (option a)")
# 3 report
RP=f"{OS}/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs"
add(RP,[
 (".unwrap_or(owner.history_issuer.as_ref().unwrap())",".map_or_else(||owner.history_issuer.as_ref().ok_or(VcsError::HistoryReplaying),Ok)?"),
 ("input:owner.history_input.take().unwrap(),","input:owner.history_input.take().ok_or_else(||VcsError::HistoryReplaying)?,"),
 ("self.retain_refused_replay_request(request.take().unwrap())","self.retain_refused_replay_request(request.take().ok_or_else(||VcsError::HistoryReplaying)?)")],"os-store replay report: 3 unwraps -> VcsError::HistoryReplaying")
# 4 maintenance ladder
ML=f"{OS}/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs"
add(ML,[('let owner=active.hydration.as_mut().expect("recursive document parent hydration remains retained");','let owner=active.hydration.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("recursive document parent hydration is not retained"))?;')],"maintenance ladder: expect -> ValueError (PluginLifecycleFailure: From<ValueError>)")
# 5 gen3d
G3="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
for rel in (f"{G3}/✏️editor/🪟️artifact-editor-impl/🦀️.rs",f"{G3}/👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs"):
    add(rel,[('let window_id=original_reply_text(&mut args,"windowId",true).unwrap();let window_kind_id=original_reply_text(&mut args,"windowKindId",true).unwrap();let output=original_reply_output(&mut args).unwrap();','let window_id=original_reply_text(&mut args,"windowId",true)?;let window_kind_id=original_reply_text(&mut args,"windowKindId",true)?;let output=original_reply_output(&mut args)?;')],"generation3d command_from_action returns Result<_,Fault>: 3 unwraps -> ? (Fault)")
# 7 ifc
I="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards"
add(f"{I}/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/work.rs",[("return self.type_change.demands(self.instance.as_ref().unwrap());",'return self.type_change.demands(self.instance.as_ref().ok_or_else(||ValueError::invariant("IFC2x3 type change has no instance"))?);')],"ifc work.rs:20/1404: unwrap -> ValueError::invariant")
add(f"{I}/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/staging-native/🦀️.rs",[("else{metadata(cx,size_of::<bool>(),||{self.page.as_mut().unwrap().status.ready=!self.identities.draft;self.phase=20;})?;}",'else{let page=self.page.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("IFC4x3 template page is absent"))?;metadata(cx,size_of::<bool>(),||{page.status.ready=!self.identities.draft;self.phase=20;})?;}')],"ifc4x3 staging-native: receiver hoisted out of the closure")
# 8 raster
RA="✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖼️asset-content-and-patch/🦀️.rs"
old=block(RA,"pub fn close_raster_owner_cold(mut owner: Box<dyn store::ErasedSnapshotRetirement>) {","\n}\n")
new='''pub fn close_raster_owner_cold(mut owner: Box<dyn store::ErasedSnapshotRetirement>) -> Result<(), semio_framework_value::ValueError> {
    let mut turns = 0usize;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(RASTER_OWNED_FIELD_BYTES)?;
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RASTER_OWNED_FIELD_BYTES.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        owner.close_step(grant).result?;
        turns += 1;
        if turns >= 1 << 24 { return Err(semio_framework_value::ValueError::invariant("Raster cold retirement made no progress")); }
    }
    Ok(())
}
'''
g=[l for l in old.split("\n") if "let grant" in l][0]
new=new.replace([l for l in new.split("\n") if "let grant" in l][0],g)
RV=block(RA,"pub fn retire_raster_value_cold<T: semio_framework_value::retirement::RetireOwned>(value: T) {","\n}\n")
RVN='''pub fn retire_raster_value_cold<T: semio_framework_value::retirement::RetireOwned>(value: T) -> Result<(), semio_framework_value::ValueError> {
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), maximum_depth: 2, ..Default::default() };
    let (owner, _) = semio_framework_value::retirement::admit_owned_retirement(value, birth).result.map_err(|(error, _)| error)?;
    close_raster_owner_cold(owner)
}
'''
add(RA,[(RV,RVN),(old,new)],"raster asset-content: close_raster_owner_cold and retire_raster_value_cold return Result<(),ValueError> (option a)")

# 9 draw
DP="✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
add(f"{DP}/🎬️scene/📷️raster/🦀️.rs",[("if matches!(n.content,RasterSceneContent::Path{..}){let RasterSceneContent::Path{segments,..}=&n.content else{unreachable!()};","if let RasterSceneContent::Path{segments,..}=&n.content{")],"draw raster scene: matches!+let-else(unreachable) -> if let (unrepresentable)")
add(f"{DP}/🧮️geometry/🎯️picking/🎨️paint/🦀️.rs",[('"complete"=>{},_=>unreachable!(),','"complete"=>{},_=>return Err("Painted query phase is invalid".into()),')],"draw picking paint: unreachable -> Err(String)")
add(f"{DP}/📸️snapshot/🔎️lookup/🦀️.rs",[('self.target.expect("retained Drawing lookup target")','self.target.ok_or_else(||ValueError::invariant("retained Drawing lookup target is absent"))?')],"draw lookup: expect -> ValueError::invariant")
add(f"{DP}/📸️snapshot/🔎️lookup/🔗️owned/🦀️.rs",[
 ('self.source.as_ref().expect("retained native Drawing lookup root").project(1,','self.source.as_ref().ok_or_else(|| ValueError::invariant("retained native Drawing lookup root is absent"))?.project(1,'.replace("project(1,","project(1, ")),
 ('self.current.as_ref().expect("compared native Drawing group").project(1, |layer| match layer { DrawingLayerNode::Group(group) => &group.children, _ => unreachable!() }, grant)','self.current.as_ref().ok_or_else(|| ValueError::invariant("compared native Drawing group is absent"))?.try_project(1, |layer| match layer { DrawingLayerNode::Group(group) => Some(&group.children), _ => None }, grant)')],"draw lookup owned: 2 expects + unreachable; uses new RetainedOwnedProjection::try_project")
add(f"{DP}/✅️validation/🦀️.rs",[('let root=s.get().downcast_ref::<DrawingSnapshot>().expect("checked Drawing owner");','let root=s.get().downcast_ref::<DrawingSnapshot>().ok_or_else(||fault("Drawing admission requires its original typed DrawingSnapshot"))?;')],"draw validation: expect -> fault()")
# framework-value retained-clone: new try_project (definition first)
PJ=f"{M}/🌱️value/🧬️retained-clone/🔗️projection/🦀️.rs"
anchor="    /// ♻️ Releases one alias while the source authority continues to retain the root lease.\n"
newfn='''    /// 🪤️ Projects through a fallible accessor so an absent native child refuses instead of aborting.
    pub fn try_project<U: ?Sized + Sync, F>(&self, discriminator: usize, project: F, grant: RetainedCloneGrant) -> crate::RetainedWorkOutcome<(RetainedOwnedProjection<U>,RetainedCloneProgress)>
    where F: for<'source> FnOnce(&'source T) -> Option<&'source U> {
        if let Err(error)=RetainedOwnedProjection::<U>::admit_constructor(grant){return crate::RetainedWorkOutcome{result:Err(error),work:0}}
        let source=match self.borrow(){Ok(source)=>source,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};
        match source.try_project(discriminator,|value|project(value).ok_or(())){Ok(child)=>unsafe{RetainedOwnedProjection::from_source(child,grant)},Err(())=>crate::RetainedWorkOutcome{result:Err(ValueError::invariant("native projection child is absent")),work:0}}
    }

'''
add(PJ,[(anchor,newfn+anchor)],"framework-value retained-clone: RetainedOwnedProjection::try_project (definition for the draw lookup)")
ONLY=os.environ.get('ONLY')
g=collections.OrderedDict()
for rel,e,n in W:
    if ONLY and crate_of(rel)!=ONLY:continue
    g.setdefault(crate_of(rel),[]).append((rel,e,n))
rows=[]
for cr,l in g.items():
    spec={}
    for rel,e,n in l:spec.setdefault(rel,[]).extend(e)
    build(cr,spec)
    rows.append({"crate":cr,"files":len(spec),"edits":sum(len(v) for v in spec.values()),"notes":[n for _,_,n in l],"check":open(f"{mk.OUT}/{cr}.check").read()[:300]})
json.dump(rows,open(f"{mk.OUT}/index.json","w"),ensure_ascii=False,indent=1)
print(len(rows),"patches")
