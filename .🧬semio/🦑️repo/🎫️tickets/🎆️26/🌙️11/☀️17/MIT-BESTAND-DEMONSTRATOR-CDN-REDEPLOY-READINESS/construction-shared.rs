type NativeConstructionScope<'turn, 'source> = semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'turn, 'source>;
type ConstructionEffect<'a> = semio_framework_value_retained_clone::RetainedAllocationEffect<'a>;

trait ConstructionCall {
    fn run(
        &mut self,
        scope: &mut NativeConstructionScope<'_, '_>,
        effect: &mut ConstructionEffect<'_>,
        grant: RetainedCloneGrant,
        work: u64,
    ) -> Result<(Result<RetainedCloneStep, ValueError>, u128), ValueError>;

    fn discard(&mut self);
}

struct CandidateCall<'a, O, F> {
    output: &'a mut Option<O>,
    construct: Option<F>,
}

impl<O, F> ConstructionCall for CandidateCall<'_, O, F>
where
    F: FnOnce(&mut NativeConstructionScope<'_, '_>, &mut Option<usize>) -> semio_framework_value_retained_clone::RetainedWorkOutcome<(O, RetainedCloneProgress)>,
{
    fn run(
        &mut self,
        scope: &mut NativeConstructionScope<'_, '_>,
        effect: &mut ConstructionEffect<'_>,
        grant: RetainedCloneGrant,
        work: u64,
    ) -> Result<(Result<RetainedCloneStep, ValueError>, u128), ValueError> {
        let construct = self.construct.take().ok_or_else(|| ValueError::invariant("construction call already consumed"))?;
        let turn = scope.with_limits(grant, work, |scope| construct(scope, effect.actual()))?;
        if effect.actual().is_none() {
            if let Ok((_, progress)) = &turn.result {
                if progress.retained_capacity_bytes != 0 {
                    *effect.actual() = Some(progress.retained_capacity_bytes);
                }
            }
        }
        let step = turn.result.and_then(|(candidate, progress)| {
            *self.output = Some(candidate);
            progress.checked_add(RetainedCloneProgress {
                copied_items: 1,
                copied_bytes: std::mem::size_of::<Option<O>>(),
                ..Default::default()
            }).map(RetainedCloneStep::Complete)
        });
        let total = turn.work.checked_add(u128::from(self.output.is_some())).expect("primitive work is bounded by its held construction scope");
        Ok((step, total))
    }

    fn discard(&mut self) {
        drop(self.construct.take());
    }
}

struct ScopedConstructionCall<'a>(&'a mut dyn ConstructionCall);

impl Drop for ScopedConstructionCall<'_> {
    fn drop(&mut self) {
        self.0.discard();
    }
}
