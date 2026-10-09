//! 🗂️ The rows of a schedule: which elements a schedule covers ([`candidates`]: the category, the storey scope and the phase scope, all decided by authored values), the authored facts the cells read
//! ([`AuthoredView`], built once per schedule and the only way the table reads the snapshot) and the cell of one key for one source row ([`Reader::cell`]).

use super::super::super::effective_properties::EffectiveProperties;
use super::super::super::finishes::FinishQuantity;
use super::super::super::quantities::{ElementQuantity, LayerQuantity};
use super::ScheduleCell;
use crate::{DoorLeaves, ModelSnapshot, OpeningKind, Phase, PropertyValue, Schedule, ScheduleCategory, ScheduleField, ScheduleKey, Swing};
use std::collections::BTreeMap;

//#region 🔖️Scope
fn phase_token(phase: Phase) -> &'static str {
    match phase {
        Phase::Existing => "existing",
        Phase::New => "new",
        Phase::Demolished => "demolished",
        Phase::Temporary => "temporary",
    }
}

/// 🪜️ The id of the storey an element stands on; an opening stands on the storey of its host.
pub fn storey_of<'a>(snapshot: &'a ModelSnapshot, id: &str) -> Option<&'a String> {
    snapshot
        .walls
        .get(id)
        .map(|row| &row.storey)
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.columns.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.beams.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.slabs.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.roofs.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.stairs.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.railings.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.spaces.get(id).map(|row| &row.storey))
        .or_else(|| snapshot.openings.get(id).and_then(|opening| storey_of(snapshot, &opening.host)))
}

/// 🕰️ The phase of an element: its own, an opening's is its host's, an element without a phase counts as new.
pub fn phase_of(snapshot: &ModelSnapshot, id: &str) -> Phase {
    snapshot
        .walls
        .get(id)
        .map(|row| row.phase)
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| row.phase))
        .or_else(|| snapshot.columns.get(id).map(|row| row.phase))
        .or_else(|| snapshot.beams.get(id).map(|row| row.phase))
        .or_else(|| snapshot.slabs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.roofs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.phase))
        .or_else(|| snapshot.railings.get(id).map(|row| row.phase))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.phase))
        .or_else(|| snapshot.openings.get(id).map(|opening| phase_of(snapshot, &opening.host)))
        .unwrap_or(Phase::New)
}

fn ids_of(snapshot: &ModelSnapshot, category: ScheduleCategory) -> Vec<String> {
    let openings = |wanted: fn(&OpeningKind) -> bool| -> Vec<String> { snapshot.openings.iter().filter(|(_, row)| wanted(&row.kind)).map(|(id, _)| id.clone()).collect() };
    match category {
        ScheduleCategory::Wall => snapshot.walls.keys().cloned().collect(),
        ScheduleCategory::CurtainWall => snapshot.curtain_walls.keys().cloned().collect(),
        ScheduleCategory::Slab => snapshot.slabs.keys().cloned().collect(),
        ScheduleCategory::Roof => snapshot.roofs.keys().cloned().collect(),
        ScheduleCategory::Column => snapshot.columns.keys().cloned().collect(),
        ScheduleCategory::Beam => snapshot.beams.keys().cloned().collect(),
        ScheduleCategory::Window => openings(|kind| matches!(kind, OpeningKind::Window { .. })),
        ScheduleCategory::Door => openings(|kind| matches!(kind, OpeningKind::Door { .. })),
        ScheduleCategory::Void => openings(|kind| matches!(kind, OpeningKind::Void { .. })),
        ScheduleCategory::Stair => snapshot.stairs.keys().cloned().collect(),
        ScheduleCategory::Railing => snapshot.railings.keys().cloned().collect(),
        ScheduleCategory::Space | ScheduleCategory::Finish => snapshot.spaces.keys().cloned().collect(),
        ScheduleCategory::Material => {
            let mut ids: Vec<String> = snapshot.walls.keys().chain(snapshot.curtain_walls.keys()).chain(snapshot.columns.keys()).chain(snapshot.beams.keys()).chain(snapshot.slabs.keys()).chain(snapshot.roofs.keys()).chain(snapshot.openings.keys()).chain(snapshot.stairs.keys()).chain(snapshot.railings.keys()).chain(snapshot.spaces.keys()).cloned().collect();
            ids.sort();
            ids
        }
    }
}

