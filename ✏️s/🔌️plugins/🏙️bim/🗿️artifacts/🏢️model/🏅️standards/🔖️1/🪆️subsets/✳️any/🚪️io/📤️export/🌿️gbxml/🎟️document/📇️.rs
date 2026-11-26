//! 📇️ Paid scalar indexes for the original gbXML document.

use super::*;
use semio_os_kernel::InferenceValues;

#[derive(Default)]
pub(super) struct Ranked {
    order: InferenceValues<usize>,
    next: usize,
    cursor: usize,
    after: Option<usize>,
    selected: Option<usize>,
    ready: bool,
    done: bool,
}

impl Ranked {
    pub fn advance(&mut self, extent: usize, order: impl Fn(usize, usize) -> Option<std::cmp::Ordering>, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if self.done { return Ok(true); }
        if !self.order.admit_source_step(cx)? || !self.order.initialize_step(extent, cx)? { return Ok(false); }
        if self.ready {
            if self.selected.is_none() { self.done = true; return Ok(true); }
            let selected = self.selected;
            if self.order.publish_step(self.next, &mut self.selected, cx)? {
                self.after = selected;
                self.next += 1;
                self.cursor = 0;
                self.ready = false;
            }
        } else if cx.charge_work_unit() {
            if self.cursor == extent {
                self.ready = true;
            } else {
                let index = self.cursor;
                self.cursor += 1;
                if order(index, index).is_some()
                    && self.after.is_none_or(|after| order(index, after).is_some_and(std::cmp::Ordering::is_gt))
                    && self.selected.is_none_or(|selected| order(index, selected).is_some_and(std::cmp::Ordering::is_lt))
                { self.selected = Some(index); }
            }
        }
        Ok(false)
    }

    pub fn get(&self, index: usize) -> Option<usize> { self.order.get(index).copied() }

    pub fn len(&self) -> usize { self.next }

    pub fn terminal_is_empty(&self) -> bool { self.order.terminal_is_empty() }

    pub fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> { self.order.retirement_demands(bytes) }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> { self.order.close_step(cx) }
}

#[derive(Default)]
pub(super) struct BuildingAreas {
    rows: InferenceValues<f64>,
    building: usize,
    room: usize,
    sum: f64,
    pending: Option<f64>,
}

impl BuildingAreas {
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        let buildings = model.buildings.len();
        if !self.rows.admit_source_step(cx)? || !self.rows.initialize_step(buildings, cx)? { return Ok(false); }
        if self.building == buildings { return Ok(true); }
        if self.pending.is_some() {
            if self.rows.publish_step(self.building, &mut self.pending, cx)? {
                self.building += 1;
                self.room = 0;
                self.sum = 0.0;
            }
        } else if cx.charge_work_unit() {
            let (id, _) = model.buildings.get_index(self.building).ok_or_else(|| ValueError::invariant("gbXML building is unavailable"))?;
            if self.room == model.spaces.len() {
                self.pending = Some(round9(self.sum));
            } else {
                if let Some((room, storey)) = room_parts(model, inferred, self.room).filter(|(_, storey)| &storey.building == id) {
                    let _ = storey;
                    self.sum += round9(room.floor_area);
                }
                self.room += 1;
            }
        }
        Ok(false)
    }

    pub fn get(&self, index: usize) -> Option<f64> { self.rows.get(index).copied() }

    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() }

    pub fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> { self.rows.retirement_demands(bytes) }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> { self.rows.close_step(cx) }
}

#[derive(Default)]
pub(super) struct UsedOpenings {
    curtains: InferenceValues<bool>,
    elements: InferenceValues<bool>,
    window_first: InferenceValues<usize>,
    door_first: InferenceValues<usize>,
    side: usize,
    element: usize,
    pending: Option<(bool, usize)>,
    used: Option<bool>,
    first: Option<usize>,
}

