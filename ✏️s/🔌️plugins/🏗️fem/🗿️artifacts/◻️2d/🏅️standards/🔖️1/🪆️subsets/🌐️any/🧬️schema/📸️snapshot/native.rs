//! 🛬️ Original authoring owners receive persistent FEM rows through physical grants and retain every close frontier.
use super::{Fem2dSnapshot, Fem2dSnapshotValue};
use semio_framework_value::{
    OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep, StagedSourceAdmission, retained_clone_grant_admits},
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
};
use semio_framework_value_native_owned::{OriginalReceivingPrimary, native_decoding::NativeReceivingFailure};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl, OriginalConstructionAccept, OriginalConstructionAcceptComplete, OriginalConstructionAuthority, OriginalConstructionGrant, OriginalConstructionGrantComplete, OriginalConstructionRecipient, OriginalSourceTurnIdentity, owned::{NativeReceiveSlot, native_work_limit}};
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_s_fem_material::planar::{FemMaterial, FemMaterialValue};
use semio_s_fem_node::planar::{FemNode, FemNodeValue};
use std::mem::{ManuallyDrop, size_of};
type Nodes = PersistentList<FemNode>;
type Materials = PersistentList<FemMaterial>;
type NodeReceiver = NativeReceiveSlot<Vec<FemNodeValue>, Nodes>;
type MaterialReceiver = NativeReceiveSlot<Vec<FemMaterialValue>, Materials>;
const ACTION_WORK: u64 = 4;
/// 🪢️ A synchronous recipient never leaves a child pending, so a pending poll is a typed refusal instead of a loop.
fn settle<T, E: From<ValueError>>(poll: RetainedWorkPoll<T, E>) -> RetainedWorkOutcome<T, E> {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn,
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "FEM receiving child cannot stay pending on a synchronous recipient").into()), work },
    }
}
/// 📬️ Runs one reply-shaped step inside its own action; a reply left pending is refused typed.
fn settled<T, E: From<ValueError>>(effect: &mut RetainedAllocationEffect<'_>, call: impl for<'r, 'e, 'a> FnOnce(&'e mut RetainedAllocationEffect<'a>, OriginalEffectReply<'r, T, E>) -> Result<OriginalEffectReplyComplete<'r, T, E>, ValueError>) -> RetainedWorkPoll<T, E> {
    let before = effect.performed_work();
    let mut slot = None;
    let result = (|| -> Result<Result<T, E>, ValueError> {
        let done = call(&mut *effect, OriginalEffectReply::borrow(&mut slot)?)?;
        drop(done);
        slot.take().ok_or_else(invalid)
    })();
    RetainedWorkPoll::Complete(RetainedWorkOutcome { result: match result { Ok(result) => result, Err(error) => Err(error.into()) }, work: effect.performed_work().saturating_sub(before) })
}
/// 🪢️ Maps a native receiving failure onto the snapshot failure that keeps its exact cause and progress.
fn receiving_failure(failure: NativeReceivingFailure) -> store::NativeSnapshotFailure {
    match failure {
        NativeReceivingFailure::Primary(primary) => primary.into(),
        NativeReceivingFailure::ValueEffect(effect) => effect.child.or(effect.producer_refusal).or(effect.work_refusal).map_or_else(|| effect.receiving.into(), Into::into),
        NativeReceivingFailure::RetirementEffect(effect) => effect.child.map(store::NativeSnapshotFailure::from).or_else(|| effect.producer_refusal.or(effect.work_refusal).map(Into::into)).unwrap_or_else(|| effect.receiving.into()),
        NativeReceivingFailure::ReceivingEffect(effect) => effect.child.map(store::NativeSnapshotFailure::from).or_else(|| effect.producer_refusal.or(effect.work_refusal).map(Into::into)).unwrap_or_else(|| effect.receiving.into()),
    }
}
fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "FEM receiving requires its original source and funded phase")
}
fn fixed(bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }
}
fn receipt(d: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, retained_capacity_bytes: d.capacity_bytes, released_bytes: d.release_bytes }
}


