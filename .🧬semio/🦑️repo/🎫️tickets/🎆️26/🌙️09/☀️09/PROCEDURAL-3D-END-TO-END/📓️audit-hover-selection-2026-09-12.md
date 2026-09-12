# Hover / Selection / Orbit / Round-trip — generation3d Preview Audit (2026-09-12)

Read-only. Verified every claim against **current disk state** (`grep`, `sed -n`, `cat`), not
prior ticket notes, and ran two live native `cargo test` checks. Prior notes read first:
`📓️preview-hover-select-orbit-2026-09-11.md`, `📓️selection-overflow-2026-09-09.md`,
`📓️viewer-2026-09-09.md`, `📓️viewer-window-2026-09-11.md`, `📓️wgpu-engine-surfaces-2026-09-10.md`,
`📓️status.md` tail, peer `26/09/02/PUZZLE-3D-END-TO-END/📓️2026-09-11-wave-B18-brush-preview.md` and
`📓️2026-09-08-selection-hover-mechanism.md`. Today's `📓️status.md` tail (2026-09-12) confirms this
audit is one of six W0' lanes dispatched against `📓️audit-*-2026-09-12.md`, and independently flags
`interactionSelect leaves send-message effects unmapped` — investigated below (§7).

## 0. Headline

The **wiring is real, not stubbed**: generation3d already implements `render_with_request_context`
and `context_menu_with_request_context` (one of only 3 apps repo-wide that do, per the 2026-09-08
peer audit), reads the framework's `InteractionView` for both render-time marks and command-time
selection, dispatches `interactionHover`/`interactionSelect`/`setCamera` from `World3dHost.tsx`
(not the legacy `worldPointerDown`/`graphPointerDown` verbs, which generation3d has **zero**
command directories for — confirmed by directory listing), and has real (non-stub) mutation bodies
for translate/rotate/scale/delete-selection. Orbit/pan/zoom is a real three.js `OrbitControls` wrap.
Camera is dual-classified (persisted-local in Config, ephemeral-shared in Presence) and survives
example switch by explicit clone. Mesh identity (`{widgetId}@{channel}#{index}`) is deterministic
because `widget_id` is a stable struct field, not an index.

**But two things block calling this "done":**

1. **Runtime proof is still blocked upstream of interaction entirely** — `meshes=0` as of today's
   `📓️status.md` tail (2026-09-12), the contributions push is starved by a `refreshUi` supersession
   race (documented live, not by me), so no browser session has ever painted a real mesh to click on.
2. **A live, reproducible native stack overflow** in the wgpu engine-surface tests that are the
   *only* tests proving generation3d's `interactionSelect`/`setCamera` payload shape on wgpu — see
   §6, reproduced twice just now at the platform default 2 MiB thread stack. It is masked in the
   repo's normal test invocation by a blanket 128 MiB `RUST_MIN_STACK` floor, so it reads green in
   every gate log this ticket has cited — but the underlying frame-size defect is the same *class*
   the 2026-09-09 `📓️selection-overflow-2026-09-09.md` packet fixed at a different site, and this
   site was never touched.

---

## 1. Full interaction path today (edit-mode preview)

### 1.1 DOM pointer → dispatch (`World3dHost/🟦️.tsx`, React renderer)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` (6471 lines).

| Gesture | Handler | Dispatch | Line |
|---|---|---|---|
| Instance pointer-down | `handleInstancePointerDown` | `dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, "object", [record?.interactionId ?? id], merge))` | 5331 |
| Hover move | (inline in pointer-move handler) | `dispatch("interactionHover", world3dHoverActionArgs(interactionDomainId, interactionGranularity, target))` | 5352 |
| Marquee release | (inline) | `dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, "object", domainTargets, "replace"))` | 5916 |
| Empty click | `handleEmptyClick` | `dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, interactionGranularity, [], merge))` | 6021 |
| Completed orbit/pan/zoom | `handleCameraChange` (debounced) | `dispatch("setCamera", worldCameraSetCameraDispatchArgs(windowInstanceId ?? node.surfaceId, state))` | 4886 |

`world3dSelectionActionArgs`/`world3dHoverActionArgs` at `:4541`/`:4460`. These are the **framework**
verbs, gated behind `scene.domainId` being set (`interactionDomainId`). Generation3d's preview scene
always sets `domain_id: Some("graph")` (see §1.2), so this is the live path, not a fallback.

