//! 🌊️ Job propagate, backtrack and checkpoint: the propagation, the contradiction detection, the backtracking, the checkpoint and commit materialization steps and the preview emission. See [`super`].

use super::*;

//#region 🧩️Job
impl<T: Topology + Clone> WfcJob<T> {
    fn acquire_source(&mut self) {
        let Some(source) = self.pop_queue() else {
            self.state.stage = WfcStage::DetectContradiction;
            return;
        };
        if self.state.propagation_wave.len() < PREVIEW_ITEM_LIMIT {
            self.state.propagation_wave.push(source);
        }
        self.state.arc_cursor = Some(ArcCursor::new(source, self.topology.out_arc_bound(source)));
    }

    pub(super) fn propagate_one(&mut self) {
        if self.state.contradiction.is_some() {
            self.state.stage = WfcStage::DetectContradiction;
            return;
        }
        if self.state.arc_cursor.is_none() {
            self.acquire_source();
            return;
        }
        let cursor = self.state.arc_cursor.as_mut().expect("arc cursor");
        if cursor.target.is_none() {
            if cursor.raw_index == cursor.raw_bound {
                self.state.arc_cursor = None;
                return;
            }
            let raw = cursor.raw_index;
            cursor.raw_index += 1;
            if let Some((target, relation)) = self.topology.out_arc_at(cursor.source, raw) {
                cursor.target = Some(target);
                cursor.relation = Some(relation);
                cursor.allowed = Some(PatternSet::with_word_capacity(self.model.pattern_count()));
            }
            return;
        }

        let word_count = self.model.pattern_count().div_ceil(64);
        if cursor.allowed_word < word_count {
            if cursor.allowed_pattern < self.model.pattern_count() {
                let pattern = PatternId::from_index(cursor.allowed_pattern);
                cursor.allowed_pattern += 1;
                if self.state.domains[cursor.source.index()].get(pattern) {
                    cursor.allowed_accumulator |= self.model.allowed(cursor.relation.expect("relation"), pattern).word(cursor.allowed_word);
                }
            } else {
                cursor.allowed.as_mut().expect("allowed domain").push_word(cursor.allowed_accumulator);
                cursor.allowed_word += 1;
                cursor.allowed_pattern = 0;
                cursor.allowed_accumulator = 0;
            }
            return;
        }

        let target = cursor.target.expect("target");
        if cursor.removed_word != 0 {
            let bit = cursor.removed_word.trailing_zeros() as usize;
            cursor.removed_word &= cursor.removed_word - 1;
            let pattern = PatternId::from_index(cursor.removed_word_index * 64 + bit);
            let _ = cursor;
            self.remove_pattern(target, pattern, true);
            self.state.arc_cursor.as_mut().expect("arc cursor").changed = true;
            return;
        }
        if cursor.restrict_word < word_count {
            let word_index = cursor.restrict_word;
            cursor.restrict_word += 1;
            let old = self.state.domains[target.index()].word(word_index);
            let new = old & cursor.allowed.as_ref().expect("allowed domain").word(word_index);
            cursor.removed_word = old & !new;
            cursor.removed_word_index = word_index;
            return;
        }

        self.state.compatibility_edges += 1;
        if cursor.changed {
            self.state.revisions[target.index()] += 1;
            if self.state.changed_domains.len() < PREVIEW_ITEM_LIMIT && !self.state.changed_domains.contains(&target) {
                self.state.changed_domains.push(target);
            }
            if self.state.domain_counts[target.index()] == 0 {
                self.state.contradiction = Some(target);
            } else {
                self.push_queue(target);
                self.push_entropy(target);
            }
        }
        self.state.arc_cursor.as_mut().expect("arc cursor").clear_arc();
    }

    pub(super) fn detect(&mut self) {
        if self.state.contradiction.is_some() || self.state.empty_count > 0 {
            self.state.backtrack_frame = self.state.decisions.pop();
            self.state.backtracks += 1;
            self.state.stage = WfcStage::BacktrackTrailEntry;
        } else if self.state.queue.is_empty() && self.state.arc_cursor.is_none() {
            self.state.stage = if self.state.singleton_count == self.state.domains.len() { WfcStage::Complete } else { WfcStage::CommitSlot };
        } else {
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
        }
    }

