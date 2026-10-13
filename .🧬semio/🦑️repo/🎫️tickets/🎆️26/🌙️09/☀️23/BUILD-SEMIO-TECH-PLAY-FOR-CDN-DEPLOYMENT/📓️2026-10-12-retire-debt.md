# 2026-10-12 retire-debt: 51 of 55 uncontrolled owners are controlled, 4 stay `false` with a stated reason

Executor: retire-debt (Sonnet). All cargo ran in the foreground of its own slot through `🚦️cargo-slot.sh` with `SLOTS=2` and the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/retire-debt/target` (deleted at the end; scripts `c.sh`, `b-*.sh`, `ed.py`, `scanfalse.py` and every log/status stay in `play-fleet/retire-debt/`). No git-modifying command, no ticket operation, no sub-agent, no `🗑️generated`. Every source edit of a crate was written in one quick sequence and checked with `cargo check` right after.

## Result

| proof | result |
| --- | --- |
| scan (`typed-close/scan.py` and `retire-debt/scanfalse.py`) over framework, `✏️s`, hub, teaching | 55 impls with `controlled_retirement_supported() == false` before, **4 after**, all reasoned (below); 0 impls missing the declaration; `missing_birth` = those 4 |
| native `--lib` check, every touched crate | framework: diagnostic, replication, os-flow, space-space, space-collection, value, os host (`semio-framework-os`); plugins: block shared, block 2d/3d/5d (`component-app-assembly`), cad, wires, sequence, writer, generation3d (feature), bim, note, forms, layout, puzzle 2d/3d/5d (feature). All Finished, 0 errors. A final re-check after the docstring edits (framework 5 crates, block 3d, forms, cad) is green too |
| `wasm32-wasip2` `--lib`, same set (16 plugin manifests + the six framework crates in one call) | all Finished, 0 errors. `semio-framework-os --features space-guest --lib` wasm32-wasip2 also green |
| hub `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-<x> --target wasm32-wasip2` | space, block, cad, reasoning, sequence, writer, procedural (generation3d), bim, note, forms, layout, puzzle: all 12 Finished, 0 errors |
| value retirement tests | `cargo test --manifest-path 🌱️value/…/Cargo.toml --lib -- retirement` 84 passed; whole value crate 307 passed, 0 failed. replication `bounded_history_fold_unsupported_local_refusal_returns_original_before_registration` passes; diagnostic lib 17 passed |
| new runtime laws (details below) | 9 test targets, 19 tests, all green; heap births are exact to the byte in every one |
| TS gate (language-agnostic) | `bun test` of `♻️retirement/🔓️uncontrolled-owners/🧪️tests/🟦️.ts`: 2 pass (Ajv schema check of the fixture + independent scanner equals the 4 reasoned owners) |

Not verified, with the reason: the `HashProjection` fixture of `🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (derive replaces the hand `false`). `cargo test -p semio-framework-os-kernel-db --lib --no-run` does not reach it: `semio-framework-os-kernel` fails to compile under the db feature set at `store/🧩️composition/🗄️durable-group/🦀️.rs:3587` (E0631, a file last touched 10-10 15:27, not mine). The edit is a plain derive over `[u8; 32]`.

## What the laws are

Shared helpers (mounted by `#[path]`, no crate): `🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs` and `…/⚖️transient-preparation-law/🦀️.rs`.

- `typed_input_retirement_law(label, value)`: admits `value` through the real typed-command gate (`store::artifact_retirement_owned_birth_demands` + `artifact_retirement_admit_owned`, the function that returned `UnsupportedOwner`), drives the erased owner turn by turn granting every axis exactly its own quote, asserts each receipt fits the grant, and under the global `HeapWitness` asserts **admission and every turn allocate exactly their declared births** and release at least their declared bytes. `typed_command_retirement_law(label, mutation)` wraps `ArtifactCommand::Apply`.
- `transient_preparation_law::<Owner>(…)`: the owner's own `WindowTransientOwnerBundle` prepares the mutation (Apply), prepares `inverse(base)` (Undo), prepares the mutation again (Redo), each time closing the preparation to its terminal-empty witness, then retires every state root and the mutation through the bundle's own controlled retirement factories.
- Real store catalogs: block 3d uses the shooting pattern (`authoring_law::author_through_document_store::<EditorApp<Block3dPlayApp>>`); space and collection mount `store::funded_bounded_artifact_store_owners::<Snapshot, Mutation>()` (what the OS host installs) on an `ArtifactStore` and dispatch Apply, Undo, Redo, then close to the exact terminal-empty witness.

