# 📓️ W3-T-SPATIAL Report: Gumball, Transform and Brush Gestures as Tool Machines

Executor W3-T-SPATIAL. Scope: 🎥️shooting, 🏗️fem 2d/3d, 💠️lowpoly, plus the World3d and Canvas2d gumball hosts and the
manifest's gumball bracket rule. Layout moved to W3-T-LAYOUT and generation3d to W3-T2-PROCEDURAL; for both, the
host protocol below stays stable.

Status: IN PROGRESS. This file is rewritten as each slice lands.

## 1. Census (before)

| # | Plugin | Gesture | Entry verbs | How it committed | Verdict |
|---|---|---|---|---|---|
| G1 | shooting | asset gumball drag / rotate / scale | `dragAssets`, `rotateAssets`, `scaleAssets` (+ selection fallback) | one `Emit::amend` per tick under a coalesce key; the host sent `transformBegin`/`transformEnd` around it | tool machine, ONE transaction holding the net relative leaf |
| G2 | fem 2d | Canvas2d gumball translate / rotate / scale | `translateSelection`, `rotateSelection`, `scaleSelection` | absolute whole-record `ReplaceNode`/`ReplaceRegion` per tick, coalesced; region holes never moved | new relative `move-selection` leaf + tool machine |
| G3 | fem 3d | World3d live gumball | same three verbs + empty `transformBegin`/`transformEnd` handlers | absolute `ReplaceNode`/`ReplaceSolid` per tick under `gumball-*` coalesce keys | new relative `move-selection` leaf + tool machine; brackets deleted |
| G4 | lowpoly | World3d gumball | `transformBegin`, the three verbs, `transformEnd` | `LowpolyScratch` transform scratch, then one absolute `CreateMesh` (whole half-edge JSON) via `Emit::commit` | relative mesh transform leaf + tool machine; scratch and brackets deleted |
| G5 | lowpoly | paint stroke (World3d + UV canvas) | `paintStrokeBegin`, `paintAt`/`paintStroke`/`canvasPointerDown`/`canvasPointerMove`, `paintStrokeEnd` | stroke pixel scratch in the transient, then `paintStrokeEnd` diffs whole buffers over a bounded cursor into absolute `EditPaintLayer` runs | relative paint-stroke leaf (points and brush) + tool machine; scratch and brackets deleted |
| H1 | World3dHost | gumball drag | `transformBegin`, pose deltas, `transformEnd` | brackets around per-tick deltas; non-live hosts previewed locally and sent one delta | non-live: one one-shot delta (no `phase`); live (`gumballLiveDispatch`): `phase` stream/commit/abort |
| H2 | Canvas2dGumballOverlay | 2d gumball drag | per-tick deltas | per-tick one-shot deltas | `phase` stream/commit/abort |
| H3 | wgpu World3d | gumball drag | one delta on release | already one one-shot | unchanged (one one-shot transaction) |

## 2. Host protocol (shared by every executor)

- A dispatch without `phase` is a one-shot: ONE tool transaction.
- `phase: "stream"` adds a tick to the window's open transaction (it opens on the first tick).
- `phase: "commit"` folds the last tick in and commits the net leaf. An empty tail commits with an identity delta
  (`gumballIdentityDelta`).
- `phase: "abort"` plus a `reason` drops the open gesture with zero trace. Reasons: `blur`, `captureLost`,
  `baseMoved`, `frozen`, `retired`; an absent reason means `tool`.
- World3dHost aborts on window blur and unmount (`captureLost`) once a tick was streamed. The Canvas2d overlay aborts
  `captureLost` on pointercancel / lostpointercapture / unmount and `blur` on window blur.
- The `transformBegin`/`transformEnd` brackets are gone from World3dHost, fem 3d and puzzle 3d (puzzle via
  W3-T-PUZZLE). Lowpoly is next.

## 3. Design deviation: FEM gesture state is an artifact transient keyed by window

The FEM editors keep the open gumball gesture in an artifact-level, local-only transient
(`FemGumballTransient { gestures: window id → FemGumballGesture }`, defined once in fem 2d and reused by fem 3d) rather
than a window transient. The SIBLING window paints the preview too: the results window re-solves the moved structure
while the drag goes on. Each gesture is keyed by its owning window id, so a host abort clears only the owner's entry,
and two windows never clobber each other. Never history, never shared.

## 4. Leaf naming

`transform` is not an approved semantic verb, so the relative FEM leaves are `move-selection` (verb `move`): one leaf
carries offset, angle (+ axis in 3d) and factors about a pivot, and the label names the motion actually present
("Move 1 node by (0.5, 0)", "Rotate 2 nodes by 90°", "Scale 1 solid by (2, 1, 1)", en + de).

## Session 2 — 2026-10-01

Executor S2-SPATIAL (Opus successor). Status: IN PROGRESS; this section is updated at every milestone.

### S2.1 Repairs (fem 2d/3d, shooting)

