//! 🧭️ The plan of the model graph: every node of the wanted kinds, parents before children, with the real edges of the design (`r7-design-model-graph.md` section 2).
//!
//! The indexes the edges need are built once per run, never per element: the walls of each storey and the walls each wall touches (one sweep per storey), the openings of each host, the elements of
//! each storey. The functions here only decide *which* nodes exist and *which* nodes are their parents; what a node computes is in `compute`.

use super::super::super::annotation_layout;
use super::super::super::clash_sets;
use super::super::super::rule_results;
use super::super::super::effective_properties;
use super::super::super::families;
use super::super::super::element_solids::beams;
use super::super::super::element_solids::fillers::family_of;
use super::super::super::element_solids::{SolidFamily, SolidKey};
use super::super::super::schedules::rows::{candidates, property_keys};
use super::super::super::storey_levels::{constraint_storeys, stackings};
use super::super::super::wall_layout::{band_of, joins};
use crate::standards::v1::subsets::any::schema::authored::references;
use super::super::super::energy_envelope::{neighbour_storey, Climate, EnergyScope};
use super::super::super::zones::counts;
use super::{DiagnosticScope, ModelNode, NodeKind, TotalsScope};
use crate::{ModelSnapshot, TopConstraint, ViewKind};
use protocol::InferenceStep;
use std::collections::{BTreeMap, BTreeSet};

/// 🏗️ The wall or curtain wall that hosts openings: its storey and top constraint, a wall winning over a curtain wall of the same id.
pub fn host_of<'a>(snapshot: &'a ModelSnapshot, host: &str) -> Option<(&'a String, &'a TopConstraint)> {
    match (snapshot.walls.get(host), snapshot.curtain_walls.get(host)) {
        (Some(wall), _) => Some((&wall.storey, &wall.top)),
        (None, Some(curtain)) => Some((&curtain.storey, &curtain.top)),
        (None, None) => None,
    }
}

/// 🪜️ The storey an opening stands on: the storey of its host.
pub fn opening_storey(snapshot: &ModelSnapshot, opening: &str) -> Option<String> {
    snapshot.openings.get(opening).and_then(|row| host_of(snapshot, &row.host)).map(|(storey, _)| storey.clone())
}

/// 🪜️ The storey nodes an element is resolved by (its own and the target of a `Storey` top), keeping only storeys that exist.
fn storeys_of(snapshot: &ModelSnapshot, storey: &str, top: Option<&TopConstraint>) -> Vec<ModelNode> {
    let ids = match top {
        Some(top) => constraint_storeys(storey, top),
        None => vec![storey.to_string()],
    };
    ids.into_iter().filter(|id| snapshot.storeys.contains_key(id)).map(ModelNode::Storey).collect()
}

fn step(key: ModelNode, parents: Vec<ModelNode>) -> InferenceStep<ModelNode> {
    InferenceStep { key, parents }
}

