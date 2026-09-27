#!/usr/bin/env python3
"""WG10 s13 item 6 — PREPARED PATCH for landing window 3 (NOT applied; the tree is frozen after 15:45).

Native/wgpu twin of LD's React fix (📓️wp-ld.md item 3): a hub `RebootstrapRequired` (socket fan-out lag, GIS approval checkpoint)
used to leave a native document refused forever — both kernel actors (`🏪️store/🔄️sync` `on_hub_frame`, native + wasm32) waited for
a canonical pair INSIDE a `Welcome`, which no hub ever sends (`db.hello` answers None/Tail; pairs are served only by
`GET …/active-checkpoint/pair`), and every reconnect paid the outage backoff first.

One mechanism with the open path (item 1's canonical-pair seed):
  1. actor: on `RebootstrapRequired` it drops its projection and socket, keeps its unacked work, emits
     `ArtifactEvent::RebootstrapRequired { control }` and does NOT dial until the host re-seeds it (no backoff, no refused
     Welcome loop; the dead "pair inside the Welcome" arms are deleted);
  2. shell (both targets): fetches the pair through `DirectoryClient::rebootstrap_canonical_checkpoint_pair` — decoded, digests
     verified, admitted as exactly the control's checkpoint (`CanonicalCheckpointPairV1::admit_rebootstrap`) — loads it into the
     guest and sends `ArtifactActorMsg::Reseed { pack, spr, baseline }`; a transient failure is asked again after 1 s while the
     document stays open;
  3. actor: adopts the pair (known ops, pack/spr/archive, folder persisted, `server_frontier` = baseline), hands its unacked local
     operations back to the guest (`RemoteMutations`, minus those the pair already holds) and dials at once: its `SocketHelloV1`
     names the baseline, so the hub's None/Tail Welcome is accepted like a first open.

Laws (written into the patch): fixture `rebootstrapAdmissions` (Rust schema law + TS twin runner), kernel client rebootstrap fetch,
native actor `a_rebootstrap_waits_for_the_hosts_reseed_and_says_hello_at_its_baseline`. Red before: the old actor emits no event and
refuses the None/Tail Welcome (`artifact rebootstrap returned tail without a canonical pair`).

Usage: python3 patch-rebootstrap-reseed.py            (dry run: every anchor must match exactly once; nothing written)
       python3 patch-rebootstrap-reseed.py --apply    (writes; then native `--lib --tests` os-kernel/renderer + wasm32 checks)
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
M = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules"
FILES = {
    "pair": M / "📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs",
    "pair_law": M / "📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1/🧪️tests/🔬️unit/🦀️.rs",
    "client_pair": M / "📇️directory/🔌️client/🪢️canonical-checkpoint-pair/🦀️.rs",
    "client_law": M / "📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs",
    "sync": M / "🏪️store/🔄️sync/🦀️.rs",
    "sync_law": M / "🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs",
    "shell": M / "📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
    "ts_schema": M / "📇️directory/🧬️schema/🟦️.ts",
    "ts_law": ROOT / "🧰️framework/🛍️products/💻️os/🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts",
    "json_schema": ROOT / "🧰️framework/🛍️products/💻️os/🧬️schema/🪢️canonical-checkpoint-pair-v1/🔣️.json",
    "parity_law": M / "🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🦀️.rs",
}
TEXT = {key: path.read_text() for key, path in FILES.items()}
PROBLEMS = []


def swap(key, old, new):
    count = TEXT[key].count(old)
    if count != 1:
        PROBLEMS.append(f"{key}: anchor matched {count}× — {old[:90]!r}")
        return
    TEXT[key] = TEXT[key].replace(old, new)


def append(key, addition):
    TEXT[key] = TEXT[key].rstrip("\n") + "\n" + addition


# ── 1. kernel schema: admission against a rebootstrap control ─────────────────────────────────────────────────────────────
swap("pair", "use super::{ArtifactFrontier, ArtifactHash, CheckpointId, DocumentOpenCheckpointV1, DocumentScope, PublishedArtifactBlob, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};",
     "use super::{ArtifactFrontier, ArtifactHash, CheckpointId, DocumentOpenCheckpointV1, DocumentScope, PublishedArtifactBlob, RebootstrapRequired, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};")
swap("pair", """        if self.aggregate_sha256.hex() != expected.aggregate_sha256 {
            return Err(CanonicalCheckpointPairRefusalV1::Aggregate);
        }
        Ok(())
    }
}
""", """        if self.aggregate_sha256.hex() != expected.aggregate_sha256 {
            return Err(CanonicalCheckpointPairRefusalV1::Aggregate);
        }
        Ok(())
    }

    /// 🛟️ Admits the pair only as exactly the checkpoint a hub `RebootstrapRequired` control names (scope, checkpoint id,
    /// descriptor digest, baseline frontier) — the control carries no aggregate, which the decoder already proved against the
    /// pair's own bytes.
    pub fn admit_rebootstrap(&self, control: &RebootstrapRequired) -> Result<(), CanonicalCheckpointPairRefusalV1> {
        if self.scope != control.scope {
            return Err(CanonicalCheckpointPairRefusalV1::Scope);
        }
        if self.active_checkpoint_id != control.checkpoint_id {
            return Err(CanonicalCheckpointPairRefusalV1::Checkpoint);
        }
        if self.descriptor_digest_v1 != control.descriptor_digest_v1 {
            return Err(CanonicalCheckpointPairRefusalV1::Descriptor);
        }
        if self.baseline_frontier != control.baseline_frontier {
            return Err(CanonicalCheckpointPairRefusalV1::Baseline);
        }
        Ok(())
    }
}
""")
append("pair_law", '''
#[test]
fn a_pair_is_admitted_only_as_the_checkpoint_a_rebootstrap_control_names() {
    let fixture = fixture();
    for case in fixture["rebootstrapAdmissions"].as_array().expect("rebootstrap admissions") {
        let pair = fixture["pairs"].as_array().unwrap().iter().find(|pair| pair["id"] == case["pair"]).expect("admission pair");
        let decoded = decode_canonical_checkpoint_pair_v1(&hex_bytes(&pair["bodyHex"])).expect("fixture pair decodes");
        let control = RebootstrapRequired {
            scope: DocumentScope::new(case["control"]["scope"]["spaceId"].as_str().unwrap(), case["control"]["scope"]["documentId"].as_str().unwrap()),
            checkpoint_id: hash(&case["control"]["checkpointId"]),
            descriptor_digest_v1: hash(&case["control"]["descriptorDigestV1"]),
            baseline_frontier: frontier(&case["control"]["baselineFrontier"]),
        };
        assert_eq!(decoded.admit_rebootstrap(&control).map_err(CanonicalCheckpointPairRefusalV1::code), case["refusal"].as_str().map_or(Ok(()), Err), "{}", case["id"]);
    }
}
''')

# ── 2. kernel client: one fetch, two admissions ───────────────────────────────────────────────────────────────────────────
swap("client_pair", "use super::super::schema::{decode_canonical_checkpoint_pair_v1, CanonicalCheckpointPairV1, DocumentOpenCheckpointV1, DocumentScope, CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1};",
     "use super::super::schema::{decode_canonical_checkpoint_pair_v1, CanonicalCheckpointPairV1, DocumentOpenCheckpointV1, DocumentScope, RebootstrapRequired, CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1};")
swap("client_pair", """    pub async fn document_canonical_checkpoint_pair(&self, ctx: &OperationContext, scope: &DocumentScope, expected: &DocumentOpenCheckpointV1) -> Result<CanonicalCheckpointPairV1, DirectoryClientError> {
        let url = self.url(&canonical_checkpoint_pair_path(scope));""", """    pub async fn document_canonical_checkpoint_pair(&self, ctx: &OperationContext, scope: &DocumentScope, expected: &DocumentOpenCheckpointV1) -> Result<CanonicalCheckpointPairV1, DirectoryClientError> {
        let pair = self.fetch_canonical_checkpoint_pair(ctx, scope).await?;
        pair.admit(scope, expected).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        Ok(pair)
    }

    /// 🛟️ Fetches and verifies the document's active pair for a hub `RebootstrapRequired`, admitted only as exactly the
    /// checkpoint the control names — the one seed a rebuilding document restarts from, as on its first open.
    pub async fn rebootstrap_canonical_checkpoint_pair(&self, ctx: &OperationContext, control: &RebootstrapRequired) -> Result<CanonicalCheckpointPairV1, DirectoryClientError> {
        let pair = self.fetch_canonical_checkpoint_pair(ctx, &control.scope).await?;
        pair.admit_rebootstrap(control).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        Ok(pair)
    }

    /// 🪢️ The route read shared by both admissions: bounded, transient refusals asked again, decoded and digest-verified.
    async fn fetch_canonical_checkpoint_pair(&self, ctx: &OperationContext, scope: &DocumentScope) -> Result<CanonicalCheckpointPairV1, DirectoryClientError> {
        let url = self.url(&canonical_checkpoint_pair_path(scope));""")
swap("client_pair", """        let pair = decode_canonical_checkpoint_pair_v1(&response.body).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        pair.admit(scope, expected).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        Ok(pair)
    }""", """        decode_canonical_checkpoint_pair_v1(&response.body).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))
    }""")
append("client_law", '''
/// 🛟️ A rebootstrap fetch reads the same route and admits the pair only as the control's checkpoint.
#[semio_framework_async_macros::async_test]
async fn a_rebootstrap_pair_is_admitted_only_as_the_controls_checkpoint() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json")).expect("canonical pair fixture");
    let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
    for case in fixture["rebootstrapAdmissions"].as_array().unwrap() {
        let pair = fixture["pairs"].as_array().unwrap().iter().find(|pair| pair["id"] == case["pair"]).unwrap();
        let text = pair["bodyHex"].as_str().unwrap();
        let body: Vec<u8> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap()).collect();
        let raw = &case["control"];
        let hash = |value: &serde_json::Value| crate::os_directory::ArtifactHash::parse_hex(value.as_str().unwrap()).unwrap();
        let control = crate::os_directory::RebootstrapRequired {
            scope: DocumentScope::new(raw["scope"]["spaceId"].as_str().unwrap(), raw["scope"]["documentId"].as_str().unwrap()),
            checkpoint_id: hash(&raw["checkpointId"]),
            descriptor_digest_v1: hash(&raw["descriptorDigestV1"]),
            baseline_frontier: crate::os_pack::json::from_json_str(&raw["baselineFrontier"].to_string()).expect("fixture baseline"),
        };
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 200, body })).await;
        let answer = authenticated_client(transport.clone(), &capability).rebootstrap_canonical_checkpoint_pair(&root_ctx(), &control).await;
        match case["refusal"].as_str() {
            None => assert!(answer.is_ok(), "{}: {answer:?}", case["id"]),
            Some(code) => assert!(matches!(&answer, Err(DirectoryClientError::Decode(detail)) if detail == code), "{}: {answer:?}", case["id"]),
        }
        assert_eq!(transport.requests.lock().unwrap()[0].url, format!("http://hub.local{}", canonical_checkpoint_pair_path(&control.scope)));
    }
}
''')

# ── 3. kernel sync: the event, the reseed message, both actors ────────────────────────────────────────────────────────────
swap("sync", """    /// @emoji 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,""", """    /// 🛟️ The host re-seeded a rebootstrapping hub document ([`ArtifactEvent::RebootstrapRequired`]): the canonical
    /// checkpoint pair it verified against the control and already loaded into the guest. The actor adopts it as its baseline,
    /// hands its unacked local operations back to the guest and dials at once, saying Hello at the pair's baseline.
    Reseed { pack: Vec<u8>, spr: Vec<u8>, baseline: crate::os_directory::ArtifactFrontier },
    /// @emoji 🔄️ Forces an immediate re-read + diff of the folder binding (test/manual poke hook).
    ExternalChanged,""")
swap("sync", """        ArtifactActorMsg::PublishPreview { key, seq: _, payload } => {
            text(&mut bytes, key)?;
            add(&mut bytes, 8)?;
            add(&mut bytes, field(payload.len())?)?;
        }
        ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
    }
    (bytes <= ARTIFACT_MAILBOX_BYTES).then_some(bytes)""", """        ArtifactActorMsg::PublishPreview { key, seq: _, payload } => {
            text(&mut bytes, key)?;
            add(&mut bytes, 8)?;
            add(&mut bytes, field(payload.len())?)?;
        }
        ArtifactActorMsg::Reseed { pack, spr, baseline } => {
            add(&mut bytes, field(pack.len())?)?;
            add(&mut bytes, field(spr.len())?)?;
            text(&mut bytes, &baseline.document_id)?;
            text(&mut bytes, &baseline.head_edit_id)?;
            add(&mut bytes, 48)?;
        }
        ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}
    }
    (bytes <= ARTIFACT_MAILBOX_BYTES).then_some(bytes)""")
swap("sync", """    /// @emoji 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },""", """    /// @emoji 📡️ The presence roster changed.
    Presence { peers: Vec<PresencePeer> },
    /// 🛟️ The hub made this document rebuild from its canonical checkpoint (fan-out lag, an approval checkpoint): the actor
    /// dropped its projection and socket, keeps its unacked work and waits for the host's [`ArtifactActorMsg::Reseed`] with the
    /// pair `control` names — the pair only the host's directory client fetches and only the host can load into the guest.
    RebootstrapRequired { control: crate::os_directory::RebootstrapRequired },""")
swap("sync", """/// @emoji 👋️ The client-first frame every document socket opens with.""", """/// 🛟️ A wire `RebootstrapRequired` control as the directory contract names it (its frontier already projected onto the
/// public document id by the hub).
fn rebootstrap_control(control: &crate::os_spr::RebootstrapRequired) -> crate::os_directory::RebootstrapRequired {
    let frontier = &control.baseline_frontier;
    crate::os_directory::RebootstrapRequired {
        scope: crate::os_directory::DocumentScope::new(control.space_id.as_str(), control.document_id.as_str()),
        checkpoint_id: crate::os_directory::ArtifactHash(control.checkpoint_id),
        descriptor_digest_v1: crate::os_directory::ArtifactHash(control.descriptor_hash),
        baseline_frontier: crate::os_directory::ArtifactFrontier { document_id: frontier.document_id.0.clone(), head_edit_ordinal: frontier.head_edit_ordinal, head_edit_id: frontier.head_edit_id.clone(), last_commit_seq: frontier.last_commit_seq, chain_hash: crate::os_directory::ArtifactHash(frontier.chain_hash) },
    }
}

