# PDF 1.4 canvas suite (`semio-s-artifact-stdio-pdf14-canvas`)

Counts: before 5 pass / 3 fail (the action-count test passed only after the fixture fix below, so three distinct failures), after 8 pass / 0 fail (`sh native-cargo.sh test -p semio-s-artifact-stdio-pdf14-canvas --lib`). The MuPDF oracle (`uv run --offline --with pymupdf==1.26.7 python -I 📜️pdf14-canvas-oracle.py check`) reports 26 probes, 0 failures, so the fixture itself is valid.

## 1. Action roster (22 vs 18)
Cause: stale expectation. The 1.7 page window already ends with `presence_action_definitions()` (four actions: `presence-pointer`, `presence-pointer-leave`, `presence-caret`, `presence-caret-clear`) and the tool-catalog join requires them; the 1.4 canvas window was given the same four. 8 canvas + 10 view + 4 presence = 22 is the right roster. Fix: the four ids were added to `actions` in `🧪️pdf14-canvas.json`; code unchanged.

## 2. MuPDF paint probes
Cause: not a paint-path divergence. The 1.4 canvas already paints through the shared `page::canvas_layers` of the 1.7 page editor, so it uses the same smoothing, raw-plane CMYK JPEG handling and `image_rgb` as 1.7; nothing is duplicated in production code. The 1.4 fixture holds text only (Helvetica, standard-14). The shared scene now paints the glyph outlines as one layer with a 6-value `transform` (outline units to canvas). The 1.4 test compositor ignored `transform`, so the glyph interiors tested as white (`[255,255,255] vs [0,0,0]` at `[106.4, 63.3]`). Fix: the test compositor now maps points through the inverse layer transform (same math as the 1.7 `paint-fidelity` compositor). The helper is a small test-local copy because `semio-s-artifact-stdio-pdf-page-editor` sits at its rust-source-boundary budget; a shared test-compositor crate is a follow-up.

## 3. "UI arena has no accepting lifetime registration"
Cause: `view_lane_reduces_and_searches_over_the_lifted_pages` built thumbnail and canvas trees without a registered arena. Fix: the test holds `FixtureUiArena::admit()` and retires every rendered tree with `observe_and_retire_fixture_tree(built_to_component_tree(..), |_| ())`, as the 1.7 tests do.

## Other
Removed the unused `Mutation` import in the canvas crate root.

