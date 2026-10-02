# 📓️ W3-T2-STROKES Report: wfc Bitmap Stroke, Raster Stroke, Remodel Import, Process3d Cursor and World Gestures

Executor W3-T2-STROKES. Scope: audit WP-5 (`📓️audit-remaining-tools.md` §7 row 5, §5.1 wfc/raster, §5.4, §5.5),
minus the wfc 2d/3d node-graph moves (they wait for the flow executor's node-graph record).

Status: IN PROGRESS (census written first, per the W3-T brief step 1).

## 1. Census (before)

| # | Plugin | Gesture / path | Entry verbs | How it commits today | Verdict |
|---|---|---|---|---|---|
| S1 | wfc bitmap | paint stroke on the input sample | `stroke-begin{x,y}`, `stroke-extend{x,y}`, `stroke-commit` | ticks grow a bounding box in the pane's **window config** (`BitmapInputWindowConfig.stroke`, coalesce key `wfc-bitmap-stroke`, one config edit per gesture); commit fills the WHOLE box with the armed colour as ONE absolute `set-input-pixels{x,y,w,h,base64}` + a config write | G: tool machine, stroke scratch in the window transient, parametric stroke leaf |
| S1h | wfc bitmap | host binding | none | no host sends the stroke verbs (React `Canvas2dHost` and wgpu `Scenes` dispatch only `canvasPointerDown/Move/Up` in SCREEN pixels, without the camera); the stroke is reachable from MCP / palette / tests only | host gap, see §6 |
| S2 | raster | brush / eraser stroke on a pixel layer | `editPixels{layerId, expectedImageKey, operation:{kind:"stroke",points≤2048,size,opacity,hardness,color,erase}, selection}` | React `Paint2dHost` keeps the points host-locally and dispatches ONCE at release; the retained work rasterizes, PNG-encodes and publishes `change-layer-pixels`(detach) / `remove-layer-asset` / `add-layer-asset{png}` / `change-layer-pixels`(attach): history edits opaque PNG blobs, the stroke intent is thrown away | G: parametric stroke leaf + one-shot stroke tool |
| S2m | raster | brush / eraser stroke on a layer mask | `editMask{..., operation:{kind:"alphaStroke",...}}` | same shape on the mask asset | G: same leaf, `target: mask` |
| S3 | remodel | host-decoded video import | `importVideoFramePayload` ×N + `importVideoDone` | one `Emit::amend` per decoded frame (`create-asset` + `create-stream` / `add-stream-frame`) under key `remodeling-import:{stream}`, closed by `importVideoDone`; a cancelled import leaves half the frames in the document; per-frame ops are announced live | I: import tool machine, ONE transaction per import |
| S3s | remodel | image-sequence import (file picker, `multiple`) | `importFramePayload{index,total}` ×N | same per-file amend under the same key; no done verb (the last file is `index == total-1`) | I: same machine, commit at the last file |
| S3b | remodel | in-process video bytes fallback | `importVideoBytesPayload` | already ONE `Emit` | O: stamp it as one transaction too |
| S4 | process3d | replay cursor | `setCursor`, `stepCursor`, `stepCursorBack`, `stepCursorForward`, engagement `back`/`forward`/`all` | ONE document edit per click (`change-cursor` leaf on `Process3dSnapshot.resolved_up_to`); `create-step`/`delete-step` builders also move it | V: view state, out of the document |
| S5 | process3d | push/pull face drag (`select` utility) | `worldFaceDragEnd{normal,startPoint,distance,faceExtent}` | host-local drag, ONE dispatch at release, `insert_step_mutations` (+ cursor) as a plain edit | O → one-shot tool transaction (`TransactionRef`) |
| S6 | process3d | click-to-cut / drill / attach | `worldPointerDown{position}` | one plain edit | O → same one-shot tool transaction |

## Session 2 — 2026-10-01

Successor `S2-STROKES` (coordinator `⚪552b484a…`). Scratch: `🗑️generated/s2-strokes/`. Status: IN PROGRESS (this section is
updated at every milestone; the newest state is at its end).

### S2.1 Repair (rule 21) — what the predecessor left

The predecessor's work up to 07:54 was auto-committed in `4e36b2b5012` (11:16). Inventory from that commit + the disk:

| Area | State found | Repair |
|---|---|---|
| wfc bitmap brush tool (`🖼️input/🪛️utilities/🖌️brush`), `paint-input-stroke` leaf, transient | lib compiled at 07:54; test target failed `E0609 no field id on &UtilityRef` | fixed (`utility.as_str()`), `✏️editor/🧪️tests/🔬️unit/🦀️.rs:115` |
| wfc 2d / 3d `drag-slots` + `set-slot-positions` leaves, `🛠️tools/✋️drag` over `node_drag_commit` | written 07:31–07:48, never compiled | compile pending (see S2.6) |
| process3d cursor → window config (`amend_config`, view state), world tool (`ToolMachineRunner`, one transaction per gesture) | never compiled | compile pending (see S2.6) |
| raster | untouched | converted in S2.3 |
| remodel | untouched (needs §15) | S2.5 |

### S2.2 wfc bitmap — host binding (census row S1h closed)

No host could reach the brush (React `Canvas2dHost` and wgpu `Scenes` only dispatch `canvasPointerDown/Move/Up`, and the
bitmap panes did not even declare them: every hover was refused `undeclared-action`). Now:

- Both canvas hosts stamp WORLD coordinates on every pointer command: React `createCanvasPointerGestureLane` captures
  `[x, y, modifiers, worldX, worldY]` at sample time (camera at that moment) and sends `worldX/worldY` on down/move/up/cancel and
  `worldSamples` beside `samples` on moves; wgpu `Scenes` adds `worldSamples` (it already sent `worldX/worldY`), test reference
  wire mirrored. Additive keys, every other guest unaffected.
- Bitmap editor: `CanvasPointerDown/Move/Up`, `CanvasDoubleClick`, `SyncCamera` commands (+ tool ids, contracts, proofs, both
  panes declare the five canvas verbs). `BitmapEditor::pointer_stroke` maps them onto the brush: primary press opens a stroke
  (dropping one a lost release left open), moves stream cells (repeats collapsed, clamped onto the sample edge), release commits,
  cancelled release aborts `captureLost`, hover / stray release / secondary press = nothing. `extended()` collapses repeated
  cells across ticks.
- Tests added (`🧪️brush-tool`): pointer mapping laws (2), cross-tick repeat collapse, mounted press→drag→release = ONE row with
  `TransactionRef` (+ hover and cancelled release = zero trace). React lane test: world stamping under the sample-time camera.

### S2.3 raster — parametric `paint-stroke` (design §17.2)

- ONE deterministic Rust rasterizer: `semio_framework_pixels::editing::{stroke_bounds, paint_stroke_in_place}` — walks only
  the stroke's reach (union of segment boxes, clipped), optional per-pixel selection coverage; byte-identical to the whole-image
  `PixelEditJob` (law over every brush case of the shared corpus incl. selected ones + a generated 23×17 image under a graded
  selection). TS twin `strokeBounds` + shared corpus section `strokeBounds` (4 cases) checked by Rust and TS.
- Leaf `🧬️schema/🧬️mutations/🖌️paint-stroke` `{layerId, target: pixels|mask, tool: brush|eraser, brush{size, hardness,
  opacity, color[4 unit]}, points[{x,y}] 1..2048, selection: runs|null}`; full `x-semio-ui` en/de, hard bounds, declared
  invariant `ordered-selection-runs`. Diff rasterizes the base image (decoded child) or a blank layer/mask, files the result as a
  content-addressed asset (`pixels-<layer>-<sha16>` / `mask-<layer>-…`) in the NEW lossless carrier
  `application/x-semio-image` (`SemioImageSnapshot` pack: no PNG encode on any replay), patches the layer/mask, releases the
  replaced image when nothing else shows it. Inverse: re-add the released image (exact child), point back, remove the minted one.
  Codes: Fatal `mutation.invariant` (field-named), Error `target-missing` / `target-mismatch` (locked, no pixels, no mask,
  selection past the image), Warning `no-op`, Fatal `mutation.apply.capacity|image-unmaterialized|image-invalid|rasterize`.
  Label en/de per target × tool. Surfaces: aggregate variant + KINDS, schema `oneOf`, text DSL + grammar, binary tag 17,
  proto, graphql, TS twin `parsePaintStroke()`, oracle catalog (kind, 6 scenarios, manifest row), binary digest / retirement /
  retained candidate (`RasterOneItemApply` applies the leaf's diff), `🚪️io` accepts the pack mime.
- Fixture quintets are a PRINT of the leaf (`emit_committed_fixtures`, ignored test, like bitmap's): 🖌️paints, 🎭️masks,
  ⚠️misses, 🔒️locked, 🚫️rejects, ✂️clips — generation pending the first green compile.
- Tool: `✏️editor/🎮️commands/🖌️paint-stroke` — `paintStroke{layerId, tool, xs, ys}`; a `🔄️machine` statechart through
  `ToolMachineRunner`, ONE `Emit::commit_transaction` per release (`s.raster.raster@1/*#editor#paintStroke`), the brush, target,
  mask value and pixel selection come from the session config (single source of truth for both hosts).
- Legacy deleted: `stroke`/`alphaStroke` are gone from `editPixels`/`editMask` (`editMask` = mask fill only; its corpus
  converted, the duplicate stroke case dropped), React `Paint2dHost` and wgpu `🗺️surface/🎨️paint` + `EngineCanvas` dispatch
  `paintStroke` (no PNG, no revision); host tests updated (Paint2dHost selection-focus, wgpu paint unit, EngineCanvas paint2d).
- Open (honest): the stroke is ONE dispatch at release with a host-local preview in both hosts (as before); a streamed
  stroke previewed from a window transient (§17.2 last sentence) is NOT built — it needs a composite-window transient partition
  and a streaming tool mirroring the bitmap brush.

### S2.4 Edits outside my owner list (region-scoped, compile-atomic)

`📐️Canvas2dHost/🟦️.tsx` (+ its lane test), `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (+ standalone test reference),
`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` paint2d dispatch region (+ its paint2d test) — FLOWCAD owns that file; one region.
`🔲️pixels/✍️editing` (rasterizer, TS twin, corpus).

### S2.5 remodel import on design §15 (streamed tool transaction)

- One import = ONE streamed transaction `import_transaction(stream_id)` (`TransactionRef::mint` over the stream the import builds,
  tool `s.remodel.remodeling@1/*#editor#import`): no tool state between ticks, every tick / commit / abort names the same ref.
  - `importFramePayload{payload, name, index, total}` (`total` new; the hosts already send `{index, total}` on a multiple pick,
    a single pick defaults to `total = index + 1`): files `0..total-1` stream, the last commits; an undecodable last file commits
    empty.
  - `importVideoFramePayload` ticks stream (the document shows each frame at once); `importVideoDone` commits with the
    provenance write; `importVideoBytesPayload` (in-process decode) commits once.
  - NEW `importAbort {}` (`🎮️commands/🛑️import-abort`, in the command palette so a user can free a document an interrupted
    import left open, en/de label + description): `Emit::abort_transaction` of
    the last stream's import, zero trace; with nothing open it is an empty emission. Wired: crate root, `app_commands!`
    (appended row = new last ordinal), tool ids, artifact route, proofs, bridge, manifest, interactive-job class, describe,
    retained-command fixture (`routes 36`, the stale `expected: 40` corrected), unit-test counts 36.
- No `Emit::amend` / `coalesce_key` / `remodeling-import:` key is left in the remodel plugin (grep: 0).
- Laws added (`📼️import-video-frame-payload/🧪️tests/🔬️unit`): one ref per stream; a 3-file still import = one edit (one undo
  removes stream + 3 assets, one redo restores); a video import shows streamed frames at once and commits one edit; an open
  import refuses another document verb with `toolTransaction.open`, then commits; abort = the document before the import, no
  edit left to undo, a second abort with nothing open is no edit.
- Open (honest): (1) the hosts do not yet dispatch `importAbort` when the user cancels a pick/import (`ShellHelpers`
  `dispatchOpenedFiles` abort signal + the `RequestMediaFrames` driver; S2-W2B / S2-W2C own those files); until then a
  cancelled import stays open and refuses other verbs (`toolTransaction.open`).
- Fixed on the way: `next_remodeling_id` was a process counter (`stream-1` again after every reload, colliding with a persisted
  `stream-1`, and with it the derived import transaction). Replaced by `schema::mint_remodeling_id(operation, prefix)` =
  `store::content_id(prefix, authoring_seed U+001F prefix)` at every call site (add-stream, add-gcp, the three imports); law
  `a_minted_id_is_content_addressed_over_the_admission_seed` replaces the counter law.

### S2.6 Lints and host tests (after the 21:30 resume)

- `bun ./📜️script.ts schema mutation-inputs --under <scope>` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`):
  remodel 56/56 inputs, 0 findings; raster / wfc / process: every finding is `leafUncatalogued` (paint-stroke, wfc 2d+3d
  drag-slots + set-slot-positions, bitmap paint-input-stroke) or the stale catalogue row of process3d's deleted
  `⏱️change-cursor` leaf — both clear with the central `schema generate` (coordinator). The same reader run directly on the six
  uncatalogued leaves (`🗑️generated/s2-strokes/audit-leaves.ts`, `mutationInputAudit`) found 2 real findings (paint-stroke
  `points[].x|y` used the undeclared widget `number`) — fixed to `stepper`; now 0 findings on all six.
- `bun ./📜️script.ts schema mutation-payloads --under <scope>`: raster 23/23 payloads, 18/18 leaves witnessed (after the
  emitted quintets); wfc 85/85, 82/82; process 15/15, 15/15; remodel 136/136, 36/36 — 0 findings each.
- Raster quintets emitted: `cargo test … -p semio-s-artifact-raster-raster --lib -- --ignored
  paint_stroke::component::tests::emit_committed_fixtures` → 1 passed (6 cases × 5 files under
  `🧫️fixtures/🧬️mutations/🖌️paint-stroke/`).
- React hosts (cwd `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`):
  `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 📐️Canvas2dHost 🖌️Paint2dHost` → 10 files,
  148/149 then the one failure (`input-contract`: the down/cancel actions now carry `worldX/worldY`, exactly what wgpu
  already sent) fixed by adding `world` to the shared fixture `📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json` and
  asserting it → input-contract 28/28.

### S2.7 Cargo runs 21:40–23:00 (one at a time, gated) and what they found

- `cargo check --manifest-path ✏️s/Cargo.toml --tests -p semio-s-artifact-remodel-remodeling`: 1 error (mine, `expect_err` on a
  non-`Debug` receipt) → fixed (`.err().expect(..)`); 0 warnings in my files. The follow-up remodel `cargo test` was blocked by a
  peer break in `semio-s-artifact-stdio-obj` (`Vec<usize>` vs `Vec<u64>`, value-crate migration) — NOT RUN yet.
- `cargo test … -p semio-s-artifact-raster-raster --lib` (private target, `CARGO_INCREMENTAL=0`): run 1 = 329 passed / 14 failed;
  4 were mine (test row counts not grown for `paintStroke` / `PaintStroke`: `command_wire_keywords…` 26→27, lane table, kinds
  list, binary owner-caps list) → fixed; run 2 = **333 passed / 10 failed**. The 10 left trace to ONE cause outside my change:
  a peer's dsl grammar change (`🗣️dsl` "canonical braces" for record-list items, `🧾️record-list` fixtures, old form now
  invalid) made the committed raster demo carrier unparseable (`[DEBUG]` probe: `expected LBrace, found Ident 'key'` at col
  650 = `assets=[ key=… ]`), so the boot document fell back to the empty scaffold (3 boot tests), the demo archive replay
  never terminated and leaked initializer process controls (1 + 4 poisoned binary tests), and two mounted fixtures did not reach
  terminal-empty (2). Fixed at the source: the carrier now reads `assets=[ { key=… } ]` (the probe line is removed). Re-run
  pending the cargo hold.
- Same grammar change also invalidates process3d's committed carriers (`🎬️demo`, `🌲️concrete-forest`, and the plate const in
  `📸️snapshot/📝️text/🦀️.rs`): they are a print of `regenerate_example_fixtures` (ignored test, `PROCESS3D_FIXTURE_OUT`), to be
  re-run after the hold; never hand-transcribed.

### S2.8 Source work under the 02:45 cargo hold

- `importAbort` is now a palette action (default kind icon), so a user can always free a document an interrupted import left
  open (`toolTransaction.open`), until the hosts dispatch it themselves.
- React + wgpu identity for world stamping made a law on BOTH hosts: the shared fixture's new `pointer.down.world` /
  `pointer.cancel.world` (-30, -30) is asserted by the React input-contract test (28/28 green) and now by the wgpu
  `published_canvas_pointer_capture_preserves_modifiers_and_cancels_exactly_once` law (`🐚️Shell/🧪️tests/🔬️wgpu-shell-input`,
  4 assertions; same formula `(s − centre) / zoom + camera` read off both hosts' code) — not run yet (cargo hold).
- Static review of the predecessor's never-compiled wfc 2d/3d `🛠️tools/✋️drag`: APIs it calls exist with those signatures
  (`node_drag_commit`, `NodeDragRecord::moves`, `default_now_ms`); the editor is behind `component-app-assembly`, so its
  checks need `--features component-app-assembly`.

Queue once the hold lifts (one at a time, gated, private target for tests):
1. `cargo test -p semio-s-artifact-raster-raster --lib` (expect the 10 carrier-caused failures gone).
2. `cargo test -p semio-s-artifact-remodel-remodeling --lib -- import mint retained_command_catalog every_command command_ids
   command_from_action exhaustiveness` (needs the peer `stdio-obj` break fixed).
3. `cargo check --tests --features component-app-assembly -p semio-s-artifact-wfc-2d`, `… -wfc-3d`, `… -wfc-bitmap`; then
   their tests.
4. `cargo check --tests -p semio-s-artifact-process-process3d`, then `PROCESS3D_FIXTURE_OUT=… cargo test … --
   --ignored regenerate_example_fixtures` and copy the three prints over the carriers; then its tests.
5. `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-{raster,remodel,wfc,process}`.
6. wgpu: `semio-framework-pixels` editing + surface paint + renderer-wgpu input law (root workspace).

### S2.9 After the hold (03:08)

- 03:08 raster `cargo test` blocked before my crates: `semio-framework-os-kernel` red from a peer's in-progress dsl refactor
  (`🗣️dsl/🪟️viewport` `super::schema` missing, `store` `RecordSpecProducer` called as a function) — retry pending.

## Session 3 — 2026-10-02

Successor `S3-STROKES` (coordinator `⚪b7db773a…`). Scratch: `🗑️generated/s3-strokes/`. Status: IN PROGRESS (updated at every
milestone; newest state at the end). Focus (rule 29): verify → fix → close; then remodel/§15 remaining items and the rest of
the stroke/wfc gestures.

### S3.0 Start (10:58)

- Load 122, 21 rustc (Codex peers) — cargo waits at the rule-30 gate (< 14 rustc).
- Session-2 tail (03:38 `test-raster-6.txt`): raster `cargo test` was blocked by peer `RecordSpecProducer` call sites in
  `semio-s-artifact-stdio-binary` (`🔖️raw/…/🧬️mutations/🦀️.rs:126,140` E0618) and `semio-s-artifact-stdio-obj` (8 errors).
- Files in my trees touched after the S2 report (peer churn: Cargo.toml, `👁️viewer`/`✏️editor`/`🎚️config`/`👥️presence`, text
  codecs, wfc bitmap sqlite snapshot) — reviewed in S3.1.

### S3.1 Remodel import: tool state in the importing window (§15 API rules, `📓️api-transaction-amend.md` §4) — 11:45

S2.5's stateless import (transaction minted from `streams.last()`) had three real faults: (1) a tick or done arriving after an
abort appended frames to the PREVIOUS stream and opened a new transaction on it; (2) a pick whose first file did not decode
appended its frames to an older, committed stream; (3) nothing aborted an import when its window closed (the API's rule
"its `Retiring` host event must abort"). Fix (source written, compile pending):

- NEW `✏️editor/🫧️transient/🦀️.rs`: `RemodelingWindowTransient { import: Option<RemodelingImport { streamId?, done, total }> }`,
  one generic `RemodelingWindowTransientOwner<K>` registered for all three window kinds (Model, Frames, Report — the import
  verbs live in the palette), `current` / `addressed` / `register`; JSON text + value pack codecs, retirement, one-item bound.
- `importFramePayload` / `importVideoFramePayload` / `importVideoDone` / `importAbort` run through `handle_in_window`
  (`RemodelingCommand::import_in_window`, retained `step` writes the partition via `EphemeralEmit.window_transient`; routes
  declare `[Artifact, WindowTransient]`). File/tick 0 starts an import, the first decodable one mints its stream, a later
  tick of an ended import (committed, aborted, stream gone) is dropped; the last file / the done commits and clears. A single
  picked file stays one plain commit with no tool state; a multi-file pick or a video import without a window is refused at
  its first tick (`remodeling-import-window-required`).
- `importAbort { reason? }`: `reason: None` = user cancel (window's import, else the one still open via `streams.last()` —
  the palette escape hatch); `reason: "retired"` = host abort of that window's import only. `host_event`: `Retiring` →
  `importAbort{reason: retired}`; blur / captureLost / utility / base moved leave an import running.
- A video file inside a multi-file pick contributes its in-process sampled frames to the pick's ONE stream (one stream →
  one transaction key → the escape hatch stays exact); a single picked video stays its own video stream. Shared
  `import_video_bytes_payload::sample_video` (probe + demux + decode + blur gate) replaces the inline loop.
- Tests: dispatch helper gives import verbs the Frames test window; laws added — the importing window closing aborts with
  zero trace (late tick + done dropped), other host facts / another window closing leave the import running, a pick whose
  first file does not decode mints its own stream, a windowless multi-file pick is refused; the abort law also asserts the
  late tick/done drop. Route fixture `🚧️retained-command-limits` lanes updated; lane law asserts the exact two lanes.

### S3.2 Raster bucket → parametric `fill-region` (census row S2b, missed in session 1) — 12:20

The React bucket flooded the region on the HOST (TS `floodSelection`) and committed an absolute PNG through `editPixels{fill}`:
history edited an opaque image, the click's intent (seed, tolerance, colour) was lost, and only one host could flood. Now:

- ONE Rust flood + fill engine: `semio_framework_pixels::editing::{flood_selection, fill_in_place}` (byte-equal twin of the
  TS `floodSelection`; `fill_in_place` byte-identical to the whole-image `PixelEditJob` under the same coverage). Shared
  corpus section `floodSelections` (9 cases) written by an independent Python BFS oracle `🧪️s3-strokes-flood-corpus.py`
  (`--check` mode), read by Rust (`flood_selection_language_neutral_cases`) and TS (bun test). Law
  `the_in_place_filler_equals_the_whole_image_job` (flood region + graded coverage × Fill/AlphaFill).
- Leaf `🧬️mutations/🪣️fill-region` `{layerId, target: pixels|mask, seed{x,y}, tolerance 0..255, color[4 unit], selection:
  runs|null}`: schema with full `x-semio-ui` en/de + `ordered-selection-runs` invariant, descriptor (text `fill-region`,
  binary tag 18), diff floods + fills the TARGET image (mask coverage on a mask), files the result exactly like a stroke.
  `paint-stroke`'s canvas machinery is now shared (`canvas`, `painted`, `painted_diff`, `painted_inverse`, `refused`,
  `selection_invariant`, `selection_mask`, `channel_byte`, `grey_byte` → `pub(crate)`; no duplicated repaint code).
  Codes: Fatal invariant (field), Error target-missing / target-mismatch (locked, no pixels, no mask, seed outside, selection
  past the image), Warning no-op, Fatal capacity / image-* / rasterize. Label en/de ("Bucket fill from (x, y) on layer L" /
  "Farbfüllung ab …", mask variant). Wired: aggregate variant + `KINDS`, schema `oneOf`, text DSL + grammar, binary
  protocol tag 18 + digest/retirement/target/retained-candidate arms (the candidate now applies either repaint leaf through
  `Mutation::diff`), proto (field 19), graphql, TS twin `parseFillRegion()`, oracle catalog (8 scenarios, kinds, manifest).
  Leaf laws: print-of-leaf quintets (8 cases, `emit_committed_fixtures` ignored test — emission pending compile), engine
  equality, exact inverse, edited seed/colour re-derives itself and its downstream, labels en/de, invariant breaches, store
  fold. TS: `🪣️fill-region/🧪️tests/🟦️.ts` (Ajv `semioSchemaAjvV1` ≡ twin on every committed mutation + 9 refusals).
- Tool `✏️editor/🎮️commands/🪣️fill-region` `fillRegion{layerId, x, y, tolerance}`: colour / target / mask value / selection
  from the session config, ONE `ToolTransaction` `s.raster.raster@1/*#editor#fillRegion` through the paint tool's one-shot
  runner (`raster_paint_commit` generalized to `raster_tool_commit(tool, seed, leaf)`; layer refusals extracted to
  `paint_target_refusal`). Wired: `app_commands!` row (appended), action bridge, retained ids, publication contract, proofs,
  manifest action + describe en/de + Migrated; editor test counts 26→27 / 27→28 / every_command 27→28.
- React `Paint2dHost` bucket dispatches `fillRegion` (no host flood, no `editPixels{fill}`); the wand still floods host-side
  for the selection (config, not history). `editPixels`/`editMask` remain for menu filters and mask fill (dialog actions,
  not gestures — recorded as an open item below).

### S3.3 Closure items from the coordinator (design §20.1) — 12:25

- process3d cursor: `Emit::amend_config(…, PROCESS3D_CURSOR_COALESCE_KEY)` → one plain `Emit::config` per seek (stepper
  press / step / typed back-forward-all — process3d has no play or scrub loop, every press IS a seek end); engagement replay
  likewise; constant deleted; cursor law renamed `every_cursor_verb_is_one_config_write_and_no_document_edit`.
- `coalesce_key` assertions removed from my tests (process3d cursor/world, raster paint-stroke + fill-region, wfc 2d/3d).
- Left (mechanical, die with the framework field): `protocol::Edit { coalesce_key: None }` literals raster ED:664,832, wfc
  bitmap ED:712, process3d ED:814,1149. Reported to the coordinator.

### S3.4 wfc node-graph rows + G7 labels (coordinator, design §13.3 / §20.4) — 12:55

- wfc 2d + 3d decode `nodeGraphEdit` ONLY through the shared `semio_framework_tool_machine::node_graph_edit_rows` (no
  plugin-local decoder): `move` → relative `drag-slots`, `connect` → `connect-slots` (no-op when the adjacency exists, either
  direction), `disconnect` → `disconnect-slots`, `delete {nodeIds, synapseIds}` → cuts + `delete-slot` (cascading wires),
  `setSlider` / `insertPort` refused by name (a wfc slot graph has neither); a malformed row refuses its whole batch.
  `setHostSnapshot` decoding is DELETED (`wfc2d_host_snapshot_edit`, `host_snapshot_gesture`, both `*_HOST_SNAPSHOT_GESTURE`
  presses, endpoint splitters, `WFC_3D_GRAPH_EPSILON`, the dead `slot_coordinate`). Tests rewritten on rows + a law per crate
  against the shared fixture `📺️renderer/🧑‍🎨engine/🧫️fixtures/🧫️node-graph-edit-rows` (every accepted row decodes or is the
  named wfc refusal; every refused row refuses its batch) + cut/delete laws.
- G7: all 30 hand-written `Emit { description }` labels in wfc removed (2d 7, 3d 8, bitmap 7, grid2d 3, grid3d 5): the
  command→leaf maps return the leaf only, drag-tool emits carry no description, config/effect emits use plain ctors. Helper
  `🧪️s3-strokes-drop-emit-labels.py` (balanced tuple stripper, refuses multi-argument calls). Gate
  `bun ./📜️script.ts schema mutation-labels` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): wfc/raster/remodel/
  process findings 30 → **0** (repo total 214 → 180).

### S3.5 Verification while the framework is red (12:40–13:05)

The framework went red at 12:32 (Codex peer schema-state / registry extraction: `🧬️schema/📇️registry/🦀️.rs:349,393,511…`
duplicate `ArtifactSchemaRegistry`, unresolved `semio_framework_os_kernel`), so every cargo run stopped before my crates:
`test-raster-2` (11:54, fingerprint write ENOENT in the shared build dir), `test-raster-3` (12:23, replication `StateClass`,
fixed by the peer 12:24), `test-raster-4` (12:39, schema registry). Cargo-free verification so far:

| Check | Command | Result |
|---|---|---|
| pixel corpus, TS twin + new flood section | `bun test ./🧰️framework/🔨️modules/🔲️pixels/✍️editing/🧪️tests/🟦️.ts` | **60/60** |
| flood corpus is the Python oracle's print | `python3 🧪️s3-strokes-flood-corpus.py --check` | exit 0 |
| fill-region + paint-stroke input descriptors | `bun 🗑️generated/s3-strokes/audit-leaves.ts` (`mutationInputAudit`) | 6+6 inputs, **0** findings |
| `schema mutation-inputs` per scope | cwd `…/🧪️test`, `--under ✏️s/🔌️plugins/<scope>` | wfc 153/153 0, remodel 56/56 0, process 25/25 0, raster 49/49 + 1 `leafUncatalogued` (fill-region, central `schema generate`) |
| `schema mutation-payloads` per scope | same | wfc 85/85 0, remodel 136/136 0, process 15/15 0, raster 23/23 + 1 `unwitnessed` (fill-region quintets emitted after compile) |
| G7 labels | `bun ./📜️script.ts schema mutation-labels` | my trees **0** (was 30) |
| taxonomy, new dirs | `bun ./📜️script.ts verify taxonomy report --scope <dir>` | remodel `🫧️transient` clean, raster leaf `🪣️fill-region` clean; command dir `🎮️commands/🪣️fill-region` `directory-kind-unresolved` — the same pre-existing finding every raster/remodel command dir has (`🎨️edit-pixels`, `🖌️paint-stroke`, `🛑️import-abort` checked) |
| React Paint2dHost | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost` | run 1 59/60, run 2 55/60 — a DIFFERENT selection-timing test fails each run (load ~100, none touches the bucket path); new bucket law `-t bucket` **1/1** (one `fillRegion{layerId,x,y,tolerance}`, no `editPixels`) |