/// 🧭️ The steps of the wanted kinds (a mask of [`NodeKind`] bits, already closed under `requires`) in topological order.
pub fn build(snapshot: &ModelSnapshot, wanted: u64) -> Vec<InferenceStep<ModelNode>> {
    let has = |kind: NodeKind| wanted & kind.bit() != 0;
    let mut steps: Vec<InferenceStep<ModelNode>> = Vec::new();
    let on_storey = |storey: &String| snapshot.storeys.contains_key(storey);

    if has(NodeKind::Storey) {
        steps.extend(stackings(snapshot).into_iter().map(|(id, below)| step(ModelNode::Storey(id), below.into_iter().map(ModelNode::Storey).collect())));
    }

    let walls: Vec<(&String, &crate::Wall)> = snapshot.walls.iter().filter(|(_, wall)| on_storey(&wall.storey)).collect();
    let curtains: Vec<(&String, &crate::CurtainWall)> = snapshot.curtain_walls.iter().filter(|(_, curtain)| on_storey(&curtain.storey)).collect();

    if has(NodeKind::Band) {
        steps.extend(walls.iter().map(|(id, _)| step(ModelNode::Band((*id).clone()), Vec::new())));
    }
    if has(NodeKind::Cut) {
        steps.extend(snapshot.openings.keys().map(|id| step(ModelNode::Cut(id.clone()), Vec::new())));
    }
    let properties: BTreeSet<String> = if has(NodeKind::Properties) {
        let holders = effective_properties::holders(snapshot);
        steps.extend(holders.iter().map(|(id, parent)| step(ModelNode::Properties(id.clone()), parent.iter().cloned().map(ModelNode::Properties).collect())));
        holders.into_iter().map(|(id, _)| id).collect()
    } else {
        BTreeSet::new()
    };
    if has(NodeKind::PhaseVisibility) {
        steps.extend(snapshot.storeys.keys().map(|id| step(ModelNode::PhaseVisibility(id.clone()), Vec::new())));
    }
    if has(NodeKind::CurtainLayout) {
        steps.extend(curtains.iter().map(|(id, curtain)| step(ModelNode::CurtainLayout((*id).clone()), storeys_of(snapshot, &curtain.storey, Some(&curtain.top)))));
    }
    let surfaces: BTreeSet<&str> = walls.iter().flat_map(|(_, wall)| references::targets_of(wall)).filter(|id| references::storey_of(snapshot, id).is_some_and(|storey| on_storey(storey))).collect();
    if has(NodeKind::Surface) {
        steps.extend(surfaces.iter().filter_map(|id| references::storey_of(snapshot, id).map(|storey| step(ModelNode::Surface((*id).to_string()), vec![ModelNode::Storey(storey.clone())]))));
    }
    if has(NodeKind::WallLayout) {
        let mut bands: BTreeMap<&str, BTreeMap<String, joins::Band>> = BTreeMap::new();
        for (id, wall) in &walls {
            if let Some(band) = band_of(snapshot, wall) {
                bands.entry(wall.storey.as_str()).or_default().insert((*id).clone(), band);
            }
        }
        let touching: BTreeMap<&str, BTreeMap<String, BTreeSet<String>>> = bands.iter().map(|(storey, set)| (*storey, joins::touching(set))).collect();
        steps.extend(walls.iter().map(|(id, wall)| {
            let mut parents = storeys_of(snapshot, &wall.storey, Some(&wall.top));
            parents.push(ModelNode::Band((*id).clone()));
            parents.extend(references::targets_of(wall).into_iter().filter(|target| surfaces.contains(target)).map(|target| ModelNode::Surface(target.to_string())));
            if let Some(touch) = touching.get(wall.storey.as_str()) {
                parents.extend(joins::neighbourhood(touch, id).into_iter().map(ModelNode::Band));
            }
            step(ModelNode::WallLayout((*id).clone()), parents)
        }));
    }

    let mut by_host: BTreeMap<&str, Vec<&String>> = BTreeMap::new();
    for (id, opening) in &snapshot.openings {
        by_host.entry(opening.host.as_str()).or_default().push(id);
    }
    let resolvable = |host: &str| host_of(snapshot, host).filter(|(storey, _)| on_storey(storey)).is_some();
    if has(NodeKind::Host) {
        for host in by_host.keys().filter(|host| resolvable(host)) {
            let (storey, _) = host_of(snapshot, host).expect("a resolvable host");
            let layout = if snapshot.walls.contains_key(*host) { ModelNode::WallLayout((*host).to_string()) } else { ModelNode::CurtainLayout((*host).to_string()) };
            steps.push(step(ModelNode::Host((*host).to_string()), vec![ModelNode::Storey(storey.clone()), layout]));
        }
    }
    if has(NodeKind::OpeningFrame) {
        for (id, opening) in &snapshot.openings {
            let mut parents = Vec::new();
            if resolvable(&opening.host) {
                parents.push(ModelNode::Host(opening.host.clone()));
            }
            parents.push(ModelNode::Cut(id.clone()));
            parents.extend(by_host.get(opening.host.as_str()).into_iter().flatten().filter(|other| other.as_str() != id.as_str()).map(|other| ModelNode::Cut((*other).clone())));
            steps.push(step(ModelNode::OpeningFrame(id.clone()), parents));
        }
    }
    let stairs: Vec<(&String, &crate::Stair)> = snapshot.stairs.iter().filter(|(_, stair)| on_storey(&stair.storey)).collect();
    if has(NodeKind::StairRun) {
        steps.extend(stairs.iter().map(|(id, stair)| step(ModelNode::StairRun((*id).clone()), storeys_of(snapshot, &stair.storey, Some(&stair.top)))));
    }
    let ramps: Vec<(&String, &crate::Ramp)> = snapshot.ramps.iter().filter(|(_, ramp)| on_storey(&ramp.storey)).collect();
    if has(NodeKind::RampRun) {
        steps.extend(ramps.iter().map(|(id, ramp)| step(ModelNode::RampRun((*id).clone()), storeys_of(snapshot, &ramp.storey, Some(&ramp.top)))));
    }

    let walls_by_id: BTreeMap<&str, &crate::Wall> = walls.iter().map(|(id, wall)| (id.as_str(), *wall)).collect();
    let sweeps: Vec<(&String, &crate::WallSweep)> = snapshot.wall_sweeps.iter().filter(|(_, sweep)| walls_by_id.contains_key(sweep.host.as_str())).collect();
    let filler = |id: &String| snapshot.openings.get(id).and_then(|opening| family_of(&opening.kind));
    let fillers: Vec<(&String, SolidFamily)> = snapshot.openings.keys().filter(|id| opening_storey(snapshot, id).is_some_and(|storey| on_storey(&storey))).filter_map(|id| filler(id).map(|family| (id, family))).collect();
    if has(NodeKind::Family) {
        steps.extend(snapshot.families.keys().map(|id| step(ModelNode::Family(id.clone()), Vec::new())));
    }
    let components: Vec<(&String, &crate::Component)> = snapshot.components.iter().filter(|(_, component)| on_storey(&component.storey)).collect();
    let meps: Vec<(&String, &crate::MepElement)> = snapshot.mep_elements.iter().filter(|(_, mep)| on_storey(&mep.storey)).collect();
    let mut meps_by_storey: BTreeMap<&str, Vec<&String>> = BTreeMap::new();
    for (id, mep) in &meps {
        meps_by_storey.entry(mep.storey.as_str()).or_default().push(id);
    }
    if has(NodeKind::Component) {
        for (id, component) in &components {
            let mut parents = vec![ModelNode::Storey(component.storey.clone())];
            if snapshot.families.contains_key(&component.family) {
                parents.push(ModelNode::Family(component.family.clone()));
            }
            if let Some(wall) = component.host.as_ref().filter(|wall| walls_by_id.contains_key(wall.as_str())) {
                parents.push(ModelNode::WallLayout(wall.clone()));
            }
            steps.push(step(ModelNode::Component((*id).clone()), parents));
        }
    }
    if has(NodeKind::Mep) {
        steps.extend(meps.iter().map(|(id, mep)| step(ModelNode::Mep((*id).clone()), vec![ModelNode::Storey(mep.storey.clone())])));
    }
    if has(NodeKind::MepClash) {
        steps.extend(meps_by_storey.iter().map(|(storey, ids)| step(ModelNode::MepClash((*storey).to_string()), ids.iter().map(|id| ModelNode::Mep((*id).clone())).collect())));
    }
    if has(NodeKind::Solid) {
        for (id, component) in &components {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Component, id)), vec![ModelNode::Storey(component.storey.clone()), ModelNode::Component((*id).clone())]));
        }
        for (id, mep) in &meps {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Mep, id)), vec![ModelNode::Storey(mep.storey.clone()), ModelNode::Mep((*id).clone())]));
        }
        for (id, wall) in &walls {
            let mut parents = vec![ModelNode::Storey(wall.storey.clone()), ModelNode::WallLayout((*id).clone())];
            parents.extend(by_host.get(id.as_str()).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Wall, id)), parents));
        }
        for (id, sweep) in &sweeps {
            let host = sweep.host.as_str();
            let mut parents = vec![ModelNode::Storey(walls_by_id[host].storey.clone()), ModelNode::WallLayout(host.to_string())];
            parents.extend(by_host.get(host).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::WallSweep, id)), parents));
        }
        for (id, curtain) in &curtains {
            let mut parents = vec![ModelNode::Storey(curtain.storey.clone()), ModelNode::CurtainLayout((*id).clone())];
            parents.extend(by_host.get(id.as_str()).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.extend(family_parents(snapshot, snapshot.curtain_wall_types.get(&curtain.curtain_wall_type).map(|kind| vec![&kind.interior_mullion, &kind.border_mullion]).unwrap_or_default()));
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::CurtainWall, id)), parents));
        }
        for (id, family) in &fillers {
            let storey = opening_storey(snapshot, id).expect("a filler stands on a storey");
            steps.push(step(ModelNode::Solid(SolidKey::of(*family, id)), vec![ModelNode::Storey(storey), ModelNode::OpeningFrame((*id).clone())]));
        }
        for (id, column) in snapshot.columns.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Column, id)), [storeys_of(snapshot, &column.storey, Some(&column.top)), family_parents(snapshot, snapshot.column_types.get(&column.column_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat()));
        }
        for (id, beam) in snapshot.beams.iter().filter(|(_, row)| on_storey(&row.storey)) {
            let mut parents = vec![ModelNode::Storey(beam.storey.clone())];
            for (_, column) in beams::joining(snapshot, beam) {
                for node in storeys_of(snapshot, &column.storey, Some(&column.top)) {
                    if !parents.contains(&node) {
                        parents.push(node);
                    }
                }
            }
            parents.extend(family_parents(snapshot, snapshot.beam_types.get(&beam.beam_type).map(|kind| vec![&kind.profile]).unwrap_or_default()));
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id)), parents));
        }
        for (id, slab) in snapshot.slabs.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Slab, id)), vec![ModelNode::Storey(slab.storey.clone())]));
        }
        for (id, ceiling) in snapshot.ceilings.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Ceiling, id)), vec![ModelNode::Storey(ceiling.storey.clone())]));
        }
        for (id, roof) in snapshot.roofs.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id)), vec![ModelNode::Storey(roof.storey.clone())]));
        }
        for (id, stair) in &stairs {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Stair, id)), vec![ModelNode::Storey(stair.storey.clone()), ModelNode::StairRun((*id).clone())]));
        }
        for (id, ramp) in &ramps {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Ramp, id)), vec![ModelNode::Storey(ramp.storey.clone()), ModelNode::RampRun((*id).clone())]));
        }
        for (id, railing) in snapshot.railings.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id)), [railing_parents(snapshot, railing), family_parents(snapshot, railing_profiles(railing))].concat()));
        }
    }

    let room_storeys: BTreeSet<&String> = snapshot.spaces.values().map(|space| &space.storey).filter(|storey| on_storey(storey)).collect();
    let wall_layouts = |storey: &str| -> Vec<ModelNode> { walls.iter().filter(|(_, wall)| wall.storey == storey).map(|(id, _)| ModelNode::WallLayout((*id).clone())).collect() };
    if has(NodeKind::Room) {
        for storey in &room_storeys {
            let mut parents = vec![ModelNode::Storey((*storey).clone())];
            parents.extend(wall_layouts(storey));
            steps.push(step(ModelNode::Room((*storey).clone()), parents));
        }
    }
    let wall_ids: BTreeSet<&String> = walls.iter().map(|(id, _)| *id).collect();
    let annotated: BTreeSet<&String> = snapshot.storeys.keys().filter(|storey| annotation_layout::is_annotated(snapshot, storey)).collect();
    if has(NodeKind::Annotation) {
        for storey in &annotated {
            let parents = annotation_layout::face_walls(snapshot, storey).into_iter().filter(|wall| wall_ids.contains(wall)).map(ModelNode::WallLayout).collect();
            steps.push(step(ModelNode::Annotation((*storey).clone()), parents));
        }
    }
    if has(NodeKind::Plan) {
        for storey in snapshot.storeys.keys() {
            let mut parents: Vec<ModelNode> = level_storeys(snapshot, storey).into_iter().map(ModelNode::Storey).collect();
            parents.extend(wall_layouts(storey));
            parents.extend(curtains.iter().filter(|(_, curtain)| &curtain.storey == storey).map(|(id, _)| ModelNode::CurtainLayout((*id).clone())));
            let hosted = walls.iter().filter(|(_, wall)| &wall.storey == storey).flat_map(|(id, _)| by_host.get(id.as_str()).into_iter().flatten());
            parents.extend(hosted.map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| &stair.storey == storey).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            parents.extend(ramps.iter().filter(|(_, ramp)| &ramp.storey == storey).map(|(id, _)| ModelNode::RampRun((*id).clone())));
            if room_storeys.contains(storey) {
                parents.push(ModelNode::Room(storey.clone()));
            }
            if annotated.contains(storey) {
                parents.push(ModelNode::Annotation(storey.clone()));
            }
            parents.extend(components.iter().filter(|(_, component)| &component.storey == storey).map(|(id, _)| ModelNode::Component((*id).clone())));
            parents.extend(meps_by_storey.get(storey.as_str()).into_iter().flatten().map(|id| ModelNode::Mep((*id).clone())));
            steps.push(step(ModelNode::Plan(storey.clone()), parents));
        }
    }

    if has(NodeKind::Quantity) {
        let mut quantities: Vec<(String, Vec<ModelNode>)> = Vec::new();
        for (id, _) in &walls {
            let mut parents = vec![ModelNode::WallLayout((*id).clone())];
            parents.extend(by_host.get(id.as_str()).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            quantities.push(((*id).clone(), parents));
        }
        for (id, _) in &curtains {
            let mut parents = vec![ModelNode::CurtainLayout((*id).clone())];
            parents.extend(by_host.get(id.as_str()).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.push(ModelNode::Solid(SolidKey::of(SolidFamily::CurtainWall, id)));
            quantities.push(((*id).clone(), parents));
        }
        for (id, sweep) in &sweeps {
            let host = sweep.host.as_str();
            let mut parents = vec![ModelNode::WallLayout(host.to_string())];
            parents.extend(by_host.get(host).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.push(ModelNode::Solid(SolidKey::of(SolidFamily::WallSweep, id)));
            quantities.push(((*id).clone(), parents));
        }
        quantities.extend(snapshot.slabs.keys().map(|id| (id.clone(), Vec::new())));
        quantities.extend(snapshot.ceilings.keys().map(|id| (id.clone(), Vec::new())));
        for (id, roof) in &snapshot.roofs {
            quantities.push((id.clone(), if on_storey(&roof.storey) { vec![ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id))] } else { Vec::new() }));
        }
        for (id, column) in snapshot.columns.iter().filter(|(_, row)| on_storey(&row.storey)) {
            quantities.push((id.clone(), [storeys_of(snapshot, &column.storey, Some(&column.top)), family_parents(snapshot, snapshot.column_types.get(&column.column_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat()));
        }
        for (id, beam) in &snapshot.beams {
            quantities.push((id.clone(), if on_storey(&beam.storey) { [vec![ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id))], family_parents(snapshot, snapshot.beam_types.get(&beam.beam_type).map(|kind| vec![&kind.profile]).unwrap_or_default())].concat() } else { Vec::new() }));
        }
        for id in snapshot.openings.keys() {
            let mut parents = vec![ModelNode::OpeningFrame(id.clone())];
            parents.extend(fillers.iter().filter(|(other, _)| *other == id).map(|(_, family)| ModelNode::Solid(SolidKey::of(*family, id))));
            quantities.push((id.clone(), parents));
        }
        for (id, _) in &stairs {
            quantities.push(((*id).clone(), vec![ModelNode::StairRun((*id).clone()), ModelNode::Solid(SolidKey::of(SolidFamily::Stair, id))]));
        }
        for (id, _) in &ramps {
            quantities.push(((*id).clone(), vec![ModelNode::RampRun((*id).clone()), ModelNode::Solid(SolidKey::of(SolidFamily::Ramp, id))]));
        }
        for (id, _) in &components {
            quantities.push(((*id).clone(), vec![ModelNode::Component((*id).clone()), ModelNode::Solid(SolidKey::of(SolidFamily::Component, id))]));
        }
        for (id, _) in &meps {
            quantities.push(((*id).clone(), vec![ModelNode::Mep((*id).clone()), ModelNode::Solid(SolidKey::of(SolidFamily::Mep, id))]));
        }
        for (id, railing) in &snapshot.railings {
            quantities.push((id.clone(), if on_storey(&railing.storey) { [vec![ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id))], railing_parents(snapshot, railing)].concat() } else { Vec::new() }));
        }
        let mut hosted_on_storey: BTreeMap<&str, Vec<ModelNode>> = BTreeMap::new();
        for (id, space) in snapshot.spaces.iter().filter(|(_, space)| room_storeys.contains(&space.storey)) {
            let hosted = hosted_on_storey.entry(space.storey.as_str()).or_insert_with(|| {
                let hosts = walls.iter().filter(|(_, wall)| wall.storey == space.storey).map(|(id, _)| id.as_str()).chain(curtains.iter().filter(|(_, curtain)| curtain.storey == space.storey).map(|(id, _)| id.as_str()));
                hosts.flat_map(|host| by_host.get(host).into_iter().flatten()).map(|opening| ModelNode::OpeningFrame((*opening).clone())).collect()
            });
            let mut parents = vec![ModelNode::Room(space.storey.clone())];
            parents.extend(hosted.iter().cloned());
            quantities.push((id.clone(), parents));
        }
        quantities.sort_by(|a, b| a.0.cmp(&b.0));
        steps.extend(quantities.iter().map(|(id, parents)| step(ModelNode::Quantity(id.clone()), parents.clone())));

        if has(NodeKind::Totals) {
            let storey_of = |id: &str| element_storey(snapshot, id);
            let mut storeys: BTreeMap<String, Vec<ModelNode>> = BTreeMap::new();
            let mut buildings: BTreeMap<String, Vec<ModelNode>> = BTreeMap::new();
            for (id, _) in &quantities {
                if let Some(storey) = storey_of(id).filter(|storey| snapshot.storeys.contains_key(storey)) {
                    storeys.entry(storey.clone()).or_default().push(ModelNode::Quantity(id.clone()));
                    buildings.entry(snapshot.storeys[&storey].building.clone()).or_default().push(ModelNode::Quantity(id.clone()));
                }
            }
            steps.extend(storeys.into_iter().map(|(id, parents)| step(ModelNode::Totals(TotalsScope::Storey(id)), parents)));
            steps.extend(buildings.into_iter().map(|(id, parents)| step(ModelNode::Totals(TotalsScope::Building(id)), parents)));
            steps.push(step(ModelNode::Totals(TotalsScope::Project), quantities.iter().map(|(id, _)| ModelNode::Quantity(id.clone())).collect()));
        }

        if has(NodeKind::Zone) || has(NodeKind::Scheme) {
            let measured: BTreeSet<&str> = quantities.iter().map(|(id, _)| id.as_str()).collect();
            let spaces_of = |counted: &dyn Fn(&crate::Space) -> bool| -> Vec<ModelNode> { snapshot.spaces.iter().filter(|(id, space)| measured.contains(id.as_str()) && counted(space)).map(|(id, _)| ModelNode::Quantity(id.clone())).collect() };
            if has(NodeKind::Zone) {
                steps.extend(snapshot.zones.keys().map(|zone| step(ModelNode::Zone(zone.clone()), spaces_of(&|space| space.zone.as_deref() == Some(zone.as_str())))));
            }
            if has(NodeKind::Scheme) {
                steps.extend(snapshot.area_schemes.iter().map(|(id, scheme)| step(ModelNode::Scheme(id.clone()), spaces_of(&|space| counts(scheme, space)))));
            }
        }

        if has(NodeKind::Schedule) {
            let planned: BTreeSet<&str> = quantities.iter().map(|(id, _)| id.as_str()).collect();
            for (id, schedule) in &snapshot.schedules {
                let elements = candidates(snapshot, schedule);
                let mut parents: Vec<ModelNode> = elements.iter().filter(|element| planned.contains(element.as_str())).cloned().map(ModelNode::Quantity).collect();
                if !property_keys(schedule).is_empty() {
                    parents.extend(elements.iter().filter(|element| properties.contains(*element)).cloned().map(ModelNode::Properties));
                }
                steps.push(step(ModelNode::Schedule(id.clone()), parents));
            }
        }
    }

    let envelopes: Vec<ModelNode> = if has(NodeKind::Envelope) && !snapshot.space_conditions.is_empty() {
        let mut nodes = Vec::new();
        for (id, space) in snapshot.spaces.iter().filter(|(_, space)| room_storeys.contains(&space.storey)) {
            let mut parents = vec![ModelNode::Room(space.storey.clone())];
            for up in [true, false] {
                if let Some(other) = neighbour_storey(snapshot, &space.storey, up).filter(|other| room_storeys.contains(other)) {
                    parents.push(ModelNode::Room(other));
                }
            }
            parents.extend(wall_layouts(&space.storey));
            let on_storey = walls.iter().filter(|(_, wall)| wall.storey == space.storey).flat_map(|(wall, _)| by_host.get(wall.as_str()).into_iter().flatten());
            parents.extend(on_storey.map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            nodes.push(ModelNode::Envelope(id.clone()));
            steps.push(step(ModelNode::Envelope(id.clone()), parents));
        }
        if has(NodeKind::EnergyTotals) {
            let climate = Climate::of(snapshot);
            let members = |scope: &EnergyScope| -> Vec<ModelNode> { nodes.iter().filter(|node| matches!(node, ModelNode::Envelope(space) if climate.in_scope(scope, space))).cloned().collect() };
            let scopes: Vec<EnergyScope> = snapshot.zones.keys().map(|zone| EnergyScope::Zone(zone.clone())).chain(snapshot.buildings.keys().map(|building| EnergyScope::Building(building.clone()))).chain(std::iter::once(EnergyScope::Project)).collect();
            let totals: Vec<(EnergyScope, Vec<ModelNode>)> = scopes.into_iter().map(|scope| (scope.clone(), members(&scope))).collect();
            steps.extend(totals.into_iter().map(|(scope, parents)| step(ModelNode::EnergyTotals(scope), parents)));
        }
        nodes
    } else {
        Vec::new()
    };
    if has(NodeKind::OptionScope) {
        let parents = steps.iter().filter_map(|step| matches!(step.key, ModelNode::Quantity(_)).then(|| step.key.clone())).collect();
        steps.push(step(ModelNode::OptionScope, parents));
    }
    if has(NodeKind::Diagnostics) {
        let buildings: BTreeSet<&String> = snapshot.storeys.values().map(|storey| &storey.building).collect();
        for storey in snapshot.storeys.keys() {
            let mut parents: Vec<ModelNode> = level_storeys(snapshot, storey).into_iter().map(ModelNode::Storey).collect();
            parents.extend(wall_layouts(storey));
            let hosted = walls.iter().filter(|(_, wall)| &wall.storey == storey).map(|(id, _)| id.as_str()).chain(curtains.iter().filter(|(_, curtain)| &curtain.storey == storey).map(|(id, _)| id.as_str()));
            parents.extend(hosted.flat_map(|host| by_host.get(host).into_iter().flatten()).map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| &stair.storey == storey).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            parents.extend(ramps.iter().filter(|(_, ramp)| &ramp.storey == storey).map(|(id, _)| ModelNode::RampRun((*id).clone())));
            parents.extend(curtains.iter().filter(|(_, curtain)| &curtain.storey == storey).map(|(id, _)| ModelNode::CurtainLayout((*id).clone())));
            if room_storeys.contains(storey) {
                parents.push(ModelNode::Room(storey.clone()));
            }
            parents.extend(snapshot.roofs.iter().filter(|(_, roof)| &roof.storey == storey).map(|(id, _)| ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id))));
            if annotated.contains(storey) {
                parents.push(ModelNode::Annotation(storey.clone()));
            }
            parents.extend(components.iter().filter(|(_, component)| &component.storey == storey).map(|(id, _)| ModelNode::Component((*id).clone())));
            if let Some(ids) = meps_by_storey.get(storey.as_str()) {
                parents.extend(ids.iter().map(|id| ModelNode::Mep((*id).clone())));
                parents.push(ModelNode::MepClash(storey.clone()));
            }
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Storey(storey.clone())), parents));
        }
        for building in buildings {
            let members: BTreeSet<&String> = snapshot.storeys.iter().filter(|(_, storey)| &storey.building == building).map(|(id, _)| id).collect();
            let mut parents: Vec<ModelNode> = members.iter().map(|id| ModelNode::Storey((*id).clone())).collect();
            parents.extend(walls.iter().filter(|(_, wall)| members.contains(&wall.storey)).map(|(id, _)| ModelNode::WallLayout((*id).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| members.contains(&stair.storey)).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            parents.extend(ramps.iter().filter(|(_, ramp)| members.contains(&ramp.storey)).map(|(id, _)| ModelNode::RampRun((*id).clone())));
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Building(building.clone())), parents));
        }
        steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Model), snapshot.families.keys().map(|id| ModelNode::Family(id.clone())).collect()));
        if !envelopes.is_empty() {
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Energy), envelopes.clone()));
        }
        if !(snapshot.properties.is_empty() && snapshot.classifications.is_empty() && snapshot.property_templates.is_empty() && snapshot.classification_systems.is_empty()) {
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Data), properties.iter().cloned().map(ModelNode::Properties).collect()));
        }
        if has(NodeKind::DiagnosticIndex) {
            let scopes = steps.iter().filter(|planned| planned.key.kind() == NodeKind::Diagnostics).map(|planned| planned.key.clone()).collect();
            steps.push(step(ModelNode::DiagnosticIndex, scopes));
        }
    }
    if has(NodeKind::View) {
        steps.extend(view_steps(snapshot, &steps, &on_storey));
    }
    if has(NodeKind::Sheet) {
        steps.extend(sheet_steps(snapshot, &steps));
    }
    if has(NodeKind::ClashSet) {
        steps.extend(clash_steps(snapshot, &steps));
    }
    if has(NodeKind::Rule) {
        steps.extend(rule_steps(snapshot, &steps));
    }
    if has(NodeKind::StructuralAnalysis) {
        let parents = steps.iter().filter(|step| matches!(step.key, ModelNode::Storey(_) | ModelNode::WallLayout(_) | ModelNode::Solid(_))).map(|step| step.key.clone()).collect();
        steps.push(step(ModelNode::StructuralAnalysis, parents));
    }
    steps
}

