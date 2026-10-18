//! 🔪️ Booleans and transforms: the boolean operation macro with its kernel plan job and the compound cut and transform operators. See [`super`].

use super::*;

// #region 🔖️Booleans
/// ⏱️ BRep set operations retain kernel plans across evaluator turns; mesh modeling jobs use the same domain-neutral scheduler.
macro_rules! boolean_operation {
    ($name:ident, $op:expr) => {
        pub(super) struct $name(pub(super) SessionCapture);
        impl Operator for $name {
            retire_geometry_capture!(0);
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                self.0.with_kernel(|kernel| {
                    let handle = kernel.boolean_sync(&read_geometry(input, "a")?, &read_geometry(input, "b")?, $op).map_err(|error| map_kernel_error(&error))?;
                    Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
                })
            }

            fn step_plan(&self, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant) -> Result<(neural_engine::OperatorPlanAdmission, semio_framework_value::RetainedCloneProgress), (EvalError, Dictionary)> {
                BrepBooleanOperatorJob::admit(&self.0, input, grant, $op)
            }
            fn next_plan_copy_byte_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
            fn next_plan_capacity_byte_demand(&self, _input: &Dictionary, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(BrepBooleanOperatorJob::plan_capacity()) }
            fn next_plan_release_byte_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
            fn next_plan_depth_demand(&self, _input: &Dictionary) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }
        }
    };
}

boolean_operation!(Fuse, BooleanOp::Unite);

boolean_operation!(Cut, BooleanOp::Cut);

boolean_operation!(Intersect, BooleanOp::Intersect);

/// ⏱️ One set operation as a budgeted, resumable [`neural_engine::OperatorJob`]. Admission runs the
/// kernel's microsecond-cheap fast paths eagerly (they mutate the body and produce the result in
/// place, so they can never be re-run later) and retains their answer; anything else becomes a
/// retained [`BrepBooleanJob`] the steps advance.
///
/// 🔒️ Every step re-acquires the process-global kernel lock through [`with_kernel`] and hands the
/// job the body again, which is what makes the job retainable across host turns: it borrows
/// nothing.
struct BrepBooleanOperatorJob {
    pub(super) session: Session,
    job: Option<BrepBooleanJob>,
    answered: Option<GeometryHandle>,
    value_retirement: neural_engine::ValueRetirement,
    pub(super) receipt: semio_framework_value::RetainedCloneProgress,
    pub(super) cancelled: bool,
    pub(super) progress: neural_engine::OperatorProgress,
}

impl BrepBooleanOperatorJob {
    fn plan_capacity() -> usize {
        std::mem::size_of::<BrepBooleanOperatorJob>().saturating_add(neural_engine::ValueRetirement::domain_frame_birth_bytes())
    }

    /// 🔀️ Admits one set operation. Always answers a job: even a fast-path result comes back as a
    /// job, because admission has already mutated the body and an immediate plan would make the
    /// caller evaluate the whole boolean a second time. The original input moves into the job's
    /// own value retirement.
    pub(super) fn admit(session: &Session, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant, op: BooleanOp) -> Result<(neural_engine::OperatorPlanAdmission, semio_framework_value::RetainedCloneProgress), (EvalError, Dictionary)> {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < Self::plan_capacity() {
            return Err((EvalError::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "boolean plan admission requires one item and its job frame")), input));
        }
        let (a, b) = match (read_geometry(&input, "a"), read_geometry(&input, "b")) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(error), _) | (_, Err(error)) => return Err((error, input)),
        };
        let admitted = session.with_kernel(|kernel| {
            let admission = kernel.boolean_job_sync(&a, &b, op).map_err(|error| map_kernel_error(&error))?;
            Ok(match admission {
                BrepBooleanAdmission::Answered(handle) => BrepBooleanOperatorJob { session: session.clone(), job: None, answered: Some(handle), value_retirement: Default::default(), receipt: Default::default(), cancelled: false, progress: neural_engine::OperatorProgress { units_done: 1, units_total: 1, phase: "complete" } },
                BrepBooleanAdmission::Job(job) => {
                    let progress = job.progress();
                    BrepBooleanOperatorJob { session: session.clone(), job: Some(job), answered: None, value_retirement: Default::default(), receipt: Default::default(), cancelled: false, progress: neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() } }
                }
            })
        });
        let mut job = match admitted {
            Ok(job) => job,
            Err(error) => return Err((error, input)),
        };
        let child = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..grant };
        match job.value_retirement.push_dictionary(input, child) {
            Ok(progress) => Ok((neural_engine::OperatorPlanAdmission::Job(Box::new(job)), semio_framework_value::RetainedCloneProgress { retained_capacity_bytes: progress.retained_capacity_bytes.saturating_add(std::mem::size_of::<BrepBooleanOperatorJob>()), ..progress })),
            Err((error, input)) => {
                neural_engine::OperatorJob::cancel(&mut job);
                job.job = None;
                job.answered = None;
                Err((EvalError::from(error), input))
            }
        }
    }

    /// 📦️ The out dictionary a finished boolean produces — identical to the one-shot operator's.
    fn output(session: &Session, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
        session.with_kernel(|kernel| Ok(channel_output("solid", geometry_dict(kernel, handle)?)))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.answered.is_none() && self.value_retirement.terminal_is_empty()
    }
}

