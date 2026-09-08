//! 🧬️ Exact retained recipes for the live delete, disconnect, move, and widget replacement lanes.

use super::super::super::{flow_widget_id, FlowMutation, FlowWorkingScene};
use super::super::{
    bytes::{Equality, TextCopy},
    Owner, Retirement,
};
use super::SceneCopy;
use crate::retirement::MutationRetirementFactory;
use crate::schema::mutations::{connect_widgets::ConnectWidgets, create_widget::CreateWidget, move_widgets::MoveWidgets, replace_widget::ReplaceWidget};
use semio_framework_artifact_flow_flow::retained::{FlowCopyAllocationBudget, FlowWidgetCopy};
use semio_framework_artifact_flow_flow::{FlowLayoutEntry, Widget, WidgetLayout};
use std::{mem::ManuallyDrop, sync::Arc};
use store::os_dsl::schema::ordered::{Grant, RetirementStep, Step, UpdateCursor};

//#region 🧬️Recipe
struct RecipeState {
    source: Option<Arc<FlowWorkingScene>>,
    mutation: Option<Arc<FlowMutation>>,
    copy: Option<SceneCopy>,
    replacement: Option<FlowWidgetCopy<FlowMutation>>,
    previous: Option<Widget>,
    scene: Option<FlowWorkingScene>,
    inverse: Option<Vec<FlowMutation>>,
    inverse_entries: Option<Vec<FlowLayoutEntry>>,
    text: Option<TextCopy>,
    key: Option<String>,
    inverse_id: Option<String>,
    update: Option<UpdateCursor<WidgetLayout>>,
    equality: Equality,
    phase: u8,
    scan: usize,
    target: usize,
    shift: usize,
    entry: usize,
    removed_edges: usize,
    edge_side: bool,
    retirement: Retirement,
    closing: bool,
}

pub(in super::super::super) struct Recipe {
    state: ManuallyDrop<RecipeState>,
}

impl Recipe {
    pub(in super::super::super) fn new(source: Arc<FlowWorkingScene>, mutation: Arc<FlowMutation>) -> Self {
        Self {
            state: ManuallyDrop::new(RecipeState {
                source: Some(source),
                mutation: Some(mutation),
                copy: None,
                replacement: None,
                previous: None,
                scene: None,
                inverse: Some(Vec::new()),
                inverse_entries: Some(Vec::new()),
                text: None,
                key: None,
                inverse_id: None,
                update: None,
                equality: Equality::default(),
                phase: 0,
                scan: 0,
                target: 0,
                shift: 0,
                entry: 0,
                removed_edges: 0,
                edge_side: false,
                retirement: Retirement::default(),
                closing: false,
            }),
        }
    }

    pub(in super::super::super) fn supported(mutation: &FlowMutation) -> bool {
        matches!(mutation, FlowMutation::DeleteWidget(_) | FlowMutation::DisconnectWidgets(_) | FlowMutation::MoveWidgets(_) | FlowMutation::ReplaceWidget(_))
    }

    pub(in super::super::super) fn complete(&self) -> bool {
        self.state.phase == 30 && self.state.scene.is_some()
    }

    pub(in super::super::super) fn take(&mut self) -> Option<(FlowWorkingScene, Vec<FlowMutation>)> {
        if !self.complete() {
            return None;
        }
        Some((self.state.scene.take()?, self.state.inverse.take()?))
    }

