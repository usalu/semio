//! 🖼️ Projects the exact SetImage Samples record with native paged children.

use semio_framework_dsl_record::receiving::OwnedSlot;
use semio_framework_pack_json::{JsonError, JsonGrammarCursor, JsonMemberPolicy, JsonNumberArrayCursor, PagedJsonNumbers};
use semio_framework_value::{
    DslValue, NativeDecodeControl, OriginalRetirementPoll, RetainedAllocationEffect,
    RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome,
    RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind, retained_clone::retained_clone_grant_admits,
};
use semio_framework_value_native_owned::native_decoding::{
    OriginalEffectReply, OriginalEffectReplyComplete,
    owned::{NativeRecordKey, NativeSchemaKeySource, NativeSourceKeyReply, NativeSourceKeyReplyComplete, answer_schema_key,
        tree::{NativeScalarQuery, NativeSourceScalar, NativeTreeKind, NativeValueSource}},
};

const PATH: [&str; 3] = ["image", "body", "values"];
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

#[derive(semio_framework_value::RetireOwned)]
pub struct ProjectedRecord<C: semio_framework_value::RetireOwned> {
    shape: DslValue,
    index: usize,
    child: C,
}

impl<C: NativeValueSource + semio_framework_value::RetireOwned> NativeValueSource for ProjectedRecord<C> {
    fn as_source(&self) -> &dyn NativeValueSource { self }
    fn tree_kind(&self) -> Option<NativeTreeKind> { self.shape.tree_kind() }
    fn record_len(&self) -> Result<usize, ValueError> { self.shape.record_len() }
    fn record_key(&self, index: usize) -> Result<NativeRecordKey<'_>, ValueError> { self.shape.record_key(index) }
    fn record_child(&self, index: usize) -> Result<&dyn NativeValueSource, ValueError> {
        if index == self.index { Ok(&self.child) } else { self.shape.record_child(index) }
    }
    fn record_entry(&self, index: usize) -> Result<&dyn NativeValueSource, ValueError> {
        if index == self.index {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "projected record member requires named access"));
        }
        self.shape.record_entry(index)
    }
    fn query_scalar<'r>(
        &self, query: NativeScalarQuery, effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'r, NativeSourceScalar, ValueError>,
    ) -> Result<OriginalEffectReplyComplete<'r, NativeSourceScalar, ValueError>, ValueError> {
        self.shape.query_scalar(query, effect, reply)
    }
    fn recover_scalar<'r>(
        &self, query: NativeScalarQuery, effect: &mut RetainedAllocationEffect<'_>,
        reply: OriginalEffectReply<'r, NativeSourceScalar, ValueError>,
    ) -> Result<OriginalEffectReplyComplete<'r, NativeSourceScalar, ValueError>, ValueError> {
        self.shape.recover_scalar(query, effect, reply)
    }
    fn query_schema_key<'r, 's>(
        &'s self, query: NativeSchemaKeySource, effect: &mut RetainedAllocationEffect<'_>,
        reply: NativeSourceKeyReply<'r, 's>,
    ) -> Result<NativeSourceKeyReplyComplete<'r>, ValueError> {
        answer_schema_key(self, query, effect, reply)
    }
}

pub type SetImageSource = ProjectedRecord<ProjectedRecord<ProjectedRecord<PagedJsonNumbers>>>;

/// 🪢 Holds the exact archive and payload extents alongside the lifetime-free native source.
pub struct AnchoredSetImageSource<'a> {
    original: &'a [u8],
    payload: &'a [u8],
    source: SetImageSource,
}

impl<'a> AnchoredSetImageSource<'a> {
    pub fn projected(&self) -> &SetImageSource { &self.source }

    pub fn with_original<T>(self, receive: impl FnOnce(&'a [u8], &'a [u8], SetImageSource) -> T) -> T {
        receive(self.original, self.payload, self.source)
    }
}

#[derive(Clone, Copy)]
enum Phase { Parse, Array, Marker, TakeArray, Select, Project, Ready, Failed, Closing, Closed }

