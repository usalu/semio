//! 🪞️ The projection of node values into `ModelInference`: each kind of node is one entry of one field (`Storey(id)` → `storey_levels[id]`, `Solid` → `element_solids[id]` when it has triangles,
//! `Room(storey)` → the rooms of its spaces in `spaces`, `Quantity`/`Totals` → `quantities`, `Zone` → `zone_totals`, `Scheme` → `scheme_totals`, every `Diagnostics` node → the ordered `diagnostics`, `DiagnosticIndex` → `diagnostic_index`). The same functions fill a whole
//! inference ([`project`]) and update a held one entry by entry ([`apply`], [`retract`]).

use super::super::super::diagnostics::{ordered, Diagnostic};
use super::super::super::ModelInference;
use super::{Data, ModelNode, ModelValue, TotalsScope, Values};
use std::sync::Arc;

/// 🪞️ Writes the entry of one node into the inference. `Diagnostics` nodes are not written here: the ordered list is rebuilt from all of them by [`rebuild_diagnostics`].
pub fn apply(inference: &mut ModelInference, value: ModelValue) {
    let ModelValue { node, data } = value;
    match (node, data) {
        (ModelNode::OptionScope, Data::OptionScope(scope)) => inference.option_scope = Arc::unwrap_or_clone(scope),
        (ModelNode::StructuralAnalysis, Data::Structure(value)) => {
            let analysis = Arc::unwrap_or_clone(value);
            inference.analytical_members = analysis.members.clone();
            inference.structural_analysis = analysis;
        }
        (ModelNode::Storey(id), Data::Level(level)) => {
            inference.storey_levels.insert(id, level);
        }
        (ModelNode::WallLayout(id), Data::Layout(layout)) => {
            inference.wall_layout.insert(id, Arc::unwrap_or_clone(layout));
        }
        (ModelNode::CurtainLayout(id), Data::Curtain(layout)) => {
            inference.curtain_layout.insert(id, Arc::unwrap_or_clone(layout));
        }
        (ModelNode::OpeningFrame(id), Data::Frame(frame)) => {
            inference.opening_frames.insert(id, Arc::unwrap_or_clone(frame));
        }
        (ModelNode::StairRun(id), Data::Run(run)) => {
            inference.stair_runs.insert(id, Arc::unwrap_or_clone(run));
        }
        (ModelNode::RampRun(id), Data::RampRun(run)) => {
            inference.ramp_runs.insert(id, Arc::unwrap_or_clone(run));
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
        (ModelNode::Annotation(storey), Data::Annotations(set)) => {
            inference.annotations.insert(storey, Arc::unwrap_or_clone(set));
        }
        (ModelNode::Properties(id), Data::Properties(properties)) => {
            inference.effective_properties.insert(id, Arc::unwrap_or_clone(properties));
        }
        (ModelNode::Plan(storey), Data::Plan(plan)) => {
            inference.plan_linework.insert(storey, Arc::unwrap_or_clone(plan));
        }
        (ModelNode::View(id), Data::View(view)) => {
            inference.view_linework.insert(id, Arc::unwrap_or_clone(view));
        }
        (ModelNode::Sheet(id), Data::Sheet(layout)) => {
            inference.sheet_layouts.insert(id, Arc::unwrap_or_clone(layout));
        }
        (ModelNode::Quantity(id), Data::Quantity(quantity)) => match quantity {
            Some(quantity) => {
                inference.quantities.elements.insert(id, Arc::unwrap_or_clone(quantity));
            }
            None => {
                inference.quantities.elements.remove(&id);
            }
        },
        (ModelNode::Schedule(id), Data::Schedule(table)) => {
            inference.schedules.insert(id, Arc::unwrap_or_clone(table));
        }
        (ModelNode::Zone(id), Data::Zone(totals)) => {
            inference.zone_totals.insert(id, Arc::unwrap_or_clone(totals));
        }
        (ModelNode::Envelope(id), Data::Envelope(envelope)) => {
            inference.energy_envelopes.insert(id, Arc::unwrap_or_clone(envelope));
        }
        (ModelNode::EnergyTotals(scope), Data::EnergyTotals(totals)) => {
            inference.energy_totals.insert(scope.key(), Arc::unwrap_or_clone(totals));
        }
        (ModelNode::Scheme(id), Data::Scheme(totals)) => {
            inference.scheme_totals.insert(id, Arc::unwrap_or_clone(totals));
        }
        (ModelNode::ClashSet(id), Data::Clashes(found)) => {
            inference.clash_sets.insert(id, Arc::unwrap_or_clone(found));
        }
        (ModelNode::Rule(id), Data::Rule(found)) => {
            inference.rule_results.insert(id, Arc::unwrap_or_clone(found));
        }
        (ModelNode::Family(id), Data::Family(family)) => {
            inference.families.insert(id, Arc::unwrap_or_clone(family));
        }
        (ModelNode::Component(id), Data::Component(entry)) => {
            inference.components.insert(id, Arc::unwrap_or_clone(entry).value);
        }
        (ModelNode::Mep(id), Data::Mep(value)) => {
            inference.mep.insert(id, Arc::unwrap_or_clone(value));
        }
        (ModelNode::PhaseVisibility(storey), Data::Phases(visibility)) => {
            inference.phase_visibility.insert(storey, Arc::unwrap_or_clone(visibility));
        }
        (ModelNode::DiagnosticIndex, Data::Index(index)) => inference.diagnostic_index = Arc::unwrap_or_clone(index),
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
        (ModelNode::StructuralAnalysis, _) => { inference.analytical_members.clear(); inference.structural_analysis = Default::default(); }
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
        (ModelNode::RampRun(id), _) => {
            inference.ramp_runs.remove(id);
        }
        (ModelNode::Solid(key), _) => {
            inference.element_solids.remove(&key.id);
        }
        (ModelNode::Room(_), Data::Rooms(rooms)) => rooms.keys().for_each(|id| {
            inference.spaces.remove(id);
        }),
        (ModelNode::Annotation(storey), _) => {
            inference.annotations.remove(storey);
        }
        (ModelNode::Properties(id), _) => {
            inference.effective_properties.remove(id);
        }
        (ModelNode::Plan(storey), _) => {
            inference.plan_linework.remove(storey);
        }
        (ModelNode::View(id), _) => {
            inference.view_linework.remove(id);
        }
        (ModelNode::Sheet(id), _) => {
            inference.sheet_layouts.remove(id);
        }
        (ModelNode::Quantity(id), _) => {
            inference.quantities.elements.remove(id);
        }
        (ModelNode::Schedule(id), _) => {
            inference.schedules.remove(id);
        }
        (ModelNode::Zone(id), _) => {
            inference.zone_totals.remove(id);
        }
        (ModelNode::Envelope(id), _) => {
            inference.energy_envelopes.remove(id);
        }
        (ModelNode::EnergyTotals(scope), _) => {
            inference.energy_totals.remove(&scope.key());
        }
        (ModelNode::Scheme(id), _) => {
            inference.scheme_totals.remove(id);
        }
        (ModelNode::ClashSet(id), _) => {
            inference.clash_sets.remove(id);
        }
        (ModelNode::Rule(id), _) => {
            inference.rule_results.remove(id);
        }
        (ModelNode::Family(id), _) => {
            inference.families.remove(id);
        }
        (ModelNode::Component(id), _) => {
            inference.components.remove(id);
        }
        (ModelNode::Mep(id), _) => {
            inference.mep.remove(id);
        }
        (ModelNode::PhaseVisibility(storey), _) => {
            inference.phase_visibility.remove(storey);
        }
        (ModelNode::Totals(TotalsScope::Storey(id)), _) => {
            inference.quantities.storeys.remove(id);
        }
        (ModelNode::Totals(TotalsScope::Building(id)), _) => {
            inference.quantities.buildings.remove(id);
        }
        (ModelNode::Totals(TotalsScope::Project), _) => inference.quantities.project = Default::default(),
        (ModelNode::DiagnosticIndex, _) => inference.diagnostic_index = Default::default(),
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
