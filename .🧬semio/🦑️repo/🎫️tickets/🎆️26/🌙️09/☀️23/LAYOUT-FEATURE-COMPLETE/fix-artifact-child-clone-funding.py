from difflib import unified_diff
from pathlib import Path
import sys

path = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️retained-clone/🦀️.rs")
before = path.read_text()

def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f"Expected one matching ArtifactChild cursor fragment: {old}")
    return text.replace(old, new, 1)

after = before
replacements = [
    ("nested_retained_clone_close_demands, nested_retained_clone_grant, retained_clone_grant_admits, retained_clone_close_demands", "nested_retained_clone_close_demands, nested_retained_clone_take_demands, nested_retained_clone_grant, retained_clone_cursor_take_demand, retained_clone_grant_admits, take_child_output"),
    ("OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll", "OriginalRetirementFailure, OriginalRetirementPoll"),
    ('''fn field_demands<T: RetainedClone>(cursor: &RetainedFieldCursor<T>, draining: bool, source: RetainedCloneRef<'_,T>, body:usize, maximum_work_units:u64) -> Result<semio_framework_value::RetirementDemand,ValueError> {
    if draining { return if cursor.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}) } else { nested_retained_clone_close_demands::<T>(cursor,body) }; }
    if cursor.output_ready() { return Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<T>()+size_of::<bool>(),depth:1,..Default::default()}); }
    nested_retained_clone_demands::<T>(cursor,source,body,maximum_work_units)
}''', '''fn field_finish_demand() -> semio_framework_value::RetirementDemand {
    semio_framework_value::RetirementDemand { copy_bytes: size_of::<bool>() + size_of::<usize>(), depth: 1, ..Default::default() }
}

fn field_demands<T: RetainedClone>(cursor: &RetainedFieldCursor<T>, draining: bool, source: RetainedCloneRef<'_,T>, body:usize, maximum_work_units:u64) -> Result<semio_framework_value::RetirementDemand,ValueError> {
    if draining { return if cursor.terminal_is_empty() { Ok(field_finish_demand()) } else { nested_retained_clone_close_demands::<T>(cursor,body) }; }
    if cursor.output_ready() {
        let mut demand = nested_retained_clone_take_demands::<T>(cursor)?;
        demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(|| invariant("retained child field copy extent overflow"))?;
        return Ok(demand);
    }
    nested_retained_clone_demands::<T>(cursor,source,body,maximum_work_units)
}'''),
    ('''    let grant = effect.grant();
    if cursor.output_ready() {
        let (mut candidate, mut slot) = (None, None);
        {let reply = OriginalEffectReply::borrow(&mut slot)?; cursor.take(&mut candidate, effect, reply)?;}
        slot.take().ok_or_else(|| invariant("retained child field completion left no reply"))??;
        *value = Some(candidate.ok_or_else(|| invariant("retained child ready field omitted owner"))?);
        cursor.begin_close_paid(effect).result?;
        *draining = cursor.close_started();
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}));
    }
    let nested = nested_retained_clone_grant(grant)?;''', '''    let nested = nested_retained_clone_grant(effect.grant())?;
    if cursor.output_ready() {
        let Some(mut progress) = take_child_output(cursor, value, nested, effect)? else {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        };
        record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
        *draining = true;
        progress.copied_bytes += size_of::<bool>();
        return Ok(RetainedCloneStep::Progress(progress));
    }'''),
    ('''    let grant = effect.grant();
    if !cursor.terminal_is_empty() {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let mut inner = effect.nested(nested_retained_clone_grant(grant)?);
        let step = native_reply(|reply| cursor.close_into(body, &mut inner, reply))?;
        let progress = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child field scaffold close")?.progress();
        return Ok(RetainedCloneStep::Progress(progress));
    }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    *draining = false;
    *phase += 1;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))''', '''    if let Some(step) = close_field(cursor, body, effect)? { return Ok(step); }
    record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
    *draining = false;
    *phase += 1;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: field_finish_demand().copy_bytes, ..Default::default() }))'''),
    ('''    let grant = effect.grant();
    if cursor.terminal_is_empty() { return Ok(None); }
    if !cursor.close_started() {
        let turn = cursor.begin_close_paid(effect);
        return Ok(Some(RetainedCloneStep::Progress(turn.result?)));
    }
    let mut inner = effect.nested(nested_retained_clone_grant(grant)?);''', '''    if cursor.terminal_is_empty() { return Ok(None); }
    let grant = effect.grant();
    let mut inner = effect.nested(nested_retained_clone_grant(grant)?);
    if !cursor.close_started() {
        let turn = cursor.begin_close_paid(&mut inner);
        return Ok(Some(RetainedCloneStep::Progress(turn.result?)));
    }'''),
    ('''fn settle_outcome(effect: &mut RetainedAllocationEffect<'_>, result: Result<RetainedCloneStep, OriginalRetirementFailure>) -> OriginalRetirementOutcome {
    let result = match result {
        Ok(step) if effect.performed_work() == 0 && step.progress() != RetainedCloneProgress::default() => match effect.performed(1) {
            Ok(true) => Ok(step),
            Ok(false) => Err(OriginalRetirementFailure::from_work_refusal(ValueError::literal(ValueRefusalKind::WorkLimit, "retained child step exceeds its work ceiling").with_retained_progress(step.progress()))),
            Err(error) => Err(OriginalRetirementFailure::from_work_refusal(error.with_retained_progress(step.progress()))),
        },
        other => other,
    };
    RetainedWorkOutcome { result, work: effect.performed_work() }
}

''', ''''''),
    ('''        if !retained_clone_grant_admits(grant, retained_clone_close_demands::<ArtifactChild<S>>(self, grant.maximum_copy_bytes)?) { return Ok(RetainedCloneStep::Progress(Default::default())); }''', '''        if !retained_clone_grant_admits(grant, self.close_demands(body)?) || effect.remaining_work_units()? < self.close_work_demand(body)? {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }'''),
    ('''        if !source.check_binding(&self.source)? { return Ok(semio_framework_value::RetirementDemand {copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()}); }''', '''        if let Some(demand) = source.binding_demand(&self.source)? { return Ok(demand); }'''),
    ('''            2=>Ok(semio_framework_value::RetirementDemand {copy_bytes:if source.get().local_owner.is_some(){size_of::<ArtifactChildLocalOwnerAlias>()}else{0},depth:1,..Default::default()}),
            3=>Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<ArtifactChild<S>>(),depth:1,..Default::default()}),''', '''            2=>Ok(semio_framework_value::RetirementDemand {
                copy_bytes: size_of::<Option<ArtifactChildLocalOwnerAlias>>() + size_of::<usize>(),
                depth: 1,
                ..Default::default()
            }),
            3=>Ok(semio_framework_value::RetirementDemand {
                copy_bytes: size_of::<Option<ArtifactChild<S>>>() + size_of::<Option<String>>()
                    + size_of::<Option<ArtifactRef>>() + size_of::<Option<ArtifactChildLocalOwnerAlias>>()
                    + size_of::<usize>(),
                depth: 1,
                ..Default::default()
            }),'''),
    ('''        if !source.check_binding(&self.source)?{return Ok(1)}''', '''        if self.source.is_none() { return source.binding_work_demand(&self.source); }'''),
    ('''            if draining{return if cursor.terminal_is_empty(){Ok(1)}else{cursor.close_work_demand(body)}}
            if cursor.output_ready(){return cursor.take_work_demand()?.checked_add(cursor.begin_close_work_demand()?).ok_or_else(||invariant("retained child field work overflow"))}''', '''            if draining {
                if cursor.terminal_is_empty() { return Ok(1); }
                if !cursor.close_started() { return cursor.begin_close_work_demand(); }
                return cursor.close_work_demand(body);
            }
            if cursor.output_ready() {
                return cursor.take_work_demand()?.checked_add(1).ok_or_else(|| invariant("retained child field work overflow"));
            }'''),
    ('''        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }''', '''        if !retained_clone_grant_admits(grant,demand) || effect.remaining_work_units()? < self.advance_work_demand(source, body, maximum_work_units)? {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }'''),
    ('''                self.local_owner_value = source.get().local_owner.as_ref().map(|owner| ArtifactChildLocalOwnerAlias(Arc::clone(owner)));''', '''                record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
                self.local_owner_value = source.get().local_owner.as_ref().map(|owner| ArtifactChildLocalOwnerAlias(Arc::clone(owner)));'''),
    ('''                self.output = Some(ArtifactChild {''', '''                record_retirement_work(effect, 1).map_err(|(error, _)| error)?;
                self.output = Some(ArtifactChild {'''),
    ('''        Ok(reply.complete(settle_outcome(effect, result).result))''', '''        Ok(reply.complete(result))'''),
    ('''        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() })''', '''        retained_clone_cursor_take_demand::<ArtifactChild<S>, bool>()'''),
    ('''        let demand = semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() };''', '''        let demand = self.take_demand()?;'''),
    ('''        if !retained_clone_grant_admits(effect.grant(), demand) || effect.maximum_work_units() == 0 { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }''', '''        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }'''),
]

for old, new in replacements:
    after = replace_once(after, old, new)
if len(sys.argv) == 2 and sys.argv[1] == "--apply":
    if path.read_text() != before:
        raise ValueError("ArtifactChild cursor changed while preparing the patch")
    path.write_text(after)
else:
    print("".join(unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), "a/" + str(path), "b/" + str(path))), end="")
