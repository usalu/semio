# Original Lifecycle Frontiers

Retained input for the active full-grant migration. These original bodies are not a runtime certificate.

## quotes

```rust
        fn next_maintenance_byte_demand(&self) -> usize { if self.command_prune_requested() || self.pending_command_prune.is_some() || !self.command_log.pruned_terminal_is_empty() { self.command_prune_next_release_byte_demand() } else { self.next_closing_private_child_group().map_or(0,|operation|self.private_child_group_operation_close_byte_demand(operation).unwrap_or(1)) } }
        fn next_close_byte_demand(&self) -> usize {
            if self.close_cancellation_cursor>=TOOL_CANCELLATION_SLOTS+ARTIFACT_LIVE_OUTPUT_SLOTS&&self.latest_wins_keys.terminal_is_empty()&&self.latest_wins_order.items.is_empty(){if let Some((_,id))=self.tool_operations.next_id_from(self.close_typed_operation_cursor){if let Some(operation)=self.tool_operations.get(id){let bytes=operation.reserved_close_byte_demand();if bytes!=0{return bytes;}}}}
            if self.pending_command_prune.is_some() || self.command_prune_requested() { return 0; }
            if !self.command_log.pruned_terminal_is_empty() { return self.command_log.next_pruned_close_byte_demand().unwrap_or(1); }
            if !self.private_child_groups_terminal_is_empty() { return self.private_child_group_close_byte_demand(); }
            if self.close_envelope_completed_records_drained && !self.close_store_replacement_jobs_drained {
                if let Some((_, operation)) = self.store_replacement_jobs.next_id_from(self.close_store_replacement_cursor) {
                    if let Some(active) = self.store_replacement_jobs.get(operation) { return active.next_close_byte_demand(); }
                }
            }
            if self.close_store_replacement_jobs_drained && !self.close_document_archive_loads_drained {
                if let Some((_, operation)) = self.document_archive_loads.next_id_from(self.close_document_archive_cursor) {
                    if let Some(active) = self.document_archive_loads.get(operation) { return active.next_close_byte_demand(); }
                }
            }
            if self.close_owned_stage >= 8 && !self.command_log.terminal_is_empty() {
                return self.command_log.next_close_byte_demand().unwrap_or(1);
            }
            if self.close_owned_stage >= 8 && self.command_log.terminal_is_empty() && self.history_dirty_sequences.is_empty() && self.pending_child_pins.is_empty() && !self.composition.terminal_is_empty() {
                return self.composition.next_close_byte_demand();
            }
            match self.close_owned_stage {
                0 => self.store.next_close_byte_demand(),
                1 => self.config_store.next_close_byte_demand(),
                2 => self.draft_store.next_close_byte_demand(),
                7 => self.interaction_store.next_close_byte_demand(),
                _ => usize::from(!self.close_terminal_is_empty()),
            }
        }

```

## close

