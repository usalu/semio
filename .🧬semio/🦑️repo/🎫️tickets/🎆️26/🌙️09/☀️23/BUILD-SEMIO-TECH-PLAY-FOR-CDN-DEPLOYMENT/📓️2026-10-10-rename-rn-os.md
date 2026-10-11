# 2026-10-10 rn-os report (os foundation)

## Result lines
- `cargo check -p semio-framework-os-kernel --lib` (native): Finished, 0 errors (private target dir `play-fleet/rn-os`).
- `cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2`: Finished, 0 errors.
- `cargo check -p semio-framework-plugin --lib --keep-going`: 455 errors (see `📓️2026-10-10-migration-corrections.md`, section "rn-os 11:50 FINDING").
- `plugin-host`, `plugin-describe`, `semio-framework-os` (host): not reachable (depend on plugin). `semio-framework-os-kernel-db`: 66 errors (PackError/SharedUtf8/grant drift), only the hub depends on it.
- Not run: graph layout-run, hub, fem, renderer, services (transferred to os-domains / blocked behind plugin or tool-run).

## Files changed
- job (before ownership split): `🧵️job/🦀️.rs` (mount `retained_work`, `session_return`; `WorkerJobSessionArc` over `SessionHandle`; StepContext `retained_work` init), `♻️session-return/🦀️.rs`, `♻️session-return/🔐️handle/🦀️.rs`; replication `…/🔏️seal/🦀️.rs` (sealed value API). See `📓️2026-10-10-rename-rn-os-edits-below-store.md`.
- os io: `🚪️io/🦀️.rs` (`Serializer::OWNED_FACTORY`, `IoEntry.owned_serializer`).
- store: `🦀️.rs` (removed duplicate erased `try_return_to_registry_witness`, derived-snapshot registry alias, plain test factory `begin_batch_digest`/3-generic request/text-wire error types, `ArtifactChild` canonical tree), `🔗️read/♻️retirement`, `🔗️read/🧾️return` (`ErasedSnapshotRead: RetireOwned`), `🧵️operation-wire`, `🧬️schema/🧬️mutations/🔏️canonical`, `🧬️snapshot-clone` (sealed `take_authority` replaces Arc owner take/unwrap), `🧵️canonical-edit/🧳️source` (sealed `admit_borrowed`/`project_owned(grant)`), `📦️blob`, `🔗️link` (canonical tree derives).
- artifact-reference: `🧬️schema/🗿️artifact-reference/🦀️.rs` + `Cargo.toml` (canonical trees for `ArtifactRef`/`ArtifactDialect`).
- plugin crate (source only, uncompiled): `🦀️.rs` (initialization authority `step`/`borrow_outcome`, `ArtifactReservedToolJob::step`/`borrow_outcome`, removed dead `plugin_outcome_close_progress`, `Refused{..}` patterns, duplicate fields), `🌱️initialization/♻️retirement/🦀️.rs`, `🪟️window/🎚️config/🦀️.rs` + `🧬️preparation/🔁️replace/🦀️.rs` (public `WindowConfigReplaceEdit`).
- host: `🖥️host/🦀️.rs` `close_backbone_envelope` (granted envelope retirement loop).
- renderer: `🧊️renderer/🦀️.rs`, `🌉️ProgramBridge/…/🦀️.rs`, `🖼️IconRenderHost/…/🎬️scene/🦀️.rs` (payload/outcome close through `retirement_demands` + `close_step(grant)`); tests in these crates still use the old `drive_step`/`close_step(1, bytes)` and were not migrated.

## Remaining out-of-scope / unresolved
Everything about the plugin lib (455 errors) and the three FEM/graph job families (`LdltJob`, `PcgJob`, `SubspaceIterationJob`, `MeshJob`, `FemJobGraph`, `AssemblyJob`, `LayoutRunJob`) which still implement the old `InteractiveJob` (`step -> StepOutcome`, `close_step(max_items, max_bytes)`). Hub (`SnapshotRetirementStep` ~35 sites, space preparation on the old `advance(...) -> String` shape) untouched.
