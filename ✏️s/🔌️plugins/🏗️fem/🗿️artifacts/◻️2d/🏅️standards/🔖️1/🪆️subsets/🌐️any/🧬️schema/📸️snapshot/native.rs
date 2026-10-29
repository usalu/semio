//! 🛬️ Original authoring owners receive persistent FEM rows through physical grants and retain every close frontier.
use super::{Fem2dSnapshot, Fem2dSnapshotValue};
use semio_framework_value::{
    ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneSourceTake, RetainedCloneStep, retained_clone_grant_admits},
};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeConstructionRecipient, NativeDecodeControl, NativeDecodeRetirementRecipient, NativeReceive, NativeReceiver, owned::NativePersistentListReceiver};
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
}
impl NativeDecodeConstructionRecipient for OriginalStepRecipient {
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
        if !p.fits(g) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "FEM sealed row receipt exceeds its original receiving grant").with_retained_progress(p));
        }
        self.progress = self.progress.checked_add(p)?;
        Ok(())
    }
}

impl Fem2dSnapshot {
    /// 🫴️ Receives an already admitted authoring owner inside the original snapshot wallet and retirement publication frame.
    pub fn receive_authoring(value: &mut Option<Fem2dSnapshotValue>, owner: &mut store::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        owner.receive_nested::<Fem2dReceiving, Self>(|slot, original| {
            validate_authoring(value.as_ref().ok_or_else(invalid)?, original)?;
            let g = original.remaining_grant();
            let d = Fem2dReceiving::capture_demand();
            if !retained_clone_grant_admits(g, d) {
                return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "original FEM snapshot wallet cannot capture its authoring owner"));
            }
            original.native().checkpoint()?;
            let input = value.take().ok_or_else(invalid)?;
            let (rx, p) = match Fem2dReceiving::capture(input, g) {
                Ok(value) => value,
                Err((error, input)) => {
                    *value = Some(input);
                    return Err(error);
                }
            };
            *slot = Some(rx);
            original.record_progress(p)?;
            original.drive_cursor(slot, |rx, native, g| {
                let result = if rx.ready() {
                    rx.take(g).map(|out| out.map(|(value, p)| (Some(value), p)))
                } else {
                    let mut recipient = OriginalStepRecipient { grant: g, progress: Default::default() };
                    native.with_construction_recipient(&mut recipient, |native| rx.advance(native, g.maximum_copy_bytes, g)).map(|step| Some((None, step.progress())))
                };
                match result {
                    Ok(Some((value, p))) => (Ok(value), p),
                    Ok(None) => (Ok(None), Default::default()),
                    Err(error) => {
                        let p = error.retained_progress();
                        (Err(error), p)
                    }
                }
            })
        })
    }
}

impl Fem2dSnapshot {
    /// ❄️ Reference construction receives one explicit authoring value under an effectively unbounded wallet and retires its own receiving frames before returning.
    pub fn from_reference(value: Fem2dSnapshotValue) -> Result<Self, ValueError> {
        let wallet = RetainedCloneGrant { maximum_items: 1 << 40, maximum_copy_bytes: 1 << 40, maximum_capacity_bytes: 1 << 40, maximum_release_bytes: 1 << 40, maximum_depth: 128 };
        let mut recipient = NativeDecodeRetirementRecipient::new();
        let mut callback = |_| true;
        let mut native = NativeDecodeControl::new(1 << 40, &mut callback);
        native.install_retirement_recipient(&mut recipient)?;
        let mut owner = store::NativeSnapshotDecodeOwner::new(&mut native, wallet);
        let mut input = Some(value);
        let received = Self::receive_authoring(&mut input, &mut owner);
        drop(owner);
        native.with_retirement(wallet, |native| {
            while native.has_retirement_owner() {
                native.close_retirement_recipient(wallet)?;
            }
            Ok(())
        })?;
        received
    }
}