    pub(super) fn backtrack_one(&mut self) {
        let Some(frame) = self.state.backtrack_frame else {
            self.state.stage = WfcStage::Complete;
            return;
        };
        if self.state.trail.len() > frame.trail_mark {
            let removed = self.state.trail.pop().expect("trail past frame mark");
            self.add_pattern(removed.node, removed.pattern);
            self.state.revisions[removed.node.index()] += 1;
            self.push_entropy(removed.node);
            if self.state.backtrack_path.len() < PREVIEW_ITEM_LIMIT && self.state.backtrack_path.last() != Some(&removed.node) {
                self.state.backtrack_path.push(removed.node);
            }
            return;
        }
        self.state.backtrack_frame = None;
        self.state.contradiction = None;
        self.state.arc_cursor = None;
        self.reset_queue();
        self.remove_pattern(frame.node, frame.candidate, true);
        self.state.revisions[frame.node.index()] += 1;
        if self.state.domain_counts[frame.node.index()] == 0 {
            self.state.contradiction = Some(frame.node);
            self.state.stage = WfcStage::DetectContradiction;
        } else {
            self.push_queue(frame.node);
            self.push_entropy(frame.node);
            self.state.stage = WfcStage::PropagateCompatibilityEdge;
        }
    }

    fn reset_preview_delta(&mut self) {
        self.state.propagation_wave.clear();
        self.state.changed_domains.clear();
        self.state.backtrack_path.clear();
    }

    pub(super) fn begin_checkpoint(&mut self, terminal: bool) -> Result<(), WfcFault> {
        self.checkpoint_build = Some(CheckpointBuild::new(&self.state, self.model.pattern_count(), terminal)?);
        self.state.stage = WfcStage::MaterializeCheckpoint;
        Ok(())
    }

    pub(super) fn checkpoint_one(&mut self) -> Result<Option<Vec<u8>>, WfcFault> {
        let build = self.checkpoint_build.as_mut().expect("checkpoint build");
        match build.phase {
            CheckpointPhase::Header => {
                ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_FIXED_HEADER_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                build.bytes.extend_from_slice(CHECKPOINT_MAGIC);
                let fields: [u64; CHECKPOINT_HEADER_U64_FIELDS] = [
                    self.state.operation_id,
                    self.state.base_revision,
                    self.state.generation,
                    self.state.operation_seed,
                    self.state.preview_sequence,
                    self.state.topology_nodes as u64,
                    self.state.model_fingerprint,
                    u64::from(build.terminal),
                    self.state.rng.state[0],
                    self.state.rng.state[1],
                    self.state.rng.state[2],
                    self.state.rng.state[3],
                    self.state.observations,
                    self.state.compatibility_edges,
                    self.state.backtracks,
                    self.state.domains.len() as u64,
                    self.model.pattern_count() as u64,
                    self.state.trail.len() as u64,
                    self.state.decisions.len() as u64,
                    self.state.observed.len() as u64,
                ];
                for field in fields {
                    put_u64(&mut build.bytes, field);
                }
                build.phase = CheckpointPhase::Domains;
            }
            CheckpointPhase::Domains => {
                if build.outer < self.state.domains.len() {
                    let domain = &self.state.domains[build.outer];
                    if build.inner < domain.word_count() {
                        ensure_materialization_space(&build.bytes, build.byte_limit, size_of::<u64>(), b"wfc-checkpoint-byte-limit-exceeded")?;
                        put_u64(&mut build.bytes, domain.word(build.inner));
                        build.inner += 1;
                    } else {
                        build.outer += 1;
                        build.inner = 0;
                    }
                } else {
                    build.phase = CheckpointPhase::Trail;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Trail => {
                if let Some(removal) = self.state.trail.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_TRAIL_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, removal.node.get());
                    put_u32(&mut build.bytes, removal.pattern.get());
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Decisions;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Decisions => {
                if let Some(decision) = self.state.decisions.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_DECISION_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, decision.node.get());
                    put_u32(&mut build.bytes, decision.candidate.get());
                    put_u64(&mut build.bytes, decision.trail_mark as u64);
                    for word in decision.rng_state {
                        put_u64(&mut build.bytes, word);
                    }
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Observed;
                    build.outer = 0;
                }
            }
            CheckpointPhase::Observed => {
                if let Some(&(node, pattern)) = self.state.observed.get(build.outer) {
                    ensure_materialization_space(&build.bytes, build.byte_limit, CHECKPOINT_OBSERVED_ENTRY_BYTES, b"wfc-checkpoint-byte-limit-exceeded")?;
                    put_u32(&mut build.bytes, node.get());
                    put_u32(&mut build.bytes, pattern.get());
                    build.outer += 1;
                } else {
                    build.phase = CheckpointPhase::Done;
                }
            }
            CheckpointPhase::Done => {
                return Ok(Some(std::mem::take(&mut build.bytes)));
            }
        }
        Ok(None)
    }

