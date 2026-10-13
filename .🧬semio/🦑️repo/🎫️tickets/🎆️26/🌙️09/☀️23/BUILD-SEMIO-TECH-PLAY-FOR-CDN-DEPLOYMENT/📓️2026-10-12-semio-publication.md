# 2026-10-12 stdio-semio: RetainedClone snapshot trees and the document-lane publication authority

Executor: semio-pub (Sonnet). All cargo ran in the foreground through `🚦️cargo-slot.sh` with `SLOTS=2` and the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/semio-pub/target` (deleted at the end; logs stay in `play-fleet/semio-pub/`). Test binaries outside cargo ran with `RUST_MIN_STACK=268435456`, one process per test, 300 s timeout. No git-modifying command, no ticket operation, no sub-agent. Every source edit was a complete, compiling change (checked by `cargo check` after each wave).

## Result

| proof | before | after |
| --- | --- | --- |
| hub `editor_catalog`, 19 `semio_*_editor`, one process each | 0/19; 17 stop at the host-addressed detail action with `interactive-job.publication-authority-missing` ("unsupported artifact publication lane"), brep + mesh at the settle step | 0/19 pass, but all 19 now stop at the SAME shared point as the control editors `gltf_any`, `txt`, `md` (line 162, `detail action publishes`, "mounted normal retirement retains its original fault", cause `typed operation session retirement refused: UnsupportedOwner`) |
| `preparation_law` (new, stdio-semio): Apply / Undo / Redo through the editors' real `mutation_apply` factory | did not exist (the editors had no factory) | 5/5 at 03:05 (`semio-pub/prep2.log`: value x3 cases, document, text, flow). A re-run at 03:55 fails 5/5 inside the store law (`⚖️law/🦀️.rs:87`, close accounting (544, 0)) and so does the unrelated, untouched TXT `preparation_law`: a peer changed `⚖️law` (03:12) and `mutation-apply` (03:51, in-flight `[DEBUG]` edit) in between. See "Open". |
| stdio-semio `--lib --all-features`, 3026 tests, one process per test | 2919 / 3026 (`io-sqlite/new-full.json`) | **2920 / 3026**; per-test diff: **0 regressions**, 1 fixed (`cad ... step ... real_text_round_trip_through_step_codec`, a peer's change) |
| native `cargo check --lib` stdio-semio (`--all-features`) and value | - | Finished, 0 errors |
| `cargo check --lib --target wasm32-wasip2` stdio-semio and value | - | Finished, 0 errors |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-stdio-semio --target wasm32-wasip2` | - | Finished, 0 errors |

So the semio-specific blockers are gone; what remains for the editor tests is the one shared session-retirement fault (see "Open").

## What was hard (root cause, verified)

It was NOT the `RetainedClone` derive. Adding `semio_framework_value::RetainedClone` to every snapshot type compiled on the first attempt, including the recursive types (`SemioValue::List { items: Vec<SemioValue> }`, `DocBlock::Quote { blocks: Vec<DocBlock> }`, `DrawNode`, `SlideShape`): the derive's field cursors are lazily boxed (`RetainedFieldCursor<T>` holds `Option<Box<T::Cursor>>`), so a recursive cursor type has finite size. `ArtifactChild<S>`, `ArtifactLink`, tuples, `Option`, `Vec`, `String`, scalars all already have impls; nothing had to be added to the value crate.

The obstacle was one layer deeper and invisible until the authority existed: the crate root `🦀️.rs` region `🧹️SnapshotRetirement` held 20 HAND-WRITTEN `impl RetireOwned for ...Mutation` blocks (the 18 subset mutation aggregates plus `DocPathSegment` and `SemioValuePathSegment`). 18 of them lacked `controlled_retirement_supported()` (and `retirement_birth_bytes`); only `model` and `brep` had the full form. `mutation_apply_preparation_factory::begin_demand` (`🏪️store/📬️paged-one-item`) refuses with `structural clone originals require controlled typed retirement` whenever `S` or `M` is not controlled. With the authority added but the mutation not controlled, the failure was not reported as a fault: the fixture's `settle_registered_typed_operation` kept retrying the refused begin (12,058,999 iterations over 900 s of wall time in one measured run) until its deadline, which presented exactly like the shared "30 s settle" blocker of `📓️2026-10-12-stdio-editor-fixes.md` section 5. The same trap (a hand-written `RetireOwned` without controlled support) is therefore a candidate for the other editors that end at the settle deadline.

## Changes