/// @emoji 👋️ The client-first frame every document socket opens with.""")

# native actor: the seed install shared by the open seed and the reseed
swap("sync", """        pub(super) async fn seed_hub_document(&mut self, pair: crate::os_directory::CanonicalCheckpointPairV1) {
            let seeded = match spr_op_ids(&pair.spr_bytes).await {
                Ok(op_ids) => crate::os_spr::encode_document_archive_bytes(&crate::os_spr::DocumentArchivePack { parent_pack: pair.pack_bytes.clone(), parent_spr: pair.spr_bytes.clone(), members: Vec::new() }).map(|archive| (op_ids, archive)).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };""", """        pub(super) async fn seed_hub_document(&mut self, pair: crate::os_directory::CanonicalCheckpointPairV1) {
            self.install_hub_seed(pair.pack_bytes, pair.spr_bytes, &pair.baseline_frontier).await;
        }

        /// 🛟️ Restarts a rebootstrapping hub document at the pair the host verified and loaded into the guest
        /// ([`ArtifactActorMsg::Reseed`]): the pair becomes the baseline, the unacked local operations the pair does not already
        /// hold go back to the guest, and the actor dials at once — no outage backoff, Hello at the baseline.
        async fn reseed_hub_document(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: crate::os_directory::ArtifactFrontier) {
            if !self.artifact_rebootstrap_required {
                return;
            }
            if !self.install_hub_seed(pack, spr, &baseline).await {
                return;
            }
            let known = &self.known_op_ids;
            self.outbox.retain(|envelope| !known.contains(&envelope.mutation_id.0));
            if !self.outbox.is_empty() {
                self.emit(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
            }
            self.artifact_rebootstrap_required = false;
            self.reconnect_at = None;
            self.start_connect_hub().await;
        }

        /// 🪢️ Adopts one verified pair as this document's baseline; `false` (and one conflict) when its SPR cannot be read.
        async fn install_hub_seed(&mut self, pack: Vec<u8>, spr: Vec<u8>, baseline: &crate::os_directory::ArtifactFrontier) -> bool {
            let seeded = match spr_op_ids(&spr).await {
                Ok(op_ids) => crate::os_spr::encode_document_archive_bytes(&crate::os_spr::DocumentArchivePack { parent_pack: pack.clone(), parent_spr: spr.clone(), members: Vec::new() }).map(|archive| (op_ids, archive)).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };""")
