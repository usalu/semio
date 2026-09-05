//! 🧬️ Exact retained recipes for the live delete, disconnect, move, and widget replacement lanes.

use super::SceneCopy;
use crate::artifacts::flow::retirement::MutationRetirementFactory;
use super::super::{bytes::{Equality, TextCopy}, Owner, Retirement};
use super::super::super::{flow_widget_id, FlowMutation, FlowWorkingScene};
use crate::artifacts::flow::schema::mutations::{connect_widgets::ConnectWidgets, create_widget::CreateWidget, move_widgets::MoveWidgets, replace_widget::ReplaceWidget};
use flow::{FlowLayoutEntry, Widget, WidgetLayout};
use flow::retained::{FlowCopyAllocationBudget, FlowWidgetCopy};
use std::{mem::ManuallyDrop, sync::Arc};
use store::os_dsl::schema::ordered::{Grant, RetirementStep, Step, UpdateCursor};

//#region 🧬️Recipe
struct RecipeState {
    source: Option<Arc<FlowWorkingScene>>, mutation: Option<Arc<FlowMutation>>,
    copy: Option<SceneCopy>, replacement: Option<FlowWidgetCopy<FlowMutation>>, previous: Option<Widget>, scene: Option<FlowWorkingScene>,
    inverse: Option<Vec<FlowMutation>>, inverse_entries: Option<Vec<FlowLayoutEntry>>,
    text: Option<TextCopy>, key: Option<String>, inverse_id: Option<String>, update: Option<UpdateCursor<WidgetLayout>>,
    equality: Equality, phase: u8, scan: usize, target: usize, shift: usize, entry: usize, removed_edges: usize, edge_side: bool,
    retirement: Retirement, closing: bool,
}

pub(in super::super::super) struct Recipe { state: ManuallyDrop<RecipeState> }

impl Recipe {
    pub(in super::super::super) fn new(source: Arc<FlowWorkingScene>, mutation: Arc<FlowMutation>) -> Self {
        Self { state: ManuallyDrop::new(RecipeState {
            source: Some(source), mutation: Some(mutation), copy: None, replacement: None, previous: None, scene: None,
            inverse: Some(Vec::new()), inverse_entries: Some(Vec::new()), text: None, key: None, inverse_id: None, update: None,
            equality: Equality::default(), phase: 0, scan: 0, target: 0, shift: 0, entry: 0, removed_edges: 0, edge_side: false,
            retirement: Retirement::default(), closing: false,
        }) }
    }

    pub(in super::super::super) fn supported(mutation: &FlowMutation) -> bool {
        matches!(mutation, FlowMutation::DeleteWidget(_) | FlowMutation::DisconnectWidgets(_) | FlowMutation::MoveWidgets(_) | FlowMutation::ReplaceWidget(_))
    }

    pub(in super::super::super) fn complete(&self) -> bool { self.state.phase == 30 && self.state.scene.is_some() }

    pub(in super::super::super) fn take(&mut self) -> Option<(FlowWorkingScene, Vec<FlowMutation>)> {
        if !self.complete() { return None; }
        Some((self.state.scene.take()?, self.state.inverse.take()?))
    }

