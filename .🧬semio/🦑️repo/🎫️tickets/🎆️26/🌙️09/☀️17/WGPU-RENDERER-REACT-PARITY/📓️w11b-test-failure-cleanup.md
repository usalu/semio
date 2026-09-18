# 🧪️🩹️ W11b — renderer + world suites to zero red outside W11a's lane

Packet W11b of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Lane: every red family of
`semio-framework-os-renderer-wgpu --lib` and `semio-framework-os-infinite --lib world::` that is NOT
W11a's presenter / mesh-residency / prepared-submit lane, plus the three W1-integration "own ticket"
items the packet named.

Logs: `🗑️generated/w11b-*.txt`.

---

## 1. Before / after

| suite | before | after |
| --- | --- | --- |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1` | **879 passed, 8 failed** (`w11b-renderer-1.txt`) | **892 passed, 3 failed** (`w11b-renderer-final.txt`) |
| `cargo test -p semio-framework-os-infinite --lib world:: -- --test-threads=1` | **166 passed, 4 failed** (`w11b-world-1.txt`) | **178 passed, 0 failed** (`w11b-world-final.txt`) |

**All eight of this packet's target families are green.** Of the three renderer failures left:

1. `async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` — W11a's lane end to
   end, deliberately not touched; its exact failing clause list is §4.
2. `scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack`
   — **landed by a peer during this packet's final gate.** `World3dState` grew 22 752 → 22 920 bytes
   (`left`/`right` of the budget receipt) because a peer added `brush_mesh_run:
   Option<WorldBrushMeshRun>` (`🌍️world/🦀️.rs:1468`) while the committed `FixedSlotTableBudget` still
   names the old size. This packet added no field to `World3dState`; the receipt belongs to whoever is
   landing the brush-mesh run and would go stale again if updated now.
3. `shell::window_pane_chrome_tests::each_pane_chip_dispatches_its_own_window_state` — also a peer's,
   from the live `PointerCapture` / `PointerHitOwner` pane-chip work (`shell.search_open &&
   matches!(shell.overlay_state, OverlayState::Search)`). It was green in this packet's earlier full
   runs (`w11b-renderer-3.txt`, 888/1) and went red only in the last half hour.

Both peer failures are in the same two files their authors were editing throughout this packet (§6's
"two red runs that were not real" records four separate compile breakages from the same lanes).

The world suite's pass count rises by 12, not 4: two laws that were passing vacuously now have real
content (the component-marquee oracle actually answers a component census, §3.2) and peers added laws
of their own during the pass.

---

## 2. Renderer families — root cause → fix → React reference

### 2.1 `shell::tool_run_panel_tests::*` ×2 — **a hug-width retained button solved to width ZERO**

**The reported symptom was a hit-registry gap** (`[STATS] tool-run panel targets []`, W2-W6 §4.2).
It is not. Instrumenting `register_retained_hit` (temporary, removed) printed:

```
node=NodeId { index: 5 } rect=Rect { x: 0.0, y: 21.78, w: 0.0, h: 22.4 } visible=Some(true) button=Some("framework.toolRun.1.toolRunPause")
node=NodeId { index: 7 } … w: 0.0 … button=Some("framework.toolRun.1.toolRunAbort")
```

Every button was visited, visible and correctly identified — and **zero pixels wide**.
`retained_hit_registration` refuses a zero-area rect (`📥️input/🦀️.rs:711`), so the surface's whole
pointer registry came back empty while the panel painted and keyboard focus still walked the buttons
(focus needs no geometry). That is why the family looked like a registration bug for three packets.

**Root cause:** a `Button`'s label is a FIELD of the node, not an arena child, and nothing measured
it. `LayoutNodeKind::Control` set `min_height` only (`📐️flex/🦀️.rs:262`), so taffy sized a
`width: hug` leaf with no content from nothing. Every retained button in this renderer — not only the
ToolRun panel's — published a zero-width box.

**Fix (three files, one lane):**

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs:51` — `Control { height, label_padding: Option<f32> }`.
  `Some(px)` marks a control whose own label is its content.