swap("sync", """                        message: format!("the canonical checkpoint pair could not seed the document: {error}"),
                        target: vec![self.document_id.clone()],
                        op_index: None,
                    }));
                    return;
                }
            };""", """                        message: format!("the canonical checkpoint pair could not seed the document: {error}"),
                        target: vec![self.document_id.clone()],
                        op_index: None,
                    }));
                    return false;
                }
            };""")
swap("sync", """            self.known_op_ids = op_ids;
            self.server_frontier = Some(canonical_pair_baseline(&pair.baseline_frontier));
            self.current_pack = Some(pair.pack_bytes);
            self.current_spr = Some(pair.spr_bytes);
            self.current_archive = Some(archive);
        }""", """            self.known_op_ids = op_ids;
            self.server_frontier = Some(canonical_pair_baseline(baseline));
            self.current_pack = Some(pack);
            self.current_spr = Some(spr);
            self.current_archive = Some(archive);
            true
        }""")
swap("sync", """        /// @emoji ♻️ Hub lag forces a canonical pair refresh: clear the live projection tokens, keep
        /// unacked work queued, and reconnect — mirrors the browser worker's `requireArtifactRebootstrap`.
        async fn require_artifact_rebootstrap(&mut self) {""", """        /// @emoji ♻️ Hub lag forces a canonical pair refresh: clear the live projection tokens, keep unacked work queued,
        /// drop the socket and ask the host for the pair ([`ArtifactEvent::RebootstrapRequired`]); the actor dials again only
        /// once the host re-seeded it ([`Self::reseed_hub_document`]) — never into a Welcome it would have to refuse, and never
        /// after an outage backoff (the rebuild's own close is not a shortage).
        async fn require_artifact_rebootstrap(&mut self, control: crate::os_directory::RebootstrapRequired) {""")
