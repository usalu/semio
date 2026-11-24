# Publication: "original page allocation attempt is occupied" (2026-11-23)

Status: root cause identified from the code and the commit history, source side already guarded in HEAD, `[DEBUG]` flood removed. NOT verified end to end: both verification routes (generation3d `action-producer`, component rebuild plus browser) are blocked by two unrelated, pre-existing breakages listed under "Blockers".

## Symptom

- Browser `?plugin=generation3d`: `original-turn.native-return-refused: semantic` and `member.publication: original page allocation attempt is occupied` (`🗑️generated/react-6018-console.txt`).
- Native: `action-producer` -> `widget_value_edit_and_undo_reach_the_command_through_the_editor_producer`, "the edit publishes ... original page allocation attempt is occupied" (`📓️procedural-and-retained-tests-2026-11-23.md`, Run 2, 11:34).

## What the message means

The only source of this exact text is `PagedList::reserve_page_using_for_limit` (`🌱️value/📋️list/🦀️.rs`, former line 590). It refuses when the borrowed `RetainedAllocationEffect::actual()` marker is already `Some`. Law (see `PageAllocation` safety doc and `effect/🦀️.rs`): one original effect carries exactly one physical attempt marker, so one effect may perform at most one page/vector allocation. `OriginalEffectRun` (`🎟️authoring/🦀️.rs`) and `with_allocated_effect_reported` create the marker fresh per run, so a marker never leaks across turns; "still occupied" can therefore only mean a second allocation inside the same effect, not a missing release between edits. Nested effects (`effect.nested`) share the parent's marker by design.

## Root cause

A producer drove several units on ONE effect. After the first unit allocated (for the canonical JSON tree the first unit is `frames.reserve_one`, one page of native frames), the next unit's `reserve_one` hit the already set marker. In the publication path this is the second pass of the canonical seal:

`typed-operation-unit` -> `advance_apply_batch_with_turn_ceiling` -> bundle `Preparing` -> sealer `advance` (`with_allocated_effect_reported`, one marker) -> `ArtifactStoreCanonicalSource::advance_original` -> `ArtifactCanonicalEditSealCursor::advance` -> `for _ in 0..SEAL_TURN_BYTES { tree.advance(.., &mut effect.nested(..)) }` -> `ArtifactCanonicalJsonTree::advance_one` -> `self.frames.reserve_one(effect)`.

The refusal surfaced as the original-turn refusal `semantic`, which the host maps to `native-return-refused`. Same class as the 2026-08-28 fix in `JsonGrammarCursor::step_source` (`1107-result-history2.md`: "the unit loop ends once `effect.actual()` is set").

Evidence for the timeline:

- The seal tree loop was introduced and guarded by the GET-PROCESS peer (`GET-PROCESS-WORKING-END-TO-END/seal-cursor.md`); it is in HEAD `a6e34dbae6f` (13:27) with `if tree.terminal_is_empty()||effect.actual().is_some(){break;}` after the step's accounting. That note records the same defect at baseline ("the rest was the second allocation in one effect").
- My failing evidence (Run 2 at 11:34, console at about 12:00) predates that commit. The served component and the test run therefore ran the loop before its guard.

I audited the other repeat-on-one-effect sites of the publication path: bundle `for _ in 0..4096` calls `owner.advance_with_turn_ceiling` per iteration (each call opens its own `with_allocated_effect_reported` marker); identity cursor, authority cursor, `PagedUtf8AppendCursor::advance_with` and `project_step` perform at most one allocation per call. No further second-allocation site was found, and no "marker not cleared after a refused attempt" site exists: every marker is a local of its `OriginalEffectRun`.

## Changes (this pass)

