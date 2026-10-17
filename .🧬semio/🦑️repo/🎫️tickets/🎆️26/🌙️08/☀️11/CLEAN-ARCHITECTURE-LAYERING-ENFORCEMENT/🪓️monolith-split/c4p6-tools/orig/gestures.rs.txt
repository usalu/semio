//! 👆️ Gestures of the puzzle 5d editor: the retained transform, kind-weight, focus, patch, fastener, grip and proximity work. See [`super`].

use super::*;

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dTransformStage {
    Read,
    Scan,
    Commit,
    Complete,
    Closing,
}

/// 🎞️ What `Read` states for one transform: the finished record, or the world drop whose proximity scan `Scan` pages.
enum Puzzle5dTransformGesture {
    Stated(world3d::utilities::transform::Puzzle5dSelectionRecord),
    Scanning(world3d::utilities::transform::Puzzle5dRelocateScan),
}

/// 🛠️ The ONE retained work of every selection transform — a gumball translate/rotate/scale, a target-volume
/// gumball relocate, a world drop, an inspector `x`/`y`/origin nudge. `Read` states the gesture as ONE
/// [`world3d::utilities::transform::Puzzle5dSelectionRecord`] (the gesture's own ids, else the live selection);
/// a world drop's proximity scan then runs page by page in `Scan` (one progress report per page, cancellable between
/// pages), and `Commit` runs the record through the transform tool: one `ToolTransaction` whose parametric leaves
/// publish as ONE edit stamped with the ref minted from the admission's `authoring_seed`. A gesture that moves nothing leaves
/// zero trace; nothing addressed, or everything addressed locked, is one localized refusal.
/// ♻️ The owners one `Puzzle5dTransformWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dTransformWorkOwners {
    pub(super) record: Option<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    pub(super) scan: Option<world3d::utilities::transform::Puzzle5dRelocateScan>,
}

pub(super) struct Puzzle5dTransformWork {
    pub(super) tool_id: &'static str,
    pub(super) authoring_seed: String,
    pub(super) stage: Puzzle5dTransformStage,
    pub(super) record: Option<world3d::utilities::transform::Puzzle5dSelectionRecord>,
    pub(super) scan: Option<world3d::utilities::transform::Puzzle5dRelocateScan>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dTransformWorkOwners>,
}

impl Puzzle5dTransformWork {
    pub(super) fn new(tool_id: &'static str, authoring_seed: String) -> Self {
        Self { close_owners: Default::default(), tool_id, authoring_seed, stage: Puzzle5dTransformStage::Read, record: None, scan: None }
    }

    /// 🕹️ The parts a selection-scoped verb addresses: the command's own `ids`, else the live part selection.
    fn addressed(command: &Puzzle5dCommand, interaction: &protocol::InteractionState) -> Vec<String> {
        let explicit: Vec<String> = command.args().and_then(|args| args.get("ids")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect();
        if !explicit.is_empty() {
            return explicit;
        }
        Self::selected(interaction, PUZZLE5D_GRANULARITY_PART)
    }

    pub(super) fn selected(interaction: &protocol::InteractionState, granularity: &str) -> Vec<String> {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == granularity).map_or_else(Vec::new, |selection| selection.ids.clone())
    }

