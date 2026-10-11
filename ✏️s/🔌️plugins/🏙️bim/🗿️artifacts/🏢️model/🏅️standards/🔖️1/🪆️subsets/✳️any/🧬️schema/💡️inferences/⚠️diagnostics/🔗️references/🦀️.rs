//! 🔗️ Referential integrity: every id an element names must exist. Findings of a storey cover its elements and the openings of its hosts; the
//! model findings cover what belongs to no storey: spatial structure, elements on missing storeys, openings of missing hosts, type materials,
//! orphaned properties and ids used twice.

use super::{Diagnostic, DiagnosticCode};
use crate::standards::v1::subsets::any::schema::authored::references;
use crate::{ModelSnapshot, OpeningKind, TopConstraint};
use std::collections::{BTreeMap, BTreeSet};

fn top_storey(top: &TopConstraint) -> Option<&str> {
    if let TopConstraint::Storey { storey, .. } = top {
        Some(storey)
    } else {
        None
    }
}

fn dangling_top(snapshot: &ModelSnapshot, found: &mut Vec<Diagnostic>, id: &str, storey: &str, top: &TopConstraint) {
    if let Some(target) = top_storey(top).filter(|target| !snapshot.storeys.contains_key(*target)) {
        found.push(Diagnostic::new(DiagnosticCode::RefTopStorey, &[id]).on(storey).lacking(target));
    }
}

/// 🪜️ The dangling references of the elements of one storey and of the openings in its hosts.
pub fn storey(snapshot: &ModelSnapshot, storey: &str) -> Vec<Diagnostic> {
    use DiagnosticCode::*;
    let mut found = Vec::new();
    let mut lacking = |code: DiagnosticCode, id: &str, missing: &str| found.push(Diagnostic::new(code, &[id]).on(storey).lacking(missing));
    let mut tops: Vec<(&String, &TopConstraint)> = Vec::new();
    for (id, wall) in snapshot.walls.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.wall_types.contains_key(&wall.wall_type) {
            lacking(RefWallType, id, &wall.wall_type);
        }
        tops.push((id, &wall.top));
    }
    for (id, curtain) in snapshot.curtain_walls.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.curtain_wall_types.contains_key(&curtain.curtain_wall_type) {
            lacking(RefCurtainWallType, id, &curtain.curtain_wall_type);
        }
        tops.push((id, &curtain.top));
    }
    for (id, column) in snapshot.columns.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.column_types.contains_key(&column.column_type) {
            lacking(RefColumnType, id, &column.column_type);
        }
        tops.push((id, &column.top));
    }
    for (id, beam) in snapshot.beams.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.beam_types.contains_key(&beam.beam_type) {
            lacking(RefBeamType, id, &beam.beam_type);
        }
    }
    for (id, slab) in snapshot.slabs.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.slab_types.contains_key(&slab.slab_type) {
            lacking(RefSlabType, id, &slab.slab_type);
        }
    }
    for (id, ceiling) in snapshot.ceilings.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.ceiling_types.contains_key(&ceiling.ceiling_type) {
            lacking(RefCeilingType, id, &ceiling.ceiling_type);
        }
    }
    for (id, roof) in snapshot.roofs.iter().filter(|(_, row)| row.storey == storey) {
        if !snapshot.roof_types.contains_key(&roof.roof_type) {
            lacking(RefRoofType, id, &roof.roof_type);
        }
    }
    for (id, stair) in snapshot.stairs.iter().filter(|(_, row)| row.storey == storey) {
        tops.push((id, &stair.top));
    }
    for (id, ramp) in snapshot.ramps.iter().filter(|(_, row)| row.storey == storey) {
        tops.push((id, &ramp.top));
        if !snapshot.materials.contains_key(&ramp.material) && !ramp.material.is_empty() {
            lacking(RefTypeMaterial, id, &ramp.material);
        }
    }
    for (id, railing) in snapshot.railings.iter().filter(|(_, row)| row.storey == storey) {
        if let Some(host) = railing.host.as_ref().filter(|host| !(snapshot.stairs.contains_key(&host.element) || snapshot.ramps.contains_key(&host.element) || snapshot.slabs.contains_key(&host.element))) {
            lacking(RefRailingHost, id, &host.element);
        }
    }
    for (id, opening) in snapshot.openings.iter().filter(|(_, row)| snapshot.walls.get(&row.host).map(|wall| &wall.storey).or_else(|| snapshot.curtain_walls.get(&row.host).map(|curtain| &curtain.storey)).is_some_and(|host| host == storey)) {
        match &opening.kind {
            OpeningKind::Window { window_type } if !snapshot.window_types.contains_key(window_type) => lacking(RefWindowType, id, window_type),
            OpeningKind::Door { door_type } if !snapshot.door_types.contains_key(door_type) => lacking(RefDoorType, id, door_type),
            _ => {}
        }
    }
    for (id, top) in tops {
        dangling_top(snapshot, &mut found, id, storey, top);
    }
    found
}

