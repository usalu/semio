STATUS: PANE SMOKE 1/2 GREEN (W-statik-s4 rc 1): locale render test PASSES; statik_pane_loads_applies_undoes_and_closes RED at load-example publication: runtime retained-command-job refuses closing a non-empty Emit (Effect::LoadDocument payload) - runtime-owned, reported (W-statik-s4.txt:68807). wasm --lib fem GREEN. UNVERIFIED: undo/redo, capacity law.

## Done
- Admission cap cleared by hub lane (admission-ok exists).
- Fixed: removed unused dep `semio-plugin-retained-command` (nonexistent package; real name semio-framework-plugin-retained-command, no source use) from fem 3d `✏️editor/📦️packages/🦀️rust/Cargo.toml`.
- Static capacity fix, UNVERIFIED: record matcher `frames` now `PagedList::with_page_items(4)` (`🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🦀️.rs`). Further candidates if the size-law test `receiving_charges_at_most_sixteen_kibibytes_of_retained_capacity_per_node` still fails: paged-list cursor `.../🧬️retained-clone/📋️paged-list/🦀️.rs:47`, paged-bytes cursor `.../🧬️retained-clone/📦️paged/🦀️.rs:38`.

## Blocker (foreign, routed to main)
`C/🗑️generated/1105b/statik-1.txt:158`: no matching package `semio-os-store-member-open`; required by space-member Cargo.toml:35, but 🚪️open crate is now named `semio-framework-os-store-member-open`.
Rerun: `1105b-tools/statik/run1.sh`.

## statik-1 after lock-ok
Lock refresh passed; cargo then refused `semio-plugin-decl-app` (`🔌️plugin/📜️declaration/🧩️app/📣️proofs/🦀️.rs`, reason files, remainingFiles 0). Log `C/🗑️generated/1105b/statik-1.txt:944`.

## from_reference birth authority (static read)
`Fem3dSnapshot::from_reference` (`🧬️schema/📸️snapshot/native.rs:97`) births under an explicit unbounded wallet (2^40) with its own recipient and retires receiving frames before returning; it is authoring-only. The 512 MiB OwnershipLimit fault is on the actual-owner receive path (snapshot test), not here, so per-node cost (page sizes) is the lever, not the wallet.

## statik-1 after declapp-ok (log C/🗑️generated/1105b/statik-1.txt)
- :3613 E0061 `🧰️framework/🔨️modules/🎒️pack/🎛️controlled-and-borrowed-bridge/🦀️.rs:135` advance() called with 3 args, defined with 2 (:153) (pack-json).
- :4333 E0252 `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔁️transfer/🫴️receiving/🦀️.rs:5` RetireOwned imported twice.
- :4430 E0433/E0432 `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛬️receive/🦀️.rs:6` imports NativeReceive/NativeReceiver/owned from semio_framework_value_native_decoding root (now semio_framework_value::NativeReceive...) (ui-contract-text).

## Fixed
`from_reference` in fem 2d+3d `🧬️schema/📸️snapshot/native.rs` now ends `received.result.map_err(store::NativeSnapshotFailure::into_value_error)` (RetainedWorkOutcome port). Log `C/🗑️generated/1105b/statik-1.txt`.

## instance-render port
`🧵️instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs`: build_artifact_reserved_{action,media}_job take `cx:&mut StepContext` (binding derived inside) and drive `A::build_reserved_tool_job(&mut Option<req>,&mut body,cx)` then `ArtifactReservedToolJob::admit_boxed_original` via `complete()`, which refuses (no spin) if a step needs a further turn than the caller holds. Callers updated (typed-command, reserved-routes). NOTE: app-side builders (raster, cad, sequence, play-app editors, puzzle...) still use the old `fn build_reserved_tool_job(request)->Result<Option<Job>,Fault>` signature and will fail to compile (not fem).

## DEFERRED design: multi-turn reserved builders
`complete()` in reserved-jobs faults when a builder step needs a further turn: the async callers run in `OriginalOwnedCallerTurn::execute` (single-issue wallet, no suspension primitive). Preferred end state (coordinator, runtime-owned): (b) `DeferredBuild<A>` reserved job whose build phase runs in its own steps, immediate-None path for non-reserved tools kept synchronous. Revisit only if a pane smoke/e2e shows a multi-turn builder fault. No fem route uses reserved jobs.

## stdio mesh/drawing
mesh-snapshot: `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` encode/decode wrapped in `native_snapshot_turn(|_work|Ok(..?))`; passes. drawing-snapshot: same wrap applied, 4 errors left: derive(NativeReceive) refuses validated types (`deserialize_controlled_with`) on DrawNode/DrawStyle/DrawLayer/SemioDrawingSnapshot (`🧬️schema/📸️snapshot/🦀️.rs:49,88,110,171`), needs hand-authored NativeReceive (record open bound `S:NativeReceive<RecordValue>`). Not attempted blind.

## Drawing via TryMap
New primitive NativeReceiveTryMap/NativeTryMappedReceiver (value-native-owned map/🦀️.rs) + test `fallible_native_mapping_admits_valid_text_and_refuses_marked_text_with_funded_cleanup` (UNRUN, tools/statik/tst1.sh). Four drawing types wired via manual impls (no weaker validation).

## TryMap location
New crate semio-framework-value-native-owned-try-map (🛬️decode/🧳️owned/🛡️try-map) because native-owned hit its source-extent cap. Test: 🛡️try-map/🧪️tests/🦀️.rs (valid + refused, funded cleanup, allocation-exact receipts; oracle = byte length of input).

## Fixes this phase
- fem 2d+3d editor preparation: `last_turn_work` (advance wrapper over advance_owned, last_work field).
- fem-3d-editor Cargo.toml: restored `semio-plugin-retained-command` (my earlier removal was wrong; used by 🗃️retained-commands macro).
- stdio csv-snapshot + gltf-model-snapshot-root ported to RetainedWorkOutcome/native_snapshot_turn (compiles as part of statik-1; gltf pack API still returns Result with work dropped at the pack boundary).

## Test ports (fem 3d snapshot-tests, UNVERIFIED)
native-tests/identity-index-tests/write-pack-tests/loads receiver-tests: `.result.unwrap()` on RetainedWorkOutcome, `error.kind()`, `.result?.progress()`, dup RetireOwned import removed.

## statik-s1
- Smoke target: `statik_pane_smoke` (semio-s-artifact-fem-3d-editor): load concrete forest by SetActiveExample, AddNode, undo, redo, close + new `statik_pane_body_renders_in_every_locale` (case data 🧪️tests/🪧️pane-smoke/🧫️cases/🔣️.json, fresh app per locale).
- Fixed: `#![recursion_limit]` in the target; from_reference work loan (zero-arg framework birth `ReferenceArtifactAppBirth = fn()->Result<P,Fault>` has no caller to thread; loan matches the pre-existing 2^40 authoring wallet).
- Open (runtime): `semio-plugin-instance-ladders/📥️staging/🔒️close-ladder/🦀️.rs:75` drops cached history tuple Arc<Snapshot> unretired -> SealedShared panic.

## W-statik-s2/s3
Fixed in fem: set-active-example handlers (2d+3d) retire the parsed document (retire_decoded). Logs C/🗑️generated/1105b/W-statik-s2.txt, W-statik-s3.txt.