- `…/📐️flex/🦀️.rs:268` — that arm sets `flow.text = true` and `padding.left/right = px`, so the node
  goes through the SAME `measure_text` lane a `Text` does and hugs `label + 2·padding` — exactly where
  `paint_button` starts its label (`🖌️paint/🦀️.rs:2132`, `bounds.x + theme.padding_standard`).
- `…/📌️mounted_layout/🦀️.rs:502` — `UiNode::Button` maps to `label_padding: Some(theme.padding_standard)`;
  every other control keeps `None`.
- `…/📌️mounted_layout/🦀️.rs:542` / `:557` — a labelled control enters `AdmissionPhase::Text` and
  `admit_text_one` reads `UiNode::Button(button) => button.label`.
- `…/📌️mounted_layout/🦀️.rs:766` — **the second half of the fix.** `publish_one` refuses a job whose
  `lines.len() != runs.len()` (`:821`). Admitting a button produced a text RUN but `collect_one` pushed
  a LINE only for `LayoutNodeKind::Text`, so the first attempt made the layout job fault forever and
  the panel parked in `UiDocumentFramePhase::Layout` for a million opportunities. `collect_one` now
  pushes a line for the same predicate that admits a run.

**React ref:** React's `<Button>` is an inline-flex box that hugs its label with `px-2`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s form-control primitives); it has never had a zero-width state, which
is why no React-side law caught this.

**Second, independent defect in the keyboard law** (`⏯️wgpu-tool-run-panel/🦀️.rs:134`): it collected
3 focus steps and then pressed Tab `focused.len() - 1` more times, assuming the focus ring is as long
as the sample. The ring is 3 (Pause, Abort, Finalize — Step is disabled), so 5 total tabs land on
Abort, not Pause. Rewritten to **walk until the ring wraps back onto Pause** (bounded by the ring
length) and assert that it does, which is the law's actual claim.

### 2.2 `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` — **the border bled outside the window**

**Root cause:** `push_window_silhouette_border` centred every HORIZONTAL segment on its path
(`edge - stroke * 0.5`) while both outer VERTICALS were already inset (`b.x`, `b.x + b.w - stroke`).
So a window cap's top border was painted half a stroke ABOVE `bounds.y` and the notch baseline half a
stroke BELOW the cap — a border outside the window box, and two conventions in one function.

**Fix:** `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1754` — every segment now lies INSIDE the silhouette it
outlines: an edge's outer run at `outer` (top) or `outer - stroke` (bottom), its notch baseline at
`inner - stroke` (top) or `inner` (bottom), and the chip walls at `span.left` / `span.right - stroke`.

**React ref:** `🔲️WindowSilhouette/🟦️.tsx:85` `WINDOW_SILHOUETTE_PATH_INSET = 1` and `:225`
`windowSilhouetteOutline` — React insets its outline precisely so a `vectorEffect="non-scaling-stroke"`
stroke stays within the box; its golden path starts `M1,1 H60 V25 H160 V1 H199 …`, one inset in, never
on the edge.

### 2.3 `shell::command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss` — **the rebootstrap wake answered an epoch where the caller reads a cursor**

**Root cause:** `DirectoryHomeProjection::wake(rebootstrap = true)` returned `begin_epoch(0)`'s new
BOOTSTRAP EPOCH, while the live arm right below it returns the kernel's frontier
(`bootstrap.wake(false)` → `acknowledged_through`). A caller that asked for a rebootstrap therefore
refetched from the epoch number instead of the reset frontier `0`.

