# ◻️ wfc2d — the 2D arbitrary-graph artifact (slice A3)

Crate `semio-s-artifact-wfc-2d` at `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d` (281 files, 113 Rust
modules, 75 committed fixture JSONs). Dialect `s.wfc.wfc2d`, OS kind `2d.wfc2d`, ident `wfc2d`,
editor `Wfc2dEditor` (`s.wfc.wfc2d@1/*#editor`), viewer `Wfc2dViewer`, labels "2D"/"2D".

---

## 1. Document model as authored

```
Wfc2dSnapshot { schema: String, seed: u64,
                slots: [Wfc2dSlot], edges: [Wfc2dSlotEdge], tiles: [Wfc2dTile], rules: [Wfc2dRule] }

Wfc2dSlot      { id, x: f64, y: f64, width: f64, height: f64, pinnedTileId: String? }
Wfc2dSlotEdge  { id, fromSlotId, toSlotId, relation: String }
Wfc2dTile      { id, label: String?, weight: f64, media: Wfc2dTileMedia }
Wfc2dRule      { id, tileAId, tileBId, relation: String? (absent = every class), allowed: bool }
Wfc2dColor     { r, g, b, a: u32 }                      # 0–255
Wfc2dTileMedia = Empty | Bitmap { width, height, palette: [Color], pixels: base64 }
               | Vector { paths: [VectorPath] } | Image { child: ArtifactChild<SemioImageSnapshot> }
VectorPath     { segments: [PathSegment], fill: Color?, stroke: Color?, strokeWidth: f64 }
PathSegment    = Close | Move{to} | Line{to} | Quad{ctrl,to} | Cubic{ctrl1,ctrl2,to}   # tile space 0..1
```

Deviations from plan §3 **wfc2d**, each deliberate and load-bearing:

1. **Named relation classes are real.** The plan's `SlotEdge.relation: String` and
   `GraphRule.relation: String?` are implemented as a genuine multi-relation model: every distinct
   edge relation string becomes its own `RelationId` in a `ModelBuilder`, a rule with `relation: null`
   applies to every class, and `allowed: false` compiles to a `deny` (which the engine resolves after
   every `allow`, so a deny always wins). Assembly's ancestor route only ever had `RelationId(0)`.
2. **No `index` field on create payloads.** The plan asks for "collection inserts at canonical sorted
   positions (point-invertible)". Carrying an explicit index AND inserting at the sorted position are
   contradictory, so the payloads carry no index at all: every create/connect diff inserts at
   `crate::mutations::ordered_index` (the ascending-`id` position). Because the position is a pure
   function of the id set, a delete's inverse re-creates the row at exactly the index it left, which
   is the point-invertibility the `📓️explore-artifact-taxonomy-template.md` §3.6 law asks for — and
   it is what broke 14 of remodel's own vectors when they appended instead. Every collection in every
   example and fixture is therefore in ascending `id` order, asserted by a test.
3. **`Wfc2dTileMedia::Image` round-trips but is not a declared child.** The handle is reachable only
   through `tiles[].media`, which `ArtifactSchema`'s child enumeration cannot see, so no host ever
   hydrates it and the preview paints an outline placeholder. Wiring composed-child hydration
   (genesis pack, `SemioMembers` on both surfaces) is a real remaining increment, declared in the
   type's own docstring. Every bundled example uses `Vector`.
