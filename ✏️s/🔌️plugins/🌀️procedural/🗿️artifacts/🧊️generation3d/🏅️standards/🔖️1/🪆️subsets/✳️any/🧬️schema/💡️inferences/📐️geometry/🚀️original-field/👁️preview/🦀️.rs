//! 👁️ Funded projection of connected geometry outputs into a preview sink.
use super::*;

pub(super) fn project(state: &mut State, evaluation: &mut Option<WidgetEvaluation>, output: &mut Option<SharedEvaluation>, snapshot: &GeometryInput<'_>, key: &str, parents: &[SharedEvaluation], cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let host = &snapshot.snapshot.host_snapshot;
    let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
    match state.phase {
        28 => {
            let Some(wire) = host.synapses.get(state.wire) else {
                let values = state.values.take().ok_or_else(|| invalid("preview output map disappeared"))?;
                *evaluation = Some(WidgetEvaluation::ok(values, Quality::ExactAnalytic));
                state.phase = 20;
                return post(cx, item);
            };
            if let Some(incoming) = state.equality.step(&wire.to, key, cx)? {
                if incoming { state.parent = 0; state.phase = 29; } else { state.wire += 1; }
            }
        }
        29 => {
            if cx.retained_grant().maximum_depth < state.parents.next_get_depth_demand(state.parent).map_err(ValueError::from)? { return Ok(()); }
            let ordinal = *state.parents.get(state.parent).ok_or_else(|| invalid("preview wire has no source in its plan"))?;
            let parent = host.widgets.get(ordinal).ok_or_else(|| invalid("preview source widget disappeared"))?;
            if let Some(found) = state.equality.step(widget_id(parent), &host.synapses[state.wire].from, cx)? {
                if found { state.phase = 30; state.output = 0; } else { state.parent += 1; }
            }
        }
        30 => {
            let parent = parents.get(state.parent).ok_or_else(|| invalid("preview source evaluation disappeared"))?;
            if parent.fault.is_some() {
                let bytes = size_of::<SharedEvaluation>();
                if cx.retained_grant().maximum_copy_bytes < bytes { return Ok(()); }
                *output = Some(parent.clone());
                state.phase = 20;
                return post(cx, RetainedCloneProgress { copied_bytes: bytes, ..item });
            }
            let wire = &host.synapses[state.wire];
            if let Some((port, _)) = parent.outputs.get_index(state.output) {
                if wire.from_port.is_empty() { state.phase = 31; post(cx, item)?; }
                else if let Some(found) = state.equality.step(port, &wire.from_port, cx)? {
                    if found { state.phase = 31; } else { state.output += 1; }
                }
            } else if wire.from_port.is_empty() { state.wire += 1; state.phase = 28; post(cx, item)?; }
            else { return Err(invalid("preview wire names an absent source output").into()); }
        }
        31 => {
            let (_, source) = parents[state.parent].outputs.get_index(state.output).ok_or_else(|| invalid("preview selected output disappeared"))?;
            if let Some(value) = state.preview_copy.step(source, cx)? { state.value = Some(value); state.phase = 32; state.duplicate = 0; }
        }
        32 => {
            let (port, _) = parents[state.parent].outputs.get_index(state.output).ok_or_else(|| invalid("preview selected port disappeared"))?;
            let values = state.values.as_ref().ok_or_else(|| invalid("preview output map disappeared"))?;
            if let Some((name, _)) = values.get_index(state.duplicate) {
                if let Some(equal) = state.equality.step(name, port, cx)? {
                    if equal { state.phase = 37; } else { state.duplicate += 1; }
                }
            } else { state.phase = 33; post(cx, item)?; }
        }
        33 => {
            let (port, _) = parents[state.parent].outputs.get_index(state.output).ok_or_else(|| invalid("preview output key disappeared"))?;
            if semio_s_artifact_procedural_3d_geometry_inputs::construction::text_event(&mut state.key, port, &mut state.byte, &mut state.reserved, &mut state.text_source, cx)? { state.phase = 34; }
        }
        34 => {
            let bytes = size_of::<Insert>();
            if cx.retained_grant().maximum_copy_bytes < bytes { return Ok(()); }
            let values = state.values.take().ok_or_else(|| invalid("preview output map disappeared"))?;
            let value = state.value.take().ok_or_else(|| invalid("preview output value disappeared"))?;
            state.insertion = Some(Insert::new(values, std::mem::take(&mut state.key), value));
            state.phase = 35;
            post(cx, RetainedCloneProgress { copied_bytes: bytes, ..item })?;
        }
        35 => {
            let insertion = state.insertion.as_mut().ok_or_else(|| invalid("preview insertion disappeared"))?;
            if insertion.output_ready() {
                insert_take(insertion, &mut state.values, cx)?;
                if state.values.is_some() { state.phase = 36; }
            } else { insert_advance(insertion, cx)?; }
        }
        36 => {
            let insertion = state.insertion.as_mut().ok_or_else(|| invalid("preview insertion closure disappeared"))?;
            insert_close(insertion, cx)?;
            if insertion.terminal_is_empty() { state.insertion.take(); state.phase = 39; }
        }
        37 => {
            let bytes = size_of::<GeometryValue>();
            if cx.retained_grant().maximum_copy_bytes < bytes { return Ok(()); }
            let values = state.values.as_mut().ok_or_else(|| invalid("preview output map disappeared"))?;
            if cx.fuel_remaining() <= state.duplicate as u64 + 1 { return Ok(()); }
            let mut entries = values.iter_mut();
            for _ in 0..state.duplicate {
                entries.next().ok_or_else(|| invalid("preview replacement ordinal disappeared"))?;
                cx.consume_work(1);
            }
            let (_, old) = entries.next().ok_or_else(|| invalid("preview replacement ordinal disappeared"))?;
            let mut incoming = state.value.take().ok_or_else(|| invalid("preview replacement value disappeared"))?;
            std::mem::swap(old, &mut incoming);
            match ControlledRetirement::new(incoming) {
                Ok(owner) => state.preview_replaced = Some(owner),
                Err((error, displaced)) => {
                    state.value = Some(displaced);
                    return Err(error.into());
                }
            }
            state.phase = 38;
            post(cx, RetainedCloneProgress { copied_bytes: bytes, ..item })?;
        }
        38 => {
            let owner = state.preview_replaced.as_mut().ok_or_else(|| invalid("preview replaced output disappeared"))?;
            close_owner(owner, cx)?;
            if owner.terminal_is_empty() { state.preview_replaced.take(); state.phase = 39; }
        }
        39 => {
            if host.synapses[state.wire].from_port.is_empty() { state.output += 1; state.phase = 30; }
            else { state.wire += 1; state.phase = 28; }
            post(cx, item)?;
        }
        _ => return Err(invalid("preview projection phase changed").into()),
    }
    Ok(())
}