/// 📄️ The `Sheet` nodes: every sheet, with the `View` node of each view its viewports show as parent (a view that has no node, because its building is missing, is no parent).
fn sheet_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>]) -> Vec<InferenceStep<ModelNode>> {
    let planned: BTreeSet<&String> = steps.iter().filter_map(|planned_step| if let ModelNode::View(id) = &planned_step.key { Some(id) } else { None }).collect();
    snapshot
        .sheets
        .keys()
        .map(|id| {
            let views: BTreeSet<&String> = snapshot.viewports.values().filter(|viewport| &viewport.sheet == id && planned.contains(&viewport.view)).map(|viewport| &viewport.view).collect();
            step(ModelNode::Sheet(id.clone()), views.into_iter().map(|view| ModelNode::View(view.clone())).collect())
        })
        .collect()
}

/// 🧨️ The `Probe` nodes (the spatial index of every solid some clash set picks) and the `ClashSet` nodes (one per set, the probes of the elements it picks as parents).
fn clash_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>]) -> Vec<InferenceStep<ModelNode>> {
    let solids: Vec<SolidKey> = steps.iter().filter_map(|planned| if let ModelNode::Solid(key) = &planned.key { Some(key.clone()) } else { None }).collect();
    let mut probes: BTreeSet<SolidKey> = BTreeSet::new();
    let mut sets: Vec<InferenceStep<ModelNode>> = Vec::new();
    for (id, set) in &snapshot.clash_sets {
        let picked = clash_sets::needed(snapshot, set, &solids);
        probes.extend(picked.iter().map(|key| (*key).clone()));
        sets.push(step(ModelNode::ClashSet(id.clone()), picked.into_iter().map(|key| ModelNode::Probe(key.clone())).collect()));
    }
    probes.into_iter().map(|key| step(ModelNode::Probe(key.clone()), vec![ModelNode::Solid(key)])).chain(sets).collect()
}

