//! ⏱️ Retained execution for fixed arithmetic and ordered variadic folds.

use super::{compute, number_dictionary, read_dict, read_value_number, read_xyz, xyz_dictionary, Vec3};
use neural_engine::retirement::RetainedDictionaryInput;
use neural_engine::{channel_output, Dictionary, EvalError, OperatorJob, OperatorJobStep, OperatorPlanAdmission, OperatorProgress, OperatorWorkFailure, Value, ValueRetirement, SCHEMA_KEY};
use semio_framework_job::StepContext;
use semio_framework_value::{DslValue, Number, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, ValueError, ValueRefusalKind};

macro_rules! math_operator {
    ($name:ident) => {
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                retained::evaluate(MathAction::$name, input)
            }

            fn step_plan(&self, input: Dictionary, grant: semio_framework_value::RetainedCloneGrant) -> retained::Admission {
                retained::admit(MathAction::$name, input, grant)
            }

            fn next_plan_copy_byte_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }
            fn next_plan_capacity_byte_demand(&self, _: &Dictionary, _: usize) -> Result<usize, semio_framework_value::ValueError> {
                Ok(retained::PLAN_BYTES)
            }
            fn next_plan_release_byte_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }
            fn next_plan_depth_demand(&self, _: &Dictionary) -> Result<usize, semio_framework_value::ValueError> {
                Ok(1)
            }
        }
    };
}
pub(super) use math_operator;

#[derive(Clone, Copy)]
pub(super) enum MathAction {
    Add,
    Subtract,
    Move,
    Multiply,
    Divide,
    Power,
    Modulo,
    Negate,
    Abs,
    Sqrt,
    Min,
    Max,
    Floor,
    Ceil,
    Round,
    Sin,
    Cos,
    Tan,
    Remap,
    Random,
    PassThrough,
    Sum,
}

#[derive(Clone, Copy)]
pub(super) enum MathOutput {
    Number(&'static str, f64),
    Coordinates(&'static str, &'static str, Vec3),
    Moved(Vec3),
}

#[derive(Clone, Copy)]
enum Payload<'a> {
    Number(f64),
    Coordinates(&'a str, Vec3),
}

impl MathOutput {
    fn channels<'a>(self, input: &'a Dictionary) -> (&'static str, Payload<'a>, Option<&'static str>) {
        match self {
            Self::Number(channel, value) => (channel, Payload::Number(value), None),
            Self::Coordinates(channel, schema, value) => (channel, Payload::Coordinates(schema, value), None),
            Self::Moved(value) => {
                let schema = input.get("subject").and_then(Value::as_dictionary).and_then(Dictionary::schema).unwrap_or("vector");
                let (channel, empty) = if schema == "point" { ("point", "vector") } else { ("vector", "point") };
                (channel, Payload::Coordinates(schema, value), Some(empty))
            }
        }
    }

    fn cold(self, input: &Dictionary) -> Dictionary {
        let (channel, payload, empty) = self.channels(input);
        let dictionary = match payload {
            Payload::Number(value) => number_dictionary(value),
            Payload::Coordinates(schema, value) => xyz_dictionary(schema, value),
        };
        let output = channel_output(channel, dictionary);
        match empty {
            Some(empty) => output.insert(empty, Value::null()),
            None => output,
        }
    }

    fn output_receipt(self, input: &Dictionary) -> RetainedCloneProgress {
        let (channel, payload, empty) = self.channels(input);
        let (fields, bytes) = match payload {
            Payload::Number(_) => (2, SCHEMA_KEY.len() + "number".len() + "value".len()),
            Payload::Coordinates(schema, _) => (4, SCHEMA_KEY.len() + schema.len() + 3),
        };
        let text_bytes = bytes + channel.len() + empty.map_or(0, str::len);
        let entries = fields + 1 + usize::from(empty.is_some());
        RetainedCloneProgress { copied_items: 1, copied_bytes: text_bytes, retained_capacity_bytes: text_bytes + entries * size_of::<(String, DslValue)>(), ..Default::default() }
    }

