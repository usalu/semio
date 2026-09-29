# WP-WG11 — wgpu Shells (wasm32 Browser + Native winit) Collaborate Over the Hub, Session 14

Slice WG11 (session 14, 2026-09-27 18:2x), successor of WG9 ([`📓️wp-wg9.md`](📓️wp-wg9.md), wasm32 browser shell) and WG10
([`📓️wp-wg10.md`](📓️wp-wg10.md), native winit shell + cross-shell with React). Coordinator = main chat. Rules:
[`📓️session-14-preamble.md`](📓️session-14-preamble.md). Ports: hubs 8050–8059, serves 6550–6559; 7800 = W4 (read-only live use).
Scripts `wp-wg11/`, captures `wp-wg11/generated/` (expendable), durable logs `.🧬semio/🌐hub/s14-wg11-logs/`. Private cargo
target `.tmp-ticket/wp-wg11/target`. Landing rows: [`📓️landing.md`](📓️landing.md) `# Session 14`.

## Session 14d

Resumed 2026-09-29 06:4x (after the usage cut, 4 kernel panics and the 01:14 external sweep, which deleted `wp-wg11/generated/` and
`wp-wg11/w3-backup/` — captures now ONLY under `.🧬semio/🌐hub/s14-wg11-*`, rule 26). All WG11 T3 sets LANDED (L1); LW1 ran the laws
(`📓️wp-lw1.md`, `.🧬semio/🌐hub/s14-lw1-logs/wg11-laws-{1..5}.txt`). Renderer/shell are guest code → fixes are sets for the first train after
the chain.

| # | Item | Status |
|---|------|--------|
| 1 | shell-turn law overflows its own 1 MiB thread in debug (`a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread`) → find the deep frame, root fix | **root found + set prepared** `wp-wg11/wg11-shell-footprint-patch.py` (5 files, dry run clean, Rust parses): `ShellState` = 400 888 B (icon-export batch 278 008 inlining four phase owners; World3d map inlining two whole entries; World3d retirement owner) → `ShellState::new` frame 803 KB + law closure 410 KB > 1 MiB before dispatch; boxing the phase owners → ≈ 47 KB (computed from the type-size layout); slot-budget pins updated to measured; new size law ≤ 64 KiB |
| 2 | `ui_value_to_dsl` (wgpu reconcile) turns integer UiValue args into floats (`row: 7` → `Float(7.0)`) → numeric conversion fix + law | **prepared** `wp-wg11/wg11-json-number-patch.py` (6 files, dry run clean on the post-T5 tree): ONE bridge `DslValue::json_number` (value crate) used by the wgpu reconcile AND the SDK typed-intent producer (same defect); semantics proven 11/11 vs Python `json.loads` of the JS text (bun `JSON.stringify` column); repo compile + laws = first train after the chain |
| 3 | renderer-wgpu full lib 1366 pass / 183 fail + 4 aborts → triage by cause, root fixes | triaged: 134 of 187 are artefacts of a one-process run (process-global engine state; canonical runner = nextest per process); **53 real** reds grouped (log 07:0x). Sets prepared (dry run clean on post-T5 live, applied together in WG11's overlay): `wg11-renderer-test-drift-patch.py` (rule 22, 9 files: corrupt GLB PNG checksums, swapchain/watchdog/hub source-law slices, winit theme tag, reserved-ids fixture, locale Mode identity, panel record key, icon stroke, TextEditor presentation), `wg11-renderer-product-patch.py` (T6, 5 files / 2 crates: hub task region, `Shell …` diagnostic, caret by identity not scene clone, compile via `KernelPoolFuture`, **world3d shadow pipeline lacked `@location(10)`** — every headless/native `GpuContext` panicked), `wg11-display-element-ids-patch.py` (T6, 7 files: one element-id scheme React+wgpu, schema `elementId`); Marketplace windowing on U6's RowTarget (T6, after U6) — next; remaining singletons under runtime proof (overlay build 3) |

### Session 14d log

- 06:5x durable copy of LW1's 06:42 renderer lib-test binary (current tree after T4): `.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-0642`
  (analysis without cargo). Failure clusters of `wg11-laws-5.txt` (183): presented-input candidate not sealed ~40, component/tree pointer
  document faulted 26, retained presentation/paint faulted 10, resident capacity exhausted 14, ~90 singletons across interpreter/engine_canvas/
  scenes/os_host/native_accessibility.
- 06:5x item 2: root = the UI contract carries every JSON number as `UiValue::Number(f64)` and TWO bridges turned integral values
  into floats — the wgpu reconcile's `ui_value_to_dsl` (`serde_json::to_value` → `Float(7.0)`) and the SDK's typed-intent producer
  (`DslValue::float`) — while React's JSON text wire delivers `7`; stdio (`window_kit_required_index_argument`) and tool-run
  (`tool_run_arg_u64`) had grown per-call-site integral-float tolerance for exactly this. Set `wg11-json-number-patch.py`: `DslValue::
  json_number(f64)` (safe integer → UInt/Int, non-finite → Null, else Float) + both bridges; shared vectors in `🌱️value/🧫️fixtures/
  🔣️json-projection/🔣️.json` `numbers` (TS: `JSON.stringify`, Rust: vs serde_json parse). Semantics proof (no repo build): standalone
  rustc of the exact function vs Python's `json.loads` of bun's `JSON.stringify` output → 11/11 (`.🧬semio/🌐hub/s14-wg11-scratch/json-number/`).
  The tolerant decoders stay (not mine; now redundant for UiValue-born numbers).
- 07:0x item 3 FINDING: the canonical renderer test runner (`runCargoTestBudgeted`, `📚️library/🟦️.ts`) runs cargo **nextest** (one
  PROCESS per test) with `RUST_MIN_STACK` = 128 MiB; LW1 ran plain `cargo test` (all tests in ONE process, parallel threads, default
  stack). Since 09-06 the renderer's retained engine/interpreter/scene state lives in process-global `WorkerCell`s (worker-safe for the
  shared pool), so a one-process run makes tests share one UI engine: foreign seals ("presented input candidate could not be sealed —
  existing seals" ×45 trace lines), foreign windows filling resident capacity, faulted pointer documents. Re-run of all 187 (183 + the 4
  aborters) ISOLATED, one process each, 128 MiB (the canonical conditions) with the durable binary: **134 PASS, 52 FAIL, 1 abort** (the
  shell-turn law) — `.🧬semio/🌐hub/s14-wg11-captures/isolated-1.tsv`. The 53 real reds: marketplace panel > 128 document nodes ×4, display
  projection/template ids ×3, TextEditor fixture identity ×4, scene accessibility buttons (EventFeed, GraphTimeline, Table row/editable text,
  icon-export cancel) ×6, native AccessKit ×2, source-text laws ×4 (executor bridge `block_on`, legacy scene queue, directory FIFO browser
  gate, bilingual chrome phrases), slot-table stack budgets ×2, GPU icon export (wgpu validation) ×2, ~26 singletons (camera math, footer
  bands, pane chips, watchdog signature, swapchain acquire, GLB texture decode, winit theme reason, …).
- 07:0x item 1: `-Zprint-type-sizes` of the post-T4 renderer (`s14-wg11-captures/type-sizes-1.txt`, lane 162 s) + frame table
  (`frames-0642-all.txt`): the shell-turn boxing WORKED (`dispatch_action` poll 841 KB → 76 KB); the new overflow is `ShellState` itself
  (400 888 B: `icon_export` 278 008 = `rendering` 102 704 + `source` GpuContext 100 904 + `preparing` 45 280 + `scene` 26 984;
  `world3d_states` 54 856 = inline rejected 25 872 + retired 25 864; `component_world3d_retirement` 25 864) → `ShellState::new` frame 803 KB +
  the law's closure 410 KB > 1 MiB before `dispatch_action` runs. Set `wg11-shell-footprint-patch.py`: those phase owners boxed (exist only
  while their phase runs) → ShellState ≈ 47 472 B; slot-budget pins (`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots`) to the measured sizes (World3d
  entry 25 872, map owner 3 136, EngineSurfaceSlot 80 288 / owner 32 — the latter two also clear the 2 red slot-budget laws); law
  `a_shell_state_stays_small_enough_to_live_on_a_bounded_thread` (≤ 64 KiB). Also found: frames grew a lot since 09-28 14:23
  (`RuntimeApply::start_frame_deferred` 4.9 → 13.2 MB, `restore_presenter_interaction_step` 4.2 → 11.3 MB) — runtime pool types grew;
  canonical tests hide it under RUST_MIN_STACK = 128 MiB (noted, not in this set).
- 07:1x relayed main (items 1–3 status). Coordinator 07:2x: Marketplace = windowed Tree section on U6's (a') RowTarget + RowAction
  (U6 adds an enabled flag) ordered AFTER U6's set in T6, no inline-toolbar interim, no raised ceiling; Display ids = ONE element-id
  scheme on both renderers from the schema. All WG11 sets → L1, first post-chain train (T6).
- 07:2x item 3 root causes (static, each confirmed against the source and git history):
  - GLB material fixture (09-28, new): embedded PNG bufferView 8 has a wrong zlib Adler-32 (`05fe02fe`, scanline `01 00ff00ff` sums to
    `04060200`) and so a wrong IDAT CRC — Python `zlib` refuses it too; fixture repaired byte-exactly (same pixel).
  - `world3d_shadow_pipeline`: `vs_shadow` takes `InstanceInput` whose `@location(10) emissive_cutoff` only the main/translucent
    hand-copied layouts declared → `create_render_pipeline` Validation Error → native panic in wgpu's default handler (both GPU icon
    export laws; any native `GpuContext::from_device`), WebGPU silently invalid shadow pipeline. Fix = the one shared
    `world_vertex_layout()/world_instance_layout()` pair for all three.
  - Display ids: wgpu composed template segments through `element_id_segment` (09-28), React glued raw ids, fixture pinned raw → one
    grammar both sides (`childElementId`/`child_element_id`), schema `definitions.elementId` = `ELEMENT_ID_PATTERN`, fixture rewritten.
  - Marketplace > 128 nodes: the generated activation catalogue (`📇️registry/🤖️generated/🖥️hosts`, 09-29 04:27) now names 44 plugins,
    3–4 records each (row + toolbar + buttons) → needs windowing (coordinator decision above).
  - Source laws broken by my own T3 shell-turn set: the hub sign-in law split on `async fn handle_hub_workspace_action(` (now
    `fn …<'a>(…) -> ShellTurn`); all other `async fn` anchors laws name still exist (checked).
  - `dock_mode_layout_identity` strips titles by design since 09-27 ("React Mode identity"); the locale law still read titles.
  - TextEditor focus is presented-tree addressed since 09-27 (`presented_document_id`), the seeded fixture window was never presented.
  - Peer ticket WGPU-RENDERER-REACT-PARITY (astra/sol, last report 09-28 05:00, "No native pass is claimed") introduced the icon stroke,
    the svg/graph/text layout changes and several of the remaining singletons without a native run.
