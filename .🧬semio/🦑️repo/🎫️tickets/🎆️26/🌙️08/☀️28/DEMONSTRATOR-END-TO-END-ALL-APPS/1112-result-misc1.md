# misc1 (11/12)
- W-misc1-2 wasm rc 0: sourcing-curation-domain (allocation port passed directly), demonstrator-playground-editor (original_pack_value_operation_receiving! on PlaygroundMutation), puzzle-3d-editor (store-preparation advance/receiving recovery/close_step(effect), clipboard-job effect close, play-app SharedOriginalStorage.get()).
- Next: components.
## Step 2 (hub components, W-misc1-3 rc 101, W-misc1-4 rc 101)
Fixed beyond the three crates (all compile in W-misc1-4: no error from them):
- sourcing-curation-inferences: original_pack_value_operation_receiving!(GridWindowConfigMutation) (sourcing grid window config).
- puzzle-5d-mut-any-5: Puzzle5dPlaySnapshot RetireOwned ported to retirement_seed/ArcLease (same as puzzle-3d play snapshot).
- puzzle-2d-snapshot: Puzzle2dWindowConfigMutation receiving macro; sqlite snapshot allocate_collection uses reserve_exact_capacity_one_cold (caller already charges the page bytes).
No error in demonstrator-playground / demonstrator / demonstrator-component code in W-misc1-3.
## OPEN: puzzle-2d lookup + prepare family (6 errors in W-misc1-4, root of semio-hub-puzzle red)
`🔎️lookup/🦀️.rs` (Puzzle2dLookupCursor) and its five consumers are grant-shaped against removed API:
- `retained_clone::paged::PagedUtf8BoundedOrdCursor` moved (now `semio_framework_value::paged::retained_clone::cursors::...`) and is reply-shaped (`compare_into(.., effect, reply)`, `begin_close_into`, `close_into`); the convenience `compare(.., grant)` / `close_step(grant)` are gone.
- `RetainedCloneRef::bind(binding, effect)` and `RetainedCloneBinding::close_one(binding, body, effect) -> OriginalRetirementPoll` need a RetainedAllocationEffect, but the consumers (`mutations/🔤️text/🎮️prepare`, `🟢️node/🗑️delete-node/🎮️prepare` (+📸️candidate), `🔌️handle/🪢️connect-handles/🎮️prepare`, `🟢️node/🌱create-node/🎮️prepare`, `🚩️flag/🎮️prepare`, `🧭️pose/📍move-node/🎮️prepare`) are driven from a RetainedCloneGrant only (no effect/cx), and the store `ArtifactStoreOneItemPreparation::advance` still hands a grant.
- Same debt in draw (`🖍️draw .../🔎️lookup`, rename-layer prepare), not in my lanes.
Needs a design ruling: either move these cursors onto the framework generic ordered-map lookup-cursor (`ordered-map/🔎️lookup-cursor`, reply/effect protocol) driven from a job step with `with_original_effect`, or give the one-item preparation `advance` an effect. Not done; no half-port left (lookup untouched).
Rerun: "$C/1105b-tools/chk.sh" W-misc1-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-puzzle-2d-snapshot
Other lanes' reds seen in W-misc1-3: gis-gismap-tests (flowgis), process-process3d-editor store-preparation/retained-commands (femproc: nested impl in trait, terminal_frame_bound, advance_inner missing).
Not verified: semio-hub-demonstrator(-component), semio-hub-puzzle(-component), semio-hub-sourcing-component rc 0 (cargo halted at the 2d snapshot and other lanes' crates).
STATUS: BLOCKED puzzle-2d lookup/prepare family needs effect-source design (three assigned crates FIXED-verified W-misc1-2)