swap("sync", """            self.artifact_rebootstrap_required = true;
            self.known_op_ids.clear();
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.set_remote_state(RemoteState::Connecting).await;
            self.fail_link().await;
        }""", """            self.artifact_rebootstrap_required = true;
            self.known_op_ids.clear();
            self.semio_hub = None;
            self.clear_socket_epoch();
            self.reconnect_at = None;
            self.set_remote_state(RemoteState::Connecting).await;
            self.emit(ArtifactEvent::RebootstrapRequired { control });
        }""")
swap("sync", """            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.semio_hub.is_some() || self.connect_future.is_some() || !self.link.admits_local_edits() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {""",
     """            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.artifact_rebootstrap_required || self.semio_hub.is_some() || self.connect_future.is_some() || !self.link.admits_local_edits() || self.reconnect_at.is_some_and(|deadline| deadline > Instant::now()) {""")
swap("sync", """                        Bootstrap::None => {
                            if self.artifact_rebootstrap_required {
                                self.fail_artifact_bootstrap("artifact rebootstrap returned no canonical pair").await;
                                return;
                            }
                            self.abort_artifact_bootstrap();
                            self.resume_token = Some(resume_token);
                            self.server_frontier = Some(server_frontier);
                            self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
                            self.flush_outbox().await;
                        }
                        Bootstrap::Tail => {
                            if self.artifact_rebootstrap_required {
                                self.fail_artifact_bootstrap("artifact rebootstrap returned tail without a canonical pair").await;
                                return;
                            }
                            self.abort_artifact_bootstrap();""", """                        Bootstrap::None => {
                            self.abort_artifact_bootstrap();
                            self.resume_token = Some(resume_token);
                            self.server_frontier = Some(server_frontier);
                            self.set_remote_state(RemoteState::Live { peer_count: 0 }).await;
                            self.flush_outbox().await;
                        }
                        Bootstrap::Tail => {
                            self.abort_artifact_bootstrap();""")
