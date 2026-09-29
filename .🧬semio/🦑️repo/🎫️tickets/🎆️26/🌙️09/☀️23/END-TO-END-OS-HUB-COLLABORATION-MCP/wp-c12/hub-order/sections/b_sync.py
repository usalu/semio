# 🔄️ Section B: the native sync actors (native `ArtifactActor`, `WasmActor`) hand a hub frame to the store in the hub's order —
# other authors' operations as `Sequenced`, this replica's own settled ones as `Committed` — mark every decided batch
# `Committed` before any correction, and hand a hub document's remote operations over as `Sequenced` (a folder document's stay
# `Mutations`, merged by HLC). Rust twin of the worker's `hubOrderSegmentsV1`.

edit(SYNC, """/// 🔁️ Records the operations this replica authored, so their echo is never applied a second time.
pub fn note_authored_envelopes(""", """/// 🧭️ One run of a hub `Commands` frame, in the frame's order — which is the hub's order: `Committed` names this replica's
/// own operations the hub already holds (a lost ack's settle), `Sequenced` other authors' fresh operations. The store folds
/// every `Sequenced` run before its operations the hub has not decided yet, so a frame must reach it run by run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubOrderSegment {
    Committed(Vec<String>),
    Sequenced(Vec<String>),
}

/// 🧭️ Splits a hub frame's operation ids into its ordered runs: `settled` ids become `Committed` runs, `fresh` ids
/// `Sequenced` runs, every other id (an operation this replica already holds) is skipped. TS twin: `hubOrderSegmentsV1`
/// (worker); both replay the parity scenarios `hub-order-*` (`🔄️sync/⚖️parity/🧫️fixtures/🔣️.json`).
pub fn hub_order_segments(frame: &[String], settled: &std::collections::HashSet<String>, fresh: &std::collections::HashSet<String>) -> Vec<HubOrderSegment> {
    let mut segments: Vec<HubOrderSegment> = Vec::new();
    for id in frame {
        let committed = settled.contains(id);
        if !committed && !fresh.contains(id) {
            continue;
        }
        match (segments.last_mut(), committed) {
            (Some(HubOrderSegment::Committed(ids)), true) | (Some(HubOrderSegment::Sequenced(ids)), false) => ids.push(id.clone()),
            _ => segments.push(if committed { HubOrderSegment::Committed(vec![id.clone()]) } else { HubOrderSegment::Sequenced(vec![id.clone()]) }),
        }
    }
    segments
}

/// 🔁️ Records the operations this replica authored, so their echo is never applied a second time.
pub fn note_authored_envelopes(""", "sync hub order segments")

edit(SYNC, """        BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } | BackboneMessage::Member { .. } => Err("document backbone requires a canonical mutation message".into()),""",
     """        BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } | BackboneMessage::Member { .. } | BackboneMessage::Committed { .. } | BackboneMessage::Sequenced { .. } => Err("document backbone requires a canonical mutation message".into()),""", "sync document backbone decode refuses sequencer messages")

edit(SYNC, """                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let persisted = &self.known_op_ids;
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes.into_iter().filter(|envelope| !persisted.contains(&envelope.mutation_id.0)));
                    if !fresh.is_empty() {
                        self.persist_operations(&fresh).await;
                        if !self.deliver_remote_operations(fresh).await {
                            self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                            return;
                        }
                    }""", """                    let frame: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let persisted = &self.known_op_ids;
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes.into_iter().filter(|envelope| !persisted.contains(&envelope.mutation_id.0)));
                    if !fresh.is_empty() {
                        self.persist_operations(&fresh).await;
                    }
                    if !self.deliver_hub_order(&frame, &settled, fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                        return;
                    }""", "sync native actor forwards a frame in hub order")