/// 🧭️ The ids of the elements a schedule covers, in id order: the elements of its category on the storeys and in the phases of its scope (an empty scope covers all).
pub fn candidates(snapshot: &ModelSnapshot, schedule: &Schedule) -> Vec<String> {
    ids_of(snapshot, schedule.category)
        .into_iter()
        .filter(|id| schedule.storeys.is_empty() || storey_of(snapshot, id).is_some_and(|storey| schedule.storeys.contains(storey)))
        .filter(|id| schedule.phases.is_empty() || schedule.phases.contains(&phase_of(snapshot, id)))
        .collect()
}
//#endregion 🔖️Scope

//#region 🔖️Facts
/// 🧾️ The authored facts of one element a schedule can show: every cell is already a [`ScheduleCell`], so the table never reads the snapshot again.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Facts {
    pub name: ScheduleCell,
    pub storey: ScheduleCell,
    pub level: ScheduleCell,
    pub type_name: ScheduleCell,
    pub phase: ScheduleCell,
    pub host: ScheduleCell,
    pub number: ScheduleCell,
    pub usage: ScheduleCell,
    pub swing: ScheduleCell,
    pub leaves: ScheduleCell,
    pub panes: ScheduleCell,
    pub properties: Vec<ScheduleCell>,
}

/// 🧾️ The authored facts of the candidate elements of one schedule and the names of the materials: everything besides the quantities that decides a cell.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct AuthoredView {
    pub elements: BTreeMap<String, Facts>,
    pub materials: BTreeMap<String, String>,
}

/// 🔑️ The property keys a schedule reads, in order of first appearance across columns, sort, filter and grouping.
pub fn property_keys(schedule: &Schedule) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = Vec::new();
    let keys = schedule.columns.iter().map(|column| &column.key).chain(schedule.sort.iter().map(|sort| &sort.key)).chain(schedule.filter.iter().map(|filter| &filter.key)).chain(schedule.group.iter().map(|group| &group.key));
    for key in keys {
        if let ScheduleKey::Property { set, name } = key {
            if !found.iter().any(|(known_set, known_name)| known_set == set && known_name == name) {
                found.push((set.clone(), name.clone()));
            }
        }
    }
    found
}

fn name_of(snapshot: &ModelSnapshot, id: &str) -> String {
    snapshot
        .walls
        .get(id)
        .map(|row| row.name.clone())
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.columns.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.beams.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.slabs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.roofs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.openings.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.railings.get(id).map(|row| row.name.clone()))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.name.clone()))
        .unwrap_or_default()
}

fn or_id(name: Option<&String>, id: &str) -> String {
    name.cloned().unwrap_or_else(|| id.to_string())
}

fn type_name_of(snapshot: &ModelSnapshot, id: &str) -> String {
    if let Some(wall) = snapshot.walls.get(id) {
        return or_id(snapshot.wall_types.get(&wall.wall_type).map(|row| &row.name), &wall.wall_type);
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return or_id(snapshot.slab_types.get(&slab.slab_type).map(|row| &row.name), &slab.slab_type);
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return or_id(snapshot.roof_types.get(&roof.roof_type).map(|row| &row.name), &roof.roof_type);
    }
    if let Some(column) = snapshot.columns.get(id) {
        return or_id(snapshot.column_types.get(&column.column_type).map(|row| &row.name), &column.column_type);
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return or_id(snapshot.beam_types.get(&beam.beam_type).map(|row| &row.name), &beam.beam_type);
    }
    match snapshot.openings.get(id).map(|opening| &opening.kind) {
        Some(OpeningKind::Window { window_type }) => or_id(snapshot.window_types.get(window_type).map(|row| &row.name), window_type),
        Some(OpeningKind::Door { door_type }) => or_id(snapshot.door_types.get(door_type).map(|row| &row.name), door_type),
        _ => String::new(),
    }
}

