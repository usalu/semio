//! 🛬️ Original authoring owners receive persistent FEM rows through physical grants and retain every close frontier.
use super::{Fem2dSnapshot, Fem2dSnapshotValue};
use semio_framework_value::{
    ErasedSnapshotRetirement, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep, StagedSourceAdmission, retained_clone_grant_admits},
};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeReceive, NativeReceiver, owned::NativePersistentListReceiver};
use semio_framework_value_persistent::list::persistent::PersistentList;
use semio_s_fem_material::planar::{FemMaterial, FemMaterialValue};
use semio_s_fem_node::planar::{FemNode, FemNodeValue};
use std::mem::{ManuallyDrop, size_of};
type Nodes = PersistentList<FemNode>;
type Materials = PersistentList<FemMaterial>;
type NodeReceiver = NativePersistentListReceiver<Vec<FemNodeValue>, FemNode>;
type MaterialReceiver = NativePersistentListReceiver<Vec<FemMaterialValue>, FemMaterial>;
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
impl NativeDecodeConstructionRecipient for OriginalStepRecipient {
    fn submitted(&self)->RetainedCloneProgress{self.submitted}
    fn grant(&mut self, d: RetirementDemand) -> Result<RetainedCloneGrant, ValueError> {
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
    fn accept(&mut self, g: RetainedCloneGrant, p: RetainedCloneProgress) -> Result<(), ValueError> {
        self.submitted=self.submitted.checked_add(p)?;
        if !p.fits(g) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "FEM sealed row receipt exceeds its original receiving grant").with_retained_progress(p));
        }
        self.progress = self.progress.checked_add(p)?;
        Ok(())
    }
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
            work.take(original.drive_cursor(slot,output,&mut |rx,output,native,g|{
                if rx.ready(){
                    let turn=rx.take(g);
                    RetainedWorkOutcome{work:turn.work,result:turn.result.map(|out|out.map_or(Default::default(),|(value,p)|{*output=Some(value);p})).map_err(store::NativeSnapshotFailure::from)}
                }else{
                    let mut recipient=OriginalStepRecipient{grant:g,progress:Default::default(),submitted:Default::default()};
                    let mut work=0;
                    let result=native.with_construction_recipient(&mut recipient,|native|{let turn=rx.advance(native,g.maximum_copy_bytes,g);work=turn.work;turn.result});
                    RetainedWorkOutcome{work,result:result.map(|step|step.progress())}
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
        let wallet = RetainedCloneGrant { maximum_items: 1 << 40, maximum_copy_bytes: 1 << 40, maximum_capacity_bytes: 1 << 40, maximum_release_bytes: 1 << 40, maximum_depth: 128 };
        if let Ok(mut owner) = semio_framework_value::retirement::controlled::ControlledRetirement::new(self) {
            while !owner.terminal_is_empty() {
                if owner.step(wallet).is_err() {
                    break;
                }
            }
        }
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
            2 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0, |s| &s.nodes), body),
            3 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            4 => if self.nodes_receiver.as_ref().ok_or_else(invalid)?.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<NodeReceiver>>())) } else { self.nodes_receiver.as_ref().ok_or_else(invalid)?.close_demands(body) },
            5 => Ok(fixed(size_of::<MaterialReceiver>())),
            6 => self.materials_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(3, |s| &s.materials), body),
            7 => self.materials_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            8 => if self.materials_receiver.as_ref().ok_or_else(invalid)?.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<MaterialReceiver>>())) } else { self.materials_receiver.as_ref().ok_or_else(invalid)?.close_demands(body) },
            9 => self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body),
            15 => Ok(fixed(2 * size_of::<Option<RetainedCloneSource<Fem2dSnapshotValue>>>())),
            10 | 11 => Ok(fixed(size_of::<usize>() + size_of::<FemNode>().max(size_of::<FemMaterial>()))),
            12 => Ok(fixed(2 * size_of::<Fem2dSnapshotValue>() + 2 * size_of::<Fem2dSnapshot>())),
            _ => Err(invalid()),
        }
    }
    /// ⏱️ Advances one granted physical frontier using the unchanged original decoder and construction recipient.
    pub fn advance(&mut self, control: &mut NativeDecodeControl<'_>, body: usize, g: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneStep, store::NativeSnapshotFailure> {
        store::native_snapshot_turn(|work|{
        let d = self.demands(body)?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        control.checkpoint()?;
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
                        control.charge(d.capacity_bytes)?;
                        work.take(admission.advance(&mut self.source, g))?.progress()
                    }
                }
            }
            1 => {
                let rx = <Nodes as NativeReceive<Vec<FemNodeValue>>>::receiver();
                work.performed();
                let p = receipt(d);
                *self.nodes_receiver = Some(rx);
                self.phase = 2;
                p
            }
            2 => {
                let source = self.source.as_ref().ok_or_else(invalid)?.borrow().project(0, |s| &s.nodes);
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                let p = work.take(rx.advance(source, control, body, g))?.progress();
                if rx.output_ready() {
                    self.phase = 3;
                }
                p
            }
            3 => {
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                let (value, p) = work.take(rx.take(g))?.ok_or_else(invalid)?;
                *self.nodes = Some(value);
                rx.begin_close();
                self.phase = 4;
                p
            }
            4 => {
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                if rx.terminal_is_empty() {
                    work.performed();
                    self.nodes_receiver.take();
                    self.phase = 5;
                    receipt(d)
                } else {
                    control.charge(d.capacity_bytes)?;
                    work.take(rx.close_step(g))?.progress()
                }
            }
            5 => {
                let rx = <Materials as NativeReceive<Vec<FemMaterialValue>>>::receiver();
                work.performed();
                let p = receipt(d);
                *self.materials_receiver = Some(rx);
                self.phase = 6;
                p
            }
            6 => {
                let source = self.source.as_ref().ok_or_else(invalid)?.borrow().project(3, |s| &s.materials);
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                let p = work.take(rx.advance(source, control, body, g))?.progress();
                if rx.output_ready() {
                    self.phase = 7;
                }
                p
            }
            7 => {
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                let (value, p) = work.take(rx.take(g))?.ok_or_else(invalid)?;
                *self.materials = Some(value);
                rx.begin_close();
                self.phase = 8;
                p
            }
            8 => {
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                if rx.terminal_is_empty() {
                    work.performed();
                    self.materials_receiver.take();
                    self.phase = 9;
                    receipt(d)
                } else {
                    control.charge(d.capacity_bytes)?;
                    work.take(rx.close_step(g))?.progress()
                }
            }
            9 => match {
                control.charge(d.capacity_bytes)?;
                {
                    let turn = self.source.as_mut().ok_or_else(invalid)?.take_authority(g);
                    work.take(turn)?
                }
            } {
                RetainedCloneSourceTake::Pending(p) => p,
                RetainedCloneSourceTake::Ready(value, p) => {
                    *self.pending = Some(value);
                    self.phase = 15;
                    p
                }
            },
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
        if let Some(rx) = self.nodes_receiver.as_mut() {
            rx.begin_close();
        }
        if let Some(rx) = self.materials_receiver.as_mut() {
            rx.begin_close();
        }
    }
    /// ♻️ Closure demand remains usable after the work callback has cancelled.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing {
            return Err(invalid());
        }
        if let Some(rx) = self.nodes_receiver.as_ref() {
            return if rx.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<NodeReceiver>>())) } else { rx.close_demands(body) };
        }
        if let Some(rx) = self.materials_receiver.as_ref() {
            return if rx.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<MaterialReceiver>>())) } else { rx.close_demands(body) };
        }
        if let Some(source) = self.source.as_ref() {
            return if source.terminal_is_empty() { Ok(fixed(2 * size_of::<Option<RetainedCloneSource<Fem2dSnapshotValue>>>())) } else { source.next_close_demand(body) };
        }
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<Fem2dSnapshotValue>(false, &None, body);
        }
        if self.admission.is_some() {
            return self.retirement.next_owner_demands::<StagedSourceAdmission<Fem2dSnapshotValue, Fem2dSnapshotValue>>(true, &None, body);
        }
        if self.pending.is_some() {
            return self.retirement.next_owner_demands::<Fem2dSnapshotValue>(true, &None, body);
        }
        if self.nodes.is_some() {
            return self.retirement.next_owner_demands::<Nodes>(true, &None, body);
        }
        if self.materials.is_some() {
            return self.retirement.next_owner_demands::<Materials>(true, &None, body);
        }
        if self.output.is_some() {
            return self.retirement.next_owner_demands::<Fem2dSnapshot>(true, &None, body);
        }
        Ok(Default::default())
    }
    /// 🧹️ Every source and destination frontier closes only through the supplied physical grant.
    pub fn close_step(&mut self, g: RetainedCloneGrant) -> RetainedWorkOutcome {
        let mut work = 0;
        let result = (|| {
            let d = self.close_demands(g.maximum_copy_bytes)?;
            if !retained_clone_grant_admits(g, d) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if let Some(rx) = self.nodes_receiver.as_mut() {
                if rx.terminal_is_empty() {
                    work = 1;
                    self.nodes_receiver.take();
                    return Ok(RetainedCloneStep::Progress(receipt(d)));
                }
                let turn = rx.close_step(g);
                work = turn.work;
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(rx) = self.materials_receiver.as_mut() {
                if rx.terminal_is_empty() {
                    work = 1;
                    self.materials_receiver.take();
                    return Ok(RetainedCloneStep::Progress(receipt(d)));
                }
                let turn = rx.close_step(g);
                work = turn.work;
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(source) = self.source.as_mut() {
                if source.terminal_is_empty() {
                    work = 1;
                    self.source.take();
                    return Ok(RetainedCloneStep::Progress(receipt(d)));
                }
                let turn = source.close_step(g);
                work = turn.work;
                return turn.result.map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            let p = if !self.retirement.is_empty() {
                let turn = self.retirement.step_granted(g);
                work = turn.work;
                turn.result?.progress()
            } else if let Some(s) = Self::begin(&mut self.retirement, &mut self.admission, g, &mut work)? {
                s.progress()
            } else if let Some(s) = Self::begin(&mut self.retirement, &mut self.pending, g, &mut work)? {
                s.progress()
            } else if let Some(s) = Self::begin(&mut self.retirement, &mut self.nodes, g, &mut work)? {
                s.progress()
            } else if let Some(s) = Self::begin(&mut self.retirement, &mut self.materials, g, &mut work)? {
                s.progress()
            } else if let Some(s) = Self::begin(&mut self.retirement, &mut self.output, g, &mut work)? {
                s.progress()
            } else {
                Default::default()
            };
            Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
        })();
        RetainedWorkOutcome { result, work }
    }
    fn begin<T: semio_framework_value::retirement::RetireOwned>(retirement: &mut RetainedCloneClose, value: &mut Option<T>, g: RetainedCloneGrant, work: &mut u64) -> Result<Option<RetainedCloneStep>, ValueError> {
        let turn = retirement.begin_granted(value, g);
        *work = turn.work;
        turn.result
    }
    /// 🏁️ Terminal destruction must release no original or newly received owner.
    pub fn terminal_is_empty(&self) -> bool {
        self.pending.is_none() && self.admission.is_none() && self.source.is_none() && self.nodes_receiver.is_none() && self.materials_receiver.is_none() && self.nodes.is_none() && self.materials.is_none() && self.output.is_none() && self.retirement.is_empty()
    }
}
impl ErasedSnapshotRetirement for Fem2dReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn close_step(&mut self, g: RetainedCloneGrant) -> RetainedWorkOutcome {
        Self::close_step(self, g)
    }
    fn terminal_is_empty(&self) -> bool {
        Self::terminal_is_empty(self)
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0)?.depth)
    }
}
impl semio_framework_value::retirement::RetireOwned for Fem2dReceiving {
    fn retirement(mut self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        self.begin_close();
        Box::new(self)
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
impl semio_framework_value::retirement::RetirementCursor for Fem2dReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn close_step(&mut self, g: RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        let turn = Self::close_step(self, g);
        match turn.result {
            Ok(s) => semio_framework_value::retirement::RetirementStep::Progress(s.progress(), turn.work),
            Err(e) => semio_framework_value::retirement::RetirementStep::Failure(e, turn.work),
        }
    }
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