impl neural_engine::OperatorJob for BrepBooleanOperatorJob {
    fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(neural_engine::OperatorJobStep,semio_framework_value::RetainedCloneProgress),neural_engine::OperatorWorkFailure>{
        self.receipt=Default::default();let before=cx.retained_progress();let result=self.advance(cx);let after=cx.retained_progress();
        let difference=|after:usize,before:usize|after.checked_sub(before).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Boolean recipient regressed"));
        self.receipt=semio_framework_value::RetainedCloneProgress{copied_items:difference(after.copied_items,before.copied_items)?,copied_bytes:difference(after.copied_bytes,before.copied_bytes)?,retained_capacity_bytes:difference(after.retained_capacity_bytes,before.retained_capacity_bytes)?,released_bytes:difference(after.released_bytes,before.released_bytes)?};result.map(|step|(step,self.receipt))
    }
    fn normal_step_progress(&self) -> semio_framework_value::RetainedCloneProgress {
        self.receipt
    }

    fn next_step_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_step_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(1) }

    fn progress(&self) -> neural_engine::OperatorProgress {
        self.progress
    }

    fn cancel(&mut self) {
        if self.answered.is_some() {
            return;
        }
        self.cancelled = true;
        if let Some(job) = self.job.as_mut() {
            job.cancel();
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_capacity_byte_demand(maximum_copy_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(0); }
        self.value_retirement.next_release_byte_demand()
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.job.is_some() || self.answered.is_some() { return Ok(1); }
        self.value_retirement.next_depth_demand()
    }

    fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<semio_framework_value::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.job.take().is_some() || self.answered.take().is_some() {
            self.cancelled = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let step = self.value_retirement.close_step(semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..grant })?;
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        BrepBooleanOperatorJob::terminal_is_empty(self)
    }
}

impl BrepBooleanOperatorJob {
    pub(super) fn advance(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<neural_engine::OperatorJobStep,neural_engine::OperatorWorkFailure> {
        if self.cancelled {
            return Ok(neural_engine::OperatorJobStep::Cancelled(self.progress));
        }
        if self.answered.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Boolean dictionary output has no funded producer yet").into());
        }
        let Some(job) = self.job.as_mut() else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"boolean job carries neither an answer nor a plan").into());
        };
        let step=self.session.step_boolean_job(job,cx)?;
        match step {
            BrepBooleanStep::Working(progress) => {
                self.progress = neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() };
                Ok(neural_engine::OperatorJobStep::Working(self.progress))
            }
            BrepBooleanStep::Cancelled(progress) => {
                self.cancelled = true;
                self.progress = neural_engine::OperatorProgress { units_done: progress.units_done, units_total: progress.units_total, phase: progress.phase.tag() };
                Ok(neural_engine::OperatorJobStep::Cancelled(self.progress))
            }
            BrepBooleanStep::Ready(handle) => {
                self.job = None;
                self.answered = Some(handle.clone());
                self.progress = neural_engine::OperatorProgress { units_done: self.progress.units_total.max(self.progress.units_done), units_total: self.progress.units_total.max(self.progress.units_done), phase: "complete" };
                Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Boolean dictionary output has no funded producer yet").into())
            }
        }
    }
}

pub(super) struct CompoundCut(pub(super) SessionCapture);

impl Operator for CompoundCut {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
            retire_geometry_capture!(0);
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let target = read_geometry(input, "target")?;
            let tools = read_geometry_list(input, "tools")?;
            let handle = kernel.compound_cut(&target, &tools).map_err(|error| map_kernel_error(&error))?;
            Ok(channel_output("solid", geometry_dict(kernel, &handle)?))
        })
    }
}
// #endregion 🔖️Booleans

// #region 🔖️Transforms
geo_operation!(Translate, "geometryOut", |k, i| k.translate(&read_geometry(i, "geometry")?, read_xyz(i, "offset")?));

geo_operation!(Rotate, "geometryOut", |k, i| k.rotate(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));

geo_operation!(RotateAbout, "geometryOut", |k, i| k.rotate_about(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "axis")?, read_channel_number(i, "angle")?));

geo_operation!(Scale, "geometryOut", |k, i| k.scale_axes(&read_geometry(i, "geometry")?, read_xyz(i, "factor")?, read_xyz(i, "center")?));

geo_operation!(Mirror, "geometryOut", |k, i| k.mirror(&read_geometry(i, "geometry")?, read_xyz(i, "origin")?, read_xyz(i, "normal")?));

geo_operation!(CopyShape, "geometryOut", |k, i| k.copy_shape(&read_geometry(i, "geometry")?));

geo_operation!(LinearPattern, "compound", |k, i| k.linear_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "direction")?, read_channel_number(i, "spacing")?, read_channel_number(i, "count")? as usize,));

geo_operation!(CircularPattern, "compound", |k, i| k.circular_pattern(&read_geometry(i, "geometry")?, read_xyz(i, "axis")?, read_channel_number(i, "count")? as usize,));

geo_operation!(GridPattern, "compound", |k, i| k.grid_pattern(
    &read_geometry(i, "geometry")?,
    read_xyz(i, "dirX")?,
    read_xyz(i, "dirY")?,
    read_channel_number(i, "spacingX")?,
    read_channel_number(i, "spacingY")?,
    read_channel_number(i, "countX")? as usize,
    read_channel_number(i, "countY")? as usize,
));
// #endregion 🔖️Transforms