swap("sync", """                        Bootstrap::ArtifactBootstrap(bootstrap) => {
                            self.artifact_rebootstrap_required = false;
                            self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                    self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client").await;
                }
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.fail_artifact_bootstrap("rebootstrap control scope mismatch").await;
                    } else {
                        self.require_artifact_rebootstrap().await;
                    }
                }""", """                        Bootstrap::ArtifactBootstrap(bootstrap) => {
                            self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => {
                    self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client").await;
                }
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.fail_artifact_bootstrap("rebootstrap control scope mismatch").await;
                    } else {
                        self.require_artifact_rebootstrap(rebootstrap_control(&control)).await;
                    }
                }""")
swap("sync", """                ArtifactActorMsg::ExternalChanged => {
                    self.handle_external_change().await;
                    false
                }
                ArtifactActorMsg::Detach => true,""", """                ArtifactActorMsg::ExternalChanged => {
                    self.handle_external_change().await;
                    false
                }
                ArtifactActorMsg::Reseed { pack, spr, baseline } => {
                    self.reseed_hub_document(pack, spr, baseline).await;
                    false
                }
                ArtifactActorMsg::Detach => true,""")

# wasm actor twin
swap("sync", """        /// ♻️ Hub lag forces a canonical pair refresh: keep unacked work queued, clear the projection
        /// tokens and reconnect — the native `require_artifact_rebootstrap`.
        fn require_artifact_rebootstrap(&mut self) {
            self.requeue_pending_batches();
            self.abort_artifact_bootstrap();
            self.server_frontier = None;
            self.resume_token = None;
            self.artifact_rebootstrap_required = true;
            self.close_socket();
            self.schedule_reconnect();
        }""", """        /// ♻️ Hub lag forces a canonical pair refresh: keep unacked work queued, clear the projection tokens, drop the socket
        /// and ask the host for the pair — the native `require_artifact_rebootstrap`; no dial until the host's reseed.
        fn require_artifact_rebootstrap(&mut self, control: crate::os_directory::RebootstrapRequired) {
            self.requeue_pending_batches();
            self.abort_artifact_bootstrap();
            self.server_frontier = None;
            self.resume_token = None;
            self.artifact_rebootstrap_required = true;
            self.close_socket();
            self.set_remote_state(RemoteState::Connecting);
            let _ = self.events.send(ArtifactEvent::RebootstrapRequired { control });
        }

        /// 🛟️ The browser twin of the native reseed: the pair's baseline is the next Hello's frontier, the unacked local
        /// operations go back to the guest, and the next turn dials at once.
        fn reseed_hub_document(&mut self, baseline: crate::os_directory::ArtifactFrontier) {
            if !self.artifact_rebootstrap_required {
                return;
            }
            self.server_frontier = Some(canonical_pair_baseline(&baseline));
            if !self.outbox.is_empty() {
                let _ = self.events.send(ArtifactEvent::RemoteMutations { envelopes: self.outbox.clone() });
            }
            self.artifact_rebootstrap_required = false;
        }""")
swap("sync", """            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.socket.is_some() || self.operation_cancel.is_cancelled_now() || !self.link.retry_due(wall_ms()) {""",
     """            let Some(base_url) = self.hub_base_url.clone() else { return };
            if self.artifact_rebootstrap_required || self.socket.is_some() || self.operation_cancel.is_cancelled_now() || !self.link.retry_due(wall_ms()) {""")
swap("sync", """                        Bootstrap::None => {
                            if self.artifact_rebootstrap_required {
                                self.fail_artifact_bootstrap("artifact rebootstrap returned no canonical pair");
                                return;
                            }
                            self.abort_artifact_bootstrap();""", """                        Bootstrap::None => {
                            self.abort_artifact_bootstrap();""")
swap("sync", """                        Bootstrap::Tail => {
                            if self.artifact_rebootstrap_required {
                                self.fail_artifact_bootstrap("artifact rebootstrap returned tail without a canonical pair");
                                return;
                            }
                            self.abort_artifact_bootstrap();""", """                        Bootstrap::Tail => {
                            self.abort_artifact_bootstrap();""")
swap("sync", """                        Bootstrap::ArtifactBootstrap(bootstrap) => {
                            self.artifact_rebootstrap_required = false;
                            self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client"),
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.fail_artifact_bootstrap("rebootstrap control scope mismatch");
                    } else {
                        self.require_artifact_rebootstrap();
                    }
                }""", """                        Bootstrap::ArtifactBootstrap(bootstrap) => {
                            self.start_artifact_bootstrap(*bootstrap, resume_token, server_frontier).await;
                        }
                    }
                }
                ServerFrame::SnapshotChunk { .. } | ServerFrame::SnapshotDone { .. } => self.fail_artifact_bootstrap("database-private snapshot frame cannot seed an artifact client"),
                ServerFrame::RebootstrapRequired { control } => {
                    if control.document_id != self.document_id || self.hub_space_id.as_deref() != Some(control.space_id.as_str()) || control.baseline_frontier.document_id.0 != self.document_id {
                        self.fail_artifact_bootstrap("rebootstrap control scope mismatch");
                    } else {
                        self.require_artifact_rebootstrap(rebootstrap_control(&control));
                    }
                }""")
