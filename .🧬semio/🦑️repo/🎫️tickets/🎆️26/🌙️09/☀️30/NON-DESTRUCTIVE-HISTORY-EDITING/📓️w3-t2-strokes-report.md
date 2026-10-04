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

### S3.6 Resume after the usage cut + reboot (18:40)

- The edit in flight at the cut was ALREADY complete on disk: the label stripper ran on grid2d / grid3d / bitmap and the
  remaining `description` fields + tests were fixed before 13:05 (re-verified: `description: Some` 0 in wfc editors; G7
  `schema mutation-labels` → my trees **0**, repo 18). Three stale doc comments still named `setHostSnapshot` (wfc 2d
  command doc, wfc 2d + 3d drag-tool module docs) → rewritten (rows via `node_graph_edit_rows`). A peer edited the wfc 2d
  editor at 14:43 (unrelated region); my regions intact.
- Framework compiles again (`cargo check -p semio-framework-schema --lib` exit 0, 18:46); raster `cargo test` started 18:46.
- Remodel on §15 (coordinator: "§15 PROVEN", 12:4x): the conversion is source-complete (S2.5 + S3.1); its proof is the remodel
  lib test run queued after raster.

### S3.7 State at 19:10 — cargo blocked by peer framework splits

- 18:46 raster `cargo test` (`test-raster-5`) compiled 12 crates, then sat at 0 % CPU with ~10 other idle cargos (shared
  build-dir lock wait) and was terminated (SIGTERM, exit 143) at 19:01. 19:07 retry (`test-raster-6`): kernel red from a peer's
  dsl crate extraction — `🗣️dsl/🦀️.rs:15-16` + `📖️grammar/🦀️.rs:2-3` unresolved `semio_framework_dsl`, `🧠️lsp/🦀️.rs:4`,
  `🏪️store/🦀️.rs:12182,13688,23452` + `🧩️composition/🗄️durable-group/🦀️.rs:149,166` `&[FieldValue]` vs `Vec<FieldValue>`
  (owner guess: S3-INFRA / Codex peer split). None of my crates was reached.

**Owed (WRITTEN BUT UNVERIFIED — framework red):**
1. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-strokes cargo test --manifest-path ✏️s/Cargo.toml -p
   semio-s-artifact-raster-raster --lib` (paint-stroke refactor, fill-region leaf + tool, binary/text/digest arms, counts),
   then `… -- --ignored emit_committed_fixtures` (8 fill-region quintets) and `bun test ./…/🪣️fill-region/🧪️tests/🟦️.ts`.
2. `cargo test -p semio-framework-pixels --lib` (root workspace: `flood_selection_language_neutral_cases`,
   `the_in_place_filler_equals_the_whole_image_job`).
3. `cargo test … -p semio-s-artifact-remodel-remodeling --lib` (§15 import + window tool state laws).
4. `cargo test … --features component-app-assembly -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-3d` (+ bitmap,
   grid2d, grid3d lib tests) — rows on the shared decoder, label removal.
5. `cargo check --tests -p semio-s-artifact-process-process3d`; `PROCESS3D_FIXTURE_OUT=… cargo test … -- --ignored
   regenerate_example_fixtures` → copy the three prints over `🖼️assets/{🎬️demo,🌲️concrete-forest}/🗣️.dsl.semio` and
   `PROCESS_3D_PLATE_EXAMPLE_TEXT` (still the pre-brace record-list form); then its tests.
6. `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-{raster,remodel,wfc,process}`;
   wasm32-wasip2 check of the four plugin crates.

**Open (not built, routed):**
- Raster §17.2 last sentence: stroke preview from a window transient while streaming (today: host-local preview, one
  `paintStroke` dispatch at release in both hosts).
- wgpu raster surface (`🗺️surface/🎨️paint`) has brush/eraser only — no bucket (nor wand/selection tools): parity gap for the
  wgpu owner (S3-W2C); the guest `fillRegion` verb is host-neutral.
- Hosts do not dispatch `importAbort` when a pick/import is cancelled (`🛠️ShellHelpers` `dispatchOpenedFiles` abort signal,
  `RequestMediaFrames` driver) → S3-W2B / S3-W2C; the window-closing abort and the palette action cover it meanwhile.
- Raster menu filters (`editPixels` invert / brightness / blur / crop …) are dialog actions, not gestures, but still commit
  opaque PNG images; a parametric `apply-filter` leaf (diff re-runs `PixelEditJob`) would make them history-editable —
  recommended follow-up.
- 5 `protocol::Edit { coalesce_key: None }` literals (raster ED:664,832, wfc bitmap ED:712, process3d ED:814,1149) — die with
  the framework field (CLOSURE).

**Coordinator actions:** central `schema generate` (new leaf `raster.raster` `fill-region`: clears `leafUncatalogued`);
`describe` + re-activation for raster (new `fillRegion` action), remodel (`importAbort{reason?}`, window-transient owners on
all three window kinds), wfc 2d/3d/bitmap/grid2d/grid3d (labels, rows), process3d (cursor emit); no new launch rows.

### S3.8 Audit `📓️audit-s3-tools.md` K1–K4 + open items (10-02 19:30 → 10-03 06:30, resumed on TREE GREEN core)

**K1 (major, remodel O(new frame) per tick) — done.** The import's tool state in the importing window's transient
(`✏️editor/🫧️transient`, `RemodelingImport { stream_id, done, total, rolling_scores }`) keeps the blur gate's rolling scores,
so a tick decodes ONLY its own frame; the old per-tick rebuild that re-decoded up to 15 stored frames is deleted. The gate is
ONE helper owned by the reconstruction engine (`⚙️engine/🏭️reconstruction`: `BLUR_GATE_ROLLING_WINDOW`, `sharpness_score`,
`blur_gate_admits(rolling, window, score, min_sharpness)`), used by the engine's own `FrameSource` (its `VecDeque` +
`rolling_median` deleted), by the tick (`📼️import-video-frame-payload`) and by the in-process sampler
(`💽️import-video-bytes-payload::sample_video`) — the tick module's "local mirror" copy is gone.
Law: `the_blur_gate_rolls_in_the_import_state_and_refuses_a_blurred_frame` (20 sharp ticks keep exactly
`BLUR_GATE_ROLLING_WINDOW` scores; a flat frame is refused).

**K2 / K3 — done.** A new import (tick 0 / file 0 of a multi-pick) in a window whose import still streams is refused
(`remodeling.import.open`, `refuse_while_streaming`); law `a_new_import_over_a_live_one_in_the_same_window_is_refused`. A
windowless streamed import is ONE typed fault (`remodeling.import.window-required`, `import_window_required()`) at tick,
frame-file, transient and editor; the 7 empty region pairs in `🖼️import-frame-payload` are deleted. K3's framework
boilerplate (a derive for window-transient partitions — the remodel transient hand-writes Mutation/Diff/Retire/Dsl/pack) is
routed to S3-W2A.

**K4 (major, wgpu bucket + gesture-level replay law) — done.**
- The bucket's colour tolerance is now ONE session value both hosts read: `RasterConfig.fill_tolerance` (default 24, all five
  schema surfaces + `SetFillTolerance` config mutation + `setFillTolerance` verb `🎮️commands/🌊️set-fill-tolerance`, View /
  Config lane / Migrated); `FillRegion` (the click) is `{ layerId, x, y }` only and the leaf records the session tolerance.
  `Paint2dScene.fillTolerance` (framework scene, Rust + TS + both scene fixtures + wire golden) carries it to React, whose
  wand and bucket now read it and dispatch `setFillTolerance` (its local `useState(24)` is deleted).
- Guest: new utility `paintBucket` ("Bucket"/"Farbeimer", icon `paint-bucket`) on the composite window + option panel
  `☑️options/🪣️bucket` (tolerance slider + the shared foreground picker, `brush::foreground_measure`).
- wgpu (`🗺️surface/🎨️paint`): `PaintStrokeCommand` → `PaintEditCommand`; `PaintGesture::click` maps one press under
  `paintBucket` to the layer image's pixels, refusing a point off the pixel grid (target extent now returned by `target`);
  `action()` = `fillRegion` for the bucket. EngineCanvas `write_paint2d_edit` publishes `fillRegion { surfaceId, layerId, x,
  y }` at press (strokes unchanged at release). 06:08 wasm32 E0716 in it (my edit) fixed by binding the arrays to locals.
- Laws: wgpu `native_bucket_click_is_one_fill_region_intent_in_layer_pixels` (paint unit),
  `paint2d_bucket_press_publishes_one_fill_region_click_in_layer_pixels` (EngineCanvas); guest
  `🧪️fill-tool-transactions::a_bucket_click_is_one_transaction_whose_seed_tolerance_and_colour_time_travel_edits_replay_exactly`
  (session tolerance 7 → click = ONE row stamped `#fillRegion`, head == fresh fold of the clicked leaf; then
  `historyEditInput` `/seed/x` 3, `/tolerance` 40, `/color` green → Reviewing non-blocking, committed head untouched →
  overwrite → head == `apply_raster_mutation(base, edited leaf)`, mutation id unchanged); config
  `fill_tolerance_shared_vectors_round_trip_and_restore` + TS twin over `🧫️fixtures/🌊️fill-tolerance`; composite measures
  (3 groups, bucket slider → `setFillTolerance`). Command input schemas added for `fill-region` and `paint-stroke`.