#[derive(Default)]
struct PathSelection {
    indices: [Option<usize>; 3],
    layer: usize,
    field: usize,
    tag_validated: bool,
}

fn invalid(detail: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, detail)
}

fn fixed(copy_bytes: usize, depth: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth, ..Default::default() }
}

fn receipt(copy_bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() }
}

fn fields(value: &DslValue) -> Result<&[(String, DslValue)], ValueError> {
    match value {
        DslValue::Object(fields) => Ok(fields),
        _ => Err(invalid("SetImage member path requires an object")),
    }
}

fn split(value: &mut DslValue, index: usize) -> DslValue {
    let DslValue::Object(fields) = value else { unreachable!("validated SetImage record") };
    std::mem::replace(&mut fields[index].1, DslValue::Null)
}

impl PathSelection {
    fn step(&mut self, root: &DslValue) -> Result<bool, ValueError> {
        let shape = match self.layer {
            0 => root,
            1 => &fields(root)?[self.indices[0].unwrap()].1,
            2 => &fields(&fields(root)?[self.indices[0].unwrap()].1)?[self.indices[1].unwrap()].1,
            _ => unreachable!("completed SetImage path selection"),
        };
        let record = fields(shape)?;
        if let Some((name, value)) = record.get(self.field) {
            if name == PATH[self.layer] {
                self.indices[self.layer] = Some(self.field);
            }
            if matches!((self.layer, name.as_str()), (0, "mutation") | (2, "kind")) {
                let expected = if self.layer == 0 { "setImage" } else { "samples" };
                if value.as_str() != Some(expected) {
                    return Err(invalid("SetImage frame tag or Samples discriminator does not match"));
                }
                self.tag_validated = true;
            }
            self.field += 1;
            return Ok(false);
        }
        if self.indices[self.layer].is_none() || (self.layer != 1 && !self.tag_validated) {
            return Err(invalid("SetImage projection requires the selected Samples field and its discriminators"));
        }
        self.layer += 1;
        self.field = 0;
        self.tag_validated = false;
        Ok(self.layer == PATH.len())
    }
}

/// 📥️ Projects one selected mutation leaf while the exact archive source remains immutably borrowed.
pub struct SetImageProjection<'a> {
    original: &'a [u8],
    payload: &'a [u8],
    phase: Phase,
    parser: OwnedSlot<JsonGrammarCursor<DslValue>>,
    array: Option<JsonNumberArrayCursor<'a>>,
    syntax: OwnedSlot<JsonError>,
    value: OwnedSlot<DslValue>,
    pages: OwnedSlot<PagedJsonNumbers>,
    output: OwnedSlot<SetImageSource>,
    selection: PathSelection,
}

