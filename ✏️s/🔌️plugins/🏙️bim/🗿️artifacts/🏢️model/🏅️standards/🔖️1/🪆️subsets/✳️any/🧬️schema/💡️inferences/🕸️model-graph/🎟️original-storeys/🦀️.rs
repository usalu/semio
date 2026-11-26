//! 🪜️ Original storey-level inference over the retained snapshot's stable ordinals.

use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::{resolve, StoreyLevel};
use protocol::{InferenceError, InferenceValues, InferenceWorkFailure};
use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
struct Position {
    source: Option<usize>,
    selected: Option<usize>,
    after: Option<usize>,
    pending: Option<StoreyLevel>,
    scan: usize,
    published: usize,
}

/// 🎟️ Plans and publishes storey levels under the original command's retained source.
#[derive(Default)]
pub struct OriginalStoreyLevels {
    values: InferenceValues<StoreyLevel>,
    position: Position,
}

fn compare(snapshot: &ModelSnapshot, left: usize, right: usize) -> Ordering {
    let (left_id, left) = snapshot.storeys.get_index(left).expect("a planned storey ordinal");
    let (right_id, right) = snapshot.storeys.get_index(right).expect("a planned storey ordinal");
    left.building.cmp(&right.building)
        .then_with(|| (left.level < 0).cmp(&(right.level < 0)))
        .then_with(|| {
            let order = left.level.cmp(&right.level).then_with(|| left_id.cmp(right_id));
            if left.level < 0 { order.reverse() } else { order }
        })
}

fn metadata(cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
    Ok(cx.charge_retained_item(std::mem::size_of::<Position>(), 0, 0)?)
}

impl OriginalStoreyLevels {
    /// ⏩️ Advances one funded source step, plan row, level calculation or publication.
    pub fn step(&mut self, snapshot: &ModelSnapshot, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() {
            return Err(InferenceError::Cancelled.into());
        }
        let source = std::ptr::from_ref(snapshot) as usize;
        match self.position.source {
            Some(original) if original != source => return Err(ValueError::invariant("original storey snapshot changed").into()),
            None => {
                cx.original_source_parent()?.validate_borrowed_context(cx)?;
                if metadata(cx)? {
                    self.position.source = Some(source);
                }
                return Ok(false);
            }
            _ => {}
        }
        let count = snapshot.storeys.len();
        if !self.values.admit_source_step(cx)? || !self.values.initialize_step(count, cx)? {
            return Ok(false);
        }
        let position = &mut self.position;
        if position.published == count {
            return Ok(true);
        }
        if position.pending.is_some() {
            let selected = position.selected.ok_or_else(|| ValueError::invariant("original storey selection missing"))?;
            self.values.publish_step(selected, &mut position.pending, cx)?;
            return Ok(false);
        }
        if !metadata(cx)? {
            return Ok(false);
        }
        if position.selected.is_some_and(|selected| self.values.get(selected).is_some()) {
            position.after = position.selected.take();
            position.published += 1;
            position.scan = 0;
            return Ok(false);
        }
        if position.scan < count {
            let ordinal = position.scan;
            position.scan += 1;
            if position.after.is_none_or(|after| compare(snapshot, after, ordinal) == Ordering::Less)
                && position.selected.is_none_or(|selected| compare(snapshot, ordinal, selected) == Ordering::Less)
            {
                position.selected = Some(ordinal);
            }
        } else {
            let selected = position.selected.ok_or_else(|| ValueError::invariant("original storey plan exhausted early"))?;
            let (id, row) = snapshot.storeys.get_index(selected).ok_or_else(|| ValueError::invariant("original storey ordinal missing"))?;
            let parent = position.after.and_then(|ordinal| {
                let (_, previous) = snapshot.storeys.get_index(ordinal)?;
                (previous.building == row.building && (previous.level < 0) == (row.level < 0)).then(|| self.values.get(ordinal)).flatten()
            });
            position.pending = Some(resolve(snapshot, id, parent));
        }
        Ok(false)
    }

    /// 📖️ Lends a storey's actual published level by its stable authored key.
    pub fn get(&self, snapshot: &ModelSnapshot, key: &str) -> Option<StoreyLevel> {
        if self.position.source != Some(std::ptr::from_ref(snapshot) as usize) {
            return None;
        }
        let (mut low, mut high) = (0, snapshot.storeys.len());
        while low < high {
            let ordinal = low + (high - low) / 2;
            let (id, _) = snapshot.storeys.get_index(ordinal)?;
            match id.as_str().cmp(key) {
                Ordering::Equal => return self.values.get(ordinal).copied(),
                Ordering::Less => low = ordinal + 1,
                Ordering::Greater => high = ordinal,
            }
        }
        None
    }

    /// 📏️ The next owned retirement frontier, including the original source loan.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.values.terminal_is_empty() { Ok(RetirementDemand::default()) } else { self.values.retirement_demands(body) }
    }

    /// ♻️ Releases the exact source and level rows after completion or cancellation.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if self.values.terminal_is_empty() { Ok(true) } else { self.values.close_step(cx) }
    }

    /// 🧺️ Whether the original loan and every allocated result row were released.
    pub fn terminal_is_empty(&self) -> bool {
        self.values.terminal_is_empty()
    }
}
