//! 🖌️ Session of the puzzle 5d editor: the reserved clipboard, paste and import jobs with their fixed-page state machines. See [`super`].

use super::*;

//#region 🧵️ReservedJobs
pub(super) fn puzzle5d_preflight_reserved_wire(raw: Vec<u8>, maximum_bytes: usize) -> Result<Vec<u8>, (Fault, Vec<u8>)> {
    if raw.len() > maximum_bytes {
        return Err((Fault::from("puzzle5d reserved wire exceeds its exact route cap before fixed-page copy"), raw));
    }
    Ok(raw)
}

fn puzzle5d_job_fault(detail: impl AsRef<str>) -> crate::puzzle_job::JobTurn {
    let bytes = detail.as_ref().as_bytes();
    crate::puzzle_job::JobTurn::Fault(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec())
}

fn puzzle5d_job_checkpoint(stage: u8, cursor: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    let mut state = [0; 17];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&progress.to_le_bytes());
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: state.to_vec() }
}

fn puzzle5d_import_checkpoint_bytes(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> [u8; 33] {
    let mut state = [0; 33];
    state[0] = stage;
    state[1..9].copy_from_slice(&(cursor as u64).to_le_bytes());
    state[9..17].copy_from_slice(&(nested_cursor as u64).to_le_bytes());
    state[17..25].copy_from_slice(&(decoded_items as u64).to_le_bytes());
    state[25..33].copy_from_slice(&progress.to_le_bytes());
    state
}

fn puzzle5d_import_checkpoint(stage: u8, cursor: usize, nested_cursor: usize, decoded_items: usize, progress: u64) -> crate::puzzle_job::JobTurn {
    crate::puzzle_job::JobTurn::Checkpoint { applied_progress: progress, state: puzzle5d_import_checkpoint_bytes(stage, cursor, nested_cursor, decoded_items, progress).to_vec() }
}

fn puzzle5d_step_envelope(raw: &[u8], cursor: &mut usize, page: &mut [u8; PUZZLE5D_RESERVED_PAGE_BYTES], page_len: &mut usize, progress: &mut u64, cx: &mut StepContext<'_>) -> Option<crate::puzzle_job::JobTurn> {
    if *cursor >= raw.len() {
        *page_len = 0;
        return None;
    }
    let units = raw.len().saturating_sub(*cursor).min(page.len()).min(cx.fuel_remaining() as usize);
    if units == 0 {
        return Some(crate::puzzle_job::JobTurn::Yield);
    }
    let end = cursor.checked_add(units).filter(|end| *end <= raw.len()).expect("Puzzle5d fixed-page ingress preflights the source range before copy");
    page[..units].copy_from_slice(&raw[*cursor..end]);
    *page_len = units;
    *cursor = end;
    *progress = progress.saturating_add(units as u64);
    cx.consume_fuel(units as u64);
    Some(puzzle5d_job_checkpoint(0, *cursor, *progress))
}

fn puzzle5d_selection_ids(interaction: &semio_framework::InteractionState) -> (HashSet<String>, HashSet<String>) {
    match interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN) {
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_PART => (selection.ids.iter().cloned().collect(), HashSet::new()),
        Some(selection) if selection.granularity == PUZZLE5D_GRANULARITY_FASTENER => (HashSet::new(), selection.ids.iter().cloned().collect()),
        _ => (HashSet::new(), HashSet::new()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dSelectionStage {
    Endpoints,
    Fasteners,
    Parts,
    Complete,
}

/// ♻️ The scan's retained owners, staged whole when its job closes.
#[derive(Default, semio_framework_value::RetireOwned)]
struct Puzzle5dSelectionScanOwners {
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) projection: Option<Value>,
    pub(super) part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    pub(super) parts: Vec<Puzzle5dPart>,
    pub(super) fasteners: Vec<Puzzle5dFastener>,
}

struct Puzzle5dSelectionScan {
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) projection: Option<Value>,
    pub(super) part_ids: HashSet<String>,
    explicit_fastener_ids: HashSet<String>,
    pub(super) stage: Puzzle5dSelectionStage,
    pub(super) cursor: usize,
    pub(super) parts: Vec<Puzzle5dPart>,
    pub(super) fasteners: Vec<Puzzle5dFastener>,
}

impl Puzzle5dSelectionScan {
    fn into_owners(&mut self) -> Puzzle5dSelectionScanOwners {
        Puzzle5dSelectionScanOwners { snapshot: self.snapshot.take(), projection: self.projection.take(), part_ids: std::mem::take(&mut self.part_ids), explicit_fastener_ids: std::mem::take(&mut self.explicit_fastener_ids), parts: std::mem::take(&mut self.parts), fasteners: std::mem::take(&mut self.fasteners) }
    }

    pub(super) fn new(snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        let (part_ids, explicit_fastener_ids) = puzzle5d_selection_ids(interaction);
        Self { snapshot: Some(snapshot), projection: None, part_ids, explicit_fastener_ids, stage: Puzzle5dSelectionStage::Endpoints, cursor: 0, parts: Vec::new(), fasteners: Vec::new() }
    }

    /// 🗂️ One cursored row of the scanned projection, which is derived ONCE and then cached. Re-deriving
    /// it per step — and cloning the whole row array with it — cost O(document) on every one of a
    /// selection's ~2N+M cursor turns and blew the interactive ceiling on a real document:
    /// `framework route 'copy' overran the 8 ms step ceiling for 4 consecutive steps, worst 70521us`
    /// on the 180-part Nakagin, which is what made `copy`/`cut` fault the moment the shipped examples
    /// stopped loading empty. The cache is retired on its own rung of the close ladder.
    pub(super) fn row(&mut self, key: &str, index: usize) -> Option<Value> {
        if self.projection.is_none() {
            let projection = self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot))?;
            self.projection = Some(projection);
        }
        self.projection.as_ref()?.get(key).and_then(Value::as_array).and_then(|rows| rows.get(index)).cloned()
    }

    pub(super) fn step(&mut self) -> Result<bool, String> {
        match self.stage {
            Puzzle5dSelectionStage::Endpoints => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id)) {
                        if let Some(source) = row.get("source").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(source).to_string());
                        }
                        if let Some(target) = row.get("target").and_then(Value::as_str) {
                            self.part_ids.insert(owning_part_id_local(target).to_string());
                        }
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Fasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Fasteners => {
                let cursor = self.cursor;
                if let Some(row) = self.row("fasteners", cursor) {
                    self.cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).map(owning_part_id_local);
                    let target = row.get("target").and_then(Value::as_str).map(owning_part_id_local);
                    let selected = row.get("id").and_then(Value::as_str).is_some_and(|id| self.explicit_fastener_ids.contains(id))
                        || source.zip(target).is_some_and(|(source, target)| !self.part_ids.is_empty() && self.part_ids.contains(source) && self.part_ids.contains(target));
                    if selected {
                        self.fasteners.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dSelectionStage::Parts => {
                let cursor = self.cursor;
                if let Some(row) = self.row("parts", cursor) {
                    self.cursor += 1;
                    if row.get("id").and_then(Value::as_str).is_some_and(|id| self.part_ids.contains(id)) {
                        self.parts.push(puzzle5d_record_from_projection(row).map_err(|error| error.to_string())?);
                    }
                } else {
                    self.stage = Puzzle5dSelectionStage::Complete;
                }
            }
            Puzzle5dSelectionStage::Complete => return Ok(true),
        }
        Ok(self.stage == Puzzle5dSelectionStage::Complete)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dClipboardStage {
    Envelope,
    Select,
    EncodeParts,
    EncodeFasteners,
    Complete,
}


struct Puzzle5dClipboardWork {
    pub(super) raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    pub(super) progress: u64,
    pub(super) stage: Puzzle5dClipboardStage,
    pub(super) scan: Puzzle5dSelectionScan,
    encode_cursor: usize,
    pub(super) dsl_text: String,
}

enum Puzzle5dClipboardWorkStep {
    Outcome(crate::puzzle_job::JobTurn),
    Pending,
    Complete,
}

impl Puzzle5dClipboardWork {
    pub(super) fn new(raw: Vec<u8>, snapshot: std::sync::Arc<Puzzle5dPlaySnapshot>, interaction: &semio_framework::InteractionState) -> Self {
        Self {
            raw,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dClipboardStage::Envelope,
            scan: Puzzle5dSelectionScan::new(snapshot, interaction),
            encode_cursor: 0,
            dsl_text: String::new(),
        }
    }

    fn step_work(&mut self, cx: &mut StepContext<'_>) -> Result<Puzzle5dClipboardWorkStep, String> {
        match self.stage {
            Puzzle5dClipboardStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return Ok(Puzzle5dClipboardWorkStep::Outcome(outcome));
                }
                self.stage = Puzzle5dClipboardStage::Select;
            }
            Puzzle5dClipboardStage::Select => {
                if !self.scan.step()? {
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text = format!("{{\"schema\":{},\"parts\":[", semio_framework_pack_json::to_json_string(&PUZZLE5D_SCHEMA));
                self.stage = Puzzle5dClipboardStage::EncodeParts;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeParts => {
                if let Some(part) = self.scan.parts.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(part));
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("],\"fasteners\":[");
                self.stage = Puzzle5dClipboardStage::EncodeFasteners;
                self.encode_cursor = 0;
            }
            Puzzle5dClipboardStage::EncodeFasteners => {
                if let Some(fastener) = self.scan.fasteners.get(self.encode_cursor) {
                    if self.encode_cursor != 0 {
                        self.dsl_text.push(',');
                    }
                    self.dsl_text.push_str(&semio_framework_pack_json::to_json_string(fastener));
                    self.encode_cursor += 1;
                    self.progress = self.progress.saturating_add(1);
                    cx.consume_fuel(1);
                    return Ok(Puzzle5dClipboardWorkStep::Pending);
                }
                self.dsl_text.push_str("]}");
                if self.dsl_text.len() > PUZZLE5D_RESERVED_OUTPUT_BYTES {
                    return Err("puzzle5d clipboard fragment exceeds output cap".into());
                }
                self.stage = Puzzle5dClipboardStage::Complete;
            }
            Puzzle5dClipboardStage::Complete => return Ok(Puzzle5dClipboardWorkStep::Complete),
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        Ok(if self.stage == Puzzle5dClipboardStage::Complete { Puzzle5dClipboardWorkStep::Complete } else { Puzzle5dClipboardWorkStep::Pending })
    }

    pub(super) fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.encode_cursor.max(self.scan.cursor), self.progress)
    }

    pub(super) fn fragment(&self) -> Option<ClipboardFragment> {
        (!self.scan.parts.is_empty()).then(|| ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
            dsl_text: self.dsl_text.clone(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", self.scan.parts.len()),
        })
    }
}

