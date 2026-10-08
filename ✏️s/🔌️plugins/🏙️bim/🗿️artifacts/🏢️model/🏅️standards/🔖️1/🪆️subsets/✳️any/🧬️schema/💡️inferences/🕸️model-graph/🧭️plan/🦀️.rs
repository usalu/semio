//! 🧭️ The plan of the model graph: every node of the wanted kinds, parents before children, with the real edges of the design (`r7-design-model-graph.md` section 2).
//!
//! The indexes the edges need are built once per run, never per element: the walls of each storey and the walls each wall touches (one sweep per storey), the openings of each host, the elements of
//! each storey. The functions here only decide *which* nodes exist and *which* nodes are their parents; what a node computes is in `compute`.

use super::super::super::element_solids::fillers::family_of;
use super::super::super::element_solids::{SolidFamily, SolidKey};
use super::super::super::storey_levels::{constraint_storeys, stackings};
use super::super::super::wall_layout::{band_of, joins};
use super::{DiagnosticScope, ModelNode, NodeKind, TotalsScope};
use crate::{ModelSnapshot, TopConstraint};
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
pub fn build(snapshot: &ModelSnapshot, wanted: u32) -> Vec<InferenceStep<ModelNode>> {
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
    if has(NodeKind::CurtainLayout) {
        steps.extend(curtains.iter().map(|(id, curtain)| step(ModelNode::CurtainLayout((*id).clone()), storeys_of(snapshot, &curtain.storey, Some(&curtain.top)))));
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

    let filler = |id: &String| snapshot.openings.get(id).and_then(|opening| family_of(&opening.kind));
    let fillers: Vec<(&String, SolidFamily)> = snapshot.openings.keys().filter(|id| opening_storey(snapshot, id).is_some_and(|storey| on_storey(&storey))).filter_map(|id| filler(id).map(|family| (id, family))).collect();
    if has(NodeKind::Solid) {
        for (id, wall) in &walls {
            let mut parents = vec![ModelNode::Storey(wall.storey.clone()), ModelNode::WallLayout((*id).clone())];
            parents.extend(by_host.get(id.as_str()).into_iter().flatten().map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Wall, id)), parents));
        }
        for (id, curtain) in &curtains {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::CurtainWall, id)), vec![ModelNode::Storey(curtain.storey.clone()), ModelNode::CurtainLayout((*id).clone())]));
        }
        for (id, family) in &fillers {
            let storey = opening_storey(snapshot, id).expect("a filler stands on a storey");
            steps.push(step(ModelNode::Solid(SolidKey::of(*family, id)), vec![ModelNode::Storey(storey), ModelNode::OpeningFrame((*id).clone())]));
        }
        for (id, column) in snapshot.columns.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Column, id)), storeys_of(snapshot, &column.storey, Some(&column.top))));
        }
        for (id, beam) in snapshot.beams.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Beam, id)), vec![ModelNode::Storey(beam.storey.clone())]));
        }
        for (id, slab) in snapshot.slabs.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Slab, id)), vec![ModelNode::Storey(slab.storey.clone())]));
        }
        for (id, roof) in snapshot.roofs.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id)), vec![ModelNode::Storey(roof.storey.clone())]));
        }
        for (id, stair) in &stairs {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Stair, id)), vec![ModelNode::Storey(stair.storey.clone()), ModelNode::StairRun((*id).clone())]));
        }
        for (id, railing) in snapshot.railings.iter().filter(|(_, row)| on_storey(&row.storey)) {
            steps.push(step(ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id)), vec![ModelNode::Storey(railing.storey.clone())]));
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
    if has(NodeKind::Plan) {
        for storey in snapshot.storeys.keys() {
            let mut parents: Vec<ModelNode> = level_storeys(snapshot, storey).into_iter().map(ModelNode::Storey).collect();
            parents.extend(wall_layouts(storey));
            parents.extend(curtains.iter().filter(|(_, curtain)| &curtain.storey == storey).map(|(id, _)| ModelNode::CurtainLayout((*id).clone())));
            let hosted = walls.iter().filter(|(_, wall)| &wall.storey == storey).flat_map(|(id, _)| by_host.get(id.as_str()).into_iter().flatten());
            parents.extend(hosted.map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| &stair.storey == storey).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            if room_storeys.contains(storey) {
                parents.push(ModelNode::Room(storey.clone()));
            }
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
        quantities.extend(snapshot.slabs.keys().map(|id| (id.clone(), Vec::new())));
        for (id, roof) in &snapshot.roofs {
            quantities.push((id.clone(), if on_storey(&roof.storey) { vec![ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id))] } else { Vec::new() }));
        }
        for (id, column) in snapshot.columns.iter().filter(|(_, row)| on_storey(&row.storey)) {
            quantities.push((id.clone(), storeys_of(snapshot, &column.storey, Some(&column.top))));
        }
        quantities.extend(snapshot.beams.keys().map(|id| (id.clone(), Vec::new())));
        for id in snapshot.openings.keys() {
            let mut parents = vec![ModelNode::OpeningFrame(id.clone())];
            parents.extend(fillers.iter().filter(|(other, _)| *other == id).map(|(_, family)| ModelNode::Solid(SolidKey::of(*family, id))));
            quantities.push((id.clone(), parents));
        }
        for (id, _) in &stairs {
            quantities.push(((*id).clone(), vec![ModelNode::StairRun((*id).clone()), ModelNode::Solid(SolidKey::of(SolidFamily::Stair, id))]));
        }
        for (id, railing) in &snapshot.railings {
            quantities.push((id.clone(), if on_storey(&railing.storey) { vec![ModelNode::Solid(SolidKey::of(SolidFamily::Railing, id))] } else { Vec::new() }));
        }
        for (id, space) in snapshot.spaces.iter().filter(|(_, space)| room_storeys.contains(&space.storey)) {
            quantities.push((id.clone(), vec![ModelNode::Room(space.storey.clone())]));
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
    }

    if has(NodeKind::Diagnostics) {
        let buildings: BTreeSet<&String> = snapshot.storeys.values().map(|storey| &storey.building).collect();
        for storey in snapshot.storeys.keys() {
            let mut parents: Vec<ModelNode> = level_storeys(snapshot, storey).into_iter().map(ModelNode::Storey).collect();
            parents.extend(wall_layouts(storey));
            let hosted = walls.iter().filter(|(_, wall)| &wall.storey == storey).map(|(id, _)| id.as_str()).chain(curtains.iter().filter(|(_, curtain)| &curtain.storey == storey).map(|(id, _)| id.as_str()));
            parents.extend(hosted.flat_map(|host| by_host.get(host).into_iter().flatten()).map(|opening| ModelNode::OpeningFrame((*opening).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| &stair.storey == storey).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            if room_storeys.contains(storey) {
                parents.push(ModelNode::Room(storey.clone()));
            }
            parents.extend(snapshot.roofs.iter().filter(|(_, roof)| &roof.storey == storey).map(|(id, _)| ModelNode::Solid(SolidKey::of(SolidFamily::Roof, id))));
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Storey(storey.clone())), parents));
        }
        for building in buildings {
            let members: BTreeSet<&String> = snapshot.storeys.iter().filter(|(_, storey)| &storey.building == building).map(|(id, _)| id).collect();
            let mut parents: Vec<ModelNode> = members.iter().map(|id| ModelNode::Storey((*id).clone())).collect();
            parents.extend(walls.iter().filter(|(_, wall)| members.contains(&wall.storey)).map(|(id, _)| ModelNode::WallLayout((*id).clone())));
            parents.extend(stairs.iter().filter(|(_, stair)| members.contains(&stair.storey)).map(|(id, _)| ModelNode::StairRun((*id).clone())));
            steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Building(building.clone())), parents));
        }
        steps.push(step(ModelNode::Diagnostics(DiagnosticScope::Model), Vec::new()));
    }
    steps
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
        .or_else(|| snapshot.roofs.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.columns.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.beams.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.stairs.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.railings.get(id).map(|row| row.storey.clone()))
        .or_else(|| snapshot.spaces.get(id).map(|row| row.storey.clone()))
        .or_else(|| opening_storey(snapshot, id))
}

/// 🧊️ The steps of the `Solid` nodes of one family, in plan order.
pub fn solid_steps(snapshot: &ModelSnapshot, family: SolidFamily) -> Vec<InferenceStep<ModelNode>> {
    build(snapshot, super::kinds::closure(NodeKind::Solid.bit())).into_iter().filter(|step| matches!(&step.key, ModelNode::Solid(key) if key.family == family)).collect()
}