struct OriginalStepRecipient {
    grant: RetainedCloneGrant,
    progress: RetainedCloneProgress,
    submitted: RetainedCloneProgress,
}
impl OriginalStepRecipient {
    fn submitted_progress(&self)->RetainedCloneProgress{self.submitted}
    fn grant_for(&mut self, d: RetirementDemand) -> Result<RetainedCloneGrant, ValueError> {
        let remaining = RetainedCloneGrant {
            maximum_items: self.grant.maximum_items.saturating_sub(self.progress.copied_items),
            maximum_copy_bytes: self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),
            maximum_capacity_bytes: self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),
            maximum_release_bytes: self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),
            ..self.grant
        };
        if !retained_clone_grant_admits(remaining, d) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "FEM sealed row exceeds the original receiving step grant"));
        }
        Ok(remaining)
    }
    fn accept_for(&mut self, g: RetainedCloneGrant, p: RetainedCloneProgress) -> Result<(), ValueError> {
        self.submitted=self.submitted.checked_add(p)?;
        if !p.fits(g) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "FEM sealed row receipt exceeds its original receiving grant").with_retained_progress(p));
        }
        self.progress = self.progress.checked_add(p)?;
        Ok(())
    }
}
impl OriginalConstructionRecipient for OriginalStepRecipient {
    type Context<'turn, 'source> = () where 'source: 'turn;
    fn validate_context<'turn, 'source>(&self, _: &()) -> Result<(), ValueError> where 'source: 'turn { Ok(()) }
    fn source_turn<'turn, 'source>(&self, _: &()) -> Option<OriginalSourceTurnIdentity> where 'source: 'turn { None }
    fn grant<'reply, 'turn, 'source>(&mut self, _: &mut (), quote: OriginalConstructionGrant<'reply>) -> OriginalConstructionGrantComplete<'reply> where 'source: 'turn { let result = self.grant_for(quote.demand()); quote.complete(result) }
    fn accept<'reply, 'turn, 'source>(&mut self, _: &mut (), receipt: OriginalConstructionAccept<'reply>) -> OriginalConstructionAcceptComplete<'reply> where 'source: 'turn { let result = self.accept_for(receipt.grant(), receipt.progress()); receipt.complete(result) }
    fn submitted<'turn, 'source>(&self, _: &()) -> RetainedCloneProgress where 'source: 'turn { self.submitted_progress() }
}