**Verification (10-03):**

| What | Command | Result |
|---|---|---|
| raster config schema twin | `bun test ./✏️s/…/✏️editor/🎚️config/🧪️tests/🔬️unit/🟦️.ts` | **19/19** |
| React Paint2dHost (bucket, wand tolerance, selection) | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost` | **61/61** (3 files) |
| engine contract (paint2d scene literals) | `… vitest run … engine-contract` | **699/699** |
| surface idle frames | `… vitest run … 🪶️surface-idle-frames` | **3/3** |
| Interpreter + UiDocumentStore + typed wire (exhaustive) | `SEMIO_TEST_LEVEL=exhaustive … Interpreter UiDocumentStore typed` | 392/407 — the 15 failures are NOT the paint scene: 9 × 5 s timeouts (load 30), 1 ENOENT `🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs` (peer move), 2 assertions on the `owned-hash` / `intake` fixtures; the typed-scene paint-2d cases pass |
| wgpu renderer wasm32 | `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **exit 0**, 172 warnings |

**Open item → done: menu filters are a parametric leaf (no opaque image commit).** New leaf `🌈️apply-filter`
`{ layerId, target, filter, amount, selection }` (13 in-place filters: invert, grayscale, clear, flipHorizontal/Vertical,
brightness, contrast, saturation, gamma, threshold, posterize, blur, sharpen; per-filter bounds in the schema via `allOf`
if/then and in `invariant`; a flip takes no selection). Its diff runs the ONE pixel engine (`PixelEditJob`) on the target
image and files the result through `paint-stroke`'s canvas (`painted_diff` / `painted_inverse`), so a history edit of the
filter or its amount re-derives the image on any base. Wired on every surface: aggregate enum + `KINDS`, `oneOf`, proto
(field 20), GraphQL, binary record tag 19 / variant 20 + digest phases + retained apply arm, text grammar + DSL, TS twin
`parseApplyFilter` + `RASTER_FILTERS`, oracles catalog (9 scenarios) + manifest row; leaf label en/de ("Blur 3 on layer
L" / "Weichzeichnen 3 auf Ebene L"). Command `applyFilter { layerId, filter, amount }` (`🎮️commands/🌈️apply-filter`, session
target + selection, Artifact lane, Migrated, describe en/de). React: the Adjustment "Apply", the two flips and "Clear selected
pixels" dispatch `applyFilter`; rotate / resize / crop change the image extent and stay `editPixels` (a geometry leaf is the
remaining follow-up), the mask "Fill mask" stays `editMask`.
Laws: leaf `🧪️tests` (quintet print + emit, engine equality for brightness/blur/flip/posterize, exact inverse, edited
filter re-derives itself and its downstream, labels, 8 invariant breaches, store fold); TS twin (committed quintets ≡ Ajv ≡
twin; 10 refusals); command (session target/selection, flip without selection, 5 refusals, one plain edit); React
`menu filters dispatch ONE parametric applyFilter and geometry stays a pixel edit` + the Apply/cancel/lock laws re-stated on
`applyFilter` (the lock law now cancels the still-prepared "Crop to selection").

| What | Command | Result |
|---|---|---|
| React Paint2dHost after the filter move | `… vitest run … 🖌️Paint2dHost` | **61/61** |
| apply-filter schema ≡ twin (pre-fixture probe) | `bun 🗑️generated/s3-strokes/probe-apply-filter-twin.ts` | 4 admits + 10 refusals agree, **0** failures |
| `schema mutation-inputs` raster | cwd `…/🧪️test`, `--under ✏️s/🔌️plugins/🖨️raster` | 49/49 + 2 `leafUncatalogued` (fill-region, apply-filter → central `schema generate`) |
| `schema mutation-payloads` raster | same | 23/23 + 2 `unwitnessed` (quintets emitted by the ignored Rust test once it compiles) |
| taxonomy, new dirs | `verify taxonomy report --scope …` | leaf `🌈️apply-filter` clean; `🎮️commands/🌈️apply-filter`, `🎮️commands/🌊️set-fill-tolerance`, `☑️options/🪣️bucket`, `🧫️fixtures/🌊️fill-tolerance` `directory-kind-unresolved` — same class as the pre-existing `🎮️commands/🎭️set-mask-value` and `🧫️fixtures/🎭️mask-paint` (`📚️library/🔣️taxonomy.json` `members-of-*` registry) |

**Cargo (10-03), one gated run at a time — blocked by peers, none of my crates reached:**
- 05:49 `check -p semio-s-artifact-remodel-remodeling --lib --tests`: workspace manifest load failed — `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/Cargo.toml` was a 104-byte stub with only `[dev-dependencies]` (peer mid-edit, fixed 05:50).
- 05:51 + 06:16 same check: `semio-framework-artifact-workflow-workflow` red, 222 errors — `🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs:325-328,559-562,1186-1189` (`*_controlled` expect `ValueError`, found `String`) and its `🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` `?` `From<ValueError>` (peer value-refusal migration, `🌱️value/⚠️refusal` 05:09). Raster AND remodel normal-depend on it through `semio-framework-os`.
- 06:3x `test -p semio-framework-pixels -p semio-framework-surface -p semio-framework-ui --lib` (private target dir): `semio-framework-os-kernel` red, 52 errors — `🔨️modules/🚪️io/🦀️.rs:2406-2464` (`IoError` lost `message`, `From<String>`), `🏪️store/🦀️.rs` ×12, `🏪️store/📜️space-history/…/🪶️sqlite/🦀️.rs` ×4 (same peer migration).
- 06:4x–06:55 what does NOT depend on the kernel, run instead (private target dir, `CARGO_INCREMENTAL=0`):
  `cargo test -p semio-framework-pixels -p semio-framework-ui --lib` → pixels **58/58** (owed #2 ✓: `flood_selection_language_neutral_cases`,
  `the_in_place_filler_equals_the_whole_image_job`); `cargo test -p semio-framework-ui --features wgpu --lib -- paint2d scene_records`
  → `scene_records_serialize_to_golden_json` **1/1** (the `fillTolerance` golden). `cargo test -p semio-framework-ui-scene --lib`
  (paint-2d lane contract) does not compile: peer test `🖱️ui/🎬️scene/🧪️tests/🔬️scenes-value-round-trip/🦀️.rs:41` calls
  `ValueError::new` with one argument (now two — the same value-refusal migration).
