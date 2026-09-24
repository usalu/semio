// H8 frozen hunk K1 — os-kernel `🏪️store/🦀️.rs` (apply after the freeze lifts, then cargo check -p semio-framework-os-kernel + --target wasm32-wasip2).
//
// (a) ArtifactCodec gains a field after `apply_ops_binary`:
//
//     /// 📜️ Hub Check In: (pack, spr, encode_envelopes) -> (pack, spr, ops text) through the replica fold.
//     // 🚫️async: E4 fn-pointer erasure-table thunk (R1(ii)) — see `compile_dsl`'s tag above.
//     pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecApplyFuture<'a>,
//
// (b) `same_document_codec` gains `&& std::ptr::fn_addr_eq(left.replay_envelopes, right.replay_envelopes)`.
//
// (c) inside `ArtifactCodec::of`, next to `apply_ops_binary_impl`:
//
//         fn replay_envelopes_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecApplyFuture<'a>
//         where
//             P: Clone + ToValue + FromValue + ArtifactDsl + ArtifactPack + Send + Sync + 'static,
//             Mutation: Clone + ToValue + FromValue + OpText + OpBinary + self::Mutation<P> + Send + Sync + 'static,
//         {
//             Box::pin(async move {
//                 let files = replay_envelopes_onto_pair::<P, Mutation>(pack, spr, envelopes, bounded_artifact_store_owners::<P, Mutation>()).await?;
//                 Ok((files.pack, files.spr, files.ops))
//             })
//         }
//     and `replay_envelopes: replay_envelopes_impl::<P, Mutation>,` in the returned Self.
//
// (d) the close block of `apply_ops_binary_impl` (from `let mut closed = Err(...)` through the `drop_ready`
//     check and `std::mem::forget(store)`) moves into `close_codec_reduction_store` below and both thunks call it.
//
// (e) new public functions next to `bounded_artifact_store_owners`:

/// 🧺️ Runs a throwaway reduction store's close cursor to exact terminal emptiness and forgets the
/// shell, so neither codec thunk can reach `ArtifactStore`'s asserting `Drop` with a live owner.
fn close_codec_reduction_store<P, Mutation>(mut store: ArtifactStore<P, Mutation>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let mut closed = Err(VcsError::ValidationFailed("artifact codec store did not reach terminal emptiness within its bounded close budget".into()));
    for _ in 0..ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS {
        match store.close_owned_step(1, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES) {
            Ok(SnapshotRetirementStep::Complete) => {
                closed = if store.close_owned_terminal_is_empty() { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store reported close completion without terminal emptiness".into())) };
                break;
            }
            Ok(SnapshotRetirementStep::Pending { .. }) => continue,
            Ok(SnapshotRetirementStep::Blocked) => {
                closed = Err(VcsError::ValidationFailed("artifact codec store close is blocked by an outstanding snapshot read lease".into()));
                break;
            }
            Err(error) => {
                closed = Err(VcsError::ValidationFailed(error));
                break;
            }
        }
    }
    let drop_ready = store.envelope_detached
        && store.current_detached
        && store.backbone.is_none()
        && store.dag.terminal_is_empty()
        && store.applied_edit_ids.is_empty()
        && store.redo_edit_ids.is_empty()
        && store.current_checkpoint_id.is_none()
        && store.local_actor_id.is_none()
        && store.revision_accumulator.applied.is_empty()
        && store.revision_accumulator.redo.is_empty()
        && store.tail_undo_cache.is_none()
        && store.snapshot_read_leases.terminal_is_empty()
        && store.displaced_retirements.terminal_is_empty()
        && store.owned_disposer.is_none()
        && store.owned_disposer_terminal
        && store.pending_report.edit_ids.is_none()
        && store.pending_report.messages.is_empty()
        && store.pending_report.outbound.is_empty()
        && store.pending_report.worst.is_none()
        && store.durable_group_root.is_none();
    std::mem::forget(store);
    closed?;
    if drop_ready { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store close left a live shallow-shell owner".into())) }
}

/// 📜️ Folds an `os_spr::encode_envelopes` stream onto one authoritative pair through the SAME
/// gate every replica folds a remote envelope through ([`ArtifactStore::ingest_remote`]): causal
/// order, history transitions (undo/redo/checkpoint), edit identity and conflict quarantine are
/// exactly a replica's, so the printed pair is the one any client holding that ledger prefix
/// holds. This is how a hub materializes a Check In from its own ledger; a raw-mutation apply
/// (`apply_ops_binary`) would mint new edit ids and drop every transition.
///
/// See `🌎️hub/🗿️artifact-authority/📌️check-in` and `db::document::artifact_ledger_tail`.
pub async fn replay_envelopes_onto_pair<P, Mutation>(pack: &[u8], spr: &[u8], envelopes: &[u8], owners: DocumentStoreOwners<P, Mutation>) -> Result<ArtifactPackFiles, VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + 'static,
{
    if pack.is_empty() || spr.is_empty() {
        return Err(VcsError::Deserialize("replay-envelopes has no pack+spr baseline".into()));
    }
    let envelopes = crate::os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let parsed = parse_document_pack::<P, Mutation>(pack, spr).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut store = ArtifactStore::new(parsed.into_envelope()).await?;
    store.install_document_store_owners_exact(owners);
    let mut folded = Ok(());
    for envelope in envelopes {
        if let Err(error) = store.ingest_remote(envelope).await {
            folded = Err(error);
            break;
        }
    }
    let printed = match &folded {
        Ok(()) => print_document_pack(&store.envelope).await,
        Err(_) => Err(VcsError::ValidationFailed("replay-envelopes skipped print after a refused fold".into())),
    };
    let closed = close_codec_reduction_store(store);
    folded?;
    closed?;
    printed
}

// (f) law in `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (mirrors the existing apply_ops_binary laws' fixture app):
//     `replay_envelopes_onto_pair_equals_the_replica_that_folded_the_same_ledger` —
//     replica A: genesis pair → dispatch two edits + undo of the first + commit checkpoint → event_log() envelopes;
//     replay those envelopes onto the genesis pair → printed pair == A's print_document_pack (pack AND spr bytes),
//     applied_edit_ids equal, and the undo is present; replaying a prefix ending before the undo equals A's state
//     at that prefix; a corrupt envelope stream is refused and the store closes (no abort).