impl Fem2dSnapshot {
    /// ❄️ Reference projection copies the persistent rows into the explicit authoring value for cold algebra and reports.
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

/// ❄️ Persistent snapshots decode from wire values through the explicit authoring rows and retire a decoded-but-unused owner through the funded protocol.
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

fn validate_authoring(value: &Fem2dSnapshotValue, owner: &mut store::NativeSnapshotDecodeOwner<'_, '_>) -> Result<(), ValueError> {
    owner.receive_nested::<super::FemIdentityIndex, ()>(|slot, original| {
        *slot = Some(super::FemIdentityIndex::receive(value, original)?);
        let index = slot.as_ref().unwrap();
        for element in &value.elements {
            semio_framework_dsl_record::receiving::work(original, size_of::<crate::standards::v1::subsets::any::schema::structure::FemElement>(), 1)?;
            let (start, end, material, section) = match element {
                crate::standards::v1::subsets::any::schema::structure::FemElement::Bar { start, end, material_id, section_id, .. }
                | crate::standards::v1::subsets::any::schema::structure::FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
            };
            index.lookup(value, super::FemIdentityKind::Node, start, original)?;
            index.lookup(value, super::FemIdentityKind::Node, end, original)?;
            index.lookup(value, super::FemIdentityKind::Material, material, original)?;
            index.lookup(value, super::FemIdentityKind::Section, section, original)?;
        }
        for support in &value.supports {
            semio_framework_dsl_record::receiving::work(original, size_of::<crate::standards::v1::subsets::any::schema::structure::FemSupport>(), 1)?;
            index.lookup(value, super::FemIdentityKind::Node, &support.node_id, original)?;
        }
        for case in &value.load_cases {
            semio_framework_dsl_record::receiving::work(original, size_of::<crate::standards::v1::subsets::any::schema::structure::FemLoadCase>(), 1)?;
            for load in &case.loads {
                use crate::standards::v1::subsets::any::schema::structure::FemLoad;
                semio_framework_dsl_record::receiving::work(original, size_of::<FemLoad>(), 1)?;
                let finite = match load {
                    FemLoad::Nodal { node_id, value: force, .. } => {
                        index.lookup(value, super::FemIdentityKind::Node, node_id, original)?;
                        force.is_finite()
                    }
                    FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                        index.lookup(value, super::FemIdentityKind::Element, element_id, original)?;
                        wx.is_finite() && wy.is_finite()
                    }
                    FemLoad::Area { pressure, .. } => pressure.is_finite(),
                };
                if !finite {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "FEM load components must be finite binary64 values"));
                }
            }
        }
        Ok(())
    })
}
/// 🧳️ Every partial list, source alias and untransferred snapshot stays in this owner until explicitly closed.
pub struct Fem2dReceiving {
    pending: ManuallyDrop<Option<Fem2dSnapshotValue>>,
    source: ManuallyDrop<Option<RetainedCloneSource<Fem2dSnapshotValue>>>,
    nodes_receiver: ManuallyDrop<Option<NodeReceiver>>,
    materials_receiver: ManuallyDrop<Option<MaterialReceiver>>,
    nodes: ManuallyDrop<Option<Nodes>>,
    materials: ManuallyDrop<Option<Materials>>,
    output: ManuallyDrop<Option<Fem2dSnapshot>>,
    retirement: RetainedCloneClose,
    phase: u8,
    closing: bool,
}
impl Fem2dReceiving {
    /// 📏️ Capturing the original authoring header does not copy any row, string or collection body.
    pub fn capture_demand() -> RetirementDemand {
        fixed(size_of::<Self>() + size_of::<Fem2dSnapshotValue>())
    }
    /// 📥️ A denied physical grant returns the unchanged original value and creates no receiver.
    pub fn capture(value: Fem2dSnapshotValue, g: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, Fem2dSnapshotValue)> {
        let d = Self::capture_demand();
        if !retained_clone_grant_admits(g, d) {
            return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "original FEM capture grant does not admit its fixed header"), value));
        }
        Ok((
            Self {
                pending: ManuallyDrop::new(Some(value)),
                source: ManuallyDrop::new(None),
                nodes_receiver: ManuallyDrop::new(None),
                materials_receiver: ManuallyDrop::new(None),
                nodes: ManuallyDrop::new(None),
                materials: ManuallyDrop::new(None),
                output: ManuallyDrop::new(None),
                retirement: Default::default(),
                phase: 0,
                closing: false,
            },
            receipt(d),
        ))
    }
    /// 🧮️ Each demand describes the next original source, row, page, alias or fixed header transition.
    pub fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.closing {
            return Err(invalid());
        }
        match self.phase {
            0 => {
                let d = RetainedCloneSource::<Fem2dSnapshotValue>::owned_constructor_demand::<()>();
                Ok(RetirementDemand { copy_bytes: RetainedCloneSource::<Fem2dSnapshotValue>::constructor_copy_bytes(), capacity_bytes: d.capacity_bytes, depth: d.depth, ..Default::default() })
            }
            1 => Ok(<Nodes as NativeReceive<Vec<FemNodeValue>>>::receive_birth_demand()),
            2 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(0, |s| &s.nodes), body),
            3 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            4 => self.nodes_receiver.as_ref().ok_or_else(invalid)?.close_demands(body),
            5 => Ok(<Materials as NativeReceive<Vec<FemMaterialValue>>>::receive_birth_demand()),
            6 => self.materials_receiver.as_ref().ok_or_else(invalid)?.demands(self.source.as_ref().ok_or_else(invalid)?.borrow().project(3, |s| &s.materials), body),
            7 => self.materials_receiver.as_ref().ok_or_else(invalid)?.take_demand(),
            8 => self.materials_receiver.as_ref().ok_or_else(invalid)?.close_demands(body),
            9 => self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body),
            10 => Ok(fixed(2 * size_of::<Fem2dSnapshotValue>() + 2 * size_of::<Fem2dSnapshot>())),
            _ => Err(invalid()),
        }
    }
    /// ⏱️ Advances one granted physical frontier using the unchanged original decoder and construction recipient.
    pub fn advance(&mut self, control: &mut NativeDecodeControl<'_>, body: usize, g: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let d = self.demands(body)?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        control.checkpoint()?;
        let p = match self.phase {
            0 => {
                control.charge(d.capacity_bytes)?;
                let value = self.pending.take().ok_or_else(invalid)?;
                match RetainedCloneSource::admit_owned(value, (), g) {
                    Ok((source, p)) => {
                        *self.source = Some(source);
                        self.phase = 1;
                        p
                    }
                    Err((e, value, ())) => {
                        *self.pending = Some(value);
                        return Err(e);
                    }
                }
            }
            1 => {
                let (rx, p) = <Nodes as NativeReceive<Vec<FemNodeValue>>>::begin_receive(g)?;
                *self.nodes_receiver = Some(rx);
                self.phase = 2;
                p
            }
            2 => {
                let source = self.source.as_ref().ok_or_else(invalid)?.borrow().project(0, |s| &s.nodes);
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                let p = rx.advance(source, control, body, g)?.progress();
                if rx.output_ready() {
                    self.phase = 3;
                }
                p
            }
            3 => {
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                let (value, p) = rx.take(g)?.ok_or_else(invalid)?;
                *self.nodes = Some(value);
                rx.begin_close();
                self.phase = 4;
                p
            }
            4 => {
                let rx = self.nodes_receiver.as_mut().ok_or_else(invalid)?;
                let p = rx.close_step(g)?.progress();
                if rx.terminal_is_empty() {
                    self.nodes_receiver.take();
                    self.phase = 5;
                }
                p
            }
            5 => {
                let (rx, p) = <Materials as NativeReceive<Vec<FemMaterialValue>>>::begin_receive(g)?;
                *self.materials_receiver = Some(rx);
                self.phase = 6;
                p
            }
            6 => {
                let source = self.source.as_ref().ok_or_else(invalid)?.borrow().project(3, |s| &s.materials);
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                let p = rx.advance(source, control, body, g)?.progress();
                if rx.output_ready() {
                    self.phase = 7;
                }
                p
            }
            7 => {
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                let (value, p) = rx.take(g)?.ok_or_else(invalid)?;
                *self.materials = Some(value);
                rx.begin_close();
                self.phase = 8;
                p
            }
            8 => {
                let rx = self.materials_receiver.as_mut().ok_or_else(invalid)?;
                let p = rx.close_step(g)?.progress();
                if rx.terminal_is_empty() {
                    self.materials_receiver.take();
                    self.phase = 9;
                }
                p
            }
            9 => match self.source.as_mut().ok_or_else(invalid)?.take_authority(g)? {
                RetainedCloneSourceTake::Pending(p) => p,
                RetainedCloneSourceTake::Ready(value, p) => {
                    self.source.take();
                    *self.pending = Some(value);
                    self.phase = 10;
                    p
                }
            },
            10 => {
                let value = self.pending.as_mut().ok_or_else(invalid)?;
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
                self.phase = 11;
                receipt(d)
            }
            _ => return Err(invalid()),
        };
        Ok(if self.ready() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
    }
    /// 📫️ A real snapshot becomes transferable only after both row receivers and their source aliases have closed.
    pub fn ready(&self) -> bool {
        !self.closing && self.phase == 11 && self.output.is_some()
    }
    /// 📏️ Transfers only the fixed actual snapshot header; persistent pages and authored collections retain their identities.
    pub fn take_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.ready() {
            return Err(invalid());
        }
        Ok(fixed(2 * size_of::<Option<Fem2dSnapshot>>()))
    }
    /// 📤️ Taking output leaves the original plain node/material inputs resident for funded retirement.
    pub fn take(&mut self, g: RetainedCloneGrant) -> Result<Option<(Fem2dSnapshot, RetainedCloneProgress)>, ValueError> {
        let d = self.take_demand()?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(None);
        }
        self.phase = 12;
        Ok(Some((self.output.take().ok_or_else(invalid)?, receipt(d))))
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
            return rx.close_demands(body);
        }
        if let Some(rx) = self.materials_receiver.as_ref() {
            return rx.close_demands(body);
        }
        if let Some(source) = self.source.as_ref() {
            return source.next_close_demand(body);
        }
        if !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<Fem2dSnapshotValue>(false, &None, body);
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
    pub fn close_step(&mut self, g: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let d = self.close_demands(g.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(g, d) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(rx) = self.nodes_receiver.as_mut() {
            let p = rx.close_step(g)?.progress();
            if rx.terminal_is_empty() {
                self.nodes_receiver.take();
            }
            return Ok(RetainedCloneStep::Progress(p));
        }
        if let Some(rx) = self.materials_receiver.as_mut() {
            let p = rx.close_step(g)?.progress();
            if rx.terminal_is_empty() {
                self.materials_receiver.take();
            }
            return Ok(RetainedCloneStep::Progress(p));
        }
        if let Some(source) = self.source.as_mut() {
            let p = source.close_step(g)?.progress();
            if source.terminal_is_empty() {
                self.source.take();
            }
            return Ok(RetainedCloneStep::Progress(p));
        }
        let p = if !self.retirement.is_empty() {
            self.retirement.step_granted(g)?.progress()
        } else if let Some(s) = self.retirement.begin_granted(&mut self.pending, g)? {
            s.progress()
        } else if let Some(s) = self.retirement.begin_granted(&mut self.nodes, g)? {
            s.progress()
        } else if let Some(s) = self.retirement.begin_granted(&mut self.materials, g)? {
            s.progress()
        } else if let Some(s) = self.retirement.begin_granted(&mut self.output, g)? {
            s.progress()
        } else {
            Default::default()
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
    }
    /// 🏁️ Terminal destruction must release no original or newly received owner.
    pub fn terminal_is_empty(&self) -> bool {
        self.pending.is_none() && self.source.is_none() && self.nodes_receiver.is_none() && self.materials_receiver.is_none() && self.nodes.is_none() && self.materials.is_none() && self.output.is_none() && self.retirement.is_empty()
    }
}
impl ErasedSnapshotRetirement for Fem2dReceiving {
    fn next_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(body)
    }
    fn close_step(&mut self, g: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
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
        match Self::close_step(self, g) {
            Ok(s) => semio_framework_value::retirement::RetirementStep::Progress(s.progress()),
            Err(e) => semio_framework_value::retirement::RetirementStep::Failure(e),
        }
    }
    fn terminal_is_empty(&self) -> bool {
        Self::terminal_is_empty(self)
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