- §17.2 streamed stroke preview: the audit files it as K5 minor, "track, not blocking" — kept open (design: `paintStroke{phase:
  stream|commit|abort}` with the provisional leaf in the composite window transient, a `strokePreview` scene lane, both hosts
  painting committed ⊕ provisional — the note ink tool's pattern); not started, verification of the written work comes first.

### S3.9 Resume 10-03 10:45 (usage cut ~07:15) — §17.2 streamed stroke preview, filters pixels-only

**Interrupted patch verified complete.** The apply-filter aggregate/codec wiring had landed whole (counts per file: crate
root 4, aggregate 3, oneOf 1, GraphQL 2, proto 2, protocol 1, grammar 2, text 4, binary 5, binary sample 1, mutation
sample 1, TS twin 6, oracles 7, editor 9, editor tests 3).

**§17.2 — a stroke streams into the Composite window's transient and the window previews it (done).**
- New partition `🖼️composite/🫧️transient` (`RasterCompositeWindowTransient { stroke, closed }`,
  `RasterStrokeToolState { gesture, states, authoring_seed, transaction, stroke }`, owner on the Composite window kind,
  `register` / `current` / `addressed`). Registered by `register_window_transient_owners`; `paintStroke` publishes on
  `[Artifact, WindowTransient]`; the retained route runs `paint_stroke::PaintStrokeWork` with the job context (raster's
  `build_tool_job` now hands `context: Some(request.context)`, as remodel does).
- `paintStroke` gains `phase` (`stream` | `commit` | `abort`), `reason`, `gesture`. The paint tool chart is
  idle/painting (`Stroke`/`Stream`/`Finish`/`Cancel`): ticks extend ONE leaf (`stroke:0`, consecutive repeats once, ≤ 2048
  samples) inside ONE open transaction; the commit publishes it as one edit stamped with that transaction; an abort, a
  commit with nothing to paint and an unrestorable state leave zero trace; a one-shot or a tick of another press aborts
  the open one (`captureLost`); a late tick of the press the window last closed is dropped (`closed`). Host facts
  (`host_event`): blur → `blur`, capture lost → `captureLost`, utility switch / closing → `retired`, history edit opened →
  `frozen`; a remote edit keeps the stroke (the leaf repaints on any base). Without a window, a streamed phase is refused.
- Preview: `render_with_request_context` paints the Composite body from `raster_stroke_preview` — the provisional leaf
  applied by the ONE rasterizer — and retires the preview snapshot after rendering; the document and history never see it.
- Hosts stream: React `Paint2dHost` and wgpu `RasterHost` send `stream` ticks of 16 new samples (`STROKE_STREAM_BATCH` /
  `RASTER_STROKE_STREAM_BATCH`) under one press id (`paint:<ms>:<n>` / `stroke:<n>`), commit the rest on release, abort a
  dropped streamed press (`captureLost`); a stroke shorter than one batch is still ONE plain dispatch. EngineCanvas
  publishes the host's pending edit before and after each pointer event (`flush_paint2d_edit`).
- Host gesture law changed (both hosts, both fixtures): a stroke now holds while its target's PLACEMENT holds (selected,
  paintable, same pixel grid and extent), not while the image content is unchanged — the leaf names samples and brush,
  never the pixels under them, and the streamed preview itself changes the content key. wgpu `PaintGesture` drops its
  content revision (`StrokeRevision`/`revision_value` deleted; `PaintEditCommand.target`), React `pixelGestureRevision`
  = `[id, target, width, height, matrix]`. Fixtures re-stated: `🖌️stroke-revision` `image` → no cancel;
  `🎭️mask-stroke` `mask enable` / `mask invert` → no cancel (`mask image` still cancels: its extent changes).
- Laws: command (pure) — one transaction of every sample in order, same ref across ticks, equal to the one-shot leaf;
  preview == provisional leaf through the rasterizer; cancel / host abort / another press; streamed phases need a press and
  a window. Mounted (`🧪️stroke-stream-transactions`) — ticks never touch document or history, commit = ONE row of
  `#paintStroke` whose head equals the one-shot fold, late tick dropped; blur ends the open stroke with zero trace and the
  next press paints. wgpu paint — `native_long_stroke_streams_batches_and_commits_the_rest_under_one_press`,
  `native_dropped_streamed_stroke_leaves_the_abort_of_its_press`; EngineCanvas —
  `paint2d_long_stroke_streams_ticks_then_commits_under_one_press`; React — long stroke streams 16+16 and commits 10,
  cancelled stream aborts its press, short stroke stays one dispatch.

**Filters are pixels-only.** A filter changes colour channels; on a mask canvas (`[255,255,255,coverage]`) `invert` would
zero the coverage, not invert it. `apply-filter` drops `target` (leaf, schema, twin, proto, GraphQL, text, binary, digest,
oracles: 8 scenarios); the command refuses a session painting the mask (`raster-filter-pixels-only`).

