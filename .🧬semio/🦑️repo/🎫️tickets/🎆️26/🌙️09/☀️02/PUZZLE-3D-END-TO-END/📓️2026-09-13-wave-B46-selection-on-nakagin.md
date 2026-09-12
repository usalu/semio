# Wave B46 — establishing a selection on the 180-object Nakagin document

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B46, 2026-09-12 (report file dated 2026-09-13 per the
coordinator's naming). Written incrementally while the wave ran.

Predecessor this wave is answering: `📓️2026-09-13-wave-B44-mutation-latency-large-document.md` §6.2 —
*"No world selection can be established on the 180-object document"*, plus its residual 5
(`interactionSelect` still 20× at 180 objects). Further context: `📓️2026-09-11-wave-B9-guest-lanes.md`
§2 (typed-vs-projection, `interaction_topology`, `validate_state` pruning),
`📓️2026-09-12-wave-B18-probe-hardening.md` (outliner rows → `interactionSelect`),
`📓️2026-09-09-wave-O-outliner-and-arena.md` (the panel arena),
`📓️2026-09-10-wave-P5-lanes-render.md` (paged lane set).

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with peers. No
  `git commit`/`stash`/`checkout`/worktree was run; the ticket is NOT closed; nothing under
  `🗑️generated` was deleted. `RUST_MIN_STACK=134217728` on every cargo invocation; no
  `CARGO_TARGET_DIR`/`RUSTC_WRAPPER` was set.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- React serve live on `127.0.0.1:6013` — a **Vite dev server** (`/@vite/client` in the served
  `index.html`), so the host TypeScript halves are **live off the repo** and every host fix below was
  measured in the browser with no rebuild. The guest halves rode the coordinator's **wasm #57** for this
  wave's diagnostic runs and **#58** for the battery subset (#58's release artifact landed 16:55, after
  those runs). **None of this wave's Rust changes are in either wasm** — they need a #59; the guest-side
  numbers below are native (§6.1) and the browser numbers are host-side (§6.3).
- The probe slot (`pgrep -f 'bun .*browser-probe'`) was polled and found empty before every browser run;
  the coordinator's own batteries held it for most of the wave.
- Peers were mid-refactor in two shared crates during the wave and broke the workspace twice
  (`semio-framework-plugin` `E0004` on new `AppCommand::*DocumentArchiveLoad` variants,
  `semio-framework-ui-runtime` `E0061` on `release_empty_page`). Both resolved themselves; every number
  below was taken on a green build of the crate it names.

## 1 What was already true, and what the defect is NOT

Four laws were written first, to find out whether the guest can select on this document at all. All of
them pass on wasm-source Rust (§6.1), and they close off the hypotheses B44 handed over.

