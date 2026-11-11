# proctransient result

Shared root of the 18x3 errors (not per-crate):
- `retained_set_field!` (replication-retained `🎛️set-field`): ported to accessor form (`source_accessor`/`original_accessor`/discriminators/projection work/`field_birth_work` births).
- `#[mutations(retained)]` derive (`os/dsl/✨️derive/🧳️retained`): rewritten to reply-shaped `RetainedMutationPreparation` (`decide_reply`, `inverse_reply`, `close_reply`, `begin_close_paid(effect)`, original birth/control forwarding). Leaf selection charges its quote via `retained_conclude`; denied turns charge nothing; leaf Pending passes through with state kept; terminal release charges 1 unit.
- os kernel: `pub use protocol::retained::conclude as retained_conclude`.
- 3 crate roots: hand-written `SnapshotProjection` impl replaced by `#[derive(semio_framework_value::SnapshotProjection)]`.

Verified (wasm, hub manifest): W-proctransient-1 (3d editor transient) rc0, W-proctransient-2 (2d editor transient + 3d viewer transient) rc0, no errors.

Rerun: `chk.sh W-proctransient-N 🌎️hub/Cargo.toml wasm -p <crate>`

Debts:
- Derive unit test (`🧳️retained/🦀️.rs` tests) updated, not run.
- Unused dep `semio-framework-value-snapshot-projection` remains in the 3 transient Cargo.toml (removal needs a lock run).
- `retained_set_record!` does not define `authored_birth_work`/`displaced_birth_work` (defaults refuse at runtime); not mine, not touched.
- Other `mutations(retained)` users (wfc transients, energy model, generation2d/3d retained-commands) unverified.


## Follow-ups (coordinator round 2)
- `retained_set_record!` / `retained_root_record!`: now define authored/displaced births via `field_birth_work` (edited, unverified).
- 3 transient Cargo.toml: dropped `semio-framework-value-snapshot-projection` (LOCK NEEDED logged).
- test `original-snapshot-owners`: decode/encode/SQLite recipient closes run through `OriginalRetirementPump` turns (pending/denied keep state, 4096-turn stall assert); +`semio-framework-job` dep (LOCK NEEDED). Edited, unverified.
- wfc `engine-job`: `Publication::close_step(effect)`; owner closes via `close_original_effect_step`; `close_result` removed; two `RetainedWorkOutcome` literals annotated. Edited, unverified.
- W-proctransient-3 (hub: wfc/energy/editor crates): stopped upstream at decl-views (3), window-transient (31), wfc-engine-job (5), original-snapshot-owners (9); the latter two now edited.
- W-proctransient-4 (wfc manifest) and W-proctransient-5 (hub): BLOCKED, `Cargo.lock behind its manifests`, crate `semio-framework-job-recorded-clock` (dep of `semio-framework-job-clock`) cannot be resolved. Coordinator lock run needed first.
- Derive unit test not run (needs lock too).

Rerun after lock: `chk.sh W-proctransient-N 🌎️hub/Cargo.toml wasm -p semio-s-plugin-wfc-engine-job -p semio-framework-test-original-snapshot-owners -p semio-s-artifact-wfc-2d-transient -p semio-s-artifact-wfc-3d-transient -p semio-s-artifact-wfc-bitmap-transient -p semio-s-artifact-energy-model-schema -p semio-s-artifact-procedural-3d-editor -p semio-s-artifact-procedural-2d-editor`; then `tst.sh` for the derive crate.

STATUS: FIXED-verified W-proctransient-1, W-proctransient-2 (3 procedural transient crates); BLOCKED round 2 verification on lock (hub/wfc Cargo.lock behind, job-recorded-clock unresolved)