/// ⚖️ The `Rule` nodes: one per rule, the planned nodes that hold the measures of its members as parents (stair runs, opening frames, ramp runs, rooms of the storeys of its spaces or zones).
fn rule_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>]) -> Vec<InferenceStep<ModelNode>> {
    use crate::RuleKind::{MaxCompartmentArea, MaxRampSlope, MaxRiser, MinClearHeight, MinCorridorWidth, MinDoorWidth, MinStairWidth, MinTread};
    let planned: BTreeSet<&ModelNode> = steps.iter().map(|planned| &planned.key).collect();
    snapshot
        .rules
        .iter()
        .map(|(id, rule)| {
            let members = rule_results::members(snapshot, rule);
            let nodes: BTreeSet<ModelNode> = members
                .iter()
                .filter_map(|member| match rule.kind {
                    MaxRiser | MinTread | MinStairWidth => Some(ModelNode::StairRun(member.id.clone())),
                    MinDoorWidth => Some(ModelNode::OpeningFrame(member.id.clone())),
                    MaxRampSlope => Some(ModelNode::RampRun(member.id.clone())),
                    MaxCompartmentArea => Some(ModelNode::Zone(member.id.clone())),
                    MinClearHeight | MinCorridorWidth => member.storey.clone().map(ModelNode::Room),
                })
                .filter(|node| planned.contains(node))
                .collect();
            step(ModelNode::Rule(id.clone()), nodes.into_iter().collect())
        })
        .collect()
}

