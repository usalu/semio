# Draw drawing native tests, 2026-11-23 (fourth pass)

Commands (cwd = drawing workspace `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`, `SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en`):
`repo cargo --store=isolated --configuration=prototype -- test --no-fail-fast -p <crate> -- --test-threads=1`.
Crates were run singly or in small batches, because one compile error in any selected crate aborts the whole `cargo test` invocation.

## Environment findings (not Draw bugs)
- Source admission: the PDF page editor base crate was over its 262144 byte limit because another agent had added a 250 KB binary grid next to its Rust source. Fixed by moving the grid into its own crate (see below).
- The framework was repeatedly uncompilable for minutes at a time while other agents edited it (`fund_retirement` in the job testing caller, `original_receiving_scope_held`, `last_publication_wait(&u64)`, a transient missing mod file, the PDF editor-document manifest). All were retried and resolved by their owners.
- `semio-plugin-testing-laws` was 31 bytes over the source boundary because of another agent's temporary `[DEBUG]` thread-local; it was removed by that agent.
- ENOSPC occurred three times. `repo cache gc` was run (once cancelled by my own timeout, later run to completion). The Rust cache image was lost: my manual `hdiutil attach` of the stale image replaced the root-owned mount directory, which the tool then refused ("Original underlying mount directory changed"). I removed `.cache/b/r/fixed.dmg`, `image.json` and the stranded directory, as the tool message prescribes, and the tool recreated the image (26 GiB instead of 70 GiB, sized from the free disk at that moment), which is again tight for a full workspace build.

