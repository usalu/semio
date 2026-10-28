from pathlib import Path
home = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
path = home / 'work_tests.rs'
data = path.read_bytes()
line = b'#[path="constituent_remap_tests.rs"]mod constituent_remap_tests;'
assert line in data
if b'mod primary_remap_tests;' not in data:
    result = data.replace(line, line + b'\n#[path="primary_remap_tests.rs"]mod primary_remap_tests;', 1)
    with path.open('r+b') as output:
        output.write(result)
path = home / 'constituent_roster.rs'
data = path.read_text(encoding='utf-8')
def change(old, new):
    global data
    assert data.count(old) == 1, old[:100]
    data = data.replace(old, new, 1)
change('type PartsCursor = <Vec<IfcComplexType> as RetainedClone>::Cursor;', 'type PartsCursor = <Vec<IfcComplexType> as RetainedClone>::Cursor;\ntype ValueCursor = <IfcValue as RetainedClone>::Cursor;')
change('binding: Option<(usize, usize, usize, usize, usize, usize, usize, usize, usize)>,', 'binding: Option<(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)>,')
change('    mapping: bool,', '    mapping: bool,\n    primitive_total: usize,\n    slots_ready: bool,')
change('    retyped_args: Option<Vec<IfcValue>>,', '    retyped_args: Option<Vec<IfcValue>>,\n    root_slots: Option<Vec<usize>>,')
change('        if !state.reshaping {', '''        if state.mapping {
            let ordinal = *self.owner.root_slots.as_ref()?.get(index)?;
            if ordinal == usize::MAX { return entity.args.get(index); }
            return match mutations.get(ordinal)? {
                IfcMutation::SetEntityArg(value) if value.id == state.id && value.index == index => Some(&value.value),
                IfcMutation::InsertEntityArg(value) if value.id == state.id && value.index == index => Some(&value.value),
                _ => None,
            };
        }
        if !state.reshaping {''')
change('original_address, _, _, _, _, normalization, count)', 'original_address, _, _, _, _, _, _, normalization, count)')
change('&& self.owner.primitive_count() == Some(mutations.len()),', '&& self.owner.primitive_count() == Some(mutations.len()) && (!state.mapping || self.owner.root_slots.as_ref().is_some_and(|slots| slots.len() == state.target_arity)),')
change('if self.state.primary && self.state.reshaping { self.state.arity.max(self.state.target_arity).checked_add(1) } else { Some(1) }', 'if self.state.primary && self.state.mapping { Some(self.state.primitive_total) } else if self.state.primary && self.state.reshaping { self.state.arity.max(self.state.target_arity).checked_add(1) } else { Some(1) }')
change('self.state.phase == 14\n', 'matches!(self.state.phase, 14 | 19 | 20 | 21)\n')
change('cursor: &PartsCursor, snapshot: &IfcSnapshot, body: usize)', 'cursor: &PartsCursor, value_cursor: &ValueCursor, snapshot: &IfcSnapshot, body: usize)')
change('            3 => demand.capacity_bytes', '''            18 if !self.state.slots_ready => demand.capacity_bytes = self.state.target_arity.checked_mul(size_of::<usize>()).ok_or_else(|| invalid("IFC primary ordinal capacity overflow"))?,
            19 | 20 => {
                let child = if self.state.phase == 19 {
                    let source = source.ok_or_else(|| invalid("IFC primary value source missing"))?;
                    let index = *self.remap.map.as_ref().and_then(|map| map.get(self.state.next)).ok_or_else(|| invalid("IFC primary source ordinal missing"))?;
                    value_cursor.advance_demands(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position]).project(index, |entity| &entity.args[index]), body.min(4096).saturating_sub(Self::base().copy_bytes))?
                } else if value_cursor.terminal_is_empty() {
                    RetirementDemand { copy_bytes: size_of::<ValueCursor>(), depth: 1, ..Default::default() }
                } else { value_cursor.close_demands(body.min(4096).saturating_sub(Self::base().copy_bytes))? };
                demand.copy_bytes = demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(|| invalid("IFC primary value copy overflow"))?;
                demand.capacity_bytes = child.capacity_bytes;
                demand.release_bytes = child.release_bytes;
                demand.depth = demand.depth.max(child.depth);
            }
            3 => demand.capacity_bytes''')