## Changed files
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🔭️canvas/🦀️.rs` (import)
- same folder `🧪️tests/🔬️unit/🦀️.rs` (compositor with layer transforms, arena admission and retirement)
- same folder `🧪️tests/🔬️unit/🧪️pdf14-canvas.json` (22 actions)

## Regression group
See the section below.

### 1.4 editor group (`-pdf-snapshot-value-tests --lib -- editor::pdf14`, covers pdf14, pdf14a, pdf14x)
29 passed / 5 failed. No regression from this change (canvas tests are not mounted there and the failures are in code paths untouched here):
- `closes_mid_reserved_commit` for pdf14, pdf14a, pdf14x: framework lifecycle invariant `original lifecycle child receipt differs from its whole producer receipt` (value-ownership framework in flight).
- `history_edits_end_to_end` for pdf14a (`replace-page-text`) and pdf14x (`resize-page`): the history acceptance's seed edit on the harness' initial document targets page index 1 and is refused (`mutation.target-missing`, "Page target or geometry is outside the PDF 1.4 domain"); the committed fixtures themselves (two pages, index 1) are valid. The base `pdf14` `history_edits_end_to_end` passes. Not fixed here: needs the history harness to seed the fixture's `before` snapshot, or a one-page-valid derived edit for these two subsets.

## Follow-up: the five remaining 1.4 editor failures

Counts (`-pdf-snapshot-value-tests --lib -- editor::pdf14`): before 29 pass / 5 fail, after 31 pass / 3 fail. The three left are `closes_mid_reserved_commit` for pdf14, pdf14a and pdf14x (framework, see below).

### `history_edits_end_to_end` for pdf14a and pdf14x: fixed (fixture)
Neither suspect (one-based page numbers, Pack wire) was the cause. A `[DEBUG]` probe in `draft_time_travel_input` (removed again) showed the sequence for the committed `⏮️history` case: the harness' first candidate is `index` 2 (the leaf's index plus one). The fixtures targeted page index 1 of a two-page document, so 2 is out of range, the replay is Blocked, the trial is discarded, and the next candidate fails with `original preview cancellation must settle before another selected input` (`⏪️time-travel/🗄️store-state` `Preview` refuses a new selected input while the cancelled preview's owners are still unsettled). Every later candidate hits the same refusal, so the case is skipped and no other case exists in subsets a and x (the derived initial-document case is legitimately refused: one page, index 1). Base pdf14 passes because its committed round fixtures include leaves whose first candidate is in range.
Fix: the two committed history fixtures now edit page index 0 of their two-page documents (`replace-page-text` "Corrected archive reference" for a, `resize-page` 595.276 x 841.89 for x; `before`/`after` documents updated by hand), so the first candidate (index 1) is a valid page. Result: pdf14a and pdf14x `history_edits_end_to_end` pass (33 s).
Residual framework defect, not changed here: after a Blocked draft is discarded the harness' pump (`status none && !has_pending_work`) returns while the store's preview owners (`preview_job/derived/original/source/native_input/input_close/original_close`) are still non-empty, so the next draft in a new session is refused. Either the pump predicate or `has_pending_work` of the time-travel store should include the preview close state.

### `closes_mid_reserved_commit` (pdf14, pdf14a, pdf14x): framework regression, not PDF-specific, not fixed
- Filtered run `-- closes_mid_reserved_commit` in the same crate: all six 1.7 variants (pdf17, 17e, 17a, 17ua, 17x, 17h) pass, the three 1.4 variants fail identically. The XLSX crate does not mount this test in its lib (0 tests), so no XLSX result; the same panic is documented independently for the 3D puzzle editor (`☀️04/ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END/l31-3d-close-ceiling.md`, cluster 2) and the norm editors (`☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END`), with the same 4544 released bytes, so the cause is shared framework accounting, not the 1.4 Pack wire.
- Exact invariant and site: `original_lifecycle_unposted` (`🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs`), called from `original_lifecycle_step` after `advance`. With a `[DEBUG]` probe (removed again) for pdf14: whole step progress `{copied_items 1, copied_bytes 0, capacity 0, released 0}`; context receipt before `{3961, 1393047, 1365948, released 4}`, after `{3962, 1393047, 1365948, released 4548}`. The step posted 4544 released bytes onto the context but its returned progress reports released 0, so `whole - posted` underflows on the released axis (`later - earlier = 4544 > whole = 0`). The step is `VcsArtifactApp::close_step` (`🧵️instance/🔌️plugin-app-impl`, the `close_ladder_step` branch `PluginLifecycleStep::retained(step, terminal)`), reached from `close_registered_fixture_app` with a pending reserved undo (the reserved route of `undo`, `FRAMEWORK_RESERVED_JOB_KIND`). Some owner on the ladder frees 4544 bytes through the context but returns `Progress{copied_items: 1}` without that release. Which owner is not named by the symbols (the same gap the puzzle note records).
- Not touched: the framework ownership files are being edited by other agents, and `original_lifecycle_unposted` must not be loosened.

### Changed files
- `🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧪️tests/🧫️fixtures/⏮️history/🦠️mutation/🔣️.json`
- `🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧪️tests/🧫️fixtures/⏮️history/🦠️mutation/🔣️.json`
- (temporary `[DEBUG]` probes in `⏪️time-travel/🧵️instance/✍️draft-inputs` and `🏛️ownership/⚠️failure` were added and removed; no net change there)