All removals of temporary `[DEBUG]` code that flooded the browser console and sat in the blocked-publication paths; no behaviour change.

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`: dropped the `eprintln!` in the occupied guard (the guard itself and its refusal stay).
- `🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs`: removed `blocked_once` and its four call sites.
- `🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs`: removed the zero-progress `[DEBUG]` block.
- `🏪️store/⚙️engine/🧳️bundle/🦀️.rs`: removed the `batch blocked`, `digest step not admitted` and `digest zero progress` prints.
- `🏪️store/🫧️ephemeral/🌫️transient-and-interaction-stores/🦀️.rs`, `🏪️store/👥️presence/👥️presence-store-and-peers-publication/🦀️.rs`: removed the `store blocked at` prints.
- `🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs`: removed the `digest zero work` print.
- `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`: removed the now unused `debug_state`.
- Left alone on purpose (another agent's active instrumentation, own cleanup list in `GET-NORM-PLUGIN-WORKING-END-TO-END/store-replay-receipt-regression-2026-11-23.md`): `debug_tag`/`original_debug_tag`, `replacement` tags, `🛬️decode/🧳️owned/🌳️value` `debug_state`, `plugin/🧵️instance` `[DEBUG] store parts`.

Compile check after the removals (`repo cargo ... check -j 2`, one process at a time): `semio-os-store-one-item-publication` and `semio-os-kernel` (which pulls the store engine, ephemeral, presence, composition digest, canonical-edit source, value list and replication seal) both exit 0 (`🗑️generated/pub-occ-check1.log`, `pub-occ-check3.log`).

## Blockers for verification

1. generation3d test targets do not build at all. `generation3d/📦️packages/🦀️rust/Cargo.toml` line 136 (`semio-s-artifact-stdio-gltf-model` dev-dependency with `features = ["testing"]`) entered with commit `a6e34dbae6f`. It switches on `gltf-model-snapshot/testing`, whose serde derives need `GltfJson: Serialize/Deserialize`; that impl only exists in the separate `🚪️io/📝️text/📸️snapshot/🧪️serde-oracle` crate (orphan-rule impossible). Result: 123 `E0277` errors in `semio-s-artifact-stdio-gltf-model-snapshot` (`🗑️generated/pub-occ-run2.log`, same in `proc-run5-io.log`), so no target of the crate, `action-producer` included, can compile. Needs the gltf split owner: move the `GltfJson` serde impls into the snapshot crate under `cfg(any(test, feature = "testing"))`, or split the io tests that need them into their own crate.
2. `repo run ...` cannot load the project graph: `BUILD_SOURCE_UNPARSEABLE`, `lstat .../pdf/.../♿️ua/🚪️io/💾️binary/🧬️mutations/🗑️remove-mark-info/📦️packages/🦀️rust/Cargo.toml` (git-tracked at HEAD, moved to `📝️text` by the pdf peer; every manifest already points at `📝️text`). Persists after `repo service host stop` (`🗑️generated/pub-occ-component-r*.txt`), so it comes from the tracked-but-deleted inventory entry, not a stale memo. The component rebuild and the 6018 screenshot are blocked until the pdf move is settled or committed.
3. Earlier in the session two full compiles failed from peers' mid-edits (`original_debug_tag` not yet defined in `semio_framework_job`, gltf above); a cold `action-producer` build takes about 60 min on this machine.

## To finish once the blockers are gone

1. `repo cargo --store=shared --configuration=prototype -- test --manifest-path <generation3d>/📦️packages/🦀️rust/Cargo.toml -j 2 --test action-producer` must show 3 passed.
2. `repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies`, then `S_G3D_RENDERER=react bun <TF>/wgpu-native-activate.ts`, reload `http://127.0.0.1:6018/?plugin=generation3d`, Playwright screenshot of the preview.
3. If `occupied` still appears after the rebuild, the caller is no longer in doubt: re-add the removed `eprintln!` with `std::panic::Location::caller()` (all reserve entry points are `#[track_caller]`, so it names the first non-track_caller site) and fix that loop the same way (break once `effect.actual().is_some()`).