    fn typed(self, input: &Dictionary) -> DslValue {
        let (channel, payload, empty) = self.channels(input);
        let decimal = |value| DslValue::Number(Number::Float(value));
        let field = |key: &str, value| (key.to_owned(), value);
        let payload = match payload {
            Payload::Number(value) => vec![field(SCHEMA_KEY, DslValue::String("number".into())), field("value", decimal(value))],
            Payload::Coordinates(schema, value) => vec![field(SCHEMA_KEY, DslValue::String(schema.into())), field("x", decimal(value.x)), field("y", decimal(value.y)), field("z", decimal(value.z))],
        };
        let mut channels = Vec::with_capacity(1 + usize::from(empty.is_some()));
        channels.push(field(channel, DslValue::Object(payload)));
        if let Some(empty) = empty {
            channels.push(field(empty, DslValue::Null));
        }
        DslValue::Object(channels)
    }
}

const INDEX_DIGITS: usize = usize::MAX.ilog10() as usize + 1;
const MAX_EVAL_ERROR_BYTES: usize = 20;

#[derive(Default)]
enum Order {
    #[default]
    Classify,
    Digits(usize),
    Aliases {
        previous: Option<(usize, usize)>,
        next: Option<(usize, usize)>,
    },
}

#[derive(Default)]
struct ListFold {
    order: Order,
    rank: usize,
    max_digits: usize,
    total: Option<Payload<'static>>,
}

fn index_text(mut index: usize, buffer: &mut [u8; INDEX_DIGITS]) -> &str {
    let mut at = buffer.len();
    loop {
        at -= 1;
        buffer[at] = b'0' + (index % 10) as u8;
        index /= 10;
        if index == 0 {
            return std::str::from_utf8(&buffer[at..]).expect("index consists of ASCII digits");
        }
    }
}

impl ListFold {
    fn step(&mut self, list: &Dictionary, coordinates: bool) -> Result<Option<MathOutput>, EvalError> {
        let entry = list.entry_at_rank(self.rank);
        let mut index = None;
        match &mut self.order {
            Order::Classify => {
                if let Some((key, _)) = entry {
                    if let Ok(key_index) = key.parse::<usize>() {
                        if key == index_text(key_index, &mut [0; INDEX_DIGITS]) {
                            self.max_digits = self.max_digits.max(key.len());
                        } else {
                            self.order = Order::Aliases { previous: None, next: None };
                            self.rank = 0;
                            return Ok(None);
                        }
                    }
                } else {
                    self.order = Order::Digits(1);
                    self.rank = 0;
                    return Ok(None);
                }
            }
            Order::Digits(digits) => {
                if *digits > self.max_digits {
                    return self.finish(coordinates).map(Some);
                }
                if let Some((key, _)) = entry {
                    if key.len() == *digits {
                        index = key.parse::<usize>().ok();
                    }
                } else {
                    *digits += 1;
                    self.rank = 0;
                    return Ok(None);
                }
            }
            Order::Aliases { previous, next } => {
                if let Some((key, _)) = entry {
                    if let Ok(index) = key.parse::<usize>() {
                        let candidate = (index, self.rank);
                        if previous.is_none_or(|previous| candidate > previous) && next.is_none_or(|next| candidate < next) {
                            *next = Some(candidate);
                        }
                    }
                } else {
                    let Some(found) = next.take() else {
                        return self.finish(coordinates).map(Some);
                    };
                    *previous = Some(found);
                    self.rank = 0;
                    self.accumulate(list, found.0, coordinates)?;
                    return Ok(None);
                }
            }
        }
        self.rank += 1;
        if let Some(index) = index {
            self.accumulate(list, index, coordinates)?;
        }
        Ok(None)
    }

    fn accumulate(&mut self, list: &Dictionary, index: usize, coordinates: bool) -> Result<(), EvalError> {
        let value = read_dict(list, index_text(index, &mut [0; INDEX_DIGITS]))?;
        let total = self.total.unwrap_or_else(|| match value.schema().filter(|_| coordinates) {
            Some("point") => Payload::Coordinates("point", Vec3::new(0.0, 0.0, 0.0)),
            Some("vector") => Payload::Coordinates("vector", Vec3::new(0.0, 0.0, 0.0)),
            _ => Payload::Number(0.0),
        });
        self.total = Some(match total {
            Payload::Number(total) => Payload::Number(total + read_value_number(value)?),
            Payload::Coordinates(kind, total) => Payload::Coordinates(kind, total + read_xyz(value)?),
        });
        Ok(())
    }

    fn finish(&self, coordinates: bool) -> Result<MathOutput, EvalError> {
        match self.total {
            Some(Payload::Number(total)) => Ok(MathOutput::Number("sum", total)),
            Some(Payload::Coordinates(kind, total)) => Ok(MathOutput::Coordinates("sum", kind, total)),
            None if coordinates => Err(EvalError::MissingInput("items".into())),
            None => Ok(MathOutput::Number("sum", 0.0)),
        }
    }
}