- fem 2d + fem 3d mutation catalogue unit tests: stale `kinds.len() == 29` → `30` (the `move-selection` leaf).
- shooting editor: leftover `[DEBUG] shooting retained reduce` `eprintln!` removed from `shooting_bounded_reduce`.
- Store-level replay laws appended to the gumball command tests of fem 2d, fem 3d and shooting
  (`a_gumball_drag_edited_in_history_replays_its_downstream` / `…move_edited…`): a gumball edit is committed, a
  downstream edit is stacked on it, the gumball edit's input is replaced in history (`InputReplacement::Input`), the
  report replay re-applies the downstream edit on the new base and `commit_finished_replay(…, Overwrite)` lands it.

### S2.2 Lowpoly conversion (design §17.6)

- Three new relative leaves in `💠️lowpoly/…/🧬️schema/🧬️mutations/`: `🚚️move-selection` (`offset`),
  `🌀️rotate-selection` (`pivot`, `axis`, `angle`; `x-semio-invariant: axis-nonzero`) and `🔍️scale-selection`
  (`pivot`, `factor`, every component > 0). Each names `objectId` + `vertexIds` (unique), has full `x-semio-ui`
  (mesh-domain references, vector widgets, a degree dial over radians), a descriptor (binary tags 19/20/21), a diff, an
  inverse (one `CreateMesh` restoring the prior handle + content) and en/de labels ("Move \"obj-1\" by (0.5, 0, 0)" /
  "\"obj-1\" um (0,5; 0; 0) verschieben"). Shared kernel: `LowpolySelectionMotion` + `lowpoly_selection_motion_diff`
  in the aggregate. Outcomes: invalid axis/factor = Fatal (invariant), missing object/mesh/every vertex = Error
  (target missing), undecodable content = Fatal (`mutation.apply.invalid-base`), partial vertex set and identity motion
  = Warning.
- Fixtures (16 cases: moves, pins, part, still, ghost, bare, twice; turns, lifts, still, void, ghost; wides, part,
  still, flat) generated by `T/🧪️s2-spatial-lowpoly-selection.py` from a numpy + libm oracle that reproduces the
  half-edge JSON (shortest-f32 digits, Newell normals). A standalone checker (`G/s2-spatial/mesh-check`) ran every
  moving case through the Rust `HalfedgeMesh` kernel: `failures: 0`.
- Gumball: `lowpoly_gumball_leaves` maps a component selection to one leaf with vertex ids and an object selection to
  one leaf per object about the common centroid; one tool transaction per gesture
  (`s.lowpoly.lowpoly@1/*#editor#translateSelection`). `transformBegin`/`transformEnd` and the transform scratch are
  deleted.
- Paint: `paintAt` / `paintStroke` / `canvasPointerDown` / `canvasPointerMove` + new `canvasPointerUp` accept
  `phase`/`reason`; a stroke streams `apply-paint-stroke` dabs into ONE transaction of the persisted
  `lowpoly_tool` statechart (tool `#paint`), kept in `LowpolyTransient.paint` keyed by window id (same deviation as
  FEM, §3). Abort/blur/captureLost = zero trace; baseMoved aborts. The preview is the transient's
  `paint_preview(document)`. `paintStrokeBegin`/`paintStrokeEnd`, `LowpolyScratch` stroke/transform sessions and the
  bounded paint-chunk cursor are deleted; paint verbs are `ActionKind::Mutation`. Interactive-job partition: 48
  routes, 48 Migrated, 0 BatchOnlyPendingRewrite.

### S2.3 Bracket rule deletion

- `🛂️manifest/🦀️.rs`: `GUMBALL_GESTURE_BRACKET_ACTION_IDS` deleted; only the chrome audience rule remains.
  `🧫️fixtures/🖐️gumball-verb-audience.json` rewritten (8 cases; former bracket ids are ordinary ids) and the test
  doc/assertion updated. The fixture's claimed AJV twin never existed and the claim is removed.
- World3dHost: `paintStrokeBegin`/`paintStrokeEnd` dispatches and `paintStrokeActive` state deleted; new exported
  pure reducer `worldPaintStep` (press/drag → `phase:"stream"`, release → `commit`, cancel/blur/unmount → `abort`,
  a click after a committed drag is consumed, a lone click is one one-shot).
- wgpu World3d: `PaintBegin`/`PaintEnd` plan kinds deleted; press in paint mode starts a paint pick, each dab
  publishes `paintAt{phase:"stream"}`, release publishes `paintAt{phase:"commit"}` (`plan_world3d_paint_release`).