impl Fem2dSnapshot {
    /// 🫴️ Receives an already admitted authoring owner inside the original snapshot wallet and retirement publication frame.
    pub fn receive_authoring(value:&mut Option<Fem2dSnapshotValue>,output:&mut Option<Self>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
        owner.receive_nested::<Fem2dReceiving,Self,store::NativeSnapshotFailure>(output,&mut |slot,output,birth,birth_unwind,original|store::native_snapshot_turn(|work|{
            work.take(validate_authoring(value.as_ref().ok_or_else(invalid)?,original))?;
            let g=original.remaining_grant();let d=Fem2dReceiving::birth_demand();
            if !retained_clone_grant_admits(g,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original FEM snapshot wallet cannot capture its authoring owner").into())}
            let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            let turn=original.effect(demand,birth,birth_unwind,&mut |performed,physical|{let born=Fem2dReceiving::empty(g);let actual=born.work;let(rx,p)=born.result?;*slot=Some(rx);*performed=true;*physical=p;if actual!=1{return Err(invalid())}Ok(())});work.take(turn)?;birth.reset_completed()?;
            let turn=original.move_existing(value,&mut slot.as_mut().ok_or_else(invalid)?.pending,birth,birth_unwind);work.take(turn)?;birth.reset_completed()?;
            work.take(original.drive_cursor(slot,output,&mut |rx,output,_,_,native,body|{
                let g=body.remaining_grant();
                if rx.ready(){
                    let turn=rx.take(g);
                    RetainedWorkOutcome{work:turn.work,result:turn.result.and_then(|out|out.map_or(Ok(()),|(value,p)|{*output=Some(value);body.record_progress(p)})).map_err(store::NativeSnapshotFailure::from)}
                }else{
                    let mut recipient=OriginalConstructionAuthority::new(OriginalStepRecipient{grant:g,progress:Default::default(),submitted:Default::default()});
                    let mut work=0;
                    let result=match recipient.with_loan(&mut (),|loan|native.with_construction_recipient(loan,|native|{let turn=rx.advance(native,g.maximum_copy_bytes,g);work=turn.work;turn.result})){Ok(result)=>result,Err(error)=>Err(error.into())};
                    RetainedWorkOutcome{work,result:result.and_then(|step|body.record_progress(step.progress()).map_err(store::NativeSnapshotFailure::from))}
                }
            }))
        }))
    }

}

impl Fem2dSnapshot {
    /// ❄️ Reference construction validates authoring data before exclusive cold issuance.
    pub fn from_reference(value: Fem2dSnapshotValue) -> Result<Self, ValueError> {
        use super::cold as cold;
        validate_cold_authoring(&value)?;
        let Fem2dSnapshotValue { nodes, materials, elements, regions, sections, supports, load_cases, combinations, analysis } = value;
        let nodes = cold::nodes2d(nodes)?;
        let materials = match cold::materials2d(materials) { Ok(materials)=>materials, Err(error)=>{if cold::discard_nodes2d(nodes).is_err(){unreachable!("unpublished cold node pages have no aliases")}return Err(error)} };
        Ok(Self { nodes, materials, elements, regions, sections, supports, load_cases, combinations, analysis })
    }
}

impl Fem2dSnapshot {
    /// 🔖️ Reference projection copies the persistent rows into the explicit authoring value for cold algebra and reports.
    pub fn to_reference(&self) -> Fem2dSnapshotValue {
        Fem2dSnapshotValue {
            nodes: self.nodes.iter().map(|n| FemNodeValue { id: n.id.to_string(), x: n.x, y: n.y }).collect(),
            elements: self.elements.clone(),
            regions: self.regions.clone(),
            materials: self.materials.iter().map(|m| FemMaterialValue { id: m.id.to_string(), name: m.name.to_string(), e: m.e, nu: m.nu, rho: m.rho }).collect(),
            sections: self.sections.clone(),
            supports: self.supports.clone(),
            load_cases: self.load_cases.clone(),
            combinations: self.combinations.clone(),
            analysis: self.analysis.clone(),
        }
    }
}

/// 🧲️ Persistent snapshots decode from wire values through the explicit authoring rows and retire a decoded-but-unused owner through the funded protocol.
impl semio_framework_value::FromValue for Fem2dSnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, ValueError> {
        Self::from_reference(<Fem2dSnapshotValue as semio_framework_value::FromValue>::from_value(value)?)
    }
    fn retire_decoded(self) {
        super::cold::retire(self);
    }
}

fn validate_cold_authoring(value: &Fem2dSnapshotValue) -> Result<(), ValueError> {
    use std::collections::HashSet;
    use crate::standards::v1::subsets::any::schema::structure::{FemElement,FemLoad,element_id};
    let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue, "FEM reference graph or row is invalid");
    value.validate_interaction_identities()?;
    for n in &value.nodes { if !(n.x.is_finite() && n.y.is_finite()) { return Err(invalid()); } }
    for m in &value.materials { if !(m.e.is_finite() && m.e > 0.0 && m.nu.is_finite() && m.nu > -1.0 && m.nu < 0.5 && m.rho.is_finite() && m.rho > 0.0 && true) { return Err(invalid()); } }
    let nodes: HashSet<&str> = value.nodes.iter().map(|v|v.id.as_str()).collect();
    let materials: HashSet<&str> = value.materials.iter().map(|v|v.id.as_str()).collect();
    let sections: HashSet<&str> = value.sections.iter().map(|v|v.id.as_str()).collect();
    let elements: HashSet<&str> = value.elements.iter().map(element_id).collect();
    let regions: HashSet<&str> = value.regions.iter().map(|v|v.id.as_str()).collect();
    for element in &value.elements {
        let (start,end,material,section) = match element { FemElement::Bar { start,end,material_id,section_id,.. } | FemElement::Beam { start,end,material_id,section_id,.. } => (start,end,material_id,section_id) };
        if !(nodes.contains(start.as_str()) && nodes.contains(end.as_str()) && materials.contains(material.as_str()) && sections.contains(section.as_str())) { return Err(invalid()); }
    }
    for support in &value.supports { if !nodes.contains(support.node_id.as_str()) { return Err(invalid()); } }
    for case in &value.load_cases { for load in &case.loads {
        let valid = match load {
            FemLoad::Nodal {node_id,value,..} => nodes.contains(node_id.as_str()) && value.is_finite(),
            FemLoad::MemberUdl {element_id,wx, wy,..} => elements.contains(element_id.as_str()) && wx.is_finite() && wy.is_finite(),
            FemLoad::Area {region_id,pressure,..} => regions.contains(region_id.as_str()) && pressure.is_finite(),
        };
        if !valid { return Err(invalid()); }
    }}
    Ok(())
}