fn host_name_of(snapshot: &ModelSnapshot, id: &str) -> String {
    snapshot.openings.get(id).map(|opening| snapshot.walls.get(&opening.host).map(|row| row.name.clone()).or_else(|| snapshot.curtain_walls.get(&opening.host).map(|row| row.name.clone())).unwrap_or_else(|| opening.host.clone())).unwrap_or_default()
}

fn hand_token(swing: Swing, flipped: bool) -> &'static str {
    match (swing, flipped) {
        (Swing::Left, false) | (Swing::Right, true) => "left",
        (Swing::Right, false) | (Swing::Left, true) => "right",
    }
}

/// 🏷️ The cell of an effective property: the value the element has by its own, by its type or by the default of a template; empty where it has none.
pub fn effective_cell(effective: &EffectiveProperties, set: &str, name: &str) -> ScheduleCell {
    value_cell(effective.get(set, name).map(|row| &row.value))
}

/// 🏷️ The cells of `keys` for the effective properties of every element of `view` that has some: they replace the authored cells, which see the element's own properties only.
pub fn with_effective(view: &mut AuthoredView, schedule: &Schedule, effective: &BTreeMap<&str, &EffectiveProperties>) {
    let keys = property_keys(schedule);
    for (id, facts) in view.elements.iter_mut() {
        if let Some(properties) = effective.get(id.as_str()) {
            facts.properties = keys.iter().map(|(set, name)| effective_cell(properties, set, name)).collect();
        }
    }
}

fn property_cell(snapshot: &ModelSnapshot, id: &str, set: &str, name: &str) -> ScheduleCell {
    value_cell(snapshot.properties.get(id).and_then(|sets| sets.get(set)).and_then(|group| group.get(name)))
}

fn value_cell(value: Option<&PropertyValue>) -> ScheduleCell {
    match value {
        None => ScheduleCell::Empty,
        Some(PropertyValue::Text { value }) => ScheduleCell::text(value.clone()),
        Some(PropertyValue::Boolean { value }) => ScheduleCell::text(value.to_string()),
        Some(PropertyValue::Integer { value }) => ScheduleCell::number(f64::from(*value)),
        Some(PropertyValue::Real { value } | PropertyValue::Length { value } | PropertyValue::Area { value } | PropertyValue::Volume { value } | PropertyValue::Angle { value }) => ScheduleCell::number(*value),
    }
}

impl Facts {
    /// 🧾️ The facts of element `id`, with the values of the property keys `properties` in that order.
    pub fn of(snapshot: &ModelSnapshot, id: &str, properties: &[(String, String)]) -> Self {
        let storey = storey_of(snapshot, id).and_then(|storey| snapshot.storeys.get(storey));
        let opening = snapshot.openings.get(id);
        let door = opening.and_then(|opening| match &opening.kind {
            OpeningKind::Door { door_type } => snapshot.door_types.get(door_type).map(|kind| (opening, kind)),
            _ => None,
        });
        let window = opening.and_then(|opening| match &opening.kind {
            OpeningKind::Window { window_type } => snapshot.window_types.get(window_type),
            _ => None,
        });
        let space = snapshot.spaces.get(id);
        Self {
            name: ScheduleCell::text(name_of(snapshot, id)),
            storey: ScheduleCell::text(storey.map(|row| row.name.clone()).unwrap_or_default()),
            level: storey.map_or(ScheduleCell::Empty, |row| ScheduleCell::number(f64::from(row.level))),
            type_name: ScheduleCell::text(type_name_of(snapshot, id)),
            phase: ScheduleCell::text(phase_token(phase_of(snapshot, id))),
            host: ScheduleCell::text(host_name_of(snapshot, id)),
            number: ScheduleCell::text(space.map(|row| row.number.clone()).unwrap_or_default()),
            usage: ScheduleCell::text(space.map(|row| row.usage.clone()).unwrap_or_default()),
            swing: door.map_or(ScheduleCell::Empty, |(opening, kind)| ScheduleCell::text(hand_token(kind.swing, opening.flip_hand))),
            leaves: door.map_or(ScheduleCell::Empty, |(_, kind)| ScheduleCell::text(if kind.leaves == DoorLeaves::Double { "double" } else { "single" })),
            panes: window.map_or(ScheduleCell::Empty, |kind| ScheduleCell::number(f64::from(kind.panes))),
            properties: properties.iter().map(|(set, name)| property_cell(snapshot, id, set, name)).collect(),
        }
    }
}