impl MathAction {
    fn advance(self, input: &Dictionary, fold: &mut ListFold) -> Result<Option<MathOutput>, EvalError> {
        match self {
            Self::Sum => fold.step(read_dict(input, "list")?, false),
            Self::Add => match input.get("items").and_then(Value::as_dictionary) {
                Some(items) => fold.step(items, true),
                None => compute(self, input).map(Some),
            },
            _ => compute(self, input).map(Some),
        }
    }
}

pub(super) fn evaluate(action: MathAction, input: &Dictionary) -> Result<Dictionary, EvalError> {
    let mut fold = ListFold::default();
    loop {
        if let Some(output) = action.advance(input, &mut fold)? {
            return Ok(output.cold(input));
        }
    }
}

pub(super) type Admission = Result<(OperatorPlanAdmission, RetainedCloneProgress), (EvalError, Dictionary)>;
pub(super) const PLAN_BYTES: usize = size_of::<MathJob>();

pub(super) fn admit(action: MathAction, input: Dictionary, grant: RetainedCloneGrant) -> Admission {
    let receipt = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: PLAN_BYTES, ..Default::default() };
    if !receipt.fits(grant) {
        return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "math plan allocation was not admitted").into(), input));
    }
    if grant.maximum_depth == 0 {
        return Err((ValueError::literal(ValueRefusalKind::DepthLimit, "math plan requires admitted depth").into(), input));
    }
    let job =
        MathJob { action, input: Some(input), fold: ListFold::default(), result: None, writer: None, retirement: ValueRetirement::default(), receipt: RetainedCloneProgress::default(), units: 0, phase: "math-compute", cancelled: false, done: false };
    Ok((OperatorPlanAdmission::Job(Box::new(job)), receipt))
}

struct MathJob {
    action: MathAction,
    input: Option<Dictionary>,
    fold: ListFold,
    result: Option<MathOutput>,
    writer: Option<RetainedDictionaryInput>,
    retirement: ValueRetirement,
    receipt: RetainedCloneProgress,
    units: usize,
    phase: &'static str,
    cancelled: bool,
    done: bool,
}

impl MathJob {
    fn output_receipt(&self) -> Option<RetainedCloneProgress> {
        Some(self.result?.output_receipt(self.input.as_ref()?))
    }

    fn advanced(&mut self, receipt: RetainedCloneProgress, cx: &mut StepContext<'_>) -> Result<(), OperatorWorkFailure> {
        self.receipt = receipt;
        cx.consume_retained(receipt)?;
        self.units = self.units.saturating_add(receipt.copied_items);
        Ok(())
    }

    fn write_step(&mut self, cx: &mut StepContext<'_>) -> Result<Option<Dictionary>, OperatorWorkFailure> {
        let grant = cx.retained_grant();
        let writer = self.writer.as_mut().ok_or_else(|| ValueError::invariant("math output writer is absent"))?;
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| writer.step(effect))? else {
            return Ok(None);
        };
        cx.consume_work(work);
        let step = result.map_err(|error| {
            self.receipt = error.retained_progress();
            OperatorWorkFailure::debit_evaluation(error.into(), self.receipt, cx)
        })?;
        if work == 0 && step.progress != RetainedCloneProgress::default() {
            cx.consume_fuel(1);
        }
        self.advanced(step.progress, cx)?;
        if step.dictionary.is_some() {
            self.writer = None;
        }
        Ok(step.dictionary)
    }
}