fn validate_authoring(value:&Fem2dSnapshotValue,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<(),store::NativeSnapshotFailure>{
    let mut completed=None;
    owner.receive_nested::<super::FemIdentityIndex,(),store::NativeSnapshotFailure>(&mut completed,&mut |slot,completed,progress,unwind,original|store::native_snapshot_turn(|meter|{
        meter.take(super::FemIdentityIndex::receive(value,slot,original))?;let index=slot.as_ref().unwrap();
        for element in &value.elements{
            let(start,end,material,section)=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(match element{
                crate::standards::v1::subsets::any::schema::structure::FemElement::Bar{start,end,material_id,section_id,..}|crate::standards::v1::subsets::any::schema::structure::FemElement::Beam{start,end,material_id,section_id,..}=>(start.as_str(),end.as_str(),material_id.as_str(),section_id.as_str())
            }))?;
            {let mut rank=None;meter.take(index.lookup(value,super::FemIdentityKind::Node,start,&mut rank,original))?;}{let mut rank=None;meter.take(index.lookup(value,super::FemIdentityKind::Node,end,&mut rank,original))?;}
            {let mut rank=None;meter.take(index.lookup(value,super::FemIdentityKind::Material,material,&mut rank,original))?;}{let mut rank=None;meter.take(index.lookup(value,super::FemIdentityKind::Section,section,&mut rank,original))?;}
        }
        for support in &value.supports{let node=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(support.node_id.as_str()))?;{let mut rank=None;meter.take(index.lookup(value,super::FemIdentityKind::Node,node,&mut rank,original))?;}}
        for case in &value.load_cases{
            let loads=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(case.loads.as_slice()))?;
            for load in loads{
                use crate::standards::v1::subsets::any::schema::structure::FemLoad;
                let(reference,finite)=semio_s_fem_receiving::inspect(original,progress,unwind,meter,||Ok(match load{
                    FemLoad::Nodal{node_id,value:force,..}=>(Some((super::FemIdentityKind::Node,node_id.as_str())),force.is_finite()),
                    FemLoad::MemberUdl{element_id,wx,wy,..}=>(Some((super::FemIdentityKind::Element,element_id.as_str())),wx.is_finite()&&wy.is_finite()),
                    FemLoad::Area{region_id,pressure,..}=>(Some((super::FemIdentityKind::Region,region_id.as_str())),pressure.is_finite()),
                }))?;
                if let Some((kind,id))=reference{{let mut rank=None;meter.take(index.lookup(value,kind,id,&mut rank,original))?;}}
                semio_s_fem_receiving::inspect(original,progress,unwind,meter,||if finite{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"FEM load components must be finite binary64 values"))})?;
            }
        }
        *completed=Some(());Ok(())
    }))
}
/// 🧳️ Every partial list, source alias and untransferred snapshot stays in this owner until explicitly closed.
pub struct Fem2dReceiving {
    pending: ManuallyDrop<Option<Fem2dSnapshotValue>>,
    admission: ManuallyDrop<Option<StagedSourceAdmission<Fem2dSnapshotValue, Fem2dSnapshotValue>>>,
    source: ManuallyDrop<Option<RetainedCloneSource<Fem2dSnapshotValue>>>,
    nodes_receiver: ManuallyDrop<Option<NodeReceiver>>,
    materials_receiver: ManuallyDrop<Option<MaterialReceiver>>,
    nodes: ManuallyDrop<Option<Nodes>>,
    materials: ManuallyDrop<Option<Materials>>,
    output: ManuallyDrop<Option<Fem2dSnapshot>>,
    retirement: RetainedCloneClose,
    validated: usize,
    phase: u8,
    closing: bool,
}
impl Fem2dReceiving {
    /// 📏️ Quotes only the freshly initialized receiving header.
    pub fn birth_demand()->RetirementDemand{fixed(size_of::<Self>())}
    /// 🪑️ Initializes an empty receiver only when its exact header is admitted.
    pub fn empty(g:RetainedCloneGrant)->RetainedWorkOutcome<(Self,RetainedCloneProgress),ValueError>{
        let d=Self::birth_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"FEM receiving header exceeds its admitted grant")),work:0}}
        RetainedWorkOutcome{result:Ok((
            Self {
                pending: ManuallyDrop::new(None),
                admission: ManuallyDrop::new(None),
                source: ManuallyDrop::new(None),
                nodes_receiver: ManuallyDrop::new(None),
                materials_receiver: ManuallyDrop::new(None),
                nodes: ManuallyDrop::new(None),
                materials: ManuallyDrop::new(None),
                output: ManuallyDrop::new(None),
                retirement: Default::default(),
                validated: 0,
                phase: 0,
                closing: false,
            },
            receipt(d),
        )),work:1}
    }
    /// 🫴️ Moves an existing authoring owner after a separately admitted metadata handoff.
    pub fn admit_source(&mut self,value:&mut Option<Fem2dSnapshotValue>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,ValueError>{
        if self.pending.is_some()||value.is_none(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
        if g.maximum_items==0||g.maximum_depth==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
        *self.pending=value.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
    }
    /// 🧮️ Each demand describes the next original source, row, page, alias or fixed header transition.
    pub fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing {
            return Err(invalid());
        }
        match self.phase {
            0 => match self.admission.as_ref() {
                Some(admission) => if admission.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<StagedSourceAdmission<Fem2dSnapshotValue, Fem2dSnapshotValue>>>())) } else { admission.next_demand() },
                None => Ok(fixed(2 * size_of::<StagedSourceAdmission<Fem2dSnapshotValue, Fem2dSnapshotValue>>())),
            },
            1 => Ok(fixed(size_of::<NodeReceiver>())),
            2 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0, |s| &s.nodes), body, ACTION_WORK),
            3 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            4 => if self.nodes_receiver.as_ref().ok_or_else(invalid)?.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<NodeReceiver>>())) } else { self.nodes_receiver.as_ref().ok_or_else(invalid)?.close_demands(body) },
            5 => Ok(fixed(size_of::<MaterialReceiver>())),
            6 => self.materials_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(3, |s| &s.materials), body, ACTION_WORK),
            7 => self.materials_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            8 => if self.materials_receiver.as_ref().ok_or_else(invalid)?.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<MaterialReceiver>>())) } else { self.materials_receiver.as_ref().ok_or_else(invalid)?.close_demands(body) },
            9 => self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body),
            15 => Ok(fixed(2 * size_of::<Option<RetainedCloneSource<Fem2dSnapshotValue>>>())),
            10 | 11 => Ok(fixed(size_of::<usize>() + size_of::<FemNode>().max(size_of::<FemMaterial>()))),
            12 => Ok(fixed(2 * size_of::<Fem2dSnapshotValue>() + 2 * size_of::<Fem2dSnapshot>())),
            _ => Err(invalid()),
        }
    }
    /// ⏱️ Advances one granted physical frontier; every framework action runs inside one exact allocated effect of the original decoder.
    pub fn advance(&mut self, control: &mut NativeDecodeControl<'_>, body: usize, g: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, store::NativeSnapshotFailure> {
        store::native_snapshot_turn(|work|{
        let d = self.demands(body)?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        control.checkpoint()?;
        macro_rules! effect { ($e:ident => $run:expr) => { match work.measured(control, |c| c.with_allocated_effect(d, ACTION_WORK, |_, $e| $run))? { Some(turn) => turn, None => return Ok(RetainedCloneStep::Progress(Default::default())) } } }
        macro_rules! child { ($rx:ident, $out:ident, $base:expr, $at:expr, $project:expr, $ty:ty) => {{
            match self.phase - $base {
                0 => {
                    work.performed();
                    *self.$rx = Some(<$ty>::default());
                    self.phase += 1;
                    receipt(d)
                }
                1 => {
                    let source = self.source.as_ref().ok_or_else(invalid)?.borrow().project($at, $project);
                    let rx = self.$rx.as_mut().ok_or_else(invalid)?;
                    let maximum = native_work_limit(control, ACTION_WORK)?;
                    let witness = control.source_witness()?;
                    let turn = settle(rx.advance(source, control, witness, body, maximum, g));
                    let p = work.take(RetainedWorkOutcome { result: turn.result.map_err(receiving_failure), work: turn.work })?.progress();
                    if rx.ready() {
                        self.phase += 1;
                    }
                    p
                }
                2 => {
                    let rx = self.$rx.as_mut().ok_or_else(invalid)?;
                    let mut taken = None;
                    let step = effect!(effect => settle(rx.take(&mut taken, effect)).result)?;
                    *self.$out = Some(taken.ok_or_else(invalid)?);
                    self.phase += 1;
                    step.progress()
                }
                _ => {
                    let rx = self.$rx.as_mut().ok_or_else(invalid)?;
                    if rx.terminal_is_empty() {
                        work.performed();
                        self.$rx.take();
                        self.phase += 1;
                        receipt(d)
                    } else {
                        effect!(effect => settle(settled(effect, |effect, reply| rx.close_into(body, effect, reply))).result)?.progress()
                    }
                }
            }
        }} }
        let p = match self.phase {
            0 => {
                if self.admission.is_none() {
                    let value = self.pending.take().ok_or_else(invalid)?;
                    work.performed();
                    *self.admission = Some(StagedSourceAdmission::owned(value));
                    receipt(d)
                } else {
                    let admission = self.admission.as_mut().ok_or_else(invalid)?;
                    if admission.terminal_is_empty() {
                        work.performed();
                        self.admission.take();
                        if self.source.is_some() { self.phase = 1; }
                        receipt(d)
                    } else {
                        let source = &mut self.source;
                        effect!(effect => admission.advance(source, effect)).result?.progress()
                    }
                }
            }
            1..=4 => child!(nodes_receiver, nodes, 1, 0, |s| &s.nodes, NodeReceiver),
            5..=8 => child!(materials_receiver, materials, 5, 3, |s| &s.materials, MaterialReceiver),
            9 => {
                let source = self.source.as_mut().ok_or_else(invalid)?;
                let mut taken = None;
                let p = effect!(effect => settle(settled(effect, |effect, reply| source.take_authority_into(&mut taken, body, effect, reply))).result)?.progress();
                if let Some(value) = taken {
                    *self.pending = Some(value);
                    self.phase = 15;
                }
                p
            }
            15 => {
                work.performed();
                self.source.take().ok_or_else(invalid)?;
                self.phase = 10;
                receipt(d)
            }
            10 => {
                work.performed();                let row = self.nodes.as_ref().ok_or_else(invalid)?.get(self.validated);
                match row {
                    Some(row) => {
                        row.validate().map_err(|e| e.with_retained_progress(receipt(d)))?;
                        self.validated += 1;
                    }
                    None => {
                        self.validated = 0;
                        self.phase = 11;
                    }
                }
                receipt(d)
            }
            11 => {
                work.performed();                let row = self.materials.as_ref().ok_or_else(invalid)?.get(self.validated);
                match row {
                    Some(row) => {
                        row.validate().map_err(|e| e.with_retained_progress(receipt(d)))?;
                        self.validated += 1;
                    }
                    None => {
                        self.validated = 0;
                        self.phase = 12;
                    }
                }
                receipt(d)
            }
            12 => {
                work.performed();                let value = self.pending.as_mut().ok_or_else(invalid)?;
                *self.output = Some(Fem2dSnapshot {
                    nodes: self.nodes.take().ok_or_else(invalid)?,
                    materials: self.materials.take().ok_or_else(invalid)?,
                    elements: std::mem::take(&mut value.elements),
                    sections: std::mem::take(&mut value.sections),
                    regions: std::mem::take(&mut value.regions),
                    supports: std::mem::take(&mut value.supports),
                    load_cases: std::mem::take(&mut value.load_cases),
                    combinations: std::mem::take(&mut value.combinations),
                    analysis: std::mem::take(&mut value.analysis),
                });
                self.phase = 13;
                receipt(d)
            }
            _ => return Err(invalid().into()),
        };
        Ok(if self.ready() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
        })
    }
    /// 📫️ A real snapshot becomes transferable only after both row receivers and their source aliases have closed.
    pub fn ready(&self) -> bool {
        !self.closing && self.phase == 13 && self.output.is_some()
    }
    /// 📸️ Transfers only the fixed actual snapshot header; persistent pages and authored collections retain their identities.
    pub fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.ready() {
            return Err(invalid());
        }
        Ok(fixed(2 * size_of::<Option<Fem2dSnapshot>>()))
    }
    /// 📤️ Taking output leaves the original plain node/material inputs resident for funded retirement.
    pub fn take(&mut self, g: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Fem2dSnapshot, RetainedCloneProgress)>> {
        let mut work=0;let result=(||{
        let d = self.take_demand()?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(None);
        }
        work=1;self.phase = 14;
        Ok(Some((self.output.take().ok_or_else(invalid)?, receipt(d))))
        })();RetainedWorkOutcome{work,result}
    }
    /// 🛑️ Cancellation switches to closure without dropping source, row, page or output ownership.
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    /// ♻️ Closure demand remains usable after the work callback has cancelled.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_quote(body)?.0)
    }
    /// ⛽️ Quotes the actual work of the next closure action.
    pub fn close_work(&self, body: usize) -> Result<u64, ValueError> {
        Ok(self.close_quote(body)?.1)
    }
    fn close_quote(&self, body: usize) -> Result<(RetirementDemand, u64), ValueError> {
        if !self.closing {
            return Err(invalid());
        }
        macro_rules! slot { ($rx:expr, $size:expr) => { return Ok(if $rx.terminal_is_empty() { (fixed(2 * $size), 1) } else { ($rx.close_demands(body)?, $rx.close_work_demand(body)?) }) } }
        macro_rules! owned { ($ty:ty, $present:expr) => { return Ok((self.retirement.next_owner_demands::<$ty>($present, &None, body)?, self.retirement.next_owner_work::<$ty>($present, &None, body)?)) } }
        if let Some(rx) = self.nodes_receiver.as_ref() { slot!(rx, size_of::<Option<NodeReceiver>>()) }
        if let Some(rx) = self.materials_receiver.as_ref() { slot!(rx, size_of::<Option<MaterialReceiver>>()) }
        if let Some(source) = self.source.as_ref() {
            return Ok(if source.terminal_is_empty() { (fixed(2 * size_of::<Option<RetainedCloneSource<Fem2dSnapshotValue>>>()), 1) } else { (source.next_close_demand(body)?, source.next_close_work_demand(body)?) });
        }
        if !self.retirement.is_empty() { owned!(Fem2dSnapshotValue, false) }
        if self.admission.is_some() { owned!(StagedSourceAdmission<Fem2dSnapshotValue, Fem2dSnapshotValue>, true) }
        if self.pending.is_some() { owned!(Fem2dSnapshotValue, true) }
        if self.nodes.is_some() { owned!(Nodes, true) }
        if self.materials.is_some() { owned!(Materials, true) }
        if self.output.is_some() { owned!(Fem2dSnapshot, true) }
        Ok((Default::default(), 0))
    }
    /// 🧹️ Every source and destination frontier closes only through the supplied effect.
    fn close_poll(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        use RetainedWorkPoll::Complete;
        let g = effect.grant();
        let fail = |error: ValueError| -> OriginalRetirementPoll { Complete(RetainedWorkOutcome { result: Err(error.into()), work: 0 }) };
        let d = match self.close_demands(body) { Ok(d) => d, Err(error) => return fail(error) };
        if !retained_clone_grant_admits(g, d) {
            return Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 });
        }
        macro_rules! release { ($slot:expr) => {{
            match effect.performed(1) {
                Ok(true) => {}
                Ok(false) => return fail(ValueError::literal(ValueRefusalKind::WorkLimit, "FEM receiving shell release exceeded its work authority")),
                Err(error) => return fail(error),
            }
            $slot.take();
            return Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(receipt(d))), work: 1 });
        }} }
        if let Some(rx) = self.nodes_receiver.as_mut() {
            if rx.terminal_is_empty() { release!(self.nodes_receiver) }
            return settled(effect, |effect, reply| rx.close_into(body, effect, reply)).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(rx) = self.materials_receiver.as_mut() {
            if rx.terminal_is_empty() { release!(self.materials_receiver) }
            return settled(effect, |effect, reply| rx.close_into(body, effect, reply)).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(source) = self.source.as_mut() {
            if source.terminal_is_empty() { release!(self.source) }
            return source.close_step(body, effect).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.retirement.is_empty() {
            let poll = self.retirement.step_granted(body, effect);
            return poll.map(|step| self.finish(step.progress()));
        }
        let turn = if self.admission.is_some() {
            self.retirement.begin_granted(&mut self.admission, effect)
        } else if self.pending.is_some() {
            self.retirement.begin_granted(&mut self.pending, effect)
        } else if self.nodes.is_some() {
            self.retirement.begin_granted(&mut self.nodes, effect)
        } else if self.materials.is_some() {
            self.retirement.begin_granted(&mut self.materials, effect)
        } else if self.output.is_some() {
            self.retirement.begin_granted(&mut self.output, effect)
        } else {
            return Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 });
        };
        Complete(RetainedWorkOutcome { result: turn.result.map(|step| self.finish(step.map_or_else(Default::default, |step| step.progress()))).map_err(Into::into), work: turn.work })
    }
    /// 🧹️ Every source and destination frontier closes only through the supplied effect.
    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        match self.close_poll(body, effect) {
            RetainedWorkPoll::Pending { work } => RetirementStep::Pending(work),
            RetainedWorkPoll::Complete(turn) => match turn.result {
                Err(error) => RetirementStep::Failure(error, turn.work),
                Ok(RetainedCloneStep::Complete(progress)) if progress == RetainedCloneProgress::default() => RetirementStep::Complete(turn.work),
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => RetirementStep::Progress(progress, turn.work),
            },
        }
    }
    fn finish(&self, p: RetainedCloneProgress) -> RetainedCloneStep {
        if self.terminal_is_empty() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) }
    }
}
impl RetireOwned for Fem2dReceiving {
    type RetirementCursor = Self;
    fn retirement_seed(mut self) -> Self {
        self.begin_close();
        self
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl RetirementCursor for Fem2dReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn next_computational_work(&self, body: usize) -> Result<u64, ValueError> {
        self.close_work(body)
    }
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self) -> bool {
        Self::terminal_is_empty(self)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(usize::MAX)?.depth)
    }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(usize::MAX)?.copy_bytes)
    }
    fn next_selected_work_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(body)?.copy_bytes)
    }
    fn next_close_byte_demand(&self) -> Option<usize> {
        self.close_demands(usize::MAX).ok().map(|demand| demand.release_bytes)
    }
    fn next_birth_bytes(&self, body: usize) -> Option<usize> {
        self.close_demands(body).ok().map(|demand| demand.capacity_bytes)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
impl Drop for Fem2dReceiving {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "FEM receiving requires original funded closure");
    }
}