**Fix:** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1711` — `begin_epoch(0)?` for its epoch/cancel side effects,
then answer `self.bootstrap.after()`. The kernel's own `DirectoryEventPageBootstrapV1::wake`
(`📇️directory/🔌️client/🦀️.rs:374`) has always answered the frontier for both arms; this was the one
wrapper that did not.

### 2.4 `engine_canvas::saturated_graph_and_board_wheel_queues_preserve_cameras` — **the law read the gesture before the retained commit ran**

**Root cause:** not a strand. `puzzle_board_pointer_up_into` does not finish a gesture: every
non-`Idle` plan `requires_retained_commit()`, so the handler only ARMS `begin_pointer_commit` and
answers whether the plan emits (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:5049`). The gesture ends when
the board's retained pointer authority reaches `Complete`, which the production frame drives in
`AppFrameTransactionPhase::BoardAuthority` (`🧊️renderer/🦀️.rs:12233`). The law asserted
`!defers_descriptor_sync_from_js()` immediately after the retry and never pumped that authority.

**Fix:** `⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` — new `settle_board_pointer_authority`, the
single-surface reduction of the production ladder (`drive_board_authority_step` to `Complete`, then
`publish_board_pointer_step`), called before the assertion. The law now states what it meant: a
refused publish does not strand the gesture *and* the retry's retained commit completes it.

### 2.5 `async_boundary_tests::glass_foreground_scalars_are_encoded_after_the_glass_pass…` — **law stale after a deliberate parity change**

`🧊️gpu`'s glass ladder gained a containment refinement at HEAD+working-tree: a glass-content layer
that a LATER glass region fully encloses is encoded into the SCENE, so the covering region frosts it
(`prepared_foreground_scalar_is_enclosed`, `prepared_glass_region_covers`). Three markers went stale:
`layer.foreground_of.is_some()` moved into `prepared_draw_scalar_glass_region`
(`draw.layers.get(layer)?.foreground_of`), and both phase predicates gained the
`&& !prepared_foreground_scalar_is_enclosed(draw, overlay_after, draw_cursor)` term.

**Fix:** the law re-derived against the current ladder and extended to pin the enclosure predicate and
its containment rule. **React ref** added to its docstring: `useIntroductionElevation` — React's
introduction veil blurs the whole shell except the card it spotlights, and a context menu clipping a
panel's corner must not push that panel into the backdrop, which is why containment (not overlap) is
the predicate.

### 2.6 `async_boundary_tests::raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete` — **the `retained_raster_contract` dead markers, re-derived**

W1 §3.A2 recorded 15 failing clauses, "fourteen of them naming APIs that exist nowhere in this tree".
That is no longer true: W5c's engine-texture work landed the APIs, in a different shape. Re-evaluating
all 101 clauses mechanically (script kept at the scratchpad; the ticket keeps only its findings) left
**13 failing, all in the `⚙️EngineCanvas` and `🧊️gpu` halves**:

| stale clause | what the code does now |
| --- | --- |
| `self.ensure_raster_texture_step(key, pixels` | two carriers: `RasterUploadPixels::Contiguous(pixels)` and `::Pages(pixels)` (`🧊️gpu/🦀️.rs:561`, `:566`) |
| `let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;` | `build.surface.id.with_raster_key(\|key\| gpu.reserve_engine_texture(key, build.width, build.height, candidate_generation, expected))?` |
| 5 × `guarded_allocations(engine, "…(&admission, expected)", …)` | the phase ladder passes `admission` (already a reference) — and each validation now appears exactly ONCE, not twice for the replacement pair |
| 3 × `gpu.retain_engine_allocation_fault(…)` | replaced by `RasterTextureStageFault::Returned { … }` returning the owners to the candidate (`build.admission/texture/view = Some(…)`) plus `begin_close()` and a stepped `cancel_engine_texture_admission` close ladder |
| `surface.texture = texture`, `surface.view = view`, `let published_view = std::mem::replace(&mut surface.view, replacement_view)` | publication is an owner TRANSFER: `candidate.{renderer,replacement_texture,replacement_view}.take()` → `EngineGpuSurface` → `self.live.replace(published)` → `EngineGpuRetirement::new(displaced)` |

**Fix:** every clause re-derived against the current source, keeping each one's INTENT (reservation
before any allocation; each allocation guarded by its own validation with nothing else allocating in
between; a stage fault returns its owners rather than dropping them; publication transfers, never
clones). The mutation battery was re-pointed at the new markers, and the two clone-negations kept
(`!engine.contains("surface.view.clone()")`, `…("surface.texture.clone()")`). The law is
**re-derived, not deleted** — 0 of 102 clauses fail and all 38 mutations are still rejected.

