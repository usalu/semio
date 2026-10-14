# TXT Store Initialization Proof

Read-only actual source, 2026-10-13. No build/runtime qualification.

Planned `create_document_envelope::<TxtSnapshot,TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA,id,source.clone(),None)` then `plain_test_store(envelope)` preserves the supplied exact typed snapshot. No alternative or synthetic read lease is needed.

`🏪️store/🧾️document/🦀️.rs:29` calls `AdmittedArtifactGenesis::born(initial_snapshot)`. Actual `🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs:34` encodes snapshot into pack, then `from_decoded_pack(snapshot,pack)` with the original supplied P. This hashes/stores pack and admits the supplied P into typed facts; it does **not** decode pack. Raw TXT pack may be lossy relative to legal fields, but does not replace the typed source in this constructor.

`🏪️store/🦀️.rs::ArtifactStore::construct:4256` uses `envelope.vcs.genesis.facts().share_snapshot()` when loaded_applied_edit_ids is empty. New envelope has empty history, so this is the exact original typed Arc root. Nonempty history folds begin from typed genesis, not a raw pack decode. Actual replay materialization `🧾️document/🦀️.rs:102` similarly clones typed genesis snapshot before edits.

Separate `AdmittedArtifactGenesis::from_stored_pack` does decode P and can normalize raw TXT state; do not use it for this full snapshot Store law. Imported raw child/file reload paths are not equivalent to the planned typed source setup.

`🏪️store/🦀️.rs:12207 pub mod test_support` is unconditional public module, outside cfg(test). `plain_test_store` installs real plain owner catalogs; no separate test-support feature is required in current Kernel package. This helper setup has its own allocations and policy; keep it outside file-IO measured admission window. Use actual Store.snapshot_read().into_erased(), exact source pointer and field assertion immediately after creation to confirm runtime source fidelity once owning law runs. Return exact typed read/witness and close same installed Store afterward.