impl<'a> SetImageProjection<'a> {
    /// 🎟️ The expected tag is the existing SetImage tag from the archive codec registry.
    pub fn admit(original: &'a [u8], payload_range: std::ops::Range<usize>, framed_tag: u8, set_image_tag: u8, grant: RetainedCloneGrant) -> Result<Option<Self>, ValueError> {
        let payload = original.get(payload_range).ok_or_else(|| invalid("SetImage frame escaped its original source"))?;
        if framed_tag != set_image_tag { return Err(invalid("binary tag does not select SetImage")); }
        if payload.len() > MAX_PAYLOAD_BYTES { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "SetImage JSON exceeds the archive payload bound")); }
        if !retained_clone_grant_admits(grant, Self::admission_demand()) { return Ok(None); }
        let mut parser = OwnedSlot::<JsonGrammarCursor<DslValue>>::default();
        *parser.value = Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject));
        Ok(Some(Self {
            original, payload, parser, phase: Phase::Parse, array: None, selection: Default::default(),
            syntax: Default::default(), value: Default::default(), pages: Default::default(), output: Default::default(),
        }))
    }

    pub fn admission_demand() -> RetirementDemand { fixed(size_of::<Self>(), 1) }
    pub fn source(&self) -> &[u8] { self.original }
    pub fn ready(&self) -> bool { matches!(self.phase, Phase::Ready) }
    fn parser(&self) -> &JsonGrammarCursor<DslValue> { self.parser.value.as_ref().expect("admitted SetImage grammar") }

    fn array_start(&self) -> Option<usize> {
        self.pages.value.is_none().then(|| self.parser().selected_number_array_start(self.payload, &PATH)).flatten()
    }

    pub fn demand(&self) -> Result<(RetirementDemand, u64), ValueError> {
        Ok(match self.phase {
            Phase::Parse if self.array_start().is_some() => (JsonNumberArrayCursor::admission_demand(), 1),
            Phase::Parse => {
                let mut demand = self.parser().normal_step_demands(self.payload)
                    .map_err(|error| ValueError::literal(error.kind(), "SetImage JSON demand refused"))?;
                demand.copy_bytes += Self::grammar_metadata();
                (demand, 1)
            }
            Phase::Array => self.array.as_ref().expect("admitted image Samples span").demand()?,
            Phase::Marker => (JsonGrammarCursor::<DslValue>::number_array_marker_demand(), 1),
            Phase::TakeArray => (JsonNumberArrayCursor::take_demand(), 1),
            Phase::Select => (fixed(size_of::<PathSelection>(), 3), 1),
            Phase::Project => (fixed(3 * size_of::<DslValue>() + 2 * size_of::<Option<SetImageSource>>() + size_of::<Option<PagedJsonNumbers>>(), 3), 1),
            _ => (Default::default(), 0),
        })
    }

    fn grammar_metadata() -> usize {
        size_of::<Option<DslValue>>() + size_of::<Option<JsonError>>() + size_of::<Phase>()
    }

    /// ⏯️ Provides one checkpoint per funded grammar, sample or projection action.
    pub fn advance(&mut self, native: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let boundary = effect.performed_boundary();
        let result = self.advance_one(native, effect);
        if result.is_err() && !matches!(self.phase, Phase::Closing | Phase::Closed) {
            self.phase = Phase::Failed;
        }
        let work = effect.performed_since(boundary).expect("SetImage projection retains its performed work");
        RetainedWorkOutcome { result, work }
    }

    fn advance_one(&mut self, native: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let (demand, work) = self.demand()?;
        let grant = effect.grant();
        if work == 0 || !retained_clone_grant_admits(grant, demand) || effect.remaining_work_units()? < work { return Ok(Default::default()); }
        native.checkpoint()?;
        match self.phase {
            Phase::Parse => {
                if let Some(start) = self.array_start() {
                    if !effect.performed(1)? { return Ok(Default::default()); }
                    self.array = JsonNumberArrayCursor::admit_embedded(self.payload, start, MAX_PAYLOAD_BYTES, MAX_PAYLOAD_BYTES, grant)?;
                    self.phase = Phase::Array;
                    return Ok(receipt(demand.copy_bytes));
                }
                let parser = self.parser.value.as_mut().expect("admitted SetImage grammar");
                let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - Self::grammar_metadata(), ..grant };
                let turn = parser.step_source(self.payload, 1, native, &mut effect.nested(child), false);
                let mut progress = parser.normal_step_progress();
                if progress.copied_items != 0 { progress.copied_bytes += Self::grammar_metadata(); }
                match turn {
                    Ok(Some(result)) => { *self.value.value = Some(result); self.phase = Phase::Select; }
                    Ok(None) => {}
                    Err(JsonError::Native(error)) => return Err(error.with_retained_progress(progress)),
                    Err(error) => {
                        let kind = error.kind();
                        *self.syntax.value = Some(error);
                        return Err(ValueError::literal(kind, "SetImage JSON payload was refused").with_retained_progress(progress));
                    }
                }
                return Ok(progress);
            }
            Phase::Array => {
                let array = self.array.as_mut().expect("admitted image Samples span");
                let progress = array.advance(effect, || true).result?;
                if array.is_ready() { self.phase = Phase::Marker; }
                return Ok(progress);
            }
            Phase::Marker => {
                let span = self.array.as_ref().and_then(JsonNumberArrayCursor::finished_span).expect("validated image Samples span");
                let parser = self.parser.value.as_mut().expect("admitted SetImage grammar");
                let progress = parser.accept_number_array_marker(self.payload, &PATH, &span, effect)?;
                if progress.copied_items != 0 { self.phase = Phase::TakeArray; }
                return Ok(progress);
            }
            Phase::TakeArray => {
                let progress = self.array.as_mut().expect("validated image Samples span").take(&mut self.pages.value, effect)?;
                if self.pages.value.is_some() { self.array = None; self.phase = Phase::Parse; }
                return Ok(progress);
            }
            _ => {}
        }
        if !effect.performed(1)? { return Ok(Default::default()); }
        let progress = receipt(demand.copy_bytes);
        if matches!(self.phase, Phase::Select) {
            let ready = self.selection.step(self.value.value.as_ref().expect("projecting parsed SetImage payload"))
                .map_err(|error| error.with_retained_progress(progress))?;
            if ready { self.phase = Phase::Project; }
        } else {
            self.project().map_err(|error| error.with_retained_progress(progress))?;
        }
        Ok(progress)
    }

    fn project(&mut self) -> Result<(), ValueError> {
        let [root_index, image_index, values_index] = self.selection.indices.map(Option::unwrap);
        let children = self.pages.value.take().ok_or_else(|| invalid("SetImage Samples values must be a JSON numeric array"))?;
        let mut root = self.value.value.take().expect("validated SetImage root");
        let mut image = split(&mut root, root_index);
        let body = split(&mut image, image_index);
        *self.output.value = Some(ProjectedRecord { shape: root, index: root_index,
            child: ProjectedRecord { shape: image, index: image_index,
                child: ProjectedRecord { shape: body, index: values_index, child: children } },
        });
        self.phase = Phase::Ready;
        Ok(())
    }

    pub fn take(&mut self, result: &mut Option<AnchoredSetImageSource<'a>>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        let demand = fixed(size_of::<Option<SetImageSource>>() + size_of::<Option<AnchoredSetImageSource<'_>>>(), 1);
        if !self.ready() || result.is_some() { return Err(ValueError::invariant("SetImage projection is not ready for transfer")); }
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
            return Ok(Default::default());
        }
        let source = self.output.value.take().expect("ready SetImage source");
        *result = Some(AnchoredSetImageSource { original: self.original, payload: self.payload, source });
        self.phase = Phase::Closing;
        Ok(receipt(demand.copy_bytes))
    }

    pub fn terminal_is_empty(&self) -> bool { matches!(self.phase, Phase::Closed) }

    /// ♻️ Clears all unpublished parser, source and result owners under the caller's effect.
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let result = (|| {
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if matches!(self.phase, Phase::Closing) {
                if let Some(array) = self.array.as_mut() {
                    let turn = RetirementPollExit::resume(array.close(effect))?;
                    if array.terminal_is_empty() { self.array = None; }
                    return Ok(RetainedCloneStep::Progress(turn.progress()));
                }
                macro_rules! close { ($($slot:ident),+) => { $(if !self.$slot.terminal_is_empty() {
                    return RetirementPollExit::resume(self.$slot.close_step(effect)).map(RetainedCloneStep::Progress);
                })+ }; }
                close!(output, pages, value, syntax, parser);
            }
            let demand = fixed(size_of::<Phase>(), 1);
            if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if matches!(self.phase, Phase::Closing) {
                self.phase = Phase::Closed;
                Ok(RetainedCloneStep::Complete(receipt(demand.copy_bytes)))
            } else {
                self.phase = Phase::Closing;
                Ok(RetainedCloneStep::Progress(receipt(demand.copy_bytes)))
            }
        })();
        RetirementPollExit::settle(effect, result)
    }
}