---

## 3. World families

### 3.1 `world::tests::live_renderer_retains_generation_wake…` — **the wake census missed a sixth hand-off and drifted on the host**

Four stale clauses, all against code that is that way at HEAD:

- `FrameBuildCursor` (`🧊️renderer/🦀️.rs:11588`) is a SIXTH typed wake hand-off carrying
  `cursor_wake: Option<WorldCursorWakeToken>`, so the census `== typed_handoffs.len()` was `6 != 5`
  and the `AppFrameAfterChrome` region (bounded at `struct FrameWheelCursor {`) swallowed it and
  counted 2.
- `AppPresentStep::Complete` now carries `generation` and `cursor` as well.
- `HOST_TOKEN_FIELD` spelled `Option<crate::infinite_world::world::WorldCursorWakeToken>`; the host
  writes `Option<infinite_world::world::WorldCursorWakeToken>` — the `crate::` prefix never existed.
- The host's retained token moved from `OsHostRetirement` into `struct OsHostRetirementState`
  (`🏠️os-host/🦀️.rs:108`).

**Fix:** `🌍️world/🧪️tests/🔬️unit/🦀️.rs:153` — six typed hand-offs with correct region bounds (both in
`exact` and in the erasure battery below it), the `Complete` literal updated, the host constant
corrected and the host census re-pointed at `OsHost` + `OsHostRetirementState`. Every erasure mutation
still rejected.

### 3.2 `world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry` — **the fixture had no components**

`ParseIntError` was the messenger. The fixture mesh is `triangle_mesh_oracle()`, whose `vertex_ids`,
`edge_positions`, `edge_ids` and `face_ids` are all empty. `screen_select_components`' `"vertex"` and
`"edge"` arms are guarded on `schema.vertex_ids != 0` / `schema.edges != 0`
(`🎬️scene/📐️math/🦀️.rs:1631`, `:1647`), so the oracle fell through to its INSTANCE arm and answered
`"object-000"` — which the law then parsed as a `u32`. The retained cursor, meanwhile, iterates
`admitted.edges == 0` and answers nothing: the two sides were never compared on components at all.

**Fix:** `🔬️unit/🦀️.rs:867` — `component_triangle_mesh_oracle()` (vertex ids `[11,12,13]`, three
edges with ids `[21,22,23]`, face id `[31]`) and `world_marquee_geometry_fixture_from(data, n)`; the
component law mounts that mesh and now also asserts the oracle's census is **non-empty**, so it can
never pass vacuously again. The object-granularity laws keep the original mesh untouched.

### 3.3 `world_component_marquee_publish_merges_before_one_atomic_set_selection` — **ids went out as floats**

`WorldComponentMarqueePublishJob` emitted each id with `draft.builder().number(None, id as f64)`,
i.e. `Number::Float(7.0)`, where the law asserts `DslValue::int(7)`. React's
`setSelection` carries `ids: readonly number[]` (`🌐️World3dHost/🟦️.tsx:3236`, dispatched at `:6984`),
which serializes as `7`, never `7.0` — and this repo has a standing rule that a whole-number carrier
must not collapse onto a float twin.

**Fix:** `🎬️action/🦀️.rs:365` — new `BoundedActionBuilder::integer(key, i64)` emitting
`Number::Int`; `🌍️world/🦀️.rs:3982` uses it for the component-id array. The law is right; the code
drifted.

### 3.4 `world_object_registry_enforces_capacity_revision_and_aba` — **a re-admitted id leaked its old slot AND kept resolving**

`WorldInteractionObjectRegistry::admit`'s probe only SEEDED `reusable` on a stale-revision slot and
then walked on to the next EMPTY slot, so re-admitting `(kind, id)` at a new revision wrote a second
slot and left the previous revision's entry intact. `resolve` matches on generation AND revision, both
of which the old token still carried, so a marquee cursor holding it kept picking the object it had
already replaced — and the fixed table leaked one slot per revision until it faulted.