```rust
        /// 🧾️ A stage that reports `Complete` HANDED OFF: one retained authority really did cross the
        /// close boundary, so the ladder owes `released_items: 1`, not `Pending { 0, 0 }` — the same
        /// convention `ArtifactStoreEnvelopeRetirement::close_step` was corrected to on 2026-09-10.
        /// `Pending { 0, 0 }` is what the structural livelock accountant reads as "this ladder is
        /// stuck" (`RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT = 8`), and three consecutive hand-offs plus a
        /// couple of real waits spend that whole credit on a ladder that was making progress the
        /// entire time — reproduced intermittently by the close-cost fixture's eight-document session
        /// as `plugin.internal.zero-progress` (ticket 26/09/09).
        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.pending_command_prune.is_some() || self.command_prune_requested() || !self.command_log.pruned_terminal_is_empty() {
                return self.advance_command_prune_step(maximum_items, maximum_bytes, true);
            }
            if !self.private_child_groups_terminal_is_empty() { return self.close_private_child_group_step(maximum_items, maximum_bytes); }
            #[cfg(not(target_arch = "wasm32"))]
            if std::env::var_os("SEMIO_DEBUG_CLOSE_PHASE").is_some() {
                std::thread_local! { static CLOSE_PHASE_DIAGNOSTIC: std::cell::Cell<(u8, Option<std::time::Instant>)> = const { std::cell::Cell::new((0, None)) }; }
                CLOSE_PHASE_DIAGNOSTIC.with(|state| {
                    let (count, previous) = state.get();
                    if count < 8 && previous.is_none_or(|time| time.elapsed() >= std::time::Duration::from_secs(10)) {
                        state.set((count + 1, Some(std::time::Instant::now())));
                        eprintln!("[DEBUG] app close phase app={:p} stage={} grant={} query={} timeTravelEmpty={} localQuery={} returnedReadsEmpty={} documentPump={:?} configPump={:?} interactionPump={:?} documentPhase={}", self as *const Self, self.close_owned_stage, maximum_bytes, self.next_close_byte_demand(), self.time_travel.terminal_is_empty(), self.local_interaction_query.is_some(), self.snapshot_read_returns_terminal_is_empty(), self.document_snapshot_read_returns.active.as_ref().map(|owner| (owner.terminal_is_empty(), store::artifact_retirement_box_byte_demand(owner))), self.config_snapshot_read_returns.active.as_ref().map(|owner| (owner.terminal_is_empty(), store::artifact_retirement_box_byte_demand(owner))), self.interaction_snapshot_read_returns.active.as_ref().map(|owner| (owner.terminal_is_empty(), store::artifact_retirement_box_byte_demand(owner))), self.store.close_owned_phase_witness());
                        eprintln!("[DEBUG] app close earlier ladder toolRunsEmpty={} presenceIdle={} liveInstance={:?} cancelCursor={} latestEmpty={} orderCount={} typedEmpty={} instanceDrained={} mediaCursor={} mediaCleanup={} segmentCursor={} segmentCleanup={} snapshotCursor={} childMembersEmpty={} childRootsEmpty={} childDetached={} childMemberActive={} childrenEmpty={} peerPublicationsEmpty={}", self.tool_runs.terminal_is_empty(), self.presence_store.local_read_maintenance_is_idle(), self.live_runtime_instance_id, self.close_cancellation_cursor, self.latest_wins_keys.terminal_is_empty(), self.latest_wins_order.items.len(), self.tool_operations.is_empty(), self.close_instance_operation_owner_drained, self.close_media_cursor, self.close_media_cleanup_cursor, self.close_segment_cursor, self.close_segment_cleanup_cursor, self.close_snapshot_cursor, self.child_member_retirements.is_empty(), self.child_content_retirements.is_empty(), self.close_child_root_detached, self.close_child_member.is_some(), self.children.is_empty(), self.peer_roster_publications.is_empty());
                        eprintln!("[DEBUG] app close later ladder peerDetached={} ingressDrained={} decodeJobsDrained={} fieldDecodersDrained={} completedRecordsDrained={} replacementJobsDrained={} archiveLoadsDrained={} snapshotReadsDrained={}", self.close_peer_presence_detached, self.close_envelope_ingress_drained, self.close_envelope_decode_jobs_drained, self.close_envelope_field_decoders_drained, self.close_envelope_completed_records_drained, self.close_store_replacement_jobs_drained, self.close_document_archive_loads_drained, self.close_snapshot_reads_drained);
                        if let Some((_, operation)) = self.store_replacement_jobs.next_id_from(self.close_store_replacement_cursor) {
                            if let Some(active) = self.store_replacement_jobs.get(operation) {
                                eprintln!("[DEBUG] replacement close owner operation={} state={:?} retainedStore={:?} disposer={} open={:?} ingress={:?} registry={:?} contentRetirement={} content={} children={} composition={} childRetirement={} session={} rejectedSession={} outcome={}", operation, active.state, active.retained_store.as_ref().map(|owner| (owner.next_close_byte_demand(), owner.close_owned_phase_witness())), active.retained_disposer.is_some(), active.active_member_open.as_ref().map(store::MemberOpenOperation::next_close_byte_demand), active.active_member_ingress.as_ref().map(OwnedDocumentMemberIngress::next_close_byte_demand), active.member_ingress.as_ref().map(OwnedDocumentMemberIngressRegistry::next_close_byte_demand), active.candidate_content_retirement.is_some(), active.candidate_content.is_some(), active.candidate_children.is_some(), active.candidate_composition.is_some(), active.retiring_child.is_some(), active.session.is_some(), active.session_rejected.is_some(), active.retained_outcome.is_some());
                            }
                        }
                        if let Some((_, operation)) = self.document_archive_loads.next_id_from(self.close_document_archive_cursor) {
                            if let Some(active) = self.document_archive_loads.get(operation) {
                                if let Some(hydration) = active.hydration.as_ref() { eprintln!("[DEBUG] archive hydration {}", hydration.close_phase_witness()); }
                                eprintln!("[DEBUG] archive close owner operation={} state={:?} phase={:?} target={:?} retained={:?} hydration={} ingress={} member={} history={} decodedHistory={} replacement={:?} archive={:?}", operation, active.state, active.phase, active.terminal_target, active.retained.as_ref().map(|owner| (owner.terminal_is_empty(), store::artifact_retirement_box_byte_demand(owner))), active.hydration.is_some(), active.rejected_ingress.is_some(), active.member.is_some(), active.history.is_some(), active.decoded_history.is_some(), active.replacement.map(|handle| handle.operation.0), active.archive.as_ref().map(|archive| (archive.members.len(), archive.parent_spr.len(), archive.parent_spr.capacity(), archive.parent_pack.len(), archive.parent_pack.capacity())));
                            }
                        }
                    }
                });
            }
            if !self.close_started {
                self.tool_cancellations.cancel_scope_generation();
                self.peer_roster_scope.cancel_now();
                self.local_interaction_authority.close();
                if let Some(query) = self.local_interaction_query.as_mut() {
                    query.begin_close();
                }
                self.close_started = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some((_, id)) = self.pending_reserved.next_id_from(0) {
                if let Some(pending) = self.pending_reserved.remove(id) {
                    pending.permit.finish();
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(commit) = self.reserved_commits.pop_front() {
                commit.permit.finish();
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.reserved_commit_outcome.take().is_some() {
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.tool_runs.terminal_is_empty() {
                let step = self.tool_run_close_step(maximum_items, maximum_bytes)?;
                return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if !self.time_travel.terminal_is_empty() {
                let step = self.time_travel_close_step(maximum_items, maximum_bytes)?;
                return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if self.local_interaction_query.is_some() {
                let step = self.advance_local_interaction_query_one(maximum_items, maximum_bytes)?;
                drop(self.take_local_interaction_query_reply());
                return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if !self.snapshot_read_returns_terminal_is_empty() {
                let step = self.advance_snapshot_read_returns_one(maximum_bytes)?;
                return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if !self.presence_store.retirement_started() && !self.presence_store.local_read_maintenance_is_idle() {
                return self.presence_store.maintenance_local_reads_step(1, maximum_bytes).map_err(|error| Fault::from(error.into_message())).map(|step| match step {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                    store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "presence returned local owner is held during app close" },
                    store::SnapshotRetirementStep::Complete => PluginCloseStep::Pending { released_items: 1, released_bytes: 0 },
                });
            }
            if let Some(instance_id) = self.live_runtime_instance_id {
                let mounted = A::mounted_job_close_step(instance_id, maximum_items.min(1), maximum_bytes)?;
                if mounted != PluginCloseStep::Complete {
                    return Ok(mounted);
                }
                if !A::mounted_jobs_terminal_is_empty(instance_id) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-mounted-false-terminal"), "mounted app jobs reported Complete without an exact terminal-empty witness"));
                }
            }
            if self.close_cancellation_cursor < TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS {
                let ceiling = self.close_cancellation_cursor.saturating_add(maximum_items).min(TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS);
                let mut released_items = 0;
                while self.close_cancellation_cursor < ceiling {
                    let _ = self.tool_cancellations.cleanup_slot(self.close_cancellation_cursor)?;
                    self.close_cancellation_cursor = self.close_cancellation_cursor.saturating_add(1);
                    released_items += 1;
                }
                return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
            }
            if !self.latest_wins_keys.terminal_is_empty() {
                self.latest_wins_keys.begin_close();
                let step = self.latest_wins_keys.advance(maximum_items.min(1), maximum_bytes)?;
                return Ok(if step == PluginCloseStep::Complete { PluginCloseStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if let Some(operation_id) = self.latest_wins_order.items.front().copied() {
                return self.close_latest_wins_command_step(operation_id, maximum_items, maximum_bytes);
            }
            if !self.tool_operations.is_empty() {
                return self.close_typed_operation_step(maximum_items, maximum_bytes);
            }
            if !self.close_instance_operation_owner_drained {
                let step = self.instance_operation_owner.close_step(maximum_items.min(1), maximum_bytes)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                if !self.instance_operation_owner.terminal_is_empty()? {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-instance-owner-false-terminal"), "app-instance operation owner reported Complete without an exact terminal-empty witness"));
                }
                self.close_instance_operation_owner_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.close_media_cursor < ARTIFACT_LIVE_OUTPUT_SLOTS {
                let Some(operation_id) = self.media_exports.id_at(self.close_media_cursor) else {
                    let ceiling = self.close_media_cursor.saturating_add(maximum_items).min(ARTIFACT_LIVE_OUTPUT_SLOTS);
                    let mut released_items = 0;
                    while self.close_media_cursor < ceiling && self.media_exports.id_at(self.close_media_cursor).is_none() {
                        self.close_media_cursor = self.close_media_cursor.saturating_add(1);
                        released_items += 1;
                    }
                    return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
                };
                let step = self
                    .media_exports
                    .get_mut(operation_id)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-authority"), "media close authority changed during one fixed step"))?
                    .close_step(maximum_items, maximum_bytes)?;
                if step == PluginCloseStep::Complete {
                    let active = self.media_exports.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-authority"), "completed media close lost its exact fixed owner"))?;
                    if !active.terminal_is_empty()? {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-terminal-not-empty"), "media close reported Complete without an exact terminal-empty witness"));
                    }
                    self.quarantine_media_snapshot(operation_id, active).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-snapshot-quarantine"), error.to_string()))?;
                    let active = self.media_exports.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-authority"), "terminal media owner changed before exact removal"))?;
                    drop(active);
                    self.close_media_cursor = self.close_media_cursor.saturating_add(1);
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                return Ok(step);
            }
            if self.close_media_cleanup_cursor < ARTIFACT_LIVE_OUTPUT_SLOTS {
                let Some(operation_id) = self.media_closures.id_at(self.close_media_cleanup_cursor) else {
                    let ceiling = self.close_media_cleanup_cursor.saturating_add(maximum_items).min(ARTIFACT_LIVE_OUTPUT_SLOTS);
                    let mut released_items = 0;
                    while self.close_media_cleanup_cursor < ceiling && self.media_closures.id_at(self.close_media_cleanup_cursor).is_none() {
                        self.close_media_cleanup_cursor = self.close_media_cleanup_cursor.saturating_add(1);
                        released_items += 1;
                    }
                    return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
                };
                let step = self
                    .media_closures
                    .get_mut(operation_id)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-cleanup-authority"), "media cleanup authority changed during one fixed step"))?
                    .close_step(maximum_items, maximum_bytes)?;
                if step == PluginCloseStep::Complete {
                    let active = self.media_closures.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-cleanup-authority"), "completed media cleanup lost its exact fixed owner"))?;
                    if !active.terminal_is_empty()? {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-terminal-not-empty"), "media cleanup reported Complete without an exact terminal-empty witness"));
                    }
                    self.quarantine_media_snapshot(operation_id, active).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-snapshot-quarantine"), error.to_string()))?;
                    let active = self.media_closures.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-media-cleanup-authority"), "terminal media cleanup changed before exact removal"))?;
                    drop(active);
                    self.close_media_cleanup_cursor = self.close_media_cleanup_cursor.saturating_add(1);
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                return Ok(step);
            }
            if self.close_segment_cursor < ARTIFACT_LIVE_OUTPUT_SLOTS {
                let Some(operation_id) = self.segmented_downloads.id_at(self.close_segment_cursor) else {
                    let ceiling = self.close_segment_cursor.saturating_add(maximum_items).min(ARTIFACT_LIVE_OUTPUT_SLOTS);
                    let mut released_items = 0;
                    while self.close_segment_cursor < ceiling && self.segmented_downloads.id_at(self.close_segment_cursor).is_none() {
                        self.close_segment_cursor = self.close_segment_cursor.saturating_add(1);
                        released_items += 1;
                    }
                    return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
                };
                if maximum_bytes < ARTIFACT_OUTPUT_CHUNK_BYTES {
                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                }
                let chunk = self
                    .segmented_downloads
                    .get(operation_id)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-authority"), "segmented close authority changed during one fixed step"))?
                    .chunks
                    .close_take_chunk()?;
                return match chunk {
                    Some(chunk) => Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: chunk.len() }),
                    None => {
                        let output =
                            self.segmented_downloads.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-authority"), "terminal segmented owner changed before exact removal"))?;
                        if !output.terminal_is_empty() {
                            self.segmented_downloads.insert_admitted(operation_id, output);
                            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-terminal-not-empty"), "segmented owner reached terminal without an empty exact chunk queue"));
                        }
                        drop(output);
                        self.close_segment_cursor = self.close_segment_cursor.saturating_add(1);
                        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                    }
                };
            }
            if self.close_segment_cleanup_cursor < ARTIFACT_LIVE_OUTPUT_SLOTS {
                let Some(operation_id) = self.segmented_closures.id_at(self.close_segment_cleanup_cursor) else {
                    let ceiling = self.close_segment_cleanup_cursor.saturating_add(maximum_items).min(ARTIFACT_LIVE_OUTPUT_SLOTS);
                    let mut released_items = 0;
                    while self.close_segment_cleanup_cursor < ceiling && self.segmented_closures.id_at(self.close_segment_cleanup_cursor).is_none() {
                        self.close_segment_cleanup_cursor = self.close_segment_cleanup_cursor.saturating_add(1);
                        released_items += 1;
                    }
                    return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
                };
                if maximum_bytes < ARTIFACT_OUTPUT_CHUNK_BYTES {
                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                }
                let chunk = self
                    .segmented_closures
                    .get(operation_id)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-cleanup-authority"), "segmented cleanup authority changed during one fixed step"))?
                    .chunks
                    .close_take_chunk()?;
                return match chunk {
                    Some(chunk) => Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: chunk.len() }),
                    None => {
                        let output = self
                            .segmented_closures
                            .remove(operation_id)
                            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-cleanup-authority"), "terminal segmented cleanup changed before exact removal"))?;
                        if !output.terminal_is_empty() {
                            self.segmented_closures.insert_admitted(operation_id, output);
                            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-segment-terminal-not-empty"), "segmented cleanup reached terminal without an empty exact chunk queue"));
                        }
                        drop(output);
                        self.close_segment_cleanup_cursor = self.close_segment_cleanup_cursor.saturating_add(1);
                        Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
                    }
                };
            }
            if self.close_snapshot_cursor < ARTIFACT_LIVE_OUTPUT_SLOTS {
                let Some(operation_id) = self.snapshot_retirements.id_at(self.close_snapshot_cursor) else {
                    let ceiling = self.close_snapshot_cursor.saturating_add(maximum_items).min(ARTIFACT_LIVE_OUTPUT_SLOTS);
                    let mut released_items = 0;
                    while self.close_snapshot_cursor < ceiling && self.snapshot_retirements.id_at(self.close_snapshot_cursor).is_none() {
                        self.close_snapshot_cursor = self.close_snapshot_cursor.saturating_add(1);
                        released_items += 1;
                    }
                    return Ok(PluginCloseStep::Pending { released_items, released_bytes: 0 });
                };
                let step = self
                    .snapshot_retirements
                    .get_mut(operation_id)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-snapshot-authority"), "snapshot retirement authority changed during one fixed step"))?
                    .close_step(maximum_items, maximum_bytes)?;
                if step == PluginCloseStep::Complete {
                    if !self.snapshot_retirements.get(operation_id).is_some_and(ArtifactSnapshotCloseRetention::terminal_is_empty) {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-snapshot-terminal-not-empty"), "snapshot retirement reported Complete without its exact terminal-empty witness"));
                    }
                    let retirement =
                        self.snapshot_retirements.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-snapshot-authority"), "terminal snapshot retirement changed before exact removal"))?;
                    drop(retirement);
                    self.close_snapshot_cursor = self.close_snapshot_cursor.saturating_add(1);
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                return Ok(step);
            }
            if !self.child_member_retirements.is_empty() {
                let step = self.child_member_retirement_step(maximum_items, maximum_bytes)?;
                if !matches!(step, PluginCloseStep::Blocked { .. }) || self.child_content_retirements.is_empty() {
                    return Ok(step);
                }
            }
            if !self.child_content_retirements.is_empty() {
                let Some((index, generation)) = self.child_content_retirements.next_id_from(self.close_child_root_cursor) else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-root-authority"), "child root retirement registry changed during one fixed close step"));
                };
                let step =
                    {
                        let retirements = &mut self.child_content_retirements;
                        let children = &mut self.children;
                        let retiring = &mut self.child_member_retirements;
                        let current = &*self.child_content_root;
                        let owners = retirements.sibling_content_owners(generation);
                        retirements
                            .get_mut(generation)
                            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-root-authority"), "child root retirement authority changed during one fixed close step"))?
                            .close_step(children, Some(retiring), current, &owners, maximum_items, maximum_bytes)?
                    };
                if step != PluginCloseStep::Complete {
                    self.close_child_root_cursor = index;
                    return Ok(step);
                }
                if !self.child_content_retirements.get(generation).is_some_and(ChildContentRetirement::terminal_is_empty) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-root-terminal-not-empty"), "child root retirement reported Complete without its exact terminal-empty witness"));
                }
                let retirement =
                    self.child_content_retirements.remove(generation).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-root-authority"), "terminal child root retirement changed before exact removal"))?;
                drop(retirement);
                self.close_child_root_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_child_root_detached {
                let retirement_generation = if self.child_content_root.root.as_ref().is_some_and(|root| root.len != 0) {
                    let generation = self
                        .child_content_generation
                        .checked_add(1)
                        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-root-generation"), "child root close generation exhausted before exact ownership transfer"))?;
                    if !self.child_content_retirements.can_insert(generation) {
                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    Some(generation)
                } else {
                    None
                };
                let previous = std::mem::replace(&mut *self.child_content_root, ChildContentView::EMPTY);
                if let Some(generation) = retirement_generation {
                    self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, true));
                    self.child_content_generation = generation;
                } else {
                    drop(previous);
                }
                self.close_child_root_detached = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some(retirement) = self.close_child_member.as_mut() {
                let step = retirement.close_step(maximum_items, maximum_bytes)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                if !retirement.terminal_is_empty() {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-member-terminal-not-empty"), "child member retirement reported Complete without its exact terminal-empty witness"));
                }
                drop(self.close_child_member.take());
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.children.is_empty() {
                if self.close_child_member_cursor >= CHILD_CONTENT_SLOTS {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-child-member-cursor"), "fixed child-member close cursor exhausted before every exact owner was detached"));
                }
                let index = self.close_child_member_cursor;
                self.close_child_member_cursor += 1;
                if let Some(entry) = self.children.take_at(index) {
                    *self.close_child_member = Some(ChildMemberRetirement::new(entry));
                }
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.peer_roster_publications.is_empty() {
                let Some((index, generation)) = self.peer_roster_publications.next_id_from(self.close_peer_roster_cursor) else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-roster-authority"), "peer roster publication registry changed during one fixed close step"));
                };
                let step = self
                    .peer_roster_publications
                    .get_mut(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-roster-authority"), "peer roster publication authority changed during one fixed close step"))?
                    .close_step(maximum_items, maximum_bytes)?;
                if step != PluginCloseStep::Complete {
                    self.close_peer_roster_cursor = index;
                    return Ok(step);
                }
                if !self.peer_roster_publications.get(generation).is_some_and(PeerRosterPublication::terminal_is_empty) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-roster-terminal-not-empty"), "peer roster publication reported Complete without its exact terminal-empty witness"));
                }
                let publication =
                    self.peer_roster_publications.remove(generation).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-roster-authority"), "terminal peer roster publication changed before exact removal"))?;
                drop(publication);
                let reservation_slot = Self::peer_roster_slot(generation);
                if self.peer_roster_reservations[reservation_slot].is_some_and(|(reserved, _)| reserved == generation) {
                    self.peer_roster_reservations[reservation_slot] = None;
                }
                self.close_peer_roster_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if let Some((index, generation)) = self.peer_roster_outcomes.next_id_from(self.close_peer_roster_cursor) {
                let outcome = self.peer_roster_outcomes.remove(generation).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-roster-outcome"), "peer roster outcome changed during exact close removal"))?;
                drop(outcome);
                self.close_peer_roster_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.peer_presence_retirements.is_empty() {
                let Some((index, generation)) = self.peer_presence_retirements.next_id_from(self.close_peer_presence_cursor) else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-presence-authority"), "peer-presence retirement registry changed during one fixed close step"));
                };
                let step = self
                    .peer_presence_retirements
                    .get_mut(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-presence-authority"), "peer-presence retirement authority changed during one fixed close step"))?
                    .close_step(maximum_items, maximum_bytes)?;
                if step != PluginCloseStep::Complete {
                    self.close_peer_presence_cursor = index;
                    return Ok(step);
                }
                if !self.peer_presence_retirements.get(generation).is_some_and(PeerPresenceRootRetirement::terminal_is_empty) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-presence-terminal-not-empty"), "peer-presence retirement reported Complete without its exact terminal-empty witness"));
                }
                let retirement = self
                    .peer_presence_retirements
                    .remove(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-presence-authority"), "terminal peer-presence retirement changed before exact removal"))?;
                drop(retirement);
                self.close_peer_presence_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.presence_peer_retirements.is_empty() {
                let Some((index, generation)) = self.presence_peer_retirements.next_id_from(self.close_presence_peer_cursor) else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-presence-peer-authority"), "app-typed presence retirement registry changed during one fixed close step"));
                };
                let step = self
                    .presence_peer_retirements
                    .get_mut(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-presence-peer-authority"), "app-typed presence retirement authority changed during one fixed close step"))?
                    .close_step(maximum_items, maximum_bytes)
                    .map_err(|error| plugin_sdk_fault(error.to_string()))?;
                match step {
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= maximum_items && released_bytes <= maximum_bytes => {
                        self.close_presence_peer_cursor = index;
                        return Ok(PluginCloseStep::Pending { released_items, released_bytes });
                    }
                    store::SnapshotRetirementStep::Pending { .. } => {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-presence-peer-over-budget"), "app-typed presence retirement exceeded its exact close grant"));
                    }
                    store::SnapshotRetirementStep::Blocked => {
                        self.close_presence_peer_cursor = index;
                        return Ok(PluginCloseStep::Blocked { reason: "app-typed presence retirement waits for its exact captured root" });
                    }
                    store::SnapshotRetirementStep::Complete => {}
                }
                if !self.presence_peer_retirements.get(generation).is_some_and(store::PresencePeersRetirement::terminal_is_empty) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-presence-peer-terminal-not-empty"), "app-typed presence retirement reported Complete without its exact terminal-empty witness"));
                }
                let retirement = self
                    .presence_peer_retirements
                    .remove(generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-presence-peer-authority"), "terminal app-typed presence retirement changed before exact removal"))?;
                drop(retirement);
                self.close_presence_peer_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_peer_presence_detached {
                let retirement_generation = if self.peer_presence.is_empty() {
                    None
                } else {
                    let generation = self
                        .peer_presence_generation
                        .checked_add(1)
                        .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.close-peer-presence-generation"), "peer-presence close generation exhausted before exact ownership transfer"))?;
                    if !self.peer_presence_retirements.can_insert(generation) {
                        return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    Some(generation)
                };
                let previous = std::mem::replace(&mut *self.peer_presence, PeerPresenceRoot::empty_shared());
                if let Some(generation) = retirement_generation {
                    self.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
                    self.peer_presence_generation = generation;
                } else {
                    drop(previous);
                }
                self.close_peer_presence_detached = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_envelope_ingress_drained {
                let step = self.drive_envelope_ingress(maximum_items, maximum_bytes, true)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                self.close_envelope_ingress_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_envelope_decode_jobs_drained {
                let stage = self.close_envelope_decode_stage;
                self.close_envelope_decode_stage = (stage + 1) % 3;
                let step = match stage {
                    0 => self.drive_envelope_decode_jobs(maximum_items, maximum_bytes, true)?,
                    1 => self.drive_envelope_field_decoder_returns(maximum_items, maximum_bytes, true)?,
                    2 => self.drive_envelope_completed_record_returns(maximum_items, maximum_bytes, true)?,
                    _ => unreachable!("fixed envelope close stage"),
                };
                if self.envelope_decode_jobs.is_empty() {
                    self.close_envelope_decode_jobs_drained = true;
                    return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                return Ok(step);
            }
            if !self.close_envelope_field_decoders_drained {
                let step = self.drive_envelope_field_decoder_returns(maximum_items, maximum_bytes, true)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                self.close_envelope_field_decoders_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_envelope_completed_records_drained {
                let step = self.drive_envelope_completed_record_returns(maximum_items, maximum_bytes, true)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                self.close_envelope_completed_records_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_store_replacement_jobs_drained {
                let step = self.drive_store_replacement_jobs(maximum_items, maximum_bytes, true)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                if !self.store_replacement_jobs.is_empty() {
                    return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
                }
                self.close_store_replacement_jobs_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.close_document_archive_loads_drained {
                let step = self.drive_document_archive_load_retirements(maximum_items, maximum_bytes, true)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                self.close_document_archive_loads_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let cache_step = self.close_projection_cache_step(maximum_items)?;
            if cache_step != PluginCloseStep::Complete {
                return Ok(cache_step);
            }
            if !self.close_snapshot_reads_drained {
                let pump = &mut self.document_snapshot_read_returns;
                let store = &mut self.store;
                let step = pump.drive(|| store.take_returned_snapshot_read_retirement().map_err(|error| error.into_fault()), maximum_items, maximum_bytes)?;
                if step != PluginCloseStep::Complete {
                    return Ok(step);
                }
                if !self.store.snapshot_read_leases_terminal_is_empty() {
                    return Ok(PluginCloseStep::Blocked { reason: "document snapshot read remains live outside the bounded return pump" });
                }
                self.close_snapshot_reads_drained = true;
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let window_retirement = self.retire_document_windows_step(maximum_items, maximum_bytes, true)?;
            if window_retirement != PluginCloseStep::Complete {
                return Ok(window_retirement);
            }
            let owned_step = match self.close_owned_stage {
                0 => drive_artifact_owned_disposer("document-store", &mut self.store, &mut self.close_document_disposer, maximum_items, maximum_bytes),
                1 => drive_artifact_owned_disposer("config-store", &mut self.config_store, &mut self.close_config_disposer, maximum_items, maximum_bytes),
                2 => drive_artifact_owned_disposer("draft-store", &mut self.draft_store, &mut self.close_draft_disposer, maximum_items, maximum_bytes),
                3 => drive_artifact_owned_disposer("presence-store", &mut self.presence_store, &mut self.close_presence_disposer, maximum_items, maximum_bytes),
                4 => drive_artifact_owned_disposer("transient-store", &mut self.transient_store, &mut self.close_transient_disposer, maximum_items, maximum_bytes),
                5 => self.window_config_store.close_step(maximum_items, maximum_bytes),
                6 => self.window_transient_store.close_step(maximum_items, maximum_bytes),
                7 => drive_artifact_owned_disposer("interaction-store", &mut self.interaction_store, &mut self.close_interaction_disposer, maximum_items, maximum_bytes),
                _ => return Ok(self.close_retained_fields_step(maximum_items, maximum_bytes)),
            }?;
            if owned_step == PluginCloseStep::Complete {
                self.close_owned_stage = self.close_owned_stage.saturating_add(1);
                return Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(owned_step)
        }

```