/// 🔑️ What the findings of a storey read of the hosts of its railings: whether the host exists and, for a slab, its boundary (its edges are what a hosted railing follows).
pub fn host_dependency(snapshot: &ModelSnapshot, storey: &str) -> semio_framework_value::DslValue {
    let exists = |element: &String| snapshot.stairs.contains_key(element) || snapshot.ramps.contains_key(element) || snapshot.slabs.contains_key(element);
    semio_framework_value::DslValue::object(snapshot.railings.iter().filter(|(_, row)| row.storey == storey).filter_map(|(id, row)| {
        let host = row.host.as_ref()?;
        let value = semio_framework_value::DslValue::object([("exists".to_string(), semio_framework_value::ToValue::to_value(&exists(&host.element))), ("slab".to_string(), semio_framework_value::ToValue::to_value(&snapshot.slabs.get(&host.element).map(|slab| slab.boundary.clone())))]);
        Some((id.clone(), value))
    }))
}

//#region 🔖️View
/// 🧱️ An element and the storey it stands on.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Placement {
    pub id: String,
    pub storey: String,
}

/// 🎨️ An element or type and the material it names.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct MaterialUse {
    pub id: String,
    pub material: String,
}

/// 🔗️ A wall and the roof, slab or ceiling its top or base is attached to.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct Attach {
    pub wall: String,
    pub target: String,
}

/// 🪟️ A curtain panel (of a type or of an override) and the door type, window type or material it names: `kind` is `door`, `window` or `material`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct PanelUse {
    pub id: String,
    pub kind: String,
    pub reference: String,
}

/// 🔗️ The reference structure of a model: the ids of everything and the ids each names, and no geometry. It is all `model` reads, so it is also the dependency of the model scope:
/// moving a wall or changing a height never invalidates it, creating, deleting or re-pointing an element does.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
pub struct ReferenceView {
    pub sites: Vec<String>,
    pub sheets: Vec<String>,
    pub views: Vec<String>,
    pub viewport_sheets: BTreeMap<String, String>,
    pub viewport_views: BTreeMap<String, String>,
    pub revision_sheets: BTreeMap<String, String>,
    pub building_sites: BTreeMap<String, String>,
    pub storey_buildings: BTreeMap<String, String>,
    pub grid_buildings: BTreeMap<String, String>,
    pub placed: Vec<Placement>,
    pub openings: BTreeMap<String, String>,
    pub hosts: Vec<String>,
    pub layered: BTreeMap<String, Vec<String>>,
    pub named: Vec<MaterialUse>,
    pub materials: Vec<String>,
    pub types: Vec<String>,
    pub attached: Vec<String>,
    pub doors: Vec<String>,
    pub windows: Vec<String>,
    pub curtains: Vec<String>,
    pub overrides: BTreeMap<String, String>,
    pub panels: Vec<PanelUse>,
    pub counts: BTreeMap<String, u32>,
    pub sweep_hosts: BTreeMap<String, String>,
    pub attaches: Vec<Attach>,
    pub attachable: Vec<String>,
}

