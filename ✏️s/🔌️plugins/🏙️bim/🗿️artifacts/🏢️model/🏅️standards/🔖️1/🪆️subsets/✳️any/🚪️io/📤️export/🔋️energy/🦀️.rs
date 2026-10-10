//! 🔋️ `s.bim.model@1/*` → `s.energy.model@1/*`: the thermal envelope and the conditions of a BIM model as the snapshot of an energy model, the document the energy artifact simulates. The export reads the
//! `energy_envelopes` of the inference (surfaces with boundary, neighbour, polygon and transmittance) and the authored layers, conditions, window and door data and site; it computes no geometry. The payload is the
//! artifact's own text, the `.energy` file its editor opens ([`document::dsl_text`]); [`document::json_text`] writes the same snapshot as the JSON document its `s.stdio.json` export writes, which the schema `🔣️.json`
//! types member by member. A job runs the stages of [`build::STAGES`] one per step with [`StagedEnergy`] and stops between two.
//! 🔖 `IoFidelity::Lossy`: the energy model has no home for the ground factor of a surface against the ground, the frame fraction of a window (it is folded into the SHGC), the visible transmittance, the infiltration and
//! the weather; windows and doors behind a partition are merged into their wall; curtain walls and surfaces without thermal data are not written (each is a note, a warning of the hop).
//! 📎 ../../../../../../../../../🔋️energy/🗿️artifacts/🔋️model/🦀️.rs

use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::ModelSnapshot;
use build::{Build, STAGES};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use target::EnergyModel;

#[path = "🧬️target/🦀️.rs"]
pub mod target;

#[path = "🧱️library/🦀️.rs"]
pub mod library;

#[path = "🔗️pairs/🦀️.rs"]
pub mod pairs;

#[path = "🏗️build/🦀️.rs"]
pub mod build;

#[path = "📖️document/🦀️.rs"]
pub mod document;

#[path = "📊️table/🦀️.rs"]
pub mod table;

/// 🪪️ The dialect this leaf writes: the energy model artifact.
pub const ENERGY_DIALECT: Dialect = Dialect { artifact_kind: "s.energy.model", standard: StandardId("1"), subset: SubsetId::ANY };

/// 🧵️ An export that runs one [`STAGES`] entry per step, so a job can report how far it is and stop between two stages; it owns no borrow of the model or the inference.
#[derive(Default)]
pub struct StagedEnergy {
    build: Build,
    next: usize,
}

impl StagedEnergy {
    /// 🌱️ An export that has not run a stage yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📈️ How far the stages are, from zero to one.
    pub fn fraction(&self) -> f32 {
        self.next as f32 / STAGES.len() as f32
    }

    /// 🏷️ The name of the stage the next step runs, `None` once all have run.
    pub fn stage(&self) -> Option<&'static str> {
        STAGES.get(self.next).map(|(name, _)| *name)
    }

    /// ⏩️ Runs the next stage; the model and its notes once the last has run, `None` before. A refusal ends the export.
    pub fn step(&mut self, model: &ModelSnapshot, inferred: &ModelInference) -> Result<Option<(EnergyModel, Vec<String>)>, String> {
        if let Some((_, stage)) = STAGES.get(self.next) {
            stage(&mut self.build, model, inferred)?;
            self.next += 1;
        }
        if self.next < STAGES.len() {
            return Ok(None);
        }
        self.build.finish().map(Some)
    }
}

/// 🏗️ The energy model of `model` from its inference, plus a note per item that could not be written as it is.
pub fn inferred_to_energy(model: &ModelSnapshot, inferred: &ModelInference) -> Result<(EnergyModel, Vec<String>), String> {
    let mut staged = StagedEnergy::new();
    loop {
        if let Some(done) = staged.step(model, inferred)? {
            return Ok(done);
        }
    }
}

/// 🏗️ The energy model of `model`, the inference coming from the shared session: an export after an edit recomputes only what the edit touched.
pub fn model_to_energy(model: &ModelSnapshot) -> Result<(EnergyModel, Vec<String>), String> {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::try_with_inference(None, model, |inferred| inferred_to_energy(model, inferred)).map_err(|error| error.to_string())?
}

/// 📤️ The `.energy` text of `model` plus a note per item that could not be written as it is.
pub fn export_energy(model: &ModelSnapshot) -> Result<(String, Vec<String>), String> {
    let (energy, notes) = model_to_energy(model)?;
    Ok((document::dsl_text(&energy)?, notes))
}

/// 📤️ The JSON snapshot of `model` plus a note per item that could not be written as it is.
pub fn export_energy_json(model: &ModelSnapshot) -> Result<(String, Vec<String>), String> {
    let (energy, notes) = model_to_energy(model)?;
    Ok((document::json_text(&energy)?, notes))
}

fn warning(note: String) -> semio_framework_diagnostic::Diagnostic {
    semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.energy.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() }
}

//#region 🔖️Serializer
/// 🔋️ The energy model serializer of the BIM model.
pub struct ModelIntoEnergy;

impl Serializer<ModelSnapshot> for ModelIntoEnergy {
    const INTO: Dialect = ENERGY_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (text, notes) = export_energy(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoEnergy: {message}"))))?;
        Ok(IoOutcome { value: IoPayload::Text(text), diagnostics: notes.into_iter().map(warning).collect() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
