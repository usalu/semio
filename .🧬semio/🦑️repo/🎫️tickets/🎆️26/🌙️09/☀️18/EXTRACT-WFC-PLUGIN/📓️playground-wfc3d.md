# 🧊️ wfc3d playground — end to end (slice B5)

Variant `wfc3d`, react renderer, coordinator's serve on `http://127.0.0.1:6045/?plugin=wfc`.
Probes: `🐍️wfc3d-console-dump-probe.mjs`, `🐍️wfc3d-interact-probe.mjs`, `🐍️wfc3d-viewer-probe.mjs`.
Outputs: `🗑️generated/playground-wfc3d/`. Logs of every cargo/nx run in the same folder.

---

## 1. What the baseline actually was

The first boot probe (`🗑️generated/playground-wfc3d/wfc3d-boot/`) already looked healthy: shell ready
(`data-semio-os-ready=wfc3d`), both windows mount a canvas, the World3d preview carries 3 meshes and
**3 instances** with `status_json` = `Solved 3 of 3 slots.` — so the **inline per-render solve does
run in the browser** and stays inside the interactive budget (no `interactive-ceiling`, no
quarantine, over hundreds of repaints across every probe run). That half of the brief needed no work.

Everything the user could *touch*, however, was dead. The baseline interact probe
(`🗑️generated/playground-wfc3d/interact-baseline/`) found four distinct faults:

1. **`setActiveExample refused: undeclared-action`** — no window kind declared it, exactly as the
   coordinator predicted from the bitmap boot. The example picker was inert.
2. **`change-seed refused: dispatch-failed — action 'change-seed' is not a framework-reserved action
   … dispatched exclusively through the typed command channel`** — wfc3d had **no
   `command_from_action`**, so the trait default refused *every one* of the nine declared window
   actions. The whole Actions pane was dead, not just the example picker.
3. **`interactionHover` / `interactionSelect` refused: undeclared-action** on every pointer move and
   every click over the graph canvas — the app declared no interaction domain, and the framework only
   injects those six verbs when `AppDefinition.interactions` is non-empty.
4. **A node drag and a wire gesture reached the plugin as nothing at all** — `nodeGraphEdit` was
   neither declared nor bridged, and the three slot nodes were drawn one *pixel* wide (see §3).

A fifth fault only appears once the first two are fixed:

5. **`setActiveExample refused: dispatch-failed — typed command 'setActiveExample' has no exact
   controller/owner/factory/tool/schema proof`** (`interactive-job.missing-factory`). `Migrated` is
   the only live classification, so every dispatch runs the retained route, and wfc3d had no
   `TOOL_JOB_IDS`, no publication contracts, no `bounded_first_step_tool_proofs!` and no factory.
   `ActionDefinition::bounded_catalog` builds a catalog row **without** execution authority; the
   authority is the exact app-owned factory, and nothing else grants it.

---

## 2. What landed

All in `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/…/`. The shared `wfc-graph` window module
(`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs`) carries **no wfc3d change at all**: it is a
verbatim mirror of wfc2d's copy (re-mirrored after B3 added the staged argument forms; `diff` empty,
tests file too). Every wfc3d-specific decision lives in the caller, which is what that file's own
contract asks for.

### `✏️editor/🦀️.rs`