    pub(in super::super::super) fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<Option<usize>, String> {
        use store::SnapshotRetirementStep as SnapshotStep;
        if self.state.closing || !grant.permits_one() {
            return Ok(None);
        }
        let state = &mut *self.state;
        let mutation = state.mutation.as_ref().ok_or("Flow recipe lost mutation")?;
        if state.phase == 0 {
            if !Self::supported(mutation) {
                return Err("Flow mutation has no retained recipe".into());
            }
            let source = state.source.as_ref().ok_or("Flow recipe lost source")?;
            if source.widgets.len() > 256 || source.synapses.len() > 256 || source.layout.len() > 256 {
                return Err("Flow recipe source exceeds its admitted item envelope".into());
            }
            state.inverse.as_mut().unwrap().try_reserve_exact(source.synapses.len() + 2).map_err(|_| "Flow inverse allocation failed")?;
            state.phase = 1;
            return Ok(Some(0));
        }
        if state.phase == 1 {
            let count = match &**mutation {
                FlowMutation::MoveWidgets(payload) => payload.entries.len(),
                _ => 0,
            };
            if count > 256 {
                return Err("Flow recipe move count exceeds admitted envelope".into());
            }
            state.inverse_entries.as_mut().unwrap().try_reserve_exact(count).map_err(|_| "Flow inverse layout allocation failed")?;
            state.copy = Some(SceneCopy::new(Arc::clone(state.source.as_ref().unwrap())));
            state.phase = 2;
            return Ok(Some(0));
        }
        if state.phase == 2 {
            let copy = state.copy.as_mut().unwrap();
            if copy.complete() {
                state.scene = copy.take();
                copy.begin_close();
                state.phase = 3;
                return Ok(Some(0));
            }
            return copy.advance(1, grant.maximum_bytes);
        }
        if state.phase == 3 {
            let copy = state.copy.as_mut().unwrap();
            match copy.close_step(1, grant.maximum_bytes)? {
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    if !copy.terminal_is_empty() {
                        return Err("Flow recipe copy closed with live owners".into());
                    }
                    state.copy = None;
                    state.phase = 4;
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
                    if payload.entries.is_empty() {
                        return Err("Flow move recipe contains no entries".into());
                    }
                    state.inverse.as_mut().unwrap().push(FlowMutation::MoveWidgets(MoveWidgets { entries: state.inverse_entries.take().unwrap() }));
                    state.phase = 30;
                    return Ok(Some(0));
                }
                None => return Err("Flow move recipe cursor escaped its entries".into()),
            },
            _ => return Err("Flow recipe mutation changed".into()),
        };
        match state.phase {
            4 => {
                let candidate = if matches!(&**mutation, FlowMutation::DisconnectWidgets(_)) { scene.synapses.get(state.scan).map(|edge| edge.id.as_str()) } else { scene.widgets.get(state.scan).map(flow_widget_id) };
                let candidate = candidate.ok_or("Flow recipe target is missing")?;
                let (equal, bytes) = state.equality.advance(candidate, target_id, grant.maximum_bytes);
                if let Some(equal) = equal {
                    state.equality = Equality::default();
                    if !equal {
                        state.scan += 1;
                    } else {
                        state.target = state.scan;
                        state.shift = state.scan;
                        state.phase = match &**mutation {
                            FlowMutation::DeleteWidget(_) => 5,
                            FlowMutation::DisconnectWidgets(_) => 10,
                            FlowMutation::MoveWidgets(_) => 20,
                            FlowMutation::ReplaceWidget(_) => 15,
                            _ => unreachable!(),
                        };
                    }
                }
                return Ok(Some(bytes));
            }
            5 => {
                let last = scene.widgets.len() - 1;
                scene.widgets.swap(state.target, last);
                let widget = scene.widgets.pop().unwrap();
                state.inverse.as_mut().unwrap().push(FlowMutation::CreateWidget(CreateWidget { index: state.target, widget }));
                state.phase = 6;
            }
            6 => {
                if state.shift + 1 < scene.widgets.len() {
                    scene.widgets.swap(state.shift, state.shift + 1);
                    state.shift += 1;
                } else {
                    state.phase = 20;
                }
            }
            8 => {
                let Some(edge) = scene.synapses.get(state.scan) else {
                    state.phase = 30;
                    return Ok(Some(0));
                };
                let candidate = if state.edge_side { edge.to.as_str() } else { edge.from.as_str() };
                let (equal, bytes) = state.equality.advance(candidate, target_id, grant.maximum_bytes);
                if let Some(equal) = equal {
                    state.equality = Equality::default();
                    if equal {
                        state.target = state.scan;
                        state.shift = state.scan;
                        state.phase = 10;
                        state.edge_side = false;
                    } else if state.edge_side {
                        state.edge_side = false;
                        state.scan += 1;
                    } else {
                        state.edge_side = true;
                    }
                }
                return Ok(Some(bytes));
            }
            10 => {
                let last = scene.synapses.len() - 1;
                scene.synapses.swap(state.target, last);
                let edge = scene.synapses.pop().unwrap();
                state.inverse.as_mut().unwrap().push(FlowMutation::ConnectWidgets(ConnectWidgets { index: state.target + state.removed_edges, id: edge.id, from: edge.from, from_port: edge.from_port, to: edge.to, to_port: edge.to_port }));
                state.removed_edges += 1;
                state.phase = 11;
            }
            11 => {
                if state.shift + 1 < scene.synapses.len() {
                    scene.synapses.swap(state.shift, state.shift + 1);
                    state.shift += 1;
                } else {
                    state.phase = if matches!(&**mutation, FlowMutation::DeleteWidget(_)) { 8 } else { 30 };
                }
            }
            15 => {
                state.replacement = Some(FlowWidgetCopy::new(
                    Arc::clone(mutation),
                    0,
                    |mutation, _| match mutation {
                        FlowMutation::ReplaceWidget(payload) => Some(&payload.widget),
                        _ => None,
                    },
                    Arc::new(MutationRetirementFactory),
                    FlowCopyAllocationBudget::new(16 * 1024 * 1024, 16 * 1024 * 1024),
                ));
                state.phase = 16;
            }
            16 => {
                let copy = state.replacement.as_mut().unwrap();
                if copy.complete() {
                    state.previous = Some(std::mem::replace(&mut scene.widgets[state.target], copy.take().unwrap()));
                    copy.begin_close();
                    state.phase = 17;
                } else {
                    return copy.advance(1, grant.maximum_bytes);
                }
            }
            17 => {
                let copy = state.replacement.as_mut().unwrap();
                match copy.close_step(1, grant.maximum_bytes)? {
                    SnapshotStep::Complete => {
                        if !copy.terminal_is_empty() {
                            return Err("Flow replacement copy closed with owners".into());
                        }
                        state.replacement = None;
                        state.phase = 18;
                    }
                    SnapshotStep::Pending { released_bytes, .. } => return Ok(Some(released_bytes)),
                    SnapshotStep::Blocked => return Ok(None),
                }
            }
            18 => {
                let text = state.text.get_or_insert_with(TextCopy::default);
                let bytes = text.advance(target_id, grant.maximum_bytes)?;
                if text.complete() {
                    state.inverse_id = text.take();
                    state.text = None;
                    state.phase = 19;
                }
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
                    let value = text.take().unwrap();
                    state.text = None;
                    if state.phase == 20 {
                        state.key = Some(value);
                        state.phase = 21;
                    } else {
                        state.inverse_id = Some(value);
                        state.phase = 22;
                    }
                }
                return Ok(bytes);
            }
            22 => {
                let layout = match &**mutation {
                    FlowMutation::MoveWidgets(payload) => {
                        let layout = payload.entries[state.entry].layout.clone();
                        if layout.as_ref().is_some_and(|value| !value.x.is_finite() || !value.y.is_finite()) {
                            return Err("Flow layout has non-finite position".into());
                        }
                        layout
                    }
                    _ => None,
                };
                let map = std::mem::take(&mut scene.layout);
                let key = state.key.take().unwrap();
                state.update = Some(match layout {
                    Some(layout) => map.begin_set(key, layout),
                    None => map.begin_remove(key),
                });
                state.phase = 23;
            }
            23 => {
                let update = state.update.as_mut().unwrap();
                match update.advance(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                    Step::Complete => {
                        scene.layout = update.take_result().ok_or("Flow map update lost output")?;
                        let previous = update.take_removed().map(|value| (*value).clone());
                        let id = state.inverse_id.take().unwrap();
                        if matches!(&**mutation, FlowMutation::MoveWidgets(_)) {
                            state.inverse_entries.as_mut().unwrap().push(FlowLayoutEntry { id, layout: previous });
                        } else if previous.is_some() {
                            state.inverse.as_mut().unwrap().push(FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id, layout: previous }] }));
                        } else {
                            state.retirement.push(Owner::Bytes(id.into_bytes()));
                        }
                        update.begin_close();
                        state.phase = 24;
                    }
                    Step::Progress { completed_bytes, .. } => return Ok(Some(completed_bytes)),
                    Step::Blocked => return Ok(None),
                }
            }
            24 => {
                let update = state.update.as_mut().unwrap();
                match update.close_step(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                    RetirementStep::Complete => {
                        if !update.terminal_is_empty() {
                            return Err("Flow map update closed with owners".into());
                        }
                        state.update = None;
                        state.scan = 0;
                        if matches!(&**mutation, FlowMutation::MoveWidgets(_)) {
                            state.entry += 1;
                            state.phase = 4;
                        } else {
                            state.phase = 8;
                        }
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
        if let Some(copy) = self.state.copy.as_mut() {
            copy.begin_close();
        }
        if let Some(copy) = self.state.replacement.as_mut() {
            copy.begin_close();
        }
        if let Some(update) = self.state.update.as_mut() {
            update.begin_close();
        }
    }

    pub(in super::super::super) fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Close;
        if !self.state.closing || !grant.permits_one() {
            return Ok(Close::Blocked);
        }
        let state = &mut *self.state;
        if !state.retirement.is_empty() {
            return store::ErasedSnapshotRetirement::close_step(&mut state.retirement, 1, grant.maximum_bytes);
        }
        if let Some(copy) = state.copy.as_mut() {
            return Ok(match copy.close_step(1, grant.maximum_bytes)? {
                semio_framework_job::InteractiveJobCloseStep::Complete => {
                    if !copy.terminal_is_empty() {
                        return Err("Flow copy close retained owners".into());
                    }
                    state.copy = None;
                    Close::Pending { released_items: 1, released_bytes: 0 }
                }
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => Close::Pending { released_items, released_bytes },
                semio_framework_job::InteractiveJobCloseStep::Blocked => Close::Blocked,
            });
        }
        if let Some(copy) = state.replacement.as_mut() {
            let step = copy.close_step(1, grant.maximum_bytes)?;
            if step == Close::Complete {
                if !copy.terminal_is_empty() {
                    return Err("Flow replacement close retained owners".into());
                }
                state.replacement = None;
                return Ok(Close::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(update) = state.update.as_mut() {
            return Ok(match update.close_step(Grant { maximum_items: 1, maximum_bytes: grant.maximum_bytes }) {
                RetirementStep::Complete => {
                    if !update.terminal_is_empty() {
                        return Err("Flow map close retained owners".into());
                    }
                    state.update = None;
                    Close::Pending { released_items: 1, released_bytes: 0 }
                }
                RetirementStep::Progress { released_items, released_bytes } => Close::Pending { released_items, released_bytes },
                RetirementStep::OwnedValue(_) => Close::Pending { released_items: 1, released_bytes: 0 },
                RetirementStep::Blocked => Close::Blocked,
            });
        }
        if let Some(text) = state.text.take() {
            text.retire(&mut state.retirement);
        } else if let Some(key) = state.key.take().or_else(|| state.inverse_id.take()) {
            state.retirement.push(Owner::Bytes(key.into_bytes()));
        } else if let Some(inverse) = state.inverse.take() {
            state.retirement.push(Owner::Mutations(inverse));
        } else if let Some(entries) = state.inverse_entries.take() {
            state.retirement.push(Owner::Layout(entries));
        } else if let Some(previous) = state.previous.take() {
            state.retirement.push(Owner::Widget(previous));
        } else if let Some(scene) = state.scene.take() {
            state.retirement.push(Owner::Scene(scene));
        } else if let Some(source) = state.source.take() {
            if let Some(source) = Arc::into_inner(source) {
                state.retirement.push(Owner::Scene(source));
            }
        } else if let Some(mutation) = state.mutation.take() {
            if let Some(mutation) = Arc::into_inner(mutation) {
                state.retirement.push(Owner::Mutation(mutation));
            }
        } else {
            return Ok(Close::Complete);
        }
        Ok(Close::Pending { released_items: 1, released_bytes: 0 })
    }

    pub(in super::super::super) fn terminal_is_empty(&self) -> bool {
        let state = &*self.state;
        state.closing
            && state.source.is_none()
            && state.mutation.is_none()
            && state.copy.is_none()
            && state.replacement.is_none()
            && state.previous.is_none()
            && state.scene.is_none()
            && state.inverse.is_none()
            && state.inverse_entries.is_none()
            && state.text.is_none()
            && state.key.is_none()
            && state.inverse_id.is_none()
            && state.update.is_none()
            && state.retirement.is_empty()
    }
}

impl Drop for Recipe {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            unsafe {
                ManuallyDrop::drop(&mut self.state);
            }
        } else if !std::thread::panicking() {
            panic!("Flow recipe must close before drop");
        }
    }
}
//#endregion 🧬️Recipe

//#region 🧪️RecipeLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(super) mod tests;
//#endregion 🧪️RecipeLaws