| What | Command | Result |
|---|---|---|
| React Paint2dHost (streaming, filters, bucket) | `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost` | 62/63 — the one failure is `selection combination cancels with Switch utility` (progress 0.375 vs 0.75, load 56); that group alone `-t "selection combination"` **4/4** twice |
| Paint2dHost editing twin (revision law, mask revisions) | `bun test ./🧰️framework/…/🖌️Paint2dHost/✍️editing/🧪️tests/🟦️.ts` | **62/62** |
| apply-filter schema ≡ twin (pixels-only) | `bun 🗑️generated/s3-strokes/probe-apply-filter-twin.ts` | **0** failures |
| wgpu renderer wasm32 (paint + EngineCanvas streaming) | `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **exit 0**, 172 warnings; my two (`erase`, `erases`) removed after |
| taxonomy | `verify taxonomy report --scope` `🫧️transient`, `🧪️stroke-stream-transactions` | **clean** both |

**Open (my trees):** geometry menu ops (rotate / resize / crop) still commit `editPixels` images and "Fill mask" an
`editMask` image — a parametric `transform-image` leaf (extent and placement change) and a mask fill leaf are the remaining
follow-ups; owed cargo runs unchanged (S3.7 list + the new laws), blocked on the peer value-refusal / stdio sqlite
migration. **Coordinator:** delete the stale placeholder `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🫧️transient/📌️.empty.md`
(not mine; the dir now holds `🦀️.rs`); central `schema generate` (fill-region, apply-filter); describe + re-activation for
raster (new verbs `setFillTolerance`, `applyFilter`, `paintStroke{phase,reason,gesture}`, utility `paintBucket`, Composite
window transient owner).

### S3.10 Gates to 0 (remodel + raster), geometry and mask fill as parametric commands — 10-03 11:00 → 11:48

**Remodel gates: payloads 30 → 0, inputs 175 (225 once the refs resolved) → 0.** All of it is in the schemas; no code
changed.
- Fixed refs that no longer resolved: `replace-qc`, `commit-reconstruction` and `replace-trajectory` now point at
  `$defs/CameraTrajectory` / `QcReportSnapshot`.
- `replace-stream-source` now accepts `source: anyOf[VideoSource, null]`.
- `commit-reconstruction` now matches its Rust struct:
  - `mesh` is nullable, because `Option<Box<RemodelingMesh>>` keeps the stored mesh when it is null;
  - `assets` is an array of `{id, contentId?}` (`Vec<ReconstructionAssetCommit>`, the same shape as the TS twin), not a map.
- The 4 `update-*-params` refusals now name a declared `x-semio-invariant` (en/de):
  - geo: `positive-geo-extents`
  - feature: `positive-feature-budget`
  - ingest: `positive-ingest-budget`
  - match: `unit-ratio-test`

  Each refusal outcome now carries `"invariant": "<id>"`. The Rust refusal tests read status, code and path only, and the
  Python oracle reads feature-row codes only, so neither is affected.
- Input labels:
  - en/de `x-semio-ui` labels, descriptions and option labels on 13 artifact `$defs`: GcpObservation, FrameRef,
    CameraCalibration, GroundControlPoint, RigExtrinsic, MediaStream, VideoSource, and the Dense, Feature, Geo, Ingest,
    Match, Mesh, Motion and Sfm params.
  - The 9 fields that use `Binary32/64Transport` carry no `unit`, because the lint applies a unit only to a plain number.
    Their descriptions say the unit instead.
  - Engine outputs are `widget: hidden`. This covers the commit-reconstruction lanes, replace-sparse, -dense, -mesh-result,
    -trajectory, -tracks, -geo-products and -qc, and append-content's `first` and `chunks`. append-content's `kind` is
    now an enum.
  - Semantic diff of `artifact.json` against HEAD, ignoring `x-semio-ui`: no change.

**Raster: payloads 4 / inputs 4, all owed to steps I am not allowed to run.**
- *Unwitnessed* (fill-region, apply-filter, transform-image, fill-selection): their quintets come from
  `--ignored emit_committed_fixtures`. That needs cargo on the raster crate, which depends on stdio gif/jpg/png/svg/tiff/bmp
  (the peer migration is still active).
- *leafUncatalogued* (the same four): these clear with the central `schema generate`.
- Run through the reader directly (`probe-raster-input-audit.ts`), the four leaves have **0 findings and 0 word-only
  floats**.

**Geometry and mask fill are parametric.** These are new editor commands, built like `applyFilter`. Each publishes one plain
edit holding one leaf and is refused with zero trace.
- `transformImage{layerId, operation, x, y, width, height, bilinear}` (`🎮️commands/🔄️transform-image`): the session must
  edit pixels (`raster-transform-pixels-only`), and the leaf invariant is checked before publishing
  (`raster-transform-invalid`).
- `fillSelection{layerId}` (`🎮️commands/🫗️fill-selection`): target, colour and selection come from the session; filling
  the mask uses the mask value at the brush opacity.
- Wiring: action decode, `app_commands!` rows (appended), retained ids, publication contracts (Artifact), the proofs list,
  manifest actions with en/de describe and `Migrated`, the crate-root mods, and editor unit counts (31 / 32 / 31; 32 rows;
  wire keywords). The leaf `fill-selection::invariant` is now `pub(crate)`.
- React, menu: rotate/resize → `transformImage`, crop → `transformImage{crop, bounds}`, Fill mask → `fillSelection`,
  Delete/Backspace → `applyFilter{clear}`. The `submit`/`apply` editPixels/editMask path is gone. **No host dispatches
  `editPixels` or `editMask` any more.**
- Tests: the selection-focus fixture's `Fill mask` completion command is now `fillSelection`; the crop, rotate, resize and
  mask-fill laws were re-stated; surface-idle-frames now expects `applyFilter`.

| What | Command | Result |
|---|---|---|
| remodel payloads / inputs | `schema mutation-payloads` / `mutation-inputs --under ✏️s/🔌️plugins/📸️remodel` | **136/136, 36/36 witnessed, 4 invariants, 0** / **56/56, 0** |
| raster payloads / inputs | same `--under …/🖨️raster` | 23/23 clean, 18/22 witnessed (4 owed) / 49/49, 4 uncatalogued |
| wfc, process | same | **85/85, 82/82, 0 / 153/153, 0**; **15/15, 15/15, 0 / 25/25, 0** |
| leaf labels (repo) | `schema mutation-labels` | **0** findings (3098 labels) |
| new raster leaves through the reader | `bun 🗑️generated/s3-strokes/probe-raster-input-audit.ts` | **0** findings |
| twins ≡ schema | `probe-apply-filter-twin.ts`, `probe-transform-image-twin.ts` | **0 / 0** failures |
| React Paint2dHost | `SEMIO_TEST_LEVEL=long bun x vitest run … 🖌️Paint2dHost` | **63/63** (run twice) |
| surface idle frames | `… vitest run … 🪶️surface-idle-frames` | **3/3** |
| React package types | `bun x tsc --noEmit -p tsconfig.json` (react package) | 1 error, not mine: `🌐️World3dHost/🟦️.tsx:2226` (peer, uncommitted); my `strokeCanvas` typing fixed |
| raster leaf TS twins | `bun test ./…/{🫗️fill-selection,🔄️transform-image,🌈️apply-filter,🪣️fill-region}/🧪️tests/🟦️.ts` | 0/4: they scan the committed quintets, which are not emitted yet |
| taxonomy | `verify taxonomy report --scope` on the 2 command dirs | `directory-kind-unresolved` only, the same class as every raster command dir |

**Owed cargo**, once stdio is green, one gated run at a time:
- raster `--lib`: the new laws in `🎮️commands/{🔄️transform-image,🫗️fill-selection}/🧪️tests`, the editor unit counts, and
  everything listed in S3.7–S3.9;
- `--ignored emit_committed_fixtures` for the 4 leaves, then the 4 TS twin tests and the raster payload gate.

**Coordinator actions:**
- central `schema generate`: the raster leaf catalog, and the remodel schema hashes;
- describe and re-activate raster (new verbs `transformImage` and `fillSelection`) and remodel (schemas);
- decide on deleting the now-dead `editPixels` / `editMask` routes (`🎮️commands/{🎨️edit-pixels,🖌️edit-mask}`, their
  rows, tests and `🖼️assets/🔄️replacement` test). They are not my files, and removing their rows renumbers the binary
  command variants.

## Session 4 — 2026-10-04

Executor S4-STROKES (Opus), successor of S3-STROKES. Owns raster, remodel, wfc (2d/3d/grid2d/grid3d/bitmap), process3d plugin
trees, the shared Rust flood/rasterizer engines, wgpu Paint2dHost bucket regions. Scratch: `🗑️generated/s4-strokes/`.

### S4.0 Start (02:11) — rule 34 repair-first

- Files in my trees newer than S3.10 (10-03 11:48): raster 135, remodel 269, wfc 313, process 84 — mtime clusters 10-03 19:48
  and 23:31 and 10-04 00:23/01:01 (peer value/DSL sweeps; S3-STROKES was cut ~12:07, so none is a predecessor half-edit).
  Repair = compile each crate (below).

### S4.1 D5 — raster `editPixels` / `editMask` deleted (02:40, source; compile below)

No host dispatches either verb (S3.10). Removed in one compile-atomic wave (`R` = `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`):
- **Deleted paths** (rule 32): `R/🎮️commands/🎨️edit-pixels/` (`🦀️.rs`, `🧪️tests/{🦀️.rs,🟦️.ts}`, `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`),
  `R/🎮️commands/🖌️edit-mask/` (same five files), `R/🖼️assets/` (whole: `🔄️replacement/{🦀️.rs,🟦️.ts,🧪️tests/{🦀️.rs,🟦️.ts},🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}` —
  `replacement_steps` had no caller besides the two deleted verbs, so the module and its TS twin died with them, incl. the
  `pixel_and_mask_replacement_at_capacity_publish_one_reversible_edit` test), and the stale placeholder
  `R/🎭️modes/✏️edit/🪟️windows/🖼️composite/🫧️transient/📌️.empty.md` (D24; the dir holds `🦀️.rs`).
- **Edited**: crate root `🗿️artifacts/🖨️raster/🦀️.rs` (two `pub mod`), `R/🦀️.rs` (`asset_replacement` mod, action bridge rows, `app_commands!` rows —
  greenfield renumbering: every row after `mergeDown` moves down two ordinals; the only pinned bytes, `set-layer-visible` ordinal 2, are
  before them — import line, `RASTER_RETAINED_TOOL_IDS` 31 → 29, publication contracts 31 → 29, proofs tools list, two
  `build_tool_job` arms, six manifest rows, a stale "ten/five verbs" count in a doc), `R/🧪️tests/🔬️unit/🦀️.rs` (two `every_command`
  rows, lane set, two keyword arms, counts 29/30/29 and 30 rows, tests `edit_pixels_retained_publication_survives_undo_and_redo` +
  `mask_paint_retained_publication_survives_undo_and_redo` — their successors are the `applyFilter`/`fillSelection` command laws),
  `📦️packages/🟦️typescript/📜️script.ts` (three deleted test rows), `🧫️fixtures/🔏️publication-authority/🔣️.json` (no reader besides its
  schema; rewritten from the live `RASTER_PUBLICATION_CONTRACTS`: artifact 17 routes, artifact+transient `paintStroke`, config 12,
  host-only `exportPng`; Draft-7 valid against `🧬️schema/🔣️.json#/$defs/RasterPublicationAuthority`).
- **Zero-reference proof**: `git grep --untracked -n -e editPixels -e editMask -e edit_pixels -e edit_mask -e EditPixels -e EditMask -e
  edit-pixels -e edit-mask -e asset_replacement -e 🔄️replacement -e replacementSteps -- ':!.🧬semio/🦑️repo/🎫️tickets'` → only the
  generated descriptors `🌎️hub/🧩️compositions/🖨️raster/🔣️.json` (L1745, L1907) and `🛂️.descriptor.semio` (coordinator: describe raster),
  plus the built dev copy `OSM/🧑‍💻dev/🔌️plugin-modules/🖨️raster/*` (rebuild). Taxonomy/launch rows: none.

### S4.2 D24 remodel ref, gates, cargo-free verification (02:45–03:15)