- `🔌️plugin/🦀️.rs` `ArtifactApp` contract doc: the "two blessed preview patterns" + `paintStrokeBegin/End` paragraph now
  states the tool-machine gesture contract (doc-only edit; `Emit::commit`'s doc no longer cites lowpoly strokes).
- Remaining mentions (02:50 sweep over `🧰️framework`, `✏️s`, `🌎️hub`): the hub composition descriptors
  (`🌎️hub/🧩️compositions/{demonstrator,fem,lowpoly,puzzle}/{🔣️.json,🛂️.descriptor.semio}`, regenerated by
  `describe`); the manifest fixture's two regression cases that pin the retired ids as ordinary; a puzzle3d test
  (S2-PUZZLE) asserting the brackets are absent; one incident narrative in a `🔌️plugin/🦀️.rs` test doc.

### S2.4 Verification (so far)

| Command | Result |
|---|---|
| `cargo run` in `G/s2-spatial/mesh-check` (Rust kernel vs. python oracle, every lowpoly selection fixture) | PASS, failures 0 |
| `bun ./📜️script.ts test` in lowpoly `📦️packages/🟦️typescript` | PASS 2/2 (48 Migrated, Ajv hostile oracle ok) |
| `schema mutation-inputs --under ✏️s/🔌️plugins/💠️lowpoly` | 40/40 inputs; 4 `leafUncatalogued` (3 new leaves + apply-paint-stroke) → central `schema generate` |
| `schema mutation-payloads --under ✏️s/🔌️plugins/💠️lowpoly` | new leaves witnessed, 5 negatives correct; 4 findings from the peer path-budget rename of change-paint-layer |
| `cargo test -p semio-s-artifact-fem-2d --features component-app-assembly --lib` | BLOCKED (14:02): peer crate `semio-s-artifact-stdio-gltf` failed with 210 errors |
| lowpoly `cargo check` | BLOCKED (13:44): same stdio-gltf breakage |
| `cargo test -p semio-s-artifact-lowpoly-lowpoly --lib` | 17:25–21:37 sat in a fine-grain-lock deadlock (nine idle cargos in `prebuild_lock_exclusive` for 4 h, mine among them; all nine killed per pid); 22:17 BLOCKED: peer crate `semio-s-artifact-stdio-obj` mid-edit (35 errors, u64/usize carrier change); 03:04 BLOCKED: `semio-framework-os-kernel` red (28 errors, peer `RecordSpecProducer` dsl refactor) |
| `cargo test -p semio-framework --lib -- gumball_verb_audience` | 22:49 PASS 1/1 (`gumball_verb_audience_matches_the_language_agnostic_fixture`) |
| `cargo test -p semio-framework-os-infinite --lib -- paint gumball` | 23:02: every `world::` test passes (15/15, incl. `world_drag_and_paint_plans_reserve_before_exact_mutation`); 4 failures are all in `board::ports::directed_dag` (peer: the bundled DAG demo DSL no longer parses, `expected LBrace, found Ident 'id'`) |
| `bun ./📜️script.ts verify mutation-outcome-law` (repo root) | 17:01 PASS (was 3 lowpoly breaches: each selection `🔺️diff` now refuses its own payload invariant with `mutation.invariant`; the shared kernel keeps target/partial/no-op) |
| `bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts -t "worldPaintStep\|gumball"` in the React target package | PASS 13/13 (682 skipped) |
| `bun ./📜️script.ts typecheck` in the React target package | 4 errors, none in World3dHost / the React target barrel / engine-contract (peer files: `🏪️store/👷️worker`, `🐚️Shell`, `🔄️sync` backbone-parity test) |

WRITTEN BUT UNVERIFIED (reason: stdio-gltf did not compile): every Rust change of S2.1–S2.3.

### S2.5 World3dHost live-consumer API (for S2-PROCEDURAL)

Selection record (`WorldSelectionRecord`, published by the guest's world surface): `ids` / `gumballSelectionIds`,
`selectionMode` or `granularity`, `transformMode` (`move` | `rotate` | `scale` | `transform`), `gumballActive`,
`gumballTarget` (pivot), optional `gumballConfig` (handle flags; overrides `gumballConfigForTransformMode`) and
`gumballLiveDispatch`.

- Without `gumballLiveDispatch`, React previews locally and dispatches ONE net delta on release, no `phase`
  (a one-shot transaction): `translateSelection{mode, ids, dx, dy, dz}`, `rotateSelection{mode, ids, ax, ay, az, angle}`
  (angle in radians) or `scaleSelection{mode, ids, sx, sy, sz}`. A drag that moved nothing dispatches nothing.
- With `gumballLiveDispatch: true`, every pointer move enqueues the delta from the last enqueued pose to the newest one
  as `phase: "stream"`. At most one is in flight, and the newest pose wins. The host draws the app's answer, not a local
  preview. On release the remaining tail is sent as `phase: "commit"`. A tail that moved nothing sends
  `gumballIdentityDelta(...)` (`dx=dy=dz=0` / `angle=0` about `(0,0,1)` / `sx=sy=sz=1`). Window blur sends
  `phase: "abort", reason: "blur"` and unmount sends `reason: "captureLost"`, both with the identity args of the
  gesture's verb, and only if a tick was streamed. The ids are pinned at drag start.
- Paint (`interactionMode: "paint"`): `paintAt{objectId, u, v, phase: "stream"}` per dab while held, then
  `paintAt{phase: "commit"}` on release. Cancel sends `paintAt{phase: "abort", reason}`. A lone click is
  `paintAt{objectId, u, v}` with no phase. The pure reducer is `worldPaintStep(gesture, event)`, exported with
  `WORLD_PAINT_IDLE`.
- wgpu World3d always sends ONE net delta on release (no phase), with
  `{surfaceId, windowId, mode, ids, dx|ax..angle|sx..}`. wgpu paint streams `paintAt{surfaceId, objectId, u, v,
  phase:"stream"}` and commits with `paintAt{surfaceId, phase:"commit"}`.
- What the guest must do: parse `phase` and `reason` (`LowpolyToolPhase::parse` / `ToolAbortReason::parse`). Keep the
  persisted `ToolMachineRunner` keyed by the dispatching window id in an artifact-level local-only transient. Compose
  each stream tick into ONE net relative leaf, and on commit yield ONE `Emit::commit_transaction`. Preview from the
  transient. On abort leave zero trace. Abort with `baseMoved` when the document revision moved under the open
  gesture, and with `captureLost` when a one-shot interrupts an open gesture. References:
  `fem_gumball_drive` / `FemGumballTransient` (fem 2d), `lowpoly_paint_drive` / `LowpolyTransient.paint` (lowpoly
  session), and the puzzle 2d select tool.

### S2.6 Gaps

- wgpu World3d gumball never streams: it sends one net delta. The committed transaction is identical to React's,
  but wgpu has no live guest preview.
- wgpu paint has no blur abort. React has one.
- The "use selection" input for the integer `vertexIds` of the lowpoly selection leaves is not mapped. The schema
  references the mesh vertex domain.
- `FemGumballTransient` and the lowpoly paint drive duplicate the per-window runner bookkeeping. That bookkeeping
  belongs in `🛠️tool-machine` (follow-up).
- Shooting lints: 12 input and 5 payload findings. All come from the peer REPO-PATH-BUDGET rename. Four sun leaves
  now share the slug `change-scene-sun`. The `🎚️config` fixture directories were shortened but their leaf
  directories were not. Neither was introduced by this WP.

### S2.7 Coordinator actions

- Central `schema generate`. Five leaves are `leafUncatalogued`: lowpoly `move-/rotate-/scale-selection` +
  `apply-paint-stroke`, and fem 3d `move-selection`. Shooting config `set-shot-selection` / `set-center-model`
  come from the peer rename.
- `describe` for the hub compositions demonstrator, fem, lowpoly and puzzle. Their descriptors still list
  `transformBegin` / `transformEnd` / `paintStrokeBegin` / `paintStrokeEnd`.
- Activation of the fem, lowpoly and shooting lanes for a live check.

## Session 3 — 2026-10-02

Executor S3-SPATIAL (Opus successor). Status (10-03 11:05): source complete for S3.2 items 1–7 and audit S1/S2 (S3.6).
Law-green: React gumball laws, wgpu world laws (os-infinite), manifest audience law, lints, outcome law, lowpoly TS.
WRITTEN BUT UNVERIFIED: the four plugin crates' Rust tests and the plugin-crate Use-selection laws (reason: peer
stdio/plugin-test ValueError/IoError migration keeps every dependency red). Audit S3 (twin clocks) open. This section is updated at every milestone.

### S3.0 Repair-first diff (rule 28)

Files in the owned trees newer than the S2 report (03:04): only peer churn (the landed `RecordSpecProducer` dsl refactor:
`spec_fn.ordinary` call sites, sqlite snapshot schema, obj/ply serializers, lowpoly `📋️project.json`). No half-finished
S2-SPATIAL edit found on disk; the S2 work was compile-unverified, not half-written.

### S3.1 Verification

| Time | Command | Result |
|---|---|---|
| 11:00 | `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-shooting-shooting -p semio-s-artifact-lowpoly-lowpoly --features semio-s-artifact-fem-2d/component-app-assembly,semio-s-artifact-fem-3d/component-app-assembly --lib --tests --keep-going` | 11:12 FAIL only in peer `semio-s-artifact-stdio-xml` (`🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:234` `XmlDocumentView` mismatch, fixed on disk by its owner at 11:07) |
| 11:21 | same command (`check-2.txt`) | 11:46 PASS: all four crates, lib + tests, 0 errors. Warnings in owned trees: 2 unused imports fixed (fem 2d `🛠️options/🔄️transform` `set_gumball_flag`, fem 2d editor unit tests `PluginApp`); the rest are `unnecessary qualification` lints + one peer unused import in `🏗️fem/⚙️engine/🖥️app-surface` |
| 11:40 | `bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts -t "worldGumballStep\|worldPaintStep\|gumball"` (React target package) | PASS 13/13 (682 skipped); `-t worldGumballStep --reporter=verbose`: PASS 1/1 |

### S3.2 Changes (source, this session)

1. **World3dHost live-consumer API (S2.5) as a pure reducer + language-agnostic law.** `worldGumballStep(gesture, event)`
   (`WORLD_GUMBALL_IDLE`, types `WorldGumballGesture`/`WorldGumballEvent`/`WorldGumballTargets`/`WorldGumballDispatch`) in
   `🌐️World3dHost/🟦️.tsx` replaces the eight gumball refs + `dispatchGumballPoseDelta`/`pumpGumballLiveDispatch`/
   `abortGumballLiveGesture` callbacks; the component only steps it and chains the dispatch (a stream tick settles back as
   `settled`). Fix found by the law: the host `abort` used to send `{mode, ids, phase, reason}` without the verb's identity
   args (`dx/dy/dz` …), which a guest decoder that requires them refuses (fem 3d happens to default them to 0); it now sends the verb's identity delta.
   The live flag and transform mode are pinned at the grab. Fixture `🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`
   (10 cases, schema `🧬️schema/🔣️gumball-live-protocol/🔣️.json`): TS law `worldGumballStep answers every step …`
   (engine-contract, Ajv 2020 strict) replaces the old source-text scan of the B31 "no fabricated axis step" law;
   Rust guest law `the_world3d_gumball_live_protocol_lands_as_its_guest_edits` (fem 3d `🎮️commands/🧭️gumball/🧪️tests`) decodes
   every dispatch through `ArtifactApp::command_from_action` from the host's exact wire args and checks `guest.edits` +
   `guest.offset`. **S3-PROCEDURAL**: consume the same fixture for generation3d (swap `ids` for one movable target).
2. **Bracket rule leftovers**: `🛂️manifest/🧫️fixtures/🖐️gumball-verb-audience.json` no longer pins the retired
   `transformBegin`/`transformEnd` (two real verbs `rotateSelection`/`paintAt` instead; 8 cases); test doc/assert updated.
   Remaining mentions: stale generated descriptors only (coordinator `describe`), puzzle 3d's absence law (S3-PUZZLE, kept by
   census decision), one incident narrative in a `🔌️plugin/🦀️.rs` test doc (history, not a consumer).