    /// 🩹️ The parts an inspector edit names: `partIds` then `partId`, each once, empty ids dropped.
    fn patched(command: &Puzzle5dCommand) -> Vec<String> {
        let args = command.args();
        let listed = args.and_then(|args| args.get("partIds")).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str);
        let single = args.and_then(|args| args.get("partId")).and_then(Value::as_str);
        listed.chain(single).filter(|id| !id.is_empty()).map(str::to_string).collect()
    }

    /// 🎬️ The gesture this command states on `document`, `None` when it states none.
    pub(super) fn read(&self, command: &Puzzle5dCommand, document: &Puzzle5dSnapshot, config: &Puzzle5dConfig, interaction: &protocol::InteractionState) -> Option<Puzzle5dTransformGesture> {
        use semio_s_artifact_puzzle_3d_editor_transform::Puzzle3dSelectionRecord;
        use world3d::utilities::transform::{Puzzle5dRelocateScan, Puzzle5dSelectionRecord};
        let args = command.args();
        match self.tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" => {
                let targets = [Self::addressed(command, interaction), Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME)].concat();
                Puzzle3dSelectionRecord::from_gumball(self.tool_id, args, targets).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record)))
            }
            "relocateTargetVolume" => Puzzle3dSelectionRecord::from_pose_delta(args).map(|record| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::world(record))),
            "worldRelocate" => {
                let part_id = args.and_then(|args| args.get("objectId")).and_then(Value::as_str).unwrap_or("");
                let position = args.and_then(|args| args.get("position")).and_then(puzzle5d_value_as_f64_3)?;
                Puzzle5dRelocateScan::begin(document, part_id, position, config.proximity_radius).map(Puzzle5dTransformGesture::Scanning)
            }
            _ => puzzle5d_inspector_nudge(args).map(|motion| Puzzle5dTransformGesture::Stated(Puzzle5dSelectionRecord::new(Self::patched(command), motion))),
        }
    }

    /// 🏁️ The ONE terminal emit: the committed transaction, a refusal, or nothing at all.
    fn commit(&mut self, snapshot: &Puzzle5dPlaySnapshot, view_state: Option<&semio_framework_plugin::ViewModel>) -> Emit<Puzzle5dMutation, Puzzle5dConfigMutation> {
        self.stage = Puzzle5dTransformStage::Complete;
        let Some(record) = self.record.take() else { return Emit { ui_scope: UiDirtyScope::None, ..Default::default() } };
        let base = snapshot.typed_arc();
        if !record.names_any(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str());
        }
        if record.refused_as_locked(&base) {
            return puzzle5d_notice_emit(view_state, |labels| labels.selection_locked.as_str());
        }
        let request = world3d::utilities::transform::TransformToolRequest { base, records: vec![record] };
        match world3d::utilities::transform::puzzle5d_transform_tool_commit(self.tool_id, &self.authoring_seed, request) {
            Some((transaction, mutations)) => Emit { artifact_mutations: mutations, transaction: (!self.authoring_seed.is_empty()).then_some(transaction), ui_scope: UiDirtyScope::Full, ..Default::default() },
            None => Emit { ui_scope: UiDirtyScope::None, ..Default::default() },
        }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dTransformWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    /// 🔢️ `Read` + `Commit`, plus one `Scan` step per page of parts a world drop measures.
    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let addressed = Self::addressed(command, interaction).len().checked_add(Self::selected(interaction, PUZZLE5D_GRANULARITY_TARGET_VOLUME).len())?.checked_add(Self::patched(command).len())?;
        let pages = if self.tool_id == "worldRelocate" { snapshot.typed().parts.len().div_ceil(world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE).max(1) } else { 0 };
        (addressed <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS).then_some(2 + pages)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: _hover, context, .. } = *input;
        match self.stage {
            Puzzle5dTransformStage::Read => {
                match self.read(command, snapshot.typed(), config, interaction) {
                    Some(Puzzle5dTransformGesture::Stated(record)) => self.record = Some(record),
                    Some(Puzzle5dTransformGesture::Scanning(scan)) => self.scan = Some(scan),
                    None => {}
                }
                self.stage = if self.scan.is_some() { Puzzle5dTransformStage::Scan } else { Puzzle5dTransformStage::Commit };
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-read", "Reading the gesture", "Geste wird gelesen"))
            }
            Puzzle5dTransformStage::Scan => {
                let scan = self.scan.as_mut().ok_or_else(|| Fault::from("puzzle5d-transform-scan-owner"))?;
                if scan.step(snapshot.typed(), world3d::utilities::transform::PUZZLE5D_RELOCATE_SCAN_PAGE) {
                    self.record = self.scan.take().map(world3d::utilities::transform::Puzzle5dRelocateScan::finish);
                    self.stage = Puzzle5dTransformStage::Commit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-transform-scan", "Measuring nearby grips", "Nahe Griffe werden gemessen"))
            }
            Puzzle5dTransformStage::Commit => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(self.commit(snapshot, context.and_then(|context| context.view_state.as_ref())))),
            Puzzle5dTransformStage::Complete => Err(Fault::from("puzzle5d-transform-complete-repolled")),
            Puzzle5dTransformStage::Closing => Err(Fault::from("puzzle5d-transform-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dTransformStage::Closing;
        self.close_owners.stage(Puzzle5dTransformWorkOwners { record: std::mem::take(&mut self.record), scan: std::mem::take(&mut self.scan) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dTransformStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dKindWeightStage {
    Catalog,
    InferParts,
    InferGrips,
    Validate,
    SumOthers,
    Changed,
    Build,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dKindWeightWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dKindWeightWorkOwners {
    pub(super) ids: Vec<String>,
    pub(super) seen: HashSet<String>,
    pub(super) result: HashMap<String, f64>,
    pub(super) changed_id: Option<String>,
}

pub(super) struct Puzzle5dKindWeightWork {
    pub(super) tool_id: &'static str,
    pub(super) stage: Puzzle5dKindWeightStage,
    pub(super) cursor: usize,
    pub(super) part_cursor: usize,
    pub(super) grip_cursor: usize,
    pub(super) ids: Vec<String>,
    pub(super) seen: HashSet<String>,
    pub(super) result: HashMap<String, f64>,
    missing: bool,
    base_sum: f64,
    pub(super) other_sum: f64,
    other_count: usize,
    pub(super) changed_id: Option<String>,
    pub(super) requested: f64,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dKindWeightWorkOwners>,
}

impl Puzzle5dKindWeightWork {
    pub(super) fn new(tool_id: &'static str) -> Self {
        Self { close_owners: Default::default(),
            tool_id,
            stage: Puzzle5dKindWeightStage::Catalog,
            cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            ids: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            seen: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            result: HashMap::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            missing: false,
            base_sum: 0.0,
            other_sum: 0.0,
            other_count: 0,
            changed_id: None,
            requested: 1.0,
        }
    }

    fn section(&self) -> &'static str {
        if self.tool_id == "setPartKindWeight" {
            "parts"
        } else {
            "grips"
        }
    }

    pub(super) fn weights<'a>(&self, config: &'a Puzzle5dConfig) -> &'a HashMap<String, f64> {
        if self.tool_id == "setPartKindWeight" {
            &config.object_kind_weights
        } else {
            &config.vortex_kind_weights
        }
    }

    fn catalog(&self, snapshot: &Puzzle5dPlaySnapshot) -> Vec<Value> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("kindCatalogs").and_then(|value| value.get(self.section())).and_then(Value::as_array).cloned().unwrap_or_default()
    }

    fn push_id(&mut self, id: &str, inferred: bool) -> Result<(), Fault> {
        if self.ids.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
            return Err(Fault::from("puzzle5d-kind-weight-catalog-capacity"));
        }
        if !inferred || self.seen.insert(id.to_string()) {
            self.ids.push(id.to_string());
        }
        Ok(())
    }

    fn base_weight(&self, config: &Puzzle5dConfig, id: &str) -> f64 {
        if self.ids.is_empty() {
            return 0.0;
        }
        if self.missing || self.weights(config).is_empty() {
            return 1.0 / self.ids.len() as f64;
        }
        let value = self.weights(config).get(id).copied().unwrap_or(0.0);
        if (self.base_sum - 1.0).abs() > 0.001 && self.base_sum.abs() > f64::EPSILON {
            value / self.base_sum
        } else {
            value
        }
    }

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dKindWeightWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &Puzzle5dCommand, _snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        Some(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dKindWeightStage::Catalog => {
                let entries = self.catalog(snapshot);
                if let Some(entry) = entries.get(self.cursor) {
                    if let Some(id) = entry.get("id").and_then(Value::as_str) {
                        self.push_id(id, false)?;
                    }
                    self.cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-catalog", "Reading kind owner", "Artinhaber wird gelesen"));
                }
                self.stage = if entries.is_empty() {
                    if self.tool_id == "setPartKindWeight" {
                        Puzzle5dKindWeightStage::InferParts
                    } else {
                        Puzzle5dKindWeightStage::InferGrips
                    }
                } else {
                    Puzzle5dKindWeightStage::Validate
                };
                self.cursor = 0;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-infer", "Preparing kind validation", "Artprüfung wird vorbereitet"))
            }
            Puzzle5dKindWeightStage::InferParts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                self.part_cursor += 1;
                if let Some(id) = part.get("partKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Reading inferred part kind", "Abgeleitete Teileart wird gelesen"))
            }
            Puzzle5dKindWeightStage::InferGrips => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dKindWeightStage::Validate;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating current weights", "Aktuelle Gewichte werden geprüft"));
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                self.grip_cursor += 1;
                if let Some(id) = grip.get("gripKind").and_then(Value::as_str) {
                    self.push_id(id, true)?;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-grip", "Reading inferred grip kind", "Abgeleitete Griffart wird gelesen"))
            }
            Puzzle5dKindWeightStage::Validate => {
                if self.changed_id.is_none() {
                    self.changed_id = Some(command.args().and_then(|args| args.get("kindId")).and_then(Value::as_str).unwrap_or("").to_string());
                    self.requested = command.args().and_then(|args| args.get("value")).and_then(Value::as_f64).unwrap_or(1.0).clamp(0.0, 1.0);
                }
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::SumOthers;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weights", "Geschwistergewichte werden gemessen"));
                };
                let weights = self.weights(config);
                self.missing |= !weights.contains_key(id);
                self.base_sum += weights.get(id).copied().unwrap_or(0.0);
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-validate", "Validating kind weight", "Artgewicht wird geprüft"))
            }
            Puzzle5dKindWeightStage::SumOthers => {
                let Some(id) = self.ids.get(self.cursor) else {
                    self.cursor = 0;
                    self.stage = Puzzle5dKindWeightStage::Changed;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-changed", "Preparing changed weight", "Geändertes Gewicht wird vorbereitet"));
                };
                if self.changed_id.as_deref() != Some(id.as_str()) {
                    self.other_sum += self.base_weight(config, id);
                    self.other_count += 1;
                }
                self.cursor += 1;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-sum", "Measuring sibling weight", "Geschwistergewicht wird gemessen"))
            }
            Puzzle5dKindWeightStage::Changed => {
                if self.ids.len() >= 2 {
                    self.result.insert(self.changed_id.clone().ok_or_else(|| Fault::from("puzzle5d-kind-weight-changed-owner"))?, self.requested);
                }
                self.stage = Puzzle5dKindWeightStage::Build;
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building normalized weights", "Normalisierte Gewichte werden aufgebaut"))
            }
            Puzzle5dKindWeightStage::Build => {
                let Some(id) = self.ids.get(self.cursor).cloned() else {
                    self.stage = Puzzle5dKindWeightStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-publish", "Preparing weight publication", "Gewichtsveröffentlichung wird vorbereitet"));
                };
                self.cursor += 1;
                let value = if self.ids.len() == 1 {
                    1.0
                } else if self.changed_id.as_deref() == Some(id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Keeping changed weight", "Geändertes Gewicht wird beibehalten"));
                } else {
                    let remainder = (1.0 - self.requested).max(0.0);
                    if self.other_sum <= f64::EPSILON {
                        remainder / self.other_count.max(1) as f64
                    } else {
                        self.base_weight(config, &id) / self.other_sum * remainder
                    }
                };
                self.result.insert(id, value);
                Ok(crate::puzzle_progress_step!("puzzle5d-kind-weight-build", "Building kind weight", "Artgewicht wird aufgebaut"))
            }
            Puzzle5dKindWeightStage::Publish => {
                self.stage = Puzzle5dKindWeightStage::Complete;
                let mutation = if self.tool_id == "setPartKindWeight" {
                    Puzzle5dConfigMutation::SetObjectKindWeights(Puzzle5dConfigSetObjectKindWeights{ value: std::mem::take(&mut self.result) })
                } else {
                    Puzzle5dConfigMutation::SetVortexKindWeights(Puzzle5dConfigSetVortexKindWeights{ value: std::mem::take(&mut self.result) })
                };
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { config_mutations: vec![mutation], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dKindWeightStage::Complete => Err(Fault::from("puzzle5d-kind-weight-complete-repolled")),
            Puzzle5dKindWeightStage::Closing => Err(Fault::from("puzzle5d-kind-weight-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dKindWeightStage::Closing;
        self.close_owners.stage(Puzzle5dKindWeightWorkOwners { ids: std::mem::take(&mut self.ids), seen: std::mem::take(&mut self.seen), result: std::mem::take(&mut self.result), changed_id: std::mem::take(&mut self.changed_id) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dKindWeightStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dFocusSelectionStage {
    Selection,
    Parts,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dFocusSelectionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dFocusSelectionWorkOwners {
    pub(super) selected: HashSet<String>,
}

pub(super) struct Puzzle5dFocusSelectionWork {
    pub(super) stage: Puzzle5dFocusSelectionStage,
    selection_cursor: usize,
    pub(super) part_cursor: usize,
    pub(super) selected: HashSet<String>,
    sum_2d: [f64; 2],
    sum_3d: [f64; 3],
    matched: usize,
    minimum_3d: [f64; 3],
    maximum_3d: [f64; 3],
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dFocusSelectionWorkOwners>,
}

impl Default for Puzzle5dFocusSelectionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dFocusSelectionStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            sum_2d: [0.0; 2],
            sum_3d: [0.0; 3],
            matched: 0,
            minimum_3d: [f64::MAX; 3],
            maximum_3d: [f64::MIN; 3],
        }
    }
}

impl Puzzle5dFocusSelectionWork {
    pub(super) fn source(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE5D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE5D_GRANULARITY_PART).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which parts this focus frames. An EMPTY selection frames the WHOLE document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made the focus keybinding dead on every boot before the user
    /// had picked anything (puzzle 3d's `Puzzle3dFocusSelectionWork::frames`). Costs no extra capacity:
    /// the part scan this work already declares in `extent` runs either way.
    fn frames(&self, part_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(part_id)
    }

    pub(super) fn axis(row: &Value, section: &str, field: &str, index: usize) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_array).and_then(|values| values.get(index)).and_then(Value::as_f64).unwrap_or(0.0)
    }

    fn scalar(row: &Value, section: &str, field: &str) -> f64 {
        row.get(section).and_then(|section| section.get(field)).and_then(Value::as_f64).unwrap_or(0.0)
    }

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let selected = Self::source(interaction).len();
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let items = selected.checked_add(parts)?.checked_add(1)?;
        (selected <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dFocusSelectionStage::Selection => {
                if let Some(id) = Self::source(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-owner", "Reading selected part", "Ausgewähltes Teil wird gelesen"));
                }
                self.stage = Puzzle5dFocusSelectionStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Finding selected part", "Ausgewähltes Teil wird gesucht"))
            }
            Puzzle5dFocusSelectionStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dFocusSelectionStage::Publish;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.part_cursor += 1;
                if row.get("id").and_then(Value::as_str).is_some_and(|id| self.frames(id)) {
                    self.sum_2d[0] += Self::scalar(row, "2d", "x");
                    self.sum_2d[1] += Self::scalar(row, "2d", "y");
                    self.sum_3d[0] += Self::axis(row, "3d", "origin", 0);
                    self.sum_3d[1] += Self::axis(row, "3d", "origin", 1);
                    self.sum_3d[2] += Self::axis(row, "3d", "origin", 2);
                    for axis in 0..3 {
                        let value = Self::axis(row, "3d", "origin", axis);
                        self.minimum_3d[axis] = self.minimum_3d[axis].min(value);
                        self.maximum_3d[axis] = self.maximum_3d[axis].max(value);
                    }
                    self.matched += 1;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-focus-selection-part", "Scanning selected part", "Ausgewähltes Teil wird geprüft"))
            }
            Puzzle5dFocusSelectionStage::Publish => {
                self.stage = Puzzle5dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.nothing_selected.as_str())));
                }
                let divisor = self.matched as f64;
                let centre = [self.sum_3d[0] / divisor, self.sum_3d[1] / divisor, self.sum_3d[2] / divisor];
                // 📏️ Half the diagonal of the framed parts' own axis-aligned bound, accumulated in the ONE
                // part scan `extent` already declares — no second pass, no unbounded growth.
                let span = [self.maximum_3d[0] - self.minimum_3d[0], self.maximum_3d[1] - self.minimum_3d[1], self.maximum_3d[2] - self.minimum_3d[2]];
                let radius = (span[0] * span[0] + span[1] * span[1] + span[2] * span[2]).sqrt() / 2.0;
                let distance = radius * 3.0 + 2.0;
                // 🪟️ BOTH poses are written into the one window-config value; `addressed_config` projects
                // exactly the half the addressed pane owns (board → `camera2d`, world → `camera3d`), so a
                // focus in either pane keeps that pane's own camera coherent with the 5d dual pose.
                let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                let offset = [next.camera3d.position[0] - next.camera3d.target[0], next.camera3d.position[1] - next.camera3d.target[1], next.camera3d.position[2] - next.camera3d.target[2]];
                let orbit = (offset[0] * offset[0] + offset[1] * offset[1] + offset[2] * offset[2]).sqrt();
                let scale = if orbit > f64::EPSILON { distance / orbit } else { 1.0 };
                next.camera3d.target = centre;
                next.camera3d.position = [centre[0] + offset[0] * scale, centre[1] + offset[1] * scale, centre[2] + offset[2] * scale];
                next.camera2d.x = self.sum_2d[0] / divisor;
                next.camera2d.y = self.sum_2d[1] / divisor;
                let view = view_state.ok_or_else(|| Fault::from("puzzle5d-focus-window-context-required"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dFocusSelectionStage::Complete => Err(Fault::from("puzzle5d-focus-selection-complete-repolled")),
            Puzzle5dFocusSelectionStage::Closing => Err(Fault::from("puzzle5d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dFocusSelectionStage::Closing;
        self.close_owners.stage(Puzzle5dFocusSelectionWorkOwners { selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dFocusSelectionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dPatchPartStage {
    Selection,
    Parts,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dPatchPartWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchPartWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) selected: HashSet<String>,
}

pub(super) struct Puzzle5dPatchPartWork {
    pub(super) stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    pub(super) part_cursor: usize,
    pub(super) selected: HashSet<String>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchPartWorkOwners>,
}

impl Default for Puzzle5dPatchPartWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dPatchPartWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("partIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("partId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("partIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("partId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub(super) fn mutation(command: &Puzzle5dCommand, part: &Puzzle5dPart) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "partKind" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::change_part_kind(part.id.clone(), Some(text.to_string()))),
            "anchor" => text.map(|text| {
                let anchor = match text.to_ascii_lowercase().as_str() {
                    "derived" | "connected" => crate::Puzzle5dPartAnchor::Derived,
                    _ => crate::Puzzle5dPartAnchor::Fixed,
                };
                crate::standards::v1::subsets::any::schema::mutations::change_part_anchor(part.id.clone(), anchor)
            }),
            "text" => text.map(|text| crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text(part.id.clone(), Some(text.to_string()))),
            "label" => Some(crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "meshUrl" => Some(crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh(part.id.clone(), text.filter(|text| !text.is_empty()).map(str::to_string))),
            "x" => puzzle5d_resolve_number_edit(part.part_2d.x, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), updated, part.part_2d.y)),
            "y" => puzzle5d_resolve_number_edit(part.part_2d.y, value, delta).map(|updated| crate::standards::v1::subsets::any::schema::mutations::move_part_2d(part.id.clone(), part.part_2d.x, updated)),
            _ => {
                let axis = puzzle5d_axis_index(field, "origin")?;
                let updated = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta)?;
                let mut origin = part.part_3d.origin;
                origin[axis] = updated;
                Some(crate::standards::v1::subsets::any::schema::mutations::move_part_3d(part.id.clone(), origin))
            }
        }
    }

}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchPartWork {
    fn tool_id(&self) -> &'static str {
        "patchPart"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-part-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-part-selection", "Reading part target", "Teilziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    // 🧯️ An unaddressed id or a field this inspector cannot write produced no mutation at all;
                    // the pre-migration arm fell through silently and the panel looked dead.
                    if self.mutations.is_empty() {
                        return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(puzzle5d_notice_emit(view_state, |labels| labels.edit_not_applicable.as_str())));
                    }
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.part_cursor += 1;
                let part: Puzzle5dPart = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-part-malformed"))?;
                if self.selected.contains(&part.id) {
                    if let Some(mutation) = Self::mutation(command, &part) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-part", "Patching part", "Teil wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-part-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-part-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchPartWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchFastenerWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) selected: HashSet<String>,
}

pub(super) struct Puzzle5dPatchFastenerWork {
    pub(super) stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    pub(super) fastener_cursor: usize,
    pub(super) selected: HashSet<String>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchFastenerWorkOwners>,
}

impl Default for Puzzle5dPatchFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            fastener_cursor: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dPatchFastenerWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("fastenerIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("fastenerId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("fastenerIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("fastenerId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub(super) fn mutation(command: &Puzzle5dCommand, fastener: &Puzzle5dFastener) -> Option<Puzzle5dMutation> {
        let args = command.args()?;
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        if field == "fastenerKind" {
            return Some(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), value.and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string)));
        }
        let mut geometry = [fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y];
        let index = match field {
            "gap" => 0,
            "shift" => 1,
            "rise" => 2,
            "rotation" => 3,
            "turn" => 4,
            "tilt" => 5,
            "x" => 6,
            "y" => 7,
            _ => return None,
        };
        geometry[index] = puzzle5d_resolve_number_edit(geometry[index], value, delta)?;
        Some(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }))
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchFastenerWork {
    fn tool_id(&self) -> &'static str {
        "patchFastener"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-fastener-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener-selection", "Reading fastener target", "Verbindungsziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                self.fastener_cursor += 1;
                let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-patch-fastener-malformed"))?;
                if self.selected.contains(&fastener.id) {
                    if let Some(mutation) = Self::mutation(command, &fastener) {
                        self.mutations.push(mutation);
                    }
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-fastener", "Patching fastener", "Verbindung wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-fastener-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dEditFastenerStage {
    Scan,
    Kind,
    Geometry,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dEditFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dEditFastenerWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) fastener: Option<Puzzle5dFastener>,
}

pub(super) struct Puzzle5dEditFastenerWork {
    pub(super) stage: Puzzle5dEditFastenerStage,
    pub(super) cursor: usize,
    pub(super) fastener: Option<Puzzle5dFastener>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dEditFastenerWorkOwners>,
}

impl Default for Puzzle5dEditFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), stage: Puzzle5dEditFastenerStage::Scan, cursor: 0, fastener: None, mutations: Vec::with_capacity(2) }
    }
}

