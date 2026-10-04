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