    pub(in super::super::super) fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<Option<usize>, String> {
        use store::SnapshotRetirementStep as SnapshotStep;
        if self.state.closing || !grant.permits_one() { return Ok(None); }
        let state = &mut *self.state;
        let mutation = state.mutation.as_ref().ok_or("Flow recipe lost mutation")?;
        if state.phase == 0 {
            if !Self::supported(mutation) { return Err("Flow mutation has no retained recipe".into()); }
            let source = state.source.as_ref().ok_or("Flow recipe lost source")?;
            if source.widgets.len() > 256 || source.synapses.len() > 256 || source.layout.len() > 256 { return Err("Flow recipe source exceeds its admitted item envelope".into()); }
            state.inverse.as_mut().unwrap().try_reserve_exact(source.synapses.len() + 2).map_err(|_| "Flow inverse allocation failed")?;
            state.phase = 1; return Ok(Some(0));
        }
        if state.phase == 1 {
            let count = match &**mutation { FlowMutation::MoveWidgets(payload) => payload.entries.len(), _ => 0 };
            if count > 256 { return Err("Flow recipe move count exceeds admitted envelope".into()); }
            state.inverse_entries.as_mut().unwrap().try_reserve_exact(count).map_err(|_| "Flow inverse layout allocation failed")?;
            state.copy = Some(SceneCopy::new(Arc::clone(state.source.as_ref().unwrap()))); state.phase = 2; return Ok(Some(0));
        }
        if state.phase == 2 {
            let copy = state.copy.as_mut().unwrap();
            if copy.complete() { state.scene = copy.take(); copy.begin_close(); state.phase = 3; return Ok(Some(0)); }
            return copy.advance(1, grant.maximum_bytes);
        }
        if state.phase == 3 {
            let copy = state.copy.as_mut().unwrap();
            match copy.close_step(1, grant.maximum_bytes)? {
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    if !copy.terminal_is_empty() { return Err("Flow recipe copy closed with live owners".into()); }
                    state.copy = None; state.phase = 4;
                }
                semio_framework_job::InteractiveJobCloseStep::Pending { released_bytes, .. } => return Ok(Some(released_bytes)),
                semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(None),
            }
            return Ok(Some(0));
        }
        let scene = state.scene.as_mut().ok_or("Flow recipe lost assembled scene")?;
        let target_id = match &**mutation {
            FlowMutation::DeleteWidget(payload) => payload.id.as_str(),
            FlowMutation::DisconnectWidgets(payload) => payload.id.as_str(),
            FlowMutation::ReplaceWidget(payload) => payload.id.as_str(),
            FlowMutation::MoveWidgets(payload) => match payload.entries.get(state.entry) {
                Some(entry) => entry.id.as_str(),
                None if state.phase == 4 => {
                    if payload.entries.is_empty() { return Err("Flow move recipe contains no entries".into()); }
                    state.inverse.as_mut().unwrap().push(FlowMutation::MoveWidgets(MoveWidgets { entries: state.inverse_entries.take().unwrap() }));
                    state.phase = 30; return Ok(Some(0));
                }
                None => return Err("Flow move recipe cursor escaped its entries".into()),
            },
            _ => return Err("Flow recipe mutation changed".into()),
        };
        match state.phase {
            4 => {
                let candidate = if matches!(&**mutation, FlowMutation::DisconnectWidgets(_)) {
                    scene.synapses.get(state.scan).map(|edge| edge.id.as_str())
                } else { scene.widgets.get(state.scan).map(flow_widget_id) };
                let candidate = candidate.ok_or("Flow recipe target is missing")?;
                let (equal, bytes) = state.equality.advance(candidate, target_id, grant.maximum_bytes);
                if let Some(equal) = equal {
                    state.equality = Equality::default();
                    if !equal { state.scan += 1; }
                    else { state.target = state.scan; state.shift = state.scan; state.phase = match &**mutation {
                        FlowMutation::DeleteWidget(_) => 5, FlowMutation::DisconnectWidgets(_) => 10,
                        FlowMutation::MoveWidgets(_) => 20, FlowMutation::ReplaceWidget(_) => 15, _ => unreachable!(),
                    }; }
                }
                return Ok(Some(bytes));
            }
            5 => {
                let last = scene.widgets.len() - 1; scene.widgets.swap(state.target, last);
                let widget = scene.widgets.pop().unwrap();
                state.inverse.as_mut().unwrap().push(FlowMutation::CreateWidget(CreateWidget { index: state.target, widget }));
                state.phase = 6;
            }
            6 => {
                if state.shift + 1 < scene.widgets.len() { scene.widgets.swap(state.shift, state.shift + 1); state.shift += 1; }
                else { state.phase = 20; }
            }
            8 => {
                let Some(edge) = scene.synapses.get(state.scan) else { state.phase = 30; return Ok(Some(0)); };
                let candidate = if state.edge_side { edge.to.as_str() } else { edge.from.as_str() };
                let (equal, bytes) = state.equality.advance(candidate, target_id, grant.maximum_bytes);
                if let Some(equal) = equal {
                    state.equality = Equality::default();
                    if equal { state.target = state.scan; state.shift = state.scan; state.phase = 10; state.edge_side = false; }
                    else if state.edge_side { state.edge_side = false; state.scan += 1; }
                    else { state.edge_side = true; }
                }
                return Ok(Some(bytes));
            }
            10 => {
                let last = scene.synapses.len() - 1; scene.synapses.swap(state.target, last);
                let edge = scene.synapses.pop().unwrap();
                state.inverse.as_mut().unwrap().push(FlowMutation::ConnectWidgets(ConnectWidgets {
                    index: state.target + state.removed_edges, id: edge.id, from: edge.from, from_port: edge.from_port, to: edge.to, to_port: edge.to_port,
                }));
                state.removed_edges += 1; state.phase = 11;
            }
            11 => {
                if state.shift + 1 < scene.synapses.len() { scene.synapses.swap(state.shift, state.shift + 1); state.shift += 1; }
                else { state.phase = if matches!(&**mutation, FlowMutation::DeleteWidget(_)) { 8 } else { 30 }; }
            }
            15 => {
                state.replacement = Some(FlowWidgetCopy::new(Arc::clone(mutation), 0, |mutation, _| match mutation { FlowMutation::ReplaceWidget(payload) => Some(&payload.widget), _ => None }, Arc::new(MutationRetirementFactory), FlowCopyAllocationBudget::new(16 * 1024 * 1024, 16 * 1024 * 1024)));
                state.phase = 16;
            }
            16 => {
                let copy = state.replacement.as_mut().unwrap();
                if copy.complete() {
                    state.previous = Some(std::mem::replace(&mut scene.widgets[state.target], copy.take().unwrap()));
                    copy.begin_close(); state.phase = 17;
                } else { return copy.advance(1, grant.maximum_bytes); }
            }
            17 => {
                let copy = state.replacement.as_mut().unwrap();
                match copy.close_step(1, grant.maximum_bytes)? {
                    SnapshotStep::Complete => {
                        if !copy.terminal_is_empty() { return Err("Flow replacement copy closed with owners".into()); }
                        state.replacement = None; state.phase = 18;
                    }
                    SnapshotStep::Pending { released_bytes, .. } => return Ok(Some(released_bytes)),
                    SnapshotStep::Blocked => return Ok(None),
                }
            }
            18 => {
                let text = state.text.get_or_insert_with(TextCopy::default);
                let bytes = text.advance(target_id, grant.maximum_bytes)?;
                if text.complete() { state.inverse_id = text.take(); state.text = None; state.phase = 19; }
                return Ok(bytes);
            }
            19 => {
                state.inverse.as_mut().unwrap().push(FlowMutation::ReplaceWidget(ReplaceWidget { id: state.inverse_id.take().unwrap(), widget: state.previous.take().unwrap() }));
                state.phase = 30;
            }
            20 | 21 => {
                let text = state.text.get_or_insert_with(TextCopy::default);
                let bytes = text.advance(target_id, grant.maximum_bytes)?;
                if text.complete() {
                    let value = text.take().unwrap(); state.text = None;
                    if state.phase == 20 { state.key = Some(value); state.phase = 21; }
                    else { state.inverse_id = Some(value); state.phase = 22; }
                }
                return Ok(bytes);
            }
            22 => {
                let layout = match &**mutation {
                    FlowMutation::MoveWidgets(payload) => {
                        let layout = payload.entries[state.entry].layout.clone();
                        if layout.as_ref().is_some_and(|value| !value.x.is_finite() || !value.y.is_finite()) { return Err("Flow layout has non-finite position".into()); }
                        layout
                    }
                    _ => None,
                };
                let map = std::mem::take(&mut scene.layout); let key = state.key.take().unwrap();
                state.update = Some(match layout { Some(layout) => map.begin_set(key, layout), None => map.begin_remove(key) }); state.phase = 23;
            }
            23 => {
                let update = state.update.as_mut().unwrap();
                match update.advance(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                    Step::Complete => {
                        scene.layout = update.take_result().ok_or("Flow map update lost output")?;
                        let previous = update.take_removed().map(|value| (*value).clone());
                        let id = state.inverse_id.take().unwrap();
                        if matches!(&**mutation, FlowMutation::MoveWidgets(_)) { state.inverse_entries.as_mut().unwrap().push(FlowLayoutEntry { id, layout: previous }); }
                        else if previous.is_some() { state.inverse.as_mut().unwrap().push(FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id, layout: previous }] })); }
                        else { state.retirement.push(Owner::Bytes(id.into_bytes())); }
                        update.begin_close(); state.phase = 24;
                    }
                    Step::Progress { completed_bytes, .. } => return Ok(Some(completed_bytes)),
                    Step::Blocked => return Ok(None),
                }
            }
            24 => {
                let update = state.update.as_mut().unwrap();
                match update.close_step(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                    RetirementStep::Complete => {
                        if !update.terminal_is_empty() { return Err("Flow map update closed with owners".into()); }
                        state.update = None; state.scan = 0;
                        if matches!(&**mutation, FlowMutation::MoveWidgets(_)) { state.entry += 1; state.phase = 4; } else { state.phase = 8; }
                    }
                    RetirementStep::Progress { released_bytes, .. } => return Ok(Some(released_bytes)),
                    RetirementStep::OwnedValue(_) => {}
                    RetirementStep::Blocked => return Ok(None),
                }
            }
            30 => {}
            _ => return Err("Flow recipe phase is invalid".into()),
        }
        Ok(Some(0))
    }

    pub(in super::super::super) fn begin_close(&mut self) {
        self.state.closing = true;
        if let Some(copy) = self.state.copy.as_mut() { copy.begin_close(); }
        if let Some(copy) = self.state.replacement.as_mut() { copy.begin_close(); }
        if let Some(update) = self.state.update.as_mut() { update.begin_close(); }
    }

    pub(in super::super::super) fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Close;
        if !self.state.closing || !grant.permits_one() { return Ok(Close::Blocked); }
        let state = &mut *self.state;
        if !state.retirement.is_empty() { return store::ErasedSnapshotRetirement::close_step(&mut state.retirement, 1, grant.maximum_bytes); }
        if let Some(copy) = state.copy.as_mut() {
            return Ok(match copy.close_step(1, grant.maximum_bytes)? {
                semio_framework_job::InteractiveJobCloseStep::Complete => { if !copy.terminal_is_empty() { return Err("Flow copy close retained owners".into()); } state.copy = None; Close::Pending { released_items: 1, released_bytes: 0 } }
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => Close::Pending { released_items, released_bytes },
                semio_framework_job::InteractiveJobCloseStep::Blocked => Close::Blocked,
            });
        }
        if let Some(copy) = state.replacement.as_mut() {
            let step = copy.close_step(1, grant.maximum_bytes)?;
            if step == Close::Complete { if !copy.terminal_is_empty() { return Err("Flow replacement close retained owners".into()); } state.replacement = None; return Ok(Close::Pending { released_items: 1, released_bytes: 0 }); }
            return Ok(step);
        }
        if let Some(update) = state.update.as_mut() {
            return Ok(match update.close_step(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                RetirementStep::Complete => { if !update.terminal_is_empty() { return Err("Flow map close retained owners".into()); } state.update = None; Close::Pending { released_items: 1, released_bytes: 0 } }
                RetirementStep::Progress { released_items, released_bytes } => Close::Pending { released_items, released_bytes },
                RetirementStep::OwnedValue(_) => Close::Pending { released_items: 1, released_bytes: 0 },
                RetirementStep::Blocked => Close::Blocked,
            });
        }
        if let Some(text) = state.text.take() { text.retire(&mut state.retirement); }
        else if let Some(key) = state.key.take().or_else(|| state.inverse_id.take()) { state.retirement.push(Owner::Bytes(key.into_bytes())); }
        else if let Some(inverse) = state.inverse.take() { state.retirement.push(Owner::Mutations(inverse)); }
        else if let Some(entries) = state.inverse_entries.take() { state.retirement.push(Owner::Layout(entries)); }
        else if let Some(previous) = state.previous.take() { state.retirement.push(Owner::Widget(previous)); }
        else if let Some(scene) = state.scene.take() { state.retirement.push(Owner::Scene(scene)); }
        else if let Some(source) = state.source.take() { if let Some(source) = Arc::into_inner(source) { state.retirement.push(Owner::Scene(source)); } }
        else if let Some(mutation) = state.mutation.take() { if let Some(mutation) = Arc::into_inner(mutation) { state.retirement.push(Owner::Mutation(mutation)); } }
        else { return Ok(Close::Complete); }
        Ok(Close::Pending { released_items: 1, released_bytes: 0 })
    }

    pub(in super::super::super) fn terminal_is_empty(&self) -> bool {
        let state = &*self.state;
        state.closing && state.source.is_none() && state.mutation.is_none() && state.copy.is_none() && state.replacement.is_none() && state.previous.is_none() && state.scene.is_none()
            && state.inverse.is_none() && state.inverse_entries.is_none() && state.text.is_none() && state.key.is_none() && state.inverse_id.is_none() && state.update.is_none() && state.retirement.is_empty()
    }
}

