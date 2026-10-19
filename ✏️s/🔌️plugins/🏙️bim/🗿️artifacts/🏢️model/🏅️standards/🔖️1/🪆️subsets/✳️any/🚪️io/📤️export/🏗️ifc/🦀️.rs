//! 🏗️ `s.bim.model@1/*` → `s.stdio.ifc@2x3/*` and `s.stdio.ifc@4/*`: the spatial structure, walls with openings, slabs, roofs, frames, stairs, railings, spaces, curtain walls, grids,
//! materials, property sets (own sets of elements, authored sets of types in the `HasPropertySets` of their type object; defaults and inherited values are inferred, never written), classification systems (one
//! `IfcClassification` per system, one `IfcClassificationReference` per table row, one `IfcRelAssociatesClassification` per used code listing every element and type that carries it, the parent column in the
//! `Semio_ClassificationParents` set of the project), base quantities and annotations (`IfcAnnotation`) of a [`ModelSnapshot`] as an IFC Part-21 file of the [`Schema`] asked for.
//! 🔖 `IoFidelity::Lossy`: IFC has no slot for parametric constraints (top constraints, joins, derived inferences); geometry and identity are exact. IFC 2x3 has also no slot for property set templates or for the parent of a classification
//! row (a foreign reader sees the flat reference rows); IFC4 writes them as the `IfcProjectLibrary` of the project ([`ifc4`]), its bodies as `IfcTriangulatedFaceSet`s, its windows and doors with their `IfcWindowType` / `IfcDoorType` operation and
//! no owner history. The file is written in stages ([`STAGES`]) so a job can report progress and stop between two of them.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ and https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::ElementQuantity;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Phase;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Value};
use std::collections::BTreeMap;

#[path = "🧭️options/🦀️.rs"]
pub mod options;
#[path = "📏️grids/🦀️.rs"]
pub mod grids;
#[path = "🔬️projection/🦀️.rs"]
pub mod projection;
#[path = "📚️ifc4/🦀️.rs"]
pub mod ifc4;
#[path = "🔥️energy/🦀️.rs"]
pub mod energy;

use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::Schema;
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context::{BuildingRef, Export, Staged, StoreyRef};
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::{opt_text, rf, unset, Ifc, V};

/// 🪪️ The IFC 2x3 dialect this leaf writes.
pub const IFC_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

//#region 🔖️Document
fn header(model: &ModelSnapshot, schema: Schema) -> Part21Header {
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_description = vec![Part21Value::List(vec![Part21Value::Str(schema.pick("ViewDefinition [CoordinationView_V2.0]", "ViewDefinition []").into())]), Part21Value::Str("2;1".into())];
    header.file_name = vec![
        Part21Value::Str(format!("{}.ifc", model.project.name)),
        Part21Value::Str(String::new()),
        Part21Value::List(vec![Part21Value::Str(model.project.author.clone())]),
        Part21Value::List(vec![Part21Value::Str(model.project.organization.clone())]),
        Part21Value::Str("semio BIM".into()),
        Part21Value::Str("semio BIM".into()),
        Part21Value::Str(String::new()),
    ];
    header.file_schema = vec![Part21Value::List(vec![Part21Value::Str(schema.id().into())])];
    header
}

/// 🪜️ One stage of the export: the name of the family it writes and the function that writes it.
pub type Stage = (&'static str, fn(&mut Export<'_>));

/// 🪜️ The stages of an export in the order they run; each reads the inference and the records of the stages before it.
pub const STAGES: &[Stage] = &[
    ("library", ifc4::emit_library),
    ("types", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data::emit_types),
    ("spatial", semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::spatial::emit),
    ("walls", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls::emit),
    ("wall-sweeps", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::wall_sweeps::emit),
    ("curtain-walls", semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::ifc::curtain::emit),
    ("frames", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::frame::emit),
    ("slabs-and-roofs", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::horizontal::emit),
    ("ceilings", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ceilings::emit),
    ("stairs-and-railings", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::circulation::emit),
    ("ramps", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ramps::emit),
    ("components", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::components::emit),
    ("mep", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::mep::emit),
    ("systems", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::mep::emit_systems),
    ("spaces", semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::ifc::spaces::emit),
    ("zones", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::zoning::emit),
    ("options", options::emit),
    ("grids", grids::emit),
    ("annotations", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::annotations::emit),
    ("energy", energy::emit),
    ("structure", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::structure::emit),
    ("links", semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data::emit_links),
    ("connections", semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls::connect),
];

