
const fs=require("fs");const root="🧰️framework/🛍️products/💻️os/🔨️modules/";const staged=[];
function edit(tail,run){const p=root+tail,before=fs.readFileSync(p,"utf8"),after=run(before);if(before===after)throw Error("unchanged "+tail);staged.push([p,before,after]);}
function one(s,a,b){if(s.split(a).length!==2)throw Error("expected one "+a.slice(0,80));return s.replace(a,b);}
for(const tail of ["🏪️store/📬️one-item-owners/📥️lifted/📬️publication/🦀️.rs","🏪️store/🫧️ephemeral/📤️ephemeral-one-item-publication/🦀️.rs"]){
 edit(tail,s=>one(s,"    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {",`    /// 📩️ Borrows the immutable accepted receipt without advancing or acknowledging its original publication.
    pub fn accepted_receipt(&self)->Option<LaneItemReceipt>{if self.phase==ArtifactStoreOneItemPublicationPhase::AwaitingAck{self.receipt.as_ref().copied()}else{None}}

    pub fn phase(&self) -> ArtifactStoreOneItemPublicationPhase {`));
}
for(const tail of ["🔌️plugin/🪟️window/🎚️config/🔏️pack-authority-and-publication/🦀️.rs","🔌️plugin/🪟️window/🫧️transient/🦀️.rs"]){
 edit(tail,s=>{
 const decl="    fn phase(&self) -> store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase;";
 s=one(s,decl,decl+"\n    fn accepted_receipt(&self)->Option<store::os_store::one_item::LaneItemReceipt>;");
 const marker="    fn fault(&self) -> Option<&str> {";
 const body=tail.includes("🎚️config")?"self.publication.as_ref().and_then(|publication|publication.accepted_receipt())":"self.publication.accepted_receipt()";
 return one(s,marker,"    fn accepted_receipt(&self)->Option<store::os_store::one_item::LaneItemReceipt>{"+body+"}\n\n"+marker);
 });
}
edit("🔌️plugin/🧩️composition/📬️publication/🦀️.rs",s=>one(s,"impl<A: ArtifactApp> PendingArtifactStorePublication<A> {",`impl<A: ArtifactApp> PendingArtifactStorePublication<A> {
    /// 🧾️ Returns the same accepted receipt while its original Store still awaits delivery acknowledgement.
    pub fn accepted_receipt(&self)->Option<store::os_store::one_item::LaneItemReceipt>{
        match self{
            Self::Artifact(publication)=>publication.accepted_receipt(),
            Self::Config(publication)=>publication.accepted_receipt(),
            Self::Draft(publication)=>publication.accepted_receipt(),
            Self::Presence(publication)=>publication.accepted_receipt(),
            Self::Transient(publication)=>publication.accepted_receipt(),
            Self::WindowConfig(publication)=>publication.accepted_receipt(),
            Self::WindowTransient(publication)=>publication.accepted_receipt(),
        }
    }
`));
for(const[p,before]of staged)if(fs.readFileSync(p,"utf8")!==before)throw Error("concurrent "+p);
for(const[p,,after]of staged){fs.writeFileSync(p,after);console.log(p);}