impl Puzzle5dEditFastenerWork {
    pub(super) fn id(command: &Puzzle5dCommand) -> &str {
        command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or("")
    }

    fn updated_kind(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<Option<String>> {
        let args = command.args()?;
        let mut update = args.get("fastenerKind").or_else(|| args.get("edgeKind")).and_then(Value::as_str).filter(|text| !text.is_empty()).map(|text| Some(text.to_string()));
        if matches!(args.get("field").and_then(Value::as_str), Some("fastenerKind" | "edgeKind")) {
            update = Some(args.get("value").and_then(Value::as_str).filter(|text| !text.is_empty()).map(str::to_string));
        }
        update.filter(|updated| updated != &current.fastener_kind)
    }

    fn updated_geometry(command: &Puzzle5dCommand, current: &Puzzle5dFastener) -> Option<[f64; 8]> {
        let args = command.args()?;
        let mut geometry = [current.gap, current.shift, current.rise, current.rotation, current.turn, current.tilt, current.x, current.y];
        let keys = ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"];
        let mut changed = false;
        for (index, key) in keys.iter().enumerate() {
            if let Some(value) = args.get(key) {
                if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], Some(value), None) {
                    changed |= updated != geometry[index];
                    geometry[index] = updated;
                }
            }
        }
        if let Some(index) = keys.iter().position(|key| args.get("field").and_then(Value::as_str) == Some(*key)) {
            if let Some(updated) = puzzle5d_resolve_number_edit(geometry[index], args.get("value"), args.get("delta")) {
                changed |= updated != geometry[index];
                geometry[index] = updated;
            }
        }
        changed.then_some(geometry)
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dEditFastenerWork {
    fn tool_id(&self) -> &'static str {
        "editFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len).checked_add(2).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dEditFastenerStage::Scan => {
                let target = Self::id(command);
                if target.is_empty() {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)).cloned() else {
                    self.stage = Puzzle5dEditFastenerStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                };
                self.cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(target) {
                    self.fastener = Some(puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-edit-fastener-malformed"))?);
                    self.stage = Puzzle5dEditFastenerStage::Kind;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-scan", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dEditFastenerStage::Kind => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(kind) = Self::updated_kind(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind(fastener.id.clone(), kind));
                }
                self.stage = Puzzle5dEditFastenerStage::Geometry;
                Ok(crate::puzzle_progress_step!("puzzle5d-edit-fastener-kind", "Updating fastener kind", "Verbindungsart wird aktualisiert"))
            }
            Puzzle5dEditFastenerStage::Geometry => {
                let Some(fastener) = self.fastener.as_ref() else { return Err(Fault::from("puzzle5d-edit-fastener-owner")) };
                if let Some(geometry) = Self::updated_geometry(command, fastener) {
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry(crate::standards::v1::subsets::any::schema::mutations::ReplaceFastenerGeometry { id: fastener.id.clone(), new_gap: geometry[0], new_shift: geometry[1], new_rise: geometry[2], new_rotation: geometry[3], new_turn: geometry[4], new_tilt: geometry[5], new_x: geometry[6], new_y: geometry[7] }));
                }
                self.stage = Puzzle5dEditFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dEditFastenerStage::Complete => Err(Fault::from("puzzle5d-edit-fastener-complete-repolled")),
            Puzzle5dEditFastenerStage::Closing => Err(Fault::from("puzzle5d-edit-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dEditFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dEditFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dEditFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dRetargetFastenerStage {
    Fastener,
    SourceGrip,
    TargetGrip,
    Duplicate,
    Compatibility,
    Disconnect,
    Connect,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dRetargetFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dRetargetFastenerWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) fastener: Option<Puzzle5dFastener>,
    pub(super) source: Option<String>,
    pub(super) target: Option<String>,
    pub(super) source_kind: Option<String>,
    pub(super) target_kind: Option<String>,
}

pub(super) struct Puzzle5dRetargetFastenerWork {
    pub(super) stage: Puzzle5dRetargetFastenerStage,
    pub(super) part_cursor: usize,
    pub(super) grip_cursor: usize,
    pub(super) fastener_cursor: usize,
    pub(super) compatibility_cursor: usize,
    pub(super) processed_units: usize,
    pub(super) fastener: Option<Puzzle5dFastener>,
    pub(super) source: Option<String>,
    pub(super) target: Option<String>,
    pub(super) source_kind: Option<String>,
    pub(super) target_kind: Option<String>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dRetargetFastenerWorkOwners>,
}

impl Default for Puzzle5dRetargetFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dRetargetFastenerStage::Fastener,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            fastener: None,
            source: None,
            target: None,
            source_kind: None,
            target_kind: None,
            mutations: Vec::with_capacity(2),
        }
    }
}