1. `RetainedClone` on every snapshot tree of the 17 subsets (derive appended to the existing derive list, 89 types in 16 files): `✉️base` (`SemioSubsetSnapshot`, `SemioSnapshot`), `🌊️flow`, `🎞️animation`, `🎬️video`, `🏛️model`, `📊️table`, `📐️cad`, `📑️document`, `📦️object`, `📽️presentation`, `🔊️audio`, `🔢️value`, `🔤️text`, `🕸️graph`, `🖼️image`, `🧰️kit` (`🧬️schema/📸️snapshot/🦀️.rs`). mesh, drawing and brep already had it.
2. `#[retained_clone(bitwise)]` on the six `Copy` geometry records (`SemioPoint2/3`, `SemioUv`, `SemioRgba`, `SemioQuaternion`, `SemioTransform`; `✉️base/🧬️schema/🧮️geometry/🦀️.rs`): `Vec<SemioPoint3>`/`Vec<SemioUv>` (mesh positions, normals, uvs) now clone in 64 KiB bulk runs instead of one element per turn. The other `Copy` records (`SlideFrame`, `SemioRational`, `DrawCanvas`) are single values, or hold an `Option`, so the bulk path has no use there.
3. The 20 hand-written aggregate `RetireOwned` impls are deleted from the crate root and replaced by `#[derive(semio_framework_value::RetireOwned)]` on the enums themselves (`🧬️schema/🧬️mutations/🦀️.rs` of 18 subsets, plus `DocPathSegment`, `SemioValuePathSegment`). All 236 mutation leaf payloads already derive `RetireOwned`, so the derive gives controlled retirement support and exact birth bytes uniformly, and `model`/`brep` no longer differ from the rest. The two now-unused imports (`RetirementCursor`, `artifact_retirement_sequence as seq`) are removed.
4. Publication authority: `semio_s_artifact_stdio_contract::snapshot_editing_publication_authority!();` in the 17 `ArtifactEditor` impls that use `snapshot_editing_bounded_first_step_tool_proofs!` (`🏅️standards/🔖️v1/🪆️subsets/*/✏️editor/🦀️.rs`, before `fn command_id`): base, flow, animation, video, model, table, cad, document, object, presentation, audio, value, text, graph, drawing, image, kit. mesh and brep do not use that macro and are unchanged.
5. New runtime law `🧪️tests/⚙️preparation/🦀️.rs` (test target `preparation_law`, `required-features = ["component-app-assembly"]`, registered in `📦️packages/🦀️rust/Cargo.toml`): for each case the editor's own `build_artifact_store_one_item_preparation_factory()` drives `store::snapshot_clone_preparation::drive_one_item_preparation_law` (quoted grants only, receipts equal observed allocation under the global `HeapWitness`) for **Apply** (post equals a hand-built expected snapshot), **Undo** (ordered inverse rows replayed through the same factory restore the base) and **Redo** (same post and same inverse rows). Cases: value `set-value` at a 3-segment path through map/list/map, value `set-node`, value `set-value` at depth 64 (recursive clone, 3904 turns), document `set-block-content` inside a nested quote, text `edit-run`, flow `set-node-label`.

## Evidence

- Before: `before-semio.json` (scratch): 17 x `interactive-job.publication-authority-missing` at `editor-catalog/🦀️.rs:161`, `semio_brep_editor`/`semio_mesh_editor` at :162.
- Intermediate (authority added, mutation still uncontrolled): all 17 reach :162 but `semio_value_editor` runs 904 s with `settle wall_ms=900004.8 ... iter: count=12058999`; cause above. After deriving `RetireOwned` the settle returns in 214 ms.
- After: 22 editors run (19 semio + gltf, txt, md controls): every one fails at :162 with the identical `UnsupportedOwner` session fault, in 3-4 s each.
- `preparation_law` (03:05 run): `[DEBUG] semio document set-block-content apply turns=1160 undo rows=1 redo turns=1160`, text 519 turns, flow 550, value 842, value depth 64 3904; `test result: ok. 5 passed`.
- Lib per-process diff against `io-sqlite/new-full.json`: same 3026 names, 0 passed-to-failed.

## Open (not semio-specific)

- `preparation_law` of BOTH stdio-semio and stdio-txt currently fails in `drive_one_item_preparation_law` with `law close accounting disagrees with its receipt left: (544, 0) right: (0, 0)`: a close step allocates 544 bytes its receipt does not report. The store law and `mutation-apply/🦀️.rs` were edited by a peer after my passing run; re-run `cargo test --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml --features component-app-assembly --test preparation_law` once that edit settles.

- The hub editor tests of all editors (semio included) now end at `typed operation session retirement refused: UnsupportedOwner` raised by `WorkerJobSession::close_step` in the mounted typed-command session (`🔌️plugin/🦀️.rs` ~22984 / ~23202). It is the same for gltf, txt and md, so it is the old "settle" blocker with a concrete shape: some owner held by the mounted command session (the context residual of `📓️2026-10-11-plugin-tests.md` item 1 is the likely family) has no retirement path. Owner: plugin/job (pl-tests).
- `settle_registered_typed_operation` swallows a refused `begin` and spins until its deadline; it should surface the first refusal of `begin_demand` as the fault (it would have named this root cause in one second).
- 106 stdio-semio lib failures remain as before (R1/K1/S2 buckets of the earlier reports); none touches this change.
- `preparation_law` could be widened with one case per remaining subset; the harness is generic (`authoring_law::<Editor>`), only the expected snapshots are per subset. The `✉️base` editor needs a `pub` constructor for its wrapper leaves (`pub(crate) mutation` field) to be reachable from an integration test.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs` (20 retire impls removed, imports).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml` (test target).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/⚙️preparation/🦀️.rs` (new).
- `...🧿️semio/🏅️standards/🔖️v1/🪆️subsets/<subset>/🧬️schema/📸️snapshot/🦀️.rs` for ✉️base, 🌊️flow, 🎞️animation, 🎬️video, 🏛️model, 📊️table, 📐️cad, 📑️document, 📦️object, 📽️presentation, 🔊️audio, 🔢️value, 🔤️text, 🕸️graph, 🖼️image, 🧰️kit.
- `...<subset>/🧬️schema/🧬️mutations/🦀️.rs` for animation, audio, brep, cad, document, drawing, flow, graph, image, kit, mesh, model, object, presentation, table, text, value, video.
- `...<subset>/✏️editor/🦀️.rs` for ✉️base, 🌊️flow, 🎞️animation, 🎬️video, 🏛️model, 📊️table, 📐️cad, 📑️document, 📦️object, 📽️presentation, 🔊️audio, 🔢️value, 🔤️text, 🕸️graph, 🖊️drawing, 🖼️image, 🧰️kit.
- `...🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🦀️.rs` (bitwise).
- This report and the `69.` entry in `📓️2026-10-10-migration-corrections.md`.
