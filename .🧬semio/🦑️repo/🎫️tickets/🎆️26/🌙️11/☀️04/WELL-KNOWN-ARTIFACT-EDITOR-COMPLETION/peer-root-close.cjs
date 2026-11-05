
const fs=require("fs");
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
const staged=[];
function change(path,run){const before=fs.readFileSync(root+path,"utf8"),after=run(before);if(before===after)throw Error("no edit "+path);staged.push([root+path,before,after]);}
function one(s,a,b){if(s.split(a).length!==2)throw Error("expected one "+a.slice(0,80));return s.replace(a,b);}
change("📜️declaration/👁️view/🫂️peer-presence/🦀️.rs",s=>{
const start=s.indexOf("    pub fn empty() -> Self {"),end=s.indexOf("    pub fn insert(",start);if(start<0||end<0)throw Error("empty range");
return s.slice(0,start)+`    /// 🈳️ An absent roster borrows this immutable empty value without allocating shared ownership.
    pub const EMPTY:Self=Self{entries:[const{None};PEER_PRESENCE_SLOTS],len:0};

    pub fn empty() -> Self {Self::EMPTY}

`+s.slice(end);
});
change("🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs",s=>{
const start=s.indexOf("    pub fn detach_peer_presence_step("),end=s.indexOf("\n    }",start)+6;
if(start<0||end<6)throw Error("detach range");
s=s.slice(0,start)+`    /// ♻️ Retains the exact final presence root until its independently paid retirement finishes.
    pub fn detach_peer_presence_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let demand=peer_presence_detach_demand(&*self.roster,grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if let Some(owner)=self.roster.peer_presence_close.as_mut(){
            if !owner.terminal_is_empty(){
                let step=owner.close_step(grant).map_err(plugin_retirement_fault)?;
                return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()))
            }
            self.roster.peer_presence_close.take();
            self.close.close_peer_presence_detached=true;
        }else if let Some(root)=self.roster.peer_presence.take(){
            *self.roster.peer_presence_close=Some(PeerPresenceRootRetirement::new(root));
        }else{self.close.close_peer_presence_detached=true;}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }`+s.slice(end);
const marker="impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static, X: InstanceExtension> LadderRef<'v, A, M, X> {";
s=one(s,marker,`fn peer_presence_detach_demand<A:ArtifactApp>(roster:&PeerRoster<A>,body:usize)->Result<RetirementDemand,ValueError>{
    if let Some(owner)=roster.peer_presence_close.as_ref(){
        return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<PeerPresenceRootRetirement>>()+size_of::<bool>(),depth:1,..Default::default()})}else{owner.retirement_demands(body)}
    }
    Ok(RetirementDemand{copy_bytes:if roster.peer_presence.is_some(){2*size_of::<Option<PeerPresenceRootRetirement>>()+2*size_of::<Option<std::sync::Arc<PeerPresenceRoot>>>()}else{size_of::<bool>()},depth:1,..Default::default()})
}

`+marker+`
    /// 📐️ The final presence close quotes its actual owner instead of an empty replacement root.
    pub fn detach_peer_presence_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{peer_presence_detach_demand(self.roster,body)}
`);
return s;});
change("🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs",s=>{
s=one(s," | CloseRung::DetachPeerPresence => Ok(handoff_demand()),"," => Ok(handoff_demand()),\n            CloseRung::DetachPeerPresence => self.ladder_ref().detach_peer_presence_demands(body),");
s=one(s,"self.ladder_ctx().detach_peer_presence_step()","self.ladder_ctx().detach_peer_presence_step(grant)");
return s;});
for(const[path,before]of staged)if(fs.readFileSync(path,"utf8")!==before)throw Error("concurrent edit "+path);
for(const[path,,after]of staged){fs.writeFileSync(path,after);console.log(path);}