impl Puzzle5dRetargetFastenerWork {
    fn argument<'a>(command: &'a Puzzle5dCommand, primary: &str, alias: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(primary).or_else(|| args.get(alias))).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    pub(super) fn scan_grip(&mut self, snapshot: &Puzzle5dPlaySnapshot, target: &str) -> Puzzle5dGripScan {
        let projection = puzzle5d_editor_projection(snapshot);
        let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
            return Puzzle5dGripScan::Exhausted;
        };
        let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
            self.part_cursor += 1;
            self.grip_cursor = 0;
            return Puzzle5dGripScan::Progress;
        };
        self.grip_cursor += 1;
        let Some(part_id) = part.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Puzzle5dGripScan::Progress };
        if puzzle5d_grip_full_id(part_id, grip_id) != target {
            return Puzzle5dGripScan::Progress;
        }
        let kind = grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string);
        Puzzle5dGripScan::Found(kind)
    }

    pub(super) fn complete_empty(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dRetargetFastenerStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default())
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dRetargetFastenerWork {
    fn tool_id(&self) -> &'static str {
        "retargetFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_mul(2)?.checked_add(fasteners.checked_mul(2)?)?.checked_add(compatibility)?.checked_add(2)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-retarget-fastener-work-capacity"));
        }
        self.processed_units += 1;
        match self.stage {
            Puzzle5dRetargetFastenerStage::Fastener => {
                let Some(id) = Self::argument(command, "id", "fastenerId") else { return Ok(self.complete_empty()) };
                let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)).cloned() else {
                    return Ok(self.complete_empty());
                };
                self.fastener_cursor += 1;
                if row.get("id").and_then(Value::as_str) == Some(id) {
                    let fastener: Puzzle5dFastener = puzzle5d_record_from_projection(row).map_err(|_| Fault::from("puzzle5d-retarget-fastener-malformed"))?;
                    self.source = Some(Self::argument(command, "source", "attracting").map_or_else(|| fastener.source.clone(), str::to_string));
                    self.target = Some(Self::argument(command, "target", "attracted").map_or_else(|| fastener.target.clone(), str::to_string));
                    if self.source.as_deref().is_none_or(str::is_empty) || self.target.as_deref().is_none_or(str::is_empty) || self.source == self.target {
                        return Ok(self.complete_empty());
                    }
                    self.fastener = Some(fastener);
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dRetargetFastenerStage::SourceGrip;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-fastener", "Finding fastener", "Verbindung wird gesucht"))
            }
            Puzzle5dRetargetFastenerStage::SourceGrip => {
                let source = self.source.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &source) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-source", "Finding source grip", "Quellgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.source_kind = kind;
                        self.part_cursor = 0;
                        self.grip_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::TargetGrip;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::TargetGrip => {
                let target = self.target.as_deref().unwrap_or("").to_string();
                match self.scan_grip(snapshot, &target) {
                    Puzzle5dGripScan::Progress => Ok(crate::puzzle_progress_step!("puzzle5d-retarget-target", "Finding target grip", "Zielgriff wird gesucht")),
                    Puzzle5dGripScan::Found(kind) => {
                        self.target_kind = kind;
                        self.fastener_cursor = 0;
                        self.stage = Puzzle5dRetargetFastenerStage::Duplicate;
                        Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"))
                    }
                    Puzzle5dGripScan::Exhausted => Ok(self.complete_empty()),
                }
            }
            Puzzle5dRetargetFastenerStage::Duplicate => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let id = row.get("id").and_then(Value::as_str).unwrap_or("");
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let own_id = self.fastener.as_ref().map_or("", |fastener| fastener.id.as_str());
                    let next_source = self.source.as_deref().unwrap_or("");
                    let next_target = self.target.as_deref().unwrap_or("");
                    if id != own_id && ((source == next_source && target == next_target) || (source == next_target && target == next_source)) {
                        return Ok(self.complete_empty());
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-duplicate", "Checking duplicate fastener", "Doppelte Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dRetargetFastenerStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.source_kind.is_none() || self.target_kind.is_none() {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-retarget-disconnect", "Disconnecting old fastener", "Alte Verbindung wird getrennt"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else { return Ok(self.complete_empty()) };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.source_kind.as_deref().unwrap_or("");
                let target_kind = self.target_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dRetargetFastenerStage::Disconnect;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dRetargetFastenerStage::Disconnect => {
                let id = self.fastener.as_ref().map(|fastener| fastener.id.clone()).ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id));
                self.stage = Puzzle5dRetargetFastenerStage::Connect;
                Ok(crate::puzzle_progress_step!("puzzle5d-retarget-connect", "Connecting retargeted fastener", "Neu ausgerichtete Verbindung wird erstellt"))
            }
            Puzzle5dRetargetFastenerStage::Connect => {
                let fastener = self.fastener.as_ref().ok_or_else(|| Fault::from("puzzle5d-retarget-fastener-owner"))?;
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(
                    fastener.id.clone(),
                    self.source.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-source-owner"))?,
                    self.target.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-retarget-target-owner"))?,
                    fastener.fastener_kind.clone(),
                    fastener.gap,
                    fastener.shift,
                    fastener.rise,
                    fastener.rotation,
                    fastener.turn,
                    fastener.tilt,
                    fastener.x,
                    fastener.y, None,
                ));
                self.stage = Puzzle5dRetargetFastenerStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }))
            }
            Puzzle5dRetargetFastenerStage::Complete => Err(Fault::from("puzzle5d-retarget-fastener-complete-repolled")),
            Puzzle5dRetargetFastenerStage::Closing => Err(Fault::from("puzzle5d-retarget-fastener-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dRetargetFastenerStage::Closing;
        self.close_owners.stage(Puzzle5dRetargetFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations), fastener: std::mem::take(&mut self.fastener), source: std::mem::take(&mut self.source), target: std::mem::take(&mut self.target), source_kind: std::mem::take(&mut self.source_kind), target_kind: std::mem::take(&mut self.target_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dRetargetFastenerStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle5dProximityConnectStage {
    Moved,
    Candidate,
    Existing,
    Compatibility,
    Emit,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle5dProximityConnectWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dProximityConnectWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
}

pub(super) struct Puzzle5dProximityConnectWork {
    pub(super) stage: Puzzle5dProximityConnectStage,
    pub(super) part_cursor: usize,
    pub(super) grip_cursor: usize,
    pub(super) fastener_cursor: usize,
    pub(super) compatibility_cursor: usize,
    pub(super) processed_units: usize,
    moved_id: Option<String>,
    moved_kind: Option<String>,
    moved_position: Option<[f64; 3]>,
    candidate_id: Option<String>,
    candidate_kind: Option<String>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) operation_nonce: u64,
    pub(super) fresh_cursor: u64,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dProximityConnectWorkOwners>,
}

impl Default for Puzzle5dProximityConnectWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dProximityConnectStage::Moved,
            part_cursor: 0,
            grip_cursor: 0,
            fastener_cursor: 0,
            compatibility_cursor: 0,
            processed_units: 0,
            moved_id: None,
            moved_kind: None,
            moved_position: None,
            candidate_id: None,
            candidate_kind: None,
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
            operation_nonce: 0,
            fresh_cursor: 0,
        }
    }
}