/// 📋️ Which clipboard verb one [`Puzzle5dClipboardJob`] answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Puzzle5dClipboardVerb {
    Copy,
    Cut,
}

/// ♻️ The owners one `Puzzle5dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dClipboardOwners {
    pub(super) raw: Vec<u8>,
    pub(super) dsl_text: String,
    pub(super) scan: Puzzle5dSelectionScanOwners,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub(super) struct Puzzle5dClipboardJob {
    pub(super) verb: Puzzle5dClipboardVerb,
    pub(super) work: Puzzle5dClipboardWork,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    pub(super) closing: bool,
    pub(super) owners: crate::puzzle_job::WorkClosing<Puzzle5dClipboardOwners>,
}

impl Puzzle5dClipboardJob {
    pub(super) fn new(verb: Puzzle5dClipboardVerb, request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, interaction: &semio_framework::InteractionState) -> Self {
        Self { verb, work: Puzzle5dClipboardWork::new(request.raw_wire, request.snapshot, interaction), completion: Some(request.completion), rejection: None, completed: false, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    pub(super) fn label(&self) -> &'static str {
        match self.verb {
            Puzzle5dClipboardVerb::Copy => "puzzle5d copy",
            Puzzle5dClipboardVerb::Cut => "puzzle5d cut",
        }
    }

    pub(super) fn emit(&self) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        let effects = self.work.fragment().map(|fragment| vec![Effect::ClipboardWrite { fragment }]).unwrap_or_default();
        match self.verb {
            Puzzle5dClipboardVerb::Copy => Emit { effects, ..Default::default() },
            Puzzle5dClipboardVerb::Cut => {
                // 🔒️ A LOCKED part is copied but never removed: a lock exists precisely to refuse a
                // destructive gesture, and a cut that deleted it anyway would be the one clipboard
                // path that ignores it. Its fasteners stay too — disconnecting an edge whose part
                // survives would leave the document half-cut.
                let locked: HashSet<&str> = self.work.scan.parts.iter().filter(|part| part.part_2d.locked.unwrap_or(false)).map(|part| part.id.as_str()).collect();
                let mut mutations = self
                    .work
                    .scan
                    .fasteners
                    .iter()
                    .filter(|fastener| !locked.contains(owning_part_id_local(&fastener.source)) && !locked.contains(owning_part_id_local(&fastener.target)))
                    .map(|fastener| crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(fastener.id.clone()))
                    .collect::<Vec<_>>();
                mutations.extend(self.work.scan.parts.iter().filter(|part| !locked.contains(part.id.as_str())).map(|part| crate::standards::v1::subsets::any::schema::mutations::delete_part(part.id.clone())));
                Emit { artifact_mutations: mutations, effects, ..Default::default() }
            }
        }
    }

