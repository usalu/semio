//! 🏗️ `s.stdio.ifc@2x3/*` and `s.stdio.ifc@4/*` → `s.bim.model@1/*`: the spatial structure, storeys (elevations become levels and heights), walls from their axis and layer set, openings with their windows and doors,
//! slabs, columns, beams, spaces and grids, plus materials, types, property sets (of elements and of type objects) and classification systems: each `IfcClassification` becomes a system (`cs-<slug of its name>`) whose table
//! is its `IfcClassificationReference` rows (parents from the `Semio_ClassificationParents` set of the project; a foreign file has none and imports flat tables) and each `IfcRelAssociatesClassification` the code its
//! elements and types carry in that system. Every entity class that is not understood is reported as one diagnostic with its count.
//! 🔖 `IoFidelity::Semantic`: roofs, stairs, railings and curtain walls are restored from the authored record the export writes (a foreign file gets the best-effort reading of each module), geometry that is only a mesh is reported, not imported.
//! IFC4 files also give their property set templates (the project library), their classification chains, window and door operations and material categories.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ and https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/

use crate::standards::v1::subsets::any::io::export::ifc::ifc4::IFC4_DIALECT;
use crate::standards::v1::subsets::any::io::export::ifc::{codec, Schema, IFC_DIALECT};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::{kinds, instance as inference};
use crate::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use crate::ModelSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_artifact_reference::Dialect;
use semio_s_artifact_stdio_ifc::part21::Part21Value;
use std::collections::BTreeMap;

#[path = "🧭️frames/🦀️.rs"]
pub mod frames;
#[path = "📖️reader/🦀️.rs"]
pub mod reader;
#[path = "🏛️spatial/🦀️.rs"]
pub mod spatial;
#[path = "🧬️data/🦀️.rs"]
pub mod data;
#[path = "🧱️walls/🦀️.rs"]
pub mod walls;
#[path = "🏗️elements/🦀️.rs"]
pub mod elements;
#[path = "🏘️zoning/🦀️.rs"]
pub mod zoning;
#[path = "🧭️options/🦀️.rs"]
pub mod options;
#[path = "🔥️energy/🦀️.rs"]
pub mod energy;
#[path = "🛝️ramps/🦀️.rs"]
pub mod ramps;
#[path = "🪑️components/🦀️.rs"]
pub mod components;
#[path = "🌀️mep/🦀️.rs"]
pub mod mep;
#[path = "🔲️ceilings/🦀️.rs"]
pub mod ceilings;
#[path = "🧷️wall-sweeps/🦀️.rs"]
pub mod wall_sweeps;
#[path = "🪟️curtain/🦀️.rs"]
pub mod curtain;
#[path = "⬜️horizontal/🦀️.rs"]
pub mod horizontal;
#[path = "🪜️circulation/🦀️.rs"]
pub mod circulation;
#[path = "🧗️attach/🦀️.rs"]
pub mod attach;

use frames::Rigid;
use reader::Doc;

/// 🏗️ The import under construction: the indexed document, the model being filled and what is known about each imported entity.
pub struct Import<'a> {
    pub schema: Schema,
    pub doc: Doc<'a>,
    pub model: ModelSnapshot,
    pub notes: Vec<String>,
    pub ids: BTreeMap<u64, String>,
    pub site_world: BTreeMap<String, Rigid>,
    pub building_world: BTreeMap<String, Rigid>,
    pub storey_ids: BTreeMap<u64, String>,
    pub levels: BTreeMap<String, StoreyLevel>,
    pub material_ids: BTreeMap<u64, String>,
    pub type_ids: BTreeMap<u64, String>,
}

impl<'a> Import<'a> {
    fn new(schema: Schema, doc: Doc<'a>) -> Self {
        Self { schema, doc, model: ModelSnapshot::default(), notes: Vec::new(), ids: BTreeMap::new(), site_world: BTreeMap::new(), building_world: BTreeMap::new(), storey_ids: BTreeMap::new(), levels: BTreeMap::new(), material_ids: BTreeMap::new(), type_ids: BTreeMap::new() }
    }

    /// 📝️ Records that an entity could not be imported.
    pub fn skip(&mut self, entity: &str, label: &str, reason: &str) {
        self.notes.push(format!("{entity} {label}: {reason}"));
    }

    /// 🪜️ The storey id that contains the product `ifc`.
    pub fn storey_of(&self, ifc: u64) -> Option<String> {
        self.doc.index.contained.get(&ifc).and_then(|structure| self.storey_ids.get(structure)).cloned()
    }

    /// 📐️ The placement of a product relative to the building of `storey`.
    pub fn in_building(&self, placement: &Part21Value, storey: &str) -> Rigid {
        let building = self.model.storeys.get(storey).and_then(|row| self.building_world.get(&row.building));
        let world = self.doc.world(placement);
        building.map_or(world, |building| world.relative_to(building))
    }

    /// 🧩️ Marks every part below `whole` (nested aggregation included) as imported with it, so the unsupported-class note leaves them out.
    pub fn claim_parts(&mut self, whole: u64, id: &str) {
        let mut pending = vec![whole];
        while let Some(at) = pending.pop() {
            for part in self.doc.index.parts.get(&at).cloned().unwrap_or_default() {
                self.ids.insert(part, format!("{id}:part"));
                pending.push(part);
            }
        }
    }