impl OperatorJob for MathJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<(OperatorJobStep, RetainedCloneProgress), OperatorWorkFailure> {
        self.receipt = RetainedCloneProgress::default();
        if cx.is_cancelled() {
            self.cancel();
        }
        if self.cancelled {
            return Ok((OperatorJobStep::Cancelled(self.progress()), self.receipt));
        }
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok((OperatorJobStep::Working(self.progress()), self.receipt));
        }
        if self.done {
            return Err(ValueError::invariant("finished math output was already moved").into());
        }
        if self.writer.is_some() {
            if let Some(dictionary) = self.write_step(cx)? {
                self.done = true;
                self.phase = "math-done";
                return Ok((OperatorJobStep::Done(dictionary), self.receipt));
            }
        } else if let Some(receipt) = self.output_receipt() {
            if receipt.fits(grant) {
                let output = self.result.take().ok_or_else(|| ValueError::invariant("math result is absent"))?;
                let input = self.input.as_ref().ok_or_else(|| ValueError::invariant("math input is absent"))?;
                let output = output.typed(input);
                self.writer = Some(RetainedDictionaryInput::new(output));
                cx.consume_fuel(1);
                self.advanced(receipt, cx)?;
            }
        } else if grant.maximum_copy_bytes >= MAX_EVAL_ERROR_BYTES && grant.maximum_capacity_bytes >= MAX_EVAL_ERROR_BYTES {
            let input = self.input.as_ref().ok_or_else(|| ValueError::invariant("math input is absent"))?;
            cx.consume_fuel(1);
            match self.action.advance(input, &mut self.fold) {
                Ok(output) => self.result = output,
                Err(error) => {
                    let (copied_bytes, retained_capacity_bytes) = match &error {
                        EvalError::MissingInput(key) | EvalError::InvalidInput(key) => (key.len(), key.capacity()),
                        _ => (0, 0),
                    };
                    self.receipt = RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes, ..Default::default() };
                    return Err(OperatorWorkFailure::debit_evaluation(error, self.receipt, cx));
                }
            }
            if self.result.is_some() {
                self.phase = "math-output";
            }
            self.advanced(RetainedCloneProgress { copied_items: 1, ..Default::default() }, cx)?;
        }
        Ok((OperatorJobStep::Working(self.progress()), self.receipt))
    }

    fn normal_step_progress(&self) -> RetainedCloneProgress {
        self.receipt
    }

    fn next_step_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or_else(|| Ok(self.output_receipt().map_or(MAX_EVAL_ERROR_BYTES, |receipt| receipt.copied_bytes)), RetainedDictionaryInput::next_copy_byte_demand)
    }

    fn next_step_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or_else(|| Ok(self.output_receipt().map_or(MAX_EVAL_ERROR_BYTES, |receipt| receipt.retained_capacity_bytes)), |writer| writer.next_capacity_byte_demand(copy))
    }

    fn next_step_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or(Ok(0), RetainedDictionaryInput::next_release_byte_demand)
    }

    fn next_step_depth_demand(&self) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or(Ok(1), RetainedDictionaryInput::next_depth_demand)
    }

    fn progress(&self) -> OperatorProgress {
        OperatorProgress { units_done: self.units, units_total: self.units.saturating_add(usize::from(!self.done)), phase: self.phase }
    }

    fn cancel(&mut self) {
        if !self.done {
            self.cancelled = true;
            self.phase = "math-cancelled";
            if let Some(writer) = self.writer.as_mut() {
                writer.cancel();
            }
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or_else(|| self.retirement.next_copy_byte_demand(), RetainedDictionaryInput::next_close_copy_byte_demand)
    }

    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or_else(|| self.retirement.next_capacity_byte_demand(copy), |writer| writer.next_close_capacity_byte_demand(copy))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        self.writer.as_ref().map_or_else(|| self.retirement.next_release_byte_demand(), RetainedDictionaryInput::next_close_release_byte_demand)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        match self.writer.as_ref() {
            Some(writer) => writer.next_close_depth_demand(),
            None => Ok(self.retirement.next_depth_demand()?.max(usize::from(self.input.is_some()))),
        }
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.cancel();
        let grant = effect.grant();
        if let Some(writer) = self.writer.as_mut() {
            let mut poll = writer.close_step(grant.maximum_copy_bytes, effect);
            if writer.terminal_is_empty() {
                self.writer = None;
            }
            if let RetainedWorkPoll::Complete(turn) = &mut poll {
                if let Ok(step) = &mut turn.result {
                    if self.input.is_some() {
                        *step = RetainedCloneStep::Progress(step.progress());
                    }
                }
            }
            return poll;
        }
        if self.input.is_none() {
            return self.retirement.close_step(grant.maximum_copy_bytes, effect);
        }
        let before = effect.performed_work();
        let result = if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))
        } else {
            effect.performed(1).map(|paid| {
                if paid {
                    self.retirement = ValueRetirement::from_dictionary(self.input.take().expect("math input is present"));
                    self.result = None;
                }
                RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(paid), ..Default::default() })
            })
        };
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: result.map_err(Into::into), work: effect.performed_work() - before })
    }

    fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.writer.is_none() && self.retirement.terminal_is_empty()
    }
}