- **`command_from_action`** for all 21 verbs (the nine graph mutations, the six tile/rule verbs, the
  two config verbs, `setActiveExample`, `nodeGraphEdit`, `nodeGraphViewport`) — typed against
  `dsl::DslValue`, never `serde_json::Value` (pitfall #13).
- **`wfc3d_command_id`** + `OpBinary::TOOL_JOB_IDS` + `WFC_3D_PUBLICATION_CONTRACTS` +
  `Wfc3dCommandWork` + `Wfc3dRetainedCommandJobFactory` + `register_tool_job_factories` +
  `build_tool_job` + `bounded_first_step_tool_proofs!` — the exact-owner proof chain, ported from the
  sibling `◻️2d` recipe (slice B3 wrote it first; wfc3d's differs only in having no `solve`
  transient route).
- **`build_artifact_store_one_item_preparation_factory`** and its config twin. Without them every
  route declaring the `Artifact`/`Config` publication lane registers as *unsupported* and stays
  dispatch-dead whatever its classification says.
- **`SetActiveExample`** → `example_snapshot(id)` → `Effect::LoadDocument`, plus
  **`build_document_store_initialization_job`** (the trait default REFUSES the envelope, so without
  it every example switch faults at the archive-load boundary). A `LoadDocument` is not an edit, so
  re-picking the boot example mints no phantom undo entry — pitfall #3 is structurally avoided rather
  than diffed around. The empty `exampleId` the shell sends for "the app's default document" resolves
  to the boot example; an unregistered id faults (`wfc3d.example.unknown`).
- **`NodeGraphEdit { operations_json }`** and `graph_edit_mutations` (see §4).
- **`NodeGraphViewport { x, y, zoom }`** — its own verb rather than `change-camera`, because
  `dispatch_typed_command_inner` rejects a command whose `command_id` is not the action it was
  admitted under. Emits a config mutation only, so a camera move never enters undo.
- **One `InteractionDefinition`** (`slot`, granularity `slot`, `HierarchyProvider::Flat`-shaped flat
  topology) plus `interaction_topology` over the document's slot ids. Declaring the domain is what
  injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
  `setInteractionGranularity` onto every window kind and ends fault 3.
- **`WFC_3D_GRAPH_UNIT = 120.0`** and its exact inverse `slot_coordinate` (see §3).

### `👁️viewer/🦀️.rs`

The navbar example picker is resolved by DIALECT, so the shell offers it in the **viewer** role too
and announces the boot example there — which the viewer drops as `undeclared-action`.

I built the viewer half of that (`setActiveExample` + bridge + a one-tool retained factory) and then
**removed it again**, because it cannot work and the compiler says so: `ArtifactViewer` declares no
`build_document_store_initialization_job` and `ViewerApp` forwards none, so a viewer is STRUCTURALLY
incapable of admitting the whole-document `Effect::LoadDocument` an example switch is. Every viewer in
this repo drops that verb for the same reason (`📸️remodel`'s viewer says so in as many words). Both
surfaces of this dialect boot on the same example, so the announcement has nothing to change anyway.
What landed instead is the reasoning, written down at the top of `👁️viewer/🦀️.rs`, and one narrow,
explained exception in the viewer probe so a real regression is not buried under a known line.

### `✏️editor/…/🪟️windows/🧊️preview/🦀️.rs`

Two real GL bugs in the mesh catalogue, found by reading the live console rather than the code:

- `normals` was shipped **empty**. The host binds it unconditionally at item size 3
  (`geometryFromMesh` in `🌐️World3dHost/🟦️.tsx`), so a zero-length buffer made every instanced draw
  die with `GL_INVALID_OPERATION: glDrawElements: Vertex buffer is not big enough for the draw call`
  — hundreds of times per run. `vertex_normals(positions, indices)` now computes area-weighted
  per-vertex normals, falling back to `+Z` for a degenerate mesh (a zero normal is the same crash
  with a different cause).
- `colors` was shipped **RGBA**. The host binds `color` at item size 3, so every vertex' colour was
  shifted by one channel. Now RGB.

---

## 3. Why the graph pane needed a scale

A wfc3d slot is a **box in metres**: `two-room-corridor` authors three 1×1×1 slots at x 0/1/2. The
`wfc-graph` canvas takes node rectangles in canvas units at viewport zoom 1, so the pane opened on
three one-pixel nodes stacked inside the canvas' own minimum node size — a smudge at the centre of an
empty grid (`🗑️generated/playground-wfc3d/wfc3d-boot/final.png`).

`Wfc3dGraphView` now projects `x`/`y`/`width`/`height` through `WFC_3D_GRAPH_UNIT` (120 canvas units
per document unit) and `graph_edit_mutations` divides by it again on the way back, so a released drag
lands a `move-slot` in **document** units. The scale lives in the caller, so the shared window file
stays byte-identical to wfc2d's; wfc2d does not carry it yet (see §6).

---

## 4. The gesture → mutation path, as the live canvas actually uses it

`wfc-graph` renders on the **wasm** node-graph surface (`WasmGraphSurface`), not the React Flow
fallback — `useFlowEngine` is false for a scene with no `hostSnapshotJson` and no flow capabilities.
That matters, because the two surfaces speak different gesture vocabularies:

| surface | a released drag dispatches |
|---|---|
| React Flow fallback / wgpu renderer | `nodeGraphEdit {operations:[{operation:"move",nodeId,x,y}]}` |
| wasm `WasmGraphSurface` | `nodeGraphEdit {operations:[{operation:"setHostSnapshot",hostSnapshotJson}]}` |

`graph_edit_mutations` handles **both**. `move`/`connect`/`disconnect` map straight through;
`setHostSnapshot` is diffed against the document (`host_snapshot_mutations`):

- a node whose projected position moved by more than `WFC_3D_GRAPH_EPSILON` → one `move-slot`,
  **keeping the slot's authored `z`** (the graph is a plan and never saw the third axis, so reading
  `z` back off the document is the only thing that stops a drag flattening a stack);
- an edge in the commit that the document does not have → one `connect-slots` with a deterministic
  `edge-{from}-{to}` id, endpoints resolved through `{node}@{port}`;
- a document edge missing from the commit → one `disconnect-slots`;
- an untouched graph → **nothing**, so a plain click mints no edit.

One gesture is one `Emit`, hence one undo entry.

> 🩹 The wasm surface only started committing an edited gesture at all when slice B3 landed
> `graphEditSignature`/`commitGraphFixtureIfEdited` in `🕸️NodeGraph/🟦️.tsx` (uncommitted framework
> change, verified live in the served module). Before that a drag on that surface reached no plugin
> in any wfc artifact.

---

## 5. Verification — every command run, and what it said

| gate | result |
|---|---|
| `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-wfc-3d --features component-app-assembly --lib -j 4` | ✅️ **239 passed, 0 failed, 2 ignored** (`🗑️generated/playground-wfc3d/test-10.txt`), eleven of them new laws for this slice |
| `cargo check -p semio-s-plugin-wfc --lib -j 4` | ✅️ (`check-plugin-final.txt`) |
| `activate-wfc3d-react-dev` (react dev restage) | ✅️ (`activate-7.txt`); the staged wasm was grepped for this slice's own new strings before every browser run |
| boot probe | ✅️ **0 fault lines** (`🗑️generated/playground-wfc3d/boot/`) |
| interact probe | ✅️ **10 steps, 0 failing** (`🗑️generated/playground-wfc3d/interact/`) |
| viewer probe | ✅️ pass (`🗑️generated/playground-wfc3d/viewer/`) |

### What the live app now does, measured

- **Boot.** Shell ready, `wfc-graph` mounts two canvases, `wfc-3d-preview` mounts one with 3 meshes,
  **3 instances** and `Solved 3 of 3 slots.`; camera framed on the document (`target [1.5,0.5,0.5]`).
- **All three examples load and solve** through the navbar picker, each with real geometry:
  `two-room-corridor` 3/3, `wall-roof-facade-strip` 4/4 (`bay-0-ground=tile:roof@0,0,0`, …),
  `tower-stack` 5/5 (`cantilever=tile:pier@1.5,3,0`, `storey-3=tile:cap@0,9,0`). Switching is a
  `LoadDocument`, so it mints no history row.
- **A node drag lands exactly one `move-slot`, in document units, z kept**: dragging `room-a` down
  240 canvas px moved its preview instance from `@0,0,0` to **`@0,2,0`** (240 / `WFC_3D_GRAPH_UNIT`),
  and the solve re-ran on the new document.
- **A wire gesture reaches the document**: the preview's assignment changes with it, and **undo
  reverts it** — three `mod+z` presses put `room-a` back to `@0,0,0` *and* `room-b` back to
  `tile:corridor`, so both gesture edits were journaled and revertible.
- **The Actions pane is alive**: all 21 wfc3d verbs plus the framework's own rows are listed, and
  since the shared window gained staged argument forms a row now opens its form instead of firing an
  empty-id dispatch.
- **The viewer role renders the solved assembly**: 1 canvas, 3 meshes, 3 instances,
  `Solved 3 of 3 slots.`, no faults.
- **The preview paints solid, tinted bodies** rather than the bare wireframes the empty-normals bug
  left behind, and the GL error storm is gone.
- **No `interactive-ceiling`, no quarantine, no guest trap** across every run, over hundreds of
  repaints — the inline per-render solve stays inside the interactive budget at these document sizes,
  so no tool-run migration is needed (revisit if a document reaches hundreds of slots).

### 🕳️ The one thing not proven live

A wire gesture that lands a **`connect-slots`** specifically. The headless drag reaches the document
(see above) but could not be made to anchor on the wasm canvas' port hotspot: pressing at the node
centre ±11, ±14 and ±19 px all grab the node BODY or the canvas, and the commit that comes back
describes an adjacency REMOVAL rather than an addition. The lowering itself is pinned by unit tests
over the exact commit payload the surface sends
(`a_host_snapshot_commit_lands_a_new_wire_as_connect_slots`,
`a_connect_gesture_lands_one_connect_slots_and_never_duplicates`). What is missing is a reliable way
to address a port from a headless pointer — the canvas exposes no DOM node or hover read-back for it
(`data-selection-json` mirrors the published scene, not the live canvas hover), so the probe cannot
locate it. Suggested next step for W2: expose the wasm surface's `pickTargetsAtScreenJson` on the
host element (or a `data-hover-json`), which makes every canvas gesture addressable by every probe in
the repo, not just this one.

---

## 6. For B3 / W2

- The shared `🕸️graph/🦀️.rs` is untouched by this slice; `diff` against wfc2d's copy is empty.
- **wfc2d almost certainly needs `WFC_3D_GRAPH_UNIT`'s equivalent**: its slots are authored in the
  same small units, so its graph pane will render the same one-pixel nodes. The pattern to copy is
  `Wfc3dGraphView` + `slot_coordinate` in `✏️editor/🦀️.rs` — caller-side, so the window file stays
  shared.
- **wfc2d's `wfc2d_node_graph_edit` handles only `move`/`connect`**, which the wasm surface never
  sends. It needs the `setHostSnapshot` branch (`host_snapshot_mutations` here) or its drags will
  keep reaching no document.
- **Key edge removals on the ENDPOINT PAIR, never on the edge id.** The canvas mints its own wire ids,
  so an id-keyed diff reads every authored edge as deleted: one wire gesture silently disconnected the
  graph and the solve quietly re-coloured around the hole. Both the add and the remove rule are
  unordered here (`a_host_snapshot_commit_never_disconnects_an_edge_it_only_renamed`).
- **The shared window's staged forms are 2d** (`x`/`y`, `width`/`height`). wfc3d therefore made its
  `move-slot`/`resize-slot` command carry `Option<f64>` for `z`/`depth`, falling back to the slot's
  authored value — a 2d form must not be able to flatten a stack. If wfc2d ever gains a third axis,
  copy that shape rather than widening the shared form.
- I re-mirrored `🕸️graph/🦀️.rs` from wfc2d after B3 added the argument forms; `diff` is empty again
  as of this slice's last run.