    /// 🔑️ A model id that is not used yet: `base` or `base-2`, `base-3`, …
    pub fn unused(base: &str, taken: impl Fn(&str) -> bool) -> String {
        std::iter::once(base.to_string()).chain((2..).map(|n| format!("{base}-{n}"))).find(|candidate| !taken(candidate)).unwrap_or_else(|| base.to_string())
    }

    /// 🔤️ A lower-case id fragment of a name.
    pub fn slug(name: &str) -> String {
        let slug: String = name.chars().map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' }).collect();
        let trimmed = slug.split('-').filter(|part| !part.is_empty()).collect::<Vec<_>>().join("-");
        if trimmed.is_empty() {
            "unnamed".to_string()
        } else {
            trimmed
        }
    }
}

//#region 🔖️Entry
/// 📥️ Imports the Part-21 document of a file in `schema` into a model plus one note per entity class or item that was not imported.
pub fn import_document(schema: Schema, document: &semio_s_artifact_stdio_ifc::part21::Part21Document) -> Result<(ModelSnapshot, Vec<String>), String> {
    let mut import = Import::new(schema, Doc::new(document));
    spatial::read(&mut import);
    import.levels = inference::probe::<{ kinds::LEVELS }, _>(None, &import.model, |inferred| inferred.storey_levels.clone()).map_err(|error| error.to_string())?;
    data::read_materials(&mut import);
    data::read_types(&mut import);
    walls::read(&mut import);
    elements::read(&mut import);
    ceilings::read(&mut import);
    zoning::read(&mut import);
    options::read(&mut import);
    ramps::read(&mut import);
    curtain::read(&mut import);
    horizontal::read(&mut import);
    circulation::read(&mut import);
    wall_sweeps::read(&mut import);
    attach::read(&mut import);
    components::read(&mut import);
    mep::read(&mut import);
    energy::read_conditions(&mut import);
    data::read_templates(&mut import);
    data::read_attached(&mut import);
    spatial::report_unsupported(&mut import);
    Ok((import.model, import.notes))
}

/// 📥️ Imports IFC 2x3 file bytes into a model plus one note per entity class or item that was not imported.
pub fn import_ifc2x3(bytes: &[u8]) -> Result<(ModelSnapshot, Vec<String>), String> {
    import_document(Schema::Ifc2x3, &codec::decode_document(bytes)?)
}

/// 📥️ Imports IFC4 file bytes into a model plus one note per entity class or item that was not imported.
pub fn import_ifc4(bytes: &[u8]) -> Result<(ModelSnapshot, Vec<String>), String> {
    import_document(Schema::Ifc4, &codec::decode_ifc4(bytes)?)
}
//#endregion 🔖️Entry

//#region 🔖️Deserializer
fn head_of(payload: &IoPayload) -> String {
    let head = match payload {
        IoPayload::Text(text) => text.as_bytes().get(..600).map_or(text.as_bytes(), |head| head).to_vec(),
        IoPayload::Binary(bytes) => bytes.get(..600).map_or(bytes.as_slice(), |head| head).to_vec(),
    };
    String::from_utf8_lossy(&head).to_string()
}

fn bytes_of(payload: &IoPayload) -> &[u8] {
    match payload {
        IoPayload::Text(text) => text.as_bytes(),
        IoPayload::Binary(bytes) => bytes.as_slice(),
    }
}

fn diagnostics_of(notes: Vec<String>) -> Vec<semio_framework_diagnostic::Diagnostic> {
    notes
        .into_iter()
        .map(|note| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new("bim.ifc.import.skipped"), severity: semio_framework_diagnostic::Severity::Warning, span: Default::default(), message: note, expected: None, scope: Default::default() })
        .collect()
}

fn refused(name: &str, message: String) -> IoError {
    IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("{name}: {message}")))
}

/// 🏗️ The IFC 2x3 deserializer of the BIM model.
pub struct Ifc2x3IntoModel;

impl Deserializer<ModelSnapshot> for Ifc2x3IntoModel {
    const FROM: Dialect = IFC_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Semantic;

    async fn sniff(payload: &IoPayload) -> Confidence {
        let head = head_of(payload);
        if head.starts_with("ISO-10303-21") && head.contains("IFC2X3") {
            Confidence::High
        } else {
            Confidence::None
        }
    }

    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<ModelSnapshot> {
        let _ = control;
        let (model, notes) = import_ifc2x3(bytes_of(payload)).map_err(|message| refused("Ifc2x3IntoModel", message))?;
        Ok(IoOutcome { value: model, diagnostics: diagnostics_of(notes) })
    }
}

/// 🏗️ The IFC4 deserializer of the BIM model.
pub struct Ifc4IntoModel;

impl Deserializer<ModelSnapshot> for Ifc4IntoModel {
    const FROM: Dialect = IFC4_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Semantic;

    async fn sniff(payload: &IoPayload) -> Confidence {
        let head = head_of(payload);
        if head.starts_with("ISO-10303-21") && head.contains("'IFC4'") {
            Confidence::High
        } else {
            Confidence::None
        }
    }

    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<ModelSnapshot> {
        let _ = control;
        let (model, notes) = import_ifc4(bytes_of(payload)).map_err(|message| refused("Ifc4IntoModel", message))?;
        Ok(IoOutcome { value: model, diagnostics: diagnostics_of(notes) })
    }
}
//#endregion 🔖️Deserializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