**Confirmed dead**: `dispatch("worldPointerDown", …)` exists at `World3dHost/🟦️.tsx:5851`, but it is
gated by `selection.engagementSessionActive` (line 5844) — a mode only CAD/process3d/shooting arm
via their own `engagementSessionActive` selection state. generation3d never sets it (grepped
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d` for `engagementSessionActive`: 0 hits), so
this branch is unreachable for generation3d. `graphPointerDown` does not exist anywhere in
generation3d at all (`find … 🎮️commands -maxdepth 1 -type d` lists 30 command dirs, none named
`graph-pointer-down` or `world-pointer-down`).

**Test proving the negative, live in the repo**
(`✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:687-696`,
`no_pointer_down_route_survives_the_framework_owned_selection_domain`):
```rust
let definition = create_generation3d_app();
let json = serde_json::to_string(&definition).expect("app definition json");
assert!(!json.contains("PointerDown"), "pointer-down routes are framework-owned now");
assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().all(|id| !id.ends_with("PointerDown")), …);
assert!(definition.interactions.iter().any(|interaction| interaction.id == "graph"), …);
```

### 1.2 Guest scene declaration

`✏️s/…/🧊️generation3d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:89-97` (`render()`):
```rust
&semio_framework_ui::wgpu::World3dScene {
    status_json,
    domain_id: Some(GENERATION_3D_INTERACTION_DOMAIN.into()),
    domain_granularity_id: Some(GENERATION_3D_INTERACTION_GRANULARITY.into()),
    ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun)
},
```
`GENERATION_3D_INTERACTION_DOMAIN = "graph"`, `GENERATION_3D_INTERACTION_GRANULARITY = "handle"`
(`✏️editor/🦀️.rs:2065,2074`). This is generation3d opting **into** the host's generic pick path —
the opposite of puzzle3d/block3d, which explicitly set `scene.domain_id = None` and never wired a
replacement (per the peer 2026-09-08 audit, `📓️2026-09-08-selection-hover-mechanism.md` §2, "this
strongly suggests basic click-to-select in the puzzle3d 3D viewport may not currently dispatch
anything real" — unrelated to generation3d, cited only to contrast: generation3d does not have this
gap).

### 1.3 Guest handler → framework interception → app read

`interactionSelect`/`interactionHover` are **framework-injected reserved actions** —
`VcsArtifactApp` intercepts and applies them to `protocol::InteractionState` /
`InteractionHoverState` **before** any app code runs (`🔌️plugin/🦀️.rs`, confirmed by the 2026-09-08
peer audit's own citations at `:8880-8886`, `:18637`). generation3d's `handle()`/`apply()` never see
these two action ids directly; they only ever *read* the result through `InteractionView`.

Generation3d's own commands that **read** selection (real, not stub):

- `✏️editor/🎮️commands/❌️delete-selection/🦀️.rs:34-41` (`apply`):
  ```rust
  pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, interaction: &InteractionView<'_>, _session: &mut FlowEvalSession) -> Result<Emit<…>, Fault> {
      Ok(delete_selected(&doc.snapshot.fixture, &interaction.selection("graph").ids))
  }
  ```
  The macro-forced 4-arg `handle()` (no `interaction` slot, framework-fixed shape) deliberately
  degrades to an empty selection and is documented as unreachable in production — `Generation3dPlayApp::handle`
  always routes through `apply` (`:23-28` doc comment). This is intentional, not a hidden bug.
- `translate-selection` (114 lines, has its own `🧪️tests/🔬️unit/🦀️.rs`), `rotate-selection` (106
  lines), `scale-selection` (105 lines) — same `apply(interaction: &InteractionView<'_>, …)` shape,
  real per-selected-id transform math, confirmed non-empty bodies (not stubs). **Only
  translate-selection has a dedicated test directory** — rotate/scale/delete-selection are exercised
  only indirectly through the context-menu presence test (§5).

### 1.4 Render-time marks → preview instance highlight + graph highlight

`✏️editor/🦀️.rs:1704-1719` (`render_with_request_context`):
```rust
let marks = PreviewInteractionMarks::from_interaction(interaction);
owner.with_mut::<Generation3dInstanceOperationOwner, _>(|owner| {
    owner.with_session(|session| generation3d_render_body(body_key, doc.snapshot, cfg.snapshot, preview_eval_text, view_state, &marks, session))
})
```
`PreviewInteractionMarks::from_interaction` (`:2093-2101`):
```rust
pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
    Self { hovered: interaction.hover(GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_CHANNEL).ids.iter().cloned().collect(),
           selected: interaction.selection(GENERATION_3D_INTERACTION_DOMAIN).ids.iter().cloned().collect() }
}
```
The three-level match (`marked`, `:2098-2100`) is what makes hover/selection bidirectional between
the node graph and the world preview: `set.contains(widget_id) || set.contains("{widget_id}@{channel}") || set.contains("{widget_id}@{channel}#{index}")`.

### 1.5 Context menu

`✏️editor/🦀️.rs:1738-1767` — `context_menu` (marks-free delegate, empty selection) and
`context_menu_with_request_context` (the one `VcsArtifactApp::context_menu` actually calls) both
funnel into `context_menu_body` (`:1772-1802`), which reads `marks.graph_selection_domains(&doc.snapshot.fixture)`
and only offers `translateSelection`/`rotateSelection`/`scaleSelection`/`removeWidget`/`removeGeneration`
when `has_selection` is true. **This is real, framework-owned selection, not
`request.surface.selection`** (which only carries what the clicked surface itself painted — the doc
comment at `:1748-1751` names this explicitly, matching the peer 2026-09-08 audit's finding that
`context_menu` has no interaction-aware variant *anywhere else* in the framework — generation3d is
one of the few that closed this gap for itself).

Test, live and passing at time of the 2026-09-09 selection-overflow packet (re-verified by grep
against current source, not re-run — see §6 for why I prioritized re-running the wgpu tests
instead): `graph_selection_splits_into_node_and_edge_domains`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:677-684`) and `context_menu_reads_the_framework_owned_graph_selection`
(same file, dispatches a real `interactionSelect` then asserts the menu contents change).

---

## 2. Orbit / pan / zoom

`WorldOrbitGated` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3490`) wraps
`three/addons/controls/OrbitControls.js` (`:29`) behind the package's own interface
(`@semio-tech/infinite-world-r3f`, an **in-repo** workspace package —
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/package.json:3` —
not an external runtime dependency, consistent with CLAUDE.md's "use system libraries behind an
interface"). Mounted at `World3dHost/🟦️.tsx:6283-6291`, gated off during marquee/gumball/connect-drag
(`controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}`).

**Camera ownership**: host-local `cameraState` drives `OrbitControls` every frame (ephemeral,
60fps); a completed gesture (`onCamera={handleCameraChange}`) debounces
(`CAMERA_SYNC_DEBOUNCE_MS`, imported from `../📐️Canvas2dHost/🟦️.tsx:116`) into one `setCamera`
dispatch (`World3dHost/🟦️.tsx:4874-4889`, comment: *"`setCamera` is `ActionKind::View` publishing on
the WindowConfig lane ONLY … debounced so a scroll/wheel burst doesn't spam one dispatch per
tick"*). So this is **not** a spammy per-frame guest round trip — it costs at most one turn per
completed gesture.

**Persistence across example switch**: `🎮️commands/🎨️set-active-example/🦀️.rs:21`,
`preview_camera: previous.preview_camera.clone()` — the camera is deliberately carried forward, not
reset, when the example changes. Confirmed live by grep, not assumed.

**wgpu**: fixed 2026-09-10 (`📓️wgpu-engine-surfaces-2026-09-10.md` §3.2) — orbit/pan on wgpu were
dead code because `WorldInteractionAuthority::step` retired the right-drag intent as `Complete`
before dispatch reached `plan_world3d_drag`; native `infinite_world/🌍️world/🦀️.rs:5271-5291` now
falls through instead. A live native test proves this still holds today — see §6, table row
`world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload` (passes logically; only the
*thread-stack* issue is new).

---

## 3. Mesh identity contract (round trip to flow node / operation)

`preview_payload` (`✏️editor/🦀️.rs:2642-2712`):
```rust
let id = crate::widget_id(widget).to_string();
…
let instance_id = format!("{id}@{channel}#{index}");
let own_mesh_id = format!("eval-{id}@{channel}#{index}");
…
instance_object.insert("interactionId", dsl::json::Value::String(format!("{id}@{channel}")));
```
`crate::widget_id` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs:31-42`) matches on
every `Widget` variant and returns its `id: String` **struct field** — not an array index or a
derived hash — so it is stable across re-evaluation as long as the flow graph's own node ids don't
change (which is exactly the guarantee the flow document gives). `channel` is a widget's
tessellation-channel name (stable per widget kind); `index` enumerates within a channel and is the
only piece that is evaluation-derived, which is why the **handle**-granularity topology target is
`{widgetId}@{channel}` (not the index-qualified id) — the part the framework's `validate_state`
actually prunes against.

