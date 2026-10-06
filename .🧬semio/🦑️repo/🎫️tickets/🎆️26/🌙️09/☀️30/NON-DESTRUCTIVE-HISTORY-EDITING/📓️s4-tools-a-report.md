# 📓️ S4-TOOLS-A — draw, note, layout, fem 2d/3d, lowpoly, shooting (session 4)

Successor of S3-DRAW (`📓️w3-t-draw-note-report.md`), S3-LAYOUT (`📓️w3-t-layout-report.md`) and S3-SPATIAL (`📓️w3-t-spatial-report.md`)
on ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Scratch: `🗑️generated/s4-tools-a/`. Private test target
`⚡️cache/cargo/target-nde-s4-tools-a`.

## Session 4 — 2026-10-04

### S4.1 Repair-first (rule 34), 02:10

- `git diff HEAD --stat` over the owned trees: draw 310 files, note 197, layout 277, fem 562, lowpoly 138, shooting 169, hub draw 2,
  `📐️Canvas2dHost` 2, `🌐️World3dHost` 7, `♾️infinite/🌍️world` 12 (HEAD = auto-commit 10-02 17:04, so this is all session-3 work plus peers).
- Files newer than the session-3 hand-overs (10-03 11:00) in owned trees: all peer waves — the value/DSL extraction (every plugin
  `Cargo.toml` 01:19 gains `semio-framework-dsl{,-record,-record-derive}`, `-async`, `-pack-json`, `-diagnostic`; note sources 00:32;
  layout `🧬️mutations/📝️text` 00:55), a peer's draw `📷️raster` + `🎨️fill/🎨️sampling` retirement/coverage work (01:11–01:29) and a
  peer's `🌍️world` analytic-vertex-picking / component-contract (00:34–01:28). None is a half-finished S3 edit; the three S3 hand-overs
  declare no edit in flight. Nothing to repair before compiling.

### S4.2 Verification log

| time | command | result |
|---|---|---|
| 02:23–02:45 | `cargo check --manifest-path ✏️s/Cargo.toml -p …draw-drawing -p …note-note -p …layout-layout -p …fem-2d -p …fem-3d -p …lowpoly-lowpoly -p …shooting-shooting --features …fem-2d/component-app-assembly,…fem-3d/component-app-assembly --lib --tests --keep-going` (`check-1.txt`, load 48) | **BLOCKED by peers before my crates**: `semio-framework-plugin` lib 7 errors (`🔌️plugin/🦀️.rs:44976/44982` `AppCommand::ReadChildHeads`/`AppFrame::ChildHeads` missing, `:45207` `PureCommand.head`, `:27719` arity — channel/W-b wave in flight; `⏯️tool-run/🦀️.rs:1538/1582` `ToolRunEntry.member`, `ToolRunJobRequest.children/member_ops` — child-member wave in flight), `semio-framework-artifact-playbook-playbook` 2 errors (`📖️playbook/…/🦀️.rs:625,635` `UiTreeItemAction.reason` — W1E-1 in flight). 0 errors in owned files (none reached) |

