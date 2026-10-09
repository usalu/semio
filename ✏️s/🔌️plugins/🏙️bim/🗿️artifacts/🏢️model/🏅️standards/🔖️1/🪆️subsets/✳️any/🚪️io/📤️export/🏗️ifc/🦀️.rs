//! 🏗️ `s.bim.model@1/*` → `s.stdio.ifc@2x3/*`: the spatial structure, walls with openings, slabs, roofs, frames, stairs, railings, spaces, curtain walls, grids,
//! materials, property sets, classifications, base quantities and annotations (`IfcAnnotation`) of a [`ModelSnapshot`] as an IFC 2x3 Part-21 file.
//! 🔖 `IoFidelity::Lossy`: IFC 2x3 has no slot for parametric constraints (top constraints, joins, derived inferences); geometry and identity are exact.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/

use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
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
#[path = "🏠️spaces/🦀️.rs"]
pub mod spaces;
#[path = "🏘️zoning/🦀️.rs"]
pub mod zoning;
#[path = "🪟️curtain/🦀️.rs"]
pub mod curtain;
#[path = "📏️grids/🦀️.rs"]
pub mod grids;
#[path = "🪧️annotations/🦀️.rs"]
pub mod annotations;
#[path = "🔬️projection/🦀️.rs"]
pub mod projection;

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
    /// 🌱️ Starts an export of `model` that reads its already inferred fields.
    pub fn new(model: &'a ModelSnapshot, inferred: &'a ModelInference) -> Self {
        let true_north = model.sites.values().next().map_or(0.0, |site| site.true_north);
        Self { model, inferred, ifc: Ifc::new(&model.project.author, &model.project.organization, true_north), storeys: BTreeMap::new(), buildings: BTreeMap::new(), links: Links::default(), notes: Vec::new() }
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
fn header(model: &ModelSnapshot) -> Part21Header {
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_description = vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())];
    header.file_name = vec![
        Part21Value::Str(format!("{}.ifc", model.project.name)),
        Part21Value::Str(String::new()),
        Part21Value::List(vec![Part21Value::Str(model.project.author.clone())]),
        Part21Value::List(vec![Part21Value::Str(model.project.organization.clone())]),
        Part21Value::Str("semio BIM".into()),
        Part21Value::Str("semio BIM".into()),
        Part21Value::Str(String::new()),
    ];
    header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])];
    header
}

/// 🏗️ The Part-21 document of `model` from its inference, plus a note per item that could not be written.
pub fn inferred_to_part21(model: &ModelSnapshot, inferred: &ModelInference) -> (Part21Document, Vec<String>) {
    let mut export = Export::new(model, inferred);
    data::emit_types(&mut export);
    spatial::emit(&mut export);
    walls::emit(&mut export);
    curtain::emit(&mut export);
    frame::emit(&mut export);
    horizontal::emit(&mut export);
    ceilings::emit(&mut export);
    circulation::emit(&mut export);
    ramps::emit(&mut export);
    spaces::emit(&mut export);
    zoning::emit(&mut export);
    grids::emit(&mut export);
    annotations::emit(&mut export);
    data::emit_links(&mut export);
    let notes = std::mem::take(&mut export.notes);
    let head = header(model);
    (export.ifc.finish(head), notes)
}

/// 🏗️ The Part-21 document of `model` plus a note per item that could not be written: the inference comes from the shared session, so an export after an edit recomputes only what the edit touched.
pub fn model_to_part21(model: &ModelSnapshot) -> Result<(Part21Document, Vec<String>), String> {
    registry::try_with_inference(None, model, |inferred| inferred_to_part21(model, inferred)).map_err(|error| error.to_string())
}

/// 📤️ The IFC 2x3 file bytes of `model` plus a note per item that could not be written.
pub fn export_ifc2x3(model: &ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String> {
    let (document, notes) = model_to_part21(model)?;
    let bytes = codec::encode_document(document)?;
    Ok((bytes, notes))
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