4. **Native codecs live under `🚪️io`, not `🧬️schema`.** The brief listed
   `🧬️schema/📸️snapshot/{💾️binary,📝️text}` (assembly's older layout). The repo scaffolder
   (`bun ./📜️script.ts new subset`) and `🗒️note` — the current canon — put the `ArtifactDsl`/
   `ArtifactPack`/`OpText`/`OpBinary` impls and their grammar/protocol sidecars under
   `🚪️io/{📸️snapshot,🧬️mutations,🔺️diff}/{📝️text,💾️binary}` and keep `🧬️schema/<facet>/` for the
   five normative language leaves the descriptor `include_str!`s. This artifact follows the
   scaffolder. All twenty descriptor leaves exist and are non-empty (asserted).
5. **Examples print their document; they carry no committed `🗣️.dsl.semio` asset.** An asset beside a
   Rust builder is a second copy with nothing keeping it in sync — remodel's W8 pass found exactly
   that drift. `source()` calls `store::ArtifactDsl::print_dsl(&document())`, and a per-example test
   asserts the printed text parses back to the very document it came from.

---

## 2. Mutations (15)

| dir | kind | payload | verb/entity | notes |
|---|---|---|---|---|
| `🎲️change-seed` | change-seed | `seed: u64` | change/seed | no-op warns |
| `🧩️create-slot` | create-slot | `slot` | create/slot | dup id, non-positive size, unknown pin → fatal; inserts at `ordered_index` |
| `🕳️delete-slot` | delete-slot | `id` | delete/slot | cascades every incident edge, one `info mutation.cascade` |
| `↔️move-slot` | move-slot | `id, x, y` | move/slot | ONE per drag gesture, never per tick |
| `📐️resize-slot` | resize-slot | `id, width, height` | resize/slot | non-positive → fatal |
| `🔗️connect-slots` | connect-slots | `edge` | connect/slots | dup id, unknown endpoint, empty relation → fatal |
| `✂️disconnect-slots` | disconnect-slots | `id` | disconnect/slots | |
| `📌️pin-slot` | pin-slot | `id, tileId` | **fix**/slot | `pin` is NOT in `protocol::APPROVED_VERBS`; `fix` is, and is the exact WFC term |
| `🔓️unpin-slot` | unpin-slot | `id` | **clear**/slot | same reason |
| `🀄️create-tile` | create-tile | `tile` | create/tile | negative/non-finite weight → fatal |
| `🗑️delete-tile` | delete-tile | `id` | delete/tile | cascades rules naming it AND releases every pin holding it |
| `⚖️change-tile-weight` | change-tile-weight | `tileId, weight` | change/tile-weight | |
| `🎨️change-tile-media` | change-tile-media | `tileId, media` | change/tile-media | its committed vector swaps a VECTOR tile for a 4×4 RASTER one, so the `Bitmap` branch is covered by a fixture |
| `🚦️create-rule` | create-rule | `rule` | create/rule | unknown tile → fatal |
| `❌delete-rule` | delete-rule | `id` | delete/rule | |

Each kind ships `🦀️.rs` + `🔣️.json` descriptor + `🧬️schema/🔣️.json` payload schema + `🔺️diff/🦀️.rs`
+ `↩️inverse/🦀️.rs` + one mounted `🧪️tests/<case>/🦀️.rs`, and one fixture quintet under
`🧫️fixtures/🧬️mutations/<kind>/<case>/{📸️snapshot/⬅️before,➡️after,🦠️mutation,🎯️outcome,🔺️diff}`.
Floats are written `N.0` throughout. `Wfc2dDiff` has four INDEXED collection pairs (no unordered
lane, unlike assembly's `weights`) because all four collections are order-significant here.

Regenerate the whole set with `python3 🐍️wfc2d-mutations.py` (idempotent; also re-emits the crate
root's `#[path]` mount block into `🗑️generated/wfc2d/mount-block.txt`). The language sidecars and the
facet leaves come from `python3 🐍️wfc2d-sidecars.py`.

Editor config (`✏️editor/🎚️config`) adds three of its own: `replace-config`, `change-camera`,
`change-active-tile`. The app transient (`✏️editor/🫧️transient`) adds one: `set-solve`.

---

## 3. Windows and scenes

### 3.1 `wfc-graph` — the reusable one (A5 copies this file verbatim)

`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs`, `SurfaceKind::NodeGraph`.

**The copy contract, stated at the top of the file itself.** The window knows NOTHING about
`Wfc2dSnapshot`. Its entire view of a document is three local items:

```rust
pub struct GraphSlotView { id, x, y, width, height, pinned_tile_id }
pub struct GraphEdgeView { id, from_slot_id, to_slot_id, relation }
pub trait  SlotGraphView { fn graph_slots(&self) -> Vec<GraphSlotView>; fn graph_edges(&self) -> Vec<GraphEdgeView>; }
pub struct GraphCamera   { x, y, zoom }
```

plus `definition()`, `graph_records(&impl SlotGraphView)` and
`render(&impl SlotGraphView, GraphCamera, &[String]) -> UiAssemblyResult<BuiltNode>`.

Shared constants A5 must keep spelled identically: `WFC_GRAPH_WINDOW = "wfc-graph"`,
`WFC_GRAPH_BODY = "wfc.graph"`, `WFC_GRAPH_PORT_IN = "adjacent-in"`,
`WFC_GRAPH_PORT_OUT = "adjacent-out"`, and the nine action ids
(`create-slot`/`delete-slot`/`move-slot`/`resize-slot`/`connect-slots`/`disconnect-slots`/
`pin-slot`/`unpin-slot`/`change-seed`), every one classified `Migrated`.

**What A5 writes on its own side:** one `Wfc3dGraphView<'a>(&'a Wfc3dSnapshot)` newtype in its editor
root implementing `SlotGraphView` by projecting `x`/`y` and dropping `z`, exactly as
`Wfc2dGraphView` does in `✏️editor/🦀️.rs`. Nothing else. The file itself is a byte-for-byte copy.

One node per slot at its authored rectangle, one edge per adjacency; each node carries exactly one
in-port and one out-port because the adjacency graph is undirected and the ports exist only to give
the canvas something to drag a wire between. Node captions are truncated at 96 bytes — one oversized
owned-text admission fails the WHOLE surface refresh, not just that row.

### 3.2 `wfc-2d-preview` — the solved board

`✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`, `SurfaceKind::Canvas2d`. One `rect` bounds
layer per slot at `x/y/width/height`, plus — for a solved slot whose tile is `Vector` — one `path`
layer per subpath with `transform: [width, 0, 0, height, x, y]` mapping tile space `0..1` onto the
slot rectangle. Layer records follow `Canvas2dHost`'s own prop contract (`kind`, explicit bounds,
`segments` with `{kind: move|line|quad|cubic|close}`), not the mutation field names — pitfall 8 of
`📓️explore-build-verify-playground-pipeline.md`.

A `Bitmap` tile is encoded to a real `data:image/png;base64,…` and emitted as a `kind: "image"`
layer, which `Canvas2dHost` resolves through its own image cache. The encoder lives once, in
`🚪️io/📸️snapshot/💾️binary/🦀️.rs::tile_media_png_data_url`, so the editor preview and the viewer board
share it without the viewer importing through the editor; it goes through `s.stdio.png`'s own
`encode_png` rather than a second, drifting PNG writer. A malformed bitmap (wrong payload length,
zero size, empty palette) falls back to the outline rather than failing the whole surface refresh.

The assignment comes from the app TRANSIENT (`Wfc2dTransient`), never the document, so the window can
never make an inference look persisted. An unsolved board still renders: every slot as an outline,
and any AUTHORED pin already painted.

**The transient reaches the window through `ArtifactEditor::render_with_request_context`** — the
host-facing render entry point, the only one whose signature carries the transient lane. It delegates
to the free `render_with_transient(body_key, doc, cfg, transient)`, which a test CAN drive (the two
arguments it drops, `ArtifactInstanceOperationOwnerHandle` and `InteractionView`, are unconstructible
from this crate and neither is read here). The bare `ArtifactEditor::render` has no transient
parameter at all and therefore paints authored pins only; that is the trait's shape, not a gap.

### 3.3 Layout and the viewer

`edit` mode: `create_default_layout([wfc-graph, wfc-2d-preview], "row", [50, 50])`.
The viewer declares its own read-only `wfc-2d-board` window (a duplicate of the preview projection,
on purpose — a viewer file importing through the sibling editor module is what
`policyViewerPurityBreaches` refuses) and boots the same committed example the editor does.

### 3.4 State tiers

- document → the 15 mutations;
- per-window-INSTANCE config `Wfc2dConfig { camera_x, camera_y, camera_zoom, active_tile_id }` — one
  per pane, so each pane has its own camera and its own armed tile; `wfc2d_active_tile_id` falls back
  to the document's first tile so a pin verb never receives an empty id;
- app transient `Wfc2dTransient { assignments: [{slotId, tileId}], contradiction }` — the last solve.

---

## 4. Inference `s.wfc.wfc2d.solve`

`🧬️schema/💡️inferences/🦀️.rs`. Routed cold job (`semio.infer`, payload schema
`s.wfc.wfc2d.inference.request.v1`, classification `Migrated`, `ToolExecutionContract::resumable`),
registered by `register_wfc2d_inference_factory(&ActionBus)`; metadata owner `"wfc"`.

Stages: `Tiles → Relations → Rules → Model → Slots → Edges → Topology → Fixed → {Restore|Solve} →
MapCommit → EncodeCommit → Complete`.

- **Tiles** — one `PatternId` per tile in document order; a non-finite or non-positive weight reads
  as `1.0`.
- **Relations** — the distinct edge relation strings, sorted; empty document falls back to
  `"adjacent"`. A document with no slots or no tiles short-circuits to `EncodeCommit`.
- **Rules** — `ModelBuilder::{allow,deny}` per scoped relation, both directions (adjacency is
  symmetric; the relation scopes WHICH rules apply, never a direction).
- **Model** — `ModelBuilder::compile()` in one step. (`GraphModelBuild` is the engine's steppable
  compiler but is single-relation only, so it cannot express this artifact's model.)
- **Slots/Edges/Topology** — `GraphTopologyBuild`, `add_arc` both ways per edge, stepped.
- **Fixed** — `pinnedTileId` → `(NodeId, PatternId)` pins.
- **Solve** — the resumable `WfcJob<GraphTopology>`; every child job goes through
  `semio_s_plugin_wfc_engine::job::close_job` before it is dropped (engine `📓️engine.md` §7).
- **Commit** — `{ assignments: {slotId: tileId}, contradiction: bool, entropy: {slotId: f64} }`,
  paged out with `RetainedJobPayloadWriter::write_slice_page`. A solve that completes without a
  commit is UNSATISFIABLE: the honest answer is `contradiction: true` plus the prior entropy map, not
  a fault.

Three `store::InferredField` façades over the same headless `solve_with_job`: `Wfc2dSolve`,
`Wfc2dContradiction`, `Wfc2dEntropy`. Entropy is the PRIOR (pre-propagation) Shannon entropy over the
tile weights, `0.0` for a pinned slot — declared as such in its docstring; threading the
post-propagation domain through would need the solver's live state and is a real remaining increment.

---

## 5. Examples (4)

| slug | id | shape | why |
|---|---|---|---|
| `🚪️two-room-corridor` | `two-room-corridor` | 3 slots on a line, 2 edges, 2 tiles, both same-tile adjacencies denied | FORCED: the only assignment is room·corridor·room, so its outcome is a fixture rather than a sample |
| `🧱️wall-roof-facade-strip` | `wall-roof-facade-strip` | 2×2 bays, 4 edges across TWO relation classes (`beside`, `above`), one roof bay pinned | the cyclic topology, and the relation-SCOPED rule this artifact adds over assembly |
| `⬡️hex-ring` | `hex-ring` | 6 slots on a regular hexagon, a 6-cycle on relation `ring`, `cap` may not touch `cap` | nothing about it fits a rectangular grid — the whole reason `wfc2d` exists beside `grid2d` |
| `🗺️terrain-ring` | `terrain-ring` | the same ring, but every tile is an 8×8 palette-indexed **Bitmap** (`water`/`shore`/`grass`/`rock`, ends of the transect denied) | the only example exercising the RASTER media branch end to end — without it the bitmap path is unrendered and untested, which is exactly how it stayed a label for one audit cycle |

---

## 6. Tests

| where | what |
|---|---|
| per mutation ×15 | the six-law fixture shape: applies-to-after, inverse-restores-before, committed JSON canonical, declared outcome holds, produces-committed-diff, committed-diff-is-canonical, committed-diff-applies-to-after |
| `🧬️schema/📸️snapshot/🧪️tests/🔬️unit` | default, addressing, relation universe, pack round trip for all three examples |
| `🧬️schema/🔺️diff/🧪️tests/🔬️unit` | identity, absorb precedence, refusal of a missing removal |
| `🧬️schema/🧬️mutations/🧪️tests/🔬️unit` | `KINDS` roster, `ordered_index`, and every kind round-tripping through BOTH op codecs |
| `🧬️schema/💡️inferences/🧪️tests/🔬️unit` | deterministic seed, the forced corridor, **the contradiction case** (two adjacent pinned slots of a denied pair), zeroed entropy on pins, relation-scoped rules, the hex ring, `InferredField` agreement, empty document, deny-beats-allow |
| `✏️editor/🧪️tests/🔬️unit` | all 15 document verbs emit exactly one mutation and touch no config; the 2 view verbs touch only config; pin-without-tile refused; BOTH windows render for ALL THREE examples; command binary round trip |
| window tests ×3 | graph identity/ports/projection/every-verb-Migrated; preview layers for solved, pinned and unsolved slots; viewer board |
| `✏️editor/🧪️tests/🔬️unit` (host path) | `the_host_render_path_paints_the_solved_assignment` drives `render_with_transient` — what `render_with_request_context` runs — and asserts a solved board renders differently from an unsolved one, i.e. the transient lane is really read; `the_host_render_path_paints_bitmap_tiles_as_pixels` asserts a raster tile reaches the canvas as `kind:"image"` + a `data:image/png;base64,…` URL and never as the outline placeholder |
| `📚️examples/🗺️terrain-ring/🧪️tests/🧩️example` | every tile really is an 8×8 indexed bitmap, each encodes to a PNG data URL, a malformed bitmap refuses instead of panicking, and the raster ring still solves |
| `✏️editor/🎚️config`, `🫧️transient` | defaults, armed-tile fallback, mutation inverses |
| `🧪️tests/🔬️store-fixture` (artifact root) | both envelopes round trip, decode-in-place retirement reaches terminal-empty, an edit ladder inverts |
| `🧪️tests/🧩️mount-contract` (subset) | dialect agreement, both window kinds, example round trips, localized labels, the inference route, all twenty descriptor leaves, the io declaration |
| `🧪️tests/🔬️unit` (artifact root) | dialect/schema/kind identity, `definition()` builds |
| `🧪️tests/🧩️mutate-wfc2d-1` | the `🥒️.feature` + `🐍️.py` + `🦀️.rs` triplet (see §7) |

---

## 7. Commands run

| command | result |
|---|---|
| `bun ./📜️script.ts new artifact 🀄️wfc "◻️2d"` (+ `new standard`, `new subset`) | **GREEN** — scaffolded the canonical tree shape before hand-authoring |
| `bun ./📜️script.ts new mutation … --dry-run` | **RED, not mine.** First attempt: `Invalid taxonomy schema: semanticDirectoryMemberKinds["members-of-artifacts"] has invalid exact member "▦️grid2d"` (a sibling slice's taxonomy row, since fixed). Retry after the fix: `"🧪️tests" must be an emoji-prefixed semantic verb-noun kebab name` — the scaffolder refuses to re-scaffold over a mutation directory that already holds its `🧪️tests` cases. The 15 folders were hand-authored to the documented shape (§3.1–3.8 of the template report) and match assembly's leaf-for-leaf. |
| `cargo check … --features component-app-assembly --lib --tests -j 4` | **GREEN** (`🗑️generated/wfc2d/check-7.log`) |
| `RUST_MIN_STACK=33554432 cargo test … --lib -j 4 -- --test-threads=4` | **GREEN — 174 passed, 0 failed** (`test-7.log`) |
| `cargo clippy … --lib --tests -j 4` | **GREEN — 0 warnings in this crate** (`clippy-2.log`) |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check … --target wasm32-wasip2 -j 4` | **GREEN** (`wasm-1.log`) |
| `bun nx run @semio-tech/wfc-js:test` | **GREEN — 113 tests across 5 files** (`ts-3.log`); wfc2d contributes the cross-language fixture oracle (all 15 committed quintets replayed through the TypeScript twin) plus one law suite per bundled example |
| `python3 …/🧪️tests/🧩️mutate-wfc2d-1/🐍️.py` | **GREEN — 15 kinds, 0 divergences.** The Python second implementation replays every committed quintet: produced diff, raised diagnostics, committed-diff-carries-before-to-after, and inverse-restores-before all agree, and it asserts a fixture case exists for every declared kind. It imports nothing from this repo's Rust. |
| `bun ./📜️script.ts verify taxonomy enforce --scope "…/◻️2d"` | **RED, not mine, aborts before reaching wfc**: `frozen-coordinate-evidence-invalid: 🧰️framework/…/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json: document digest does not match registered bytes`. Slice P reported the same pre-existing fault (`📓️scaffolding.md`). Re-run once that fixture's owner re-freezes it. |

### 7.1 Three real bugs this slice found and fixed

1. **A child job's `CommitCandidate` carries TWO retained payloads.** The parent kept `state` and
   dropped `output`, which trips `RetainedJobPayload`'s own Drop assertion
   (`RetainedJobPayload requires one-page close to terminal-empty`) inside `step`. Fixed with a
   `retire_payload` helper applied to the candidate's `output` in the `Solve` branch and to BOTH
   payloads in the `Restore` branch. ⚠️ **`🧩️assembly`'s inference has the identical latent bug**
   (`…/💡️inferences/🦀️.rs`, `AssemblyInferenceStage::Solve`): it keeps `candidate.state` and drops the
   candidate, and its `Restore` branch discards the whole candidate. Worth telling slice C / the
   procedural owners before that code is copied anywhere else. A4/A5 should apply the same fix.
2. **`terminal_is_empty` must not be gated on a `closing` flag.** The framework's close ladder calls
   `job.begin_close()` itself at close stage 0 and then refuses to advance past stage 1 while
   `terminal_is_empty()` answers false (`worker_job_close_step`, `🧵️job/🦀️.rs:2724`), returning
   `Blocked` — which a `while !terminal_is_empty() { close_step(…) }` driver spins on forever rather
   than failing. Assembly gates on `self.closing`; this artifact reports ownership only, and its
   headless close loop carries a `guard` that asserts instead of hanging.
3. **`wfc-unsatisfiable` is a FAULT, not a commit-less completion.** The engine publishes
   `PublicationKind::Fault` with detail `b"wfc-unsatisfiable"` (`⚙️engine/💼️job/🦀️.rs:1771`) when the
   solve contradicts. The inference intercepts exactly that detail, retires the fault payload, closes
   the child and answers the contradiction verdict plus the prior entropy map. Any other fault still
   passes through.

Two further corrections, in this slice's own tests rather than its code: a path graph under
"no two neighbours alike" has TWO valid colourings, not one, so `the_forced_corridor_…` now asserts
the ALTERNATION law (middle differs from both ends, ends agree) instead of one specific answer.

Two peer-caused RED windows were hit and waited out, neither in this artifact:
`semio-framework-os-infinite` failed to compile for ~10 minutes on
`camera_grid_fade_distance`/`lod_grid_fade_alpha` (a `🖱️ui` refactor landing mid-flight), and the
plugin-folder emoji rename `🌊️wfc` → `🀄️wfc` moved 44 of this slice's files mid-write (all recovered,
newer-wins, and both generator scripts repointed).

---

## 8. Audit fixes (`📓️audit-wfc2d.md`)

The conformance audit rated everything else PASS and flagged two blocking gaps plus two minor ones.
All four are now closed:

1. **`ArtifactEditor::render` always passed `Wfc2dTransient::default()`** — the host render path could
   never show a solve. Fixed by overriding `render_with_request_context` (§3.2), with two tests that
   drive the same projection the host runs.
2. **`Bitmap` tile media rendered as a text-labelled rect** — now a real PNG `image` layer (§3.2),
   shared by both previews, with a bundled raster example (`terrain-ring`), a fixture vector
   (`change-tile-media` now swaps a vector tile for a raster one, so the Python and TypeScript oracles
   replay it too) and six new tests including the malformed-payload refusal.
3. **Two bare `//` comments inside `impl Drop for Wfc2dSnapshotRetirement`** — folded into a `///` doc
   comment on the `drop` fn.
4. **The bitmap-preview gap was not in §9's list** — it never needed to be listed, because it is fixed.

One latent bug surfaced while regenerating the fixtures: `🐍️wfc2d-mutations.py` still carried the
UNAPPROVED `pin`/`unpin` verbs (the `fix`/`clear` fix had only been hand-applied to the emitted
files), so re-running it reintroduced an `E0080: Mutations requires an approved semantic verb`. The
generator is now the single source of that truth.

---

## 9. Known gaps

1. **`Wfc2dTileMedia::Image` is carried, not hydrated** — see §1.3. No example or fixture uses it.
2. **Entropy is the prior, not post-propagation** — see §4.
3. **The `🥒️`/`🦀️` halves of `🧪️tests/🧩️mutate-wfc2d-1` are the host-runner shape and are not
   mounted into the crate.** The executed proof of that case is the Python reference's own standalone
   replay plus the 15 mounted per-kind fixture tests, which assert the same laws inside `--lib`.
   Wiring the `semio_repo_test_host` adapter is W2/W3 work.
4. **`ModelBuilder::compile()` runs in one job step** rather than incrementally. For a tile alphabet
   of the size this artifact targets that is well inside the step budget; a multi-relation steppable
   compiler in the engine would be the honest fix if a catalogue ever grows large.
5. **The solve job still has to publish into the transient.** The lane is now wired all the way to
   the window (§3.2) and proven by a host-path test, but *who* writes `set-solve` after a spawned
   solve completes is the plugin-root/playground side of the wire — W2 work, and the memory
   `React Host Never Refreshes On Spawned Job Progress` is the thing to follow there.
6. **The plugin TS barrel names paths the scaffolder layout does not produce.** `📦️packages/🟦️typescript/🟦️.ts`
   expects `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/{📝️text,💾️binary}/🟦️.ts` per artifact. wfc2d now
   ships those six as thin TS facades (language id + carrier type) so the barrel resolves, with the
   real codecs staying under `🚪️io`. If a sibling artifact instead put its Rust codecs under
   `🧬️schema`, the plugin would carry two layouts at once — worth one decision at plugin-root level.
7. **No foreign io hop.** `io().entries` is deliberately empty: a WFC problem spec has no external
   interchange format. Its own DSL and pack envelope ARE the format.