    pub(super) fn commit_one(&mut self) -> Result<Option<Vec<u8>>, WfcFault> {
        let build = self.commit_build.as_mut().expect("commit build");
        if !build.started {
            ensure_materialization_space(&build.bytes, build.byte_limit, br#"{"assignment":["#.len(), b"wfc-commit-byte-limit-exceeded")?;
            build.bytes.extend_from_slice(br#"{"assignment":["#);
            build.started = true;
            return Ok(None);
        }
        if build.cursor < self.state.domains.len() {
            if build.assignment.len() >= build.item_limit {
                return Err(empty_job_fault());
            }
            let value = self.state.domains[build.cursor].first_set().expect("complete domain").get();
            let encoded = value.to_string();
            let additional = usize::from(build.cursor != 0).checked_add(encoded.len()).ok_or_else(empty_job_fault)?;
            ensure_materialization_space(&build.bytes, build.byte_limit, additional, b"wfc-commit-byte-limit-exceeded")?;
            if build.cursor != 0 {
                build.bytes.push(b',');
            }
            build.assignment.push(value);
            build.bytes.extend_from_slice(encoded.as_bytes());
            build.cursor += 1;
            return Ok(None);
        }
        if build.assignment.len() != build.item_limit {
            return Err(empty_job_fault());
        }
        let tail = format!("],\"observations\":{},\"compatibility_edges\":{},\"backtracks\":{}}}", self.state.observations, self.state.compatibility_edges, self.state.backtracks);
        ensure_materialization_space(&build.bytes, build.byte_limit, tail.len(), b"wfc-commit-byte-limit-exceeded")?;
        build.bytes.extend_from_slice(tail.as_bytes());
        self.completed_commit = Some(WfcCommit { assignment: std::mem::take(&mut build.assignment), observations: self.state.observations, compatibility_edges: self.state.compatibility_edges, backtracks: self.state.backtracks });
        Ok(Some(std::mem::take(&mut build.bytes)))
    }

    pub fn take_completed_commit(&mut self) -> Option<WfcCommit> {
        self.completed_commit.take()
    }

    /// 🤝️ The sealed commit state payload this job lent with its completion, still owned here until close.
    pub fn commit_state(&self) -> Option<&semio_framework_job_payload::RetainedJobPayload> {
        self.publication.as_ref().and_then(|publication| publication.commit_state())
    }

    pub(super) fn preview_stage(&self) -> bool {
        matches!(self.state.stage, WfcStage::InitializeDomains | WfcStage::FindMinimumEntropySlot | WfcStage::ChooseCandidate | WfcStage::PropagateCompatibilityEdge | WfcStage::DetectContradiction | WfcStage::BacktrackTrailEntry)
    }

    pub(super) fn preview_due(&self, now_ms: u64) -> bool {
        self.last_preview_ms.is_none() || self.preview_units >= PREVIEW_UNIT_INTERVAL || self.last_preview_ms.is_some_and(|last| now_ms.saturating_sub(last) >= PREVIEW_TIME_INTERVAL_MS)
    }

    pub(super) fn emit_preview(&mut self, context: &mut StepContext<'_>, now_ms: u64) -> WfcRun {
        let sequence = match context.next_preview_sequence() {
            Ok(sequence) => sequence.max(self.state.preview_sequence),
            Err(_) => return self.stage_fault(Vec::new()),
        };
        self.state.preview_sequence = sequence + 1;
        self.operation.preview_sequence = self.state.preview_sequence;
        let preview = self.preview(sequence);
        self.reset_preview_delta();
        self.preview_units = 0;
        self.last_preview_ms = Some(now_ms);
        let bytes = semio_framework_pack_json::to_json_string(&preview).into_bytes();
        self.publication = Some(Publication::new(PublicationKind::Preview, bytes, Vec::new()));
        WfcRun::Staged
    }

    pub(super) fn stage_fault(&mut self, detail: Vec<u8>) -> WfcRun {
        self.publication = Some(Publication::new(PublicationKind::Fault, detail, Vec::new()));
        WfcRun::Staged
    }
}
//#endregion 🧩️Job