swap("sync", """                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                }
                ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}""", """                ArtifactActorMsg::PublishPreview { key, seq, payload } => {
                    self.send_frame(&ClientFrame::PreviewPublish { key, seq, payload }, Lane::Preview).await;
                }
                ArtifactActorMsg::Reseed { baseline, .. } => self.reseed_hub_document(baseline),
                ArtifactActorMsg::ExternalChanged | ArtifactActorMsg::Detach => {}""")

# native law
append("sync_law", '''
/// 🛟️ A hub `RebootstrapRequired` never strands a native document again (LD item 3, WG10 item 6): the actor asks its host for
/// the pair and does not dial into a Welcome it would have to refuse; the host's reseed restarts it at the pair — unacked local
/// work handed back to the guest — and its next Hello names the pair's baseline, so the hub's None/Tail Welcome goes Live.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn a_rebootstrap_waits_for_the_hosts_reseed_and_says_hello_at_its_baseline() {
    let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let baseline = bootstrap.baseline_frontier.clone();
    let (_, remote) = ChannelBackbone::pair("native-rebootstrap-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, mut event_rx) = broadcast::channel(32);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { document_id: "demo".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url: "http://hub.local".into(), space_id: "space-reseed".into(), surface: None }], watch_external: false, actor: "rebootstrap-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let local = sample_operation_envelope("unacked-local", 5).await;
    actor.queue_test_outbox(vec![local.clone()]);
    let control = crate::os_spr::RebootstrapRequired { space_id: "space-reseed".into(), document_id: "demo".into(), checkpoint_id: [0x22; 32], descriptor_hash: [0x11; 32], baseline_frontier: baseline.clone() };
    actor.inject_hub_frame(ServerFrame::RebootstrapRequired { control }).await;
    let mut asked = None;
    while let Ok(event) = event_rx.try_recv() {
        if let ArtifactEvent::RebootstrapRequired { control } = event {
            asked = Some(control);
        }
    }
    let asked = asked.expect("the actor asks its host for the pair instead of waiting for it inside a Welcome");
    assert_eq!((asked.scope.document_id.as_str(), asked.checkpoint_id.0, asked.baseline_frontier.head_edit_ordinal), ("demo", [0x22; 32], baseline.head_edit_ordinal));
    assert!(actor.bootstrap_test_state().8, "the actor waits for its host's reseed");
    let baseline_frontier = crate::os_directory::ArtifactFrontier { document_id: "demo".into(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: crate::os_directory::ArtifactHash(baseline.chain_hash) };
    let _ = actor.handle_test_cmd(ArtifactActorMsg::Reseed { pack: pair.pack.clone(), spr: pair.spr.clone(), baseline: baseline_frontier }).await;
    let mut replayed = Vec::new();
    while let Ok(event) = event_rx.try_recv() {
        if let ArtifactEvent::RemoteMutations { envelopes } = event {
            replayed.extend(envelopes.into_iter().map(|envelope| envelope.mutation_id.0));
        }
    }
    assert_eq!(replayed, vec![local.mutation_id.0.clone()], "the unacked local operation goes back to the re-seeded guest");
    let (pack, spr, frontier, _, resume, _, _, outbox, rebootstrap) = actor.bootstrap_test_state();
    assert_eq!((pack.as_deref(), spr.as_deref(), frontier, resume, rebootstrap), (Some(pair.pack.as_slice()), Some(pair.spr.as_slice()), Some(baseline.clone()), None, false));
    assert_eq!(outbox, vec![local.mutation_id.0.clone()], "and stays queued for the hub");
    let mut socket = connect(&mut actor, "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").await;
    let ClientFrame::SocketHelloV1 { frontier, .. } = receive_frame(&mut socket).await else { panic!("the first frame is the hello") };
    assert_eq!(frontier, Some(baseline.clone()), "the rebuilt socket says Hello at the pair's baseline");
    actor.inject_hub_frame(ServerFrame::Welcome { session_id: "session-reseed".into(), resume_token: "resume-reseed".into(), server_frontier: baseline.clone(), bootstrap: Bootstrap::None }).await;
    assert!(matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "the hub's None Welcome is accepted like a first open");
}
''')
swap("sync", """        #[cfg(test)]
        pub(super) async fn inject_hub_frame(&mut self, frame: ServerFrame) {
            self.on_hub_frame(frame).await;
        }""", """        #[cfg(test)]
        pub(super) async fn inject_hub_frame(&mut self, frame: ServerFrame) {
            self.on_hub_frame(frame).await;
        }

        #[cfg(test)]
        pub(super) async fn handle_test_cmd(&mut self, message: ArtifactActorMsg) -> bool {
            self.handle_cmd(message).await
        }""")

swap("parity_law", """        ArtifactEvent::Presence { .. } => "presence",""", """        ArtifactEvent::Presence { .. } => "presence",
        ArtifactEvent::RebootstrapRequired { .. } => "rebootstrapRequired",""")