impl UsedOpenings {
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, sides: &OriginalSurfaces, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.curtains.admit_source_step(cx)? || !self.curtains.initialize_step(model.curtain_wall_types.len(), cx)? { return Ok(false); }
        if !self.elements.admit_source_step(cx)? || !self.elements.initialize_step(model.openings.len(), cx)? { return Ok(false); }
        if !self.window_first.admit_source_step(cx)? || !self.window_first.initialize_step(model.window_types.len(), cx)? { return Ok(false); }
        if !self.door_first.admit_source_step(cx)? || !self.door_first.initialize_step(model.door_types.len(), cx)? { return Ok(false); }
        if self.side < sides.extent() {
            if let Some((curtain, index)) = self.pending {
                let target = if curtain { &mut self.curtains } else { &mut self.elements };
                if target.publish_step(index, &mut self.used, cx)? { self.pending = None; self.side += 1; }
            } else if cx.charge_work_unit() {
                let side = sides.source_at(self.side).and_then(|at| surface(model, inferred, at))
                    .ok_or_else(|| ValueError::invariant("gbXML used opening source disappeared"))?;
                self.pending = match side.kind {
                    SurfaceKind::CurtainWall => index_of(&model.curtain_wall_types, &side.construction).filter(|index| self.curtains.get(*index).is_none()).map(|index| (true, index)),
                    SurfaceKind::Window | SurfaceKind::Door => index_of(&model.openings, &side.element).filter(|index| self.elements.get(*index).is_none()).map(|index| (false, index)),
                    _ => None,
                };
                if self.pending.is_some() { self.used = Some(true); } else { self.side += 1; }
            }
            return Ok(false);
        }
        if self.element == model.openings.len() { return Ok(true); }
        if let Some((window, index)) = self.pending {
            let target = if window { &mut self.window_first } else { &mut self.door_first };
            if target.publish_step(index, &mut self.first, cx)? { self.pending = None; self.element += 1; }
        } else if cx.charge_work_unit() {
            self.pending = model.openings.get_index(self.element).filter(|_| self.elements.get(self.element).is_some())
                .and_then(|(_, opening)| match &opening.kind {
                    OpeningKind::Window { window_type } => index_of(&model.window_types, window_type)
                        .filter(|index| self.window_first.get(*index).is_none())
                        .filter(|_| curtain_collision(model, self, window_type).is_none()).map(|index| (true, index)),
                    OpeningKind::Door { door_type } => index_of(&model.door_types, door_type)
                        .filter(|index| self.door_first.get(*index).is_none())
                        .filter(|index| model.door_types.get_index(*index).is_some_and(|(_, row)| row.u_value.is_some())).map(|index| (false, index)),
                    OpeningKind::Void { .. } => None,
                });
            if self.pending.is_some() { self.first = Some(self.element); } else { self.element += 1; }
        }
        Ok(false)
    }

    pub fn curtain(&self, index: usize) -> bool { self.curtains.get(index).is_some() }

    pub fn window(&self, index: usize) -> Option<usize> { self.window_first.get(index).copied() }

    pub fn door(&self, index: usize) -> Option<usize> { self.door_first.get(index).copied() }

    pub fn terminal_is_empty(&self) -> bool { self.curtains.terminal_is_empty() && self.elements.terminal_is_empty() && self.window_first.terminal_is_empty() && self.door_first.terminal_is_empty() }

    pub fn close_demand(&self, bytes: usize) -> Result<RetirementDemand, ValueError> {
        if !self.curtains.terminal_is_empty() { self.curtains.retirement_demands(bytes) }
        else if !self.elements.terminal_is_empty() { self.elements.retirement_demands(bytes) }
        else if !self.window_first.terminal_is_empty() { self.window_first.retirement_demands(bytes) }
        else { self.door_first.retirement_demands(bytes) }
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.curtains.terminal_is_empty() { self.curtains.close_step(cx)?; }
        else if !self.elements.terminal_is_empty() { self.elements.close_step(cx)?; }
        else if !self.window_first.terminal_is_empty() { self.window_first.close_step(cx)?; }
        else if !self.door_first.terminal_is_empty() { self.door_first.close_step(cx)?; }
        Ok(self.terminal_is_empty())
    }
}
