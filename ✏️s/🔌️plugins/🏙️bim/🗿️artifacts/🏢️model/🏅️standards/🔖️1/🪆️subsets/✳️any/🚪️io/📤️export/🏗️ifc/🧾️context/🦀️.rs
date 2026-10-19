//! 🧾️ The context of an IFC export: the records the families write into (`Export`, `Staged`, `Links`), the references of the storeys and buildings written so far and the base quantities. Every family node reads it; none of them reaches back into the exporter root.

use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::{opt_text, rf, unset, Ifc, Schema, V};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::ElementQuantity;
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::ModelInference;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Phase;
use std::collections::BTreeMap;

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
        let phase = semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::phase_of(self.model, id).filter(|phase| *phase != Phase::New);
        if let Some(phase) = phase {
            self.links.authoring.push((product, vec![("Phase", writer::label(&format!("{phase:?}")))]));
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