3. **N12 lowpoly `apply-paint-stroke.color`**: now the framework colour vocabulary — sRGB `[r, g, b]` in `0..=1`
   (`[f32; 3]`; the old 4th byte was never rasterized: opacity governs alpha). Schema (number 0..1, 3 items, no `step`),
   leaf (`color_bytes()` rounds half away from zero; invariant refuses a channel outside `[0, 1]`), paint command (config
   bytes / 255), proto/graphql/grammar/binary protocol/TS twin, python oracle (`🧪️tests/💠️mutate-lowpoly-1/🐍️.py`: unit
   channels → bytes), six fixtures (`[200,30,30,255]` → `[0.78431374, 0.11764706, 0.11764706]`, exact byte round trip).
4. **Shooting closure (§20.1, coordinator-approved deletions, rule 32)**: `SetCamera` = ONE `Emit::config` per dispatch (both
   World3d hosts send it once per settled navigation gesture). The camera-label field is uncontrolled (`on_change: None`,
   `value: None`, id `shooting.camera-label`); `saveCamera{label}` takes the submitted text (both hosts pass `value` on
   submit; blank → `Camera N`) and publishes ONE artifact edit (no config lane). Deleted: `ShootingConfig.camera_draft_label`
   (Rust ×2, JSON schema, TS, proto, graphql), config leaf `SetCameraDraftLabel`, verb `setCameraDraftLabel` (registration,
   decode, tool id, publication contract, interactive-job row, describe), its oracle rows, contract vector, feature rows and
   adapter arm. Deleted paths: `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🏷️set-camera-draft-label/{🔣️.json,🦀️.rs,🧬️schema/🔣️.json}`,
   `…/✏️editor/🎚️config/🧫️fixtures/🏷️set-camera/✅️set/{🦠️mutation,🎯️outcome,🔺️diff,📸️snapshot/⬅️before,📸️snapshot/➡️after}/🔣️.json`.
   The `📸️replace` config vector (which only changed the draft label) now replaces selection, centre toggle and fit revision.
   Shooting has no `amend`/`amend_config` caller left.