| hypothesis from the brief | verdict |
| --- | --- |
| the outliner publishes ZERO object rows for 180 objects (`PanelRowBudget`/arena capacity) | **false in the guest.** `document::render` on the real `nakagin_fixture()` presents **7 object rows + 1 `+173` continuation, 13 nodes** — `[DEBUG] b46.outliner nakagin objects=180 nodes=13 objectRows=7 continuations=1` |
| the same through the route the host actually asks for (`render_panel_body`, no `window_id`) | **also false.** `[DEBUG] b46.panelBody objects=180 firstRowPresent=true continuation=true bytes=7568` |
| `interaction_selection_loss_v1` verdict `validate-state-pruned` — the topology is narrower than the render (B9's class) | **false.** `ArtifactApp::interaction_topology` and `render_fixture` are the SAME decode (`puzzle3d_fixture_from_projection`, `✏️editor/🦀️.rs:323` / `:3273`), and a pick by id on the 180-object document persists: `[DEBUG] b46.pick objects=180 picked=01890804-… worldSelected=["01890804-…"]`, and Inspection names it |
| topology pruning of ids not in the typed document | **false**, same evidence |

So the guest can be selected on this document by id, the outliner body it publishes carries selectable
rows, and the row's `interactionSelect` args are intact. Everything that follows is therefore about the
HOST half and about the per-pick cost.

## 2 `interactionSelect` at 180 objects — the O(n) term, found and removed

### 2.1 The hop

| hop | file:line | cost at 180 objects |
| --- | --- | --- |
| `interactionSelect` → `dispatch_interaction_action`, `InteractionVerb::Select` arm resolves the domain topology | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22676` | 1 full fixture decode + ~900 `TopologyNode`s |
| the same call, again, inside `revalidate_and_persist_interaction_state` → `build_full_interaction_topology` | same file, `:22589` | **a SECOND full decode + ~900 nodes** |
| `protocol::validate_state` prunes each selected and each hovered id with `DomainTopology::contains` | `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:2318` | one **linear scan of ~900 nodes per id** |

`A::interaction_topology` for puzzle3d (`✏️editor/🦀️.rs:8031`) decodes the whole fixture from the
projection and emits one `TopologyNode` (two `String`s each) per object, per vortex, per attraction, per
target volume, per reference and per object kind — on Nakagin ~900 nodes. Nothing between the two call
sites can move the document, so the second build is pure waste, and it is paid by every one of the six
framework interaction verbs.

### 2.2 The fixes

1. **`Puzzle3dApp`-independent topology memo** (`🔌️plugin/🦀️.rs`): new
   `interaction_topology_memo: Option<((u64, u64), protocol::InteractionTopology)>` plus
   `app_interaction_topology()`, keyed on `(store.generation(), config_store.generation())` — the pair
   every input of `A::interaction_topology` is derived from, `live_render_operation()` included — so a
   mutation invalidates it in the turn it lands. `resolve_domain_topology`'s `Topology` arm now reads the
   memo. The app builds ALL domains in one pass, so this also stops a multi-domain app from re-deriving
   the same forest once per domain.
2. **`DomainTopology::membership()`** (`📡️wire/🦀️.rs`) — one `BTreeSet<&str>` over the topology, built
   once per domain in `validate_state` and shared by the selection prune and the hover prune. Pruning is
   now `O(n + selected·log n)` instead of `O(selected·n)`; a marquee selection was quadratic.

### 2.3 Measured

`[DEBUG] b46.topology small=1objects/1builds large=180objects/1builds` — one pick builds the app
interaction topology exactly ONCE, and the count is identical at 1 and at 180 objects. The law
(`one_pick_builds_the_interaction_topology_once_whatever_the_document_size`) asserts both.

## 3 B44 §6.2 is not a product defect — it is two reader defects

Both halves of "no world selection can be established on the 180-object document" are artifacts of how
B44's own probe read the page. Measured on the live serve (`🔍️b46-selection-nakagin.ts`, outputs
`🗑️generated/b46-2026-09-12T15-22-06-diag.md`, `…15-40-…-guestlane.md`, `…15-47-57-census.md`).

### 3.1 The outliner DOES publish object rows on Nakagin

```
[261.62s] outliner panels=[{"anchor":"top-left","tab":"framework.panel.artifact","visible":"true","h":282}, …]
[261.62s] outliner treeRows=13 entityRows=7 first={"id":"panel:puzzle3d-play-document/01890804-66f2-4544-98f0-b6f0c0615492","text":"Capsule With Balcony J Hide Lock"}
[261.62s] outliner ids=["puzzle3d-play-document","panel:puzzle3d-play-document/puzzle3d-play-document.objects",
          "panel:puzzle3d-play-document/01890804-…", … ,"panel:puzzle3d-play-document/puzzle3d-play-document.objects.more", …]
```

Seven object rows, each carrying its Hide and Lock row actions, closed by `…objects.more` — exactly the
13-node page the guest builds natively (§1). B44's probe read `entityRows=0` with the SAME selector; the
difference is that it waited 2 500 ms after clicking the tab (and only inside its
`if (!selected.length)` fallback branch, after a long mutation sequence). Four seconds is enough. So it
is a settle artifact, not a missing page — **the outliner was never the second route that failed.**

### 3.2 The pick DOES select — B44's probe read the wrong attribute keys

`worldSurfaceSelectionDomV1` publishes `selectedIds`/`activeObjectId`
(`🌐️World3dHost/🟦️.tsx:1494`). `🔍️b44-mutation-latency.ts`'s `selectionOf` reads `parsed.ids` and
`parsed.activeObjectId` — so on EVERY pane of EVERY document it read `[]`, whatever was selected. The
Concrete Forest success B44 contrasted against came from a DIFFERENT reader
(`🔍️browser-probe.ts`'s `worldInteraction()`, which reads `selectedIds`), which is why the two
documents looked different when the product did not.

What the pane actually publishes after one canvas click at pane fraction 0.50/0.50 on Nakagin:

```
{"selectedIds":["e80dc9d0-a547-42f8-943d-b3df6a4165c3"],"activeObjectId":"e80dc9d0-a547-42f8-943d-b3df6a4165c3",
 "targetVolumeIds":[],"referenceSelectedId":null,"hoverTarget":{"domain":"object","id":"e80dc9d0-…"},
 "hoveredVortexFullId":null,"hoveredKindId":null,"gumballActive":true,"gumballTarget":[-8.85,-12.55,26.95],
 "transformMode":"move","activeUtility":"select"}
```

This wave's probe is the corrected reader, and it says so in its own docstring so the next wave cannot
re-derive the same false negative.

## 4 The canvas pick — the defect that IS there, and its fix

### 4.1 The hop table (measured with three temporary `b46.*` host taps, removed before finishing)

| hop | file:line | Nakagin, before | Nakagin, after |
| --- | --- | --- | --- |
| R3F pointerdown on the GLB instance group → `handleInstancePointerDown` | `🌐️World3dHost/🟦️.tsx:5600` | **works** — `b46.pick.instance id=e80dc9d0-… index=161 disabled=false domain=vortex merge=replace` on 14 of 20 grid points | unchanged |
| pointerup fallback `resolveClickInstanceId` (also the MARQUEE's and the RELOCATE grab's only route) | `:3946` / `:4016` / `:6051` | **blind — 0 of 20**: `candidates=180 containing=0 hit=none boxes=["28x30@427,394","27x30@450,441","41x38@406,184"]` | **`candidates=180 containing=14 hit=e80dc9d0-… boxes=["97x88@392,349","95x88@415,395", …]`** |
| `interactionSelect` ingress → settle | — | ingress at +40 992 ms, `interactionSelectSettled=2` in the following 150 s | unchanged |
| the pane's own `data-selection-json` | `:1494` | `selectedIds:[picked]`, **`activeObjectId:null`** | `selectedIds:[picked]`, `activeObjectId:[picked]` |
| the GUEST's own lane `data-guest-selection-json` | `:1523` | **`selectedIds:[]` at every sample of a 12-minute run** | unchanged — §5 |

### 4.2 Root cause of the blind fallback

`resolveClickInstanceId` and `resolveMarqueeInstanceIds` built each instance's screen-space AABB from
`meshById.get(meshId)?.data` — and a URL-backed mesh record has NO inline `data`: the guest publishes
`{id, url}` for every catalogue representation (`world3d_meshes_json_from_kinds_and_urls`,
`🔌️plugin/🦀️.rs:36039`). The pick therefore fell back to a **unit cube** and the marquee to the
**single point `[0,0,0]`**. The tap's own census proves both halves of it in one line:
`meshesWithData=2` of the whole mesh set (the procedural fallback kind plus `vortex-marker`), and every
projected box **28 × 30 px** for capsules that render 97 × 88 px. Twenty clicks spread over the whole
pane contained **zero** candidates.

That is also the most likely reading of B44's residual 3 (`relocate-pose-delta` fails at ONE object):
`beginRelocateDrag` (`:6051`) grabs through the same function, and at one object
`world3dRelocateDragTargetV1` covers the miss from the live selection, so the gesture only looks
size-independent.

### 4.3 The fix

`GLB_MESH_LOCAL_BOUNDS` + `glbMeshFrameCorners` + `world3dInstanceLocalCorners`
(`🌐️World3dHost/🟦️.tsx`, region `🔖️GlbMeshBounds`). `GlbInstanceMesh` — the only place that knows a
GLB's real size — records the eight corners of the loaded scene's AABB **after** the
`GLB_MESH_FRAME_ROTATION_X` frame rotation the instance group applies, keyed by the mesh record's own
`url`. Both hit tests now ask `world3dInstanceLocalCorners(meshData, meshById.get(meshId)?.url)`:
inline geometry first, the recorded GLB extents second, the unit cube only while a GLB is still
loading. The marquee's degenerate single point is gone with it.

Second fix, same region: `mergeWorldSelectionWithLeftoverV1` (`:1453`) now carries
`activeObjectId` with the ids it replaces. The overlay exists to make a pick visible before the guest's
lane answers, and `ids` alone only covers the readers that take a list — the gumball target,
Inspection's focus and `data-selection-json`'s own `activeObjectId` all read that single field. The
base's active id is kept whenever the overlay still names it.

## 5 What is still broken on the large document, precisely

The pick lands, the framework persists it (**zero** `interaction selection lost` records in any run) and
the pane paints it. What does NOT happen is the guest's own re-publication. The census over the 150 s
after one pick:

```
[194.19s] pick 0.50,0.50 census lines=285 interactionSelectIngress=6 interactionSelectSettled=2
          registerBrushMesh=123 settledAny=46 refreshUi=0
[194.19s] pick 0.50,0.50 guest-selection-json=[{"surface":"framework.window.puzzle3dMainTop",
          "raw":"{\"selectedIds\":[],\"activeObjectId\":null,\"hoveredId\":null,\"gumballActive\":false}"}, …]
```

Two facts, both new:

1. **`interactionSelect` settles and the guest still never republishes its selection lane.** The
   temporary `b46.panel` tap (one line per SUCCESSFUL projection of `puzzle.3d.play.document`) was
   silent for the whole 150 s after the pick AND for the 150 s after an outliner row click, while the
   declared scope for `InteractionVerb::Select` names that body
   (`puzzle3d_selection_scope`, `✏️editor/🦀️.rs:2269` → `main::BODY_KEY` + inspection + document +
   history). `ownedUiRefreshResponse`'s `uiRefreshSectionUnchanged` short-circuit
   (`🔌️PluginRuntime/🟦️.tsx:1958`) is upstream of that tap, so the reading is: **the guest's retained
   surfaces were not re-rendered after the pick, so the host kept the pre-pick bodies.** Every
   selection-scoped symptom on this document — Inspection empty, `gumball-scene-delta`,
   `duplicate-selection`, `delete-selection` — is downstream of exactly that.
2. **`registerBrushMesh` never drains on this document.** 123 of the 285 console lines in that window
   are `registerBrushMesh` (`Background` lane), at seq 22 → seq 124 over 306 s, still arriving 8 minutes
   after the example switch. B44 measured "24 commands for the 14-mesh Nakagin catalog, ~300 ms each,
   and they settle" — at #57/#58 they do not stop. A permanently busy guest is the simplest explanation
   for (1) and it is wave H's re-announce loop (`📓️2026-09-10-wave-H-brush-mesh-reannounce.md`).

Both are handed over with those numbers (§8).

## 6 Laws and verification

### 6.1 New laws

| law | file | what it pins |
| --- | --- | --- |
| `the_nakagin_outliner_page_carries_selectable_object_rows` | `✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | the flagship page presents object ROWS, each with its `interactionSelect` activation binding and both `setSelectionFlag` row actions, closed by a `setPanelPage` continuation. The pre-existing Nakagin law only asserted "four sections", which is why a page of zero rows would have read green |
| `a_pick_by_id_on_the_flagship_document_persists_and_reaches_inspection` | `✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs` | a pick by id on the 180-object document survives `validate_state` and reaches both the world lane and Inspection |
| `one_pick_builds_the_interaction_topology_once_whatever_the_document_size` | same | the per-pick topology work is the same at 1 and at 180 objects, and is exactly one build |
| `the_flagship_outliner_panel_body_carries_paged_object_rows` | same | the same page through the route the host asks for (`render_panel_body`, no `window_id`) |
| `🎯️ world-3d pick bounds` (10 laws) | `🧑‍🎨engine/🧪️tests/🎯️world3d-pick-bounds/🟦️.ts` + `🌐️World3dHost/🧫️fixtures/🎯️pick-bounds.json` | inline geometry wins, a URL-backed mesh uses the loaded GLB's recorded extents, the unit cube stands in only while a GLB is loading, an EMPTY inline mesh does not shadow the recorded extents, a unit-cube-sized candidate misses the click a GLB-sized one contains, and the leftover overlay carries `activeObjectId` with the ids it replaces (4 cases) |

The TypeScript suite is registered in `⚛️react/📦️packages/🟦️typescript/vitest.config.ts`'s
`engineTestSuites` — a co-located suite in no include list is a gate that reads green while measuring
nothing (wave B38).

### 6.2 Commands, all foreground

| command | result |
| --- | --- |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | **0 errors** (`Finished dev profile … in 40.44s`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests` | **0 errors** (`… in 47.19s`) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | **0 errors** (`Finished dev profile … in 1m 09s`) |
| `cargo check -p semio-framework-replication` | **0 errors** |
| `RUST_MIN_STACK=… cargo test … --lib selection_scale -- --test-threads=1` | **3 passed / 0 failed** |
| `… --lib outliner` | **10 passed / 0 failed** |
| `… --lib document::tests` | **9 passed / 0 failed** |
| `… --lib interaction` | **13 passed / 0 failed** |
| `… --lib selection` | 24 passed / **1 failed** — a peer's (§6.4) |
| `… --lib nakagin` | 26 passed / **1 failed** — B42's (§6.4) |
| `cargo test -p semio-framework-replication --lib -- --test-threads=1` | **270 passed / 0 failed** |
| `SEMIO_TEST_LEVEL=long bun x vitest run` (renderer-react engine, 32 suites) | **1 011 passed / 0 failed**; one failing FILE, `🧩️package-integration`, on a peer's missing `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script` (B44 §7.1's own entry) |
| `SEMIO_TEST_LEVEL=long bun x vitest run --testNamePattern='world-3d pick bounds'` | **10 passed** |
| `bun x tsc --noEmit -p .` (renderer-react) | **775 `error TS`**, the same count as the baseline taken before this wave's first edit; **zero** in any file it touched |

```
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 742 filtered out; finished in 1.82s
[DEBUG] b46.pick objects=180 picked=01890804-66f2-4544-98f0-b6f0c0615492 worldSelected=["01890804-66f2-4544-98f0-b6f0c0615492"]
[DEBUG] b46.topology small=1objects/1builds large=180objects/1builds
[DEBUG] b46.panelBody objects=180 firstRowPresent=true continuation=true bytes=7568
[DEBUG] b46.outliner nakagin objects=180 nodes=13 objectRows=7 continuations=1
```

```
 Test Files  1 failed | 31 passed (32)
      Tests  1011 passed (1011)
```

### 6.3 Before / after in the browser (host-live, same serve, minutes apart)

| measurement, Nakagin, pane fraction 0.50/0.50 | before | after |
| --- | --- | --- |
| `resolveClickInstanceId` candidates containing the click | **0 of 180** (20 of 20 grid points) | **14 of 180** |
| the id it resolves | `none` | `e80dc9d0-a547-42f8-943d-b3df6a4165c3` |
| the projected box it tests | `28x30@427,394` | `97x88@392,349` |
| `data-selection-json.activeObjectId` after the pick | `null` | `e80dc9d0-a547-42f8-943d-b3df6a4165c3` |
| outliner rows on the artifact panel | 7 object rows + `+173` (unchanged — never broken) | same |

### 6.4 The two native failures that are NOT this wave's

| failure | attribution |
| --- | --- |
| `gumball_active_only_for_transform_utilities_with_object_selection` — `an unattached gumball must never render, left: Some(true)` | **a peer's.** Bisected: it fails identically with this wave's topology memo forced OFF, and `validate_state`'s new membership set is semantically identical to the `contains` loop it replaced (a `None` topology still keeps every id). `git diff --stat` over the crate shows **+469 lines uncommitted in `🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`** and +190 in `📌️panels/🔍️inspection/🦀️.rs` — the selection/gumball render path itself, mid-refactor |
| `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin` | **B42's**, already attributed in B44 §7.1 (it measures `✏️editor/⏳️precompute/🦀️.rs`, which this wave never touches) |
| `semio-framework-plugin --lib`, 9 `app_builder_tests`/`child_member_registry` failures | **a peer's**: `app-definition.interactive-job-classification: unclassified interactive command 'main:addLayer'` — an in-flight interactive-job classification change, nothing this wave declares |
| `@semio-tech/ui-react` 14 failures (icon hover keyframes, control chrome, UIDialog, Diagram, tutorial engine) | **peers'**; none in a file this wave touched |

## 7 Live evidence

### 7.1 The coordinator's battery subset — guest on wasm #58, host live off the repo

`bun 🔍️browser-probe.ts --only=example-switch,selection-surfaces,outliner-rows,selection-keybindings --port=6013`
→ `🗑️generated/probe-2026-09-12T16-09-07.md` / `.ndjson`, log kept as
`🗑️generated/b46-battery-subset.txt`.

```
battery PASS=17 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
done booted=true faults=0 hard=0 collateral=0 verdicts=34
```

Every verdict green, selection-scoped ones included: `selection-precondition`,
`inspection-object-fields`, `inspection-locked-flag-row`, `outliner-panel-opens`,
`outliner-hide-control-present`, **`outliner-hide-applies`**, **`outliner-show-restores`**,
`duplicate-selection`, `duplicate-reselects-clone`, `focus-selection`, `delete-selection`,
`guest-alive-mutate`, `example-switch-instances`, `guest-alive-replace`. The two bold ones are the pair
that FAILED in the coordinator's own #57 and #58 full batteries
(`verdict outliner-hide-applies FAIL rowChanged=false … waitedMs=30132`).

The selection a pick established, quoted off the reader the verdicts use:

```
[17.0s]  selection precondition attempt=0 via=pane-fraction ids=["seed-left-001","seed-left-001"]
         state=["puzzle3d-main-top=seed-left-001","puzzle3d-main-perspective=seed-left-001"] waitedMs=26
[42.8s]  keybindings precondition attempt=0 via=pane-fraction ids=["object-1","object-1"]
         state=["puzzle3d-main-top=object-1","puzzle3d-main-perspective=object-1",
                "panel:puzzle3d-play-document/object-1=Hexagonal Cut Concrete Forest Left Hide Lock"] waitedMs=1
```

and the outliner's own rows:

```
[23.8s] outliner rows before=[{"id":"puzzle3d-play-document","text":"OBJECTS Hexagonal Cut Concrete Forest Left Hide Lock REFERENCES TARGET VOLUMES A…"},
        {"id":"panel:puzzle3d-play-document/seed-left-001","text":"Hexagonal Cut Concrete Forest Left Hide Lock"}, …]
[29.8s] step outliner-rows: ok windows=2 canvases=2 treeItems=16 newFaults=none
```

**The honest caveat, the same one B44 §6.1 records**: `example-switch` is registered in the `replace`
group and the three selection steps in `mutate`, and the probe runs `read → mutate → replace`, so these
greens are all on the **1-object Concrete Forest** document (grown to 8–16 rows by the step's own
edits) — not on Nakagin. This subset is therefore a REGRESSION gate for the two host fixes, not evidence
about the large document. The large-document evidence is §3–§5, taken with `🔍️b46-selection-nakagin.ts`,
which switches the example BEFORE it measures anything.

### 7.2 What rode what

| half | build |
| --- | --- |
| host (`🌐️World3dHost/🟦️.tsx` pick bounds + overlay `activeObjectId`, `🌐️World3dHost/🧫️fixtures/*`) | **live off the repo** — Vite dev server, measured in the browser before/after in §6.3 |
| guest (`🔌️plugin/🦀️.rs` topology memo, `📡️wire/🦀️.rs` membership, `📌️panels/🗿️artifact/🦀️.rs` arena clamp + row-action degradation) | **not in any materialized wasm.** The diagnostic runs rode #57, the battery subset rode #58. A #59 is needed to see them in the browser (§8 item 6) |

## 8 Residual / handed over

1. **The guest does not re-publish its selection after `interactionSelect` on the large document** (§5).
   The command settles, no `interaction selection lost` is recorded, the pane paints the pick from the
   host overlay — and `data-guest-selection-json` stays `selectedIds:[]` for 150 s, with no successful
   projection of `puzzle.3d.play.document` in that window. This is the single defect that keeps every
   selection-scoped verb red on Nakagin, and it is now isolated to "the guest's retained surfaces are not
   re-rendered for the declared `Select` scope", one hop upstream of
   `ownedUiRefreshResponse`'s `uiRefreshSectionUnchanged`. Next wave should instrument the GUEST side of
   `plugin_refresh_ui` against `UiDirtyScope::Partial`, not the host side.
2. **`registerBrushMesh` never drains on the 12-mesh Nakagin catalogue** (§5) — 123 of 285 console lines
   in one 150-second window, still arriving 8 minutes after the switch, `Background` lane. Wave H's
   re-announce loop. A permanently busy guest is the simplest available explanation for residual 1.
3. **`interactionSelect` settle is still ~40 s at 180 objects** in the browser (B44 measured 6–25 s at
   #56/#57). This wave removed the per-pick O(n) topology term (§2) but that fix is not in a materialized
   wasm yet, and B44's residual 1 — ~100–250 host↔guest round trips per command from the one-item
   typed-operation publication grant — is untouched and remains the ceiling.
4. **`relocate-pose-delta`** (B44 residual 3) should be re-measured: `beginRelocateDrag` grabs through
   the same blind `resolveClickInstanceId` this wave fixed (§4.2), so the gesture may simply have had
   nothing to grab.
5. **`🔍️b44-mutation-latency.ts`'s `selectionOf` still reads the wrong keys** (§3.2). It is B44's input
   file and was left in place; any wave reusing it must read `selectedIds`, not `ids`.
6. **A #59 wasm is needed** to see this wave's guest changes (topology memo, `validate_state` membership,
   the outliner's arena clamp and row-action degradation) in the browser at all.