impl ReferenceView {
    /// 🔗️ The reference structure of a snapshot.
    pub fn of(snapshot: &ModelSnapshot) -> Self {
        let placed = |rows: Vec<(&String, &String)>| rows.into_iter().map(|(id, storey)| Placement { id: id.clone(), storey: storey.clone() }).collect::<Vec<_>>();
        let mut stray = placed(snapshot.walls.iter().map(|(id, row)| (id, &row.storey)).collect());
        stray.extend(placed(snapshot.curtain_walls.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.columns.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.beams.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.slabs.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.ceilings.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.roofs.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.stairs.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.railings.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.ramps.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.components.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.mep_elements.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.spaces.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.dimensions.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.tags.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.text_notes.iter().map(|(id, row)| (id, &row.storey)).collect()));
        stray.extend(placed(snapshot.leaders.iter().map(|(id, row)| (id, &row.storey)).collect()));
        let layers = |rows: Vec<(&String, &Vec<crate::Layer>)>| rows.into_iter().map(|(id, layers)| (id.clone(), layers.iter().map(|layer| layer.material.clone()).collect::<Vec<_>>())).collect::<Vec<_>>();
        let mut layered: BTreeMap<String, Vec<String>> = BTreeMap::new();
        layered.extend(layers(snapshot.wall_types.iter().map(|(id, kind)| (id, &kind.layers)).collect()));
        layered.extend(layers(snapshot.slab_types.iter().map(|(id, kind)| (id, &kind.layers)).collect()));
        layered.extend(layers(snapshot.ceiling_types.iter().map(|(id, kind)| (id, &kind.layers)).collect()));
        layered.extend(layers(snapshot.roof_types.iter().map(|(id, kind)| (id, &kind.layers)).collect()));
        let use_of = |id: &String, material: &String| MaterialUse { id: id.clone(), material: material.clone() };
        let mut named: Vec<MaterialUse> = snapshot.column_types.iter().map(|(id, kind)| use_of(id, &kind.material)).collect();
        named.extend(snapshot.beam_types.iter().map(|(id, kind)| use_of(id, &kind.material)));
        named.extend(snapshot.window_types.iter().map(|(id, kind)| use_of(id, &kind.material)));
        named.extend(snapshot.door_types.iter().map(|(id, kind)| use_of(id, &kind.material)));
        for (id, row) in &snapshot.curtain_wall_types {
            named.push(use_of(id, &row.panel_material));
            named.push(use_of(id, &row.mullion_material));
        }
        let panel_use = |id: &String, panel: &crate::CurtainPanel| match panel {
            crate::CurtainPanel::Solid { material } => Some(PanelUse { id: id.clone(), kind: "material".into(), reference: material.clone() }),
            crate::CurtainPanel::Door { door_type } => Some(PanelUse { id: id.clone(), kind: "door".into(), reference: door_type.clone() }),
            crate::CurtainPanel::Window { window_type } => Some(PanelUse { id: id.clone(), kind: "window".into(), reference: window_type.clone() }),
            crate::CurtainPanel::Glass | crate::CurtainPanel::Empty => None,
        };
        let panels: Vec<PanelUse> = snapshot.curtain_wall_types.iter().filter_map(|(id, row)| panel_use(id, &row.panel)).chain(snapshot.curtain_panel_overrides.iter().filter_map(|(id, row)| panel_use(id, &row.panel))).collect();
        named.extend(snapshot.railings.iter().map(|(id, row)| use_of(id, &row.material)));
        named.extend(snapshot.ramps.iter().map(|(id, row)| use_of(id, &row.material)));
        named.extend(snapshot.wall_sweeps.iter().map(|(id, row)| use_of(id, &row.material)));
        named.extend(snapshot.openings.iter().filter_map(|(id, row)| row.reveal_material.as_ref().map(|material| use_of(id, material))));
        let mut counts: BTreeMap<String, u32> = BTreeMap::new();
        let ids = snapshot.sites.keys().chain(snapshot.buildings.keys()).chain(snapshot.storeys.keys()).chain(snapshot.grids.keys()).chain(snapshot.walls.keys()).chain(snapshot.curtain_walls.keys()).chain(snapshot.columns.keys()).chain(snapshot.beams.keys()).chain(snapshot.slabs.keys()).chain(snapshot.ceilings.keys());
        ids.chain(snapshot.roofs.keys()).chain(snapshot.openings.keys()).chain(snapshot.stairs.keys()).chain(snapshot.railings.keys()).chain(snapshot.ramps.keys()).chain(snapshot.spaces.keys()).chain(snapshot.wall_sweeps.keys()).chain(snapshot.components.keys()).chain(snapshot.mep_elements.keys()).for_each(|id| *counts.entry(id.clone()).or_insert(0) += 1);
        snapshot.curtain_panel_overrides.keys().for_each(|id| *counts.entry(id.clone()).or_insert(0) += 1);
        snapshot.dimensions.keys().chain(snapshot.tags.keys()).chain(snapshot.text_notes.keys()).chain(snapshot.leaders.keys()).chain(snapshot.annotation_styles.keys()).chain(snapshot.sheets.keys()).chain(snapshot.viewports.keys()).chain(snapshot.sheet_revisions.keys()).chain(snapshot.zones.keys()).chain(snapshot.area_schemes.keys()).chain(snapshot.views.keys()).for_each(|id| *counts.entry(id.clone()).or_insert(0) += 1);
        Self {
            sites: snapshot.sites.keys().cloned().collect(),
            sheets: snapshot.sheets.keys().cloned().collect(),
            views: snapshot.views.keys().cloned().collect(),
            viewport_sheets: snapshot.viewports.iter().map(|(id, row)| (id.clone(), row.sheet.clone())).collect(),
            viewport_views: snapshot.viewports.iter().map(|(id, row)| (id.clone(), row.view.clone())).collect(),
            revision_sheets: snapshot.sheet_revisions.iter().map(|(id, row)| (id.clone(), row.sheet.clone())).collect(),
            building_sites: snapshot.buildings.iter().map(|(id, row)| (id.clone(), row.site.clone())).collect(),
            storey_buildings: snapshot.storeys.iter().map(|(id, row)| (id.clone(), row.building.clone())).collect(),
            grid_buildings: snapshot.grids.iter().map(|(id, row)| (id.clone(), row.building.clone())).collect(),
            placed: stray,
            openings: snapshot.openings.iter().map(|(id, row)| (id.clone(), row.host.clone())).collect(),
            hosts: snapshot.walls.keys().chain(snapshot.curtain_walls.keys()).cloned().collect(),
            layered,
            named,
            materials: snapshot.materials.keys().cloned().collect(),
            types: snapshot.wall_types.keys().chain(snapshot.slab_types.keys()).chain(snapshot.ceiling_types.keys()).chain(snapshot.roof_types.keys()).chain(snapshot.column_types.keys()).chain(snapshot.beam_types.keys()).chain(snapshot.window_types.keys()).chain(snapshot.door_types.keys()).chain(snapshot.curtain_wall_types.keys()).cloned().collect(),
            attached: snapshot.properties.keys().chain(snapshot.classifications.keys()).cloned().collect(),
            doors: snapshot.door_types.keys().cloned().collect(),
            windows: snapshot.window_types.keys().cloned().collect(),
            curtains: snapshot.curtain_walls.keys().cloned().collect(),
            overrides: snapshot.curtain_panel_overrides.iter().map(|(id, row)| (id.clone(), row.curtain.clone())).collect(),
            panels,
            counts,
            sweep_hosts: snapshot.wall_sweeps.iter().map(|(id, row)| (id.clone(), row.host.clone())).collect(),
            attaches: snapshot.walls.iter().flat_map(|(id, wall)| references::targets_of(wall).into_iter().map(move |target| Attach { wall: id.clone(), target: target.to_string() })).collect(),
            attachable: snapshot.roofs.keys().chain(snapshot.slabs.keys()).chain(snapshot.ceilings.keys()).cloned().collect(),
        }
    }
}
//#endregion 🔖️View