/// 🖼️ The `View` nodes: a plan or ceiling plan view has the parents of the `Plan` node of its storey, a section or elevation the levels and the solids of its building, a camera none; a view whose building is missing has no node.
fn view_steps(snapshot: &ModelSnapshot, steps: &[InferenceStep<ModelNode>], on_storey: &dyn Fn(&String) -> bool) -> Vec<InferenceStep<ModelNode>> {
    let mut by_storey: BTreeMap<String, Vec<ModelNode>> = BTreeMap::new();
    let mut planned: BTreeMap<&String, &Vec<ModelNode>> = BTreeMap::new();
    for planned_step in steps {
        match &planned_step.key {
            ModelNode::Solid(key) => {
                if let Some(storey) = element_storey(snapshot, &key.id) {
                    by_storey.entry(storey).or_default().push(planned_step.key.clone());
                }
            }
            ModelNode::Plan(storey) => {
                planned.insert(storey, &planned_step.parents);
            }
            _ => {}
        }
    }
    snapshot
        .views
        .iter()
        .filter(|(_, view)| snapshot.buildings.contains_key(&view.building))
        .map(|(id, view)| {
            let parents = match view.kind {
                ViewKind::Plan | ViewKind::CeilingPlan => view.storey.as_ref().filter(|storey| on_storey(storey)).and_then(|storey| planned.get(storey)).map(|parents| (*parents).clone()).unwrap_or_default(),
                ViewKind::Section | ViewKind::Elevation => snapshot
                    .storeys
                    .iter()
                    .filter(|(_, storey)| storey.building == view.building)
                    .flat_map(|(storey, _)| std::iter::once(ModelNode::Storey(storey.clone())).chain(by_storey.get(storey).cloned().unwrap_or_default()))
                    .collect(),
                ViewKind::Orthographic | ViewKind::Perspective => Vec::new(),
            };
            step(ModelNode::View(id.clone()), parents)
        })
        .collect()
}