- 07:3x overlay sync now also mirrors gitignored generated inputs (`*.rs` + everything under `🤖️generated/`: tokens, icon SVGs, registry
  tables) — the first two overlay builds failed on `🔤️tokens/🦀️.rs` and the generated icon SVGs; overlay dev loop is incremental (private
  build-dir). Build 3 queued: renderer lib tests + every red in its own process (`wg11-isolated.sh`, `isolated-2.tsv`), ui reconcile laws,
  value `json_number` law.
- 07:4x coordinator: approved the FULL P5 cut (wgpu twin of `treeWindowBodyRequestsV1` + served memory, shared fixture + cost model,
  fed by the retained tree's measured viewport/scroll, driving shell-owned windowed panels AND guest `tree_windows`/
  `tree_viewport_rows`; laws: 10 000-row end reached, served window ≤ body budget, React ≡ wgpu for the same viewport vector) and a
  native pipeline-creation law; Marketplace = windowed section on U6's RowTarget + `RowAction.disabled` (U6 relay), ordered AFTER U6.
- 07:49 overlay build 3 (all five sets applied, private build-dir): renderer lib tests COMPILE (8 min); every red in its own process
  (`isolated-2.tsv`): **22 of 54 now green** — the 1 MiB shell-turn law, the size law, both slot budgets, TextEditor ×4, Display ×2, GLB,
  executor bridge, watchdog, swapchain, directory FIFO, chrome phrase, hub sign-in, winit theme, reserved ids, locale identity,
  command-dock form, icon lighting. New finds from that run: (a) `world3d_painted_shader` — its `.replacen` anchor for the emissive
  line no longer matched (the lit shader gained `+ in.emissive_cutoff.rgb`), `lit_color` was silently never declared → every GPU
  context refused the painted shader module: edits become one table `WORLD3D_PAINTED_SHADER_EDITS` + CPU law (every anchor exactly
  once) — in the product set; (b) the value law's fixture rewrite dropped the closing brace (my own set bug, caught by the proof —
  fixed + a read-back guard); (c) `the_reserved_media_transport_projects_only_truthful_localized_host_status` red for the SAME root as
  item 2: an Extension's `params_json` is `serde_json::to_string(&UiValue)` and the derived serializer writes `1.0`, so every valid
  media transport read "invalid contract" → item 2's set now fixes the ROOT: one rule `protocol::value::json_integer` (no DslValue in
  its signature, so the UI contract may use it), a hand-written `Serialize for UiValue` writing integers as JSON integers, and
  `DslValue::json_number` on the same rule (the wgpu reconcile needs no change of its own any more); (d) the projection pane parks in
  `layout` for 1024 opportunities (0.2 s) — the retained layout is pool-driven now and the pane's opportunity cap counts spins;
  (e) the ingress law's legacy helper list names helpers deleted on 09-26 (drift, fixed).
- 08:0x P5 written: `wg11-tree-window-patch.py` (ui crate: rule module + neutral vectors/schema written by React's rule, spacer geometry
  in the retained layout, `Ui::tree_window_measures`, React law reads the same vectors) and `wg11-tree-window-shell-patch.py` (Shell:
  scheduler twin of `createTreeWindowSchedulerV1`, observer on every completed body paint, settle-pump driven, `live_view_state` stamps
  the fields — GAP closed); both dry-run clean, queued in overlay build 4 with overlay-only `[DEBUG]` instrumentation for the
  remaining runtime reds.
- 08:2x overlay build 4 (`s14-wg11-captures/overlay-dev-4.txt`, `reds-6-panics.txt`): renderer lib compiles with EVERY set; green
  now: P5 scheduler 3/3, native AX ×2, receipts, ingress, icon GPU export ×2; ui lib failed on two flex-unit `TreeRow` literals
  without `lead` → tree-window set gains the literals, a flex lead law (both flows) and the rewritten P5 GAP doc in `🧩️component`.
- 08:3x–08:5x remaining reds root-caused (debug + reading): (a) since 09-21 a retained scene's host id is ENGINE-MINTED and since
  09-27 input/AX resolve in the PRESENTED tree and only ACCEPTED scene cells publish — the EventFeed / Table button / GraphTimeline /
  Table editable ×2 / NodeGraph caret wiring laws staged under the authored host id and drove never-presented windows → shared
  helpers in test-drift (`present_seeded_scene_window`, `retained_scene`, `rebased_scene_key`, `accept_table_editable_text_cells`);
  (b) map pinch: map camera clamps to cover (800×600 ⇒ zoom ≥ 400), law seeds relative; (c) export-batch law never `publish_hits`;
  (d) panel tabs are `aria-pressed` since 09-26 (settings law); (e) display-transfer law cleared deferred actions but not the paired
  journal debt; (f) refused-peer law predates WG8's detached settle refresh → bounded settle drive. Test-drift now 18 files.
- 08:4x product set 2 `wp-wg11/wg11-renderer-product-2-patch.py` (8 files, dry run clean; after test-drift + product): SVG back-face sign
  (culled every front face), GraphTimeline chips measured at `text-2xs` with the measured track retained for hit/AX (was
  `0.43 × xs` per scalar: 85.8 vs React 80.9), centered band rounds INSIDE the band (TS + Rust twin + 2 shared fixture vectors),
  projection-body rows (`<surface>/<id>`) are chrome presses. Product set 1 gains pre-wrap hanging spaces (`a  b  c  d` law).
- Open (design needed): projection pane's 1024-opportunity budget counts worker-pool wait polls (712 layout yields, ~600 of them
  `WorkerPool`/`WorkerTake`/`WorkerOutcome`/`CloseSession` for two layouts — load-dependent); an undecodable image `data:` URL
  rejects its raster reservation → the upload cursor answers `Fault` → the frame transaction faults, the source is re-offered every
  paint and blocks the `ui-image` lane for every later image; the 65 535-scalar TextEditor close ladder exceeds 262 144 steps
  (measuring in build 5). Display fixture logs `panel 'framework.marketplace' exceeds 128 document nodes` (Marketplace item).
- 08:5x overlay build 5 queued (`overlay-dev-5.txt`): ui + renderer + contract + replication lib tests `--no-run` with all sets
  (+ product-2); tests run per process from the built binaries afterwards.