    pub(super) fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.rejection.is_some() {
            return puzzle5d_job_fault(format!("{} completion remains rejected", self.label()));
        }
        if self.completed {
            return crate::puzzle_job::JobTurn::Complete;
        }
        match self.work.step_work(cx) {
            Ok(Puzzle5dClipboardWorkStep::Outcome(outcome)) => return outcome,
            Ok(Puzzle5dClipboardWorkStep::Pending) => return self.work.checkpoint(),
            Err(error) => return puzzle5d_job_fault(error),
            Ok(Puzzle5dClipboardWorkStep::Complete) => {}
        }
        if !self.work.raw.is_empty() {
            return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.work.raw));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault(format!("{} lost its completion authority", self.label())) };
        if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
            let message = rejected.fault.message.clone();
            self.rejection = Some(rejected);
            return puzzle5d_job_fault(message);
        }
        self.completed = true;
        crate::puzzle_job::JobTurn::Complete
    }
}

impl InteractiveJob for Puzzle5dClipboardJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dClipboardOwners {
            raw: std::mem::take(&mut self.work.raw),
            dsl_text: std::mem::take(&mut self.work.dsl_text),
            scan: self.work.scan.into_owners(),
            completion: self.completion.take(),
            rejection: self.rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dClipboardJob {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPasteStage {
    Envelope,
    Decode,
    FragmentParts,
    TargetParts,
    TargetFasteners,
    MaterializeParts,
    MaterializeFasteners,
    Complete,
}

/// ♻️ The owners one `Puzzle5dPasteJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPasteOwners {
    pub(super) raw: Vec<u8>,
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) args: Option<Value>,
    pub(super) fragment_value: Option<Value>,
    pub(super) fragment_parts: Vec<Puzzle5dPart>,
    pub(super) id_map: HashMap<String, String>,
    pub(super) fresh_ids: Puzzle5dFreshIds,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub(super) struct Puzzle5dPasteJob {
    pub(super) raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    pub(super) progress: u64,
    pub(super) stage: Puzzle5dPasteStage,
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) args: Option<Value>,
    pub(super) fragment_value: Option<Value>,
    pub(super) fragment_parts: Vec<Puzzle5dPart>,
    pub(super) cursor: usize,
    source_sum: (f64, f64),
    target_sum: (f64, f64),
    target_count: usize,
    pub(super) placement: PastePlacement,
    pub(super) delta: (f64, f64),
    pub(super) id_map: HashMap<String, String>,
    pub(super) fresh_ids: Puzzle5dFreshIds,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    pub(super) closing: bool,
    pub(super) owners: crate::puzzle_job::WorkClosing<Puzzle5dPasteOwners>,
}

impl Puzzle5dPasteJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, args: Option<Value>) -> Self {
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dPasteStage::Envelope,
            snapshot: Some(request.snapshot),
            args,
            fragment_value: None,
            fragment_parts: Vec::new(),
            cursor: 0,
            source_sum: (0.0, 0.0),
            target_sum: (0.0, 0.0),
            target_count: 0,
            placement: PastePlacement::default(),
            delta: (0.0, 0.0),
            id_map: HashMap::new(),
            fresh_ids: Puzzle5dFreshIds::default(),
            mutations: Vec::new(),
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    pub(super) fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_job_checkpoint(self.stage as u8, self.cursor, self.progress)
    }
}