/// 🧭️ The storeys whose levels an element set of `storey` depends on: its own, then every storey a top constraint targets, sorted and unique.
pub fn level_storeys(snapshot: &ModelSnapshot, storey: &str) -> Vec<String> {
    super::super::super::bodies::level_storeys(snapshot, storey)
}

/// 🪜️ The storey an element stands on (a space, wall, … by its record; an opening by its host).
pub fn element_storey(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    snapshot
        .walls
        .get(id)
        .map(|row| row.storey.clone())
        .or_else(|| snapshot.curtain_walls.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.slabs.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.ceilings.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.roofs.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.columns.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.beams.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.railings.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.ramps.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.components.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.mep_elements.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.storey.clone()))
        .or_else(|| opening_storey(snapshot, id))
        .or_else(|| snapshot.wall_sweeps.get(id).and_then(|row| snapshot.walls.get(&row.host)).map(|wall| wall.storey.clone()))
}

/// 🧩️ The family nodes of the profile families the given profiles name (only families that exist).
pub fn family_parents(snapshot: &ModelSnapshot, profiles: Vec<&crate::Profile>) -> Vec<ModelNode> {
    let named: BTreeSet<&str> = profiles.into_iter().filter_map(families::family_of_profile).filter(|id| snapshot.families.contains_key(*id)).collect();
    named.into_iter().map(|id| ModelNode::Family(id.to_string())).collect()
}