Port/handle ids on the node-graph side come from the **same projection**:
`generation3d_port_ids_by_node` (`✏️editor/🦀️.rs:221-224`):
```rust
fn generation3d_port_ids_by_node(fixture: &FlowFixture) -> BTreeMap<String, Vec<String>> {
    let (graph_nodes, _) = …fixture_to_workflow(&host.dag.fixture);
    graph_nodes.into_iter().map(|node| (node.id, node.inputs.into_iter().chain(node.outputs).map(|port| port.id).collect())).collect()
}
```
doc comment: *"read from the SAME `fixture_to_workflow` projection the node-graph window paints — so
an interaction target and a graph pick can never drift apart."* This is what makes the round trip
deterministic: a preview pick's `{widgetId}@{channel}` and a graph port pick's `{nodeId}@{portId}`
are literally the same string space, minted from the same source, once per render — not synced
after the fact.

**Not covered by generation3d's own fixture is the puzzle3d-style multi-instance
`surfaceId`/`recordId` ambiguity** documented in the peer ticket's
`📓️2026-09-12-wave-B20-world-surface-identity.md` (`🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json`) —
that fixture is about two panes of ONE puzzle3d document sharing one DFS record id. generation3d's
preview window is single-instance per document today (`GENERATION_3D_PLAY_WINDOW_PREVIEW =
"procedural-preview"`, one window kind, no multi-pane story), so that specific identity collision
class does not currently apply — flagged only so a future multi-pane preview does not silently
inherit it.