#[derive(Default, semio_framework_value::RetireOwned)]
pub(super) struct ValueCopy {
    frames: Vec<Vec<GeometryValue>>,
    completed: Option<GeometryValue>,
    leaf: Option<GeometryValue>,
    byte: usize,
    reserved: bool,
    binding: Option<(usize, usize)>,
}

impl ValueCopy {
    fn step(&mut self, source: &GeometryValue, cx: &mut StepContext<'_>) -> Result<Option<GeometryValue>, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.completed.is_some() {
            if let Some(parent) = self.frames.last_mut() {
                if grant.maximum_copy_bytes < size_of::<GeometryValue>() { return Ok(None); }
                if parent.len() == parent.capacity() { return Err(invalid("preview list lost its reserved output slot").into()); }
                parent.push(self.completed.take().unwrap());
                post(cx, RetainedCloneProgress { copied_bytes: size_of::<GeometryValue>(), ..item })?;
                return Ok(None);
            }
            if self.frames.capacity() != 0 {
                let bytes = self.frames.capacity() * size_of::<Vec<GeometryValue>>();
                if grant.maximum_release_bytes < bytes { return Ok(None); }
                self.frames = Vec::new();
                post(cx, RetainedCloneProgress { released_bytes: bytes, ..item })?;
                return Ok(None);
            }
            post(cx, item)?;
            return Ok(self.completed.take());
        }
        let depth = self.frames.len();
        if depth > grant.maximum_depth || cx.fuel_remaining() <= depth as u64 { return Ok(None); }
        let mut current = source;
        let mut complete_list = false;
        for frame in &self.frames {
            cx.consume_work(1);
            let GeometryValue::List(values) = current else { return Err(invalid("preview list source changed").into()); };
            if let Some(child) = values.get(frame.len()) { current = child; }
            else { complete_list = true; break; }
        }
        if complete_list {
            let values = self.frames.pop().ok_or_else(|| invalid("preview list frame disappeared"))?;
            self.completed = Some(GeometryValue::List(values));
            post(cx, item)?;
            return Ok(None);
        }
        match current {
            GeometryValue::List(values) => {
                if self.frames.len() == self.frames.capacity() {
                    let old = self.frames.capacity() * size_of::<Vec<GeometryValue>>();
                    let needed = old.checked_add(size_of::<Vec<GeometryValue>>()).ok_or_else(|| invalid("preview list stack is too large"))?;
                    if grant.maximum_copy_bytes < old || grant.maximum_release_bytes < old || grant.maximum_capacity_bytes < needed { return Ok(None); }
                    self.frames.try_reserve_exact(1).map_err(|_| invalid("preview list stack allocation failed"))?;
                    post(cx, RetainedCloneProgress { copied_bytes: old, retained_capacity_bytes: self.frames.capacity()*size_of::<Vec<GeometryValue>>(), released_bytes: old, ..item })?;
                    return Ok(None);
                }
                let bytes = values.len().checked_mul(size_of::<GeometryValue>()).ok_or_else(|| invalid("preview list is too large"))?;
                if grant.maximum_capacity_bytes < bytes { return Ok(None); }
                let mut children = Vec::new();
                children.try_reserve_exact(values.len()).map_err(|_| invalid("preview list allocation failed"))?;
                let capacity = children.capacity()*size_of::<GeometryValue>();
                self.frames.push(children);
                post(cx, RetainedCloneProgress { retained_capacity_bytes: capacity, ..item })?;
            }
            GeometryValue::Text(text) => {
                let target = self.leaf.get_or_insert_with(|| GeometryValue::Text(String::new()));
                let GeometryValue::Text(value) = target else { return Err(invalid("preview text source changed").into()); };
                if semio_s_artifact_procedural_3d_geometry_inputs::construction::text_event(value, text, &mut self.byte, &mut self.reserved, &mut self.binding, cx)? {
                    self.completed = self.leaf.take();
                }
            }
            GeometryValue::Selection(selection) => {
                if let Some(GeometryValue::Selection(target)) = self.leaf.as_mut() {
                    if let Some(id) = selection.ids.get(target.ids.len()) {
                        if grant.maximum_copy_bytes < size_of::<u64>() { return Ok(None); }
                        target.ids.push(*id);
                        post(cx, RetainedCloneProgress { copied_bytes: size_of::<u64>(), ..item })?;
                    } else { self.completed = self.leaf.take(); post(cx, item)?; }
                } else {
                    let bytes = selection.ids.len().checked_mul(size_of::<u64>()).ok_or_else(||invalid("preview selection is too large"))?;
                    if grant.maximum_capacity_bytes < bytes { return Ok(None); }
                    let mut ids = Vec::new();
                    ids.try_reserve_exact(selection.ids.len()).map_err(|_|invalid("preview selection allocation failed"))?;
                    let capacity = ids.capacity()*size_of::<u64>();
                    self.leaf = Some(GeometryValue::Selection(SelectionValue { component: selection.component, ids }));
                    post(cx, RetainedCloneProgress { retained_capacity_bytes: capacity, ..item })?;
                }
            }
            fixed => {
                let bytes = match fixed {
                    GeometryValue::Boolean(_) => 1,
                    GeometryValue::Integer(_) | GeometryValue::Number(_) => 8,
                    GeometryValue::Vector(_) | GeometryValue::Point(_) => 24,
                    GeometryValue::Plane(_) => size_of::<PlaneValue>(),
                    GeometryValue::Shape(_) | GeometryValue::Mesh(_) => size_of::<usize>(),
                    _ => unreachable!(),
                };
                if grant.maximum_copy_bytes < bytes { return Ok(None); }
                self.completed = Some(fixed.clone());
                post(cx, RetainedCloneProgress { copied_bytes: bytes, ..item })?;
            }
        }
        Ok(None)
    }
}