5. **Lowpoly hand label (§20.4)**: `deleteSelection` at object granularity publishes `Emit::mutations` (row labelled from its
   `delete-object` leaves) instead of `Emit::commit(…, "Delete objects")`; the body comments moved into the docstring.
6. **N3 "Use selection" for composite domain rows**: new hook `ArtifactApp::selection_reference_id(kinds, row) -> Option<String>`
   (default: the row itself; mirrored on `ArtifactEditor`, forwarded by `EditorApp`) in `OS/🔌️plugin/🦀️.rs`, applied by
   `draft_time_travel_selection` (`OS/🔌️plugin/⏪️time-travel/🦀️.rs`) to every selected row before
   `time_travel_selection_value` (signature unchanged; S3-W2A's laws untouched). Lowpoly maps `lowpoly-document.<object>` →
   object id (`object` refs) and `…<object>.<granularity>.<n>` → `n` (refs of that granularity; integer `idType` is inferred
   from the schema's integer items). Laws: `mesh_rows_name_the_references_of_their_granularity` (unit) and
   `use_selection_retargets_a_vertex_move_onto_the_selected_vertices` (mounted: gumball move of vertex 0, select vertices 2 + 3,
   Begin → UseSelection `/vertexIds` → Accept → Finalize → overwrite = fresh fold). Chips: the generic N3 default (document
   `name`, else glossary kind word + short id) reads well for fem nodes/solids, lowpoly objects/components and shooting
   assets, so no `entity_label` override was added.
7. **Fem playback (§20.1, no amend on any lane)**: `PLAYBACK_COALESCE_KEY` is deleted in fem 2d AND fem 3d. Every playback
   publication is ONE plain window-config edit; a dragged phase/speed slider rides S3-CONTROLS' config-lane press (both
   hosts already send `gesture`/`commit`/`abort` on every slider Change: React `createContinuousGestureLane`, wgpu
   `🖱️ui/🎯️targets/🧊️wgpu/⚡️events` scrub binding), so its ticks stay provisional and the release is ONE edit. Fem 2d
   got fem 3d's window-transient playback CLOCK (ported, same laws): new
   `…/◻️2d/…/🪟️windows/📊️results/🫧️transient/` (state `Fem2dResultsWindowTransient{clock}`, leaf `set-playback-clock`,
   schema/TS/graphql/proto facets, fixtures, oracle with recomputed hashes, unit laws); `Fem2dResultsAnimation::advanced`
   moved to `Fem2dPlaybackClock::advanced`; `config::effective` draws the running clock; the tick carries `windowId`, frames
   land in the transient only, park/end are one config edit; editor: `WindowTransient` publication lane, retained
   `CompleteWithEphemeral` for both playback verbs, `register_window_transient_owners`, `retained_window_transient_target`,
   render reads the clock. Tick laws ported from fem 3d to fem 2d (`🎮️commands/⏱️result-animation-tick/🧪️tests`). New law
   (both fem crates) `playback_presses_are_one_config_edit_and_never_a_history_row`: play + pause = 2 edits, two slider
   ticks = 0, release = 1, a cancelled press = 0 (phase unchanged), no history row. Deleted (dead after the port): the
   `📌️.empty.md` placeholder of fem 2d's results `🫧️transient`.

### S3.3 Verification (continued)

| Time | Command | Result |
|---|---|---|
| 12:21 / 12:35 / 12:43 | `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly [-p semio-s-artifact-shooting-shooting] --features …component-app-assembly --lib --no-fail-fast` (private target `target-nde-s3-spatial`) | BLOCKED ×3 before reaching my crates: 12:30 SIGKILL (exit 137) while waiting for a peer's file lock; 12:35 peer `🧬️schema/📇️registry/🦀️.rs` duplicated definitions (28 errors); 12:58 peer `🛂️manifest/🦀️.rs:1266-1269` calls schema-registry exports the schema crate no longer re-exports. Both reported to the coordinator. |
| 13:00 | `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/<p>` (cwd `🦑️repo/🔨️modules/🧪️test`) | lowpoly 60/60, **0 findings** (N12 closed); shooting 66/66, 1 finding = the deleted `set-camera-draft-label` still in the central catalog; fem 109/109, 1 finding = new fem 2d `set-playback-clock` `leafUncatalogued`. Both need the coordinator's central `schema generate`. |
| 19:05 | `cargo check … (4 crates) --lib --tests --keep-going` (`check-4.txt`) | libs PASS; test targets: 3 errors in my trees fixed — fem 2d `🛠️options/🔄️transform` test lost `set_gumball_flag` with my import cleanup (now imported in the test); shooting/lowpoly tests used `protocol::{Terminology, Locale}`, now private after a peer change (→ `semio_framework_plugin::{Terminology, Locale}`); lowpoly `🧪️tests/🔬️unit` imported `framework_schema::parse_state_class_kebab`, moved by the schema split (→ `semio_framework_schema_state`) |
| 19:08 / 19:17 | same (`check-5/6.txt`) | BLOCKED by peers: os-kernel `OS/🗣️dsl/🦀️.rs:931` ambiguous `canonicalize` (fixed 19:16), then `OS/🔌️plugin/🦀️.rs:3015…39124` `dsl::LanguageSpec`/`preflight_languages`/`register_languages` gone (dsl refactor in flight; reported) |
| 19:18 | `bun ./📜️script.ts verify mutation-outcome-law` (repo root) | PASS |
| 19:19 | `schema mutation-payloads --under ✏️s/🔌️plugins/<p>` | lowpoly 39/39, 21/21 witnessed, **0**; shooting 100/100, 38/38, **0**; fem 250/250, 62/62, **0** |
| 19:20 | lowpoly `📦️packages/🟦️typescript` `bun ./📜️script.ts test` | PASS 2 + 6, 0 fail |
| 19:21 | `verify taxonomy report --scope …/🌐️World3dHost` | my new schema dir renamed `🧬️schema/🔣️gumball-live-protocol` (open pattern) → clean; 4 remaining findings are pre-existing (`🧭️gesture-targets`, `🔀️projection-pane` ×2, `🤏️multi-touch`) |
| 19:22 | `verify taxonomy report --scope` fem 2d results | new `🫧️transient` fixture dirs renamed to `🧫️fixtures/🧫️codec`, `🧫️clock` (also in fem 3d, tests updated); the remaining transient findings are `path-too-long` (> 240 B), the same class fem 3d's original has (REPO-PATH-BUDGET, §14 hand-off) |
| 19:23 | engine-contract `-t "worldGumballStep\|worldPaintStep\|gumball" --reporter=verbose` after the rename | PASS 13/13 |

### S3.4 Open items

- **Rust laws not yet run** (cargo blocked by peer breaks; see S3.3): fem 2d/3d/lowpoly/shooting `--lib` tests incl. the new
  laws (`the_world3d_gumball_live_protocol_lands_as_its_guest_edits`, `playback_presses_are_one_config_edit_and_never_a_history_row`
  ×2, the ported fem 2d tick + clock laws, `use_selection_retargets_a_vertex_move_onto_the_selected_vertices`,
  `mesh_rows_name_the_references_of_their_granularity`, `save_camera_stores_the_submitted_label`), the manifest
  `gumball_verb_audience` law, and the `wasm32-wasip2` checks of the four plugin crates.
- **N9 wgpu parity (P3), not implemented — design**: (a) the wgpu world engine (`OS/♾️infinite/🌍️world/🦀️.rs`) has no
  `gumballLiveDispatch`: `World3dState` must parse it from the selection record, `WorldGumballGesture` must remember the pose its
  last tick reached, every completed `update_step` must publish the incremental delta as `phase:"stream"` (a stream variant of
  `WorldGumballCommitJob`), the release must publish the tail as `phase:"commit"` (identity when zero), and the local instance
  preview must be skipped while live (the guest previews); the same `🛠️gumball-live-protocol.json` cases are the law.
  (b) wgpu has no window-level blur at all (only widget focus blur in `🖱️ui/…/⚡️events`), so a paint `blur` abort needs a
  renderer focus-lost signal first (S3-W2C, wgpu shell); the reachable part is `captureLost` on `WorldInteractionPhase::Close`
  while `paint_stroke_active` (publish `paintAt{phase:"abort", reason:"captureLost"}` before `begin_close`).
- `FemGumballTransient` and lowpoly's paint drive still duplicate the per-window runner bookkeeping (S2.6, follow-up for
  `🛠️tool-machine`). Fem 2d and fem 3d results-window playback clocks are twin modules (ported verbatim); unifying them
  needs one shared `FemResultsAnimation` config type first.