edit(SYNC, """                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);
                    if !self.deliver_remote_operations(fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed");
                        return;
                    }""", """                    let frame: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    let settled = settle_committed_envelopes(&mut self.outbox, &mut self.pending_batches, &envelopes);
                    self.document_backbone_retention.release(&settled);
                    note_authored_envelopes(&mut self.applied_op_ids, &settled);
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);
                    if !self.deliver_hub_order(&frame, &settled, fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed");
                        return;
                    }""", "sync wasm actor forwards a frame in hub order")

HANDLE_ACK_OLD = """                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                self.document_backbone_retention.release(&sent);
                match *outcome {"""
HANDLE_ACK_NEW = """                let Some(sent) = self.pending_batches.remove(&batch_id) else { continue };
                self.document_backbone_retention.release(&sent);
                let _ = self.deliver_commit(sent.iter().map(|envelope| envelope.mutation_id.0.clone()).collect()).await;
                match *outcome {"""
text = read(SYNC)
if text is not None and text.count(HANDLE_ACK_OLD) == 2:
    files[SYNC] = text.replace(HANDLE_ACK_OLD, HANDLE_ACK_NEW)
    plan.append("edit    sync actors commit a decided batch before any correction")
elif text is not None and text.count(HANDLE_ACK_NEW) == 2 and HANDLE_ACK_OLD not in text.replace(HANDLE_ACK_NEW, ""):
    plan.append("present sync actors commit a decided batch before any correction")
else:
    problems.append(("sync actors commit a decided batch before any correction", None if text is None else text.count(HANDLE_ACK_OLD)))

edit(SYNC, """                BackboneMessage::Genesis { pack } => self.persist_genesis(pack).await,
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Member { .. } => return Err(vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),""",
     """                BackboneMessage::Genesis { pack } => self.persist_genesis(pack).await,
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Member { .. } => return Err(vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
                BackboneMessage::Committed { .. } | BackboneMessage::Sequenced { .. } => return Err(vcs::VcsError::Backbone("a store never publishes a sequencer's message; those flow from the hub to the store".into())),""", "sync native relay refuses a store's sequencer message")
edit(SYNC, """                BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } => {}
                BackboneMessage::Member { .. } => return Err(vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),""",
     """                BackboneMessage::Genesis { .. } | BackboneMessage::Ack { .. } => {}
                BackboneMessage::Member { .. } => return Err(vcs::VcsError::Backbone("a composed member requires its exact member transport lane".into())),
                BackboneMessage::Committed { .. } | BackboneMessage::Sequenced { .. } => return Err(vcs::VcsError::Backbone("a store never publishes a sequencer's message; those flow from the hub to the store".into())),""", "sync wasm relay refuses a store's sequencer message")