impl Drop for Recipe {
    fn drop(&mut self) {
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.state); } }
        else if !std::thread::panicking() { panic!("Flow recipe must close before drop"); }
    }
}
//#endregion 🧬️Recipe

//#region 🧪️RecipeLaws
#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use store::SnapshotRetirementStep;

    pub(in super::super) fn source(label: &str) -> FlowWorkingScene {
        FlowWorkingScene {
            widgets: ["a", "b", "c"].into_iter().map(|id| Widget::InputSlider { id: id.into(), label: if id == "b" { label.into() } else { id.into() }, value: 1.0, min: 0.0, max: 10.0, step: 1.0 }).collect(),
            synapses: [("ab", "a", "b"), ("bc", "b", "c"), ("ac", "a", "c")].into_iter().map(|(id, from, to)| flow::SynapseSpec { id: id.into(), from: from.into(), from_port: "value".into(), to: to.into(), to_port: "value".into() }).collect(),
            layout: [("b".to_owned(), WidgetLayout { x: 1.0, y: 2.0 })].into_iter().collect(),
        }
    }

    fn close(recipe: &mut Recipe, grant: store::ArtifactStoreOneItemGrant) -> usize {
        recipe.begin_close(); let mut bytes = 0;
        for _ in 0..500_000 {
            match recipe.close_step(grant).unwrap() {
                SnapshotRetirementStep::Complete => { assert!(recipe.terminal_is_empty()); return bytes; }
                SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes); bytes += released_bytes; }
                SnapshotRetirementStep::Blocked => panic!("positive recipe close grant cannot block"),
            }
        }
        panic!("Flow recipe did not reach terminal emptiness");
    }

    fn retire(scene: FlowWorkingScene, inverse: Vec<FlowMutation>, grant: store::ArtifactStoreOneItemGrant) {
        let mut retirement = Retirement::default(); retirement.push(Owner::Scene(scene)); retirement.push(Owner::Mutations(inverse));
        for _ in 0..500_000 {
            if store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, grant.maximum_bytes).unwrap() == SnapshotRetirementStep::Complete { return; }
        }
        panic!("Flow recipe output did not retire");
    }

    fn apply_inverse(post: &mut serde_json::Value, inverse: &serde_json::Value) {
        for mutation in inverse.as_array().unwrap() {
            match mutation["mutation"].as_str().unwrap() {
                "createWidget" => post["widgets"].as_array_mut().unwrap().insert(mutation["index"].as_u64().unwrap() as usize, mutation["widget"].clone()),
                "connectWidgets" => {
                    let mut edge = mutation.as_object().unwrap().clone(); edge.remove("mutation"); let index = edge.remove("index").unwrap().as_u64().unwrap() as usize;
                    post["synapses"].as_array_mut().unwrap().insert(index, serde_json::Value::Object(edge));
                }
                "moveWidgets" => for entry in mutation["entries"].as_array().unwrap() {
                    let id = entry["id"].as_str().unwrap();
                    if entry["layout"].is_null() { post["layout"].as_object_mut().unwrap().remove(id); }
                    else { post["layout"][id] = entry["layout"].clone(); }
                },
                "replaceWidget" => {
                    let widget = post["widgets"].as_array_mut().unwrap().iter_mut().find(|widget| widget["id"] == mutation["id"]).unwrap(); *widget = mutation["widget"].clone();
                }
                _ => panic!("unexpected recipe inverse"),
            }
        }
    }

    #[test]
    fn retained_recipes_match_immer_fixture_and_exact_inverse_at_one_and_production_bytes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🧬️artifact-recipes.json")).unwrap();
        let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
        for bytes in fixture["grants"].as_array().unwrap() {
            let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
            for row in fixture["cases"].as_array().unwrap() {
                let root = Arc::new(source(&label)); let baseline = serde_json::Value::from(dsl::ToValue::to_value(&*root)); let weak = Arc::downgrade(&root);
                let mutation: FlowMutation = dsl::FromValue::from_value(dsl::DslValue::from(row["mutation"].clone())).unwrap();
                let mut recipe = Recipe::new(root, Arc::new(mutation));
                for _ in 0..500_000 { if recipe.complete() { break; } assert!(recipe.advance(grant).unwrap().unwrap() <= grant.maximum_bytes); }
                assert!(recipe.complete()); let (post, inverse) = recipe.take().unwrap();
                let mut json = serde_json::Value::from(dsl::ToValue::to_value(&post)); let inverse_json = serde_json::Value::from(dsl::ToValue::to_value(&inverse));
                assert_eq!(json["widgets"].as_array().unwrap().iter().map(|widget| widget["id"].clone()).collect::<Vec<_>>(), *row["widgets"].as_array().unwrap());
                assert_eq!(json["synapses"].as_array().unwrap().iter().map(|edge| edge["id"].clone()).collect::<Vec<_>>(), *row["synapses"].as_array().unwrap());
                assert_eq!(inverse_json.as_array().unwrap().iter().map(|mutation| mutation["mutation"].clone()).collect::<Vec<_>>(), *row["inverseKinds"].as_array().unwrap());
                apply_inverse(&mut json, &inverse_json); assert_eq!(json, baseline);
                close(&mut recipe, grant); assert!(weak.upgrade().is_none()); retire(post, inverse, grant);
            }
        }
    }

    #[test]
    fn recipe_cancellation_retires_every_partial_frontier_without_losing_original_root() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🧬️artifact-recipes.json")).unwrap();
        let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
        for bytes in fixture["grants"].as_array().unwrap() {
            let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
            for after in fixture["cancelAfterSteps"].as_array().unwrap() {
                let root = Arc::new(source(&label)); let weak = Arc::downgrade(&root);
                let mutation = dsl::FromValue::from_value(dsl::DslValue::from(fixture["cases"][0]["mutation"].clone())).unwrap();
                let mut recipe = Recipe::new(root, Arc::new(mutation));
                for _ in 0..after.as_u64().unwrap() { if recipe.complete() { break; } recipe.advance(grant).unwrap(); }
                assert!(weak.upgrade().is_some()); let retired = close(&mut recipe, grant); assert!(weak.upgrade().is_none());
                if after.as_u64().unwrap() == 0 { assert_eq!(retired, 4849); }
            }
        }
    }
}
//#endregion 🧪️RecipeLaws
