//! 🪞️ The projection of node values into `ModelInference`: each kind of node is one entry of one field (`Storey(id)` → `storey_levels[id]`, `Solid` → `element_solids[id]` when it has triangles,
//! `Room(storey)` → the rooms of its spaces in `spaces`, `Quantity`/`Totals` → `quantities`, every `Diagnostics` node → the ordered `diagnostics`). The same functions fill a whole
//! inference ([`project`]) and update a held one entry by entry ([`apply`], [`retract`]).

use super::super::super::diagnostics::{ordered, Diagnostic};
use super::super::super::ModelInference;
use super::{Data, ModelNode, ModelValue, TotalsScope, Values};
use std::sync::Arc;

/// 🪞️ Writes the entry of one node into the inference. `Diagnostics` nodes are not written here: the ordered list is rebuilt from all of them by [`rebuild_diagnostics`].
pub fn apply(inference: &mut ModelInference, value: ModelValue) {
    let ModelValue { node, data } = value;
    match (node, data) {
        (ModelNode::Storey(id), Data::Level(level)) => {
            inference.storey_levels.insert(id, level);
        }
        (ModelNode::WallLayout(id), Data::Layout(layout)) => {
            inference.wall_layout.insert(id, Arc::unwrap_or_clone(layout));
        }
        (ModelNode::CurtainLayout(id), Data::Curtain(layout)) => {
            inference.curtain_layout.insert(id, layout);
        }
        (ModelNode::OpeningFrame(id), Data::Frame(frame)) => {
            inference.opening_frames.insert(id, Arc::unwrap_or_clone(frame));
        }
        (ModelNode::StairRun(id), Data::Run(run)) => {
            inference.stair_runs.insert(id, Arc::unwrap_or_clone(run));
        }
        (ModelNode::Solid(key), Data::Solid(entry)) => {
            let solid = Arc::unwrap_or_clone(entry).solid;
            if solid.is_empty() {
                inference.element_solids.remove(&key.id);
            } else {
                inference.element_solids.insert(key.id, solid);
            }
        }
        (ModelNode::Room(_), Data::Rooms(rooms)) => inference.spaces.extend(Arc::unwrap_or_clone(rooms)),
        (ModelNode::Plan(storey), Data::Plan(plan)) => {
            inference.plan_linework.insert(storey, Arc::unwrap_or_clone(plan));
        }
        (ModelNode::Quantity(id), Data::Quantity(quantity)) => match quantity {
            Some(quantity) => {
                inference.quantities.elements.insert(id, Arc::unwrap_or_clone(quantity));
            }
            None => {
                inference.quantities.elements.remove(&id);
            }
        },
        (ModelNode::Totals(scope), Data::Totals(totals)) => {
            let totals = Arc::unwrap_or_clone(totals);
            match scope {
                TotalsScope::Storey(id) => {
                    if totals.kinds.is_empty() {
                        inference.quantities.storeys.remove(&id);
                    } else {
                        inference.quantities.storeys.insert(id, totals);
                    }
                }
                TotalsScope::Building(id) => {
                    if totals.kinds.is_empty() {
                        inference.quantities.buildings.remove(&id);
                    } else {
                        inference.quantities.buildings.insert(id, totals);
                    }
                }
                TotalsScope::Project => inference.quantities.project = totals,
            }
        }
        _ => {}
    }
}

/// 🪞️ Removes the entry of a node that left the graph (or whose value is about to be replaced by a different set of entries).
pub fn retract(inference: &mut ModelInference, value: &ModelValue) {
    match (&value.node, &value.data) {
        (ModelNode::Storey(id), _) => {
            inference.storey_levels.remove(id);
        }
        (ModelNode::WallLayout(id), _) => {
            inference.wall_layout.remove(id);
        }
        (ModelNode::CurtainLayout(id), _) => {
            inference.curtain_layout.remove(id);
        }
        (ModelNode::OpeningFrame(id), _) => {
            inference.opening_frames.remove(id);
        }
        (ModelNode::StairRun(id), _) => {
            inference.stair_runs.remove(id);
        }
        (ModelNode::Solid(key), _) => {
            inference.element_solids.remove(&key.id);
        }
        (ModelNode::Room(_), Data::Rooms(rooms)) => rooms.keys().for_each(|id| {
            inference.spaces.remove(id);
        }),
        (ModelNode::Plan(storey), _) => {
            inference.plan_linework.remove(storey);
        }
        (ModelNode::Quantity(id), _) => {
            inference.quantities.elements.remove(id);
        }
        (ModelNode::Totals(TotalsScope::Storey(id)), _) => {
            inference.quantities.storeys.remove(id);
        }
        (ModelNode::Totals(TotalsScope::Building(id)), _) => {
            inference.quantities.buildings.remove(id);
        }
        (ModelNode::Totals(TotalsScope::Project), _) => inference.quantities.project = Default::default(),
        _ => {}
    }
}

/// 🪞️ The ordered findings of every `Diagnostics` node.
pub fn rebuild_diagnostics<'a>(values: impl IntoIterator<Item = &'a ModelValue>) -> Vec<Diagnostic> {
    ordered(values.into_iter().filter_map(|value| if let (ModelNode::Diagnostics(_), Data::Findings(found)) = (&value.node, &value.data) { Some(found.iter().cloned()) } else { None }).flatten().collect())
}

/// 🪞️ The whole inference of a run: every value written once.
pub fn project(values: Values) -> ModelInference {
    let mut inference = ModelInference { diagnostics: rebuild_diagnostics(values.values()), ..ModelInference::default() };
    for (_, value) in values {
        apply(&mut inference, value);
    }
    inference
}