# ── 4. shell (both targets): fetch, verify, load, reseed ──────────────────────────────────────────────────────────────────
swap("shell", """    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,""", """    pub sync_bootstrap_progress: Option<(u64, u64, u32, u32)>,
    /// 🛟️ The canonical-pair reseed a rebootstrapping hub document is waiting for ([`ShellState::advance_sync_reseed`]).
    sync_reseed: Option<ShellSyncReseed>,""")
swap("shell", """            sync_bootstrap_progress: None,""", """            sync_bootstrap_progress: None,
            sync_reseed: None,""")
swap("shell", """/// 🪢️ What a hub document's seed step asks:""", """/// 🛟️ How long a rebootstrapping document waits before asking the hub for its pair again after a transient refusal.
const SYNC_RESEED_RETRY_MS: f64 = 1_000.0;

/// 🛟️ One rebootstrap reseed: the control the actor raised, the pair fetch in flight, and when to ask again after a failure.
struct ShellSyncReseed {
    control: semio_framework_os_kernel::os_directory::RebootstrapRequired,
    pending: Option<ShellDetached<Result<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1, String>>>,
    retry_at_ms: f64,
}

/// 🪢️ What a hub document's seed step asks:""")
swap("shell", """        self.sync_status = None;
        self.sync_bootstrap_progress = None;
        self.presence_peers.clear();""", """        self.sync_status = None;
        self.sync_bootstrap_progress = None;
        self.sync_reseed = None;
        self.presence_peers.clear();""")
swap("shell", """        let directory_changed = self.advance_document_opening().await || directory_changed;""", """        let directory_changed = self.advance_document_opening().await || directory_changed;
        let directory_changed = self.advance_sync_reseed().await || directory_changed;""")
swap("shell", """                ArtifactEvent::Presence { peers } => {""", """                ArtifactEvent::RebootstrapRequired { control } => {
                    self.sync_reseed = Some(ShellSyncReseed { control, pending: None, retry_at_ms: 0.0 });
                    changed = true;
                    sync_panel_changed = true;
                }
                ArtifactEvent::Presence { peers } => {""")
swap("shell", """    /// 🧯️ Settles the open as failed, out loud: its band keeps the reason and the notice names it.""", """    /// 🛟️ Drives a rebootstrapping hub document back to Live — the same canonical-pair seed as its first open
    /// ([`seed_hub_document`]): the pair is fetched detached and admitted only as the actor's control's checkpoint, loaded into
    /// the guest, and handed to the actor ([`ArtifactActorMsg::Reseed`]), which then dials at once. A transient failure is asked
    /// again after [`SYNC_RESEED_RETRY_MS`] while the document stays open; closing the document drops the reseed. Answers
    /// whether the guest's document changed.
    async fn advance_sync_reseed(&mut self) -> bool {
        let Some(mut reseed) = self.sync_reseed.take() else { return false };
        let Some(owner) = self.sync_channel.as_ref().map(shell_sync_channel_owner) else { return false };
        match reseed.pending.as_ref().and_then(ShellDetached::take) {
            None if reseed.pending.is_some() => {
                self.sync_reseed = Some(reseed);
                false
            }
            None => {
                if chrome_now_ms() < reseed.retry_at_ms {
                    self.sync_reseed = Some(reseed);
                    return false;
                }
                let Some(client) = self.directory_client.clone() else {
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync_reseed = Some(reseed);
                    return false;
                };
                let (ctx, control) = (self.directory_ctx(), reseed.control.clone());
                reseed.pending = Some(ShellDetached::spawn(async move { client.rebootstrap_canonical_checkpoint_pair(&ctx, &control).await.map_err(|error| format!("canonical checkpoint pair: {error}")) }));
                self.sync_reseed = Some(reseed);
                false
            }
            Some(Err(error)) => {
                Self::debug_log(&format!("[DEBUG] wgpu shell rebootstrap reseed refused: {error}"));
                reseed.pending = None;
                reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                self.sync_reseed = Some(reseed);
                false
            }
            Some(Ok(pair)) => {
                let Some(plugin) = self.plugins.iter().find(|entry| entry.plugin_id == owner.plugin_id).cloned() else { return false };
                if let Err(error) = plugin.load_app_document_pack(owner.instance_id, &pair.pack_bytes, &pair.spr_bytes).await {
                    Self::debug_log(&format!("[DEBUG] wgpu shell rebootstrap reseed load failed: {error}"));
                    reseed.pending = None;
                    reseed.retry_at_ms = chrome_now_ms() + SYNC_RESEED_RETRY_MS;
                    self.sync_reseed = Some(reseed);
                    return false;
                }
                let message = ArtifactActorMsg::Reseed { pack: pair.pack_bytes, spr: pair.spr_bytes, baseline: pair.baseline_frontier };
                if let Some(channel) = self.sync_channel.as_ref() {
                    let _ = channel.cmd_tx.send(message);
                }
                self.owe_refresh(UiDirtyScope::Full);
                true
            }
        }
    }

    /// 🧯️ Settles the open as failed, out loud: its band keeps the reason and the notice names it.""")