- 09:0x coordinator: L1 paused until window 4; register sets in `📓️t6-queue.md` (rows 7/8 updated: order, crates, laws); the three
  open design items are WG11's. Designs + sets (all dry-run clean on live except where noted; proven in overlay build 5/6):
  - `wg11-own-work-budget-patch.py` (ui engine + Interpreter + Shell + navbar laws): `UiLayoutStep::Awaiting` for a pool wait;
    `UiDocumentFrameCursor::last_step_was_own_work` (false for shared retirement lanes, pool waits and other surfaces' layout
    steps); the Shell's four retained-document budgets spend an opportunity only on own work (`document_opportunity_remains`).
  - `wg11-raster-refusal-patch.py` (ui prepared + Scenes + Interpreter + 2 law files): `PreparedRasterRejected::is_content_refusal`;
    `queue_canvas_image_upload_with` → `Result<String, RasterUploadRefusal::{Busy, Invalid}>`; refused owners park in a fixed
    per-surface refusal ring retired silently by the upload cursor (admission keeps a slot) — only authority faults still fault the
    frame; the ui-image resolver keeps `Invalid` sources per id (`UI_IMAGE_REFUSED`) instead of re-offering them every paint.
  - `wg11-engine-close-pages-patch.py` (EngineCanvas + ✍️editor + laws): every scalar-popping close helper releases a 64 KiB page
    per grant (`retire_string_page`/`retire_bytes_page`, `EDITOR_RETIREMENT_PAGE_BYTES`) — popping a scalar never freed memory.
  - `wg11-marketplace-window-patch.py` (T6 row 8; after U6 + tree-window-shell; dry-run clean against U6's overlay via `--base`):
    rows carry row actions on ONE target with disabled verbs, one windowed roster section fed by the P5 scheduler, windowed shell
    bodies publish as scroll roots, `PanelProjection::tree_item` projects `actions` → `row_actions` + one `RowTarget`.
- 09:1x full renderer lib, per process, on the build-4 binary (`s14-wg11-captures/full4-c{0..3}.tsv`, 1 568 tests): 28 reds, all
  root-caused above, plus two new ones found by the full run — `tool_run_panel_*` ×2 pinned the OLD float echo (`generation: 0.0`)
  that json-number fixes; the json-number set now updates them to the JSON integer React sends. Marketplace ×3 +
  `every_shell_owned_leaf_projects_into_retained_records` (`exceeds 128 document nodes`) are row 8's. TS laws in the overlay
  (`overlay-ts-2.txt`, ignored wasm bindings mirrored): ChromePanels/display ids 11/11, engine-contract Display+delivery 20/20,
  Interpreter tree-window vectors 28/28. Finding (not WG11's): the value module's TS laws (`testSharedDynamicValueOracle`,
  `testPagedListOwnership`) are exported functions no runner calls — run by hand in the overlay with bun: PASS.
- U6's own ui run (`s14-u6-logs/t4-q6-prove2.txt`) lists six ui-crate reds; four are renderer-owned and checked in WG11's build 5 ui
  run: `world_mesh_instance_packs_policy_and_standard_material_without_stride_growth` (the 09-28 GLB material channel grew the
  stride 96 → 112), `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` (slot-size fixture),
  `every_world_color_cursor_uses_the_encoded_composite_attachment`, `an_enabled_shadow_pass_measures_every_caster_before_its_receivers`.
- 09:21 overlay build 5 (`overlay-dev-5.txt`): ALL 12 sets compile (json-number, shell-footprint, test-drift, product, product-2,
  raster-refusal, display ids, tree-window, tree-window-shell, own-work-budget, engine-close-pages + debug). Per process:
  ui lib 739/743 (`ui5-c*.tsv`; reds = four 09-28 law drifts: stride 96→112, encoded-pipeline list +4 authored pipelines, prepared
  order label `world-postprocess`, slot budget 164 672→164 712 — all added to test-drift, now 22 files); renderer lib 1 556/1 570
  (`full5-c*.tsv`): fixed and green — display transfer, refused peer, settings, icon export cancel, footer bands, pane chip, diff
  pre-wrap, SVG fill, raster avatar, draft-pages close, closed_world3d, table button + graph checkpoint wiring, tool-run (json-number).
  Remaining 10 → second round of drift edits in test-drift (EventFeed action names the MOUNTED surface id; table draft seeds before
  the edit; NodeGraph cadence API answers `Some((surface, None))`; AX value publishes through the retained page; the sibling law
  drains its retired World3d owner before the shell drops) + build-6 debug for the projection pane (now parks in PAINT once layout
  waits stopped counting), GraphTimeline shaped track numbers, and the board half of the pinch law. Marketplace ×3 +
  `every_shell_owned_leaf_projects_into_retained_records` are row 8's.
- 10:27 (after the session cut; disk cleanup removed the overlay build outputs) build 6 queued behind lb2/c12/p9 (cold).
- 11:0x T7b set `wp-wg11/wg11-text-kerning-patch.py` (after T7a; 9 files, crate semio-framework-ui; dry run clean on the overlay with
  all 12 sets + T7a; rustfmt-clean; joined build 6's patch list so T7a and T7b prove in one lane hold). Probes (`text/`): Chromium
  kerning is PAIRWISE for Anta/Share Tech Mono — every corpus row's kerned width = unkerned + Σ adjacent pair kerns within 0.008 px
  (`kerning-pairs-probe.ts`); opentype.js 1.3.4 under-applies Anta's GPOS (2.7 px off) so it is NOT an oracle — Chromium is.
  Design: `text::PairKerning` shapes the pair alone (swash, design units, liga/clig/calt off) less its nominal advances, faces held
  once so the context stays warm; the atlas caches it per face+pair (bounded, bitmap never kerns); ONE rule (`pen_kerning`: kern
  against the previous scalar once the pen left the line start) in measure, range, pre-wrap and greedy wrap, the retained painter
  (wrap pricing, overflow, pen) and the widget pen loops (now one `pen_glyph_run`); `pen_advance`/`pen_at` give the per-scalar walk
  and the caret x (a caret after a kerned pair includes it, as Chromium's); the layout worker carries each glyph's kerning to the
  next scalar of its run and adds it only while both scalars share a line. Corpus (T7b) = T7a's 120 rows + one strongly kerned
  en/de pair (`AVATAR To Wave`, `VATER, Tätowierung`) with a `kernedWidthPx` column: 54/144 rows kern, up to 9.5 px. Laws: T7a's
  unkerned law now sums advances; ADDED kerned-column law, one-source law (A+V −0.0928, f+o −0.0391 em = Chromium), measure/wrap/
  caret law, painter law (glyph x = kerned caret; kerned wrap keeps `over`), mounted-layout worker law. TS law (overlay): 2/2 incl.
  Chromium re-measure of both columns. Product set: the pre-wrap law's `"one\n\ntwo"` was emitted as raw newlines — now escaped.
- 11:1x rustfmt pass over every set (`wp-wg11/wg11-fmt-sets.py`: rustfmt --check in the overlay vs the live tree, each group only
  the overlay shows is rewritten in the set's replacement text): 15 groups auto-fixed + 7 by hand (import order in the Tree
  target and mounted layout, `UiLayoutStep` stays compact with `Awaiting` documented on the enum, the three world pipelines'
  `VertexState` collapse after the shared layouts, the editor close condition, the Scenes admission chain, the Display windows
  section literal). Remaining rustfmt groups in touched files are pre-existing (icon-export's compact arms, export-batch law).
  All 13 sets re-applied cleanly to a fresh overlay sync. Incident: that manual sync+apply overlapped build 6's own hold
  (granted 11:13:12, cargo from 11:13:33, cold deps) — it rewrote the set files at 11:14:18 (same final content) and dropped the
  debug lines, re-added at 11:15 before the renderer crate compiled; no set file changes again until build 6 ends.
- 11:13 build 6 HELD (`overlay-dev-6.txt`): all 13 sets (incl. T7a + T7b) + debug applied rc=0; ui lib → renderer lib → editor.
- 11:15 ui lib test binary (`ui-bin-6`, 751 tests, per process `ui6-c{0,1}.tsv`): **748 ok / 3 red**. All T7a + T7b laws GREEN —
  unkerned corpus (advance sum) and KERNED corpus (144 rows ≤ 0.5 px vs Chromium), quarter-pixel DPI key, exact fractional advances,
  one layout/atlas advance source, one kerning source (A+V, f+o = Chromium), measure/wrap/caret rule, retained painter at the kerned
  caret + kerned wrap, mounted-layout worker kerning. The 3 reds, all root-fixed in the sets (proof = build 7):
  (1) `ui_surface_slot_table_is_heap_first…` 164 712 → 165 840 B: every retained window's inline layout jobs carry the worker's
  shaping context → T7b pins the measured size in the boxed-fixed-slots fixture;
  (2) `section_and_field_measure_the_shared_wrapped_chrome_fixture` (field hug 83.2 vs 115.2): the fixture's 112 px field box was
  sized under the 0.625 em stand-in — Chromium sets "Shown before control" (111.9 px, 0.06 px from the break) and "Fix this value
  now" on ONE line there; T7a moves the box to 80 px where Chromium wraps both (probe `text/section-field-probe.ts`);
  (3) `section_and_field_chrome_paint_only_inside_their_measured_bands`: painted with the fixed-pitch bitmap atlas a layout priced in
  Anta's advances — T7a paints it with `FontAtlas::shaped_default` (the layout's own advance source). Finding (no gap): Chromium's
  synthetic bold does NOT widen Anta's advances (`text/synthetic-bold-probe.ts`), so the faux semibold keeping advances is right.
- 11:22 TS laws in the overlay (`overlay-ts-6.txt`): ChromePanels 11/11, engine-contract 20/20, Interpreter tree windows 28/28,
  text corpus (both columns, Chromium re-measure) + navbar band + GraphTimeline layout 26/26.
- 11:24–11:35 renderer lib per process (`renderer-bin-6`, `full6-c{0..3}.tsv`): **1 561 / 1 570**. Green now: all second-round
  wiring laws (EventFeed, table draft/Enter/Escape, AX value echo, NodeGraph cadence), `a_focused_world_window_does_not_retire…`,
  GraphTimeline `shared_layout_contract…` (track 80.959 vs React 80.9375 — the T7 criterion), SVG, footer bands, map pinch. Reds:
  marketplace ×3 + panel-anchor (row 8, U6); `icon_gpu_export_cancellation…` (8.7 ms > 2 ms ceiling at load 21; green twice in
  isolation — load flake); and three root-fixed in sets for build 7:
  (a) board pinch — `[DEBUG] board down scene.2.1 hosted=true drag_active=false`: a backdrop press opens
  `Interaction::SelectionPending` (click-vs-marquee threshold) which `board_drag_active` did not count, so a release off-surface
  before the threshold never reached the host → NEW set `wg11-board-lane-patch.py` (T6 row 7, independent):
  `BoardHost::pointer_lane_in_flight` (pending or live area select + every `defers_descriptor_sync_from_js` gesture), EngineCanvas
  asks it (twin of `tiled_map_drag_active`, live from the map's press);
  (b) `checkpoint_regions_match_react…` — the fixture's `x` (read only by the wgpu law; React's twin clicks DOM regions) put the
  selectable edge at 157, a boundary no React geometry makes: React's checkpoint spans the `auto` label column (`Head` 23.33 +
  `px-1.5`×2 + `px-single`×2 = 41.70, LayoutUnit 3.1875) and the 96 px graph column from 23.19 → 160.89 (probe
  `text/chip-probe.ts`). product-2 re-derives `graph-select` 160.5 / inert 162.5 (clear of T6's quantized 161.9); T7a tightens
  the inert sample to 161.25 (exact advances land at 160.93);
  (c) projection pane ×2 — paint advances one grant per step (`[DEBUG] paint … progress 0…418`, `proj6.txt`) and faulted at
  the 1 024 ceiling: the converging body needs ~1 034 OWN opportunities (~600 layout = worker outcome take + apply per job step
  for each scalar and node stage, ~430 paint). The ceiling's derivation ("15 rows × 9 phases + one grant per scalar fit several
  times over") omitted the worker round trips → own-work-budget sets it to 4 096 (≈4× the measured need; the pane is React's
  fixed taxonomy). Build 7 prints the pane's own count (overlay-only debug).
- 11:35 build 7 queued (7th in FIFO): 14 sets (+ board-lane) + T7 fixes + debug (projection layout steps + pane count).
  Chain verified in a fresh overlay sync (all rc=0) while far from the lane head; TS twins of the changed fixtures (section-field,
  checkpoint-hit, text corpus) 19/19.
- 12:00 build 7 HELD (`overlay-dev-7.txt`, all 14 sets + T7 + debug rc=0; ui 7 s, renderer 28 s incremental — binaries verified to
  carry the new code). **ui lib 751/751** per process (`ui7-c{0,1}.tsv`) — T7a + T7b fully green incl. the three former reds.
  **renderer lib 1 564/1 570** (`full7-c{0..3}.tsv`): board pinch ✓ (board-lane), checkpoint regions ✓ (fixture re-derived),
  icon-export timing green; left = row 8 ×4 and projection ×2, which now COMPLETE (own work 1 405 < 4 096) but publish 19 row hits
  against React's 15: the four group rows also carry their disclosure hit (`tree.chevron.<id>`, the retained-section-collapse
  design) and the laws counted every hit under the pane parent → test-drift filters row (label) hits. Debug trace (`proj7.txt`):
  two layout passes (the pane re-solves once it hugs its measured content), 600 pool waits not counted, ~800 paint grants.
- 12:04 build 8 queued (projection law filter + measured budget doc).
- 12:50 build 8 (`overlay-dev-8.txt`): renderer 1 564/1 570 (`full8-c*.tsv`); the projection laws now count 15 label rows, sit
  inside the pane, read top-down — and stop on the pane's ROW IDS: `world_projection_pane_rows` glued the raw template id
  (`…paneTop.three-point`) while React's `projection-pane` fixture, the Display rows and the Shell's own default-selection stamp
  (`element_id_segment`) use the element-id grammar (`…paneTop.threePoint`) — so the stamped selection never matched a row
  (fold law `selected == None`) and the declared-order lookup found nothing. Root fix joins the display-element-ids set (one
  element-id scheme): the pane rows compose through `child_element_id`. Build 9 queued 12:52.

## Session 14c

Successor agent (2026-09-28 16:5x, second restart; chain relaunched 16:55:46, GUEST FREEZE ON, 7800 DOWN until the chain moves it onto ALL).

| # | Item | Status |
|---|------|--------|
| 0 | Reconcile the predecessor's last step (`native-react-cursors` own journey/check, `--mode` dropped) | **staged only, complete**: no tree file touched (target dir absent, no verb in the renderer `📜️script.ts`); staged harness has no `--mode`, journey `native-react-cursors` → `wgpu-peer-cursors-native-react`; tsc **0 errors** 16:59 (`wp-wg11/generated/s14c/tsc-harness-5.txt`); landing dry run clean (2 files, 1 new; `s14c/harness-land-dryrun.txt`) |
| 1 | Window-3 sets dry-run clean on the live tree | **all clean again 00:0x on the T1-written tree** (+ table painter); full WG11 T3 chain composed on a scratch mirror (order: shell-turn LAST, relayed to L1). Earlier: **all 5 clean 17:02** (`wp-wg11/generated/s14c/*-dryrun.txt`): reseed (11 files), board-pointer (3), a11y text-name (3), shell-turn (2 files, 12 turns; diff identical to 14b's), harness landing (2) |
| 2 | wgpu TableRow painter (with LB2's SDK half, window 3) | **prepared** `wp-wg11/wg11-table-row-painter-patch.py` (10 files + 2 new, dry run clean, all patched Rust parses — nightly rustfmt, fmt-clean in its own hunks); compile + laws = window 3 (native lane, `semio-framework-ui --features wgpu-engine`), lands in ONE step with LB2's `lb2-p3-row-actions.py` |
| 3 | Live suite on 7800 p24 (channel 19), catalog-exact roots | wasm32 en **24/24**, de **24/24** (1 late-joiner flake 22/24 before); agent-pixels **12/12** (live == cold reference 0 px); wasm32 ↔ native **11/11**; native ↔ React **8/8**; native ↔ React cursors **4/4**; native two-shell gate **12/12**; wasm32 ↔ React **13/14** (a11y counts paragraph unnamed → text-name set); wasm32 cursors **5/7** (frozen presence pointer → board-pointer set). Native runs need RUST_MIN_STACK 8 MiB (default overflows → shell-turn set). All sets land via L1 T3 |
| 4 | wgpu shell drops `os.local-catalog.*` + `os.create-space-artifact` (SH2) → T3 set | **prepared** `wp-wg11/wg11-replay-routes-patch.py` (4 files + 1 new; dry run clean, patched Rust parses; `--write` backs up to `wp-wg11/w3-backup/replay-routes/`, `--revert` restores) — create routed into the wgpu creation door, local-catalog + unknown `os.*` = typed localized refusals, replay arms on wasm32 too; relayed to L1 + main |
| 5 | G12 × WG11 joint T3 set: wgpu shell's agent-bridge offer scope | **prepared** `wp-wg11/wg11-offer-scope-patch.py` (11 files + 2 new; dry run clean; all patched Rust parses, the new kernel module fmt-clean; `--write` backs up to `wp-wg11/w3-backup/offer-scope/`, `--revert` restores) — relayed to L1 + G12 |

### Session 14c log

- 16:58–17:00 item 0 reconciled (see table): the harness edit lives only in `wp-wg11/harness/🤝️hub-collaboration/🟦️.ts` (13:18:46); the
  tree has neither `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/` nor the verb → nothing half-applied, nothing to revert.
- 17:0x–17:3x item 2 **TableRow painter prepared** (`wp-wg11/wg11-table-row-painter-patch.py`, dry run `generated/s14c/table-painter-dryrun.txt`,
  parse check `wg11-parse-check.py` → `generated/s14c/table-painter-parse.txt`: 11/11 parse). Measured first: besides the known Table/TableRow
  projection gap, **no retained tree row action was ever clickable or announced on wgpu** (paint drew the icons; no hit path in `events`, no
  registry entry, no accessibility node) — so the fix covers tree rows too. Design (supersedes the 14b sketch; column grid stays on the document):
  - `🔀️reconcile`: `Table` → `UiNode::Tree` with ONE section keyed by the table's record key (`window` = `Table.window`), rows mount directly under
    the table node; `TableRow` → tree item (name = first cell, `action` = record `Activate`, `actions` = `RowAction`s) + keyed identity row (the
    Button/`cells.join(" · ")` and bare-Stack arms go). `TableProps.columns/actions_label` + `TableRowProps.cells` are read through the node
    binding (`mounted_layout::document_table{,_row}`, `document_row_action`) — no second copy in the legacy spec, no `UiTreeItemNode` field change.
  - ONE grid (`🧮️layout::table_column_rect`, equal columns between the leading gap and the actions column = `table_actions_width(max row actions)`):
    `flex` `LayoutNodeKind::Tree{header}` reserves the header band, new `LayoutNodeKind::TableRow` flows an editable row's cell children in the same
    columns (basis 0 + grow); `paint` draws header labels + plain cells there and skips materialised cells (children paint themselves); a row whose
    cell hosts a surface (draft/read-only) is `TREE_DETAIL_HEIGHT` tall in layout AND paint.
  - ONE action geometry (`🧮️layout::tree_row_action_rect/tree_row_action_at`, `TreeRowMetrics.action_gap`): paint's icon slots, the pointer router
    (`EventRouter::pointer_row_action` → `row_action_command` = row address + the RowAction's OWN versioned `ActionId` — `build_intent` would have
    answered the row's `Activate` binding), a row with Row-placed actions is a hit target; accessibility walk adds virtual
    `<rowKey>::row-action::<i>` buttons named `"<label>: <row name>"` (React `TableView`), activated via the engine's virtual-key route.
  - Laws: NEW `🧪️tests/📊️table-row-grid` over NEW fixture `🧫️fixtures/📊️table-row-grid/🔣️.json` (metrics pin, rows/columns/editable cell rects,
    action slots, 5 clicks → fired action, painted glyph columns + no table glyphs in the editable row, a11y buttons + activation) + **taffy
    oracle** for the editable row's cells; the 2 reconcile laws re-pinned (identity row, one child per cell, RowAction as the row's action);
    flex-unit `Tree` literals gain `header: 0.0`. Expected numbers computed by hand from the theme tokens (row 24, gap 3.2, icon 12, pad 3.2).
  - Window-3 landing: `python3 wp-lb2/lb2-p3-row-actions.py --write` + `python3 wp-wg11/wg11-table-row-painter-patch.py --apply` in one step →
    native lane `cargo test -p semio-framework-ui --features wgpu-engine --lib -- table_row_grid reconcile::tests flex::tests
    accessibility` + `cargo check -p semio-framework-ui --features wgpu-engine --lib --tests` + LB2's SDK law; wasm32 = the renderer bundle in
    the chain's next rebuild; live = Home in the wgpu shell (SH2 verify-home). `--revert` restores exactly (embedded pre-image).
- 17:4x RELAY LB2 (via main): painter ready, landing = ONE window-3 step with `lb2-p3 --write`; runbook above. No tree edit by WG11 this session (no landing row). Other prepared sets touching the same wgpu files: none live (`wp-u5`, `wp-wg8` are landed history).
- 21:10 coordinator: 7800 READY on `s14-w4-catalog-p24` (channel 19, current-tree hub) → live suite. Catalog-exact module roots
  materialized from p24 (`wg11-all-roots.sh`, fixed a zsh `${@:-…}` word-split bug first; `s14-wg11-logs/all-roots-p24-2.txt`): note
  `284b3d18…`, block `f094ec31…`, puzzle `b62d4859…` (served == catalog, shard worker fresh). Note serve **6552** (pid 20118, renderer wasm-release 20:47).
- 21:22–21:29 **wasm32 journey `w-3` (en) on 7800 p24: 24/24 PASS, rc=0** (`s14-wg11-logs/harness-w-3.txt`, `wp-wg11/generated/hub-collaboration/w-3/`):
  sign-in ×2, attach ×2 `Persisted`, roster both ways, A→B 4.1 s, B→A 4.0 s, frames name the document, real 15 s cut (6 connections) never
  freezes A (1–2 ms), offline edit kept (`reconnecting` after 4.7 s), relink 3.2 s + B receives it, medium cut spoken after 2.1 s, relink 14.2 s,
  long cut expires B after 62.5 s (en line), never relinks by itself, late joiner = hub head 3. Record `wgpu-collaboration-wasm32-en` PASS.
- 21:30–21:39 wasm32 `w-4` (de): 22/24 — every step through the long-cut expiry (German line) PASS; the LATE JOINER's fresh session (7800 direct)
  activated `Remote` but the path editor/attach never appeared within 45 + 30 s (`pressed: absent`, card `Remote: getrennt`, final projection shows
  the sync panel collapsed to file/folder/remote) → late-joiner count red. The late shell's console was never written (harness gap) → harness now
  dumps `console-late.txt`/`hub-late.txt` (tsc 0, `generated/s14c/tsc-harness-6.txt`). Both runs log `ui-doc begin refused … fault=InterruptedClose`
  bursts (en 166, de 184 lines, windows `s-sync-status`, `framework.hub`, `framework.panel.artifact`) — retried and absorbed in passing runs.
- 21:40–21:48 **wasm32 `w-5` (de) re-run: 24/24 PASS, rc=0** (`harness-w-5.txt`): A→B 4.0 s, B→A 4.0 s, relink 3.8 s, expiry 62.5 s (German), late
  joiner 3 = head 3. Record `wgpu-collaboration-wasm32-de` PASS. The w-4 late-joiner miss did not reproduce (1 of 3 runs today) — recorded as an
  open flake with evidence `wp-wg11/generated/hub-collaboration/w-4/projection-A-538818.json`; the next occurrence carries the late console.
- 21:49–21:59 agent-pixels `ap-3` (en): 11/12 — agent = own delegated principal, roster names it `(AI agent)`, MCP `addBlock` SUCCEEDED, block decoded
  3.1 s later, head 0→1, frame changes 1 173 px; red "live == cold reference" (34 740 px) = HARNESS: the live frame had the Sync card open, the
  reference had it closed (`ap-3/agent-live.png` vs `agent-reference.png`: document content identical). Fix: `closeSyncCard()` judged by the painted
  `framework.sync.panel` group (not the pill's switch state) before both captures; tsc 0 (`generated/s14c/tsc-harness-7.txt`).
- 21:59–22:09 **agent-pixels `ap-4` (en): 12/12 PASS, rc=0** (`s14-wg11-logs/harness-ap-4.txt`): decoded 2.0 s after the commit, head 0→1, frame
  changes 1 173 px, **live frame == cold reference 0 px** (noise 0). Record `wgpu-agent-reply-pixels-en` PASS (rows 3.12, 4.10).
- 22:0x coordinator FYI: the chain's release-modules re-materialization was stopped at 32/60 (L1 landing T1); the shared release plugin-module root
  is partially refreshed — WG11's serves use the catalog-exact p24 roots + renderer wasm-release 20:47, unaffected.
- 22:10–22:16 cursors `cu-2` (puzzle2d, en; serve **6554** on `s14-wg11-all-puzzle`, note serve 6552 stopped first): **5/7 = current state** —
  sign-in ×2, attach ×2 `Persisted`, presence both ways PASS; peer cursors red both ways (pixel deltas 0/0/0, noise 0) = the frozen presence
  pointer the prepared board-pointer set fixes (lands via L1) (`s14-wg11-logs/harness-cu-2.txt`). Record `wgpu-peer-cursors-en` FAIL (expected).
- 22:16 block2d serve **6553** on `s14-wg11-all-block` (puzzle serve 6554 stopped first); catalog-exact native runtime `.🧬semio/🌐hub/
  s14-wg11-native-block2d/🔣️runtime.json` (block `f094ec31…` = the p24 catalog component + the materialized descriptor; the dev dist runtime still
  pointed at a 09-25 component). Native binary = `s14-wg11-bin/renderer-tests-1423` (channel 19).
- 22:17–22:22 wasm32 ↔ native `wn-2` (en, DEFAULT test-thread stack): native law `has overflowed its stack` → abort (3rd reproduction of item 3 on
  today's tree; the prepared shell-turn set lands via L1) — `wp-wg11/generated/hub-collaboration/wn-2/native.txt`.
- 22:27–22:39 **wasm32 ↔ native `wn-3` (en, `RUST_MIN_STACK=8388608` as a measurement condition): 11/11 PASS, rc=0** (`s14-wg11-logs/harness-wn-3.txt`):
  browser opens the native user's document live, presence both ways (8 ms), native edit → browser 6→7 (9 ms), browser edit → native ingests 7→8,
  each undoes only their own, reload converges (live 31.0 s) and the next native edit arrives, native law exit 0. Record
  `wgpu-collaboration-wasm32-native-en` PASS (row 3.8).
- ~22:42 MACHINE REBOOT (coordinator): every process died (serve 6553 pid 52059 included); all WG11 runs had finished before it (wn-3 rc=0 22:38:50;
  ap-4 12/12 at 22:09 — not lost). 7800 resumed on p24 (os-hub pid 2218). Post-reboot load 78 → waiting for < 40 before the next run.
- 22:5x React `s` dev serve **6555** (S18 `ensureDevServe`, joined to 7800, pid 4817; `wg11-react-serve.ts`).
- 22:55–23:06 **native ↔ React `nr-2` (en, RUST_MIN_STACK 8 MiB): 8/8 PASS, rc=0** (`s14-wg11-logs/harness-nr-2.txt`): React opens the native user's
  document `Persisted`, presence both ways (6 ms), native edit → React 6→7 (11 ms), React edit → native ingests 7→8, each undoes only their own
  (8→7→6), React reload converges (live 3.0 s) + next native edit arrives, native law exit 0. Record `wgpu-collaboration-native-react-en` PASS
  (row 3.5) — first native ↔ React pass on the channel-19 hub.
- 23:08–23:24 **native ↔ React cursors `nrc-1` (en, RUST_MIN_STACK 8 MiB): 4/4 PASS, rc=0** (`harness-nrc-1.txt`): React opens the native user's
  board live (6.3 s), cursors both ways (React sees the native pointer after 5 ms; native sees React's), native law exit 0 (754 s). Record
  `wgpu-peer-cursors-native-react-en` PASS (row 3.6). Contrast: wasm32 ↔ wasm32 cursors stay red (cu-2) — the frozen presence pointer lives in
  `AppInteractionState::handle_pointer_move` (the winit/browser-worker move path), which the native law's ShellState-level pointer does not cross.
- 23:2x coordinator (queued): implement `os.local-catalog.*` (SH2 route B) + `os.create-space-artifact` in the wgpu shell (no silent drop; parity
  with ShellHost or typed localized refusal), a law each, as a T3 set with `--write/--revert` + byte backups under `wp-wg11/w3-backup/` → next after the live runs.
- 23:23–23:28 wasm32 ↔ React `wr-3` (en; block2d serve 6553 pid 21451 + React 6555): **13/14** (`harness-wr-3.txt`) — presence both ways across shell
  types (`User One · User Two` / `UO UT`), wasm32 edit → React 6→7 (2 ms), React edit → wasm32 7→8 (1.0 s), each undoes its own (8→7, 7→6), late
  React joiner 6 = 6, late wasm32 joiner 6 = 6. Only red = the a11y step: the board's counts paragraph (`6 Handle Kinds, 11 Handles`) has NO
  accessible name → the prepared a11y text-name set (lands via L1 T3). Record `wgpu-collaboration-wasm32-react-en` FAIL on that step only.
- 23:28–23:35 **native two-shell gate `gate-p24-1` (RUST_MIN_STACK 8 MiB, catalog-exact native block2d): 12/12 PASS, EXIT 0, 390 s**
  (`s14-wg11-logs/gate-p24-1.txt`): sign-in ×2, same space, roster, create through the door (`accepted → ready`), both guests mounted + hub-bound,
  socket Live 43 ms, presence 165 ms, A authors 8.8 s + B ingests, B authors 10.8 s + A ingests, per-actor undo, connection loss (20 severed,
  offline edit, stale → ready, relive 10.5 s). Serves 6553/6555 stopped before it (WG11 has no serve/hub/browser running now).
- 23:3x–23:5x item 4 prepared: `wp-wg11/wg11-replay-routes-patch.py` (dry run `generated/s14c/replay-routes-dryrun.txt`; both patched Rust
  files parse under nightly rustfmt). Measured first: on wasm32 BOTH `ReplayShellCommand` arms were `cfg(not(wasm32))` → the browser wgpu shell
  dropped EVERY replay (directory, open, create) at `queue_host_effects`' debug line; native served only directory + open. Set: schema-first
  `🏛️ShellHost/📣️replay-refusal/🔣️.json` (reasons, en + de, code prefix, shared vectors; + `local-catalog-unavailable`), TS module derives
  from it (API unchanged), wgpu `ReplayRefusalReasonV1`/`REPLAY_REFUSAL_V1` (same JSON) + `refuse_replay` (warning notice + fault code),
  `os.create-space-artifact` → the ONE creation door (`space_artifact_creation_replay_choice` = ShellHost's gate: exact `{kindChoice,name}`,
  canonical choice == one offered kind of the index space's ready catalog → seal, submit, open ready), `os.local-catalog.*` → refusal until
  route B's native half (after SH2's B1 lands its Rust twins), unknown `os.directory.*` → invalid-request, other unknown `os.*` → unrouted.
  Laws: Rust ×3 (vocabulary + vectors; 7 unserved-replay cases en/de; creation choice gate 1 accept + 7 refusals), TS engine-contract ×2 (same
  vectors). Crates: `semio-framework-os-renderer-wgpu` (native + wasm32) + os TS (ShellHost, engine-contract vitest). Open gap (not in the set):
  non-`os.` replays (guest follow-ups, e.g. `setActiveExample`) — React queues them back into the guest; wgpu still ignores them.
- 00:0x item 5 prepared: `wp-wg11/wg11-offer-scope-patch.py` (dry run `generated/s14c/offer-scope-dryrun.txt`). The scope's inputs (session
  capability, hub, open space) live in the Rust shell, so: kernel directory client `live_agent_delegation_principals` + pure
  `live_agent_principals` (NEW `📇️directory/🔌️client/🤖️agent-delegations/🦀️.rs` — NEW dir → R10 taxonomy; Rust twin of the TS parser +
  ShellHost's live filter) with shared vectors NEW `📇️directory/🤖️delegations/🧫️fixtures/📋️agent-delegation-list.json` (5 cases; Rust client
  unit law + TS AgentDelegations law); renderer `AgentBridgeOfferScopeOwnerV1` (sign-out / space switch retire at once, stale listing dropped,
  re-listed every 30 s = discovery max so a revoke leaves it) + `pump_agent_bridge_offer_scope` on the directory pump + wasm export
  `dumpAgentBridgeOfferScope` (only `{hubOrigin, spaceId, agentPrincipalIds}` or `null` crosses; the capability never); TS
  `watchAgentBridgeOffer({offerScope})` re-read before every poll + strict `agentBridgeOfferScopeFromJsonV1`, transport probe
  `agent-bridge-scope`, frame worker answers it, both page entries pass it; laws: Rust owner state machine (hub-projection-workspace),
  TS watcher asks with each re-read scope + scope JSON strictness (AgentBridge). Crates: `semio-framework-os-kernel`,
  `semio-framework-os-renderer-wgpu` (native + wasm32); os TS (AgentBridge offer, transport, frame worker, both boots, 2 vitest files).
- 00:0x L1: replay-routes registered in T3 (after wg11-harness); laws on request after T3's proof.
- 00:0x item (2): all WG11 window-3 sets re-dry-run clean on the T1-written tree (`generated/s14c/*2-dryrun.txt`: reseed 11, board 3, a11y 3,
  harness 2, table painter 10+2, shell-turn 2/12). ORDER CONSTRAINT found + relayed to L1: shell-turn must land AFTER replay-routes and
  offer-scope (it re-boxes `handle_replay_shell_command`, whose text replay-routes anchors on). Proof: the whole WG11 T3 chain composed on a
  29-file scratch mirror in L1's corrected order reseed → board → a11y → replay-routes → offer-scope → shell-turn: every step applied, all 16
  resulting Rust files parse (`generated/s14c/compose-t3-chain.txt`).
- 00:1x housekeeping (rule 7): deleted my own oversized captures `wp-wg11/generated/s14b/wg10bin-graph.pickle` (209 MB) and
  `generated/stack-frames-wg10bin-all.txt` (7 MB) — predecessor's call-graph/frame caches whose results are recorded in this report. No WG11 hub, serve, browser or native run is alive. Next on request: laws for the T3 sets when L1 asks;
  the TableRow painter rides T2 with LB2's p3 (set `lb2-p3-wg11`).

## Session 14b

Successor agent (2026-09-28 12:0x, restart after the usage cut). Chain launched 12:02:46, GUEST FREEZE ON (window 3 closed).

| # | Item | Status |
|---|------|--------|
| 1 | Window-3 prepared patches dry-run on the live tree (overnight peer set ~1 870 files) | reseed **clean** (11 files); board-pointer **re-anchored** (renderer `handle_pointer_move` now takes `PointerInfo` → anchor `holder(pointer.id)`, law drives `shell_input_tests::mouse_pointer(1)`, `mouse_pointer` → `pub(super)`) then **clean** (3 files); a11y text-name **clean** (3 files); RUST_MIN_STACK root fix: in progress (static call-graph of the overflowing path) |
| 2 | WG9 lost-Ack kernel laws green on today's tree (native lane) | queued (hold 4) |
| 3 | Live on 7800 B3 | in progress |
| 4 | Row 3.8 cross-shell-TYPE pairings | wasm32 ↔ React 13/14 (s14 `wr-2`); wasm32 ↔ native running (`wn-1`) |
| 5 | Row 4.10 agent-reply pixel assertion | judgement fixed (see log); re-run pending |
| 6 | Permanent `hub-collaboration-acceptance` + RELAY spec to R10 | pending |

### RELAY spec (R10, window 3) — `hub-collaboration-acceptance`

- **Project** `@semio-tech/framework-renderer-wgpu` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`).
- **Target** `hub-collaboration-acceptance`: `nx:run-commands`, `cache: false`, cwd = that package dir, command
  `bun ./📜️script.ts hub-collaboration-acceptance`; one nx **configuration per journey** appending `--journey <j>` (table). The
  gate appends `--hub <url> --locale en|de` and the serve flags. Verb + code land by WG11 in window 3: verb in the package's
  `📜️script.ts` (stages the block2d release native runtime exactly like `hub-live-collaboration-check` for native journeys), code
  in NEW dir `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts` (taxonomy registration = R10).
- **Args**: `--journey <j>` `--hub <url>` `[--serve <wgpu release serve url>]` `[--react-serve <React s dev serve url>]`
  `[--locale en|de]` `[--space <id> --document <id>]` `[--native-binary <renderer test binary>]` `[--native-modules <dir>]`
  `[--tag <t>]` `[--out <dir>]`. Exit ≠ 0 unless every step passes; missing hub/serve/credential/native runtime → `blocked` record.
- **Env** (never argv, never logged): `SEMIO_TWO_HUMAN_USER1_EMAIL`, `SEMIO_TWO_HUMAN_USER1_PASSWORD`, `SEMIO_TWO_HUMAN_USER2_EMAIL`,
  `SEMIO_TWO_HUMAN_USER2_PASSWORD` (the two-human matrix's variables). The agent journey mints its own delegation with human 1.
- **Hub precondition**: credential sign-in for both humans, catalog with `note`, `block`, `puzzle` (B3 has them), agent delegations
  (agent-pixels). No `hubAdmin`, no `backends`.

| Configuration (`--journey`) | Requires | Browsers | Check (`-<locale>`) | Criteria | en title | de title | Wall (load 50–90) |
|---|---|---|---|---|---|---|---|
| `wasm32` | hub, serve = wgpu release `note` | 1 chromium, 2 contexts (+2 in-process TCP relays) | `wgpu-collaboration-wasm32` | 3.2, 3.7, 3.9, 3.10 | wgpu wasm32 ↔ wasm32 collaboration over the hub | wgpu-wasm32 ↔ wasm32-Zusammenarbeit über den Hub | 8 min |
| `wasm32-react` | hub, serve = wgpu release `block2d`, localServe = React `s` dev | 1 / 2 | `wgpu-collaboration-wasm32-react` | 3.8 | wgpu wasm32 ↔ React collaboration | Zusammenarbeit wgpu-wasm32 ↔ React | 12 min |
| `wasm32-native` | hub, serve = wgpu release `block2d`, native block2d runtime + renderer test build | 1 / 1 + native | `wgpu-collaboration-wasm32-native` | 3.8 | wgpu wasm32 ↔ native wgpu collaboration | Zusammenarbeit wgpu-wasm32 ↔ natives wgpu | 13 min + build |
| `native-react` | hub, localServe = React `s` dev, native block2d runtime + renderer test build | 1 / 1 + native | `wgpu-collaboration-native-react` | 3.5 | native wgpu ↔ React collaboration | Zusammenarbeit natives wgpu ↔ React | 15 min + build |
| `native-react-cursors` | as `native-react` | 1 / 1 + native | `wgpu-peer-cursors-native-react` | 3.6 | native wgpu ↔ React peer cursors | Fremdcursor natives wgpu ↔ React | 15 min + build |
| `cursors` | hub, serve = wgpu release `puzzle2d` | 1 / 2 | `wgpu-peer-cursors` | 3.6 | wgpu wasm32 peer cursors on one board | wgpu-wasm32-Fremdcursor auf einem Brett | 8 min |
| `agent-pixels` | hub, serve = wgpu release `note`, os-mcp binary (`requireMcpBinary`) | 1 / 2 sequential | `wgpu-agent-reply-pixels` | 3.12, 4.10 | wgpu renders an AI agent's edit (pixel check) | wgpu zeichnet die Bearbeitung eines KI-Agenten (Pixelprüfung) | 35 min |

### Agreement WG11 ↔ LB2 (13:3x) — wgpu TableRow painter, window 3, both halves land together

Existing `TableRowProps` contract, no schema change. (1) Plain rows (`table_window_row`: Home, space, bcf, wav) have NO children after
LB2's pass — the SDK's `row-action-<i>` child buttons go; editable rows (`editable_table_window_row_at`: csv, tsv, xlsx, bcf, wav editors
+ xlsx viewer) keep exactly ONE child per materialised cell, positional to the column slice: key `cell-<logicalColumn>` (Input + Commit
binding, or Text) or a paged text-draft Surface `table-draft-<rowKey>-<col>` / read-only Surface `table-value-<rowKey>-<col>` → wgpu
paints cell i from child i when present, else `cells[i]` (as React). (2) `placement: Row` → trailing actions column headed
`Table.actions_label`; `Menu` → the row's context menu. (3) Row primary activation = the row record's `Trigger::Activate`. (4) The SDK's
`table_row_action` always sets `label` (LB2's law asserts it); wgpu uses it as accessible name + tooltip. (5) Row windowing =
`Table.window` (`TreeWindow`, `row_extent`) exactly like trees. `RowAction` stays the one representation (`row_actions`).

### Design — wgpu TableRow painter (window 3, with LB2)

Measured today in the tree (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` ~1137–1170): `Table` → bare vertical Stack (no header, no
windowing); `TableRow` with children → bare horizontal Stack (cells = children; row actions only through the SDK's duplicate child
buttons); childless `TableRow` → ONE Button labelled `cells.join(" · ")` (no actions). The contract a11y projection names `Table` (grid)
and `TableRow` (row) but never projects `row_actions`, so the wgpu mirror cannot reach a row action by keyboard.
1. **Plain rows** (no children): `Table` → retained `UiNode::Tree` with ONE section (`window` = `Table.window`, header = the column labels
   + `actions_label`), each `TableRow` → a tree item with NEW `cells` (painted in the section's column grid, ellipsized; label = first
   cell for the row's name) + `actions` from `row_actions` (existing trailing-action paint + hit path; `Menu` placement → row menu) +
   `action` = the row's Activate; the `TableRow` records mount as the keyed identity rows the tree arm already uses.
2. **Editable rows** (one child per cell): the cell children stay real nodes; the row lays child i into column rect i (the tree row's
   single inline `control` generalises to per-cell controls) with the trailing actions reserved; Surface cells paint through the
   existing scene slot.
3. **A11y**: both contract projections (Rust + TS twin + shared fixture) project each `placement: Row` action as a `button` child of its
   row named by the action's label (keys `<rowKey>#row-action-<i>`; mirror activation dispatches the RowAction binding).
4. **Laws**: reconcile unit (plain + editable rows → cells/actions/window), paint pixel law (column text + trailing icons vs a reference
   draw list), hit law (icon → RowAction, row → Activate), projection fixture rows; live: Home in the wgpu shell shows cell text +
   actions (SH2's verify-home) after LB2's SDK change.

### Session 14b log

- 12:0x reconcile. Predecessor's only applied tree edit = item 1c echo pins (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`, 27 18:37) — still in the
  tree. Unlogged predecessor work found: agent-pixels run `ap-1` (27 20:15–20:50, 9/11, `s14-wg11-logs/harness-ap-1.txt`), native
  hold 2 (27 21:23: hub check RED on peer `os-kernel-db` `PlannedEntries`/`apply_one`; **kernel lost-Ack laws 13/13 rc=0 21:24**),
  hold 3 (renderer test build RED on peer `semio-framework-os-infinite` missing `semio_framework_ui_viewport`), overlay proof never
  held. No `[DEBUG]` lines of WG11 in the tree; no half-applied hunk (the a11y patch script was untouched since 27 19:45).
- 12:09 **own slip, repaired at once**: a `--revert` flag on `wg11-echo-hub-law-pins.py` (meant as a no-op probe; the script's
  `--revert` WRITES) restored the pre-pin lines of `🔬️bin-unit/🦀️.rs`; re-applied with `--apply` within ~2 min; the dry run now
  refuses again (pins present) — net change zero (`generated/s14b/echo-pins-*.txt`).
- 12:10 dry runs on the live tree (`generated/s14b/*-dryrun*.txt`): WG10 reseed clean (11 files); a11y text-name clean (3 files);
  board-pointer RED (anchor `holder(pointer_id)` gone: the overnight set changed `handle_pointer_move` to take a `ui_render::
  PointerInfo`) → patch re-anchored on `holder(pointer.id)`, its law now drives `shell_input_tests::mouse_pointer(1)` (+ that helper
  → `pub(super)`) → **clean, 3 files** (`board-pointer-dryrun-2.txt`). WG9's original script stays red (superseded).
- 12:12 native hold 4 + 5 queued (`wg11-native-45.sh`, capture `s14-wg11-logs/native-4-5.txt`). Hold 4 (12:23–12:36): hub check
  **RED on peer** `semio-s-artifact-vcs-vcs` (`XlsxSnapshot.workbook` E0609/E0560, overnight set); kernel lib-test **does not compile**
  on the peer's overnight store set (missing fixture `🏪️store/🧬️retained-clone/🧪️tests/…/🧫️fixtures/📦️copy/🔣️.json`, E0618
  `RetainedCloneGrant` called as a fn) → item 2 BLOCKED on today's tree (last green: 27 21:24, 13/13). Kernel is frozen → no fix.
- 12:15–12:28 **row 3.8 wasm32 ↔ native — first run ever: 11/11 PASS, native law EXIT 0** (`harness-wn-1.txt`,
  `generated/hub-collaboration/wn-1/`; 7800 B3, block2d serve 6553 on the catalog-exact root `s14-wg11-b3-block`, WG10's 14:12 renderer
  test binary, `RUST_MIN_STACK=8388608` as a measurement condition): B signs in + attaches (`Persisted`), browser opens the native
  user's document live, presence both ways (`User Two · User One`, 6 ms), native edit → wasm32 6→7 (3 ms after the handshake),
  wasm32 edit → native ingests (7→8), each undoes only their own (8→7→6), wasm32 reload converges (live 30.7 s) and the next native
  edit arrives. Record `wgpu-collaboration-wasm32-native-en` PASS.
- 12:3x item 5 judgement fix (harness): ap-1's "changes the frame" was judged against 0.2 % of the body (2 970 px) while an empty
  text block paints ~1 200 px; its "equals reference" PASS was false — the reference session (same human, first session still open)
  never attached (`Remote: getrennt`) and so equalled the BEFORE frame. Now: bounds `agentEditMinPixels` 256 / `agentSameMaxPixels` 64
  (or 4× / 2× measured noise); the live session closes before the cold reference session boots; new step "a fresh session decodes the
  same committed state"; equality requires it.
- 12:30–12:40 **row 4.10 agent-pixels (en) 12/12 PASS** (`harness-ap-2.txt`, `generated/hub-collaboration/ap-2/`; note serve 6552 on
  `s13-wg9-b3-note`): agent = its own delegated principal, roster `wgpu acceptance agent ap-2 (AI agent) · User One`, MCP `addBlock`
  SUCCEEDED, block decoded 2.0 s later without reload, hub head 0→1, **frame changes 1 173 px, live frame == cold reference render
  0 px** (noise 0). Record `wgpu-agent-reply-pixels-en` PASS. Finding: the Artifact panel row of a text block paints its label
  `Text` and preview `text` overlapping (`generated/s14b/ap1-row-zoom.png`) — a wgpu outline-row layout defect (not fixed; frozen).
- 12:3x R10 relays: RELAY spec written above + sent; harness made self-provisioning (omitted `--serve`/`--react-serve` → own serve
  on a free port via `ensureDevServe`, stops only its own) + `native-react-cursors` split into its own journey/check; tsc 0 errors
  (`generated/s14b/tsc-harness-3.txt`). Live self-start of a wgpu serve: pending window 3 (no plugin `dist` in the tree now).
- 12:2x item 3 static call graph (`wg11-stack-paths.py`, graph of WG10's 14:12 binary): step 8's stack = test body 250 KB + `author_edit`
  56 KB + `drive` 56 KB + **`ShellState::dispatch_action` poll 841 KB** + `apply_mutations` 75 KB + `apply_ops_inner` 152 KB +
  `refresh_ui` subtree 359 KB ≈ 1.79 MB + libtest ≈ over 2 MiB. `dispatch_action`'s future is only ~55 KB (its `drive` frame): the
  841 KB are debug temporaries of the ~40 awaited child futures across its arms (≈25 `note_shell_setting_command` sites). Hold 5
  (`-Zprint-type-sizes`) measures the child future sizes for the fix.
- 12:42–12:50 **cursors (puzzle2d, de) 5/7** (`harness-cu-1.txt`; serve 6554 on `s13-wg9-b3-puzzle`): sign-in ×2, attach ×2
  (`Gespeichert`), presence both ways PASS; **peer cursors RED both ways (pixel deltas 0/0/0 over a control pass)** = the known
  frozen-presence-pointer defect the prepared board-pointer patch fixes (window 3) → re-run after it lands.
- 12:53–13:11 native ↔ React `nr-1`: **INVALID precondition** — the React serve (6555, current tree via `ensureDevServe`) speaks
  channel 19 since the 12:55 alignment; B3 is 18 → React `Remote: detached` (coordinator 12:55 broadcast); native law steps 1–5 PASS.
  Stopped by SIGTERM: the harness aborted, killed its native child and closed its browser (cancellation path proven). React serve stopped.
  Live React pairings wait for 7800 on ALL.
- 13:12–13:18 **wasm32 journey `w-2` (de, fresh space + note, 2 severable relays): 23/24** (`harness-w-2.txt`): painted, sign-in,
  attach, presence both ways, committed content, A→B 4.0 s, B→A 4.0 s, frames name the document, short cut never freezes A (10/6 ms),
  A relinks 3.0 s after heal + B receives the offline edit, medium cut spoken `reconnecting` after 5.1 s (German kept-edits line),
  relink + line clears, long cut expires B after 62.4 s in German, never relinks by itself, late joiner = hub head 3. Red: the kept-edits
  judgement read the card ONCE ~1 s after the offline edit (`Remote: erneuter Versuch`, no code yet) → judgement now polls ≤ 8 s
  (`keptLineMs`) for `pending`/`reconnecting` (the medium cut shows the line takes ~5 s); tsc 0.
- 13:2x coordinator: new window-3 item **wgpu TableRow painter** (with LB2). Contract proposal sent (`RELAY LB2`, existing
  `TableRowProps`, no schema change; SDK child-button duplication dropped by LB2).
- 13:3x item 3 **prepared** `wp-wg11/wg11-shell-turn-patch.py` (window 3; dry run clean, `generated/s14b/shell-turn-dryrun-2.txt`,
  2 files): 12 heavy shell turns (`dispatch_action`, `dispatch_command`, `apply_os_command`, `apply_mutations`, `apply_shell_uri`,
  `handle_hub_workspace_action`, `handle_sync_action`, `handle_checkin_action`, `execute_staged_action`, `handle_replay_shell_command`,
  `observe_invocation_history`, `set_extension_enabled`) return `ShellTurn<'a, R>` — their body boxed at the DEFINITION
  (`shell_turn(async move {…})`; `Send` on native because `spawn_dispatch_reserved` requires it, not on wasm32); the 9 call-site
  `Box::pin(self.<turn>(…))` E0733 wrappers and their two stale comments go. Child slot sizes measured from the disassembly of
  `dispatch_action`'s poll (`generated/s14b/dispatch-action-poll.s`, sret slot gaps): `apply_os_command` ~55 KB ×3, `dispatch_command`
  ~42 KB ×3, `apply_mutations` ~37 KB, `handle_hub_workspace_action` ~34 KB, `apply_shell_uri` ~33 KB, `handle_sync_action` ~30 KB,
  `execute_staged_action` ~59 KB, `handle_replay_shell_command` ~18 KB, `observe_invocation_history` ~16 KB (Σ slots ≈ 833 KB of the
  841 KB frame). Law `a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread` (`⚙️settings-general-layout`): red before (two
  nested 841 KB frames), must pass after. Parse-checked (rustfmt on the patched copy: no parse error). Window-3 proof: native
  `--lib --tests` check + the law + frame table (`wg11-stack-frames.sh`) + live gate at the DEFAULT stack. Hold 5 (`-Zprint-type-sizes`)
  stays queued to confirm the set.
- 13:4x rule 23: no overlay build for WG11's sets (each < 20 files, renderer-only); `s14-wg11-overlay` has no build-dir (never held).
- 14:16–14:23 native hold 5: **renderer lib-test build of today's tree rc=0** (6 m 44 s, 450 warnings = type-checked;
  `generated/s14b/n5-build.txt`); durable binary `.🧬semio/🌐hub/s14-wg11-bin/renderer-tests-1423` (channel 19 — for live native runs on
  ALL). Today's `dispatch_action` poll frame = **844 KB** (prologue `0xce000 + 0x420`) — the overflow class is unchanged by the overnight
  set. The type-size capture came out empty (my awk kept the LAST `: n bytes` = the alignment) → filter fixed (`wg11-native-5.sh`, tested
  on a sample line); the sizes will be taken on the PATCHED build in window 3 (one hold: patch → `-Zprint-type-sizes` → frame table →
  law), no extra lane hold now.

## Session 14

| # | Item | Status |
|---|------|--------|
| 1a | WG10 item 6 `wp-wg10/patch-rebootstrap-reseed.py` (native/wgpu shell re-seeds from `active-checkpoint/pair` on an actor-bound rebuild) — window 3 | **dry run clean 18:40** (11 files, every anchor once; `wp-wg11/generated/reseed-dryrun.txt`); compile proof in an overlay: pending |
| 1b | WG9 `wp-wg9/s13-board-presence-pointer-patch.py` (renderer notes `presence_pointer` while a board claims the move) — window 3 | **dry run clean 18:40** (2 files; `wp-wg11/generated/board-pointer-dryrun.diff`); its law is a source-order text law → re-deriving with a behavioural Shell law (routing moved into `ShellState`) |
| 1c | WG9 hub bin-unit echo-suppression pins (hub test file only, open during the freeze) | **applied 18:37** (`wp-wg11/wg11-echo-hub-law-pins.py`, dry run clean); native check + law queued (`s14-wg11-logs/native-1.txt`) |
| 2 | WG9 lost-Ack kernel fix (`settle_committed_envelopes`, `rollback_envelope → Option`, 3 laws, parity `lost-ack-committed-op-settles`) green on the current tree | queued (same native hold) |
| 3 | RUST_MIN_STACK: root-fix the 2 MiB test-thread overflow of the live gate (debug), no env var | pending |
| 4 | Live on 7800 (B3 now, ALL later): wasm32 s13b suite (23), native `hub-live-collaboration-check`, native ↔ React cross-shell (8), puzzle2d peer cursors, a11y of the wgpu roster + link states; re-run after window 3 | pending |
| 5 | Permanent wgpu collaboration acceptance (`📜️script.ts` verb + nx target + generated launch row; with R10) | pending |
| 6 | (coordinator 18:3x, audit-s14-state row 3.8) cross-shell-TYPE pairings wasm32 ↔ React and wasm32 ↔ native on one hub document (attach, presence both ways, edits both ways, late joiner), live + in the permanent harness | pending |
| 7 | (coordinator 18:3x, audit-s14-state row 4.10) wgpu agent-reply PIXEL assertion: a semio-MCP agent's edit is rendered correctly (readback/screenshot diff vs a reference render), in the harness | pending |

### Session 14 log

- 18:2x start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet; W4 unblocking the chain),
  `📓️wp-wg9.md`, `📓️wp-wg10.md`, `📓️fleet-13-agents.md` from 14:00. 0 wasm32 lanes held; native lane: lb2 holding, w4 queued.
- 18:3x reconcile of the predecessors' tree edits:
  - WG9 lost-Ack fix (applied 15:18): kernel laws **13/13 rc=0 at 16:21** + wasm32 browser/wasip2 rc=0 15:31/15:32
    (`.🧬semio/🌐hub/s13-wg9-logs/settlement-checks-1.txt`) — the landing row still said "checks pending".
  - WG9 sign-in patch (applied 14:56): renderer native `--lib --tests` check **rc=0 16:27**, renderer laws **23/23 rc=0 16:29**
    (`s13-wg9-logs/signin-native-2.txt`) — landing row said "native check pending".
  - WG9 hub bootstrap `HUB_CATCH_UP_ORIGIN` ×2 in the tree (bootstrap lines 5081/5194/5382); its bin-unit pins were reverted 06:32.
  - Unreported by the predecessors: WG9 run `s13e` (16:01–16:06, fresh note on serve 6554, A en / B de): 11/15 — attach ×2,
    presence, frames ×2 green; 5–8 red `no-edit-control` (probe could not find the Add Text control) —
    `s13-wg9-logs/collab-s13e.txt`. WG10 cross-shell de run `b3e2de` (15:38–16:21): native rc=101, React side timed out 180 s
    waiting for `s-space-create-artifact` (React never opened the doc) → steps 3–8 red — `s13-wg10-captures/cross-shell-b3e2de/`.
- 18:37 item 1c applied: `wp-wg11/wg11-echo-hub-law-pins.py` (imports WG9's `LAW_EDITS`; dry run clean `generated/echo-pins-dryrun.diff`):
  reconnect + joiner catch-up asserts pin `origin == HUB_CATCH_UP_ORIGIN` (≠ the joiner's actor), fixture `hubCatchUpOrigin`
  equality, both bootstrap call sites counted. Native hold 1 queued (`wp-wg11/wg11-native-1.sh`, pid 44569): hub check → hub law →
  kernel settlement laws.
- 18:4x native hold 1 (18:48–18:52, `s14-wg11-logs/native-1.txt`): **hub check + hub law + kernel laws all RED on peers' in-flight
  guest-linked edits, not on item 1c** — `semio-framework-3d` `🥽️mesh` E0282, `semio-framework-ui-contract` `🛡️limits`/`📐️layout`
  E0308/E0277 (`&bool`, `&&[str]` — value derive/codec change), `semio-framework-actor`, os-kernel 55 errors: the Codex peer's
  18:35–18:44 value-derive/codec set (coordinator 18:5x). Re-queue when the kernel compiles again.
- 18:4x item 1b re-derived: `wp-wg11/wg11-board-presence-pointer-patch.py` (dry run clean, `generated/board-pointer-v2-dryrun.diff`,
  3 files) = WG9's one renderer line + a BEHAVIOURAL law instead of WG9's source-order text law: every `👕️canvas-presence`
  `publish` case with a pointer is driven through `AppInteractionState::handle_pointer_move` (the path winit and the browser worker
  drive) over a painted board (`painted_board_with_input`, `shell_input_tests::pointer_interaction` → `pub(super)`); the heartbeat's
  view must equal the fixture's expected view and ≥ 1 case must be claimed by the board surface (the frozen path). Supersedes
  WG9's script for window 3.
- 18:5x inherited React serve 6590 (WG10's, pid 24033 + vite 24162, AV2's range now) stopped. React `s` serve **6555** started through
  S18's `ensureDevServe` (`wp-wg11/wg11-react-serve.ts`, joined to 7800, log `s14-wg11-logs/react-serve-6555.log`); boot > 240 s at load 111.
- 18:55–19:01 **wasm32 suite `s14a` (7800 B3, fresh note `artifact-077a7975756477104325d9df8793bea7` in WG9's space
  `01a0e2bd-…`, serve 6552 = WG9's note serve (renderer wasm-release 13:49), A en-US / B de-DE, outage on): 21/23**
  (`s14-wg11-logs/collab-s14a.txt`, captures `wp-wg11/generated/collab-s14a-*`; harness `wp-wg11/wg11-browser-collab.mjs` = WG9's).
  PASS: painted ×2, sign-in ×2 (B's chrome German), attach ×2 (`Persisted`/`Gespeichert`), roster `User One · User Two`, 4b late
  content ×2, A→B edit (+4.2 s), B→A edit (+14.3 s, within the probe's poll), frames name the document ×2, 15 s cut never freezes
  A (worker answers 5/11 ms), offline edit admitted `Pending (1)`, **12: after the 15 s cut A relinks in 3.0 s and B receives the
  offline edit** (red in s13b = the lost-Ack class), 13 B's long cut expires (87 s) in German, 13d German line.
  FAIL 12b: the 20 s cut was never spoken as `reconnecting` — Playwright's `setOffline` does not close an open WebSocket, so the link
  never dropped (sync stayed `Persisted`) → harness limitation, not product: the permanent harness cuts through a severable TCP relay.
  FAIL 14: card empty when read (`Remote: getrennt` = detached, never relinked — the product fact holds; the read raced the card
  toggle) → permanent harness re-opens and re-reads the card.
- 19:02–19:08 **native gate run `gate-b3-1` (7800 B3, WG10's durable 14:12 renderer test binary, DEFAULT test-thread stack):
  steps 1–7 PASS, then `has overflowed its stack` → SIGABRT (exit 134) in step 8 (A's first `addHandleKind`)** —
  `s14-wg11-logs/gate-b3-1.txt`. Item 3 reproduced on B3 (same step as WG10's 14:13 run 1).
- 19:10–19:20 **native gate `gate-b3-2` (same binary, `RUST_MIN_STACK=8388608` as a measurement condition only): 12/12 PASS,
  EXIT 0, 553 s** (`s14-wg11-logs/gate-b3-2.txt`): door `accepted → ready`, both guests mounted, both bound to the hub actor,
  socket Live 49 ms, presence both 145 ms, A authors 8.3 s + B ingests, B authors 14.2 s + A ingests, per-actor undo, connection
  loss (20 connections severed, offline edit, stale → ready, relive 26.7 s). Latencies are load-bound (load avg 80–111).
- 19:1x catalog-exact block module root `s14-wg11-b3-block` (served wasmSha256 `b7c334ee…` == catalog B3; shard worker fresh)
  (`wp-wg11/wg11-b3-block.sh`, `s14-wg11-logs/b3-block.txt`); wasm32 block2d serve **6553** (pid 40822, WG9's serve recipe with
  variant `block2d`). Seeded block2d `artifact-342b6cb6d06c5138ccaea5fb8b7dc9df` in WG9's space.
- 19:0x–19:2x **permanent harness written (staged in the ticket, lands in window 3)**:
  `wp-wg11/harness/🤝️hub-collaboration/🟦️.ts` (target `🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts`, NEW dir → R10
  told via RELAY 19:0x): journeys `wasm32`, `wasm32-react`, `wasm32-native`, `native-react` (+ `--mode cursors`), `cursors`,
  `agent-pixels`; severable TCP relay per wasm32 human (real cuts: `setOffline` never closes an open WebSocket); native peer =
  the renderer's live laws (`--native-binary` or `cargo test`) following its file handshake with a wasm32 OR React follower;
  records via `withAcceptanceRecord` + `publishAcceptanceCheckResult` (`<check>-<locale>`), `blocked` for missing hub/serve/
  credentials; humans only from `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}`. Scratch tsc (`wp-wg11/tsc/tsconfig-wg11.json`,
  extends the os tsconfig): 1 error in my file (duplicate key) → fixed; re-run pending. Ticket entry `wp-wg11/harness/wg11-verb.ts`,
  runner `wp-wg11/wg11-harness-run.sh`.
- 19:23–19:35 harness run `wr-1` (`--journey wasm32-react`, block2d `artifact-342b…`, wasm32 6553 / React 6555): 8/13 — sign-ins,
  attach, React open (Live 11.0 s), presence both ways, React undo, late React joiner, late wasm32 attach PASS; wasm32 edit/count
  red: the wasm32 board's counts paragraph carries NO accessible name and the actions live in the window's folded Actions pane.
- 19:3x exploration (`wp-wg11/wg11-explore-wasm32.ts`, `generated/explore-block-*`): `dumpStructure` paints `6 Handle Kinds, 11
  Handles`; the projection announces `block2d-play-board.summary`/`.counts` as unnamed `paragraph`s (**a11y defect, item 4e**);
  unfolding `framework.window.block2dBoard.engagement.toggle` exposes 16 named action treeitems incl. `action.addHandleKind`; the
  History panel exposes `framework.history.undo.run`. Harness: document count = painted text (`dumpStructure`, what a sighted user
  reads) + a SEPARATE a11y step (counts paragraph named = painted text); edits via the Actions pane; undo via History.
- 19:3x **a11y root fix prepared** `wp-wg11/wg11-text-accessible-name-patch.py` (dry run clean, `generated/text-name-dryrun.diff`,
  3 files): the shared projection (`accessibility_projection_node` + TS twin `uiAccessibilityProjectionNodeV1`) names a `Text`
  node by its `value` unless an explicit label wins; shared fixture row `#caption` (Rust contract law, wgpu target projection law,
  TS runner all answer it). Guest-linked (ui-contract) → window 3.
- 19:42–19:47 **harness run `wr-2`: wasm32 ↔ React (block2d, 7800 B3, en): 13/14 PASS — first cross-shell-TYPE pairing ever run**
  (`s14-wg11-logs/harness-wr-2.txt`, `wp-wg11/generated/hub-collaboration/wr-2/report.json`): presence both ways (`User One · User
  Two` / `UO UT`), wasm32 edit → React 6→7 (seen 5 ms after the edit settled), React edit → wasm32 7→8 in 2.0 s, wasm32 undoes
  its own (8→7, React 7), React undoes its own (7→6, wasm32 6), late React joiner converges (6), late wasm32 joiner converges (6).
  Only red: the a11y step (counts paragraph unnamed) → the prepared patch. Acceptance record published en + de (FAIL on that step).
- 19:4x scratch overlay `.🧬semio/🌐hub/s14-wg11-overlay` (APFS clones of 77 832 tracked + untracked files, `wp-wg11/wg11-overlay-sync.py`)
  for compile/law proofs of the window-3 patches (overlay lane, private build-dir).
- 19:56–20:03 **harness run `w-1` (`--journey wasm32`, 7800 B3, fresh space + note created by the harness, both humans through
  their own SEVERABLE RELAY, en): 22/24** (`s14-wg11-logs/harness-w-1.txt`, `generated/hub-collaboration/w-1/report.json`):
  painted ×2, sign-in ×2 (through relays), attach ×2 (`Persisted`), roster `User Two · User One` both ways, committed content ×2
  (head 0), A→B 4.1 s, B→A 4.1 s, frames name the document ×2, **a REAL 15 s cut (6 connections severed) never freezes A (worker
  answers 16/9 ms)**, A relinks 3.1 s after heal and B receives the offline edit, **medium cut spoken `reconnecting` after 2.1 s**
  (`Connection lost. Your edits are kept…`), relink + line clears (13.3 s, backoff), **long cut expires B after 63.5 s** in English,
  late joiner shows 3 = hub head 3. Red (judgement, not product): the offline edit's pill read `Remote: backoff` (the kept-edits
  line is the card's `reconnecting` text) and after heal + 15 s the expired card shows `Remote: detached` with the line gone —
  judgements now accept the card's kept-edits line / a detached state (the product facts are recorded as detail).
- 20:0x item 3: macOS `lldb` and `sample` both need developer-tools authorization (both hung on attach; my pids killed at once) →
  static measurement instead. `wp-wg11/wg11-stack-frames.sh` (objdump prologues incl. Rust's inline stack-probe `sub x9, sp, #…`)
  over WG10's debug renderer test binary: **largest single frames 4.9 MB `RuntimeApply::start_frame_deferred`, 4.2 MB
  `RuntimeMailbox::restore_presenter_interaction_step`, 2.69 MB `ShellState::handle_shell_hit` poll, 1.58 MB
  `handle_pointer_button_for` poll, 1.42 MB `spawn_dispatch_reserved`, 1.18 MB `handle_keyboard_async`, 1.0 MB
  `handle_accessibility_event`, 938 KB `Ui::frame_step`, 840 KB `ShellState::dispatch_action` poll**
  (`generated/stack-frames-wg10bin-2.txt`). By-value type sizes read off `Option<T>::take` frames: `AppInteractionState` ≈ 68 KB,
  `FrameDeferredExecutionOwner` ≈ 82 KB, `RetainedPaintFrame` ≈ 77 KB, `UiWindow` ≈ 80 KB. Step 8 = `drive(dispatch_action(…))` on
  libtest's 2 MiB thread: the future itself is < 133 KB (its `drive` frame is not in the top 400) — the POLL frames overflow (debug
  gives every awaited sub-future temporary and every by-value move its own slot). Full table running (`stack-frames-wg10bin-all.txt`).
- 20:1x lanes: native hold 2 (hub + kernel laws) + hold 3 (current-tree renderer test binary) queued (`native-2-3.txt`, 9 ahead);
  overlay proof of the three window-3 patches queued (`overlay-proof-1.txt`, 12 ahead; `wg11-overlay-proof.sh`,
  `wg11-overlay-apply.py` stages each patch with its repo paths rewritten to the overlay — tested: all three dry-run + apply clean in
  the overlay, tree files verified untouched). Swap 32.8/33.8 GB, load ~90, 18 rustc.