/// 🏗️ The Part-21 document of `model` in `schema` from its inference, plus a note per item that could not be written.
pub fn inferred_to_part21(schema: Schema, model: &ModelSnapshot, inferred: &ModelInference) -> (Part21Document, Vec<String>) {
    let mut export = Export::new(schema, model, inferred);
    for (_, stage) in STAGES {
        stage(&mut export);
    }
    finish(export, model)
}

/// 🏁️ Seals an export whose stages have all run: the document with its header and the notes.
pub fn finish(mut export: Export<'_>, model: &ModelSnapshot) -> (Part21Document, Vec<String>) {
    let notes = std::mem::take(&mut export.notes);
    let head = header(model, export.ifc.schema);
    (export.ifc.finish(head), notes)
}

/// 🏗️ The Part-21 document of `model` in `schema` plus a note per item that could not be written: the inference comes from the shared session, so an export after an edit recomputes only what the edit touched.
pub fn model_to_part21(schema: Schema, model: &ModelSnapshot) -> Result<(Part21Document, Vec<String>), String> {
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, model, |inferred| inferred_to_part21(schema, model, inferred)).map_err(|error| error.to_string())
}

/// 📤️ The IFC 2x3 file bytes of `model` plus a note per item that could not be written.
pub fn export_ifc2x3(model: &ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String> {
    let (document, notes) = model_to_part21(Schema::Ifc2x3, model)?;
    Ok((encode(Schema::Ifc2x3, document)?, notes))
}

/// 📤️ The IFC4 file bytes of `model` plus a note per item that could not be written.
pub fn export_ifc4(model: &ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String> {
    let (document, notes) = model_to_part21(Schema::Ifc4, model)?;
    Ok((encode(Schema::Ifc4, document)?, notes))
}

/// 📤️ The file bytes of an already written document in `schema`.
pub fn encode(schema: Schema, document: Part21Document) -> Result<Vec<u8>, String> {
    match schema {
        Schema::Ifc2x3 => semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec::encode_document(document),
        Schema::Ifc4 => semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec::encode_ifc4(&document),
    }
}

/// 🧵️ An export that runs one [`STAGES`] entry per step, so a job can report how far it is and stop between two stages; it owns no borrow of the inference.
pub struct StagedExport {
    staged: Option<Staged>,
    next: usize,
}

impl StagedExport {
    /// 🌱️ An export of `model` in `schema` that has not run a stage yet.
    pub fn new(schema: Schema, model: &ModelSnapshot) -> Self {
        Self { staged: Some(Staged::new(schema, model)), next: 0 }
    }

    /// 📈️ How far the stages are, from zero to one.
    pub fn fraction(&self) -> f32 {
        self.next as f32 / STAGES.len() as f32
    }

    /// 🏷️ The name of the stage the next step runs, `None` once all have run.
    pub fn stage(&self) -> Option<&'static str> {
        STAGES.get(self.next).map(|(name, _)| *name)
    }

    /// ⏩️ Runs the next stage; the document and its notes once the last has run, `None` before.
    pub fn step(&mut self, model: &ModelSnapshot, inferred: &ModelInference) -> Option<(Part21Document, Vec<String>)> {
        let mut export = Export::resume(model, inferred, self.staged.take()?);
        if let Some((_, stage)) = STAGES.get(self.next) {
            stage(&mut export);
            self.next += 1;
        }
        if self.next < STAGES.len() {
            self.staged = Some(export.suspend());
            return None;
        }
        Some(finish(export, model))
    }
}
//#endregion 🔖️Document

//#region 🔖️Serializer
/// 🏗️ The IFC 2x3 serializer of the BIM model.
pub struct ModelIntoIfc2x3;

impl Serializer<ModelSnapshot> for ModelIntoIfc2x3 {
    const INTO: Dialect = IFC_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let (bytes, notes) = export_ifc2x3(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoIfc2x3: {message}"))))?;
        let diagnostics = notes
            .into_iter()
            .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.ifc.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
            .collect();
        Ok(IoOutcome { value: IoPayload::Binary(bytes), diagnostics })
    }
}
//#endregion 🔖️Serializer

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;