# ── 5. TS twin + fixture schema + TS law ──────────────────────────────────────────────────────────────────────────────────
swap("ts_schema", """  if (canonicalCheckpointPairHexV1(pair.aggregateSha256) !== expected.aggregateSha256) throw new Error("canonical-checkpoint-pair.aggregate");
}
//#endregion 🪢️CanonicalCheckpointPair""", """  if (canonicalCheckpointPairHexV1(pair.aggregateSha256) !== expected.aggregateSha256) throw new Error("canonical-checkpoint-pair.aggregate");
}

/** 🛟️ Admits a decoded pair only as exactly the checkpoint a hub `RebootstrapRequired` control names — the twin of the kernel's
 * `CanonicalCheckpointPairV1::admit_rebootstrap` (the control carries no aggregate; the digests prove the bytes). */
export function admitCanonicalCheckpointPairForRebootstrapV1(pair: CanonicalCheckpointPairV1, control: { readonly scope: DocumentScope; readonly checkpointId: ArtifactHash; readonly descriptorDigestV1: ArtifactHash; readonly baselineFrontier: ArtifactFrontier }): void {
  if (pair.scope.spaceId !== control.scope.spaceId || pair.scope.documentId !== control.scope.documentId) throw new Error("canonical-checkpoint-pair.scope");
  if (canonicalCheckpointPairHexV1(pair.activeCheckpointId) !== canonicalCheckpointPairHexV1(control.checkpointId)) throw new Error("canonical-checkpoint-pair.checkpoint");
  if (canonicalCheckpointPairHexV1(pair.descriptorDigestV1) !== canonicalCheckpointPairHexV1(control.descriptorDigestV1)) throw new Error("canonical-checkpoint-pair.descriptor");
  const frontier = pair.baselineFrontier;
  const baseline = control.baselineFrontier;
  if (frontier.documentId !== baseline.documentId || frontier.headEditOrdinal !== baseline.headEditOrdinal || frontier.headEditId !== baseline.headEditId || frontier.lastCommitSeq !== baseline.lastCommitSeq || canonicalCheckpointPairHexV1(frontier.chainHash) !== canonicalCheckpointPairHexV1(baseline.chainHash)) throw new Error("canonical-checkpoint-pair.baseline");
}
//#endregion 🪢️CanonicalCheckpointPair""")
swap("ts_law", """  admitCanonicalCheckpointPairV1,
  decodeCanonicalCheckpointPairV1,""", """  admitCanonicalCheckpointPairForRebootstrapV1,
  admitCanonicalCheckpointPairV1,
  decodeCanonicalCheckpointPairV1,""")
swap("ts_law", """  for (const admission of fixture.admissions) {""", """  for (const admission of fixture.rebootstrapAdmissions) {
    it(`rebootstrap ${admission.id} is ${admission.refusal ?? "admitted"}`, () => {
      const pair = fixture.pairs.find((candidate: { id: string }) => candidate.id === admission.pair);
      const decoded = decodeCanonicalCheckpointPairV1(hexBytes(pair.bodyHex));
      const control = { ...admission.control, checkpointId: [...hexBytes(admission.control.checkpointId)], descriptorDigestV1: [...hexBytes(admission.control.descriptorDigestV1)] };
      const admit = () => admitCanonicalCheckpointPairForRebootstrapV1(decoded, control);
      if (admission.refusal === null) expect(admit).not.toThrow();
      else expect(admit).toThrow(admission.refusal);
    });
  }

  for (const admission of fixture.admissions) {""")
swap("json_schema", """  "required": ["schema", "mediaType", "limits", "pairs", "refusals", "admissions"],""", """  "required": ["schema", "mediaType", "limits", "pairs", "refusals", "admissions", "rebootstrapAdmissions"],""")
swap("json_schema", """    "admissions": {
      "type": "array",""", """    "rebootstrapAdmissions": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["id", "pair", "control", "refusal"],
        "properties": {
          "id": { "type": "string", "minLength": 1 },
          "pair": { "type": "string", "minLength": 1 },
          "control": {
            "type": "object",
            "additionalProperties": false,
            "required": ["scope", "checkpointId", "descriptorDigestV1", "baselineFrontier"],
            "properties": {
              "scope": { "$ref": "#/definitions/scope" },
              "checkpointId": { "$ref": "#/definitions/hex32" },
              "descriptorDigestV1": { "$ref": "#/definitions/hex32" },
              "baselineFrontier": { "$ref": "#/definitions/frontier" }
            }
          },
          "refusal": { "oneOf": [{ "type": "null" }, { "type": "string", "pattern": "^canonical-checkpoint-pair\\\\.[a-z-]+$" }] }
        }
      }
    },
    "admissions": {
      "type": "array",""")

if PROBLEMS:
    print("DRY RUN FAILED:")
    for problem in PROBLEMS:
        print("  " + problem)
    sys.exit(1)
if "--apply" in sys.argv:
    for key, path in FILES.items():
        path.write_text(TEXT[key])
    import subprocess
    subprocess.run([sys.executable, str(pathlib.Path(__file__).with_name("gen-canonical-pair-fixture.py"))], check=True)
    print("applied; fixture regenerated with rebootstrapAdmissions")
else:
    print(f"dry run clean: {sum(1 for _ in FILES)} files, every anchor matched exactly once")
