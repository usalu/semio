# 🌳️ wgpu DOCUMENT-RECONCILE — the published document now reaches the paintable arena

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu document→arena reconcile + guest effects",
2026-09-12. Resumes `📓️wgpu-blank-paint-2026-09-12.md` §5 (deliverable A) and §6 (deliverable B).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. Evidence under
`🗑️generated/wgpu-reconcile/baseline-1 … run-10`; nothing under any `🗑️generated` folder was swept.
The react serve on 6018 was not touched, the procedural guest was not rebuilt or restaged, and no
git-state-modifying command was run.

---

## 1. TL;DR

| question | answer |
|---|---|
| Does the published document reach the paintable arena? | **Yes.** `dumpStructure` went from **0 nodes** to the real Flow window body — `procedural-play-main.body` → `tree` + `procedural-play-main.canvas` → `componentScene#procedural-main` (§4.1). |
| Does it lay out? | **Yes, to real geometry**: root `1433.6×836`, outline tree `0,0,716.8,836`, canvas column and its node-graph surface `716.8,0,716.8,836` — exactly the row split the plugin authors (§4.2). |
| Does it paint? | **Natively yes, in production not yet.** The Rust law drives publish → reconcile → layout → `Ui::frame` on the live arena and asserts non-empty draw layers, quads and shaped glyphs (§5.1). On 6118 the document paint phase no longer faults, but the frame dies one stage later, in the GPU present (§4.4). |
| What blocks the pixels now? | **`raster commit candidate witness was stale`** — the raster-texture operation authority in `✍️draw.rs`/`🧊️renderer`, newly reachable because a document finally composites an `engine:<surface>` raster. It quarantines the surface at ~7.8 s. Not this lane's code; named exactly in §6. |
| Two defects fixed on the way | The retained `Tree`'s rows must be REAL arena children (§3.3), and both introspection probes were reading fields the production paint path never writes (§3.4) — which is why `drawCalls: 0` survived every earlier lane regardless of what painted. |
| Defect B (guest effects) | **Not taken on.** No line of the effects path was changed. `renderSurface … effects=0` is unchanged from the baseline. §7 says exactly where it stands. |

---

## 2. The feature

`UiTree::publish_document` stored the published `UiDocumentTree` in a field whose only readers were a
generation comparison and its own retirement. Nothing walked its `UiNodeRecord`s into the arena, so
`UiTree::root` stayed `None` forever, `Ui::frame_into_step` answered `Missing` on its first line, and
the arena's only writer — `Ui::apply_tree` — is `#[cfg(any(test, feature = "testkit"))]`. **In
production this target had no path at all from a retained document to a paintable tree.**

That path now exists, in three pieces.

### 2.1 The projection — `Component` → `UiNode`

`🔀️reconcile/🦀️.rs`, region `🌳️DocumentTreeReconcile`: `ui_node_from_record(document, record, surface,
controller)` projects every one of the contract's eighteen `Component` variants onto the retained
`UiNode` this target paints. Three rules make it total rather than partial:

* **Children are left EMPTY on every container variant.** The arena's own
  parent/first-child/sibling links are the tree — `mounted_layout`, `paint` and `scene_slots` all walk
  those links and never a spec's inline `children`. A `Field`'s non-optional `child` is a never-painted
  placeholder for the same reason (its real child is `children[0]` on the record).
* **Geometry comes off the RECORD, not the props.** `LayoutSpec::Stack{axis,gap,padding}` becomes
  `UiStackNode`'s `direction`/`gap`/`padding` through the `SpaceToken` → `"none"/"tight"/"loose"`
  table `layout::gap_for_token` already reads; `Grid`/`Scroll`/`Overlay` fall back to their own
  padding, `Leaf`/`Absolute` to the theme defaults.
* **Presence is split the way the contract splits it.** `disabled`/`transition`/`activity` are
  document state and project onto `UiPresence.state`/`.status`; hover, selection, own colour and peer
  marks travel on the separate coalesced presence channel and are stamped by
  `sync_interactive_state_node_step`, never by this projection.

`Component::Surface` is the one component whose subtree the projection CONSUMES: its children are the
out-of-doc payload lane carriers, and `surface_scene_node` decodes the spine out of `doc.bytes` per
kind and re-attaches every world-3d lane whose carrier has fully arrived (byte-length checked against
the spine's own manifest, so a half-arrived `meshes` lane is left at its spine value rather than
parsed as an empty scene). Its three identities are React's own, verbatim — `surfaceId` is the owning
DOCUMENT's surface, `controllerId` the owning app's controller, `paneId` the record's authored key
(`surfaceHostIdentityV1`, ticket 26/09/02 wave B20). A schema this target cannot decode yields a scene
node with no payload, never a dropped record — the contract's own "an unrecognised `doc_schema` must
never reject the surrounding patch" rule.

