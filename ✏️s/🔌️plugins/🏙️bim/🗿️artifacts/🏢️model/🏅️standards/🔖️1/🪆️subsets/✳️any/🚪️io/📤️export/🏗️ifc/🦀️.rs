//! 🏗️ `s.bim.model@1/*` → `s.stdio.ifc@2x3/*` and `s.stdio.ifc@4/*`: the spatial structure, walls with openings, slabs, roofs, frames, stairs, railings, spaces, curtain walls, grids,
//! materials, property sets (own sets of elements, authored sets of types in the `HasPropertySets` of their type object; defaults and inherited values are inferred, never written), classification systems (one
//! `IfcClassification` per system, one `IfcClassificationReference` per table row, one `IfcRelAssociatesClassification` per used code listing every element and type that carries it, the parent column in the
//! `Semio_ClassificationParents` set of the project), base quantities and annotations (`IfcAnnotation`) of a [`ModelSnapshot`] as an IFC Part-21 file of the [`Schema`] asked for.
//! 🔖 `IoFidelity::Lossy`: IFC has no slot for parametric constraints (top constraints, joins, derived inferences); geometry and identity are exact. IFC 2x3 has also no slot for property set templates or for the parent of a classification
//! row (a foreign reader sees the flat reference rows); IFC4 writes them as the `IfcProjectLibrary` of the project ([`ifc4`]), its bodies as `IfcTriangulatedFaceSet`s, its windows and doors with their `IfcWindowType` / `IfcDoorType` operation and
//! no owner history. The file is written in stages ([`STAGES`]) so a job can report progress and stop between two of them.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ and https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/

use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use crate::standards::v1::subsets::any::schema::inferences::phase_visibility;
use crate::standards::v1::subsets::any::schema::inferences::quantities::ElementQuantity;
use crate::{ModelInference, ModelSnapshot, Phase};
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Header, Part21Value};
use std::collections::BTreeMap;

#[path = "🧱️codec/🦀️.rs"]
pub mod codec;
#[path = "🧰️writer/🦀️.rs"]
pub mod writer;
#[path = "🧭️frames/🦀️.rs"]
pub mod frames;
#[path = "🧊️brep/🦀️.rs"]
pub mod brep;
#[path = "🏛️spatial/🦀️.rs"]
pub mod spatial;
#[path = "🧬️data/🦀️.rs"]
pub mod data;
#[path = "🏰️walls/🦀️.rs"]
pub mod walls;
#[path = "🧷️wall-sweeps/🦀️.rs"]
pub mod wall_sweeps;
#[path = "🏗️frame/🦀️.rs"]
pub mod frame;
#[path = "⬜️horizontal/🦀️.rs"]
pub mod horizontal;
#[path = "🔲️ceilings/🦀️.rs"]
pub mod ceilings;
#[path = "🪜️circulation/🦀️.rs"]
pub mod circulation;
#[path = "🛝️ramps/🦀️.rs"]
pub mod ramps;
#[path = "🪑️components/🦀️.rs"]
pub mod components;
#[path = "🌀️mep/🦀️.rs"]
pub mod mep;
#[path = "🏠️spaces/🦀️.rs"]
pub mod spaces;
#[path = "🏘️zoning/🦀️.rs"]
pub mod zoning;
#[path = "🧭️options/🦀️.rs"]
pub mod options;
#[path = "🪟️curtain/🦀️.rs"]
pub mod curtain;
#[path = "📏️grids/🦀️.rs"]
pub mod grids;
#[path = "🪧️annotations/🦀️.rs"]
pub mod annotations;
#[path = "🔬️projection/🦀️.rs"]
pub mod projection;
#[path = "📚️ifc4/🦀️.rs"]
pub mod ifc4;
#[path = "🔥️energy/🦀️.rs"]
pub mod energy;

pub use writer::Schema;
use writer::{opt_text, rf, unset, Ifc, V};

/// 🪪️ The IFC 2x3 dialect this leaf writes.
pub const IFC_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

//#region 🔖️Context
/// 🪜️ One storey as written: its entity, its placement and its building-relative elevation.
#[derive(Clone, Copy, Debug)]
pub struct StoreyRef {
    pub ifc: u64,
    pub placement: u64,
    pub elevation: f64,
}