---

## 4. CLAUDE.md state classification (persisted local-only / persisted shared / ephemeral local-only / ephemeral shared)

Framework-level definitions, `🔌️plugin/🦀️.rs`:

| Concern | Classification | Evidence |
|---|---|---|
| Selection (`InteractionState`) | **Persisted-local** | `:9285-9287` doc comment: *"`state` is the persisted-local half (selection/mode/granularity, one `VcsArtifactApp::interaction_store` `ApplyInLane{lane: HistoryLane::Interaction}` dispatch away from every change)"*. `:12715-12718`: `ArtifactToolPublicationLane::Interaction` — *"Never a document/config lane: it is persisted-local and excluded from undo/redo by declaration."* |
| Hover (`InteractionHoverState`) | **Ephemeral-local**, mirrored to **ephemeral-shared** for presence | `:9287-9288`: *"`hover` is the ephemeral-local half … never persisted"*. `:19108`: *"mirrored into the ephemeral-shared `PresenceInteraction` broadcast … exactly like `presence_store`'s own local half"* |
| Preview camera | **Persisted-local** (Config) **and** **ephemeral-shared** (Presence) — dual | `Generation3dConfig.preview_camera` (`✏️editor/🎚️config/🦀️.rs:76-78`, `#[dsl(block)]`) **and** `Generation3dPresence.preview_camera` (`✏️editor/👥️presence/🦀️.rs:20-26`) are separate fields on separate structs. Viewer's `Generation3dViewCommandWork` explicitly documents the split: *"finishes with `CompleteWithEphemeral` carrying **presence** (camera + show mode, so a co-viewer follows)"* (`📓️viewer-2026-09-09.md` §2.4) |