impl Puzzle5dProximityConnectWork {
    fn argument<'a>(command: &'a Puzzle5dCommand, key: &str) -> Option<&'a str> {
        command.args().and_then(|args| args.get(key)).and_then(Value::as_str).filter(|value| !value.is_empty())
    }

    pub(super) fn grip_kind(grip: &Value) -> Option<String> {
        grip.get("gripKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()).or_else(|| grip.get("2d").and_then(|value| value.get("gripKind")).and_then(Value::as_str).filter(|kind| !kind.is_empty())).map(str::to_string)
    }

    pub(super) fn world_position(part: &Value, grip: &Value) -> [f64; 3] {
        let origin = part.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let orientation = part.get("3d").and_then(|part| part.get("orientation")).and_then(puzzle5d_value_as_f64_4).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let position = grip.get("3d").and_then(|grip| grip.get("position")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default();
        let rotated = quat_rotate_vector(orientation, position);
        [origin[0] + rotated[0], origin[1] + rotated[1], origin[2] + rotated[2]]
    }

    fn clear_candidate(&mut self) {
        self.candidate_id = None;
        self.candidate_kind = None;
        self.fastener_cursor = 0;
        self.compatibility_cursor = 0;
        self.stage = Puzzle5dProximityConnectStage::Candidate;
    }

    pub(super) fn complete(&mut self) -> semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>> {
        self.stage = Puzzle5dProximityConnectStage::Complete;
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() })
    }
}