HUB_ORDER_FNS = """        /// 🧭️ Hands one hub frame to the store run by run, in the hub's order: other authors' fresh operations `Sequenced`,
        /// this replica's own settled ones `Committed` (see [`hub_order_segments`]).
        async fn deliver_hub_order(&mut self, frame: &[String], settled: &[MutationEnvelope], fresh: Vec<MutationEnvelope>) -> bool {
            let settled_ids: std::collections::HashSet<String> = settled.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
            let fresh_ids: std::collections::HashSet<String> = fresh.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
            let mut fresh: std::collections::HashMap<String, MutationEnvelope> = fresh.into_iter().map(|envelope| (envelope.mutation_id.0.clone(), envelope)).collect();
            for segment in hub_order_segments(frame, &settled_ids, &fresh_ids) {
                let delivered = match segment {
                    HubOrderSegment::Committed(op_ids) => self.deliver_commit(op_ids).await,
                    HubOrderSegment::Sequenced(op_ids) => self.deliver_remote_operations(op_ids.iter().filter_map(|id| fresh.remove(id)).collect()).await,
                };
                if !delivered {
                    return false;
                }
            }
            true
        }

"""
DELIVER_NATIVE_OLD = """        /// 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers.
        async fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            if self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).await.is_err() {
                return false;
            }
            if self.hub_space_id.is_some() {
                let message = match (BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).encode_op() {
                    Ok(message) => message,
                    Err(_) => return false,
                };
                self.emit(ArtifactEvent::DocumentBackbone { message });
            } else {
                self.emit(ArtifactEvent::RemoteMutations { envelopes });
            }
            true
        }
"""
DELIVER_NATIVE_NEW = HUB_ORDER_FNS + """        /// ✅️ Tells the store the hub decided `op_ids` of its own, on its inbound queue and — for a hosted guest store — on
        /// the document backbone. 🚫️`&self`: the actor is not `Sync`, so a shared borrow held across the push would make
        /// the actor turn non-`Send` (see `emit`).
        async fn deliver_commit(&mut self, op_ids: Vec<String>) -> bool {
            if op_ids.is_empty() {
                return true;
            }
            let message = BackboneMessage::Committed { op_ids };
            let Ok(hosted) = message.encode_op() else { return false };
            if self.remote.push(message).await.is_err() {
                return false;
            }
            self.emit(ArtifactEvent::DocumentBackbone { message: hosted });
            true
        }

        /// 🕸️ Pushes remote operations into the store's inbound queue and notifies subscribers: a hub document's in the
        /// hub's order (`Sequenced`, also on the document backbone a hosted guest store reads), a folder document's by
        /// their HLC (`Mutations`).
        async fn deliver_remote_operations(&mut self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            let hub = self.hub_space_id.is_some();
            let message = |envelopes: &[MutationEnvelope]| if hub { BackboneMessage::Sequenced { envelopes: encode_envelopes(envelopes) } } else { BackboneMessage::Mutations { envelopes: encode_envelopes(envelopes) } };
            if self.remote.push(message(&envelopes)).await.is_err() {
                return false;
            }
            if hub {
                let Ok(message) = message(&envelopes).encode_op() else { return false };
                self.emit(ArtifactEvent::DocumentBackbone { message });
            } else {
                self.emit(ArtifactEvent::RemoteMutations { envelopes });
            }
            true
        }
"""
edit(SYNC, DELIVER_NATIVE_OLD, DELIVER_NATIVE_NEW, "sync native actor hub-order delivery")

DELIVER_WASM_OLD = """        async fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            if self.remote.push(BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).await.is_err() {
                return false;
            }
            if self.hub_space_id.is_some() {
                let message = match (BackboneMessage::Mutations { envelopes: encode_envelopes(&envelopes) }).encode_op() {
                    Ok(message) => message,
                    Err(_) => return false,
                };
                let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
            } else {
                let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
            }
            true
        }
"""
DELIVER_WASM_NEW = HUB_ORDER_FNS + """        /// ✅️ Mirrors the native actor's `deliver_commit`.
        async fn deliver_commit(&self, op_ids: Vec<String>) -> bool {
            if op_ids.is_empty() {
                return true;
            }
            let message = BackboneMessage::Committed { op_ids };
            let Ok(hosted) = message.encode_op() else { return false };
            if self.remote.push(message).await.is_err() {
                return false;
            }
            let _ = self.events.send(ArtifactEvent::DocumentBackbone { message: hosted });
            true
        }

        /// 🕸️ Mirrors the native actor's `deliver_remote_operations`.
        async fn deliver_remote_operations(&self, envelopes: Vec<MutationEnvelope>) -> bool {
            if envelopes.is_empty() {
                return true;
            }
            let hub = self.hub_space_id.is_some();
            let message = |envelopes: &[MutationEnvelope]| if hub { BackboneMessage::Sequenced { envelopes: encode_envelopes(envelopes) } } else { BackboneMessage::Mutations { envelopes: encode_envelopes(envelopes) } };
            if self.remote.push(message(&envelopes)).await.is_err() {
                return false;
            }
            if hub {
                let Ok(message) = message(&envelopes).encode_op() else { return false };
                let _ = self.events.send(ArtifactEvent::DocumentBackbone { message });
            } else {
                let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes });
            }
            true
        }
"""
edit(SYNC, DELIVER_WASM_OLD, DELIVER_WASM_NEW, "sync wasm actor hub-order delivery")