**Fix:** `🌍️world/🦀️.rs:2634` — a stale-revision slot of the SAME `(kind, id)` breaks the probe and is
reused. An empty slot still wins over a FOREIGN stale slot (linear probing puts a live id's own slot
before the first hole, so the same-id case is always reached first, and a foreign id's tokens must not
be invalidated early). Docstring added at `:2626`.

---

## 4. Left for W11a, with its exact failing clause list

`async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` reads
`presenter_retirement_contract(LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE, DRAW_SOURCE,
OS_HOST_SOURCE, WINT_APP_SOURCE)` — the prepared presenter witness, the glue's present/acknowledge
ladder, the mesh GPU table and `close_world_owners_step`. **That is W11a's lane in full**, so it is
not touched here. Re-implementing its 60 clauses mechanically leaves exactly **7 failing**:

```
glue  packet.scene_revision() != expected.scene_revision || packet.preview_generation() != expected.input_generation
glue  runtime_presentation_authority_and_candidate_identity_change_independently   (marker absent from the glue)
glue  matches("runtime.presentation_witness_for(self.generation.0)").count() == 1
glue  matches("let expected = self.presentation_authority.current();").count() == 2
glue  matches("self.presentation_authority.mark_scene_changed();").count() == 2
draw  mesh_gpu_retirement_preserves_acknowledged_versions                          (test-name marker)
draw  fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner    (test-name marker)
```

Five are the presentation-authority census in `🧊️renderer`; two are law NAMES the `🖍️draw` mesh table
is expected to carry. Whoever lands the presenter fix should re-derive these five and restore (or
rename) those two laws.

---

## 5. The three W1-integration "own ticket" items

### 5.1 Drop witnesses turning a failure into SIGABRT — **18 more found and guarded**

W3d fixed the asset-probe one. A brace-matching sweep over every `impl Drop` in `📺️renderer` and
`♾️infinite` found **18 more** whose assertion had no `std::thread::panicking()` guard: a witness that
fires while the thread is already unwinding from a test failure is a double panic, i.e. SIGABRT — one
red test takes the whole binary down and hides every test after it.

All 18 now carry the repo's established guard (`📡️replication`, `🌱️value`, `🧬️contract` have used it
for months): `🎞️Scenes` `PendingRasterCheckedOut`; `⚙️EngineCanvas` `EngineSurfaceRetirement`,
`EngineCanvasPresenter`; `📐️surface-lane` `MountedSurfaceResizeLane`; `🏠️os-host`
`OsHostRetirementState` and `OsHostRetirement` (whose `std::process::abort()` is now skipped while
panicking); `🕸️dag` `DagSnapshotRetirement`; `🎲️board` `IconPaintCache`, `BoardFillSnapshot`; and nine
in `🌍️world` (`WorldDrawDraft`, `WorldDrawRebuildCursor`, `WorldDrawRegistry`, `World3dState`,
`WorldTerrainMeshCursor`, `WorldPlaceholderMeshCursor`, `WorldFaceOverlayMeshCursor`,
`WorldAssetFetchOwner`, `WorldAssetIoAuthority`). No predicate weakened: each guard is an `||
std::thread::panicking()` on an already side-effect-free predicate, so the witness is as strict as
before on every non-panicking drop.

Zero unguarded panicking `Drop` impls remain in either crate.

### 5.2 Per-frame world3d `[DEBUG]` gating — verified, and two lanes closed

- `🎞️Scenes`' per-frame `world3d surface=… bounds=… ingest=… geometry=…` census already goes through
  `debug_log_diagnostic` → `semio_framework_trace::runtime_diagnostics_enabled()`. ✅ as W3a left it.
