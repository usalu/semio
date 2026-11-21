//! ♻️ Retires the writer, controlled projection and exact graph under the exporting job's source lease.

use std::mem::ManuallyDrop;
use semio_framework_value::{
    RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetireOwned, RetirementDemand, ValueError,
    retirement::controlled::ControlledRetirement,
};
use semio_os_kernel::os_store::snapshot_read::settle_close_poll;
use semio_plugin::{app::{plugin_grant_funds, plugin_typed_owner_demand, plugin_typed_owner_original},
    __job::{InteractiveJobCloseStep, OriginalTurnFailure, StepContext, close_original_retained_effect_step}};
use crate::standards::v1_7::subsets::base::{
    io::export::{PinnedPdfGraph, exact_graph_proof::ProjectionTrailer, ordinary_graft::OrdinaryRows},
    modules::{lexer::CosGrant, lift::{ControlledLiftEvent, ControlledLiftPartial}},
    schema::snapshot::{PdfPage, PdfSnapshot},
};
use super::program::ClassicPdfSaveRetirement;

#[derive(RetireOwned)]
struct OwnedGraphParts {
    graph: PinnedPdfGraph,
    pending_rows: OrdinaryRows,
    typed_projection: Option<PdfSnapshot>,
    partial_page: Option<PdfPage>,
    control_trailer: Option<ProjectionTrailer>,
    scratch: Option<Vec<u8>>,
}

pub struct ProgramClose {
    active: ManuallyDrop<Option<ClassicPdfSaveRetirement>>,
    owned: ManuallyDrop<Option<OwnedGraphParts>>,
    cursor: ManuallyDrop<Option<ControlledRetirement<OwnedGraphParts>>>,
    proof_closed: bool,
    release: usize,
}

impl ProgramClose {
    pub fn new(mut source: ClassicPdfSaveRetirement) -> Self {
        source.proof.begin_close();
        Self { active: ManuallyDrop::new(Some(source)), owned: Default::default(), cursor: Default::default(),
            proof_closed: false, release: 0 }
    }

    pub fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some(active) = self.active.as_ref() else {
            return plugin_typed_owner_demand(&self.owned, &self.cursor, body);
        };
        if !active.control_birth.terminal_is_empty() { return active.control_birth.close_demand(body); }
        if !active.scratch_birth.terminal_is_empty() { return active.scratch_birth.close_demand(body); }
        if !self.proof_closed {
            return Ok(RetirementDemand { release_bytes: self.release, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { copy_bytes: size_of::<OwnedGraphParts>(), depth: 1, ..Default::default() })
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if let Some(active) = self.active.as_mut() {
            if let Some(progress) = active.control_birth.close_step(cx)? { return Ok(InteractiveJobCloseStep::Pending { progress }); }
            if let Some(progress) = active.scratch_birth.close_step(cx)? { return Ok(InteractiveJobCloseStep::Pending { progress }); }
        }
        let demand = self.demands(cx.retained_grant().maximum_copy_bytes)?;
        close_original_retained_effect_step(cx, demand, |effect| self.close_one(effect))
    }

    fn close_one(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let grant = effect.grant();
        if let Some(active) = self.active.as_mut() {
            if !self.proof_closed {
                if grant.maximum_items == 0 || !effect.performed(1)? {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                let turn = active.proof.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: grant.maximum_release_bytes });
                self.release = match turn.event {
                    Ok(ControlledLiftEvent::NeedsRelease { bytes }) => bytes,
                    Ok(ControlledLiftEvent::Closed) => { self.proof_closed = true; 0 }
                    Ok(_) => 0,
                    Err(_) => return Err(ValueError::invariant("PDF controlled graph close failed")),
                };
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { released_bytes: turn.released_bytes, ..Default::default() }));
            }
            let demand = self.demands(grant.maximum_copy_bytes)?;
            if grant.maximum_items == 0 || !plugin_grant_funds(demand, grant) || !effect.performed(1)? {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            let mut active = self.active.take().ok_or_else(|| ValueError::invariant("PDF graph close lost its original owner"))?;
            let ControlledLiftPartial { snapshot, page } = active.proof.take_partial();
            *self.owned = Some(OwnedGraphParts { graph: active.graph, pending_rows: active.pending_graft.take_partial(),
                typed_projection: snapshot, partial_page: page, control_trailer: active.control_trailer, scratch: active.scratch });
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1,
                copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        settle_close_poll(plugin_typed_owner_original(&mut self.owned, &mut self.cursor, effect))
    }

    pub fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.owned.is_none() && self.cursor.is_none() }
}

impl Drop for ProgramClose {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "PDF graph requires paid close");
    }
}