- Census F hazard (lowpoly `move-/rotate-/scale-selection` declared flat2): verified by reading — every selection leaf inverts
  to at most ONE `CreateMesh` (`lowpoly_selection_motion_inverse`), so forward + inverse = 2 rows per item is exact.
- Peer-owned warnings seen in my crates: `🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:15` unused `Buildable`/`HasBase`; lowpoly
  `🌐️model/🦀️.rs:10`, `🖼️uv/🦀️.rs:6,9` unused `lowpoly_window_engagement` (window-engagement peer).

### S3.5 Coordinator actions

- Central `schema generate`: removes the deleted shooting `set-camera-draft-label`, catalogues fem 2d `set-playback-clock`
  (the two remaining `schema mutation-inputs` findings in my scope).
- `describe` for the hub compositions `🎥️shooting` (verb `setCameraDraftLabel` gone, `saveCamera` describe text, lanes),
  `🏗️fem` (playback lanes, `resultAnimationTick{windowId}`), `💠️lowpoly`, `🎪️demonstrator`, `🧩️puzzle` (stale
  `transformBegin/End`, `paintStrokeBegin/End`), and the `OS/🧑‍💻dev/🔌️plugin-modules/*` copies.
- Re-activation of the fem, lowpoly and shooting lanes for a live check (gumball stream/commit/abort, playback press, the
  camera-label field, lowpoly colour picker on `apply-paint-stroke`).