- **Not gated:** the glue's own `world3d ingest surface=…` and `world3d interaction surface=…`
  enter/step/leave traces called `log_debug` directly. They are rate-limited by construction (stride
  512; one pair per intent, plus a census-change and a 4096-stride line), but an orbit drag raises one
  intent per pointer move, so the pair is per-frame chatter on exactly the gesture whose latency this
  ticket measures. All four call sites moved to `log_debug_diagnostic`
  (`🧊️renderer/🦀️.rs:14916`, `:14937`, `:14956`, `:14959`) and the ingest line gained the `[DEBUG] `
  prefix it was missing. Docstrings updated at `:14897` and `:14919`.

### 5.3 The two env-order-dependent laws — already fixed, and a third one converted

`env_lock_ignores_unset_and_empty` and `shell_pref_locks_reads_the_four_lockable_envs` no longer touch
process env at all: they go through `with_boot_locks` / `apply_boot_descriptor` and restore the
previous descriptor (a prior wave's fix, documented at `🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs:78`).
**But the file still had one law using the pattern its own docstring calls unreachable**:
`load_ui_prefs_once_prefers_a_lock_over_storage` set `SEMIO_LOCKED_APPEARANCE` with
`std::env::set_var` — which `env_lock` cannot see once `BOOT_DESCRIPTOR` is installed on that thread,
so it passes or fails purely on test order. Converted to `with_boot_locks` + `apply_boot_descriptor`
like its two siblings. No `set_var`/`remove_var` remains in that file.

---

## 6. Gates

| gate | result | log |
| --- | --- | --- |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` | **892 passed, 3 failed** — one W11a's (§4), two peers' (§1) | `🗑️generated/w11b-renderer-final.txt` |
| `cargo test -p semio-framework-os-infinite --lib world:: -j 4 -- --test-threads=1` | **178 passed, 0 failed** | `🗑️generated/w11b-world-final.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ clean, 62 warnings (proof the expansion ran) | `🗑️generated/w11b-native-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | ✅ clean, 70 warnings | `🗑️generated/w11b-wasm-check.txt` |

An intermediate full run with every fix except §5.2/§5.3 measured **888 passed, 1 failed**
(`🗑️generated/w11b-renderer-3.txt`) — the cleanest reading of this packet's own surface, before the
two peer regressions above landed.

**Two red runs that were not real**, both re-run before being believed:

- `directory_home_bootstrap_…` once panicked with `WorkerPool: mandatory submission failed closed:
  Contended` (`⏳️async/🦀️.rs:1879`) — a pool-contention flake under a peer's concurrent cargo, not the
  law. Green on the immediate re-run and in the targeted run before it.
- Both `cargo check` gates failed **four times on peers' in-flight edits**, never on this packet's:
  `world::WorldInteractionPhase::PointerLeave` not covered (a new enum variant mid-landing), then
  `PointerHitOwner` unimported in `🧊️renderer`, then `WorldBrushMeshRun` / `skip_draw` mid-landing in
  `🌍️world`, then — wasm ONLY — `missing field pointer_capture in initializer of AppInteractionState`
  at `🌐️browser-worker/🦀️.rs:645`, the textbook case of a native gate not compiling `cfg(wasm32)`
  code. Each compiled again once the peer finished; one rustc ICE (`evaluate_obligation`,
  incremental) came with them. Both gates are green on the tree as it stands.

No `activate-*`, no trunk, no wasm build, no probe run from this packet — W11a owns the activation.

---

## 7. Files

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` — `Control { label_padding }`.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` — labelled-control text lane, line census.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs` — `BoundedActionBuilder::integer`.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs` — kind constructor.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` — silhouette stroke inset.
- `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — rebootstrap wake cursor.
- `…/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs` — keyboard ring walk.
- `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs` — boot-descriptor lock.
- `…/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` — board authority settle.
- `…/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, `…/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`,
  `…/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs`, `…/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs` — drop-witness guards.
- `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — world3d trace gating.
- `…/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` — glass law, raster contract.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs` — object-registry ABA, integer ids, drop guards.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — wake census, component fixture.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`,
  `…/♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs`, `…/➡️directed/➕️normal/🦀️.rs` — drop-witness guards.