- **D24 `Binary64Transport` spelling**: `…/📸️remodeling/…/🧬️mutations/⏱️change-stream-sync/🧬️schema/🔣️.json` `newSyncOffsetMs` now refs the canonical
  `https://json.schemas.assets.semio-tech.com/framework/value/schema.json#/$defs/Binary64Transport` (repo census: 258 local `#/$defs/…`, 10 canonical
  in trinity, this was the only cross-artifact spelling). Its one Ajv consumer `🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` registers the framework
  value schema (`ajv.addSchema(valueSchema)`): `bun test --timeout 60000 ./…/🪶️sqlite/🟦️.ts` **21/21** (default 5 s timeout: 20/21, the 272-fixture
  decode law timed out at load 72; not a regression). Leaf hash changes → central `schema generate`.
- Gates (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `--under ✏️s/🔌️plugins/<scope>`):

| scope | `schema mutation-payloads` | `schema mutation-inputs` |
|---|---|---|
| raster | 23/23, 18/22 witnessed, **4 unwitnessed** (fill-region, apply-filter, transform-image, fill-selection — need `--ignored emit_committed_fixtures`) | 49/49, **4 `leafUncatalogued`** (same 4 — central `schema generate`) |
| remodel (after the ref change) | 136/136, 36/36, 5 negative (4 by invariant), **0** | 56/56, **0** |
| wfc | 85/85, 82/82, **0** | 153/153, **0** |
| process | 15/15, 15/15, **0** | 25/25, **0** |

- `cargo test -p semio-framework-ui --features wgpu --lib -- paint2d scene_records` (private target) → **1/1** (`scene_records_serialize_to_golden_json`).
- React `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost` (cwd `…/🎯️targets/⚛️react/📦️packages/🟦️typescript`) → **63/63** (3 files).
- `bun test ./…/🖌️Paint2dHost/✍️editing/🧪️tests/🟦️.ts` → **62/62**.
- Raster `cargo check --lib --tests` 02:34 stopped in `semio-framework-plugin` (peer S4-BUMP mid-wave: `ReadChildHeads`/`ChildHeads`/`PureCommand.head`;
  coordinator: green 02:58). Every crate I own except wfc-bitmap pulls `semio-s-artifact-stdio-semio` (red, peer graph leaf half-add) — waiting for the
  coordinator's "STDIO-SEMIO GREEN". renderer-wgpu `-- paint2d` 03:00: plugin lib red again at `🔌️plugin/🦀️.rs:32203` (`ArtifactStoreOneItemFootprint::for_gesture`
  missing — peer store/plugin edit in flight).

### S4.3 Raster value-refusal migration (03:40–04:27; cut ~04:15, resumed 06:45)

The peer value-refusal migration (`close_step`/sqlite/codec errors `String` → `semio_framework_value::ValueError`) had never reached raster
(not in S4-INFRA's sweep): `check-raster-2` 128 errors, all in my tree. Converted with three ticket input scripts (anchor-counted, residue-checked,
refuse a concurrently changed file):
- `🧪️s4-strokes-raster-sqlite-valueerror.py` — `🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` (27 terminals; `invalid`/`work`/`positioned` helpers; IoError via
  `IoError::from_value_error` in `validate_sqlite_snapshot_subset`).
- `🧪️s4-strokes-raster-record-valueerror.py` — `📦️record/🦀️.rs` (13 terminals, 70 `into_message` shims removed, row/map limits → `OwnershipLimit`,
  overflows → `WorkLimit`, allocation → `AllocationFailed`), `📸️snapshot/🦀️.rs` (`TextError::from_value_error`, `PackError::ValueRefusal`), crate root
  `RasterOwnedMap` `FromValue` (two-argument `ValueError::new`).
- `🧪️s4-strokes-raster-retirement-valueerror.py` — `💾️binary/🦀️.rs` (16 cursor signatures → `ValueError`, 29 + 2 + 12 literal refusals →
  `invariant(..)` = `InvariantViolated`), then by hand: two `control.release()` / one `return_one()` `&str` codes, the disposer fault, two job-fault
  byte sinks (`into_message().into_bytes()`).
- By hand: editor `close_step` ×2 (+ `InvariantViolated` base-root refusals), `advance` bridge, `interaction_topology` → `Result<_, ValueError>`,
  `MediaPayload::Intrinsic` arm in the import job, presence `close_step`, export job `rejected_download: None` initializer + `Fault::from(error.message)`,
  composite window transient `dsl::json` → `semio_framework_pack_json` (`JsonMemberPolicy::Reject`, `PackError::ValueRefusal`).