impl Puzzle5dPasteJob {
    pub(super) fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d paste completion remains rejected");
        }
        match self.stage {
            Puzzle5dPasteStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dPasteStage::Decode;
            }
            Puzzle5dPasteStage::Decode => {
                let Some(args) = self.args.as_ref() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let Some(fragment_value) = args.get("fragment").cloned() else {
                    self.stage = Puzzle5dPasteStage::Complete;
                    return self.checkpoint();
                };
                let fragment: ClipboardFragment = match puzzle5d_record_from_projection(fragment_value) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                if fragment.media_type != (MediaType { class: MediaClass::Kit, form: MediaForm::Design }) {
                    return puzzle5d_job_fault("puzzle5d paste received an incompatible media type");
                }
                if fragment.dsl_text.len() > PUZZLE5D_RESERVED_RAW_BYTES {
                    return puzzle5d_job_fault("puzzle5d paste fragment exceeds its predecode cap");
                }
                self.fragment_value = match parse(&fragment.dsl_text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                    Ok(value) => Some(value),
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.placement = match puzzle5d_paste_placement(args) {
                    Ok(placement) => placement,
                    Err(error) => return puzzle5d_job_fault(error.to_string()),
                };
                self.stage = Puzzle5dPasteStage::FragmentParts;
                self.cursor = 0;
            }
            Puzzle5dPasteStage::FragmentParts => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("parts")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let part: Puzzle5dPart = match puzzle5d_record_from_projection(row) {
                        Ok(part) => part,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.source_sum.0 += part.part_2d.x;
                    self.source_sum.1 += part.part_2d.y;
                    self.fragment_parts.push(part);
                } else {
                    self.stage = Puzzle5dPasteStage::TargetParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetParts => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().parts).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_part(&row.id);
                    self.target_sum.0 += row.part_2d.x;
                    self.target_sum.1 += row.part_2d.y;
                    self.target_count += 1;
                } else {
                    let offset = self.placement.position.map_or((0.0, 0.0), |position| (position[0], position[1]));
                    self.delta = if matches!(self.placement.anchor, PasteAnchor::Original) || self.fragment_parts.is_empty() || self.target_count == 0 {
                        offset
                    } else {
                        (self.target_sum.0 / self.target_count as f64 - self.source_sum.0 / self.fragment_parts.len() as f64 + offset.0, self.target_sum.1 / self.target_count as f64 - self.source_sum.1 / self.fragment_parts.len() as f64 + offset.1)
                    };
                    self.stage = Puzzle5dPasteStage::TargetFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::TargetFasteners => {
                let rows = self.snapshot.as_ref().map(|snapshot| &snapshot.typed().fasteners).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor) {
                    self.cursor += 1;
                    self.fresh_ids.observe_fastener(&row.id);
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeParts => {
                if let Some(part) = self.fragment_parts.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fresh_id = self.fresh_ids.next_part();
                    self.id_map.insert(part.id.clone(), fresh_id.clone());
                    let mut next = part;
                    next.id = fresh_id;
                    next.part_2d.x += self.delta.0;
                    next.part_2d.y += self.delta.1;
                    next.part_3d.origin[0] += self.delta.0;
                    next.part_3d.origin[1] += self.delta.1;
                    let typed = match <crate::Puzzle5dPart as FromValue>::from_value(next.to_value()) {
                        Ok(typed) => typed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::create_part(typed, None));
                } else {
                    self.stage = Puzzle5dPasteStage::MaterializeFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dPasteStage::MaterializeFasteners => {
                let rows = self.fragment_value.as_ref().and_then(|value| value.get("fasteners")).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
                if let Some(row) = rows.get(self.cursor).cloned() {
                    self.cursor += 1;
                    let fastener: Puzzle5dFastener = match puzzle5d_record_from_projection(row) {
                        Ok(fastener) => fastener,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                        self.fresh_ids.next_fastener(),
                        rewrite_grip_ref_local(&fastener.source, &self.id_map),
                        rewrite_grip_ref_local(&fastener.target, &self.id_map),
                        fastener.fastener_kind,
                        fastener.gap,
                        fastener.shift,
                        fastener.rise,
                        fastener.rotation,
                        fastener.turn,
                        fastener.tilt,
                        fastener.x + self.delta.0,
                        fastener.y + self.delta.1, None,
                    ));
                } else {
                    self.stage = Puzzle5dPasteStage::Complete;
                }
            }
            Puzzle5dPasteStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    // 🕹️ The pasted parts BECOME the selection: a paste whose result is invisible until the
                    // user hunts for it is indistinguishable from one that landed nowhere, and the fresh ids
                    // are the only handle on it. Sorted, so one fragment always re-selects in one order.
                    let mut fresh: Vec<String> = self.id_map.values().cloned().collect();
                    fresh.sort();
                    let emit = Emit { artifact_mutations: std::mem::take(&mut self.mutations), interaction_writes: vec![InteractionWrite::replace(PUZZLE5D_INTERACTION_DOMAIN, PUZZLE5D_GRANULARITY_PART, fresh)], ..Default::default() };
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d paste lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl InteractiveJob for Puzzle5dPasteJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dPasteOwners {
            raw: std::mem::take(&mut self.raw),
            snapshot: self.snapshot.take(),
            args: self.args.take(),
            fragment_value: self.fragment_value.take(),
            fragment_parts: std::mem::take(&mut self.fragment_parts),
            id_map: std::mem::take(&mut self.id_map),
            fresh_ids: std::mem::take(&mut self.fresh_ids),
            mutations: std::mem::take(&mut self.mutations),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dPasteJob {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dImportStage {
    Envelope,
    Decode,
    CensusParts,
    ReserveCatalogParts,
    ReserveCatalogGrips,
    ReserveCatalogFasteners,
    ReserveCatalogRopes,
    ReserveCompatibility,
    ReservePartIndex,
    ReserveGripIndex,
    ReserveCompatibilityIndex,
    ReserveMutations,
    LoadCatalogParts,
    LoadCatalogGrips,
    LoadCatalogFasteners,
    LoadCatalogRopes,
    LoadCompatibility,
    IndexParts,
    IndexGrips,
    IndexCompatibility,
    Parts,
    PartReserve,
    PartVortices,
    PartPublish,
    Grips,
    Compatibility,
    CatalogMutation,
    Complete,
}

/// ♻️ The owners one `Puzzle5dImportJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dImportOwners {
    pub(super) raw: Vec<u8>,
    pub(super) port: String,
    media_json: Option<String>,
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) fragment: Option<Value>,
    pub(super) catalogs: crate::Puzzle5dKindCatalogs,
    pub(super) compatibility: Vec<crate::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    current_part: Option<crate::Puzzle5dCatalogPartKind>,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
}

pub(super) struct Puzzle5dImportJob {
    pub(super) raw: Vec<u8>,
    raw_cursor: usize,
    raw_page: [u8; PUZZLE5D_RESERVED_PAGE_BYTES],
    raw_page_len: usize,
    pub(super) progress: u64,
    pub(super) stage: Puzzle5dImportStage,
    pub(super) port: String,
    media_json: Option<String>,
    pub(super) snapshot: Option<std::sync::Arc<Puzzle5dPlaySnapshot>>,
    pub(super) fragment: Option<Value>,
    pub(super) catalogs: crate::Puzzle5dKindCatalogs,
    had_catalogs: bool,
    catalog_changed: bool,
    pub(super) compatibility: Vec<crate::Puzzle5dKindCompatibility>,
    part_index: Vec<(String, usize)>,
    grip_index: Vec<(String, usize)>,
    compatibility_index: Vec<((String, String), usize)>,
    mutation_pages: [Vec<Puzzle5dMutation>; PUZZLE5D_IMPORT_MUTATION_PAGES],
    pub(super) cursor: usize,
    nested_cursor: usize,
    decoded_items: usize,
    current_part: Option<crate::Puzzle5dCatalogPartKind>,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<Puzzle5dPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>>,
    completed: bool,
    outbox: crate::puzzle_job::JobOutbox,
    pub(super) closing: bool,
    pub(super) owners: crate::puzzle_job::WorkClosing<Puzzle5dImportOwners>,
}

fn puzzle5d_import_vec3(value: Option<&Value>) -> Result<[f64; 3], &'static str> {
    let values = value.and_then(Value::as_array).ok_or("puzzle5d kit:in vector is not an array")?;
    if values.len() != 3 {
        return Err("puzzle5d kit:in vector must contain exactly three coordinates");
    }
    let mut result = [0.0; 3];
    for (index, target) in result.iter_mut().enumerate() {
        *target = values.get(index).and_then(Value::as_f64).filter(|value| value.is_finite()).ok_or("puzzle5d kit:in vector contains a non-finite coordinate")?;
    }
    Ok(result)
}

fn puzzle5d_import_keys_are(value: &Value, allowed: &[&str]) -> bool {
    value.as_object().is_some_and(|object| object.iter().all(|(key, _)| allowed.contains(&key)))
}

fn puzzle5d_decode_import_fragment(media_json: &str) -> Result<Value, String> {
    if media_json.len() > PUZZLE5D_IMPORT_MEDIA_BYTES {
        return Err("puzzle5d kit:in payload exceeds its predecode cap".into());
    }
    parse(media_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

impl Puzzle5dImportJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle5dPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            semio_framework_plugin::MediaPayload::Structured { json, .. } => Some(json),
            semio_framework_plugin::MediaPayload::Intrinsic { value, .. } => Some(semio_framework_pack_json::to_json_string(&value)),
            semio_framework_plugin::MediaPayload::Binary { .. } => None,
        };
        Self {
            raw: request.raw_wire,
            raw_cursor: 0,
            raw_page: [0; PUZZLE5D_RESERVED_PAGE_BYTES],
            raw_page_len: 0,
            progress: 0,
            stage: Puzzle5dImportStage::Envelope,
            port,
            media_json,
            snapshot: Some(request.snapshot),
            fragment: None,
            catalogs: Default::default(),
            had_catalogs: false,
            catalog_changed: false,
            compatibility: Vec::new(),
            part_index: Vec::new(),
            grip_index: Vec::new(),
            compatibility_index: Vec::new(),
            mutation_pages: std::array::from_fn(|_| Vec::new()),
            cursor: 0,
            nested_cursor: 0,
            decoded_items: 0,
            current_part: None,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            completed: false,
            outbox: Default::default(),
            closing: false,
            owners: Default::default(),
        }
    }

    pub(super) fn checkpoint(&self) -> crate::puzzle_job::JobTurn {
        puzzle5d_import_checkpoint(self.stage as u8, self.cursor, self.nested_cursor, self.decoded_items, self.progress)
    }

    pub(super) fn rows(&self, key: &str) -> &[Value] {
        self.fragment.as_ref().and_then(|value| value.get(key)).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
    }

    fn snapshot_rows(&self, parent: &str, key: &str) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get(parent).and_then(|value| value.get(key)).and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn snapshot_kind_compatibility_rows(&self) -> Vec<Value> {
        self.snapshot.as_ref().map(|snapshot| puzzle5d_editor_projection(snapshot)).and_then(|projection| projection.get("kindCompatibility").and_then(Value::as_array).cloned()).unwrap_or_default()
    }

    fn push_mutation(&mut self, mutation: Puzzle5dMutation) -> Result<(), &'static str> {
        let mutation_count = self.mutation_pages.iter().map(Vec::len).sum::<usize>();
        if mutation_count >= PUZZLE5D_IMPORT_MUTATION_ITEMS {
            return Err("puzzle5d kit:in mutation limit exceeded");
        }
        let page_index = mutation_count / PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE;
        let Some(page) = self.mutation_pages.get_mut(page_index) else {
            return Err("puzzle5d kit:in mutation page limit exceeded");
        };
        if page.len() == page.capacity() {
            return Err("puzzle5d kit:in mutation page reserve exhausted");
        }
        page.push(mutation);
        Ok(())
    }
}