### 2.2 The reconcile — budgeted, resumable, identity-preserving

`UiDocumentReconcileCursor` advances by exactly ONE unit per step across six phases:

| phase | one unit |
|---|---|
| `Adopt` | drop a root this ledger never minted (only the testkit `apply_tree` path can leave one) |
| `Plan` | one record: append to a pre-order DFS plan, push its children unless it consumes its subtree |
| `Unlink` | one mounted node: sever its tree links so sibling order can be rebuilt without losing identity |
| `Mount` | one planned record: reuse-or-create its arena node, update its spec, attach it to its parent |
| `Retire` | one ledger entry the new plan does not name: free its arena slot and drop the binding |
| `Publish` | mark the root dirty and enqueue the layout the fresh root now needs |

Identity is the `UiNodeId`, never tree position. `UiTree` gains a sorted `document_nodes` ledger
(`UiNodeId → NodeId`, binary search, no allocation on lookup); the producer mints one id per
`(parent, key)` and reuses it while that identity survives
(`🧠️runtime`'s `SurfaceReconcileStage::AllocateIdentities`), so a node that only moved or changed
props keeps its arena slot — and with it its `WidgetState`: a focused editor's live buffer, a scroll
offset, an open `Select`. That is the Rust twin of React's `uiChildReactKeys` rule, and §5.1's second
law pins it against the live arena.

`Ui::step_document_reconcile(window_id, controller, cx)` spends the opportunity's whole `StepBudget`
rather than one unit, and `Ui::close_document_step` now drives `close_document_binding_step` first, so
a surface that retires its document frees the arena slots it minted instead of leaking them.

### 2.3 The call site

`UiDocumentFramePhase` gains a `Reconcile` phase between `Ingress` and `Viewport` — layout reads the
arena's parent/child links, which do not exist until the reconcile has run. `render_ui_document_step`
takes the owning controller (`ShellState::document_controller_id`: the live session's app, falling
back to the host program's) because the semantic contract moved that identity off the node onto the
session, and an action binding without a controller has nobody to answer it.

---

## 3. Defects found and fixed on the way

### 3.1 A retained document's arena root had no writer at all — §2, the feature itself.

### 3.2 `dumpStructure` reported an unlaid-out tree even when layout had published

`walk_dump` read `Node::layout` — the IMMEDIATE-MODE bucket. The retained path publishes through
`mounted_layout`'s double-buffered `AcceptedLayout` (`write_inactive_layout` + `commit_inactive_layout`),
which is what `paint_node_step` and `RetainedPaintWalk` consume; `Node::layout` stays zero forever.
Every rect the probe printed was `[0,0,0,0]` no matter what the layout engine had computed. Fixed by a
public `UiTree::mounted_layout(id)` accessor and by `walk_dump` reading it.

### 3.3 The retained `Tree`'s rows must be REAL arena children

First cut consumed a `Component::Tree`'s subtree into the inline `UiTreeNode` spec, mirroring React's
`collectTreeItems`. On 6118 that faulted every frame with the new `synchronize-node` site (§3.5):
this engine's `sync_interactive_state_node_step` resolves EVERY section and item row to an arena child
keyed by that row's id (`TreeApplyPrepare` → `TreeApplyScan` → `TreeApplyWrite`) and returns `Fault`
the moment the row is missing — `reconcile::children_of`'s `Tree` arm synthesizes exactly those rows
for the declarative path.

The document path does not have to synthesize them, because the producer already published them as
`TreeSection`/`TreeItem` records. They now mount as themselves, projected onto the same keyed
`Stack` rows (`tree_section_row`/`tree_item_row`'s shape), while the owning `Tree` keeps its inline
sections for paint. `Surface` is the only component left that consumes its subtree.

### 3.4 `dumpFrameStats` measured a draw list the production path never fills

`build_frame_stats` read `Ui::draw_list`, i.e. `UiWindow::draw` — which only the `frame_step` entry
publishes into. The production entry is `frame_into_step`, which appends into the CALLER's draw list
(the window body's) and never touches `UiWindow::draw`. **`drawCalls: 0` was therefore guaranteed on
this target regardless of what painted**, and every earlier lane's blank-paint verdict rests on a
number that could not have been anything else.

Fixed with `UiFramePaintCensus { layers, quads, glyphs }`, measured as the DELTA the window's own paint
frame appended to whatever target it was handed (baseline captured when the paint frame opens, diffed
at `Publish`), published as `Ui::paint_census(window_id)` and preferred by `build_frame_stats` over the
retained list.

### 3.5 A `UiFrameStep::Fault` could not say where it came from

`frame_into_step` has eight distinct fault sites and one verdict. `RetainedPaintFrame` gains a
`fault_site`, surfaced through `Ui::paint_frame_phase(window_id)`, and the interpreter logs it on a
terminal document fault (`eprintln!` is a no-op inside a `wasm32-unknown-unknown` Worker, so a fault
recorded there left no trace at all — the same trap `📓️wgpu-blank-paint` §7 hit). That is what turned
"the paint faults" into "`synchronize-node` faults", i.e. §3.3, in one boot.

Likewise `commit_presented_step`'s single `"raster commit witness was stale"` covered two different
faults; it now distinguishes a retired candidate, a stale candidate, an unarmed presentation and a
stale presentation, and `GpuContext::commit_presented_rasters_step` appends the three witnesses it
compared.

---

## 4. What 6118 does now, measured

### 4.1 The arena mounts (`🗑️generated/wgpu-reconcile/run-2`, `run-3`)

Baseline (`baseline-1`, before this lane): `nodes 0`, `{drawCalls: 0, quadCount: 0, glyphCount: 0}`,
window area empty (navbar and footer painted; the body did not).

After the reconcile:

```
nodeCount 4
stack[0]#procedural-play-main.body
stack[0]#procedural-play-main.body/tree[0]
stack[0]#procedural-play-main.body/stack[1]#procedural-play-main.canvas
stack[0]#procedural-play-main.body/stack[1]#procedural-play-main.canvas/componentScene[0]#procedural-main
```

That is exactly what `✏️s/🔌️plugins/🌀️procedural/…/✏️edit/🪟️windows/🕸️flow/🦀️.rs::render` authors: a
`row(grow)` holding the graph outline and a `column(grow)` whose only child is the node-graph
`scene_surface`. The surface's own lane carriers are correctly absent from the arena.

### 4.2 …and lays out (`run-3`)

```
procedural-play-main.body     rect [0,     0, 1433.6, 836]
  tree[0]                     rect [0,     0,  716.8, 836]
  procedural-play-main.canvas rect [716.8, 0,  716.8, 836]
    componentScene#procedural-main rect [716.8, 0, 716.8, 836]
```

Real geometry, the row split the document asks for, and the node-graph surface holding a full half of
the window body.

### 4.3 …and the document paint stops faulting (`run-7`)

Before §3.3: `[DEBUG] ui-doc paint fault window=procedural-main phase=Some("synchronize-node")` on
every frame. After it: no `ui-doc paint fault` line in the whole run.

### 4.4 …and the frame then dies in the GPU present (`run-7` … `run-10`)

```
worker-present-failed: raster commit candidate witness was stale
Surface: quarantined · Boot stage: ready · silent for 994 ms
UI turns over budget 0/0 · Worker steps over budget 0/0
```

at **t ≈ 7.8 s**, on every boot. `drawCalls` cannot be read after that point: the quarantine stops the
introspection beacon, so the paint census of §3.4 has no live frame to report and the probe reads 0.
**This lane does NOT claim a painted canvas on 6118.** What it claims is measured above: the document
mounts, lays out and reaches the paint pipeline without faulting, and the first thing downstream of it
now fails instead.

---

## 5. Tests actually run, verbatim

### 5.1 The Rust law, against the live arena

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs` (new), mounted from
`🔀️reconcile/🦀️.rs`, reading the neutral fixture
`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json` (new — the Flow window body's
own shape: a horizontal row, an outline column with a text leaf, a canvas column, the node-graph
surface, and the surface's paged lane carrier).

```
$ cargo test -p semio-framework-ui --features wgpu-engine --lib -- document_tree_reconcile
running 3 tests
test wgpu::reconcile::document_tree_reconcile_tests::retiring_a_document_frees_every_node_it_mounted_one_step_at_a_time ... ok
test wgpu::reconcile::document_tree_reconcile_tests::a_second_generation_preserves_surviving_identities_and_retires_only_what_left ... ok
test wgpu::reconcile::document_tree_reconcile_tests::a_published_document_mounts_its_records_and_the_arena_it_produces_paints ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 387 filtered out
```

The headline lane asserts, in order: an unreconciled tree has no root; publishing alone mounts
nothing; the reconcile terminates inside its own node budget; the mounted keys are the document's own
in document order and NOTHING under the surface; the surface projects onto a `ComponentScene` carrying
React's three identities; and then it drives the real `Ui` — publish → reconcile → `step_layouts` →
`Ui::frame` — and asserts **at least one non-empty draw layer, at least one quad instance and at least
one shaped glyph**. That is the end-to-end paint claim this lane can make honestly today, and it is
made against the live arena rather than a mock.

### 5.2 The TypeScript twin, on the same fixture

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/{🟦️.ts,laws.json}`
(new, registered in the wgpu `vitest.config.ts`).

```
$ bunx vitest run --config vitest.config.ts 🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts
 Test Files  1 passed (1)
      Tests  9 passed (9)
```

It proves the half a Rust unit cannot: the fixture's own arithmetic closes (mounted + skipped =
node count); the skipped set is exactly the surface's transitive subtree; `record_consumes_subtree`
names `Surface` and NOT `Tree`, with the paint sync's three tree-row phases still present to say why;
every semantic component has a projection arm; the identity ledger and its React twin both exist; the
surface identities match React's `surfaceHostIdentityV1`; the frame cursor really does run
`Reconcile` between `Ingress` and `Viewport` through the engine entry, with `apply_tree` still
`cfg(test/testkit)` so the reconcile is the only production writer; the probes read the census and the
mounted layout rather than the fields the production path never writes; and the shell resolves one
controller per retained document.

### 5.3 Checks

```
$ cargo check -p semio-framework-ui --features wgpu-engine --lib          # 0 errors
$ cargo check -p semio-framework-ui --features testkit --lib              # 0 errors
$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings   # unchanged from the lane baseline
```

### 5.4 Observed, not mine

* A peer moved the whole wgpu UI target **mid-session**, from
  `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/<file>.rs` to `🖱️ui/🎯️targets/🧊️wgpu/<name>/🦀️.rs`. Every
  edit in this lane survived the move and the peer rewrote this lane's own test-mount path with it;
  the workspace was red for ~6 minutes on `mod icon_name_gen`'s now-wrong relative depth and went
  green on its own. Nothing was reverted and nothing was fought.
* Two further peer refactors then made the WASM build red for the rest of the session, so the final
  rebuild of the §3.5 witness-VALUE diagnostic could not be applied to the serve: first
  `cargo metadata` failed on a missing `💻️os/🧪️testkit/⚖️scale/📦️packages/🦀️rust/Cargo.toml` (which is
  trunk's `pre_build` hook), then `--locked` failed with
  ``semio-framework-artifact-infinite-dag depends on semio-framework-os-kernel with feature `testkit`
  but semio-framework-os-kernel does not have that feature``. Both are in crates this lane never
  touched. The serve left running on 6118 (`screen g3dwgpu`, trunk pid 57461, artifact 10:56) carries
  §2, §3.1–§3.4 and the split refusal messages of §3.5; only the three witness ids inside that
  refusal are compiled-and-checked but not yet served.

---

## 6. The remaining blocker — the raster operation authority

`run-8`/`run-9`, with the split refusal of §3.5, name it exactly:

```
worker-present-failed: raster commit candidate witness was stale
```

— not "already retired", so `RasterTextureStore::candidate` is OCCUPIED and holds a DIFFERENT witness
than the one the retirement is committing. The relevant sites:

| piece | file |
|---|---|
| `commit_presented_step`'s candidate/presenting comparison | `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` |
| `begin_presenting` (arms `presenting` from `candidate`) | same file |
| `admit_engine_texture` (sets `candidate` only when vacant) | same file |
| `AppPresentPhase::{BeginGpu,Engine,Uploads,Render,Acknowledge}` and the witness they thread | `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` |
| `AppPresentedRetirement::step`'s `RasterCandidateRetirement::Commit(witness)` | same file |

The shape to look for: the store's `candidate` is ONE slot shared across frames, `admit_engine_texture`
sets it only while vacant, and `RasterCandidateRetirement` is driven on a different cadence from the
present cursor — so a later frame's engine admission can occupy the slot with a new operation witness
before the earlier frame's commit retirement runs. The witness values are already compiled into the
refusal (§3.5) and will print the three operation ids the moment the serve can be rebuilt.

This is pre-existing code that was **unreachable until now**: it only runs when a document composites
an `engine:<surface>` raster, which had never happened on this target. It is also why the surface now
quarantines where it previously only stayed blank — the honest consequence of the paint path finally
doing real work, and the next thing to fix.

---

## 7. Defect B — guest effects on wgpu: NOT taken on

No line of the effects path was touched, and this lane makes no claim about it. Its state is exactly
`📓️wgpu-blank-paint-2026-09-12.md` §6 left it, re-measured here in `baseline-1` and every later run:

```
[DEBUG] wgpu-bridge renderSurface surface=procedural-main turn=3 effects=0 tags=- intakeSteps=493338
[DEBUG] contributions installed {plugin: procedural, …, effects: 0, tags: -, crossings: 1}
invokeExtension 0×   ·   respond 0×   ·   meshes 0
```

The named next probe is unchanged and still one boot's work: a `[DEBUG]` of `current.effects` inside
`settleInstanceLifecycle` (`🐚️plugin-bridge.ts`), which calls `route.accept(current, execute)` and
re-reads `current` without ever collecting `current.effects` — so a `flowEvalTick` armed while the
instance opens is dropped by the host while the guest's latch stays armed. The wgpu shell must then
dispatch the guest's requested effects after every settle exactly as React's `ShellHost` does
(`setContributions` deferred effects → `applyHostEffects`).

It was deprioritised deliberately: without §4.4 there is no painted preview for a mesh to land in, and
the deliverable-A chain turned out to hold three separate defects (§3.2–§3.4) each of which hid the
next.

---

## 8. Files

**New**
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json` — the neutral oracle:
  the document, the identity rule, the phase ladder, the consume-subtree rule with its reason, the
  eighteen-variant projection table, the surface identities, the space-token table, the expected
  mounted/skipped sets and tree order, and the second-generation identity expectations.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs` — the Rust law (§5.1).
- `📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/{🟦️.ts,laws.json}` — the TypeScript twin (§5.2).
- `<ticket>/🐍️wgpu-reconcile-probe.mjs` — the structure/stats/console probe this lane ran.

**Changed**
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` — the whole `🌳️DocumentTreeReconcile`
  region: the projection, the cursor, `UiTree::step_document_reconcile`.
- `…/🌳️tree/🦀️.rs` — the `🪪️DocumentIdentity` region (`document_nodes` ledger, `clear_links`,
  `attach_child`, `insert_detached`, `remove_detached`, `close_document_binding_step`) and the public
  `mounted_layout` accessor.
- `…/⚙️engine/🦀️.rs` — `UiWindow::document_reconcile`, `Ui::step_document_reconcile`,
  `Ui::publish_document` (testkit), `UiFramePaintCensus` + `Ui::paint_census`,
  `RetainedPaintFrame::{baseline,fault_site}` + `Ui::paint_frame_phase`, and the binding retirement in
  `close_document_step`.
- `…/🖍️draw/🦀️.rs` — `presentation_witnesses` and the four distinct commit refusals (§3.5).
- `…/🧊️gpu/🦀️.rs` — `commit_presented_rasters_step` appends the three witnesses it compared.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — the `Reconcile` frame phase,
  the `controller_id` parameter, `document_debug_log` and the two terminal-fault lines, and the two
  introspection builders now reading `mounted_layout`/`paint_census`.
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `document_controller_id` and the
  two document call sites that pass it.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts` — registers the twin.

No `launch.json` entry was added: `procedural3d-wgpu` already exists and is the row this lane ran, and
the new vitest suite runs inside the existing `@semio-tech/framework-renderer-wgpu:test` target.

**Temporary logs still in the tree** — the two `[DEBUG] ui-doc {reconcile,paint} fault …` lines in the
Interpreter. They are the only trace a terminal retained-document fault leaves inside the Worker (§3.5
explains why); remove them together with the `fault_site` plumbing if that diagnostic is ever
considered shipped noise, but note that removing the plumbing alone makes the next fault of this class
undiagnosable again.

## 9. Rebuilds and the serve

The renderer wasm was rebuilt six times, through `trunk build --config Trunk.toml` in the foreground
followed by `screen -dmS g3dwgpu <ticket>/📜️serve-generation3d-wgpu.sh` — always verifying the new
build actually reached `⚡️cache/📺️renderer-modules/🧊️wgpu/` by grepping the served `_bg.wasm` for a
string only that build carries. Two probe runs in this lane were against a STALE dist before that check
was adopted; both were re-run. `🎞️frame-worker.js` was not regenerated (no frame-worker TypeScript
changed). Shared cargo build dir throughout, `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`.