- Notices (S4-GATES via `main`): wfc 2d / wfc 3d / remodel `*.retained.tool-mismatch` → framework `app.command.tool-mismatch`.
- **`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib --tests`** (`check-raster-4`, 04:15–04:27) → **0 errors**
  (warnings: 21 lib / 105 test, all peer-sweep `unnecessary qualification` + 3 unused imports, none from this session's edits).

### S4.4 First raster lib test run (07:36–07:55) and rule 43

`CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-strokes cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib --no-fail-fast`
(`test-raster-3.txt`; two earlier attempts died: SIGKILL 137 after a silent lock wait, then a shared build-dir fingerprint ENOENT):
**393 tests: 345 ok, 21 FAILED, ~27 not run — the binary ABORTED (SIGABRT)** in
`snapshot::component::sqlite_tests::sqlite_snapshot_raster_deep_flat_native_ownership_and_late_refusal`: the test (new since HEAD, sqlite-snapshot
campaign, S4-INFRA's file class) plants a 1024-deep `DslValue` array in an adjustment parameter; dropping its `Owner` runs `retire_snapshot`, whose
raster retirement cursor admits only `RASTER_RETIREMENT_VALUE_FRAMES = 2 × RASTER_MAXIMUM_NESTED_DEPTH = 64` value frames → `InvariantViolated "Raster
retirement exceeded its admitted fixed depth"` → panic inside a destructor during unwinding → abort. Pre-existing law conflict (the refusal text and bound
predate this session; my conversion only changed its type): either parameter decode refuses values deeper than the retirement admits, or the retirement
cursor pages arbitrary depth — routed to `main` (owner of the sqlite test: S4-INFRA / peer).
Failed (messages lost to the abort; to be re-run once tests resume):
- 4 × `the_committed_quintets_are_a_print_of_the_leaf` (fill-region, apply-filter, transform-image, fill-selection) — EXPECTED until `--ignored emit_committed_fixtures`.
- 4 × binary `raster_store_initializer_*` / `raster_empty_bounds_and_mounted_sixty_four_fuel_progress_across_second_map_page` — compiled while S4-STORE was
  migrating the raster store initializer to the stepped supersession fold (coordinator 07:1x); re-run after.
- `semio_payload_law_raster_mutation`, `snapshot::binary::tests::pack_round_trips_representative_document`, `unit_tests::{composite_scene_syncs_document_and_assets,
  two_instances_converge_disjoint_layer_edits_via_backbone, window_measures_expose_brush_and_eraser_option_groups, context::history_edit_inputs_resolve}`,
  `media_export::tests::{retained_image_export_completes_or_cancels_without_mutating_history, retained_export_refuses_a_superseded_document_and_retires_its_output}`,
  `merge_down::tests::layer_baking_at_asset_capacity_publishes_and_restores_retained_history`, `fill_tool_transactions::a_bucket_click_…replay_exactly`,
  `stroke_stream_transactions::{a_host_fact_ends_the_open_stroke_with_zero_trace, a_streamed_stroke_is_one_row_…}`, `paint_stroke::tests::a_cancel_or_another_press_ends_the_open_stroke_with_zero_trace`.
**Rule 43 (coordinator, ~08:0x, disk 1–7 GiB): checks only** — a `--skip <abort>` re-run was queued and stopped in its gate (no cargo started); the built
binary was pruned by the disk guard. Every raster/remodel/wfc/process3d test run and `--ignored emit_committed_fixtures` are **OWED (rule 43)**.

### S4.5 Raster retirement cursor: arbitrarily deep values, oversized strings (coordinator decision 08:2x) — source

Coordinator: the SIGABRT is a raster defect (a panic in `Drop` is never acceptable); retire arbitrarily deep values iteratively. In
`💾️binary/🦀️.rs` `RasterOwnedRetirement` (no `*StoreInitialization*` region touched):
- **Tail replacement**: an `Array` whose last remaining child is a container, and an `Object` popping its last entry, release their (now empty)
  backing and REPLACE their own frame with the child (`Value` / `ValueEntry`) instead of pushing — a single-child chain of any depth (the sqlite law's
  1024-deep array) retires in ONE frame. `ValueEntry` likewise becomes its value's frame after releasing its key (no push).
- **Frame budget exhausted** (`room = top index + 1 < RASTER_RETIREMENT_STACK_CAPACITY` passed into `frame_action`): the top `Value` frame pushes its
  popped child back and hands the remaining value to the framework's paged owned retirement (`semio_framework_value::retirement::owned_retirement`,
  unbounded cursor stack) as `RasterRetirementOwner::Delegated`; that arm steps it with the frame's byte grant, skipping at most
  `RASTER_DELEGATED_FRONTIER_TURNS = 4096` zero-release frontier pushes per step (so a deep chain never trips a caller's 256-step stall bound), and
  pops on its terminal witness. The fixed-depth `Err` remains only for layers/strings, which admission bounds (`RASTER_MAXIMUM_NESTED_DEPTH`).
- **`release_string` pages from the tail** (memory note "Retirement Cursor Must Page Oversized Strings"): a string larger than the grant is truncated at a
  char boundary and shrunk to `max(keep, capacity − grant)`, `released_bytes ≤ grant`, instead of `Pending {0, 0}` forever.
- Editor law `window_measures_expose_brush_and_eraser_option_groups` asserted 2 groups; the composite has 3 since the bucket (S3.8) → renamed
  `window_measures_expose_brush_eraser_and_bucket_option_groups`, asserts all three.

### S4.6 Resume 11:35 (cut ~08:55) — owned-crate lib reds (rules 41/42/43: checks only)

Batched `cargo check --manifest-path ✏️s/Cargo.toml -p <raster, remodel, wfc ×5, process3d> --lib --tests --keep-going` (`check-all-1`, 08:16–08:43, before
rule 43): raster + process3d lib clean; process3d `--tests` 2 errors; remodel lib 137; wfc 2d/3d/grid2d/grid3d lib 13/11/15/7; all wfc `--tests`
~140–210 each (`dsl::json` 712×, peer facade removal). Since then a peer converted wfc 2d/3d/grid2d/bitmap (`dsl::json`, Cargo deps, io leaves;
mtimes 09:23–11:21) — left alone. My conversions (source, compile pending):
- `🧪️s4-strokes-dsl-json-to-pack-json.py` (balanced-call rewrite; `from_json_str`/`parse` gain `JsonMemberPolicy::Reject`): remodel 26 sites / 8 files,
  wfc grid3d 202 sites / 21 files (lib: preview window, window, scene-internals `use … as json`; tests: 14 mutation quintet tests + unit tests).
- `🧪️s4-strokes-ioerror-literals.py`: `IoError { message, diagnostics }` → `IoError::from_value_error(ValueError::new(InvalidValue, …))`, remodel 39 sites / 13 io leaves.
- By hand (remodel): `JsonValue` → `semio_framework_pack_json::Value` (report 36, frames 17); crate-root `Float32Buffer` / `ByteBuffer` `DslField`
  controlled codecs → `ValueError` (typed `InvalidValue` refusals, no `to_string` bridges); inference `protocol::ValueError` (now private) →
  `semio_framework_value::ValueError`; the PNG raster codec moves from the stdio document type (`PngSnapshot` is now `{schema, bytes}`) to the
  framework's `semio_framework_pixels::{decode_png, encode_png}` (domain-neutral, already a dependency); `scene_from_png_bytes` reads the extent via
  `png_layout_bytes` (no pixel decode).

### S4.7 Rule 44 cargo freeze (12:2x) — static fixes, TS runs, failure routing

The 11:5x batched lib check never ran (gate congested 30 min, then the background limit stopped it in its gate; no orphan cargo). Rule 44: no cargo
until "CARGO OPEN"; everything below is source-only, **checks OWED**.
- **wfc (static scan for the peer-migration break patterns, census 08:56 vs current mtimes)**: grid2d + grid3d `Grid*SnapshotRetirement::close_step`
  `String` → `ValueError` (bodies have no `Err`); grid3d `Grid3dTileMedia` `DslField` controlled codecs → `ValueError` (typed `InvalidValue`, no
  `.message` bridge); wfc 2d / 3d fill tools `dsl::DslValue` (now private) → `semio_framework_value::DslValue`; bitmap editor
  `BitmapOneItemPreparation::close_step` (`ArtifactStoreOneItemPreparation` now returns `ValueError`) → `ValueError` + `InvariantViolated` base-root
  refusal (Edit on a hot file: S4-RUNTIME converted its transient root at 12:18; region untouched). The other census reds (missing
  `semio-framework-pack-json` deps, wfc 2d/3d/grid2d/bitmap `dsl::json`, io leaves) were fixed by a peer 09:23–11:21. No `IoError { message }`,
  `protocol::ValueError`, non-`Result` `interaction_topology`, stale `ArtifactCommandWork::step` signatures or `dsl::json` remain in my four trees.
- **process3d `--tests`** (2 errors at 08:43): unit-test matches gain `ArtifactCommandWorkStep::CompleteDownload` and `MediaPayload::Intrinsic` arms.
- **Raster TS package** `bun ./📜️script.ts test` (cwd raster TS package): **131 pass / 21 fail**, all 21 = `ValueError: binary64/binary32 requires an
  unsigned … word` from `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts` `parseBinary64` called by raster's TS twin
  `🧬️schema/🟦️.ts` `parseRasterTransform`/opacity (sqlite-snapshot campaign edit 10-02 22:45 made the twin demand exact `{bits: bigint}` words while the
  artifact JSON schema and every fixture carry plain numbers). Same pattern in ~10 plugin twins (gen2d/3d, gisterrain, program, layout, trinity ×2,
  drawing, stl). D5 rows themselves pass. Routed to `main` (campaign owner); not mine to redesign.
- **The 21 Rust failures of `test-raster-3`** (messages lost to the abort, so by evidence only):
  4 quintet-print laws — expected until `--ignored emit_committed_fixtures` (OWED, STROKES); 4 binary `raster_store_initializer_*` /
  `…mounted_sixty_four_fuel…` + the mounted-app laws that load a document through the store initializer (`composite_scene_syncs_document_and_assets`,
  `two_instances_converge_…`, `history_edit_inputs_resolve`, both `media_export` laws, `merge_down …asset_capacity…`, `fill_tool_transactions …`, both
  `stroke_stream_transactions`, `paint_stroke … another_press …`) — compiled 07:36 while S4-STORE was mid-migration of the raster store initializer
  (supersession folds, coordinator 07:1x–07:4x) → re-run first, owner S4-STORE if they persist; `window_measures_…` — FIXED (S4.5);
  `semio_payload_law_raster_mutation`, `snapshot::binary::pack_round_trips_representative_document` — unknown until re-run (STROKES).
- **OWED (rule 43/44)**, exact commands:
  `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster -p semio-s-artifact-remodel-remodeling -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-grid3d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-process-process3d --lib --keep-going --message-format=short`;
  `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-raster -p semio-hub-remodel -p semio-hub-wfc -p semio-hub-process --target wasm32-wasip2 --lib`;
  then (TESTS RESUMED) `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-strokes cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-raster-raster --lib --no-fail-fast`,
  `… -p semio-s-artifact-raster-raster --lib -- --ignored emit_committed_fixtures`, the 4 raster leaf TS twins, `schema mutation-payloads --under ✏️s/🔌️plugins/🖨️raster`,
  remodel / wfc ×5 / process3d `--lib` tests (process3d `-- example`), `RUST_MIN_STACK=67108864 … -p semio-framework-os-renderer-wgpu --lib -- paint2d`.
- Coordinator routing (INFRA census, `🧪️s4-infra-close-step-abi.py` pattern): grid2d/grid3d binary retirements + bitmap preparation done (above).
  process3d `💾️binary/🦀️.rs:3189` is an inherent `Process3dSnapshotCopyCursor::close_step` (not the trait ABI; census 0 process3d errors) whose only
  caller `pump_terminal_retirement` (`String`) sits in `RetainedStoreInitialization`, S4-STORE's frozen region → converted together once released.
- process3d E0061 "`✏️editor/🦀️.rs:312`" (S3 owed): no longer present — line 312 is now `Process3dPlayApp`'s context menu, and process3d `--lib`
  compiled clean in `check-all-1` (08:43). Closed.
- `verify taxonomy report --scope …/🖨️raster/…/✏️editor`: 31 × `directory-kind-unresolved` (the pre-existing raster command/panel dir class), none for the
  deleted `🎨️edit-pixels`/`🖌️edit-mask`/`🖼️assets` or the `🫧️transient` dir.

### S4.8 Raster TS twin consumers after S4-AGNOSTIC's `Binary64Transport` readers (12:55–13:15)

After the shared transport readers landed (raster package 132/20), the twin's native representation (binary64/binary32 words, intrinsic params — what the
sqlite TS twin consumes) stays; consumers now read through an explicit JSON projection in `🧬️schema/🟦️.ts` (region `🔖️JsonProjection`):
`rasterTransformNumbers` (derived numbers for compositing math and the wire), `rasterBinary32Number` (shortest decimal that rounds back to the same binary32 —
the native printer's `0.65`, not `0.6499999761581421`), `parseRasterParameter` / `printRasterParameter` (adjustment params: JSON `DslValue` projection ↔ owned
`IntrinsicValue`, iterative; the JSON params wire is plain JSON, so `parseIntrinsicValue`'s tagged form was the wrong reader), `printRasterLayerMask`,
`printRasterLayerNode` (inverse of the parsers). Mask extents now follow the schema (`width`/`height` 1..16384, `imageKey` minLength 1 — the old 0..u32 bound
"passed" only because the transform parse threw first).
- Consumers converted: `📐️change-layer-transform/🟦️.ts` and `🔺️diff/🟦️.ts` (`inverse` over `rasterTransformNumbers`); tests `📐️change-layer-transform/🧪️tests`,
  `🧪️tests/🎭️mask`, `🧪️tests/🔒️protection`, `✏️editor/💾️document/🧪️tests` compare through the projection; `📸️snapshot/🧪️tests/🪶️sqlite` scalar-word law reads only
  the word fields (`parseBinary32Transport`, `parseRasterTransform`) instead of re-validating its deliberately schema-extreme snapshot (u32 extents) as a document.
- **Raster TS package `bun ./📜️script.ts test` → 152/152** (was 131/21 → 132/20); `bun test ./…/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` → **9/9**.
- Note: pixel-layer `width`/`height` in the twin still admit 0..u32 (native domain) — the schema's pixel bounds were not part of this ask.

### S4.9 Hand-over state (13:10, rule 44 cargo freeze still in force)

| Item | State |
|---|---|
| D5 delete `editPixels`/`editMask` (+ dead `🖼️assets/🔄️replacement`, stale `📌️.empty.md`) | DONE in source; raster `--lib --tests` check **0 errors** (04:27); zero-reference proof clean except generated descriptors |
| D24 remodel `Binary64Transport` ref | DONE; remodel Ajv TS 21/21; remodel gates 0/0 |
| Raster value-refusal migration (128 errors) | DONE; check green (04:27) |
| Raster deep-value retirement (SIGABRT in Drop) | DONE in source (§S4.5); check + test OWED |
| Remodel 137 lib reds, wfc residue, process3d test matches, close_step ABI (grid2d/grid3d/bitmap) | DONE in source (§S4.6–S4.7); check OWED; process3d inherent `close_step` → S4-STORE (F3) |
| Raster TS twin consumers / mask extents | DONE: raster TS 152/152, sqlite TS 9/9 |
| Gates | remodel 136/136·0 / 56/56·0; wfc 85/85·0 / 153/153·0; process 15/15·0 / 25/25·0; raster 4 unwitnessed + 4 uncatalogued (owed: emit + central generate) |
| React `🖌️Paint2dHost` 63/63; editing twin 62/62; `semio-framework-ui --features wgpu -- paint2d scene_records` 1/1 | VERIFIED |
| renderer-wgpu `-- paint2d`, raster/remodel/wfc/process3d lib tests, `--ignored emit_committed_fixtures`, hub wasip2 ×4 | OWED (rules 43/44) — commands in §S4.7 |
| K3 window-transient derive | S4-RUNTIME (§21.5) |
| K4 wgpu bucket + fill-region replay law, K1 remodel per-tick decode | source CLOSED (S3); runs OWED |

**Coordinator actions:** (1) when CARGO OPEN: I run the batched native check + hub wasip2 for raster/remodel/wfc/process (then "COMPOSITION GREEN …");
(2) TESTS RESUMED: raster lib tests (re-run the 21 + aborting law), `--ignored emit_committed_fixtures` → then central `schema generate` (raster leaves
`fill-region`, `apply-filter`, `transform-image`, `fill-selection`; remodel `change-stream-sync` hash); (3) describe + re-activate raster (verbs removed:
`editPixels`, `editMask`; new: `setFillTolerance`, `applyFilter`, `transformImage`, `fillSelection`, `paintStroke{phase,reason,gesture}`, utility `paintBucket`,
Composite window transient owner), remodel (schemas, fault code `app.command.tool-mismatch`), wfc ×5 (labels, rows, tool-mismatch code), process3d; stale
generated descriptors `🌎️hub/🧩️compositions/🖨️raster/{🔣️.json,🛂️.descriptor.semio}` and the dev copy `OSM/🧑‍💻dev/🔌️plugin-modules/🖨️raster/*`.

### S4.10 AUDIT-TOOLS items (`📓️audit-s4-tools.md` F5/F9/F15/F21) — PARKED mid-way (coordinator usage limit), source-only, all checks OWED (rule 44)

**F5 — DONE in source (uncompiled):** both plugin-local streamed-gesture runners now ride the shared `GestureTool` / `drive_gesture`
(`TM/🦀️.rs`); the local phase enums and drive bodies are deleted (0 references to `RasterStrokePhase` / `BitmapStrokePhase` repo-wide).
- Raster `🖌️paint-stroke/🦀️.rs`: `impl GestureTool for RasterPaintTool` (Gesture = `RasterStrokeToolState`, Tick = `PaintToolRequest`);
  the stroke's gesture identity is its PRESS (`verb()` = press id, so a tick of another press interrupts like another verb, `captureLost`);
  `base_revision()` = `""` (the leaf names samples + brush, repaints on any base); inherent `RasterPaintTool::at_rest(tool, press, seed)` keeps
  `raster_tool_commit` (bucket uses its own tool id). `handle_in_window` keeps only the raster-specific window rules around the drive: the
  late-tick drop (`closed`), an abort for a press other than the open one (keeps the stroke), and building the opening leaf (named refusal)
  only when a new stroke opens. Behaviour change: an UNRESTORABLE open stroke now leaves zero trace on its next tick/commit (was: restarted
  with only that tick's samples) — matches the handler's own contract. Known shared gap (F21, S4-TOOLS-A): `drive_gesture` swallows a
  `ToolRefusal` from `send`, so a refused commit is a silent no-op (raster used to name it).
- wfc bitmap `🪛️utilities/🖌️brush/🦀️.rs`: `impl GestureTool for BitmapBrushTool` (verb `paint-stroke`, base `""`); `bitmap_brush_dispatch`
  is one `drive_gesture` call; editor (`✏️editor/🦀️.rs`) and the brush test use `GesturePhase`; the wire spelling of a phase is written at its
  two producers (host-event abort `"abort"`, pointer mapping) — `GesturePhase` has `parse` but no inverse; proposal for S4-TOOLS-A: add
  `GesturePhase::wire(self) -> (Option<&'static str>, Option<&'static str>)` to `TM` and use it there (not staged yet).
- Also edited: raster `✏️editor/🦀️.rs` host-event abort builds `phase: Some("abort")` (was the deleted enum's `as_str`).

**NOT STARTED (next steps, in order):**
1. F9: verify the raw codes the audit lists — wfc grid2d `✏️editor/🦀️.rs:718` + `👁️viewer/🦀️.rs:305`, bitmap `:856`, grid3d `:666`
   (`wfc.grid3d.retained.tool-mismatch`), process3d `:567`, `:1413` — and replace each `Fault::from("…tool-mismatch")` by
   `Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), …)` (my S4.3 conversion only covered wfc 2d/3d + remodel).
2. F9: `fault_notices()` (en + de, `ArtifactApp::fault_notices`, pattern: draw/fem tables) for raster (`raster-stroke-phase-invalid`,
   `raster-stroke-window-required`, `raster-stroke-gesture-required`, `raster-paint-*`, `raster-layer-*`, `raster-filter-pixels-only`,
   `raster-transform-*`), wfc ×5 (`wfc-bitmap-stroke-*`, `wfc-bitmap-unknown-palette-color`, …), remodel (`remodeling.import.open`,
   `remodeling.import.window-required`, …), process3d.
3. F15/F21: delete the `[DEBUG]` narration in raster tests (2: `📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` ×2) and remodel tests (4).
4. At CARGO OPEN: batched native check (§S4.7 command) — it now also proves the F5 refactor; then hub wasip2 ×4; at TESTS RESUMED: raster lib
   tests (incl. the stroke-stream laws, which exercise exactly this refactor), wfc-bitmap `🧪️brush-tool` tests.

### S4.11 CARGO OPEN (checks only) — 20:52–21:20

- Batched `cargo check --manifest-path ✏️s/Cargo.toml -p <raster, remodel, wfc ×5, process3d> --lib --tests --keep-going` (`check-all-2`, 20:52–21:15, 20 min
  lock wait behind a peer nextest build): **18 stdio crates red** from the new `semio-framework-pack-error` API (`PackError::Schema`/`Malformed`/`ValueRefusal`
  gone; avi, mp4, bmp, tiff, las, jpg, json, xml, gif, stl, ply, obj, dxf, step, dwg, pdf; later also `stdio-semio` 77× + `stdio-xml` `From<FromUtf8Error>`) —
  not mine (shared / S4-STDIO); they block raster, remodel, process3d and wfc 2d/3d/grid2d/grid3d. wfc-bitmap compiled and showed 3 errors of its own
  (`PackError::Schema` in its text codec).
- **Pack-error fallout in my trees, converted** (`🧪️s4-strokes-pack-error-api.py`, 88 sites / 25 files over raster, remodel, wfc ×5, process3d):
  `PackError::Schema(x)` → `PackError::from(ValueError::new(InvalidValue, x))`; `PackError::ValueRefusal` (fn) → `PackError::from`; the grid2d json
  importer's `PackError::TextRefusal(e)` pattern → `PackError::Refusal(PackRefusal::TextRefusal(e))`.
- **F15 done:** `🧪️s4-strokes-strip-debug-prints.py` removed every `[DEBUG]` print in my trees: raster sqlite owner 2 (production code, added 18:27 by a peer
  probe), raster sqlite tests 9 (2 audit + 7 peer probe), remodel video tests 3, remodel sqlite owned-test script 1 → `git grep '\[DEBUG\]'` over raster/remodel/
  wfc/process = **0**.
- **F9 done in source:** the 6 audit lines + raster's own now use the framework code: wfc grid2d editor + viewer, bitmap, grid3d, process3d ×2 →
  `Fault::new(App, "app.command.tool-mismatch", …)` (process3d's TS source-scan updated); raster splits its retained guard into `app.command.tool-mismatch`
  and `raster.document.too-large`. Named codes + en/de `fault_notices()`:
  - raster: `raster_fault(code)` + `raster_fault_notices()` (16 codes) in `🖌️paint-stroke`, wired on `RasterPlayApp`; the stroke/paint/brush/fill/filter/transform
    refusals moved from anonymous `Fault::from("raster-…")` (= code `app.message`, so no notice could ever resolve) to `raster.<area>.<name>`; the layer
    family (`raster-layer-*`, shared with TS twins + fixtures) is left for the D17 sweep;
  - wfc: `🧪️s4-strokes-fault-notices.py` renamed 26 anonymous sites to `wfc.{bitmap,grid2d,grid3d}.<area>.<name>` and added tables on all five editors
    (bitmap 13, grid2d 3, grid3d 3, wfc2d 8, wfc3d 4); remodel 5 (`remodeling.import.open`, `…import.window-required`, `…stream.unknown-camera`,
    `…qc-report.missing`, `…retained.extent`); process3d 2 (`process3d.action.invalid`, `process3d.media.export`).
- `cargo check … -p semio-s-artifact-wfc-bitmap -p …-wfc-2d -p …-wfc-3d -p …-wfc-grid3d --lib --tests --keep-going` (`check-wfc-3`, 21:18–21:20):
  **wfc-bitmap lib + lib test GREEN** (proves the F5 `GestureTool` brush, its notices and pack-error conversion); wfc 2d/3d/grid3d stopped at
  `stdio-semio` (peer, 77 errors). Raster/remodel/process3d/grid2d + hub wasip2 ×4 **OWED** until stdio is green (`main` told).
- **Aligned with the canonical S4-PACKFIX mapping** (`📓️s4-packfix-report.md` § Schema Decisions; `🧪️s4-strokes-pack-error-canonical.py`, 62 sites /
  24 files): envelope build/unwrap (`SemioError`) → `PackError::from(e.into_value_error())` (48), UTF-8 → `PackError::from(ValueError::from(e))` (7),
  typed JSON/value decode (`ValueError`) → `PackError::from(e)` (7); the 24 `"pack envelope mismatch …"` texts stay `InvalidValue` (canonical), raster's
  base64 asset decode stays `ValueError::new(InvalidValue, …)` (canonical "base64 input errors"). 0 stringified refusals remain in my trees.
- State: waiting for the coordinator's "STDIO GREEN" (S4-PACKFIX owns stdio, stdio-semio first) → then, one gated invocation each: the batched native
  `--lib --tests` check of the 8 crates, then `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-raster -p semio-hub-remodel -p semio-hub-wfc -p semio-hub-process --target wasm32-wasip2 --lib`,
  then "COMPOSITION GREEN <plugin>" per plugin. All raster/remodel/wfc 2d·3d·grid2d·grid3d/process3d edits since 04:27 (F5, F9, F15, pack-error, deep
  retirement, value refusals in remodel/wfc) are WRITTEN BUT UNVERIFIED until then (blocked by stdio).

### S4.12 STDIO GREEN (21:52) → compositions green (21:54–22:18)

| Check (gated, one at a time) | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p <raster, remodel, wfc-2d, wfc-3d, wfc-grid2d, wfc-grid3d, wfc-bitmap, process3d> --lib --tests --keep-going` (`check-all-3`, 21:55–22:04) | raster, remodel, wfc-2d, wfc-grid3d, wfc-bitmap **0 errors**; 3 reds → fixed below |
| re-check `-p wfc-grid2d -p wfc-3d -p process3d --lib --tests --keep-going` (`check-fix-4`, 22:06–22:11) | **exit 0, 0 errors** |
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-raster -p semio-hub-remodel -p semio-hub-wfc -p semio-hub-process --target wasm32-wasip2 --lib --keep-going` (`check-hub-wasip2-1`, 22:11–22:18) | **exit 0, 0 errors** (all 8 artifact crates + 4 hub crates checked; warnings raster 22, remodel 43, wfc-2d 28, wfc-3d 37, bitmap 12, grid2d 25, grid3d 19, process3d 19) |

Fixes: process3d `💾️binary/🦀️.rs` mutation decode `ByteReader` errors are now `PackRefusal` → `.map_err(protocol::ProtocolError::from)` ×10 (was `::Pack`,
which takes `PackError`); wfc-grid2d json importer projects `into_value_error()`'s `Result` (transport → `InvariantViolated`, the canonical io pattern); wfc-3d
snapshot unit test (peer, 21:28) `include_str!("../../🧫️fixtures/🪆️mesh-child/🔣️.json")` ×2 pointed one level too high — the fixture the peer created (21:20)
lives in `🧪️tests/🧫️fixtures/`, so the path is now `../🧫️fixtures/…`.
**COMPOSITION GREEN raster, remodel, wfc, process** sent to `main` (22:18). This also verifies, natively and for wasip2, every source edit since 04:27: F5
`GestureTool` (raster + bitmap), F9 notices / named codes, F15, pack-error conversion, raster deep-value retirement, remodel/wfc value-refusal fixes.
Still OWED (rule 43, tests): raster/remodel/wfc/process3d lib tests, `--ignored emit_committed_fixtures`, renderer-wgpu `-- paint2d`.