| target | tests | covers |
| --- | --- | --- |
| block 3d `retirement_law` (`required-features = component-app-assembly`) | 3 | 6 mutations through the typed lane; `Block3dSnapshot` with sole / aliased / absent catalog owner; real-store authoring law for rename-object-kind and create-representation (forward row 1, inverse row 1, undo restores, redo re-applies) |
| writer `retirement_law` | 2 | the 3 `WriterMainWindowTransientMutation` leaves + state through the typed lane; Apply/Undo/Redo preparation through `WriterMainWindowTransientOwner`'s bundle |
| wires `retirement_law` | 3 | `SetDrag` + canvas state typed lane; Apply/Undo/Redo through `WiresCanvasTransientOwner`; `WiresSnapshot` with sole / aliased / absent board owner |
| space `retirement_law` | 2 | 6 `SpaceMutation` variants + manifest typed lane; each variant Apply/Undo/Redo through the bounded catalog, store closes terminal-empty |
| collection `retirement_law` | 2 | same for `CollectionMutation` and the tree |
| forms `retirement_law` | 3 | answer tree empty / aliased / last alias; transient + `Snapshot` mutation typed lane; Apply/Undo/Redo through `FormsTryWindowTransientOwner` |
| cad `retirement_law` | 1 | `CadReferenceIndex` empty and with 3 keys / 3 rows |
| sequence `retirement_law` | 1 | `StepParams` empty and nested |
| os-flow `flow_retirement_law` | 2 | `MeshPackEncodingJob` fresh and mid-encoding, `MapDelta<String>` empty and merged |

Declared births / releases measured (bytes): block3d snapshot sole 13888 / 13633, aliased 8696 / 8130, bare 8312 / 7746; wires snapshot sole 11432 / 11514, aliased 6720 / 6695; forms try values last alias 66200 / 67366, live alias 4952 / 4848; block3d `rename-object-kind` command 29 turns, 5280 / 5031. Only the last alias pays the catalog, board or answer-tree release; that is asserted (`sole.releases > aliased.releases`, and `Arc::strong_count == 1` after retiring the aliased document).

## Findings the laws surfaced (all fixed)

1. `StepParams` (sequence): its `Drop` hook cold-retires the `Dictionary` it leaves behind after `mem::take`, and that cold retirement of an *empty* dictionary allocates and frees 6816 bytes inside every retirement turn (law: turn 2 allocated 9080, declared 2264). `retirement(self)` now moves the dictionary out under `ManuallyDrop` + `ptr::read`, so the Drop hook never runs and the turn is exact.
2. `CadReferenceIndex`: `false` made `close::<CadReferenceIndex>` (cold close on every decode failure) panic with "CAD native cold retirement refused". It also had no birth quote.
3. Forms Try values: `SharedText` / `SharedChunks` / `TryValueContent` / `FormsTryValues` had no birth quote (`Deferred` refuses with "deferred original lacks controlled child birth") and hand cursors that decided aliasing with `strong_count` on unsized `Arc<str>`. Answer text is now `SharedUtf8` (controlled, `Arc<String>`), and the two shared `Arc` fields retire as `SharedControlledRetirement::lease` (last alias retires the content, every other alias only drops its count).
4. `Block3dSnapshot`: the old `SnapshotRetirement` cursor built its wrapper `Vec`s and boxes inside one unquoted step, and answered `BudgetExhausted` forever when the child's local owner had another type. The 16 private wrapper structs are gone (the underlying types already own `RetireOwned`); the snapshot is a plain paid sequence plus the catalog lease.
5. `WiresSnapshot` had the same unmetered drop of the materialized board (`ArtifactChild` retirement drops its typed local owner inline); it now leases it the same way as block 3d.