## Per crate (last confirmed run)
| crate | pass / fail | state |
|---|---|---|
| semio-s-artifact-draw-drawing-structure-tests | 34 / 0 | green |
| semio-s-artifact-draw-drawing-style-tests | 47 / 0 | green (was 31 / 16) |
| semio-s-artifact-draw-drawing-transform-tests | 56 / 0 | green (was 42 / 14) |
| semio-s-artifact-draw-drawing-scene-tests | 48 / 0 | green (was 42 / 6; the six heap-accounting failures were another agent's allocating `[DEBUG]` code, since removed) |
| semio-s-artifact-draw-drawing-editor-commands-tests | 30 / 0 | green (was 28 / 2) |
| semio-s-artifact-draw-drawing-editor-interaction-tests | 37 / 3 at last confirmed run (was 11 / 29) | see Remaining |
| semio-s-artifact-draw-drawing-editor-tests-2 | 30 / 1 | `export_work_tests::original_export_read_returns_under_exact_physical_grants` (export path, see Remaining) |
| semio-s-artifact-draw-drawing-editor-tests | `archive_load` 0 / 1; rest aborts (SIGTRAP) | see Remaining |
| semio-s-artifact-draw-drawing-examples | not run to a result | a transient framework compile error stopped it; not retried successfully |
| semio-s-artifact-draw-drawing-host-tests | does not compile (27 errors) | see Remaining |
| semio-s-artifact-draw-drawing-model-tests | not compiled this pass | see Remaining |
| semio-s-artifact-draw-drawing-host (companion in-crate laws) | n/a | |

## Root causes fixed
1. Source admission (PDF page editor base, 356 KB): `🧮️function-colour/🌈️cmyk-image-clut.bin` moved into the new crate `semio-s-artifact-stdio-pdf-cmyk-image-clut` (`🧮️function-colour/🌈️cmyk-image-clut/`, with its generator script); `function-colour` now imports `GRID`/`TABLE` from it. New dependency in the page editor base `Cargo.toml`.
2. Stale committed diff fixtures (style and transform subsets, 17 files): the `DrawingLayerPatch` schema gained the `imageKey`, `imageWidth`, `imageHeight` lanes and no longer carries `fontFamily` on non-text layers, and the typed diff encodes every number as an f64 (`20.0`). The committed JSON still had `fontFamily: "anta"` on shape layers (which the apply refused with "Text target has another kind"), lacked the image lanes and wrote integers, so produce / canonical / apply laws failed. Fixed by `canonicalize-diff-fixtures.py` (ticket folder), preserving CRLF.
3. `update_text` inverse fixture was a single object, the inverse API returns a list: wrapped in an array.
4. `update_image` test compared an f64 JSON against an integer JSON: compared numerically.
5. Native encoding workload (real bug): `admit_clone_identities` (`io/text/identity/clone`) and `prepare_paste_identity` (`.../paste`) stepped the shared `NativeEncodeControl` after the previous identity stage had already consumed its declared workload ("native encoding exceeds declared workload"). Each now begins its own stage first. This broke duplicate-layer and every clipboard paste of more than one identity.
6. Clipboard native JSON encoder dropped `fontFamily` of text layers (the serde form includes it) and the hydration whitelist rejected it: both now carry it.
7. `changes_layer` (patch-layer): the derived `ColdMutation::inverse` returns an empty list for a no-op, which the command treated as a missing target ("The layer field target is missing"). It now asks the new `drawing_mutation_is_refused` (mutations crate, semantic admission) to tell a refused missing target from an admitted no-op.
8. Interaction tests: panel tests built UI values without the registered UI arena ("UI arena has no accepting lifetime registration"); every panel test now holds `FixtureUiArena::admit()`. Clipboard tests: documents without an id (identity preimage needs a nonempty document id) got `id: "document"`; `PagedMap` collected via `try_from_entries`; `default_layer_base` import; asset JSON compared as values, not as key-ordered text.
9. Framework (clearly broken): `PagedUtf8` had no `FromValue::from_value_controlled`, so every controlled layer decode of the clipboard hydration ("base.id.value owner has no controlled native construction implementation") failed. Implemented through the existing `try_from_str_controlled`. NOT yet confirmed by a test run (see Remaining).

## Files changed (repo-root relative; Draw root = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets`)
- Draw root `/✳️any/🚪️io/📝️text/🪪️identity/📋️clone/🦀️.rs`, `/📋️paste/🦀️.rs`
- Draw root `/✳️any/✏️editor/📋️clipboard/🛫️encode/🦀️.rs`, `/🛬️decode/🦀️.rs`, `/🧪️tests/🔬️unit/🦀️.rs`
- Draw root `/✳️any/✏️editor/🎮️commands/🩹️patch-layer/🦀️.rs`, `/✳️any/🧬️schema/🧬️mutations/🦀️.rs` (new `drawing_mutation_is_refused`)
- Draw root `/✳️any/✏️editor/📌️panels/🗂️layers/🧪️tests/🔬️unit/🦀️.rs`, `/📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs`, `/✳️any/✏️editor/🕹️interaction/📌️panels/🗂️layers/...` (tests only)
- Draw root `/🎨️style/🧬️schema/🧬️mutations/🖼️update-image/🧪️tests/🔬️unit/🦀️.rs`
- Fixtures: all `🔺️diff/🔣️.json` under `/🔀️transform/🧪️tests/🧫️fixtures/🧬️mutations/*` and `/🎨️style|🏷️metadata/...` that `canonicalize-diff-fixtures.py` reported; `/🎨️style/🧪️tests/🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/↩️inverse/🔣️.json`
- Framework: `🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs` (`PagedUtf8::from_value_controlled`)
- PDF page editor base (admission only): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🧮️function-colour/🦀️.rs` (import), `.../🧱️model/📦️packages/🦀️rust/Cargo.toml`, new crate dir `.../🧮️function-colour/🌈️cmyk-image-clut/`
- Ticket folder: `canonicalize-diff-fixtures.py`, `🗑️generated/*` logs and run scripts
All `[DEBUG]` code I added was removed, except the ones named under Remaining if the final run did not finish (listed there).

## Remaining and why
All `[DEBUG]` code I added is removed (checked: no `[DEBUG]` remains in the Draw tree).

1. editor-interaction-tests: three tests failed at the last confirmed run.
   - `clipboard::...hydration_preserves_admitted_identity...` and `...native_codec_matches_serde_and_resumes_large_text`: root cause identified (item 9, `PagedUtf8` had no controlled decode); the fix is written but the verification run died with ENOSPC. Unverified.
   - `panels::properties::...inspector_controls_bind_the_events_the_host_dispatches`: after the arena guard the failure moved to "UI arena descendants remain borrowed at lifetime closure". This test renders a gradient path with an open two-row tree window twice (EN, DE) and retires the `ViewModel` last; a UI collection created by that render is not retired. Not diagnosed further (needs one run with per-node arena accounting).
2. editor-tests-2: `export_work_tests::original_export_read_returns_under_exact_physical_grants` (test file lives in the export-document path that another agent owns; not touched). The store disposer never reaches terminal-empty: `SnapshotReadLeaseRegistryOwner::close_custody_step` waits for `strong_count() == 1`, so something still holds a registry alias after the work and the returned reads are gone.
3. editor-tests: the demo `archive_load` test no longer stalls in worker-session admission (8 s, then fails): `published_load` ends its loop with `has_pending_typed_operations() == false` without ever seeing `Effect::LoadDocument` ("the example publishes a document load"). The typed-operation publication path (`artifact-app-laws` fixture dispatch) was being rewritten by another agent while I ran. The other 81 tests of that crate cannot be judged: a failing test leaves a controlled retirement in a thread-local and the process aborts with SIGTRAP at the next test (`--test-threads=1` runs all tests on one thread). Run them with a filter excluding `archive_load` once it is fixed.
4. host-tests (27 compile errors, test code only, stale since 20 Nov): the retained mutation authority tests still call `authority.step(&mut DrawingSnapshot, &DrawingMutation, cx)`; the production `step` takes `DrawingSourceAccess` (currently `pub(super)`, so a companion crate cannot name it) plus a `RetainedCloneRef<DrawingMutation>`, which needs a `FixtureSource<DrawingMutation>` per mutation. Other drift: `owned::initialization_turn` no longer exists, `PagedList::reserve_one` takes an effect (`reserve_one_cold(usize)` is the cold form), `close_step` takes no argument, `DrawingNativeCloneAuthority` has no `index` field, `create_drawing_boolean_layer` takes a `PagedList`, `RenameLayer.layer_id` is a `PagedUtf8`, and `arena-laws` borrows `preview_sequence` twice. This is a port of about 160 KB of heap-accounting laws and needs the new step API exposed first; not started.
5. model-tests: not compiled this pass. The previous pass listed 71 errors of the same kind (removed APIs, lost private names, dev-dependencies); the test file `🧬️schema/🧪️tests/🔬️unit/🦀️.rs` has not been touched since 20 Nov.
6. examples: its run stopped on a transient framework compile error (`fund_retirement` in the job testing caller) and was not repeated with a result.
7. Rust build volume: after the cache image was recreated it is 26 GiB and the concurrent builds of several agents fill it (ENOSPC at about 14 rustc processes); every long run of mine after 22:55 either failed or was slowed by this. `repo cache gc` frees it only temporarily.