impl Puzzle5dProximityConnectWork {
    /// 🪪️ Binds the operation nonce its minted ids derive from.
    pub(super) fn bound(mut self, operation_nonce: u64) -> Self {
        self.operation_nonce = operation_nonce;
        self
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dProximityConnectWork {
    fn tool_id(&self) -> &'static str {
        "proximityConnect"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let parts = projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len);
        let fasteners = projection.get("fasteners").and_then(Value::as_array).map_or(0, Vec::len);
        let compatibility = projection.get("kindCompatibility").and_then(Value::as_array).map_or(0, Vec::len);
        let items = parts.checked_add(fasteners.checked_mul(parts)?)?.checked_add(compatibility.checked_mul(parts)?)?.checked_add(parts)?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        if self.processed_units >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
            return Err(Fault::from("puzzle5d-proximity-connect-work-capacity"));
        }
        self.processed_units += 1;
        let part_id = Self::argument(command, "partId").or_else(|| Self::argument(command, "objectId")).unwrap_or("");
        if part_id.is_empty() {
            return Ok(self.complete());
        }
        match self.stage {
            Puzzle5dProximityConnectStage::Moved => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                self.part_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.first()) else { return Ok(self.complete()) };
                    let Some(grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(self.complete()) };
                    self.moved_id = Some(puzzle5d_grip_full_id(part_id, grip_id));
                    self.moved_kind = Self::grip_kind(grip);
                    self.moved_position = Some(Self::world_position(part, grip));
                    self.part_cursor = 0;
                    self.grip_cursor = 0;
                    self.stage = Puzzle5dProximityConnectStage::Candidate;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-moved", "Finding moved grip", "Verschobener Griff wird gesucht"))
            }
            Puzzle5dProximityConnectStage::Candidate => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    return Ok(self.complete());
                };
                let Some(grip) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)) else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.grip_cursor += 1;
                if part.get("id").and_then(Value::as_str) == Some(part_id) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let Some(peer_part_id) = part.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed part", "Fehlerhaftes Teil wird übersprungen")) };
                let Some(peer_grip_id) = grip.get("id").and_then(Value::as_str) else { return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Skipping malformed grip", "Fehlerhafter Griff wird übersprungen")) };
                let peer_id = puzzle5d_grip_full_id(peer_part_id, peer_grip_id);
                if self.moved_id.as_deref() == Some(peer_id.as_str()) {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                let moved = self.moved_position.ok_or_else(|| Fault::from("puzzle5d-proximity-position-owner"))?;
                let peer = Self::world_position(part, grip);
                let radius = command.args().and_then(|args| args.get("radius")).and_then(Value::as_f64).unwrap_or(PUZZLE5D_PROXIMITY_RADIUS).max(0.0);
                let dx = moved[0] - peer[0];
                let dy = moved[1] - peer[1];
                let dz = moved[2] - peer[2];
                if (dx * dx + dy * dy + dz * dz).sqrt() > radius {
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                }
                self.candidate_id = Some(peer_id);
                self.candidate_kind = Self::grip_kind(grip);
                self.fastener_cursor = 0;
                self.stage = Puzzle5dProximityConnectStage::Existing;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Existing => {
                if let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.fastener_cursor)) {
                    self.fastener_cursor += 1;
                    let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                    let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                    let peer = self.candidate_id.as_deref().unwrap_or("");
                    let moved = self.moved_id.as_deref().unwrap_or("");
                    if (source == peer && target == moved) || (source == moved && target == peer) {
                        self.clear_candidate();
                    }
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-existing", "Checking existing fastener", "Bestehende Verbindung wird geprüft"));
                }
                self.stage = Puzzle5dProximityConnectStage::Compatibility;
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Compatibility => {
                let rows = projection.get("kindCompatibility").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
                if rows.is_empty() || self.candidate_kind.is_none() || self.moved_kind.is_none() {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-emit", "Connecting nearby grip", "Naher Griff wird verbunden"));
                }
                let Some(row) = rows.get(self.compatibility_cursor) else {
                    self.clear_candidate();
                    return Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"));
                };
                self.compatibility_cursor += 1;
                let source = row.get("source").and_then(Value::as_str).unwrap_or("");
                let target = row.get("target").and_then(Value::as_str).unwrap_or("");
                let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
                let source_kind = self.candidate_kind.as_deref().unwrap_or("");
                let target_kind = self.moved_kind.as_deref().unwrap_or("");
                if (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind) {
                    self.stage = Puzzle5dProximityConnectStage::Emit;
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-compatibility", "Checking kind compatibility", "Artkompatibilität wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Emit => {
                if self.mutations.len() >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-proximity-connect-output-capacity"));
                }
                let id = format!("fastener-{:016x}-{}", self.operation_nonce, self.fresh_cursor);
                self.fresh_cursor = self.fresh_cursor.saturating_add(1);
                let source = self.candidate_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-source-owner"))?;
                let target = self.moved_id.as_ref().cloned().ok_or_else(|| Fault::from("puzzle5d-proximity-target-owner"))?;
                let arg = |key: &str| command.args().and_then(|args| args.get(key)).and_then(Value::as_f64).unwrap_or(0.0);
                let kind = command.args().and_then(|args| args.get("fastenerKind").or_else(|| args.get("edgeKind"))).and_then(Value::as_str).filter(|kind| !kind.is_empty()).map(str::to_string);
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::connect_grips(id, source, target, kind, arg("gap"), arg("shift"), arg("rise"), arg("rotation"), arg("turn"), arg("tilt"), arg("x"), arg("y"), None));
                self.clear_candidate();
                Ok(crate::puzzle_progress_step!("puzzle5d-proximity-candidate", "Scanning nearby grip", "Naher Griff wird geprüft"))
            }
            Puzzle5dProximityConnectStage::Complete => Err(Fault::from("puzzle5d-proximity-connect-complete-repolled")),
            Puzzle5dProximityConnectStage::Closing => Err(Fault::from("puzzle5d-proximity-connect-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dProximityConnectStage::Closing;
        self.close_owners.stage(Puzzle5dProximityConnectWorkOwners { mutations: std::mem::take(&mut self.mutations), moved_id: std::mem::take(&mut self.moved_id), moved_kind: std::mem::take(&mut self.moved_kind), moved_position: std::mem::take(&mut self.moved_position), candidate_id: std::mem::take(&mut self.candidate_id), candidate_kind: std::mem::take(&mut self.candidate_kind) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dProximityConnectStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dPatchGripWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dPatchGripWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) selected: HashSet<String>,
}

pub(super) struct Puzzle5dPatchGripWork {
    pub(super) stage: Puzzle5dPatchPartStage,
    selection_cursor: usize,
    pub(super) part_cursor: usize,
    pub(super) grip_cursor: usize,
    processed_grips: usize,
    pub(super) selected: HashSet<String>,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dPatchGripWorkOwners>,
}

impl Default for Puzzle5dPatchGripWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle5dPatchPartStage::Selection,
            selection_cursor: 0,
            part_cursor: 0,
            grip_cursor: 0,
            processed_grips: 0,
            selected: HashSet::with_capacity(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS),
            mutations: Vec::with_capacity(crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS),
        }
    }
}