change('        pending: &mut Option<Vec<IfcMutation>>,', '        pending: &mut Option<Vec<IfcMutation>>,\n        value_cursor: &mut ValueCursor,\n        value: &mut Option<IfcValue>,')
change('            pending as *const _ as usize,', '            pending as *const _ as usize,\n            value_cursor as *const _ as usize,\n            value as *const _ as usize,')
change('self.demands(Some(source), cursor, snapshot, body)?', 'self.demands(Some(source), cursor, value_cursor, snapshot, body)?')
change('        if self.state.phase == 2 {', '''        if self.state.phase == 19 || self.state.phase == 20 && !value_cursor.terminal_is_empty() {
            cx.consume_retained(RetainedCloneProgress { copied_bytes: Self::base().copy_bytes, ..Default::default() })?;
            let step = if self.state.phase == 19 {
                let index = *self.remap.map.as_ref().and_then(|map| map.get(self.state.next)).ok_or_else(|| invalid("IFC primary source ordinal missing"))?;
                value_cursor.advance(source.borrow().project(self.state.position, |snapshot| &snapshot.entities[self.state.position]).project(index, |entity| &entity.args[index]), body.saturating_sub(Self::base().copy_bytes), cx.retained_grant())
            } else { value_cursor.close_step(cx.retained_grant()) }.map_err(|error| semio_framework_job::OriginalTurnFailure::debit_original(error, cx))?;
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(1);
            if self.state.phase == 19 && matches!(step, RetainedCloneStep::Complete(_)) {
                if value.is_some() { return Err(invalid("IFC primary value receiver occupied").into()); }
                *value = Some(value_cursor.take().ok_or_else(|| invalid("IFC primary moved payload missing"))?);
                value_cursor.begin_close();
                self.state.phase = 20;
            }
            return Ok(false);
        }
        if self.state.phase == 2 {''')
change('let mapping = complete && !self.state.primary && old != next && old != Some(0) && next != Some(0);', 'let mapping = complete && (!self.state.primary || !entity.complex.is_empty()) && old != next && old != Some(0) && next != Some(0);')
change('''                let part = &entity.complex[self.state.origin];
                let mut previous''', '''                let (previous_name, args) = if self.state.primary { (entity.name.as_str(), entity.args.as_slice()) } else { let part = &entity.complex[self.state.origin]; (part.name.as_str(), part.args.as_slice()) };
                let mut previous''')
change('canonical_name(&part.name, &mut previous)', 'canonical_name(previous_name, &mut previous)')
change('self.remap.advance::<C>(previous, target, &part.args, self.state.target_arity)?', 'self.remap.advance::<C>(previous, target, args, self.state.target_arity)?')
change('''                if complete {
                    self.state.phase = 2;
                }''', '''                if complete {
                    self.state.phase = if self.state.primary { 7 } else { 2 };
                }''')
change('''                    self.state.phase = if self.state.primary {
                        10''', '''                    self.state.phase = if self.state.primary && self.state.mapping {
                        18
                    } else if self.state.primary {
                        10''')
change('''                    if self.state.guard_values {
                        if let Some(value) = values.get(self.state.guard_slot) {''', '''                    let mapped = self.state.primary && self.state.mapping && self.state.guard_part == 0;
                    let value_at = |index: usize| -> Option<&IfcValue> {
                        if mapped { let previous = *self.remap.map.as_ref()?.get(index)?; if previous == usize::MAX { Some(&IfcValue::Unset) } else { entity.args.get(previous) } } else { values.get(index) }
                    };
                    if self.state.guard_values {
                        if let Some(value) = value_at(self.state.guard_slot) {''')