/// 🌍️ The dangling references that belong to no storey, plus orphaned properties and ids used twice.
pub fn model(view: &ReferenceView) -> Vec<Diagnostic> {
    use DiagnosticCode::*;
    let mut found = Vec::new();
    let sites: BTreeSet<&String> = view.sites.iter().collect();
    let materials: BTreeSet<&String> = view.materials.iter().collect();
    let hosts: BTreeSet<&String> = view.hosts.iter().collect();
    for (id, site) in view.building_sites.iter().filter(|(_, site)| !sites.contains(site)) {
        found.push(Diagnostic::new(RefBuildingSite, &[id]).lacking(site));
    }
    for (id, building) in view.storey_buildings.iter().filter(|(_, building)| !view.building_sites.contains_key(*building)) {
        found.push(Diagnostic::new(RefStoreyBuilding, &[id]).lacking(building));
    }
    for (id, building) in view.grid_buildings.iter().filter(|(_, building)| !view.building_sites.contains_key(*building)) {
        found.push(Diagnostic::new(RefGridBuilding, &[id]).lacking(building));
    }
    found.extend(view.placed.iter().filter(|row| !view.storey_buildings.contains_key(&row.storey)).map(|row| Diagnostic::new(RefElementStorey, &[&row.id]).lacking(&row.storey)));
    for (id, host) in view.openings.iter().filter(|(_, host)| !hosts.contains(host)) {
        found.push(Diagnostic::new(RefOpeningHost, &[id]).lacking(host));
    }
    for (id, layers) in &view.layered {
        let missing: BTreeSet<&String> = layers.iter().filter(|material| !materials.contains(material)).collect();
        found.extend(missing.into_iter().map(|material| Diagnostic::new(RefLayerMaterial, &[id]).lacking(material)));
    }
    found.extend(view.named.iter().filter(|row| !row.material.is_empty() && !materials.contains(&row.material)).map(|row| Diagnostic::new(RefTypeMaterial, &[&row.id]).lacking(&row.material)));
    let attachable: BTreeSet<&String> = view.attachable.iter().collect();
    found.extend(view.sweep_hosts.iter().filter(|(_, host)| !hosts.contains(host)).map(|(id, host)| Diagnostic::new(RefWallSweepHost, &[id]).lacking(host)));
    found.extend(view.attaches.iter().filter(|row| !attachable.contains(&row.target)).map(|row| Diagnostic::new(RefAttachTarget, &[&row.wall]).lacking(&row.target)));
    let (doors, windows, curtains): (BTreeSet<&String>, BTreeSet<&String>, BTreeSet<&String>) = (view.doors.iter().collect(), view.windows.iter().collect(), view.curtains.iter().collect());
    for panel in &view.panels {
        let exists = match panel.kind.as_str() {
            "door" => doors.contains(&panel.reference),
            "window" => windows.contains(&panel.reference),
            _ => materials.contains(&panel.reference),
        };
        if !exists {
            found.push(Diagnostic::new(RefCurtainPanel, &[&panel.id]).lacking(&panel.reference));
        }
    }
    found.extend(view.overrides.iter().filter(|(_, curtain)| !curtains.contains(curtain)).map(|(id, curtain)| Diagnostic::new(RefCurtainOverrideHost, &[id]).lacking(curtain)));
    let (sheets, views): (BTreeSet<&String>, BTreeSet<&String>) = (view.sheets.iter().collect(), view.views.iter().collect());
    found.extend(view.viewport_sheets.iter().filter(|(_, sheet)| !sheets.contains(sheet)).map(|(id, sheet)| Diagnostic::new(RefViewportSheet, &[id]).lacking(sheet)));
    found.extend(view.viewport_views.iter().filter(|(_, shown)| !views.contains(shown)).map(|(id, shown)| Diagnostic::new(RefViewportView, &[id]).lacking(shown)));
    found.extend(view.revision_sheets.iter().filter(|(_, sheet)| !sheets.contains(sheet)).map(|(id, sheet)| Diagnostic::new(RefRevisionSheet, &[id]).lacking(sheet)));
    found.extend(view.counts.iter().filter(|(_, count)| **count > 1).map(|(id, _)| Diagnostic::new(DuplicateId, &[id])));
    let known = |id: &String| view.counts.contains_key(id) || materials.contains(id) || view.types.contains(id);
    found.extend(view.attached.iter().filter(|id| !known(id)).collect::<BTreeSet<_>>().into_iter().map(|id| Diagnostic::new(RefPropertyElement, &[id])));
    found
}