impl AuthoredView {
    /// 🧾️ The view of `schedule` over the elements `ids`: their facts, and the material names when the schedule shows materials.
    pub fn of(snapshot: &ModelSnapshot, schedule: &Schedule, ids: &[String]) -> Self {
        let properties = property_keys(schedule);
        let shows_materials = schedule.category.is_material() || schedule.category.is_finish() || schedule.columns.iter().map(|column| &column.key).chain(schedule.sort.iter().map(|sort| &sort.key)).chain(schedule.filter.iter().map(|filter| &filter.key)).chain(schedule.group.iter().map(|group| &group.key)).any(|key| *key == ScheduleKey::field(ScheduleField::Material));
        Self {
            elements: ids.iter().map(|id| (id.clone(), Facts::of(snapshot, id, &properties))).collect(),
            materials: if shows_materials { snapshot.materials.iter().map(|(id, row)| (id.clone(), row.name.clone())).collect() } else { BTreeMap::new() },
        }
    }
}
//#endregion 🔖️Facts

//#region 🔖️Cells
/// 🧱️ One source row of a table: an element with its quantities, for the material category the layer or material run the row stands for and for the finish category the finished surface.
#[derive(Clone, Copy, Debug)]
pub struct Source<'a> {
    pub id: &'a str,
    pub quantity: &'a ElementQuantity,
    pub layer: Option<&'a LayerQuantity>,
    pub finish: Option<&'a FinishQuantity>,
}

/// 🧱️ The source rows of a schedule from the quantities of its candidates: one per element, one per layer of each element for the material category, one per finished surface of each room for the finish category.
pub fn sources<'a>(schedule: &Schedule, quantities: &BTreeMap<&'a str, &'a ElementQuantity>) -> Vec<Source<'a>> {
    quantities
        .iter()
        .flat_map(|(id, quantity)| -> Vec<Source<'a>> {
            if schedule.category.is_material() {
                quantity.layers.iter().filter(|layer| !layer.material.is_empty()).map(|layer| Source { id, quantity, layer: Some(layer), finish: None }).collect()
            } else if schedule.category.is_finish() {
                quantity.finishes.iter().map(|finish| Source { id, quantity, layer: None, finish: Some(finish) }).collect()
            } else {
                vec![Source { id, quantity, layer: None, finish: None }]
            }
        })
        .collect()
}

/// 👁️ Reads cells: the property order of the schedule and the authored view.
pub struct Reader<'a> {
    view: &'a AuthoredView,
    properties: Vec<(String, String)>,
}

impl<'a> Reader<'a> {
    /// 👁️ A reader of `schedule` over `view`.
    pub fn new(schedule: &Schedule, view: &'a AuthoredView) -> Self {
        Self { view, properties: property_keys(schedule) }
    }

    fn materials(&self, source: &Source<'_>) -> ScheduleCell {
        let name = |material: &str| self.view.materials.get(material).cloned().unwrap_or_else(|| material.to_string());
        match (source.layer, source.finish) {
            (Some(layer), _) => ScheduleCell::text(name(&layer.material)),
            (None, Some(finish)) => ScheduleCell::text(name(&finish.material)),
            (None, None) => {
                let mut names: Vec<String> = Vec::new();
                for layer in source.quantity.layers.iter().filter(|layer| !layer.material.is_empty()) {
                    let shown = name(&layer.material);
                    if !names.contains(&shown) {
                        names.push(shown);
                    }
                }
                ScheduleCell::text(names.join(", "))
            }
        }
    }