### S3.6 Audit fixes (`📓️audit-s3-tools.md` S3-SPATIAL, 2026-10-03)

- **S1 (major) — wgpu live gumball consumer: DONE, law-green.** In `OS/♾️infinite/🌍️world/🦀️.rs` (my World3d region):
  - The selection record now carries `gumballLiveDispatch` (`World3dState.gumball_live_dispatch`), pinned into the gesture at
    the grab.
  - `WorldGumballGesture` remembers the pose its last tick reached (`sent_*`) and whether it `streamed`. Its protocol methods
    are `owed`, `moves`, `stream`, `release`, `cancel` and `publication`.
  - `WorldGumballPhase { Skip, Once, Stream, Commit, Abort(reason) }` drives the one publication job
    (`WorldGumballCommitJob::new(generation, gesture, phase)` appends `phase`/`reason` after the motion).
  - Every completed pose update of a live gesture publishes its owed delta as `phase:"stream"`. The release commits the tail
    (the identity when it moved nothing). A local gesture publishes ONE net delta, and nothing when it never moved along its
    handle or grabbed nothing (it used to publish a zero delta on every release).
  - The local preview is skipped while live (`retained_gumball_preview_model`).
  - New host cancel `WorldInteractionPhase::Cancel` / `WorldInteractionIntent::cancel(WorldCancelReason::{Blur, CaptureLost})`:
    a streamed live gesture aborts with its identity motion plus `reason`, and the rest of that drag is swallowed through its
    release. An open paint stroke publishes `paintAt{phase:"abort", reason}` (`plan_world3d_paint_abort`,
    `WorldFlatActionKind::PaintAbort`).
  - S3-W2C wired window blur → `cancel(Blur)` on every world state (native `Focused(false)`, browser `semioWgpuHostWindowBlur`).
    I asked for `cancel(CaptureLost)` one drive before a world surface retires. Plugin-local window gestures (fem
    `FemGumballTransient`, lowpoly paint, gen3d `GumballGestures`) are not covered by the runtime's `retired` abort.
- **S2 (major) — the corpus runs in every consumer.** React (engine-contract) and fem 3d as before. **wgpu**: the new law
  `world_gumball_gesture_answers_the_shared_live_protocol` drives the real gesture methods and publication job over all
  10 cases. Terminal dispatch (once/commit/abort/none) is exact; a local gesture never streams; stream ticks sum to the
  motion React streamed (wgpu has no in-flight hold, its queue coalesces moves). **gen3d**: S3-PROCEDURAL wrote
  `the_world3d_gumball_live_protocol_lands_as_its_guest_edits` (`DEV/🧊️generation3d/…/↔️translate-selection`) from my recipe.
