//! 🖊️ Bounded DWG output over one owned triangle mesh: the polyface drawing, the stepped container writer and strict base64 text.
//!
//! @see <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> — §4 R2004 file structure

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand, ValueRefusalKind, base64_standard_encode, retained_clone::retained_clone_grant_admits};
use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::mesh_to_dwg_drawing;
use semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::{DwgWriteCursor, DwgWriteStep};

type Progress = RetainedCloneProgress;
const CHUNK: usize = 3072;
const PAGE_BYTES: usize = 0x7400;

#[derive(Clone, Copy)]
enum Phase { Convert, Write, Reserve, Encode, Ready, Taken }
semio_framework_value::artifact_retire_leaf!(Phase);

/// 🧾️ Holds the mesh, the resumable container writer and the base64 text under one retained owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct DwgMeshExport {
    mesh: Option<semio_framework::MeshData>,
    cursor: Option<DwgWriteCursor>,
    bytes: Vec<u8>,
    output: String,
    phase: Phase,
    record: usize,
}

fn invalid(reason: impl std::fmt::Display) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, format!("DWG export failed: {reason}"))
}

impl DwgMeshExport {
    /// 🎟️ Admits an owned mesh header without losing its backing on refusal.
    pub fn new(mesh: &mut Option<semio_framework::MeshData>) -> Result<Self, ValueError> {
        let source = mesh.as_ref().ok_or_else(|| invalid("the mesh is missing"))?;
        if source.positions.len() < 9 || source.positions.len() % 3 != 0 || source.indices.len() < 3 || source.indices.len() % 3 != 0 {
            return Err(invalid("the mesh holds no triangle"));
        }
        Ok(Self { mesh: mesh.take(), cursor: None, bytes: Vec::new(), output: String::new(), phase: Phase::Convert, record: 0 })
    }

    pub fn ready(&self) -> bool { matches!(self.phase, Phase::Ready) }

    pub fn take_output(&mut self) -> Option<String> {
        if !self.ready() { return None; }
        self.phase = Phase::Taken;
        Some(std::mem::take(&mut self.output))
    }

    /// 📏️ Quotes one conversion, one writer unit, the exact text reservation or one base64 chunk.
    pub fn demand(&self) -> RetirementDemand {
        let (copy_bytes, capacity_bytes) = match self.phase {
            Phase::Convert => (size_of::<DwgWriteCursor>(), 0),
            Phase::Write => (PAGE_BYTES, 0),
            Phase::Reserve => (0, 4 * self.bytes.len().div_ceil(3)),
            Phase::Encode => (CHUNK + CHUNK / 3 * 4, 0),
            Phase::Ready => (size_of::<String>(), 0),
            Phase::Taken => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    /// ⏱️ Runs one funded unit; cancellation retires the owner and drops the unfinished writer.
    pub fn step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand();
        if matches!(self.phase, Phase::Ready | Phase::Taken) || grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let mut receipt = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        self.advance(&mut receipt).map_err(|error| error.with_retained_progress(receipt))?;
        if !receipt.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "DWG export output exceeded its grant").with_retained_progress(receipt));
        }
        Ok(receipt)
    }

    fn advance(&mut self, receipt: &mut Progress) -> Result<(), ValueError> {
        match self.phase {
            Phase::Convert => {
                let mesh = self.mesh.take().ok_or_else(|| invalid("the mesh is missing"))?;
                self.cursor = Some(DwgWriteCursor::from_drawing(&mesh_to_dwg_drawing(&mesh).map_err(invalid)?).map_err(invalid)?);
                self.phase = Phase::Write;
            }
            Phase::Write => {
                let cursor = self.cursor.as_mut().ok_or_else(|| invalid("the writer is missing"))?;
                if let DwgWriteStep::Complete(bytes) = cursor.step().map_err(invalid)? {
                    self.bytes = bytes;
                    self.cursor = None;
                    self.phase = Phase::Reserve;
                }
            }
            Phase::Reserve => {
                let expected = 4 * self.bytes.len().div_ceil(3);
                self.output.try_reserve_exact(expected).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "DWG export text allocation failed"))?;
                receipt.retained_capacity_bytes = self.output.capacity();
                self.record = 0;
                self.phase = Phase::Encode;
            }
            Phase::Encode => {
                let start = self.record * CHUNK;
                let end = (start + CHUNK).min(self.bytes.len());
                self.output.push_str(&base64_standard_encode(&self.bytes[start..end]));
                self.record += 1;
                if end == self.bytes.len() { self.phase = Phase::Ready; }
            }
            Phase::Ready | Phase::Taken => unreachable!("terminal DWG text is transferred separately"),
        }
        Ok(())
    }

    /// 📈️ The completed fraction of the writer, for the job's progress report.
    pub fn progress(&self) -> f32 {
        match self.phase {
            Phase::Convert => 0.0,
            Phase::Write => 0.9 * self.cursor.as_ref().map_or(0.0, DwgWriteCursor::progress),
            Phase::Reserve => 0.9,
            Phase::Encode => 0.9 + 0.1 * (self.record * CHUNK) as f32 / self.bytes.len().max(1) as f32,
            Phase::Ready | Phase::Taken => 1.0,
        }
    }
}