## maintenance

```rust
        /// 🌡️ Pressure beats fairness: a queue a quarter full is drained out of turn until it is
        /// under the mark again, and the stage cursor does not move — the rotation resumes where
        /// it stood once the burst is over.
        ///
        /// 🎡️ The rotation is FAIR, not lazy. Every stage still runs at most one bounded unit, but an
        /// EMPTY stage must not consume the whole call: it answers `Pending { 0, 0 }`, releases
        /// nothing, and the cursor moves on to the next stage inside the same call. Until this loop
        /// existed a caller that drove `maintenance_step` once per turn released about one owner per
        /// [`MAINTENANCE_STAGES`] turns, because 25 of every 26 turns landed on an idle stage —
        /// measured on the assembled `s.flow.flow@1/*#editor` surface as 2_051 items released in
        /// 100_000 close turns, 96_588 of which released nothing, so a real editor could not finish
        /// closing inside any committed turn budget. The grant is still respected exactly: the scan
        /// only continues past a stage that released NOTHING, so at most one stage in a call spends
        /// it. A `Blocked` stage no longer hides a later stage that can still progress — it is
        /// remembered and reported only if the whole rotation had nothing else to give.
        fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            LAST_MAINTENANCE_STAGE.store(u64::MAX, std::sync::atomic::Ordering::Relaxed);
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.command_prune_requested() || self.pending_command_prune.is_some() || !self.command_log.pruned_terminal_is_empty() {
                return self.advance_command_prune_step(maximum_items, maximum_bytes, false);
            }
            if let Some(operation) = self.next_closing_private_child_group() { return self.close_private_child_group_operation_step(operation,maximum_items,maximum_bytes); }
            if self.tool_operations.is_empty()
                && self.tool_cancellations.active_operation_count() == 0
                && self.latest_wins_commands.is_empty()
                && self.latest_wins_keys.map.as_ref().is_some_and(|map| map.is_empty())
                && self.latest_wins_keys.can_begin()
                && self.media_closures.is_empty()
                && self.segmented_closures.is_empty()
                && self.snapshot_retirements.is_empty()
                && self.child_content_retirements.is_empty()
                && self.child_member_retirements.is_empty()
                && self.peer_roster_publications.is_empty()
                && self.peer_presence_retirements.is_empty()
                && self.presence_peer_retirements.is_empty()
                && self.envelope_ingress.is_empty()
                && self.envelope_decode_jobs.is_empty()
                && self.store_replacement_jobs.is_empty()
                && self.document_archive_loads.is_empty()
                && self.envelope_field_decoder_retirements.is_empty()
                && self.envelope_field_decoders.next_returned_ticket().is_none()
                && self.envelope_completed_record_retirements.is_empty()
                && self.envelope_completed_records.terminal_is_empty()
                && self.store.maintenance_retirements_terminal_is_empty()
                && self.config_store.maintenance_retirements_terminal_is_empty()
                && self.draft_store.maintenance_retirements_terminal_is_empty()
                && self.window_config_store.maintenance_retirements_terminal_is_empty()
                && self.retired_window_transient_stores.is_empty()
                && self.presence_store.local_read_maintenance_is_idle()
                && !semio_framework_job::worker_job_retirements_are_parked()
                && self.live_runtime_instance_id.is_none_or(|instance_id| A::mounted_jobs_terminal_is_empty(instance_id))
            {
                return self.advance_snapshot_read_returns_one(maximum_bytes);
            }
            if self.store.maintenance_retirements_under_pressure() {
                LAST_MAINTENANCE_STAGE.store(u64::from(MAINTENANCE_DOCUMENT_DISPLACED_STAGE), std::sync::atomic::Ordering::Relaxed);
                return self.maintenance_document_displaced_step(maximum_items, maximum_bytes);
            }
            if self.config_store.maintenance_retirements_under_pressure() || self.draft_store.maintenance_retirements_under_pressure() || self.window_config_store.maintenance_retirements_under_pressure() {
                LAST_MAINTENANCE_STAGE.store(u64::from(MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE), std::sync::atomic::Ordering::Relaxed);
                return self.maintenance_config_lane_displaced_step(maximum_items, maximum_bytes);
            }
            if self.child_retirements_under_pressure() {
                for stage in [MAINTENANCE_CHILD_ROOT_STAGE, MAINTENANCE_CHILD_MEMBER_STAGE] {
                    LAST_MAINTENANCE_STAGE.store(u64::from(stage), std::sync::atomic::Ordering::Relaxed);
                    let step = self.maintenance_stage_step(stage, maximum_items, maximum_bytes)?;
                    if matches!(step, PluginCloseStep::Pending { released_items, released_bytes } if released_items > 0 || released_bytes > 0) {
                        return Ok(step);
                    }
                }
            }
            let mut unproductive = PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
            for _ in 0..MAINTENANCE_STAGES {
                let stage = self.maintenance_stage;
                LAST_MAINTENANCE_STAGE.store(stage as u64, std::sync::atomic::Ordering::Relaxed);
                self.maintenance_stage = (self.maintenance_stage + 1) % MAINTENANCE_STAGES;
                match self.maintenance_stage_step(stage, maximum_items, maximum_bytes)? {
                    PluginCloseStep::Pending { released_items: 0, released_bytes: 0 } => {}
                    PluginCloseStep::Complete => unproductive = PluginCloseStep::Complete,
                    answer => return Ok(answer),
                }
            }
            Ok(unproductive)
        }

```