/// 🏢️ One building as written: its entity and its placement.
#[derive(Clone, Copy, Debug)]
pub struct BuildingRef {
    pub ifc: u64,
    pub placement: u64,
}

/// 🔢️ One base quantity of an element.
#[derive(Clone, Debug)]
pub enum Quantity {
    Length(&'static str, f64),
    Area(&'static str, f64),
    Volume(&'static str, f64),
    Count(&'static str, i64),
}

/// 🔗️ Everything the families record while writing so the relationships can be emitted once, in canonical order.
#[derive(Default)]
pub struct Links {
    pub elements: BTreeMap<String, u64>,
    pub contained: BTreeMap<String, Vec<u64>>,
    pub in_building: BTreeMap<String, Vec<u64>>,
    pub aggregated: BTreeMap<u64, Vec<u64>>,
    pub typed: BTreeMap<u64, Vec<u64>>,
    pub materials: BTreeMap<u64, Vec<u64>>,
    pub quantities: Vec<(u64, &'static str, Vec<Quantity>)>,
    pub authoring: Vec<(u64, Vec<(&'static str, V)>)>,
    pub types: BTreeMap<(&'static str, String), u64>,
    pub material_defs: BTreeMap<String, u64>,
    pub layer_sets: BTreeMap<(&'static str, String), u64>,
    pub connections: Vec<(String, String, &'static str)>,
    pub library: Option<u64>,
    pub templates: BTreeMap<String, u64>,
    pub templated: BTreeMap<u64, Vec<u64>>,
    pub sources: BTreeMap<String, u64>,
    pub references: BTreeMap<(String, String), u64>,
    pub derived: BTreeMap<String, Vec<(&'static str, Vec<(&'static str, V)>)>>,
}

/// 🧳️ Everything an export keeps between two stages: it owns no borrow, so a job can drop the inference between steps and read it again.
pub struct Staged {
    pub ifc: Ifc,
    pub storeys: BTreeMap<String, StoreyRef>,
    pub buildings: BTreeMap<String, BuildingRef>,
    pub links: Links,
    pub notes: Vec<String>,
}

impl Staged {
    /// 🌱️ The empty state of an export of `model` in `schema`.
    pub fn new(schema: Schema, model: &ModelSnapshot) -> Self {
        let true_north = model.sites.values().next().map_or(0.0, |site| site.true_north);
        Self { ifc: Ifc::in_schema(schema, &model.project.author, &model.project.organization, true_north), storeys: BTreeMap::new(), buildings: BTreeMap::new(), links: Links::default(), notes: Vec::new() }
    }
}

/// 🏗️ The export under construction: the model, the one inference of it every family reads, the writer and the recorded links.
pub struct Export<'a> {
    pub model: &'a ModelSnapshot,
    pub inferred: &'a ModelInference,
    pub ifc: Ifc,
    pub storeys: BTreeMap<String, StoreyRef>,
    pub buildings: BTreeMap<String, BuildingRef>,
    pub links: Links,
    pub notes: Vec<String>,
}

impl<'a> Export<'a> {
    /// 🌱️ Starts an export of `model` in `schema` that reads its already inferred fields.
    pub fn new(schema: Schema, model: &'a ModelSnapshot, inferred: &'a ModelInference) -> Self {
        Self::resume(model, inferred, Staged::new(schema, model))
    }

    /// ▶️ Continues an export from the state a stage left.
    pub fn resume(model: &'a ModelSnapshot, inferred: &'a ModelInference, staged: Staged) -> Self {
        Self { model, inferred, ifc: staged.ifc, storeys: staged.storeys, buildings: staged.buildings, links: staged.links, notes: staged.notes }
    }

    /// ⏸️ Gives the state back so the borrows end.
    pub fn suspend(self) -> Staged {
        Staged { ifc: self.ifc, storeys: self.storeys, buildings: self.buildings, links: self.links, notes: self.notes }
    }

    /// 🔖️ The schema of the file.
    pub fn schema(&self) -> Schema {
        self.ifc.schema
    }

    /// 🎚️ `v2x3` for an IFC 2x3 file, `v4` for an IFC4 file.
    pub fn by<T>(&self, v2x3: T, v4: T) -> T {
        self.ifc.schema.pick(v2x3, v4)
    }

    /// 🧊️ The inferred solid of an element.
    pub fn solid(&self, id: &str) -> Option<&'a ElementSolid> {
        self.inferred.element_solids.get(id)
    }

    /// 🧮️ The inferred take-off of an element.
    pub fn measure(&self, id: &str) -> Option<&'a ElementQuantity> {
        self.inferred.quantities.elements.get(id)
    }

    /// 🧮️ Records the base quantities of `entity` as `set` when the take-off has a row for element `id`; `pick` names the measures the set carries.
    pub fn quantify(&mut self, entity: u64, set: &'static str, id: &str, pick: impl FnOnce(&ElementQuantity) -> Vec<Quantity>) {
        if let Some(row) = self.measure(id) {
            self.links.quantities.push((entity, set, pick(row)));
        }
    }

    /// 🏷️ Writes one product: `GlobalId`, `Name`, then `ObjectType`, `ObjectPlacement`, `Representation`, `Tag` and the entity's own `tail`.
    pub fn product(&mut self, entity: &str, id: &str, name: &str, placement: u64, shape: Option<u64>, tail: Vec<V>) -> u64 {
        let mut args = vec![unset(), rf(placement), shape.map_or(unset(), rf), Self::tag(id)];
        args.extend(tail);
        self.ifc.rooted(entity, id, &Self::label(name, id), "", args)
    }

    /// 📝️ Records that an item could not be written.
    pub fn skip(&mut self, kind: &str, id: &str, reason: &str) {
        self.notes.push(format!("{kind} {id}: {reason}"));
    }

    /// 📌️ Registers a written product: contained in its storey, findable by element id.
    pub fn contain(&mut self, storey: &str, id: &str, product: u64) {
        self.links.elements.insert(id.to_string(), product);
        self.links.contained.entry(storey.to_string()).or_default().push(product);
        self.phase(id, product);
    }

    /// 🕰️ Records the construction phase of the written product of element `id` as the `Phase` row of its authoring data; new work (the default, and an element that carries no phase) writes no row, an opening carries the phase of its host.
    pub fn phase(&mut self, id: &str, product: u64) {
        let phase = phase_visibility::phase_of(self.model, id).filter(|phase| *phase != Phase::New);
        if let Some(phase) = phase {
            self.links.authoring.push((product, vec![("Phase", data::label(&format!("{phase:?}")))]));
        }
    }

    /// 🏷️ The `Name` of an element: its authored name, else its id.
    pub fn label(name: &str, id: &str) -> String {
        if name.is_empty() {
            id.to_string()
        } else {
            name.to_string()
        }
    }

    /// 📋️ `Tag` argument of an element: its id.
    pub fn tag(id: &str) -> V {
        opt_text(id)
    }
}
//#endregion 🔖️Context

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
    ("types", data::emit_types),
    ("spatial", spatial::emit),
    ("walls", walls::emit),
    ("wall-sweeps", wall_sweeps::emit),
    ("curtain-walls", curtain::emit),
    ("frames", frame::emit),
    ("slabs-and-roofs", horizontal::emit),
    ("ceilings", ceilings::emit),
    ("stairs-and-railings", circulation::emit),
    ("ramps", ramps::emit),
    ("components", components::emit),
    ("mep", mep::emit),
    ("systems", mep::emit_systems),
    ("spaces", spaces::emit),
    ("zones", zoning::emit),
    ("options", options::emit),
    ("grids", grids::emit),
    ("annotations", annotations::emit),
    ("energy", energy::emit),
    ("structure", structure::emit),
    ("links", data::emit_links),
    ("connections", walls::connect),
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
    inference::try_with_inference(None, model, |inferred| inferred_to_part21(schema, model, inferred)).map_err(|error| error.to_string())
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
        Schema::Ifc2x3 => codec::encode_document(document),
        Schema::Ifc4 => codec::encode_ifc4(&document),
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
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let (bytes, notes) = export_ifc2x3(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ModelIntoIfc2x3: {message}"))))?;
        let diagnostics = notes
            .into_iter()
            .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.ifc.export.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
            .collect();
        Ok(IoOutcome { value: IoPayload::Binary(bytes), diagnostics })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔭️schema4/🦀️.rs"]
mod schema4_tests;

#[path = "🦴️structure/🦀️.rs"]
pub mod structure;