| 03:11 | `cargo check -p semio-framework-tool-machine --lib --tests` (`check-tm-2.txt`; 1st try: my test `CountingTool` collided with the test file's own `GestureTool` → `impl super::GestureTool`) | **PASS, 0 errors, 0 warnings in tool-machine** |
| 03:12 | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-tools-a cargo test -p semio-framework-tool-machine --lib` (`test-tm-1.txt`) | **36 passed, 0 failed** (33 + 3 new gesture laws) |
| 03:0x | LA pkg `bun ./📜️script.ts verify layout-frame-selection` / `layout-window-ownership` / `layout-document-contract` | **4 pass / 0 fail (65 expects)**; exit 0; exit 0 (90 native snapshots, 39 diffs) |
| 03:0x | `.venv/bin/python T/🧪️s3-layout-path-paint-corpus.py --check` / `T/🧪️s2-layout-gumball-dispatch-corpus.py --check` | **current** (16 cases, 68 probes) / **current** (23 cases, 20 handles, 33 dispatches) |
| 03:0x | `.venv/bin/python T/🧪️w3-t-layout-author-vectors.py` | was **28 files pending** (peer drift: layout diff `dataFieldsJson` → typed `dataFields` at the end, fallible `inverse()`); my input script updated (`DIFF_KEYS`, 3 `inverse(..).expect(..)` templates) → **17 vectors, 0 pending** |
| 03:15 | React pkg (`⚛️react/📦️packages/🟦️typescript`) `bun ./📜️script.ts test long Canvas2dHost` (`vitest-canvas2d-1.txt`) | **8 files, 108 passed** (after the overlay move) |
| 03:16 | same `test long engine-contract -t "worldGumballStep|worldPaintStep|gumball"` | **13 passed** (687 skipped by the filter) |
| 03:17 | same `test long World3dHost` | **4 files, 17 passed** |
| 03:18–03:22 | `bun ./📜️script.ts verify taxonomy report --scope` CH / `🏗️fem/⚙️engine/🖥️app-surface` / new fem 2d corpus dir / fem 3d `📊️results` | CH **16 → 7** errors (all 9 overlay findings gone; 7 pre-existing `directory-kind-unresolved` peer dirs); app-surface 1 pre-existing (`👁️result-mode`), my `🔣️results-animation` clean; corpus dir `🫧️mutate-…` unresolved → renamed to the open pattern `🧪️mutate-fem-2d-1-any-editor-edit-results-transient` (oracle comment updated); fem 3d `📊️results` 6 pre-existing (`🪪️document-contract` unresolved + path-too-long) — the deleted transient subtree added none |
| 03:51 | owned ✏️s check (`check-3.txt`) after TREE GREEN | **618 peer-migration reds in my crates** (sqlite ABI ≈ 420, `dsl::json` ≈ 90, private re-exports …) → S4.4b |
| 04:2x | same (`check-4.txt`) after the fallout script | 159 (32 sqlite → INFRA, 127 non-sqlite → hand fixes S4.4b) |
| 07:1x | bun: fem 2d + fem 3d results-window config document-contract TS (Ajv strict, shared `results-animation` `$ref`) | **2 pass, 0 fail** |
| 07:1x | strict `tsc --noEmit` over the shared `results-animation`, both config TS schemas and the transient TS | **exit 0** |
| 06:46 / 07:17 / 07:4x | same owned check (`check-5/6/7.txt`) | infra deaths: SIGKILL after a 27-min lock wait; `failed to write …/build/<unit>/fingerprint/invoked.timestamp` (build dir pruned under the run at disk 1–5 GiB) |
| 07:59 | same (`check-7.txt`) | 27 errors → hand fixes (note `👥️presence` close_step, note `ui_label` → `semio_framework_ui_contract::Label`, note pdf test `serialize(&note, &ArchiveChildren::empty())`, shooting test `from_dsl_value`, fem 3d `close_step`/`add-combination` policy/`set-result-display` dirty scope via `Fem3dResultsPlayback`); draw `🧰️owned:5825` E0061 = S4-STORE's in-flight store signature change (theirs, green at 08:29 per S4-STORE) |
| 08:12–08:27 | same (`check-8.txt`, `--lib --tests`) | **all 7 libs PASS natively (0 errors)**; 7 errors left, all in sqlite TEST files → fixed on S4-INFRA's request (fem 2d/3d `restore()` → `ValueError`, shooting `.map_err(to_string)` dropped, note guest `FromValue` from `semio_framework_value`) — test targets not re-checked since: rule 43 forbids `--tests` builds |
| 08:3x / 11:35 / 11:44 | `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-{draw,note,layout,fem,lowpoly,shooting} --lib --keep-going` (`wasm-1/2/3.txt`) | 1st SIGKILL (cut), 2nd infra (`semio_framework_schema_derive` rlib missing / build-dir unit pruned, disk 7.6 GiB); 3rd (12:11–12:4x) reached `Checking` fem 2d / note / shooting / lowpoly / draw with **0 errors** before it was stopped (background cap) — not a pass; then RULE 44 CARGO FREEZE → **COMPOSITION GREEN still OWED** for all six |

### S4.3 D24 — layout gumball overlay home + `verify layout-frame-selection` target (02:50, source)

- Taxonomy (hot shared, Edit tool, one anchor): `🧭️gumball` added to `members-of-members-of-elements.memberNames` (beside
  `🗂️local-catalog`, `📎️local-folders`; precedent `🏛️ShellHost/📎️local-folders/🟦️.tsx`).
- `CH/🟦️GumballOverlay.tsx` → `CH/🧭️gumball/🟦️.tsx` (plain `mv`), its `CanvasCamera` import → `../🟦️.tsx`; importers
  `CH/🟦️.tsx:19` and `CH/🧪️tests/🧪️gumball-dispatch/🟦️.ts:26` repointed; the wgpu twin's 3 doc references updated. The wgpu twin
  itself stays `CH/🎯️targets/🧊️wgpu/🦀️.rs` (the element's wgpu target; the 10-04 taxonomy report shows 0 findings on it, nested
  `🎯️targets` under a member has no precedent). Before: 16 errors on CH, 9 of them on the overlay (kind-only-basename,
  semantic-stem-unresolved, 5 normalization collisions with `🟦️.tsx`, 2 reference-edit-required); the 7 remaining
  `directory-kind-unresolved` (`🧪️tests|🧫️fixtures/{👕️peer-presence,🖊️path,🖱️gesture-sample-lane,🖱️input-contract}`) are not this WP's.
- LA `📋️project.json`: new nx target `verify-layout-frame-selection` (`bun ./📜️script.ts verify layout-frame-selection`, metadata
  `workspaceCommand ["verify","layout-frame-selection"]`) beside the two existing verify targets — launch row = coordinator.

### S4.4 D14 — one FEM playback clock + shared streamed-gesture runner (02:50–03:40, source)

**One playback transport (no twin clocks, no twin transport types).** Decision: the clock, its transient, the leaf AND the transport
record are shared, so the `FemPlaybackTransport` the brief names is the per-window-kind plumbing trait (config ↔ animation, owners,
dirty scope), not a bridge between twin config types (which would have needed a third loop-mode enum).
- NEW shared schema `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🧬️schema/🔣️results-animation/{🔣️.json,🦀️.rs,🟦️.ts,🔗️.graphql,🛰️.proto}`
  (`$id …/fem/results-animation/schema.json`, beside `👁️result-mode`): `FemLoopMode`, `FemWaveform`, `FemResultsAnimation`. App surface
  `🦀️.rs` region `⏯️Playback`: the 5 transport constants, `Default`, `amplitude`, `start`, `FemLoopMode::key`/`FemWaveform::key`.
- Both results-window configs (Rust/JSON/TS/graphql/proto) drop their `Fem{2,3}dLoopMode`/`Waveform`/`ResultsAnimation` twins and
  reference the shared record (`animation: crate::app_surface::FemResultsAnimation`, JSON `$ref` to the shared `$id`); both
  document-contract TS tests register the shared schema; the 3 animation laws moved from the fem 2d config tests to the app-surface tests.
- Clock: fem 2d `📊️results/🫧️transient` now holds THE `FemPlaybackClock`, `FemResultsWindowTransient` (`fem.resultswindowtransient`),
  `FemResultsWindowTransientMutation`/`SetPlaybackClock` (`$id …/s/fem/results-window/transient/…`), `owners()`, the generic
  `addressed_to::<O>`/`captured_clock::<O>`/`required_clock::<O>` and the marker trait `FemResultsWindowTransientOwner`. fem 3d's
  `🫧️transient/🦀️.rs` is only `Fem3dResultsWindowTransientOwner` (`fem3d-results`) + explicit re-exports.
- Commands: fem 2d `⏯️set-result-animation` = THE `SetResultAnimation`, `apply_field`, `merge`, `FemPlaybackTransport`, `FemPlaybackStep<M>`,
  `rearm_effect::<T>`, `resting_step::<T>`, `set_result_animation_step::<T>` + `Fem2dResultsPlayback`; `⏱️result-animation-tick` = THE
  `ResultAnimationTick` + `result_animation_tick_step::<T>`. fem 3d's two command modules shrink to `Fem3dResultsPlayback` + `handle`/
  `handle_window` + re-exports. Both editors drop `fem{2,3}d_captured_clock` for `required_clock::<Owner>`. Fault codes of the shared
  code are `fem.result-animation*` (were `fem2d.`/`fem3d.` twins).
- Feature corpus moved to the owner of the lane: NEW `◻️2d/…/🌐️any/🧪️tests/🫧️mutate-fem-2d-1-any-editor-edit-results-transient/` (the fem 2d
  oracle already claimed it — it was missing); bridge `🏗️fem/🏭️bridge/🦀️.rs` measures `FemResultsWindowTransientMutation` on the s.fem.2d
  surface coordinate (fem 2d feature `component-app-assembly` explicit in its `Cargo.toml`).
- Parity fix found on the way: the fem 2d results PANEL drew the resting phase while playing (fem 3d's folds the running clock);
  `results_panel::render` now takes `clock` and draws `config::effective(window, clock)` like fem 3d (editor passes the captured clock).
- Tests: fem 2d command/transient/window/panel tests renamed onto the shared types; fem 3d keeps its mounted-app integration laws,
  its pure-logic duplicates are deleted (`set_result_animation_refuses_unknown_fields_and_values`, `rearm_effect_addresses_the_window_at_the_frame_delay`,
  `long::result_animation_frame_cost_stays_flat_across_a_long_run` — each still runs in fem 2d).
- **Deleted (rule 32; only self-references, fem tree grep clean):** fem 3d `…/📊️results/🫧️transient/{🧬️schema,🧫️fixtures,🔮️oracles,🧪️tests}/`
  (21 files: schema ×5 + mutations json/rs + leaf rs/json/schema, fixtures ×7, oracle, unit tests) and
  `🧊️3d/…/🌐️any/🧪️tests/🫧️mutate-fem-3d-1-any-editor-edit-results-transient/` (feature + adapter).

**Shared streamed-gesture runner (FEM gumball ≡ lowpoly paint).** `🛠️tool-machine/🦀️.rs` new region `🌊️Gesture`: `GesturePhase` (+`parse`),
trait `GestureTool` (start/resume/verb/base_revision/abort/send/persist), `GestureDrive<G, M>`, `drive_gesture::<T>` — the one control flow
(abort drops, moved base drops / one-shot commits fresh, verb switch or one-shot interrupts `captureLost`, change detection). FEM:
`FemGumballPhase`, trait `FemGumballTool` and the drive body deleted; `fem_gumball_drive` maps `drive_gesture` onto the transient;
`Fem2dGumballTool`/`Fem3dGumballTool` implement `GestureTool`. Lowpoly: `LowpolyToolPhase` and the drive body deleted, `LowpolyPaintTool`
implements `GestureTool` (`LOWPOLY_PAINT_VERB`), `lowpoly_paint_drive` maps onto `drive_gesture`. New laws (tool-machine unit tests,
region `🌊️GestureLaws`): `gesture_phases_parse_their_wire_words`, `a_streamed_gesture_accumulates_and_commits_once`,
`aborts_moved_bases_and_interruptions_drop_the_open_gesture`. Follow-up (not my tree): gen3d `GumballGestures` / CAD streamed gumball
can adopt `drive_gesture` the same way.

### S4.4b Peer fallout in my crates (coordinator 03:52 "migrate it yourself"; sqlite ABI handed to S4-INFRA at 04:05)

The 03:51 check showed 618 reds in my crates, all from the value/DSL/pack extraction. Idempotent input script
`T/🧪️s4-tools-a-peer-fallout.py` (dry run by default, `--apply` writes, skips files someone else touched < 30 min ago unless they are in its
own ledger `🗑️generated/s4-tools-a/fallout-ledger.txt`), in the peer's direction only:
- `dsl::json::*` / `store::json::*` → `semio_framework_pack_json::*` (2 320 sites; the 1-argument `from_json_str`/`parse` gain
  `JsonMemberPolicy::Reject`, as the peer's migrated trees do); `use dsl::json;` → `use semio_framework_pack_json::json;` (macro) with
  `json::x` → `semio_framework_pack_json::x`;
- kernel-private re-exports `protocol|dsl|store::{ValueError, DslValue, ToValue, FromValue}` → `semio_framework_value::…`;
  `semio_framework_plugin|protocol::{Terminology, Locale}` → `semio_framework_ui_locale::…`; grammar API `::dsl::{parse_grammar, SemioDialect, …}`
  → `semio_framework_dsl::…`;
- `IoError { message, diagnostics }` literals → `IoError::from_value_error(ValueError::new(InvalidValue, …))`; 2-argument
  `TextError::new(msg, span)` → `TextError::new(InvalidValue, msg, span)`.
- Its `migrate_sqlite` stage rewrote 7 `🪶️sqlite/🦀️.rs` (+ `🏗️fem/🧩️sqlite`) files before the 04:05 ownership change; the stage is disabled
  now (`SQLITE_TARGETS = []`) and S4-INFRA (a1efcc6feb102bd39) was told which files and got the 32-line residue
  (`🗑️generated/s4-tools-a/sqlite-residue-4.txt`).
Hand fixes: draw `🕹️nudge-selection` derive paths, `👁️view` imports + its test's private `canvas_window` path; layout `💡️inferences`
`ValueError`, `🧾️dictionary` `native_decoding::NativeDecodeProgress`; fem 2d + lowpoly `close_step` → `ValueError` (3 impls, 3 `Err` sites typed
`InvariantViolated`); fem `🧮️analyses` `PagedListError.reason`; shooting `📝️text` `parse_op` via `TextError::from_value_error(error.under(…))`;
`MediaPayload::Intrinsic` arms in fem 2d / shooting tests; note png import/export and lowpoly png test onto `semio_framework_pixels`
(`decode_png`/`encode_png(RasterImage)`; note drops its `semio-s-artifact-stdio-png` dependency for `semio-framework-pixels`); my own D14 test leftovers
(`Fem2dResultsWindowTransientOwner::build_owners`, `result_animation_tick_step::<Fem2dResultsPlayback>`).

### S4.4c Fault notices (S4-GATES routing 03:50, design §20.12)

Idempotent input script `T/🧪️s4-tools-a-notices.py`: tool-mismatch refusals → framework `app.command.tool-mismatch` (draw bounded + gesture +
viewer, note retained, shooting retained — 5); named + en/de `fault_notices()`: draw `drawing_fault_notices()` (9 `drawing.gesture.*`), note
`note_fault_notices()` (4 `note.ink-*` + `note.retained.transaction`), fem `fem_fault_notices()` in fem 2d `🎮️commands/🧭️gumball` (8 codes:
`fem.gumball.{phase-unknown,transient-context-required}`, `fem.gumball-flag.{window-context-required,window-required,window-stale,window-kind}`,
`fem.canvas.{window-required,window-kind}`) declared by BOTH fem editors; the 10 anonymous `Fault::from("fem2d|fem3d.gumball…")` and the 2
`canvas-gesture` `format!` faults are now `Fault::new(App, FaultCode::new(…), …)` (2d/3d twins share the `fem.` codes).
Gate 11:55 `bun ./📜️script.ts schema fault-notices --scope history-editing` (cwd `🦑️repo/🔨️modules/🧪️test`): my plugins' only findings are
`faultNoticeDescriptor: describe owed` for draw / note / fem (the new tables are unpublished until `describe`) — shooting 0; the 1 + 12 + 6 +
12 S4-GATES rows of 03:50 are otherwise closed (38 findings scope-wide remain, other owners).

### S4.5 Open items

- **L1 (audit minor) — kept as documented prose, not corpus cases.** The four wgpu approximations (group opacity per piece, rotated
  images/text axis-aligned, ≥ 1 px strokes, raster quads in a later pass) live in `render_canvas_scene_node` (scene level: groups,
  images, text), while the path-paint corpus is single-path geometry whose every probe keeps `margin` px from every boundary so "any
  faithful rasterization agrees" — an `approximate: true` case would contradict that contract. Pinning them needs a separate
  scene-level parity corpus (React vs wgpu screenshots) — follow-up, not done. `gl-matrix` devDependency of the React package: GATES.
- **Z4 (audit minor) — corpus side already present.** `🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json` carries 4 local one-shot cases
  (`live: false`: net delta on release, unmoved → nothing, turn under a move handle → nothing, no targets → skip). What remains is a puzzle
  guest replay law over those local cases (puzzle crate, S4-PUZZLE's tree) — handed over via `main`.
- **OWED (rule 44 — cargo frozen until "CARGO OPEN")**: `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-draw -p semio-hub-note -p semio-hub-layout -p semio-hub-fem -p semio-hub-lowpoly -p semio-hub-shooting --lib --keep-going` → one "COMPOSITION GREEN <plugin>" per passing composition to `main`;
  (native libs are verified: the 08:27 run already included the fallout and notices edits; only the 4 sqlite TEST files changed since → item 1 below).
- **OWED (rule 43 — no test builds until "TESTS RESUMED")**, private target `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-nde-s4-tools-a`, `CARGO_INCREMENTAL=0`:
  1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-{draw-drawing,note-note,layout-layout,fem-2d,fem-3d,lowpoly-lowpoly,shooting-shooting} --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib --tests` (the 7 sqlite-test fixes + fallout in test targets);
  2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --lib`; 3. `… -p semio-s-artifact-note-note --lib`;
  4. `cargo test -j 2 --manifest-path ✏️s/Cargo.toml --no-fail-fast --lib -p semio-s-artifact-layout-layout -p semio-s-artifact-lowpoly-lowpoly`;
  5. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-shooting-shooting --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib --no-fail-fast` (D14 laws: shared clock/transient/leaf, both mounted playback suites, `fem_gumball_drive` laws, notices);
  6. `cargo test -p semio-framework-plugin --lib -- use_selection selection_value reference_chips`;
  7. `RUST_MIN_STACK=67108864 cargo test -p semio-framework-os-renderer-wgpu --lib -- canvas2d` and `-- paint2d` (overlay move touched only docs of the wgpu twin);
  8. `cargo test -p semio-framework-os-infinite --lib -- gumball paint cancel world`;
  9. the fem feature corpus `🧪️mutate-fem-2d-1-any-editor-edit-results-transient` through the repo test host, and `bun ./📜️script.ts test inventory` for fem (bridge coordinate).

### S4.5b AUDIT-TOOLS items (`📓️audit-s4-tools.md`, resumed 13:14, PARKED 13:2x on coordinator order)

**Done (source, my crates; cargo OWED under rule 44):**
- **F9:** the 5 raw tool-mismatch codes now raise `Fault::new(App, FaultCode::new("app.command.tool-mismatch"), …)`: fem 2d editor
  (`fem2d-command-tool-mismatch`), fem 3d editor (`fem3d-…`), lowpoly editor (`lowpoly-…`), layout editor ×2 (`layout-export-…`,
  `layout-command-…`); no test asserted the old codes. Fully qualified paths, so no import changes.

**Found at park (13:2x); items 1-4 are resolved in S4.5c, 5-6 stay open:**
1. Layout editability residue (cross-plugin `$ref` to forms' dictionary) → S4.5c (2).
2. F8 `drive_gesture` corpus → S4.5c (1).
3. F21 refusals → S4.5c (3); the planned "resume refusal under `Stream`/`Commit` → `Err`" was revised (see there).
4. F21 `Emit::child_node_drag` → S4.5c (3). Equation (`:125`) and rewriting (`:114`) no longer call `node_drag_child` (they publish a
   root `NodeDragEmit` through `From`), so they are out of scope.
5. **OPEN — F21 legacy laws:** delete the one-per-event pointer-wire laws in draw (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1205-1224`), layout
   (`🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs:178-181`) and fem 2d (`:337`); make `samples`/`cancelled` required in those payloads.
6. **OPEN — F14** (to stage): unique first docstring emojis in `TM/🦀️.rs` (⏱ ×5, 🛠 ×4).

### S4.5c RESUME 19:53 — F8, layout editability, F21 staging (bun/python only, no cargo)

**(1) F8: `drive_gesture` corpus, TS twin and oracles. Landed (TS, JSON, Python); the Rust law is staged.**
- Schema of record: new `$defs` in `TM/🧬️schema/🔣️.json`: `GesturePhaseWord`, `GesturePhase`, `GestureDriveTicks`, `GestureDriveGesture`,
  `GestureDriveDispatch`, `GestureDriveOutcome`, `GestureDriveLawFixture`. They sit beside the other law fixtures' `$defs`
  (`ScrubLawFixture`, …), not in a separate `🔣️gesture-drive-law` schema as planned.
- Fixture `TM/🧫️fixtures/🧫️gesture-drive-law/🔣️.json`: 16 phase words and **31 rows** (9 commit, 6 refused). These are the 27 planned rows
  plus 4 more. Three pin "a refusal has no effect at all": a refused start after a verb switch, a refused tick after a one-shot interruption,
  and a refused tick on an unrestorable gesture. The fourth, a bare stream tick on an unrestorable gesture, pins the zero-trace drop.
- Generator `T/🧪️s4-tools-a-gesture-drive-law.py` (`.venv/bin/python … [--check]`): an independent counting-tool model. It validates the
  fixture with `jsonschema` (draft 7) and checks that 9 hostile mutations are rejected. `--check` → `fixture current: 31 rows (9 commit,
  6 refused), 16 phase words`.
- TS twin in `TM/🟦️.ts`, region `🌊️Gesture`: `GesturePhase`, `parseGesturePhase`, `GestureTool`, `GestureToolKind`, `GestureNext`,
  `GestureDrive(Result)` and `driveGesture`.
- Law `TM/🧪️tests/🧪️gesture-drive-law/🟦️.ts` runs these checks:
  - Ajv: the fixture validates and hostile mutations are rejected.
  - The phase table parses like the fixture.
  - Every row replays.
  - Coverage: every phase from every slot state, and every surfaced refusal.
  - An xstate idle/open/unrestorable slot machine agrees with every fixture row.
  - fast-check: 600 random sequences (up to 40 dispatches, unrestorable injections at weight 3/17). Runner and xstate agree at every
    step. The reached-path census keeps only paths reached in 30/30 consecutive runs; two rarer unrestorable refusal paths flaked at
    2/30, so they are pinned by fixture rows instead.
  - Result: `bun test ./…/🧪️gesture-drive-law/🟦️.ts` → **8 pass / 0 fail**, stable over 30 consecutive runs. Final combined run of
    gesture-drive-law + `🧪️conformance` + `🧪️mutation-history-gates` → **85 pass / 0 fail** (30955 expects). Strict `tsc`
    (`🗑️generated/s4-tools-a/tsconfig.tool-machine.json`: twin, new law, conformance) → **exit 0**.
- Mutation check `T/🧪️s4-tools-a-gesture-drive-mutants.py`: **5/5 mutants killed**. The mutants are: abort before start, verb before base,
  unrestorable refuses, refused tick clears, `same` ignored.
- Oracle registry `TM/🔮️oracles/🔣️.json`: capability `tool-machine.gesture-drive` added to the xstate and fast-check entries.
- Taxonomy: `verify taxonomy report --scope 🛠️tool-machine` → **clean** (new `🧫️gesture-drive-law` and `🧪️gesture-drive-law` dirs).

**F21 semantic revision.** The S4.5b plan returned `Err` for a resume refusal under `Stream`/`Commit`. That would make an undecodable
persisted gesture *sticky*: every later stream tick or commit in that window faults until an abort or one-shot arrives. It would also break
the WFC owner's deliberate law `a_tampered_persisted_stroke_is_dropped_with_zero_trace`. The law is now:
- An unrestorable gesture is dropped with zero trace and the dispatch runs from rest, in every phase.
- A refused **start or tick** is the dispatch's refusal, with no effect at all. The interrupted gesture's abort is deferred until start and
  send succeed, and the persisted gesture stays unchanged.

The Python model, the TS twin, xstate, the fixture and the staged Rust all encode this.

**(2) Layout editability residue → 0 findings.**
- Root cause: `#[derive(MutationLeaf)]` embeds a leaf's referenced documents only from its own plugin tree. The runtime publishes
  `INPUT_SCHEMA_DOCUMENTS` beside the inputs, and anything else falls to the OS registry, which only framework scopes feed. Layout's typed
  `dataFields` (`semio-s-artifact-forms-forms` is a path dependency of the layout crate) `$ref`s forms' published
  `s/forms/forms/dictionary.json`, so the history editor really cannot resolve `change-data-fields`. This is a runtime defect, not a gate
  quirk.
- Fix: the search scope = the leaf's own tree + the plugin of every inline `path` dependency of its crate's `[dependencies]` (nearest
  `Cargo.toml` above the payload schema, beside a directory or in its `📦️packages/🦀️rust`). Schema-first: forms keeps ownership, and there
  is no copy and no move.
- **Landed (TS gate twin):** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts`. New functions
  `mutationLeafCrateManifest`, `mutationManifestPathDependencies` (pure) and `mutationSchemaSearchRoots`. The editability loop now unions
  the indexes, own root first.
- **Landed (language-agnostic law):** `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🧫️manifest-path-dependencies/🔣️.json`
  (5 cases). TS laws in `🧪️tests/🧪️mutation-history-gates/🟦️.ts`:
  - Layout resolves forms' dictionary through its crate dependency, and is unresolved without it.
  - The fixture cases match both our reader and Bun's TOML parser (oracle).
  - Every artifact crate manifest is read like Bun's TOML parser.
- **Staged (rule 45, group `schema`):** the derive twin (`mutation_schema_search_roots`, `mutation_leaf_crate_manifest`,
  `mutation_manifest_path_dependencies` in `🗣️dsl/✨️derive/🦀️.rs`) and its fixture law in `🧪️tests/🔬️mutation-leaf-derive/🦀️.rs`. It must
  land at "CARGO OPEN": until then the gate (twin) is ahead of the runtime.
- Verification:
  - `schema mutation-editability --under ✏️s/🔌️plugins/📏️layout` → **48/48 editable, 0 findings** (`🗑️generated/s4-tools-a/edit-layout-2.txt`).
  - Repo-wide → 3073/3077 editable, **0 `leafReferenceUnpublished`**. 14 findings remain, all `parentLeafReadsChild` in cad (8) and
    trinity (6), which are other owners' findings.
  - `bun test …/🧪️mutation-history-gates/🟦️.ts` → **43 pass / 0 fail**. Strict `tsc` over it and its imports → **exit 0**.
  - Derive taxonomy: the new fixture dir resolves. 19 pre-existing `directory-kind-unresolved` remain in `✨️derive`; they are not mine.

**(3) F21 staging (rule 45): `T/🧪️s4-tools-a-stage-f21.py`.** It re-derives every edit from the CURRENT tree by anchored replacement.
- Staging: no flag → copies + patches under `🗑️generated/s4-tools-a/staged/`.
- Landing: `--land drive|schema|emit` writes a group into the tree at "CARGO OPEN", together with its check. Each group is compile-atomic.
- All three groups re-derive cleanly on 10-04 ~21:00. `rustfmt --check` on the staged copies shows **no parse error**, and my hunks are
  rustfmt-neutral (any remaining diffs are pre-existing).
- **`drive`** (16 files, `staged/drive.patch`):
  - `drive_gesture` → `Result<GestureDrive, ToolRefusal>` with the revised semantics, and an exhaustive `ToolStep` match.
  - Rust fixture law `TM/🧪️tests/🧪️gesture-drive-law/🦀️.rs` (phase table + every row; `LawTool` with a thread-local abort trace), mounted
    as `gesture_drive_law_tests`. It replaces the ad-hoc `🌊️GestureLaws` unit region, together with its `super::GestureTool` hack.
  - Callers map the refusal to `Fault::new(App, refusal.code(), …)` (`toolTransaction.closed|unclosed`):
    - fem 2d `fem_gumball_drive` and lowpoly `lowpoly_paint_drive` → `Result<…, Fault>`.
    - fem 2d/3d `gumball_step`/`gumball_once` and lowpoly paint `?`.
    - raster `handle_in_window` `map_err`.
    - generation3d `dispatch` retires the splice rows on refusal.
    - wfc `bitmap_brush_dispatch` → `Result<…, Fault>` and its editor `?`.
    - `.expect(..)` in the fem 2d/3d interaction, generation3d (4) and wfc brush (21) tests.
  - **Owners touched beyond mine: raster, PROCEDURAL (generation3d), WFC.** The API change forces them to land with the tool-machine patch.
- **`schema`** (2 files, `staged/schema.patch`): the derive twin above.
- **`emit`** (6 files, `staged/emit.patch`):
  - `Emit::child_node_drag(app_id, verb, authoring_seed, gesture, leaves, slot, child_id)` replaces `Emit::node_drag_child`, running the
    node-drag machine and the composed child in one call.
  - The redundant `ui_scope: Full` / `NodeDragEmit::Nothing` matches are dropped: `Emit::default()` already is `UiDirtyScope::Full`, and
    `node_drag_child(Nothing)` already was the default emit. Now-unused imports are removed in flow (factored `flow_drag_release`), dag,
    wires and sequence ×2.
  - **Owners to adopt at "CARGO OPEN": FLOWCAD (flow), GRAPHS (dag), WIRES-MATH (wires), TEXT/sequence owner.**
- Still owed by S4-GATES: framework fault notices for `toolTransaction.closed|unclosed`, now raised by fem, lowpoly, raster, generation3d and
  wfc once `drive` lands.

Scratch (tool output, delete at ticket close): `🗑️generated/s4-tools-a/{edit-layout-2,edit-all-1,tax-tool-machine,tax-derive*,tsc-*,test-gates-*}.txt`,
`mutants/`, `staged/`. The tsconfigs `🗑️generated/s4-tools-a/tsconfig.{tool-machine,gates}.json` are inputs for re-running strict `tsc`.

### S4.6 Coordinator actions

- **S4.5c:**
  - Land the three staged groups at "CARGO OPEN", in this order, each with its own check:
    - `python3 T/🧪️s4-tools-a-stage-f21.py --land drive` → `cargo check -p semio-framework-tool-machine --lib --tests` (root workspace) and
      `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-raster-raster -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-wfc-bitmap --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib --tests`.
    - `--land schema` → `cargo check -p semio-framework-os-kernel-dsl-derive --lib --tests`, plus the layout crate. Its expansion must now
      embed forms' `dictionary.json`.
    - `--land emit` → `cargo check -p semio-framework-plugin --lib` and `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-dag-dag -p semio-s-artifact-reasoning-wires -p semio-s-artifact-sequence-sequence --lib --tests`.
  - Tests: tool-machine lib (`gesture_drive_law_tests`), `cargo test -p semio-framework-os-kernel-dsl-derive -- reads_the_path_dependencies`,
    and the touched plugin crates' gumball/paint/brush/drag laws.
  - Launch rows for `T/🧪️s4-tools-a-gesture-drive-law.py --check`, `T/🧪️s4-tools-a-gesture-drive-mutants.py` and `T/🧪️s4-tools-a-stage-f21.py`.
- Central `schema generate`: new shared `🏗️fem/⚙️engine/🖥️app-surface/🧬️schema/🔣️results-animation`; renamed transient lane
  `fem.resultswindowtransient` (`$id …/s/fem/results-window/transient/…`, leaf `set-playback-clock`); deleted fem 3d transient schemas
  (`…/s/fem/3d/results-window/transient/…`); both results-window config schemas now `$ref` the shared record; S3's still-owed rows
  (shooting `set-camera-draft-label` gone, `CH/🧬️schema/🔣️path-paint`, `🔣️gumball-meta`, `🔣️gumball-dispatch`).
- `describe` for `🏗️fem` (transient schema id, fault codes `fem.result-animation*`, NEW `fem_fault_notices` table), `🖍️draw` + `🗒️note` (NEW
  `fault_notices()` tables — the notices gate reports `describe owed` for these three), `💠️lowpoly`, `🎥️shooting`, `📏️layout`, `🎪️demonstrator`
  and the `OS/🧑‍💻dev/🔌️plugin-modules/*` copies; `test inventory` for fem re-reads the bridge (fem 2d transient surface coordinate).
- Launch rows (`.vscode/launch.json` regeneration): LA `verify-layout-frame-selection`; the two Python corpus scripts
  (`T/🧪️s3-layout-path-paint-corpus.py --check`, `T/🧪️s2-layout-gumball-dispatch-corpus.py --check`).
- Re-activation after green: fem (2d/3d: playback, gumball), lowpoly (paint), layout React 6079 / wgpu 6179 (gumball overlay moved),
  draw 6064/6164, note — then the visual wgpu probe S3-LAYOUT listed.

## Session 5 — 2026-10-05

Executor S5-TOOLS (inherits S4-TOOLS-A + S4-TOOLS-B; the TOOLS-B report links here). Brief: design §22.10 (framework-owned
`Frozen` mapping + ONE window gesture slot), adoption in my plugins, the staged session-4 waves, owed verification.
Scratch: `🗑️generated/s5-tools/`. Nothing below is called green unless its command and count are quoted.

### S5.1 Repair-first (rule 46), 00:25–00:40 — read-only, landing lock held by COORDINATOR-ACTIVATION

- `🛠️tool-machine`, `🔌️plugin/🛠️tool-machine`, `🗣️dsl/✨️derive`: **0 files newer** than the last S4 section (10-04 21:02). The
  session-4 staged groups are still off the tree.
- My plugin trees: 100 files newer than 21:02, all peer sweeps (21:19 sqlite ABI / pack-error, 22:01–22:14 TOOLS-B pack-refusal
  alignment, 23:19 playbook editor + draw gesture-operation-owner test). No half-written file found by reading; compile state is
  UNKNOWN until the owed checks run (cargo is frozen while the activation builds).
- `python3 T/🧪️s4-tools-a-stage-f21.py` (dry run, 00:34): **`drive` re-derives (16 files, 481 changed lines), `schema` re-derives
  (2 files, 74 lines), `emit` FAILS** — the `node_drag_child` doc anchor in `PLG` moved and trinity rewriting
  (`🎮️commands/🕸️node-graph-edit/🦀️.rs:126`) is a new caller. `emit` must be re-derived by hand (S5.6).

### S5.2 Census of every gesture entry point (P1c) — read 00:30–00:45, `git grep` over `✏️s/**` + `🌎️hub/**`, tests excluded

Raw hits: `🗑️generated/s5-tools/census-raw.txt`. Paths are below `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/<any>/✏️editor/`
(`E/`) unless written out. Classes: **D** = shared `drive_gesture`; **W** = hand-written start/resume/abort control flow beside it
(a near-copy of `TM/🦀️.rs:509-538`); **O** = one-shot runner (`ToolMachineRunner::start` + one `send`, no persisted gesture);
**L** = long-lived runner kept in an app-owned session; **N** = the shared node-drag machine (`node_drag_emit`); **X** = stamps
transactions with no machine. "Wire" = the plugin-owned persisted-gesture type the framework slot replaces. "Frozen" = a
per-editor `HostEvent::TimeTravelFrozen` arm.

| Owner WP | Plugin / artifact | Entry point (file:line) | Class | Wire (plugin-owned) | Frozen arm |
|---|---|---|---|---|---|
| S5-TOOLS | fem 2d | `E/🎮️commands/🧭️gumball/🦀️.rs:55` (retained), `:75` (one-shot) → `E/🫧️transient/🦀️.rs:98` `fem_gumball_drive` | D | `FemGumballGesture` + `FemGumballTransient` (artifact transient; schema twins) | none |
| S5-TOOLS | fem 3d | `E/🎮️commands/🧭️gumball/🦀️.rs:57`, `:77` (same `fem_gumball_drive`; `Fem3dGumballTool: GestureTool` `E/🕹️interaction/🧭️gumball/🦀️.rs:270`) | D (the audit's "wrapper" is the `GestureTool` impl) | shares fem 2d's | none |
| S5-TOOLS | lowpoly | paint `E/🎮️commands/🖌️paint/🦀️.rs:141` → `E/🖌️session/🦀️.rs:594` `lowpoly_paint_drive` | D | `LowpolyPaintGesture` in `LowpolyTransient` | none |
| S5-TOOLS | lowpoly | `lowpoly_tool_once` `E/🖌️session/🦀️.rs:502` (every non-paint verb) | O | — | — |
| S5-TOOLS | procedural generation3d | gumball `E/🎮️commands/🧭️transforms/🦀️.rs:527` | D | `GumballGesture` (typed, in the app struct `self.open`) | `E/🦀️.rs:2470-2476` (all six events) |
| S5-TOOLS | procedural generation3d | `E/🎮️commands/🥽️edit-mesh-selection/🦀️.rs:186` | O | — | — |
| S5-TOOLS | layout | `layout_transform_dispatch` `E/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🦀️.rs:356-401` (tool `:286-341`; caller `E/🎮️commands/🧭️gumball/🦀️.rs:124`) | **W** | `LayoutTransformToolState` + `…Entry` (window transient; json/graphql/proto/ts twins) | `E/🦀️.rs:1252-1258` |
| S5-TOOLS | note | `note_ink_dispatch` `E/🎮️commands/🖊️ink-apply-events/🦀️.rs:412-459` (tool `:337-401`; callers `:493`, `🕹️nudge-selection/🦀️.rs:30`) | **W** | `NoteInkToolState` + `…Entry` (window transient; four twins) | none |
| S5-TOOLS | draw | `DrawingTool` `🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:701-740`, host abort `:1509` | **L** (7-state chart, context without an open transaction) | none (runner lives in the gesture operation owner) | none |
| S5-TOOLS | shooting | `shooting_gumball_commit` `E/🎮️commands/🧭️gumball/🦀️.rs:117` | O | — | — |
| S5-TOOLS | generation2d | `E/🎮️commands/✏️node-graph-edit/🦀️.rs:107`, `🚚️move-media-node/🦀️.rs:38` | N | — | — |
| S5-TOOLS | generation3d | `E/🎮️commands/✏️node-graph-edit/🦀️.rs:82` | N | — | — |
| S5-TOOLS | energy, forms, gis, playbook, block | no gesture code (command-driven; forms/energy ride the framework `ScrubLedger`) | — | — | — |
| S5-PUZZLE | puzzle 2d | `transform_selection` `E/🦀️.rs:1887-1937` (tool `E/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️.rs:337-…`; one-shot `:408`) | **W** | `Puzzle2dSelectToolState` (window transient) | `E/🦀️.rs:5035-5041` |
| S5-PUZZLE | puzzle 3d | `puzzle3d_transform_tool_commit` `E/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️.rs:337` | O | — | — |
| S5-PUZZLE | puzzle 5d | `E/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️.rs:286` | O | — | — |
| S5-STROKES-NORM | raster | paint stroke `E/🎮️commands/🖌️paint-stroke/🦀️.rs:381` | D | `RasterStrokeToolState` (window transient) | `E/🦀️.rs:1257-1263` |
| S5-STROKES-NORM | raster | fill `E/🎮️commands/🪣️fill-region/🦀️.rs:56` | O | — | — |
| S5-STROKES-NORM | wfc bitmap | brush `E/🎭️modes/✏️edit/🪟️windows/🖼️input/🪛️utilities/🖌️brush/🦀️.rs:232` | D | `BitmapBrushToolState` in `BitmapInputWindowTransient` | `E/🦀️.rs:808-813` |
| S5-STROKES-NORM | wfc 2d / 3d | `E/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs:35` (both) | N | — | — |
| S5-STROKES-NORM | process3d | `process3d_world_commit` `E/🎮️commands/🌍️world/🦀️.rs:85` | O | — | — |
| S5-STROKES-NORM | remodel | `E/🎮️commands/{🖼️import-frame-payload:135-136, 📼️import-video-frame-payload:31,132, 💽️import-video-bytes-payload:71, ✅️import-video-done:35, 🛑️import-abort:25,27}` | **X** (design §15 streamed ingest; mints its own ref) | `RemodelingWindowTransient.import` | none |
| S5-FLOWCAD | cad | `cad_transform_tool_commit` `E/🎭️modes/✏️edit/🛠️tools/🧭️transform/🦀️.rs:188` | O | — | — |
| S5-FLOWCAD | flow | `E/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs:36` | N | — | — |
| S5-GRAPHS-WIRES | dag | `E/🎮️commands/✏️node-graph-edit/🦀️.rs:121`, `🚚️move-media-node/🦀️.rs:31` | N | — | — |
| S5-GRAPHS-WIRES | sequence | `E/🦀️.rs:1491`, `E/🎮️commands/🕸️node-graph/🦀️.rs:86` | N | — | — |
| S5-GRAPHS-WIRES | wires | `E/🦀️.rs:320` | N | — | — |
| S5-GRAPHS-WIRES | equation | `E/🎮️commands/🕸️node-graph-edit/🦀️.rs:125` | N | — | — |
| S5-GRAPHS-WIRES | space (hub) | `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🦀️.rs:82`, `🚚️move-media-node/🦀️.rs:32` | N | — | — |
| S5-TEXT-STDIO | trinity rewriting | `E/🎮️commands/🕸️node-graph-edit/🦀️.rs:126`, `:157` | N | — | — |

Totals: **D 6** (fem 2d, fem 3d, lowpoly paint, generation3d, raster, wfc bitmap), **W 3** (layout, note, puzzle 2d),
**L 1** (draw), **O 9** (lowpoly, generation3d mesh selection, shooting, puzzle 3d, puzzle 5d, raster fill, process3d, cad, and
puzzle 2d's one-shot), **N 12 sites / 10 artifacts**, **X 1** (remodel). Frozen arms: **5** (generation3d, layout, puzzle 2d,
raster, wfc bitmap). Corrections to `📓️audit-s5-goal.md` clause 11: fem 3d already rides `drive_gesture` through
`fem_gumball_drive` (its `🧭️gumball/🦀️.rs:264-296` is the `GestureTool` impl, not a private driver); shooting, cad, process3d and
puzzle 3d/5d are one-shots (class O), not streamed wrappers. Every D/W site repeats the same five pieces: a 2-state
`idle/streaming` chart, a `*Tool{runner, verb, authoring_seed, base_revision}` struct, start/resume/persist plumbing, a
persisted `{states, verb, authoringSeed, baseRevision, transaction, entries[{key, mutation}]}` type with its own schema twins,
and a preview that folds the entries on the document.

### S5.3 Design of §22.10 (framework half) — what the three waves put on disk

Priority note (rule 54): P1's corpus law in Rust extends `TM/🧪️tests/🧪️gesture-drive-law/🦀️.rs`, which only exists in the staged
F21 `drive` group, and the corpus already encodes F21's refusal semantics. So wave A = F21 `drive` (a P3 item) lands first; it
is the precondition of P1, not a skip-down.

- **Pure crate (`TM/🦀️.rs`, region `🌊️Gesture`), wave B**
  - `GestureHostEvent` (`blur | captureLost | utilityChanged | retiring | timeTravelFrozen | baseMoved`) and
    `abort_reason(base_bound)`: the ONE table "host fact → `ToolAbortReason`" (`frozen` for a history edit; `retired` for a
    utility switch or closing window; a moved base ends only a gesture pinned to a base revision).
  - `GestureState<M>`: the framework-owned persisted gesture `{states, verb, authoring_seed, base_revision, transaction,
    entries, context}` (`context` = tool context its entries do not already say, `Null` for the six simple tools).
  - `GestureChart`: what a statechart tool declares (tool id, host, input, `restore` of its context, the event of one
    dispatch, `BASE_BOUND`). `ChartGesture<T>` is the ONE `GestureTool` of every statechart tool (start / resume / persist /
    abort / clock); `drive_chart_gesture` is the unmounted entry. `GestureTool` gains `const BASE_BOUND = true`.
  - `GestureLedger<M>`: at most one `GestureState` per window; `drive`, `settle`, `abort`, `host_event`, `host_event_all`,
    `retain_windows`, `provisional`.
  - Schema of record `$defs`: `GestureHostEvent`, `GestureHostAbort`, `GestureState`, `GestureSlotOutcome`,
    `GestureSlotStep`, `GestureSlotScenario`; `GestureDriveLawFixture` requires `hostEvents` (12 rows) and `slots`.
    A gesture `base` may be empty (= pinned to no revision).
  - Corpus `🧫️gesture-drive-law`: + `hostEvents` (6 facts × base-bound yes/no) + `slots` (17 scenarios, 51 steps: a gesture
    persisted by one dispatch and resumed by the next, a history edit freezing two windows with zero trace, one frozen window,
    blur, capture lost, utility switch, closing window, moved base on a pinned and on an unpinned gesture, facts at rest,
    windows never sharing a slot, the roster retiring windows, a verb switch, a refused tick, an unrestorable slot dropped by
    a dispatch and by a host fact, a moved base under a dispatch). Author: the independent Python model in
    `T/🧪️s4-tools-a-gesture-drive-law.py` (extended; `--module <dir>` targets a staged copy).
  - TS twin `TM/🟦️.ts`: `GESTURE_HOST_EVENTS`, `gestureHostAbortReason`, `GestureState<M>`, `GestureLedger`,
    `GestureToolKind.baseRevision`. TS law: + 4 tests (host table, every slot scenario, coverage, the xstate slot oracle with a
    `host` event judging every window of every scenario) and `host` operations in the fast-check sequences.
  - Rust law: the counting tool as a REAL `statechart!` chart on `ChartGesture` drives the real `GestureLedger` through every
    slot scenario; host table; persist → resume → persist identity and the two refusals (`Closed`, `Unclosed`).
- **Plugin runtime (`PLG/🛠️tool-machine/🦀️.rs` + 6 anchored hunks in `PLG/🦀️.rs`), wave C**
  - `ToolMachineRuntime.gestures: GestureLedger<A::Mutation>` — the ONE window slot, beside the press and typing ledgers.
  - `deliver_host_event` first calls `end_window_gesture(&event)`; `deliver_host_event_to_every_window` first calls
    `end_every_gesture` (so a window no view lists is frozen too). This is the one mapping; an app needs no arm.
  - `ArtifactOwnedToolJobContext::gesture() -> &GestureSlot<A::Mutation>`: the dispatching window's slot as of admission.
    A handler calls `context.gesture().drive::<ChartGesture<MyChart>>(verb, phase, tick, &operation.authoring_seed)` and
    publishes `gesture_emit(committed, seed)`. `settle_tool_operation` keeps what the dispatch decided only when it publishes,
    while no history edit freezes the document, and while the window still holds the gesture the dispatch was admitted on.
  - Preview is the framework's: `follow()` folds every open gesture's entries into the render overlay (committed ⊕ presses ⊕
    gestures ⊕ typing) and `provisional_values()` carries them to derived views. No plugin-owned preview fold.
  - A dispatch that only advanced or ended its gesture logs no history row (same rule as a press tick).
  - `hostEvent` answers `UiDirtyScope::Full` when the fact ended a gesture (the preview must repaint).
  - Laws `PLG/🧪️tests/🧪️gesture/🦀️.rs` (mounted from the time-travel tests to reuse the toy history app, which answers no
    host event): Frozen with zero trace over two windows + a tick on the frozen document opens nothing; a window's blur /
    lost capture through the real `hostEvent` verb ends only its gesture; the slot follows only a publishing dispatch admitted
    on the gesture the window still holds.
- **Known limits, stated**: (1) only the app-owned job route (`ArtifactOwnedToolJobContext`) reaches the slot; a bounded
  `ArtifactApp::handle` has no access (`ArtifactView` is not generic over the mutation). (2) Two dispatches admitted on the same
  slot before either publishes: the second is dropped (law 3) — same exposure the transient-based persistence had, now explicit.
  (3) A late `commit` tick after a host abort still commits a one-shot for tools that map commit-at-rest to a one-shot (fem,
  lowpoly); the scrub ledger's "closed press" memory has no gesture twin because gesture verbs carry no press id. (4) Draw's
  7-state canvas tool keeps context without an open transaction; `GestureState` requires an open transaction, so draw needs
  `transaction: Option<…>` before it can move (not done). (5) The overlay fold costs one leaf application per tick; raster's
  stroke leaf re-rasterizes, so raster needs a preview opt-out when it adopts.

### S5.4 Staged at 00:58, lock still HELD — nothing saved under `🧰️framework/**`, no cargo run

| Wave | How to land (under `landing`, one at a time) | Verified so far |
|---|---|---|
| A = F21 `drive` | `python3 T/🧪️s4-tools-a-stage-f21.py --land drive` | re-derives against today's tree (16 files); Rust uncompiled |
| B = `slot` | `python3 T/🧪️s5-tools-stage.py slot --land` (needs A on disk) | staged copy `🗑️generated/s5-tools/stage/tm/`: generator validates the fixture against the staged schema and rejects 16 hostile mutations; `bun test <staged>/🧪️tests/🧪️gesture-drive-law/🟦️.ts` **12 pass / 0 fail, stable ×30** (one census path that flaked 9/30 is pinned by a fixture row instead); strict `tsc -p 🗑️generated/s5-tools/tsconfig.stage-tm.json` **exit 0**; Rust parses (`rustfmt --check`: my hunks neutral), uncompiled |
| C = `runtime` | `python3 T/🧪️s5-tools-stage.py runtime --land` (needs B on disk) | all 26 anchors match today's `PLG` once; staged copies + `🗑️generated/s5-tools/staged/runtime.patch` (409 changed lines); Rust parses, uncompiled |

Checks per wave: A → `cargo check -p semio-framework-tool-machine --lib --tests` + `-p semio-framework-plugin --lib` + the six
caller crates (S4.6); B → the same two + `cargo test -p semio-framework-tool-machine --lib gesture_drive_law`; C →
`cargo check -p semio-framework-tool-machine -p semio-framework-plugin --lib` then `--lib --tests --features artifact-app-testing`
and `cargo test -p semio-framework-plugin --lib --features artifact-app-testing gesture_laws`.

### S5.5 LANDED 01:23–01:47 — waves A + B + C on disk, laws RUN (coordinator: "accepted as landed")

One `landing` hold 01:23:36 → 01:47:08 (23.5 min; 7 min in the v4 gate, two peer reds of the Codex pack wave in
`📡️replication/🎮️mutation/📦️bytes:191` and kernel `🗣️dsl:219` in between — not mine, reported to `main`).

| When | Command | Result |
|---|---|---|
| 01:26:11 | `cargo check -p semio-framework-tool-machine --lib --tests` (A + B) | **exit 0**, 0 tool-machine warnings |
| 01:34:39 | `cargo check -p semio-framework-tool-machine -p semio-framework-plugin --lib` (A + B + C) | **exit 0** (kernel + plugin compiled; 283 plugin warnings, none in my hunks) |
| 01:46:56 | `cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` | **exit 0** (compiles `🔌️plugin/🧪️tests/🧪️gesture/🦀️.rs`) |
| 01:53:33 | `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-raster-raster -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-wfc-bitmap --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib` (wave A callers) | **exit 0** (6 m 04 s) |
| 01:54 | `.venv/bin/python T/🧪️s4-tools-a-gesture-drive-law.py --check` | fixture current: 31 rows, 16 phase words, 12 host facts, 17 slot scenarios (51 steps) |
| 01:54 | `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts ./…/🧪️conformance/🟦️.ts` | **46 pass / 0 fail** (31 186 expects) |
| 01:59:04 | `cargo test -p semio-framework-tool-machine --lib -- gesture_drive_law` | **5 passed / 0 failed** (33 filtered out) |
| 02:03:48 | `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- gesture_laws` | **3 passed / 0 failed** (1010 filtered out): the Frozen law, the per-window host-fact law, the slot-follows-publication law |
| 02:08:17 | same binary, `-- tool_machine:: opening_a_history_edit a_committed_tool_transaction a_streamed_tool_transaction artifact_and_history_lane_verbs` | **10 passed / 0 failed** (scrub ×5, typing ×2, time-travel ×3 incl. host-event delivery): no regression |

Compile fixes made under the lock (both test-only): the law's chart host `CounterHost` is `pub` (it is the associated type of a
public chart); the plugin law names `DslValue`, `ActorId`, `HybridLogicalTimestamp`, `ViewWindowInstance`, `Locale`,
`Terminology`, `gesture_emit` unqualified (`protocol::DslValue` is a private re-export in that scope).
Incident (mine): the wave B save of `TM/🟦️.ts` at 01:25:55 hot-reloaded the React serve under probe batch A (the file was not on
rule 52's list; rule 57 now covers every framework `🟦️.ts`). I take `serve` for such saves from now on.
OWED for wave A: `--tests` of the six caller crates (`…--lib --tests`, same command) and their gumball / paint / brush laws.
Files landed: `TM/{🦀️.rs, 🟦️.ts, 🧬️schema/🔣️.json, 🧫️fixtures/🧫️gesture-drive-law/🔣️.json, 🔮️oracles/🔣️.json,
🧪️tests/🔬️unit/🦀️.rs, 🧪️tests/🧪️gesture-drive-law/{🦀️.rs (new), 🟦️.ts}}`; `PLG/{🦀️.rs (6 hunks), 🛠️tool-machine/🦀️.rs,
🧪️tests/🧪️gesture/🦀️.rs (new), 🧪️tests/🧪️time-travel/🦀️.rs (mount line)}`; wave A callers: fem 2d `E/{🫧️transient,
🎮️commands/🧭️gumball, 🕹️interaction/🧭️gumball/🧪️tests/🔬️unit}`, fem 3d `E/{🎮️commands/🧭️gumball,
🕹️interaction/🧭️gumball/🧪️tests/🔬️unit}`, lowpoly `E/{🖌️session, 🎮️commands/🖌️paint}`, generation3d
`E/🎮️commands/🧭️transforms/{🦀️.rs, 🧪️tests/🔬️unit}`, raster `E/🎮️commands/🖌️paint-stroke`, wfc bitmap `E/{🦀️.rs,
🎭️modes/…/🖌️brush, 🧪️tests/🧪️brush-tool}` (raster + wfc with the coordinator's leave; S5-STROKES-NORM told through `main`).

### S5.6 P4 — layout `change-data-fields` off the out-of-vocabulary widget (01:17, S5-UI's shape relayed by `main`)

- `📏️layout/…/🧬️mutations/🧾change-data-fields/🧬️schema/🔣️.json`: `"widget": "dictionary"` dropped (the declared
  `{entries:[{questionId, value}]}` reads as a nullable object → list rows).
- `📋️forms/…/🧬️schema/🧾️dictionary/🔣️.json`: `x-semio-ui.label` on `entries` ("Entries" / "Einträge") and on `questionId`
  ("Question" / "Frage") with `widget: "text"` (else it infers a `question` reference).
- RAN (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/📏️layout`
  → **146/146 inputs of 48 leaves, 0 findings**; `schema mutation-editability --under …/📏️layout` → **48/48, 0 findings**;
  `schema mutation-inputs --under ✏️s/🔌️plugins/📋️forms` → **29/29 inputs of 15 leaves, 0 findings**.
- Still staged: F21 `schema` (derive twin embeds forms' dictionary for the layout leaf at runtime). It edits the `MutationLeaf`
  proc-macro crate, so landing it rebuilds every plugin crate of the fleet — ask `main` for a slot before the next describe wave.

### S5.7 P2 — adoption in my plugins: NOT DONE (02:25). Why, and the exact next steps

The per-owner adoption list went to `main` at 01:55 (relayed to S5-PUZZLE, S5-STROKES-NORM, S5-FLOWCAD). For my own trees I
read fem 2d/3d to the end and stopped before writing, because the fold-based preview is not a mechanical swap there:

- **Finding (fem 3d, blocks a blind port):** `Fem3dPlayApp::render_body` (`E/🦀️.rs:1221-1246`) takes the gesture preview as a
  SEPARATE snapshot on purpose: with a preview the model window renders "from its own meshes rather than the committed
  revision's live visual pages" (`render_with_progress(preview, …, None)` vs `with_live_visual(doc.render_operation(), …)`) and
  the results window solves uncached (`render(preview, …, None)` vs `render(doc.snapshot, …, doc.render_operation())`). Under
  the framework fold `doc.snapshot` already is committed ⊕ gesture while `doc.render_operation()` still names the committed
  revision, so the cached visuals / solve would be stale for the dragged geometry. A render needs to know that its snapshot is
  provisional. `ArtifactView::provisional_generation()` only says that it moved, not that an overlay is active.
- **Framework step owed before fem (one small wave C2, `landing`):** `ArtifactView::provisional() -> bool` (bound next to
  `with_provisional_generation` from `tool_machines.overlay.is_some()`), then fem 3d's two `match preview` arms become
  `if doc.provisional()`. The same predicate serves any app that caches by `render_operation` — check press (scrub) previews
  of fem 3d too: they hit the same seam today.
- **fem 2d/3d port, file by file (11 files, both crates symmetric):**
  1. `E/🕹️interaction/🧭️gumball/🦀️.rs`: delete `Fem{2,3}dGumballTool` struct + `impl GestureTool` (2d `:295-342`, 3d `:263-310`);
     add `pub type Fem{2,3}dGumballTool = ChartGesture<gumball_tool::GumballTool>;` and `impl GestureChart for
     gumball_tool::GumballTool` (`tool` = `{FEM?D_EDITOR_APP_ID}#{verb}`, `restore` = the `MoveSelection` entry under
     `FEM?D_GUMBALL_LEAF_KEY`, `event` = today's `send` match).
  2. `E/🎮️commands/🧭️gumball/🦀️.rs`: `gumball_step` → `context.gesture().drive::<Fem?dGumballTool>(verb, phase, tick,
     &operation.authoring_seed)?` + `ArtifactCommandWorkStep::Complete(Emit { ui_scope, ..semio_framework_plugin::app::gesture_emit(…) })`
     (no `CompleteWithEphemeral`); `gumball_once` → `semio_framework_tool_machine::drive_gesture::<Fem?dGumballTool>(None, …, "")`.
  3. `E/🦀️.rs`: `type Transient = NoTransient` / `NoTransientMutation`, `no_transient_store_disposer()`,
     `no_transient_local_root_retirement_factory()`, drop `build_transient_store_one_item_preparation_factory`; the
     `TransientView<'_, FemGumballTransient>` parameter and the `transient.snapshot.preview::<…>` line go; `render_body` loses
     `preview` (2d: `let canvas = doc.snapshot`).
  4. Delete `◻️2d/…/✏️editor/🫧️transient/**` (module, schema, fixture, unit test) and `pub mod transient;` (`◻️2d/🦀️.rs:1395`)
     after a path-limited grep proves zero references (rule 32); catalog rows `fem.gumballtransient` → central `schema generate`.
  5. Tests `E/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs` (both): the `drive` helper becomes a
     `GestureLedger<Fem?dMutation>` (`ledger.drive::<Fem?dGumballTool>(window, …)`, `ledger.open("w")`, the net-transform
     assertion folds `ledger.provisional()`); the three laws keep their statements.
  6. Law per plugin (brief): a mounted dispatch stream → commit is ONE history row whose `move-selection` stays editable
     (`history_edit_acceptance_law!` already covers the leaf; add the row-count assertion to the mounted gumball suite).
- **lowpoly** (`LowpolyPaintGesture` in `LowpolyTransient`, `E/🖌️session/🦀️.rs:519-597`), **generation3d**
  (`GumballGesture` with `ids` → `GestureChart::context`, app-held `self.open` map → the slot, Frozen arm `E/🦀️.rs:2470-2476`),
  **layout** (`E/…/🔄️transform/🦀️.rs:256-401` + arm `E/🦀️.rs:1252-1258` + four schema twins), **note**
  (`E/🎮️commands/🖊️ink-apply-events/🦀️.rs:290-459`; its tick reads the open entries: `context.gesture().open()`), each the same
  recipe. **draw** waits for a slot whose gesture may hold no open transaction (§ S5.3 limit 4).
- Until an app adopts, its own transient keeps working exactly as before: wave C changes nothing for a tool that never
  touches `context.gesture()` (its slot stays empty, so no host fact, fold or settle applies).

### S5.8 Owed verification — what I ran on my trees (02:13–02:25) and what is red

`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-{draw-drawing,note-note,layout-layout,shooting-shooting,energy-model,forms-forms,gis-gismap,gis-gisterrain,procedural-generation2d,playbook-playbook,block-2d,block-3d,block-5d} --lib --keep-going`
(02:13:24 → 02:20:08, output `🗑️generated/s5-tools/check-mytrees-lib.txt`):

| Crate | Native `--lib` |
|---|---|
| fem-2d, fem-3d, lowpoly, generation3d (wave A callers, 01:53:33) | exit 0 |
| draw, note, layout, shooting, forms, gisterrain, generation2d, playbook, block-2d, block-5d | compiled (10 of 13) |
| block-3d | 9 errors (peer API: `IoError` lost `message`/`diagnostics`) → **fixed**: `T/🧪️s5-tools-block3d-io-error.py --apply` (9 sites / 4 io files, the form block 2d/5d use); re-check 02:24:36 **exit 0** |
| **energy-model** | **RED, 13 errors** (peer API fallout, not fixed): `dsl::json` gone (`E/🦀️.rs:315, 1354`, `👁️viewer/🦀️.rs:147, 173`); `ValueError::new` shape in `🧬️schema/📸️snapshot/🦀️.rs:120`, `🚪️io/{📤️export,📥️import}/…/🎒️zip/🔖️2.0/✳️any/🦀️.rs:9`; `Result<T, String>` vs `ValueError` in `…/🪶️sqlite/⚙️systems/🦀️.rs:27` |
| **gis-gismap** | **RED, 38 errors** (peer API fallout, not fixed): `💡️inference/👷️worker/🦀️.rs:5` imports `os_dsl::{DslValue, ToValue, FromValue}` that no longer exist there (+ 14 `to_value`/`from_value` sites `:185-250`); `🧬️schema/📸️snapshot/📦️pack/🦀️.rs:15` `from_value` error type; `🧬️mutations/💾️binary/🦀️.rs:263`; 7 io serializers at line 13/14 (`🖊️dwg`, `📐️dxf` import; `🎨️svg`, `📖️pdf`, `📷️png`, `🖊️dwg`, `📐️dxf` export) with the old `ValueError::new` shape — gisterrain (green) holds the current form to copy |

S5-GATES' tool-mismatch codemod on block: `python3 T/🧪️s5-gates-tool-mismatch.py --root "✏️s/🔌️plugins/🧱️block" --apply` →
3 sites rewritten (2d `:481`, 3d `:822`, 5d `:448`), second run "none left"; covered by the checks above.

**OWED (exact commands; gate v5 + `CARGO_BUILD_JOBS=3`):**
1. Fix energy-model + gismap (lists above), then `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model -p semio-s-artifact-gis-gismap --lib`.
2. Wave A callers' test targets: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-raster-raster -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-wfc-bitmap --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib --tests`.
3. Hub wasip2 (no COMPOSITION GREEN sent for any of my 11): `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-draw -p semio-hub-note -p semio-hub-layout -p semio-hub-fem -p semio-hub-lowpoly -p semio-hub-shooting -p semio-hub-forms -p semio-hub-procedural -p semio-hub-playbook --lib --keep-going`; energy + gis after item 1; block's hub likewise.
4. S5-GATES input gate on my trees: numericUndeclared 24 (gis 9, lowpoly 6, energy 6, draw 2, procedural 1), labelMissing 6 (gis 4, lowpoly 2), plus the unowned tail the coordinator gave me (animate 22, framework workflow 4, sourcing 2, infinite 2, architect 2): `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/<plugin>" --inputs`. Not started.
5. §22.20 energy `disconnect-referenced` / `unbind-weather-file`: wait for "EDITABLE MARKER ON DISK".
6. P3 leftovers: F21 `emit` (re-derive by hand: the `node_drag_child` doc anchor moved, trinity rewriting `:126` is a sixth
   caller); F21 `schema` (staged, re-derives; fleet-wide rebuild); D11 `settle_press` clock (`PLG/🛠️tool-machine/🦀️.rs`,
   `authoring_clock(0)` → `self.tool_machines.clock()` + the two-presses-one-millisecond law); D24 gen2d/gen3d example
   normalization; F21 legacy pointer-wire laws; F14 emoji repeats in `TM/🦀️.rs` (my new docstrings are unique).
7. Playbook F2: still blocked on S5-NESTED.

### S5.9 Coordinator actions

- Central `schema generate`: tool-machine `$defs` (`GestureHostEvent`, `GestureHostAbort`, `GestureState`,
  `GestureSlotOutcome`, `GestureSlotStep`, `GestureSlotScenario`); forms `dictionary.json` labels; layout `change-data-fields`.
- Launch rows: `T/🧪️s5-tools-stage.py`, `T/🧪️s5-tools-block3d-io-error.py` (inputs, kept), `T/🧪️s4-tools-a-gesture-drive-law.py --check`.
- Next describe wave: forms + layout (leaf descriptors changed), block (fault codes).
- Live probe (rule 49): no user-visible step changes yet — wave C is inert until an app adopts `context.gesture()`. The first
  live proof of §22.10 is "drag a fem / layout gumball, press Edit on a history row mid-drag → the preview vanishes, no row
  appears", once one of them is ported.
- Scratch (tool output, delete at ticket close): `🗑️generated/s5-tools/{check-*,test-*,ts-*,tsc-*,inputs-*,edit-*,land-b,fmt/,pre-c/,staged/,stage/tm/,census-raw.txt}.*`; the input scripts live in the ticket root
  (`T/🧪️s5-tools-stage.py`, `T/🧪️s5-tools-slot-{schema,ts,rust}.py`, `T/🧪️s5-tools-slot-tsconfig.json`; all three slot scripts
  answer "already applied" on today's tree).

### S5.10 Reds in my trees after the 02:40 and 07:45 cuts — ported in source; ONE run green, the rest never got a build slot

- **Port script** `T/🧪️s5-tools-energy-gismap-port.py` (explicit list of 17 files, counted anchors, idempotent: a second run reports
  0 sites): energy-model 8 sites / 6 files (`dsl::json` → `semio_framework_pack_json` with `JsonMemberPolicy::Reject`,
  `TextError::new(kind, message, span)` ×3, `DslField::from_value` result mapped through `invalid`); gis-gismap 12 sites / 10 files
  (worker imports `semio_framework_value::{DslValue, ToValue, FromValue}`, `Octets::from_value -> Result<_, String>`,
  `advance -> Result<_, ValueError>` with its two "lost owner" refusals as `InvariantViolated`, 7 io `TextError::new` sites);
  animate presentation (unowned, given to me) 2 sites in `🚪️io/🧬️mutations/💾️binary/🦀️.rs` (`String` → `ValueError`).
- **Hub gis**: `🌎️hub/🧩️compositions/🌍️gis/🦀️.rs:43` `protocol::ToValue` (private) → `semio_framework_value::ToValue`, saved under
  the `hub` lock 07:38 (acquired, edited, released).
- **RAN**: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model -p semio-s-artifact-gis-gismap --lib --keep-going`
  → **exit 0 at 02:43:25** (3 m 20 s; energy 44 warnings, both crates compiled). That was before wave B / channel 22.
- **NOT re-verified since**: the 07:33 run (`… + animate-presentation + sourcing-curation`) stopped at 08:03 on a red that is not
  mine — `semio-s-artifact-stdio-pdf` `🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs:15:46`
  E0603 `apply_validated_snapshot_patch` is private (S5-TEXT-STDIO's tree, it held `stdio`) — before reaching my crates. The
  09:38 run waited 11 min in gate v5 and never started: 4–5 cargos the whole time (two childless for 12 min:
  `cargo check … -p semio-s-artifact-stdio-semio --lib --tests --all-features` pid 57200 and
  `cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-os-renderer-wgpu --lib` pid 57700; the
  rest are the Codex peer's `nx run-many --target=test-snapshot-sqlite-native`). I stopped my own loop (rule 65).
- **sourcing-curation** (`✏️editor/🦀️.rs:640, 812` at the coordinator's reading): the lines have moved and I have no current
  error text; not touched.
- **OWED, exact**: (1) `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model -p semio-s-artifact-gis-gismap -p semio-s-artifact-animate-presentation --lib --keep-going`;
  (2) `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-sourcing-curation --lib` → fix → re-check;
  (3) `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-gis -p semio-hub-energy -p semio-hub-block --lib --keep-going`,
  then the other nine hubs in batches of ≤ 3. No "COMPOSITION GREEN" line has been sent for any plugin of mine.

### S5.11 Gesture press identity (§22.29) — STAGED and verified as far as bun/tsc/python go; NOT landed (needs `landing` + `serve`)

Decision implemented: the slot owns a gesture's host press and the press each window last closed.
- **Rule** (one function, `drive_press` / `drivePress`, used by `GestureLedger::drive` and by the runtime's `GestureSlot::drive`):
  a gesture belongs to the press that opened it (`GestureState.press`, stamped by the slot, never by the tool); a dispatch of the
  press the window last closed is dropped with zero trace; a dispatch of another press drives from rest and replaces the open
  gesture (the interrupted press is closed); a named gesture that ends — by a host fact, a moved base, a verb switch, anything —
  closes its press; a named one-shot, commit or abort closes its press; a refused dispatch changes neither slot nor memory; a
  retired window forgets its closed press. One closed press per window, like the scrub ledger. `drive_gesture` reports
  `continued` so the slot knows whether the dispatch resumed the persisted gesture (no transaction-equality guess).
- This closes limit 3 of § S5.3: a late `commit` after a blur or a freeze no longer commits a one-shot when the host names its press.
- **Stage script** `T/🧪️s5-tools-press.py` (8 files, counted anchors, re-derives from the current tree; `--land` writes them):
  `TM/{🧬️schema/🔣️.json, 🟦️.ts, 🦀️.rs, 🧫️fixtures/🧫️gesture-drive-law/🔣️.json, 🧪️tests/🧪️gesture-drive-law/{🟦️.ts, 🦀️.rs}}`,
  `PLG/{🛠️tool-machine/🦀️.rs, 🧪️tests/🧪️gesture/🦀️.rs}`. API changes: `GestureLedger::drive(window, press, …)`,
  `GestureSlot::drive(press, verb, phase, tick, seed)`, `GestureState.press`, `GestureDrive.continued`,
  `GestureLedger::{closed, close, closed_presses}`; TS `GestureToolKind.{press, withPress}`.
- **Corpus**: the independent Python model (`T/🧪️s4-tools-a-gesture-drive-law.py`, already extended on disk — it is a ticket
  script, so `--check` against the TREE fixture is stale until the wave lands) now writes **30 slot scenarios / 96 steps** (13 new
  press scenarios: late tick of a committed press, late release after a blur, frozen press stays ended, another press
  interrupts, a refused tick of another press, a press that opened nothing is not closed, a one-shot closes its press, a moved
  base ends a named press for good, unnamed ↔ named dispatches, a verb switch under one press, a retired window forgets, an
  unrestorable named slot) and a `closed` map in every step outcome; schema `$defs` updated; 19 hostile mutations rejected.
- **RAN on the staged copy** (`🗑️generated/s5-tools/staged/press/`): `bun test <staged>/🧪️tests/🧪️gesture-drive-law/🟦️.ts` →
  **14 pass / 0 fail, stable ×30** (two new tests: scenario coverage of dropped / interrupting / refused / forgotten presses, and
  a fast-check property of the press invariants over 400 random named sequences; the xstate slot oracle keeps judging the
  press-less scenarios); strict `tsc -p 🗑️generated/s5-tools/tsconfig.press.json` → **exit 0**; `rustfmt --check` parses the four
  staged Rust files, my hunks neutral. **Rust uncompiled.**
- **To land** (hold ≤ 20 min): `acquire landing S5-TOOLS` → `acquire serve S5-TOOLS` → `python3 T/🧪️s5-tools-press.py --land` →
  `cargo check -p semio-framework-tool-machine --lib --tests` → `cargo check -p semio-framework-tool-machine -p semio-framework-plugin --lib`
  → `cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` → release `serve`, `landing` →
  `cargo test -p semio-framework-tool-machine --lib -- gesture_drive_law` (expect 5) and
  `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- gesture_laws` (expect 4: + the press law
  `a_late_release_of_a_press_the_runtime_ended_leaves_zero_trace`) → `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts`
  → `.venv/bin/python T/🧪️s4-tools-a-gesture-drive-law.py --check` → tell `main` "GESTURE PRESS IDENTITY ON DISK".
  No plugin calls `GestureLedger::drive` / `GestureSlot::drive` yet, so the signature change breaks nobody.

### S5.12 Adoption census → `📓️s5-tools-adoption.md` (counts first)

0 of 10 streamed gestures ride the framework slot; 5 per-editor host-fact arms and 8 plugin-owned persisted-gesture types are
still on disk; grep-level, 7 pointer-driven files publish a plain edit with no transaction marker (layout canvas-drop, draw
drop-layer-kind, block 3d place-vortex — mine; stdio png / tiff / bmp paint-region and pdf page — S5-TEXT-STDIO), unread.
`ArtifactView::provisional()` and every port of § S5.7 are NOT started.

### S5.13 Press wave APPLIED via the landing train (10:07), fix-forward 10:08 — Rust verdict is the train's, not mine yet

- **Applied** (`landing` + `serve` held 13 s, apply-only per rule 67): `python3 T/🧪️s5-tools-press.py --land` → 8 files; train
  line `10:07:12 S5-TOOLS press 8 files restore: python3 T/🧪️s5-tools-press.py --restore` (pre-landing copies under
  `🗑️generated/s5-tools/pre-press/`). "PRESS ON DISK" sent to `main`.
- **Fix-forward, by desk check, before the train reached the line** (`T/🧪️s5-tools-press-bound.py --apply`, second apply-only
  hold, train line `press-bound 2 files`): `drive_press`, `GestureLedger::drive` and `GestureSlot::drive` now state
  `M: PartialEq` — `drive_press` compares the stamped gesture with the held one, and a generic caller knows `PartialEq` only for
  the projection `T::Gesture`, not for `GestureState<M>`.
- **RAN on the tree after the landing**: `.venv/bin/python T/🧪️s4-tools-a-gesture-drive-law.py --check` → fixture current
  (31 rows, 12 host facts, **30 slot scenarios / 96 steps**); `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️gesture-drive-law/🟦️.ts ./…/🧪️conformance/🟦️.ts`
  → **48 pass / 0 fail** (32 500 expects).
- **Rust: NO VERDICT yet.** `train.status` read `CHECKING since 09:59:44 through: baseline` at 10:13. The two targeted tests are
  OWED after the GREEN that names `press-bound`: `zsh T/🚦️gate.sh 3 25 && CARGO_BUILD_JOBS=3 cargo test -p semio-framework-tool-machine --lib -- gesture_drive_law`
  (expect 5) and `… cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- gesture_laws` (expect 4).
  A RED naming `🛠️tool-machine/🦀️.rs`, `🔌️plugin/🛠️tool-machine/🦀️.rs` or `🔌️plugin/🧪️tests/🧪️gesture/🦀️.rs` is mine: fix forward or
  run the restore above.
- **Owed reds: still not run.** Gate v6 exited 5 twice (10:06 after 15 min, 10:12:52 after 240 s: `shared-cargo=3`). Commands in
  § S5.10 unchanged; add `-p semio-s-artifact-block-3d`: `📍️place-vortex/🦀️.rs:32` builds
  `Emit { artifact_mutations, description: None, ..Default::default() }`, and I saw no `description` field on `Emit` when I read
  it — verify with the check before believing either.
- **§22.32 share (b)/(c)/(d): not started.** Block 3d has neither `semio-framework-tool-machine` nor `machine` in its
  `Cargo.toml`; a `GestureChart` there adds two dependency edges to the `✏️s` workspace lock — say so to `main` before B2.

### S5.14 10:20–11:30 — press wave verified at the crate level, notices, one-step tool, three ports written (B2 freeze from 11:21)

**Press wave (§22.29) — verdicts**
- `train.status`: FRAMEWORK GREEN 10:18:38 through the 10:09:16 line → press + press-bound compile (`--lib`).
- RAN (private build dir, both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` = `🗑️generated/s5-tools/target`; gate v6 had
  closed a third time at 10:25): `cargo test -p semio-framework-tool-machine --lib -- gesture_drive_law` → **5 passed / 0 failed**
  (33 filtered out) at 10:29:35 — the 30 slot scenarios incl. the 13 press scenarios run through the real `GestureLedger` and the
  statechart counting tool.
- OWED: `gesture_laws` (expect 4) on the plugin test binary S5-RUNTIME builds — no path received; so no "PRESS GREEN" sent.

**Notices of the gesture refusals** (`T/🧪️s5-tools-gesture-notices.py`, train 10:41:57; fix-forward
`T/🧪️s5-tools-gesture-notice-family.py`, train 11:01): rows for `toolTransaction.closed`, `toolTransaction.unclosed` and the
poisoned slot in the kernel table, its TS twin and the fixture (82 → 85 rows). My first code `toolGesture.slot-poisoned` was
outside the fixture schema's closed family list and reddened S5-LOAD's run of the notices law; the code is now
`toolTransaction.slot-poisoned` (runtime + 3 twins). RAN: `bun test ./🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧪️framework-notices/🟦️.ts`
→ **4 pass / 0 fail**; the three twins agree row by row (85, unique codes); `bun ./📜️script.ts schema fault-notices --scope
history-editing` runs (208 findings in scope, none naming these codes); `train.status` FRAMEWORK GREEN 11:06:37 covers both lines.
Lesson: I ran my own twin cross-check instead of the law that pins the schema — the law is the check.

**The ONE one-step tool** (`T/🧪️s5-tools-once.py`, train 11:12:18 `tool-once 3 files`): `tool_once_emit(app_id, verb, seed,
leaves)` in `TM/🦀️.rs` (the continuous-control machine driven as a press that opens and releases in one event — what
`node_drag_emit` already was for a one-shot) and `Emit::tool_once(app_id, verb, seed, mutations)` in `PLG/🦀️.rs`, so a
one-click tool of any plugin publishes ONE tool transaction with no statechart and NO new dependency of its own (this replaces
the block 3d `Cargo.toml` change the coordinator had allowed — not needed). Law
`a_one_step_tool_commits_one_transaction_of_its_leaves` in `TM/🧪️tests/🔬️unit/🦀️.rs`. **Uncompiled** (no train verdict read
for the line yet; activation B2 holds the tree since 11:21).

**Ports written, all UNVERIFIED (rule 68: no cargo while `activation.flag` exists)**
| Tool | Script | What changed | Law |
|---|---|---|---|
| block 3d surface click | `T/🧪️s5-tools-block3d-place.py` (applied 11:12) | `📍️place-vortex/🦀️.rs`: `Emit::tool_once(BLOCK3D_PLAY_APP_ID, "worldSurfacePlace", seed, operations)` instead of a plain emit | `a_surface_click_is_one_tool_transaction_of_its_mutations` (mounted dispatch: one edit, stamped `block3d-play#worldSurfacePlace`) |
| wfc grid 2d Pin / Mask / Select | `T/🧪️s5-tools-wfc-grid-tools.py` (applied 11:25) | `Grid2dEditor::armed_tool` for the pick verb and the raw canvas press; `TOOL_APP_ID` | `a_cell_click_of_an_armed_utility_is_one_tool_transaction` (pin + mask: one transaction `…#<utility>` of one leaf; select: zero trace; no admission: plain) |
| wfc grid 3d Pin / Mask / Select | same script | `grid3d_command_emit` pick arm; `GRID3D_EDITOR_APP_ID` | same law name in the grid 3d unit tests |

Block 3d `🖌️hover-surface` only writes the window's brush preview (no document mutation), so it is not an offender.
"Abort / freeze = none" for a one-click tool is the runtime's: a frozen document refuses the emit.

**Energy §22.20**: `✂️disconnect-referenced` and `🌤️unbind-weather-file` descriptors carry `"editable": false`; RAN
`schema mutation-inputs --under ✏️s/🔌️plugins/🔋️energy` → 2 leaves withdraw-only, **0 `inputless`**; 22 other findings remain
(numeric declarations — owed).

**Owed reds — still NO VERDICT.** Run 3 (10:29, cold private dir) was killed by the harness at 11:09 after 40 min without an
error; run 4 (11:12:25, warm, same five crates + the one-step tool + the block 3d port) was already running when the B2 flag
appeared and is allowed to finish; the harness stops it at ~11:52. Output: `🗑️generated/s5-tools/check-reds-4.txt`.

**After "SERVE UP (B2)" — exact commands** (private dir, no gate):
`P=…/🗑️generated/s5-tools/target; CARGO_TARGET_DIR=$P CARGO_BUILD_BUILD_DIR=$P CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model -p semio-s-artifact-gis-gismap -p semio-s-artifact-animate-presentation -p semio-s-artifact-block-3d -p semio-s-artifact-sourcing-curation --lib --keep-going`
→ then `… -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-grid3d --lib` → then (≥ 25 GiB free) the three laws:
`… cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-block-3d --lib -- a_surface_click_is_one_tool_transaction`,
`… -p semio-s-artifact-wfc-grid2d --lib -- a_cell_click_of_an_armed_utility`, `… -p semio-s-artifact-wfc-grid3d --lib -- a_cell_click_of_an_armed_utility`,
and `… cargo test -p semio-framework-tool-machine --lib -- a_one_step_tool`. Restore of each port = its script's inverse by hand
(the scripts are forward-only; the pre-port text is in `git diff`).

**Noted from S5-NESTED for playbook F2**: derived child ids must be unique per document — two sibling members with equal
derived content collide as `DuplicateMember`; the forms child of a playbook block needs its block id in the derivation.


### S5.15 USAGE STOP 11:30 — state for the 14:20 resume
- **Reds: VERDICT GREEN at `--lib`.** Run 4 (private dir, started 11:12:25 before the B2 flag) ended 11:29: `Finished dev profile in 17m 17s`, zero `error[`, zero `could not compile` in `🗑️generated/s5-tools/check-reds-4.txt` for `-p semio-s-artifact-energy-model -p semio-s-artifact-gis-gismap -p semio-s-artifact-animate-presentation -p semio-s-artifact-block-3d -p semio-s-artifact-sourcing-curation --lib --keep-going`. It compiled the one-step tool (`tool_once_emit`, `Emit::tool_once`), the block 3d port and the energy `editable: false` markers. sourcing-curation compiled without any edit of mine.
- **On disk, uncompiled:** wfc grid 2d + grid 3d ports and their two laws (`T/🧪️s5-tools-wfc-grid-tools.py`, applied 11:25, after run 4 started and outside its closure).
- **On disk, compiled but laws not run:** block 3d `a_surface_click_is_one_tool_transaction_of_its_mutations`; tool-machine `a_one_step_tool_commits_one_transaction_of_its_leaves` (test target not built); plugin `gesture_laws` incl. the press law (needs S5-RUNTIME's test binary).
- **Nothing is staged-only**: every script of mine reports 0 pending edits on today's tree.
- **Owed, in order** (private dir `P=T/🗑️generated/s5-tools/target`, both `CARGO_TARGET_DIR=$P CARGO_BUILD_BUILD_DIR=$P`, `CARGO_BUILD_JOBS=3`, after "SERVE UP (B2)"): (1) `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-grid3d --lib`; (2) `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-gis -p semio-hub-energy -p semio-hub-block --lib --keep-going`, then the other nine hubs ≤ 3 per call → "COMPOSITION GREEN <plugin>"; (3) tests at ≥ 25 GiB free: `cargo test -p semio-framework-tool-machine --lib -- a_one_step_tool`, `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-block-3d --lib -- a_surface_click_is_one_tool_transaction`, `… -p semio-s-artifact-wfc-grid2d --lib -- a_cell_click_of_an_armed_utility`, `… -p semio-s-artifact-wfc-grid3d --lib -- a_cell_click_of_an_armed_utility`, and `gesture_laws` on RUNTIME's binary → "PRESS GREEN".
- **Not started:** `ArtifactView::provisional()`, the streamed-gesture ports onto the slot (fem, lowpoly, generation3d, layout, note, draw), the Frozen-arm deletions, block 2d board producer, F21 `emit`/`schema`, the 22 + 8 input findings, D11, D24. Delete `🗑️generated/s5-tools/target` (≈ 1 GiB) when the owed checks are done.