## Per owner, before → after

Before: hand-written `impl RetireOwned` with `controlled_retirement_supported() == false` and, for all of them, `retirement_birth_bytes() == None`. After: below. "derive" = `#[derive(semio_framework_value::RetireOwned)]` added to the type and the hand impl deleted.

| # | owner | file (crate) | after |
| --- | --- | --- | --- |
| 1 | `MapDelta<V>` | `📡️replication/🧬️retirement` | hand, forwards to `BTreeMap<String, MapEntryDelta<V>>`: flag = that map's (so `V`'s), birth = the map's |
| 2 | `std::vec::IntoIter<T>` | `🌱️value/♻️retirement` | **stays `false`**, docstring: the consumed `Vec`'s capacity is not observable, so the backing release cannot be quoted exactly; only an internal cursor field of parents that price their own scaffold, never a payload, snapshot or factory input. A correct controlled cursor needs `OriginalVectorCursor` (it carries `backing_bytes`) |
| 3 | `TextError` | `⚠️diagnostic` | derive (hand impl removed; it also skipped `kind`) |
| 4 | `MeshPackEncodingJob` | `🌊️flow/🎒️mesh` | `artifact_retire_struct!` over all 9 fields (the hand impl listed 4) |
| 5 | `SpaceMutation` | `🪐️space/🗿️artifacts/🪐️space` | derive; empty `♻️retirement` module deleted |
| 6 | `CollectionMutation` | `🪐️space/🗿️artifacts/🗂️collection` | derive; empty `♻️retirement` module deleted |
| 7 | `HashProjection` | `🛢️db/🗿️artifact/🧪️tests` | derive (not compiled, see above) |
| 8-10 | three test `Unsupported` | value unit tests, value queue ownership tests, replication fold retirement tests | **stay `false` by design**, docstring on each impl: they exist to be refused |
| 11 | `BlockCamera3d` | block shared `♻️retirement` | `artifact_retire_struct!` |
| 12-27 | block 3d private wrappers `KindIdentity`, `Attribute`, `Author`, `Compatibility`, `Representation`, `Camera`, `Meta`, `VortexKindExtra`, `VortexKind`, `Vortex`, `KitType`, `Transform`, `KitPiece`, `KitConnection`, `KitDesign`, `KitSnapshot` | block 3d `🧬️schema/♻️retirement` | **deleted**; `Block3dVortexKindExtra` gained the derive, every other underlying type already owned `RetireOwned` (shared macros, derives, semio crate `retire_struct!`) |
| 28 | `Block3dSnapshot` | same | hand: `take_local_owner::<SemioKitSnapshot>()`, then `sequence(deferred(field)…)` with the owner as `Option<SharedControlledRetirement<SemioKitSnapshot>>` (lease); exact birth from the same field types |
| 29 | `Block3dMutation` | `🧬️schema/🧬️mutations` | derive (all 37 leaf payloads already derived) |
| 30-32 | `Block3dBrushPreview`, `Block3dWorldWindowTransient`, `Block3dWorldWindowTransientMutation` | `…🌐️world/🫧️transient` | derive |
| 33-34 | `Block5dPart3d`, `Block5dGripTemplate` | block 5d `🧬️schema/♻️retirement` | `artifact_retire_struct!` |
| 35 | `Block5dMutation` | `🧬️schema/🧬️mutations` | derive |
| 36 | `Block2dMutation` | block 2d `🧬️schema/🧬️mutations` | derive |
| 37 | `CadReferenceIndex` | cad `📎️references` | hand, forwards to `Vec<(String, CadReferenceList)>` incl. birth quote |
| 38 | `WiresSnapshot` | wires `🧬️schema/📸️snapshot` | hand with the board lease (like #28); empty `schema::retirement` module deleted |
| 39 | `WiresCanvasTransientMutation` | wires canvas `🫧️transient` | derive |
| 40 | `StepParams` | sequence root | hand: flag = `Dictionary`'s, birth = `Dictionary`'s, `retirement` moves the dictionary out without its Drop hook (finding 1) |
| 41-42 | `WriterEditorSelection`, `WriterMainWindowTransient` | writer `…✒️main/🫧️transient/🧬️schema` | derive (impls removed from `📢️publication`) |
| 43-44 | `Generation3dViewTransientMutation`, `Generation3dPreviewWindowTransientMutation` | generation3d viewer / preview transient | derive |
| 45-46 | `BimPresence`, `BimPresenceMutation` | bim `👥️presence` | derive |
| 47 | `NoteCompositeWindowTransientMutation` | note `🪟️window` | derive |
| 48-51 | `SharedText`, `SharedChunks`, `TryValueContent`, `FormsTryValues` | forms Try `🫧️transient` | `SharedText` / `SharedChunks` and their cursors deleted; answer text is `SharedUtf8`; `TryValueContent` and `FormsTryValues` hand impls with exact birth and `SharedControlledRetirement::lease` of the shared `Arc`s |
| 52 | `LayoutDropPreviewState` | layout root | derive |
| 53-55 | `Puzzle2dWindowTransientMutation`, `Puzzle3dWindowTransientMutation`, `Puzzle5dWindowTransientMutation` | puzzle `✏️editor/🪟️window` | derive |

Totals: 55 before; 51 converted (22 derive, 4 `artifact_retire_struct!`, 7 hand with exact birth, 18 deleted wrappers: rows 12-27 and the two forms cursors); 4 stay `false` (rows 2, 8, 9, 10), pinned by the fixture below. `scanfalse.py` prints exactly those four.

## Gate that keeps the debt at four

`🧰️framework/🔨️modules/🌱️value/♻️retirement/🔓️uncontrolled-owners/`: `🧬️schema/🔣️.json` (draft-07), `🧫️fixtures/🔣️.json` (roots and the 4 allowed owners with reasons), `🧪️tests/🟦️.ts` (independent TS scanner over `git ls-files`, Ajv validates the fixture). Adding a fifth `false` impl, or removing one without updating the fixture, fails `bun test`. Not registered in the value `📜️script.ts` / `launch.json` yet (same as typed-close's structural-refusal twin).

## Value derive crate

No derive change was needed: none of the 55 was recursive or mutually recursive, and every non-derivable one is a type with a `Drop` hook or a foreign local owner (`StepParams`, `Block3dSnapshot`, `WiresSnapshot`, `FormsTryValues`, `MapDelta`, `CadReferenceIndex`), which the derive correctly refuses to move out of. One honesty gap remains and is deliberately untouched (as in `📓️2026-10-12-typed-session-close.md` §2): the derive emits `controlled_retirement_supported() -> true` whatever its fields declare, so a derived container of a `false` child still claims `true` (this is why forms/layout/puzzle containers were "controlled" while their children were not). Making it the AND of the field flags needs a coinductive guard for recursive types first; with the debt at four and pinned, only `IntoIter` could ever make it flip.

## Open / follow-ups

- `ArtifactChild<S>`'s retirement (`🏪️store/♻️retirement`) drops its typed local owner inline for every other snapshot with a composed child (writer, note, bim, … snapshots). Block 3d and wires lease it now; the general fix is to require `S: RetireOwned + Sync` there and lease the local owner once, then delete both per-owner leases.
- The runtime laws cover writer, block 3d, space, collection, wires, forms, cad, sequence, flow. bim, note, layout, generation3d, puzzle 2d/3d/5d, block 2d/5d and `TextError` are derive-only conversions proven by native + wasm32 + hub builds, not by a runtime law yet; the same two helpers apply unchanged.
- `IntoIter<T>`: a controlled iterator owner needs the backing capacity; migrate its users to `OriginalVectorCursor` and delete the impl.
- Cargo: `semio-framework-trace` added as dev-dependency (path) to block 3d, writer, wires, forms, cad, sequence; their `Cargo.lock` changed accordingly.

## Files touched

- value: `🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs` (docstring), `♻️retirement/🧪️tests/🔬️unit/🦀️.rs`, `♻️retirement/📋️queue/🧪️tests/🎮️ownership/🦀️.rs` (docstrings), new `♻️retirement/🔓️uncontrolled-owners/{🧬️schema/🔣️.json,🧫️fixtures/🔣️.json,🧪️tests/🟦️.ts}`.
- shared test helpers (new): `🧰️framework/🔨️modules/🧪️test/⚖️typed-input-law/🦀️.rs`, `🧰️framework/🔨️modules/🧪️test/⚖️transient-preparation-law/🦀️.rs`.
- replication: `📡️replication/🧬️retirement/🦀️.rs`, `📡️replication/🔗️causal/🔀️transition/🔁️fold/📏️retirement/🧪️tests/🦀️.rs`. diagnostic: `⚠️diagnostic/🦀️.rs`, `⚠️diagnostic/🧬️retirement/🦀️.rs`.
- os: `🛍️products/💻️os/🔨️modules/🌊️flow/🎒️mesh/🦀️.rs`, `🌊️flow/🧪️tests/⚖️retirement-law/🦀️.rs` (new), `🌊️flow/📦️packages/🦀️rust/Cargo.toml`; `🪐️space/🗿️artifacts/🪐️space/{🦀️.rs,🧪️tests/⚖️retirement-law/🦀️.rs (new),📦️packages/🦀️rust/Cargo.toml}`, deleted `🪐️space/🗿️artifacts/🪐️space/♻️retirement/🦀️.rs`; the same for `🗂️collection`; `🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`.
- `✏️s/🔌️plugins`: block shared `🧬️schema/🧱️shared/♻️retirement/🦀️.rs`; block 3d `🦀️.rs`, `🧬️schema/♻️retirement/🦀️.rs` (rewritten), `🧬️schema/🧬️mutations/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/{🦀️.rs,🧬️schema/🧬️mutations/🦀️.rs}`, new `✏️editor/🧪️tests/⚖️retirement-law/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, `Cargo.lock`; block 5d and 2d `🧬️schema/♻️retirement/🦀️.rs`, `🧬️schema/🧬️mutations/🦀️.rs`; cad `🧬️schema/📎️references/{🦀️.rs,🧪️tests/⚖️retirement-law/🦀️.rs (new)}`, `Cargo.toml`, `Cargo.lock`; wires `🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/🦀️.rs`, `✏️editor/…/🕸️canvas/🫧️transient/{🦀️.rs,🧬️schema/🧬️mutations/🦀️.rs,🧪️tests/⚖️retirement-law/🦀️.rs (new)}`, deleted `🧬️schema/♻️retirement/🦀️.rs`, `Cargo.toml`, `Cargo.lock`; sequence `🦀️.rs`, new `🧪️tests/⚖️retirement-law/🦀️.rs`, `Cargo.toml`, `Cargo.lock`; writer `…✒️main/🫧️transient/{🧬️schema/🦀️.rs,📢️publication/🦀️.rs,🧪️tests/⚖️retirement-law/🦀️.rs (new)}`, `Cargo.toml`, `Cargo.lock`; generation3d viewer transient mutations and preview transient `🦀️.rs`; bim `✏️editor/👥️presence/🦀️.rs`; note `✏️editor/🪟️window/🦀️.rs`; forms `…▶️try/🫧️transient/{🦀️.rs,🧪️tests/⚖️retirement-law/🦀️.rs (new)}`, `✏️editor/🎮️commands/🎯️set-try-value/🦀️.rs`, `Cargo.toml`, `Cargo.lock`; layout `🦀️.rs` and `…📐️blueprint/🫧️transient/🦀️.rs`; puzzle 2d/3d/5d `✏️editor/🪟️window/🦀️.rs`.
- This report and the `71.` entry in `📓️2026-10-10-migration-corrections.md`.

## Debug hygiene

No `[DEBUG]` line is left in committed files (the laws print nothing; their numbers above come from the runs before the prints were turned into assertions, which were then re-run green). Private `target` deleted.