/// 🛤️ The rail, post and baluster sections of a railing.
pub fn railing_profiles(railing: &crate::Railing) -> Vec<&crate::Profile> {
    [Some(&railing.profile), Some(&railing.post_profile), railing.baluster.as_ref().map(|row| &row.profile)].into_iter().flatten().collect()
}

/// 🪝️ The nodes the solid of a railing is computed from: the storey it stands on and, when it is hosted, the run of its stair or ramp host or the storey of its slab host.
pub fn railing_parents(snapshot: &ModelSnapshot, railing: &crate::Railing) -> Vec<ModelNode> {
    let mut parents = vec![ModelNode::Storey(railing.storey.clone())];
    let Some(host) = railing.host.as_ref() else { return parents };
    if let Some(stair) = snapshot.stairs.get(&host.element).filter(|stair| snapshot.storeys.contains_key(&stair.storey)) {
        parents.extend(storeys_of(snapshot, &stair.storey, Some(&stair.top)).into_iter().filter(|node| !parents.contains(node)).collect::<Vec<_>>());
        parents.push(ModelNode::StairRun(host.element.clone()));
    } else if let Some(ramp) = snapshot.ramps.get(&host.element).filter(|ramp| snapshot.storeys.contains_key(&ramp.storey)) {
        parents.extend(storeys_of(snapshot, &ramp.storey, Some(&ramp.top)).into_iter().filter(|node| !parents.contains(node)).collect::<Vec<_>>());
        parents.push(ModelNode::RampRun(host.element.clone()));
    } else if let Some(slab) = snapshot.slabs.get(&host.element).filter(|slab| snapshot.storeys.contains_key(&slab.storey)) {
        let node = ModelNode::Storey(slab.storey.clone());
        if !parents.contains(&node) {
            parents.push(node);
        }
    }
    parents
}

/// 🧊️ The steps of the `Solid` nodes of one family, in plan order.
pub fn solid_steps(snapshot: &ModelSnapshot, family: SolidFamily) -> Vec<InferenceStep<ModelNode>> {
    build(snapshot, super::kinds::closure(NodeKind::Solid.bit())).into_iter().filter(|step| matches!(&step.key, ModelNode::Solid(key) if key.family == family)).collect()
}