change('!matches!(values.get(self.state.guard_slot),Some(IfcValue::String(value))if !value.is_empty())', '!matches!(value_at(self.state.guard_slot),Some(IfcValue::String(value))if !value.is_empty())')
change('''            14 => {
                let values''', '''            18 => {
                if !self.state.slots_ready {
                    let slots = self.root_slots.get_or_insert_with(Vec::new);
                    slots.try_reserve_exact(self.state.target_arity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primary ordinal allocation failed"))?;
                    cx.consume_retained(RetainedCloneProgress { retained_capacity_bytes: slots.capacity() * size_of::<usize>(), ..Default::default() })?;
                    self.state.primitive_total = self.state.arity.saturating_sub(self.state.target_arity).checked_add(1).ok_or_else(|| invalid("IFC primary primitive count overflow"))?;
                    self.state.slots_ready = true;
                } else if self.state.next == self.state.target_arity {
                    self.state.next = 0;
                    self.state.phase = 10;
                } else {
                    let index = self.state.next;
                    let previous = *self.remap.map.as_ref().and_then(|map| map.get(index)).ok_or_else(|| invalid("IFC primary target ordinal missing"))?;
                    let unchanged = previous == index || previous == usize::MAX && matches!(snapshot.entities[self.state.position].args.get(index), Some(IfcValue::Unset));
                    let ordinal = if unchanged { usize::MAX } else { let ordinal = self.state.primitive_total; self.state.primitive_total = ordinal.checked_add(1).ok_or_else(|| invalid("IFC primary primitive count overflow"))?; ordinal };
                    self.root_slots.as_mut().ok_or_else(|| invalid("IFC primary ordinal owner missing"))?.push(ordinal);
                    self.state.next += 1;
                }
            }
            20 => {
                *value_cursor = IfcValue::retained_clone_cursor();
                self.state.phase = 21;
            }
            21 => {
                use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity_arg::InsertEntityArg, set_entity_arg::SetEntityArg};
                let index = self.state.next;
                let value = value.take().ok_or_else(|| invalid("IFC primary prepared payload missing"))?;
                let values = pending.as_mut().ok_or_else(|| invalid("IFC primary argument receiver missing"))?;
                values.push(if index < self.state.arity { IfcMutation::SetEntityArg(SetEntityArg { id: self.state.id, index, value }) } else { IfcMutation::InsertEntityArg(InsertEntityArg { id: self.state.id, index, value }) });
                self.state.next += 1;
                self.state.phase = 14;
            }
            14 if self.state.mapping => {
                use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg, set_entity_arg::SetEntityArg};
                let values = pending.as_mut().ok_or_else(|| invalid("IFC primary argument receiver missing"))?;
                let removals = self.state.arity.saturating_sub(self.state.target_arity);
                if values.len() <= removals {
                    values.push(IfcMutation::RemoveEntityArg(RemoveEntityArg { id: self.state.id, index: self.state.arity - values.len() }));
                } else if self.state.next == self.state.target_arity {
                    if values.len() != self.state.primitive_total { return Err(invalid("IFC primary sparse batch count changed").into()); }
                    self.state.published = Some((values.as_ptr() as usize, values.len()));
                    self.state.phase = 6;
                } else {
                    let index = self.state.next;
                    let ordinal = *self.root_slots.as_ref().and_then(|slots| slots.get(index)).ok_or_else(|| invalid("IFC primary argument ordinal missing"))?;
                    if ordinal == usize::MAX { self.state.next += 1; }
                    else if ordinal != values.len() { return Err(invalid("IFC primary prepared ordinal changed").into()); }
                    else if self.remap.map.as_ref().and_then(|map| map.get(index)) == Some(&usize::MAX) {
                        values.push(if index < self.state.arity { IfcMutation::SetEntityArg(SetEntityArg { id: self.state.id, index, value: IfcValue::Unset }) } else { IfcMutation::InsertEntityArg(InsertEntityArg { id: self.state.id, index, value: IfcValue::Unset }) });
                        self.state.next += 1;
                    } else {
                        if value.is_some() || !value_cursor.terminal_is_empty() { return Err(invalid("IFC primary payload owner occupied").into()); }
                        self.state.phase = 19;
                    }
                }
            }
            14 => {
                let values''')
change('self.retyped_args.is_none() && self.remap.map.is_none()', 'self.retyped_args.is_none() && self.remap.map.is_none() && self.root_slots.is_none()')
change('''        } else if self.remap.map.is_some() {
            self.retirement''', '''        } else if self.root_slots.is_some() {
            self.retirement.next_owner_demands::<Vec<usize>>(true, &None, body)
        } else if self.remap.map.is_some() {
            self.retirement''')
change('''        if self.remap.map.is_some() {
            return self.retirement''', '''        if self.root_slots.is_some() {
            return self.retirement.begin_granted(&mut self.root_slots, grant)?.ok_or_else(|| invalid("IFC primary ordinal retirement missing"));
        }
        if self.remap.map.is_some() {
            return self.retirement''')
change('''            _ => {}''', '''            _ => {}''') if False else None
data = data.replace('self.remap.metadata())', 'self.remap.metadata(), self.root_slots.as_ref().map_or(0, Vec::len), self.root_slots.as_ref().map_or(0, Vec::capacity))')
data = data.replace('[usize; 17]) {', '[usize; 17], usize, usize) {')
result = data.encode('utf-8')
assert len(result) >= path.stat().st_size
with path.open('r+b') as output:
    output.write(result)