impl Puzzle5dPatchGripWork {
    fn source_len(command: &Puzzle5dCommand) -> usize {
        let args = command.args();
        args.and_then(|args| args.get("gripFullIds")).and_then(Value::as_array).map_or(0, Vec::len) + usize::from(args.and_then(|args| args.get("gripFullId")).and_then(Value::as_str).is_some())
    }

    fn source_id(command: &Puzzle5dCommand, index: usize) -> Option<&str> {
        let args = command.args()?;
        let ids = args.get("gripFullIds").and_then(Value::as_array);
        if let Some(id) = ids.and_then(|ids| ids.get(index)).and_then(Value::as_str) {
            return (!id.is_empty()).then_some(id);
        }
        (index == ids.map_or(0, Vec::len)).then(|| args.get("gripFullId").and_then(Value::as_str)).flatten().filter(|id| !id.is_empty())
    }

    pub(super) fn patch(command: &Puzzle5dCommand, grip: &mut crate::Puzzle5dGrip) -> bool {
        let Some(args) = command.args() else { return false };
        let field = args.get("field").and_then(Value::as_str).unwrap_or("");
        let value = args.get("value");
        let delta = args.get("delta");
        let text = value.and_then(Value::as_str);
        match field {
            "gripKind" => {
                let Some(text) = text else { return false };
                grip.grip_kind = Some(text.to_string());
                grip.grip_2d.grip_kind = Some(text.to_string());
            }
            "angle" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_2d.angle, value, delta) else { return false };
                grip.grip_2d.angle = updated;
            }
            "radius" => {
                let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.radius.unwrap_or(0.0), value, delta) else { return false };
                grip.grip_2d.radius = Some(updated);
                grip.grip_3d.radius = Some(updated);
            }
            "label" => grip.grip_3d.label = text.filter(|text| !text.is_empty()).map(str::to_string),
            _ => {
                if let Some(axis) = puzzle5d_axis_index(field, "position") {
                    let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.position[axis], value, delta) else { return false };
                    grip.grip_3d.position[axis] = updated;
                } else if let Some(axis) = puzzle5d_axis_index(field, "direction") {
                    let mut direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
                    let Some(updated) = puzzle5d_resolve_number_edit(direction[axis], value, delta) else { return false };
                    direction[axis] = updated;
                    grip.grip_3d.direction = Some(direction);
                } else {
                    return false;
                }
            }
        }
        true
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dPatchGripWork {
    fn tool_id(&self) -> &'static str {
        "patchGrip"
    }

    fn extent(&self, command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        let items = Self::source_len(command).checked_add(projection.get("parts").and_then(Value::as_array).map_or(0, Vec::len))?;
        (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        match self.stage {
            Puzzle5dPatchPartStage::Selection => {
                if let Some(id) = Self::source_id(command, self.selection_cursor) {
                    if self.selected.len() >= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle5d-patch-grip-selection-capacity"));
                    }
                    self.selected.insert(id.to_string());
                    self.selection_cursor += 1;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip-selection", "Reading grip target", "Griffziel wird gelesen"));
                }
                self.stage = Puzzle5dPatchPartStage::Parts;
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Parts => {
                let Some(part) = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.get(self.part_cursor)) else {
                    self.stage = Puzzle5dPatchPartStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
                };
                let Some(grip_value) = part.get("grips").and_then(Value::as_array).and_then(|grips| grips.get(self.grip_cursor)).cloned() else {
                    self.part_cursor += 1;
                    self.grip_cursor = 0;
                    return Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip-part", "Advancing grip owner", "Griffinhaber wird gewechselt"));
                };
                if self.processed_grips >= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS {
                    return Err(Fault::from("puzzle5d-patch-grip-work-capacity"));
                }
                self.processed_grips += 1;
                self.grip_cursor += 1;
                let part_id = part.get("id").and_then(Value::as_str).ok_or_else(|| Fault::from("puzzle5d-patch-grip-part-id-malformed"))?;
                let mut grip: crate::Puzzle5dGrip = <crate::Puzzle5dGrip as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&grip_value)).map_err(|_| Fault::from("puzzle5d-patch-grip-malformed"))?;
                let full_id = puzzle5d_grip_full_id(part_id, &grip.id);
                if self.selected.contains(&full_id) && Self::patch(command, &mut grip) {
                    let grip_id = grip.id.clone();
                    self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::replace_part_grip(part_id.to_string(), grip_id, grip));
                }
                Ok(crate::puzzle_progress_step!("puzzle5d-patch-grip", "Patching grip", "Griff wird geändert"))
            }
            Puzzle5dPatchPartStage::Complete => Err(Fault::from("puzzle5d-patch-grip-complete-repolled")),
            Puzzle5dPatchPartStage::Closing => Err(Fault::from("puzzle5d-patch-grip-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle5dPatchPartStage::Closing;
        self.close_owners.stage(Puzzle5dPatchGripWorkOwners { mutations: std::mem::take(&mut self.mutations), selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle5dPatchPartStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

/// ♻️ The owners one `Puzzle5dDeleteFastenerWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
struct Puzzle5dDeleteFastenerWorkOwners {
    pub(super) mutations: Vec<Puzzle5dMutation>,
}

pub(super) struct Puzzle5dDeleteFastenerWork {
    pub(super) cursor: usize,
    pub(super) closing: bool,
    pub(super) mutations: Vec<Puzzle5dMutation>,
    pub(super) close_owners: crate::puzzle_job::WorkClosing<Puzzle5dDeleteFastenerWorkOwners>,
}

impl Default for Puzzle5dDeleteFastenerWork {
    fn default() -> Self {
        Self { close_owners: Default::default(), cursor: 0, closing: false, mutations: Vec::with_capacity(1) }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<Puzzle5dPlayApp>> for Puzzle5dDeleteFastenerWork {
    fn tool_id(&self) -> &'static str {
        "deleteFastener"
    }

    fn extent(&self, _command: &Puzzle5dCommand, snapshot: &Puzzle5dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Puzzle5dPlayApp>>>) -> Option<usize> {
        let projection = puzzle5d_editor_projection(snapshot);
        projection.get("fasteners").and_then(Value::as_array).map_or(Some(1), |fasteners| fasteners.len().checked_add(1)).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        crate::retained_command::step_demands::<EditorApp<Puzzle5dPlayApp>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Puzzle5dPlayApp>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<Puzzle5dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, interaction: _interaction, hover: _hover, context: _context, .. } = *input;
        let projection = puzzle5d_editor_projection(snapshot);
        let target = command.args().and_then(|args| args.get("id").or_else(|| args.get("fastenerId"))).and_then(Value::as_str).filter(|id| !id.is_empty());
        let Some(row) = projection.get("fasteners").and_then(Value::as_array).and_then(|fasteners| fasteners.get(self.cursor)) else {
            return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { artifact_mutations: std::mem::take(&mut self.mutations), ui_scope: UiDirtyScope::Full, ..Default::default() }));
        };
        self.cursor += 1;
        if target == row.get("id").and_then(Value::as_str) {
            if let Some(id) = target {
                self.mutations.push(crate::standards::v1::subsets::any::schema::mutations::disconnect_grips(id.to_string()));
            }
        }
        Ok(crate::puzzle_progress_step!("puzzle5d-delete-fastener", "Scanning fastener", "Verbindung wird geprüft"))
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.close_owners.stage(Puzzle5dDeleteFastenerWorkOwners { mutations: std::mem::take(&mut self.mutations) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