- **S3 (minor) — twin fem playback clocks: OPEN.** The clean fix is one `FemResultsAnimation` (+ loop mode/waveform) and
  one `FemPlaybackClock` in the fem 2d crate (the shared FEM editor crate fem 3d already depends on for `FemGumballTransient`),
  each editor keeping only its window-kind owner. That changes both results-window config schemas (`$defs`, DSL records,
  fixtures), and the fem 3d transient leaf also feeds the language-neutral bridge (`🏗️fem/🏭️bridge/🦀️.rs`) and the
  `🧪️tests/🫧️mutate-fem-3d-1-any-editor-edit-results-transient` feature corpus. I left it until both crates compile again.
  Plan: shared `FemPlaybackClock` / `FemResultsWindowTransient{Mutation}` / `set-playback-clock` in fem 2d, plus a trait
  `FemPlaybackTransport { phase, reverse, speed, playback_loop, rested_at }` that each results config implements; fem 3d keeps
  only `Fem3dResultsWindowTransientOwner` (`fem3d-results`) and the `addressed_to`/`captured_clock` wrappers; the bridge and
  the feature corpus move to the shared leaf.

| Time (10-03) | Command | Result |
|---|---|---|
| 05:51 | `cargo check -p semio-framework-os-infinite --lib --tests` | PASS, 0 errors, no new warnings in my code |
| 05:54 | `cargo test -p semio-framework-os-infinite --lib -- gumball paint` (private target) | PASS 31/31 (incl. the corpus law) |
| 05:58 | same `-- gumball paint world` | 222 passed / 30 failed; again with `--test-threads=1` 207/30. My 3 new laws pass; the 30 are scene-bridge/pick tests unrelated to the gumball ("scene bridge stopped at Fault(Unavailable)", `pick_hover_action` → None). Peer, owner unknown (05:41 texture-sampler edit suspected), reported. |
| 06:05 | `cargo check --manifest-path ✏️s/Cargo.toml` (my 4 crates) | BLOCKED: peer ValueError migration in `OS/🔁️workflow/🦀️.rs:325-1189` and its sqlite snapshot (E0053/E0277), reported |
| 06:10 | `bun ./📜️script.ts typecheck` (React target package) | 1 error, a peer's: `🌐️World3dHost/🟦️.tsx:2225` surface-material `normalScale` (`'{}'` not assignable to `number`). The gumball reducer/driver code and engine-contract have 0 errors |
| 06:20 | new `pub fn world3d_cancel_owed(&World3dState) -> bool` (S3-W2C defers a closing world's retirement only when owed) + law `world_cancel_is_owed_only_by_a_streamed_live_gesture_or_an_open_stroke` | `cargo test -p semio-framework-os-infinite --lib -- gumball paint cancel`: 42 pass, 1 fail = the peer `authored_inline_surface_…_cancellation` (one of the 30) |
| 06:27 | `cargo check --manifest-path ✏️s/Cargo.toml` (my 4 crates) | still BLOCKED by the peer ValueError migration (`workflow` 222 errors, stdio jpg/gif/zip/svg/gltf) |
| 10:44 | `cargo test -p semio-framework --lib -- gumball_verb_audience` | PASS 1/1 (bracket-free fixture) |
| 10:47 | `cargo test -p semio-framework-os-infinite --lib -- gumball paint cancel` | 42 pass (all S3-SPATIAL laws), 1 fail = peer `authored_inline_surface_…_cancellation` |
| 10:50 | `cargo check -p semio-framework-plugin --lib` | PASS (the `selection_reference_id` hook + `draft_time_travel_selection` mapping compile; no warning in my lines) |
| 10:55 | `cargo test -p semio-framework-plugin --lib -- use_selection selection_value reference_chips` | BLOCKED: the plugin TEST target is red from the peer sqlite/IoError migration (`🧪️tests/🧩️composition/🦀️.rs:19,29,569`, `🔬️app-declarations-fixture/🦀️.rs:74-325`, `🖥️test-app-mutations-document`, `🧬️mutation-fixtures-*`: E0053/E0560/E0277) |
| 11:03 | `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d …` | BLOCKED: every fem/lowpoly/shooting crate depends on a red stdio crate (ply, dxf, svg, pdf, jpg, gif, zip, gltf, workflow — the same peer migration) |

**Owed after the stdio/plugin-test peer migration lands** (one gated cargo each, private target `target-nde-s3-spatial`):
1. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-shooting-shooting --features …component-app-assembly --lib --no-fail-fast`;
2. `cargo test -p semio-framework-plugin --lib -- use_selection selection_value reference_chips`;
3. `cargo check --manifest-path ✏️s/Cargo.toml -p <each of the 4> --lib --target wasm32-wasip2`;
4. then S3 (twin fem playback clocks → one shared clock + transient in fem 2d, plan above), which needs compile iterations.

## Session 4 — 2026-10-04

Continued by S4-TOOLS-A (draw + note + layout + fem/lowpoly/shooting) in `📓️s4-tools-a-report.md` § Session 4.
