//#region 🔖TestSupport
/// @emoji 🧪 Round-trip assertions shared by every technology crate's `Operation` test suite.
pub mod test_support {
    use super::*;

    /// @emoji 🔁 Asserts that applying `operation` then applying its reversed `backwards(pre)` restores `pre`.
    pub fn assert_operation_round_trip<P, Operation>(pre: &P, operation: Operation)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Operation: crate::Operation<P>,
    {
        let post = apply_operation(pre, &operation);
        let mut backwards = operation.backwards(pre);
        backwards.reverse();
        let restored = backwards
            .iter()
            .fold(post, |projection, back_operation| apply_operation(&projection, back_operation));
        assert_eq!(&restored, pre, "operation backwards did not restore pre-state");
    }

    /// @emoji 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply projection, and replay-materialization agrees with the live store projection.
    pub fn assert_store_roundtrip<P, Operation>(initial: P, operation: Operation)
    where
        P: Clone + Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
        Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
    {
        let envelope = create_document_envelope("test/v1", "test", initial.clone(), None);
        let mut store = DocumentStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![operation],
                description: None,
            })
            .expect("apply");
        let post = store.projection().expect("post projection");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(
            store.projection().expect("undo projection"),
            initial,
            "undo did not restore initial projection"
        );
        store.dispatch(DocumentCommand::Redo).expect("redo");
        assert_eq!(
            store.projection().expect("redo projection"),
            post,
            "redo did not restore post projection"
        );
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store projection");
    }

    /// @emoji 📜 Asserts a DSL round trip: `P::parse_dsl(&projection.print_dsl())` recovers an equal
    /// projection. The compile-time validation ground truth for every technology's `🔖Dsl` region —
    /// call this from a `#[test]` over every `include_str!` fixture.
    pub fn assert_dsl_round_trip<P>(projection: &P)
    where
        P: DocumentDsl + PartialEq + std::fmt::Debug,
    {
        let printed = projection.print_dsl();
        let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&parsed, projection, "dsl round trip diverged;\nprinted:\n{printed}");
    }

    /// @emoji 📦 Asserts a pack round trip: `P::decode_pack(&projection.encode_pack())` recovers an
    /// equal projection — the pack sibling of `assert_dsl_round_trip`.
    pub fn assert_pack_round_trip<P>(projection: &P)
    where
        P: DocumentPack + PartialEq + std::fmt::Debug,
    {
        let bytes = projection.encode_pack();
        let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, projection, "pack round trip diverged");
    }

    /// @emoji ⚖️ Asserts dsl and pack are two projections of the SAME value: `decode_pack(
    /// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
    /// for the whole pack rollout's central LAW (see `DocumentPack`'s doc comment).
    pub fn assert_dsl_pack_equivalence<P>(projection: &P)
    where
        P: DocumentDsl + DocumentPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&projection.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&projection.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&via_pack, projection, "pack round trip diverged from source projection");
        assert_eq!(&via_dsl, projection, "dsl round trip diverged from source projection");
        assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
    }

    /// @emoji ⚡ Asserts an op-text round trip for a single operation: `print_op` contains no newline
    /// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
    /// truth for every technology's `🔖OpText` region — call this once per `Operation` variant.
    pub fn assert_op_line_round_trip<Op>(operation: &Op)
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
        assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
    }

    /// @emoji 📄 Asserts that printing a store's envelope to text and parsing it back yields the same
    /// live projection the store already holds — the ground truth for {@link print_document_text}/
    /// {@link parse_document_text} on any technology once it implements `DocumentDsl` + `OpText`.
    pub fn assert_document_text_round_trip<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + DocumentDsl + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + OpText + crate::Operation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.projection().expect("store projection");
        let files = print_document_text(store.envelope()).expect("print document text");
        let parsed: ParsedDocumentText<P, Operation> =
            parse_document_text(&files.dsl, &files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed.projection, live, "document-text round trip diverged from store projection");
    }

    /// @emoji 🗄️ Asserts a full pack-based document round trip: mirrors
    /// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`, and
    /// additionally asserts the pack path's parsed projection agrees with the text path's — the two
    /// storage formats must never diverge on the same store.
    pub fn assert_document_pack_round_trip<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + DocumentDsl + DocumentPack + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + OpText + crate::Operation<P> + PartialEq + Serialize + DeserializeOwned,
    {
        let live = store.projection().expect("store projection");
        let pack_files = print_document_pack(store.envelope()).expect("print document pack");
        let parsed_pack: ParsedDocumentText<P, Operation> =
            parse_document_pack(&pack_files.pack, &pack_files.ops).unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed_pack.projection, live, "document-pack round trip diverged from store projection");

        let text_files = print_document_text(store.envelope()).expect("print document text");
        let parsed_text: ParsedDocumentText<P, Operation> =
            parse_document_text(&text_files.dsl, &text_files.ops).unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        assert_eq!(parsed_pack.projection, parsed_text.projection, "document-pack path diverged from document-text path");
    }

    /// @emoji ✉️ Asserts that converting an `Edit<Operation>` into `protocol::OperationEnvelope`s
    /// (`protocol_causal`'s canonical wire/causal representation, moved from `framework/core` in CW3,
    /// via `protocol::operation_envelope_from_edit`) preserves every operation's essential facts —
    /// the causal-wire sibling of `assert_pack_round_trip`/`assert_dsl_round_trip` for the app
    /// fan-out's "pack laws" cluster.
    ///
    /// `OperationEnvelope` is a runtime struct that is never itself re-serialized back into an
    /// `Edit` (unlike `encode_pack`/`decode_pack`, there is no `envelope_to_edit` inverse — vcs's OWN
    /// `edit_from_operation_envelope` recovers a *whole edit* from vcs's own, differently-shaped,
    /// per-edit `semio_framework_core::OperationEnvelope`, not from this per-operation
    /// `protocol_causal` one), so a byte-level encode-then-decode law is not meaningful here.
    /// Instead this checks the two LAWS that actually matter for this bridge: (1) whatever
    /// `edit.operation_meta` explicitly recorded for a slot (the ground-truth source
    /// `operation_envelope_from_edit` prefers over its own `Operation`-trait/structural fallbacks —
    /// see that function's own doc comment) survives unchanged onto the envelope's
    /// `operation_id`/`dependencies`/`actor`/`timestamp`; and (2) `envelope.diff.payload`/
    /// `envelope.inverse.inverse_diff` decode back (via `Operation`'s own `Deserialize` impl) into
    /// operations equal to `edit.forwards[i]`/`edit.backwards[i]` — the part a hand-rolled
    /// `Serialize`/`Deserialize` pair can silently break. Deliberately does NOT recompute the
    /// envelope's fallback chain (id/actor/deps when `operation_meta` is absent) itself, since doing
    /// so would just re-run `operation_envelope_from_edit`'s own logic against itself and always
    /// agree — see this function's `🧪Tests` sibling for a deliberately lossy `Operation` impl that
    /// trips law (2).
    pub fn assert_command_envelope_round_trip<P, Operation>(edit: &Edit<Operation>, document_id: &DocumentId)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Operation: crate::Operation<P> + PartialEq + std::fmt::Debug,
    {
        let envelopes = protocol::operation_envelope_from_edit::<P, Operation>(edit, document_id);
        assert_eq!(envelopes.len(), edit.forwards.len(), "one envelope must be produced per forward operation");
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.document_id, *document_id, "document id did not survive the envelope conversion");
            if let Some(meta) = edit.operation_meta.get(index) {
                if let Some(operation_id) = &meta.operation_id {
                    assert_eq!(&envelope.operation_id, operation_id, "explicit operation id did not survive the envelope conversion");
                }
                assert_eq!(envelope.dependencies, meta.dependencies, "explicit dependencies did not survive the envelope conversion");
                if let Some(author_id) = &meta.author_id {
                    assert_eq!(&envelope.actor, author_id, "explicit author id did not survive the envelope conversion");
                }
                assert_eq!(envelope.timestamp, meta.timestamp, "explicit timestamp did not survive the envelope conversion");
            }
            let recovered_forward: Operation = serde_json::from_value(envelope.diff.payload.clone())
                .unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
            assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
            match edit.backwards.get(index) {
                Some(backward) => {
                    let recovered_backward: Operation = serde_json::from_value(envelope.inverse.inverse_diff.clone())
                        .unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
                    assert_eq!(&recovered_backward, backward, "envelope inverse payload did not decode back into an equal backward operation");
                }
                None => assert_eq!(envelope.inverse.inverse_diff, serde_json::Value::Null, "inverse payload must be Null when the edit has no corresponding backwards op"),
            }
        }
    }

    /// @emoji 🩺 Asserts the store's incrementally-maintained live projection agrees with a
    /// from-scratch full replay — the differential check for `DocumentStore`'s stateful `current`
    /// field. Call after arbitrary command sequences (apply/amend/undo/redo/checkpoint/switch
    /// interleavings) in a tech's own tests to confirm the incremental fast paths never diverge from
    /// the replay ground truth.
    pub fn assert_live_equals_replay<P, Operation>(store: &DocumentStore<P, Operation>)
    where
        P: Clone + PartialEq + std::fmt::Debug + Serialize + DeserializeOwned,
        Operation: Clone + Serialize + DeserializeOwned + crate::Operation<P>,
    {
        let live = store.projection().expect("store projection");
        let replayed = materialize_document_projection(store.envelope(), store.applied_edit_ids()).expect("replay");
        assert_eq!(live, replayed, "store's live projection diverged from full-replay materialization");
    }
}
//#endregion 🔖TestSupport