    fn field(&self, source: &Source<'_>, field: ScheduleField) -> ScheduleCell {
        let quantity = source.quantity;
        let facts = self.view.elements.get(source.id);
        let fact = |read: fn(&Facts) -> &ScheduleCell| facts.map_or(ScheduleCell::Empty, |row| read(row).clone());
        let layer = |measure: fn(&LayerQuantity) -> f64| source.layer.map_or(ScheduleCell::Empty, |row| ScheduleCell::number(measure(row)));
        let finish = |read: fn(&FinishQuantity) -> ScheduleCell| source.finish.map_or(ScheduleCell::Empty, read);
        match field {
            ScheduleField::Id => ScheduleCell::text(source.id),
            ScheduleField::Name => fact(|row| &row.name),
            ScheduleField::Kind => ScheduleCell::text(quantity.kind.key()),
            ScheduleField::Storey => fact(|row| &row.storey),
            ScheduleField::Level => fact(|row| &row.level),
            ScheduleField::Type => fact(|row| &row.type_name),
            ScheduleField::Phase => fact(|row| &row.phase),
            ScheduleField::Material => self.materials(source),
            ScheduleField::Host => fact(|row| &row.host),
            ScheduleField::Number => fact(|row| &row.number),
            ScheduleField::Usage => fact(|row| &row.usage),
            ScheduleField::Surface => finish(|row| ScheduleCell::text(row.surface.key())),
            ScheduleField::Swing => fact(|row| &row.swing),
            ScheduleField::Leaves => fact(|row| &row.leaves),
            ScheduleField::Panes => fact(|row| &row.panes),
            ScheduleField::Count => ScheduleCell::number(f64::from(quantity.count)),
            ScheduleField::Length => ScheduleCell::number(quantity.length),
            ScheduleField::Width => ScheduleCell::number(quantity.width),
            ScheduleField::Height => ScheduleCell::number(quantity.height),
            ScheduleField::Perimeter => ScheduleCell::number(quantity.perimeter),
            ScheduleField::GrossSideArea => ScheduleCell::number(quantity.gross_side_area),
            ScheduleField::OpeningArea => ScheduleCell::number(quantity.opening_area),
            ScheduleField::NetSideArea => ScheduleCell::number(quantity.net_side_area),
            ScheduleField::GrossArea => ScheduleCell::number(quantity.gross_area),
            ScheduleField::NetArea => ScheduleCell::number(quantity.net_area),
            ScheduleField::SurfaceArea => ScheduleCell::number(quantity.surface_area),
            ScheduleField::GrossVolume => ScheduleCell::number(quantity.gross_volume),
            ScheduleField::NetVolume => ScheduleCell::number(quantity.net_volume),
            ScheduleField::Mass => ScheduleCell::number(quantity.mass),
            ScheduleField::Risers => ScheduleCell::number(f64::from(quantity.risers)),
            ScheduleField::Thickness => layer(|row| row.thickness),
            ScheduleField::LayerArea => layer(|row| row.area),
            ScheduleField::LayerVolume => layer(|row| row.volume),
            ScheduleField::LayerMass => layer(|row| row.mass),
            ScheduleField::FinishArea => finish(|row| ScheduleCell::number(row.area)),
        }
    }

    /// 🧾️ The cell of `key` for one source row.
    pub fn cell(&self, source: &Source<'_>, key: &ScheduleKey) -> ScheduleCell {
        match key {
            ScheduleKey::Field { field } => self.field(source, *field),
            ScheduleKey::Property { set, name } => self
                .properties
                .iter()
                .position(|(known_set, known_name)| known_set == set && known_name == name)
                .and_then(|index| self.view.elements.get(source.id).and_then(|facts| facts.properties.get(index)))
                .cloned()
                .unwrap_or_default(),
        }
    }
}
//#endregion 🔖️Cells