impl Puzzle5dImportJob {
    pub(super) fn turn(&mut self, cx: &mut StepContext<'_>) -> crate::puzzle_job::JobTurn {
        if self.closing || cx.is_cancelled() {
            return crate::puzzle_job::JobTurn::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return puzzle5d_job_fault("puzzle5d import completion remains rejected");
        }
        match self.stage {
            Puzzle5dImportStage::Envelope => {
                if let Some(outcome) = puzzle5d_step_envelope(&self.raw, &mut self.raw_cursor, &mut self.raw_page, &mut self.raw_page_len, &mut self.progress, cx) {
                    return outcome;
                }
                self.stage = Puzzle5dImportStage::Decode;
            }
            Puzzle5dImportStage::Decode => {
                if self.port != "kit:in" {
                    return puzzle5d_job_fault("puzzle5d import only implements kit:in");
                }
                let Some(media_json) = self.media_json.as_ref() else {
                    return puzzle5d_job_fault("puzzle5d kit:in requires a structured payload");
                };
                let fragment = match puzzle5d_decode_import_fragment(media_json) {
                    Ok(fragment) => fragment,
                    Err(error) => return puzzle5d_job_fault(error),
                };
                let fragment_items = ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().try_fold(0usize, |total, key| total.checked_add(fragment.get(key).and_then(Value::as_array).map_or(0, Vec::len)));
                let snapshot_items = ["parts", "grips", "fasteners", "ropes"]
                    .into_iter()
                    .try_fold(0usize, |total, key| total.checked_add(self.snapshot_rows("kindCatalogs", key).len()))
                    .and_then(|total| total.checked_add(self.snapshot_kind_compatibility_rows().len()));
                self.decoded_items = match fragment_items.and_then(|fragment_items| snapshot_items.and_then(|snapshot_items| fragment_items.checked_add(snapshot_items))) {
                    Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                    _ => return puzzle5d_job_fault("puzzle5d kit:in decoded item limit exceeded"),
                };
                if fragment.as_object().is_none() {
                    return puzzle5d_job_fault("puzzle5d kit:in root must be an object");
                }
                if !puzzle5d_import_keys_are(&fragment, &["schema", "objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"]) {
                    return puzzle5d_job_fault("puzzle5d kit:in root contains an unknown field");
                }
                if fragment.get("schema").is_some_and(|value| value.as_str() != Some("manifest")) {
                    return puzzle5d_job_fault("puzzle5d kit:in schema must be manifest when present");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).is_some_and(|value| value.as_array().is_none())) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection must be an array");
                }
                if ["objectKinds", "vortexKinds", "cableKinds", "attractionKinds", "kindCompatibility"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| rows.len() > PUZZLE5D_IMPORT_SEMANTIC_ITEMS)) {
                    return puzzle5d_job_fault("puzzle5d kit:in collection exceeds its fixed-page descriptor cap");
                }
                if ["cableKinds", "attractionKinds"].into_iter().any(|key| fragment.get(key).and_then(Value::as_array).is_some_and(|rows| !rows.is_empty())) {
                    return puzzle5d_job_fault("puzzle5d kit:in cannot silently discard unmapped cable or attraction kinds");
                }
                self.had_catalogs = self.snapshot.as_ref().is_some_and(|snapshot| snapshot.typed().kind_catalogs.is_some());
                self.catalog_changed = !self.had_catalogs;
                self.fragment = Some(fragment);
                self.stage = Puzzle5dImportStage::CensusParts;
                self.cursor = 0;
            }
            Puzzle5dImportStage::CensusParts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "meshUrl", "vortices"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in object kind contains an unknown field");
                    }
                    if row.get("vortices").is_some_and(|value| value.as_array().is_none()) {
                        return puzzle5d_job_fault("puzzle5d kit:in object-kind vortices must be an array");
                    }
                    let vortices = row.get("vortices").and_then(Value::as_array).map_or(0, Vec::len);
                    if vortices > PUZZLE5D_IMPORT_SEMANTIC_ITEMS {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex collection exceeds its fixed-page descriptor cap");
                    }
                    self.decoded_items = match self.decoded_items.checked_add(vortices) {
                        Some(items) if items <= PUZZLE5D_IMPORT_DECODED_ITEMS => items,
                        _ => return puzzle5d_job_fault("puzzle5d kit:in nested vortex item limit exceeded"),
                    };
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::ReserveCatalogParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::ReserveCatalogParts => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.parts.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in part catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogGrips;
            }
            Puzzle5dImportStage::ReserveCatalogGrips => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.grips.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogFasteners;
            }
            Puzzle5dImportStage::ReserveCatalogFasteners => {
                let capacity = self.snapshot_rows("kindCatalogs", "fasteners").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.fasteners.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in fastener catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCatalogRopes;
            }
            Puzzle5dImportStage::ReserveCatalogRopes => {
                let capacity = self.snapshot_rows("kindCatalogs", "ropes").len();
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.catalogs.ropes.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in rope catalog reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibility;
            }
            Puzzle5dImportStage::ReserveCompatibility => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReservePartIndex;
            }
            Puzzle5dImportStage::ReservePartIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "parts").len().saturating_add(self.rows("objectKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.part_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in part index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveGripIndex;
            }
            Puzzle5dImportStage::ReserveGripIndex => {
                let capacity = self.snapshot_rows("kindCatalogs", "grips").len().saturating_add(self.rows("vortexKinds").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.grip_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in grip index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveCompatibilityIndex;
            }
            Puzzle5dImportStage::ReserveCompatibilityIndex => {
                let capacity = self.snapshot_kind_compatibility_rows().len().saturating_add(self.rows("kindCompatibility").len());
                if capacity > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || self.compatibility_index.try_reserve_exact(capacity).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in compatibility index reserve rejected");
                }
                self.stage = Puzzle5dImportStage::ReserveMutations;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::ReserveMutations => {
                let capacity = self.rows("kindCompatibility").len().saturating_mul(2).saturating_add(1);
                if capacity > PUZZLE5D_IMPORT_MUTATION_ITEMS {
                    return puzzle5d_job_fault("puzzle5d kit:in mutation reserve rejected");
                }
                let page_start = self.nested_cursor.saturating_mul(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                if page_start < capacity {
                    let page_items = capacity.saturating_sub(page_start).min(PUZZLE5D_IMPORT_MUTATIONS_PER_PAGE);
                    let Some(page) = self.mutation_pages.get_mut(self.nested_cursor) else {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    };
                    if page.try_reserve_exact(page_items).is_err() {
                        return puzzle5d_job_fault("puzzle5d kit:in mutation page reserve rejected");
                    }
                    self.nested_cursor += 1;
                    return self.checkpoint();
                }
                self.stage = Puzzle5dImportStage::LoadCatalogParts;
                self.cursor = 0;
                self.nested_cursor = 0;
            }
            Puzzle5dImportStage::LoadCatalogParts => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "parts").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogPartKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.parts.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogGrips => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "grips").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogGripKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.grips.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogFasteners;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogFasteners => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "fasteners").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogFastenerKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.fasteners.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCatalogRopes;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCatalogRopes => {
                if let Some(row) = self.snapshot_rows("kindCatalogs", "ropes").get(self.cursor) {
                    let parsed = match <crate::Puzzle5dCatalogRopeKind as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.catalogs.ropes.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::LoadCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::LoadCompatibility => {
                let rows = self.snapshot_kind_compatibility_rows();
                if let Some(row) = rows.get(self.cursor) {
                    let parsed = match <crate::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    self.compatibility.push(parsed);
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexParts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexParts => {
                if let Some(row) = self.catalogs.parts.get(self.cursor) {
                    self.part_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexGrips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexGrips => {
                if let Some(row) = self.catalogs.grips.get(self.cursor) {
                    self.grip_index.push((row.id.clone(), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::IndexCompatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::IndexCompatibility => {
                if let Some(row) = self.compatibility.get(self.cursor) {
                    self.compatibility_index.push(((row.source.clone(), row.target.clone()), self.cursor));
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Parts;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Parts => {
                if let Some(row) = self.rows("objectKinds").get(self.cursor) {
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks id"),
                    };
                    let name = match row.get("name").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks name"),
                    };
                    let label = match row.get("label").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in object kind lacks label"),
                    };
                    let mesh_url = row.get("meshUrl").and_then(Value::as_str).map(str::to_string);
                    self.current_part = Some(crate::Puzzle5dCatalogPartKind {
                        id,
                        name,
                        label,
                        representations: mesh_url.map(|url| vec![crate::Puzzle5dRepresentation { id: "mesh".into(), name: "mesh".into(), url, mime: "model/gltf-binary".into(), ..Default::default() }]).unwrap_or_default(),
                        grips: Vec::new(),
                        ..Default::default()
                    });
                    self.nested_cursor = 0;
                    self.stage = Puzzle5dImportStage::PartReserve;
                } else {
                    self.stage = Puzzle5dImportStage::Grips;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::PartReserve => {
                let grip_count = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).map_or(0, Vec::len);
                let Some(part) = self.current_part.as_mut() else {
                    return puzzle5d_job_fault("puzzle5d kit:in lost its part before nested reserve");
                };
                if grip_count > PUZZLE5D_IMPORT_SEMANTIC_ITEMS || part.grips.try_reserve_exact(grip_count).is_err() {
                    return puzzle5d_job_fault("puzzle5d kit:in nested grip reserve rejected");
                }
                self.stage = Puzzle5dImportStage::PartVortices;
            }
            Puzzle5dImportStage::PartVortices => {
                let vortex = self.rows("objectKinds").get(self.cursor).and_then(|row| row.get("vortices")).and_then(Value::as_array).and_then(|rows| rows.get(self.nested_cursor));
                if let Some(vortex) = vortex {
                    if !puzzle5d_import_keys_are(vortex, &["id", "vortexKind", "position", "direction", "radius"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex contains an unknown field");
                    }
                    let vortex_kind = match vortex.get("vortexKind").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks kind"),
                    };
                    let point = match puzzle5d_import_vec3(vortex.get("position")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let direction = match puzzle5d_import_vec3(vortex.get("direction")) {
                        Ok(value) => value,
                        Err(error) => return puzzle5d_job_fault(error),
                    };
                    let radius = match vortex.get("radius").and_then(Value::as_f64).filter(|value| value.is_finite()) {
                        Some(value) => value,
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex lacks finite radius"),
                    };
                    let Some(part) = self.current_part.as_mut() else {
                        return puzzle5d_job_fault("puzzle5d kit:in lost its current part owner");
                    };
                    part.grips.push(crate::Puzzle5dGripTemplate {
                        id: format!("g{}", self.nested_cursor),
                        name: vortex_kind.clone(),
                        label: vortex_kind.clone(),
                        grip_kind: Some(vortex_kind),
                        point,
                        direction,
                        radius: Some(radius),
                        ..Default::default()
                    });
                    self.nested_cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::PartPublish;
                }
            }
            Puzzle5dImportStage::PartPublish => {
                let Some(next) = self.current_part.take() else {
                    return puzzle5d_job_fault("puzzle5d kit:in lost its completed part owner");
                };
                let id = next.id.clone();
                match self.part_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                    Some(index) => self.catalogs.parts[index] = next,
                    None => {
                        self.part_index.push((id, self.catalogs.parts.len()));
                        self.catalogs.parts.push(next);
                    }
                }
                self.catalog_changed = true;
                self.cursor += 1;
                self.stage = Puzzle5dImportStage::Parts;
            }
            Puzzle5dImportStage::Grips => {
                if let Some(row) = self.rows("vortexKinds").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["id", "name", "label", "color", "defaultCableKind"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in vortex kind contains an unknown field");
                    }
                    let id = match row.get("id").and_then(Value::as_str) {
                        Some(value) => value.to_string(),
                        None => return puzzle5d_job_fault("puzzle5d kit:in vortex kind lacks id"),
                    };
                    let next = crate::Puzzle5dCatalogGripKind {
                        id: id.clone(),
                        code: row.get("name").and_then(Value::as_str).map(str::to_string),
                        label: row.get("label").and_then(Value::as_str).map(str::to_string),
                        color: row.get("color").and_then(Value::as_str).unwrap_or_default().to_string(),
                        default_rope_kind: row.get("defaultCableKind").and_then(Value::as_str).unwrap_or_default().to_string(),
                        ..Default::default()
                    };
                    match self.grip_index.iter().find_map(|(candidate, index)| (candidate == &id).then_some(*index)) {
                        Some(index) => self.catalogs.grips[index] = next,
                        None => {
                            self.grip_index.push((id, self.catalogs.grips.len()));
                            self.catalogs.grips.push(next);
                        }
                    }
                    self.catalog_changed = true;
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::Compatibility;
                    self.cursor = 0;
                }
            }
            Puzzle5dImportStage::Compatibility => {
                if let Some(row) = self.rows("kindCompatibility").get(self.cursor) {
                    if !puzzle5d_import_keys_are(row, &["source", "target", "bidirectional", "important", "specificity"]) {
                        return puzzle5d_job_fault("puzzle5d kit:in compatibility contains an unknown field");
                    }
                    let parsed = match <crate::Puzzle5dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(row)) {
                        Ok(parsed) => parsed,
                        Err(error) => return puzzle5d_job_fault(error.to_string()),
                    };
                    let key = (parsed.source.clone(), parsed.target.clone());
                    match self.compatibility_index.iter().find_map(|(candidate, index)| (candidate == &key).then_some(*index)) {
                        Some(index) if self.compatibility[index] == parsed => {}
                        Some(index) => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility(parsed.source.clone(), parsed.target.clone())) {
                                return puzzle5d_job_fault(error);
                            }
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
                            }
                            self.compatibility[index] = parsed;
                        }
                        None => {
                            if let Err(error) = self.push_mutation(crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility(parsed.source.clone(), parsed.target.clone(), parsed.bidirectional, parsed.important, parsed.specificity, None)) {
                                return puzzle5d_job_fault(error);
                            }
                            self.compatibility_index.push((key, self.compatibility.len()));
                            self.compatibility.push(parsed);
                        }
                    }
                    self.cursor += 1;
                } else {
                    self.stage = Puzzle5dImportStage::CatalogMutation;
                }
            }
            Puzzle5dImportStage::CatalogMutation => {
                if self.catalog_changed {
                    let mutation = crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs(Some(std::mem::take(&mut self.catalogs)));
                    if let Err(error) = self.push_mutation(mutation) {
                        return puzzle5d_job_fault(error);
                    }
                    self.catalog_changed = false;
                }
                self.stage = Puzzle5dImportStage::Complete;
            }
            Puzzle5dImportStage::Complete => {
                if !self.raw.is_empty() {
                    return crate::puzzle_job::JobTurn::Prepare(std::mem::take(&mut self.raw));
                }
                if !self.completed {
                    let mut mutation_pages = std::mem::take(&mut self.mutation_pages);
                    let mut mutations = std::mem::take(&mut mutation_pages[0]);
                    for page in mutation_pages.iter_mut().skip(1) {
                        mutations.append(page);
                    }
                    let Some(completion) = self.completion.as_ref() else { return puzzle5d_job_fault("puzzle5d import lost its completion authority") };
                    if let Err(rejected) = completion.complete(Ok(Emit::mutations(mutations)), EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        return puzzle5d_job_fault(message);
                    }
                    self.completed = true;
                }
                return crate::puzzle_job::JobTurn::Complete;
            }
        }
        self.progress = self.progress.saturating_add(1);
        cx.consume_fuel(1);
        self.checkpoint()
    }
}

impl InteractiveJob for Puzzle5dImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = if cx.fuel_exhausted() || cx.deadline_exceeded() { crate::puzzle_job::JobTurn::Yield } else { self.turn(cx) };
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle5dImportOwners {
            raw: std::mem::take(&mut self.raw),
            port: std::mem::take(&mut self.port),
            media_json: self.media_json.take(),
            snapshot: self.snapshot.take(),
            fragment: self.fragment.take(),
            catalogs: std::mem::take(&mut self.catalogs),
            compatibility: std::mem::take(&mut self.compatibility),
            part_index: std::mem::take(&mut self.part_index),
            grip_index: std::mem::take(&mut self.grip_index),
            compatibility_index: std::mem::take(&mut self.compatibility_index),
            mutation_pages: std::mem::take(&mut self.mutation_pages),
            current_part: self.current_part.take(),
            completion: self.completion.take(),
            pending_completion_rejection: self.pending_completion_rejection.take(),
        });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.begin_close();
        crate::puzzle_job::job_close_step(&mut self.outbox, &mut self.owners, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(crate::puzzle_job::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl ArtifactReservedJob for Puzzle5dImportJob {}
//#endregion 🧵️ReservedJobs