**Assessment**: this matches CLAUDE.md's four-way distinction cleanly — selection is
persisted-local (survives reload, excluded from undo/redo so it doesn't pollute document history),
hover is ephemeral-local (never persisted, would be meaningless after reload), camera is the
interesting dual case: persisted-local so *your own* camera survives a reload/example-switch, and
separately broadcast ephemeral-shared so a co-viewer's live camera state is visible without ever
being written to the document. **No CQRS/event-sourcing mismatch found** — the six reserved
interaction verbs (`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
`setSelectionMode`/`setInteractionGranularity`, per the 2026-09-08 peer audit) are the only writers
of `InteractionState`, and generation3d's own commands only ever *read* through `InteractionView` —
no CRUD-style direct mutation of selection state from app code was found anywhere in generation3d.

---

## 5. Tests — what exists, what is missing

### 5.1 Exists (Rust, native)

| Test | File:line | Proves |
|---|---|---|
| `no_pointer_down_route_survives_the_framework_owned_selection_domain` | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:687` | No `worldPointerDown`/`graphPointerDown` route reachable |
| `context_menu_reads_the_framework_owned_graph_selection` | same file, per 2026-09-09 packet | Context menu changes with a real `interactionSelect` dispatch |
| `graph_selection_splits_into_node_and_edge_domains` | same file:677 | `PreviewInteractionMarks` → node/edge domain split |
| `generation3d_interaction_selection_owns_its_persisted_history` | `✏️editor/🧪️tests/🔬️unit/🦀️.rs:572` (per 2026-09-09 packet) | Selection persists through the interaction store |
| `every_view_config_leaf_owner_directory_exists_on_disk` etc. (viewer) | `👁️viewer/🎚️config/🧪️tests/🔬️unit/🦀️.rs` | Viewer's own camera/show-mode config leaves are real, invertible |
| translate-selection unit tests | `✏️editor/🎮️commands/↔️translate-selection/🧪️tests/🔬️unit/🦀️.rs` | Real per-id transform |

### 5.2 Exists (wgpu native, generation3d-specific payload shape)

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`
(387 lines) — doc comment at `:10-12`: *"The `World3d` payload is generation3d's own
`hexagonal-mushroom-column` preview scene … pinned against React's `World3dHost`, the other
implementation of this same host."*

- `world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid` (`:254`)
- `world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches` (`:282`) — asserts
  `select.action == "interactionSelect"`, `select.controller_id == "generation3d"`, and the target
  id is the bare channel-qualified id (never `surfaceId/id`)
- `world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload` (`:311`) — asserts wheel-zoom
  moves the orbit camera and emits `setCamera` with a 3-axis position; asserts an
  `alt`/`meta`+button-2 drag also emits `setCamera` and actually turns the camera

**This is real cross-implementation pinning**: the wgpu engine and the React `World3dHost` are
tested against the *same* fixture data and asserted to emit the *same* action shape — exactly the
"multi-implementation… same output… at least one third-party" spirit CLAUDE.md asks for (here the
"third party" is the other renderer implementation, not a library, which is the correct read since
there is no external oracle for this framework-owned interaction protocol).

### 5.3 Missing / gaps found live

1. **World3dHost.tsx (6471 lines) has zero interactive vitest coverage.** All coverage lives in
   `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
   (10 092 lines) and is limited to: pure function unit tests
   (`world3dHoverActionArgs`/`world3dSelectionActionArgs`/`worldCameraSetCameraDispatchArgs`/camera
   merge/dom-json helpers, `:3950-3996`, `:5165-5416`) and two `renderToStaticMarkup` SSR snapshot
   tests (`:5112-5150`, checks `data-surface-id`/gizmo markup only). **No simulated pointer
   down/move/up, no orbit drag simulation, no click-then-assert-`interactionSelect`-dispatched test
   exists for the React host** — confirmed by `grep -c "describe\|it(\|test("` returning only the
   pure-function/SSR set, and by `find 🌐️World3dHost -type f` returning only the `.tsx` and one
   fixture JSON, no test file.
2. **rotate-selection / scale-selection / delete-selection have no dedicated unit test directories**
   (only translate-selection does — confirmed by directory listing) and are exercised only
   indirectly, as menu-item-presence assertions in the context-menu test, never as
   dispatch-and-assert-the-mutation tests.
3. **No end-to-end browser proof of hover/select/orbit exists for generation3d** — blocked upstream
   by `meshes=0` (contributions-push starvation, `📓️status.md` 2026-09-12 tail, unrelated to
   interaction code). The `📓️preview-hover-select-orbit-2026-09-11.md` punchlist (§5, all eight
   examples) remains entirely unchecked as of today.
4. **wgpu World3d hover/select/orbit tests for generation3d's payload shape overflow the native
   default thread stack** — see §6, a new finding, live-reproduced, not previously flagged in any
   ticket note read.
5. Viewer (`👁️viewer/**`) hover/selection is declared identically to editor (§ per
   `📓️viewer-window-2026-09-11.md` §4.1-4.2) but has **zero runtime evidence** — `📓️status.md`
   boots exercised editor almost exclusively (per that note's §5.4).

---

## 6. Live finding: wgpu World3d generation3d tests overflow the default native thread stack

Ran, foreground, no state-modifying commands, shared cargo target dir (no `CARGO_TARGET_DIR`
override, per `[[project-shared-cargo-build-dir-fine-grain-locking]]`):

```
cargo test -p semio-framework-os-renderer-wgpu --lib -- \
  world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches \
  world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload \
  world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid \
  --test-threads=1
```

Result (verbatim, first run — orbit ran first alphabetically and aborted before the other two could run):
```
running 3 tests
test engine_canvas::engine_surface_attach_tests::world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload ...
thread '…world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

Reran each in isolation — **all three individually overflow** at the platform default 2 MiB thread
stack, including the simplest one with no interaction dispatch at all
(`world3d_preview_window_attaches_the_world_engine_and_paints_the_tessellated_solid`).

Diagnostic-only (not a fix, matching the 2026-09-09 packet's own methodology — raising
`RUST_MIN_STACK` only proves the descent is finite, never used as a real fix):
```
RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-renderer-wgpu --lib -- <all three> --test-threads=1
test result: ok. 3 passed; 0 failed; …
```
32 MiB clears it — so this is **not a logic bug**, it is the same class of oversized-generator-frame
defect the 2026-09-09 packet fixed in `dispatch_framework_reserved_action`/`close_step`, at a
**different, untouched site** (the wgpu `engine_surface_attach_tests` module's
`attach_and_settle_world3d`/`paint_scene`/`drive_world3d_ladder` chain, or a frame inside
`WorldInteractionAuthority`/`plan_world3d_drag`/`step_world3d_interaction` reached from it — root
cause not isolated at this budget; would need the same `objdump -d` prologue reading the 2026-09-09
packet did).

**Why every ticket gate log this ticket has cited shows these tests green**: the repo's own test
runner (`runCargoTestBudgeted`,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1655-1663`) force-floors
`RUST_MIN_STACK` to 128 MiB for every assertion thread unless the caller already set one, with the
doc comment stating plainly: *"because app-fixture laws routinely exceed libtest's 2 MiB default
stack."* That is an accepted, institutionalized workaround for *test* threads — legitimate for a
test harness, but it means **this class of defect is currently undetectable by the repo's own gates**,
and nothing here proves the same oversized frames are absent from the *production* wgpu call path
(orbit/pick dispatch inside the real renderer, not the test harness) — unlike the 2026-09-09 packet,
which was motivated specifically because the analogous native-plugin-dispatch frames also threatened
the 8 MiB **wasm guest shadow stack** in production. Whether the wgpu renderer's production frame
loop runs any `World3d` interaction step on a thread thinner than 32 MiB was not established in this
audit (would require locating wherever the wgpu frame/interaction loop's OS thread is spawned and
its stack size, out of scope for the time budget here) — flagged as the follow-up question, not
answered.

---

## 7. `send-message` effect unmapped (flagged live by the coordinating ticket today, investigated here)

Today's `📓️status.md` tail (2026-09-12) notes: *"`interactionSelect` leaves `send-message` effects
unmapped (`wireEffectToFriendly` drops them)"*. Traced:

`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts:189-224`
(`wireEffectToFriendly`) has a `switch (effect.tag)` covering `invoke-extension`, `request-sync`,
`notify`, `navigate`, `open-external-url`, `set-panel`, `set-active-utility`, `open-window`,
`close-window`, `spawn-plugin-instance`, `open-plugin-instance`, `dispatch-action` — **no
`"send-message"` case**, so it falls to `default:` and is dropped with
`console.warn("[DEBUG] wireEffectToFriendly: unmapped effect \"send-message\" dropped …")` (`:222`).

**This does not affect the render-time marks/highlight path** (§1.4) — that travels through the
command's `Emit`/`output.interactionView`, read by `ShellHost`'s `applyLeftoverInteractionView`
(`🏛️ShellHost/🟦️.tsx:1928`), never through `wireEffectToFriendly`. A **separate** function,
`shellFrameBytes` (`🖼️wire-turn.ts:97-105`), already special-cases `"send-message"` targeting
`Shell{instance}` to extract `AppFrame` reply bytes — so *some* `send-message` consumers exist; the
question is whether anything an `interactionSelect` completion emits as a `send-message` **effect**
(as opposed to an Emit output) is currently silently lost. Not fully root-caused at this audit's
budget — recorded as an open question for the coordinating session's own lane, not claimed fixed or
broken beyond what is quoted above.

---

## 8. Ranked gap list

| # | Gap | Evidence | Minimal clean fix | Verify natively | Verify in browser |
|---|---|---|---|---|---|
| 1 | Contributions push starved (`meshes=0`), blocks **all** runtime interaction proof | `📓️status.md` 2026-09-12 tail, root-caused to a `refreshUi` supersession race in `🏛️ShellHost/🟦️.tsx` `refreshUi` (~4303) vs. the contributions push (~4500-4600) | Not this audit's fix (another lane's, dispatched today per status.md) | — | `data-meshes-json` length > 0 after boot |
| 2 | wgpu `World3d` generation3d tests overflow the native default 2 MiB thread stack (§6) | Live repro, twice, this session | Same technique as `📓️selection-overflow-2026-09-09.md` §3: `objdump -d` the `engine_surface_attach_tests`/`attach_and_settle_world3d`/`WorldInteractionAuthority` chain for the dominant frame, split it into a non-generator helper | `cargo test -p semio-framework-os-renderer-wgpu --lib -- world3d_orbit_and_wheel_emit_the_generation3d_set_camera_payload --test-threads=1` at **default** stack (no `RUST_MIN_STACK`) must pass | n/a (native test, not browser-reachable) |
| 3 | No interactive vitest coverage of `World3dHost.tsx`'s own pointer/orbit handlers (only pure-function + SSR-snapshot tests exist) | §5.3 item 1 | Add a jsdom+RTL (or existing repo pattern) test that mounts `World3dHost`, simulates pointer-down on an instance, asserts `onAction` received `"interactionSelect"` with the right args; same for a wheel/drag orbit gesture | `bun x vitest run …engine-contract` (new test in same or a colocated file) | N/A — this closes the native-side gap the browser probe can't reach headlessly |
| 4 | rotate-selection/scale-selection/delete-selection have no dedicated dispatch-and-assert unit tests | §5.3 item 2, directory listing | Mirror `translate-selection`'s `🧪️tests/🔬️unit/🦀️.rs` for the other three | `cargo test -p semio-s-plugin-procedural` | — |
| 5 | Per-example runtime proof of hover/select/orbit (all 8 examples) still 100% unchecked | `📓️preview-hover-select-orbit-2026-09-11.md` §5 table, unchanged as of today | Run the punchlist's own suggested probe command once gap #1 is closed | — | `bun 🔍️browser-probe.ts --mode=interact --steps=example,hover,select,orbit --example=<id> --settle=120` per example |
| 6 | `send-message` effect unmapped in `wireEffectToFriendly` — scope of impact on `interactionSelect` not established | §7 | Add a `case "send-message":` branch once the actual payload shape an `interactionSelect` completion sends is confirmed (or confirm it's dead for this action and document why) | grep for what emits `Effect::SendMessage` on the `interactionSelect` route, native test asserting it is/isn't consumed | — |
| 7 | Viewer preview hover/select/orbit has zero runtime evidence (declared identically to editor, per `📓️viewer-window-2026-09-11.md` §4) | Same note §5.4 | Add a `VITE_SEMIO_APP_ROLE=viewer` playground launch row (that note's own P0 punchlist item 2) and run the same probe against it | — | Same probe, viewer app role |

---

## 9. File index (absolute)

```
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❌️delete-selection/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️translate-selection/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx
🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts
```
