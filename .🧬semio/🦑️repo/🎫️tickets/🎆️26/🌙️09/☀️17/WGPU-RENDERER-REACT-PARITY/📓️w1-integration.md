# 🧩️ Wave 1 integration — wgpu renderer React parity

Integrator pass over the fifteen parallel packets `📓️w1a` … `📓️w1o`. Every command below was run in
the foreground, one `cargo` at a time, plain env, `-j 4`. Logs under `🗑️generated/w1int-*.txt`.

> ⚠️ **The tree moved continuously during this pass.** Packet owners kept landing follow-up work
> minute by minute (mobile panel flattening, `DockTabSkeleton.trees`, content-hug panel height,
> the TextEditor rename popup, Paint2d engine laws, `DockState::remove_window` → `close_window`,
> `Ui::focused_node`). Several gates had to be re-run three to six times because a file changed
> between a check's start and its end; three gate failures below are peer code that was mid-edit at
> the moment of measurement and are named as such. Compile gates were all green as of their final run.

---

## 1. Gate results

| # | Gate | Command | Outcome |
|---|---|---|---|
| 1 | ui crate, wgpu engine | `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going -j 4` | **0 errors**, `Finished` (`w1int-ui-check-3.txt`) |
| 2 | os renderer, native | `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going -j 4` | **0 errors**, `Checking semio-framework-os-renderer-wgpu` present, `Finished` (`w1int-osr-native-6.txt`) |
| 3 | os renderer, native-bin | `cargo check -p semio-framework-os-renderer-wgpu --lib --bins --features native-bin --keep-going -j 4` | **0 errors**, `Finished` (`w1int-osr-nativebin-3.txt`) |
| 4 | os renderer, wasm32 | `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going -j 4` | **0 errors**, `Checking …` present, `Finished` in 1m 16s (`w1int-osr-wasm-4.txt`) |
| 5 | ui crate tests | `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4 -- --test-threads=1` | **506 passed / 5 failed** (`w1int-ui-test-2.txt`); all 5 proven pre-existing (§3.A) |
| 6 | os renderer tests | `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1 --skip renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length` | **752 passed / 32 failed**, no aborts (`w1int-osr-test-7.txt`, names in `w1int-osr-failures.txt`) |
| 7 | os infinite world/terrain | `cargo test -p semio-framework-os-infinite --lib -j 4 -- --test-threads=1 world:: terrain` | **157 passed / 7 failed** (`w1int-infinite-test.txt`); all 7 proven outside the Wave 1 diff (§3.D) |
| 8a | styling codegen freshness | `NX_DAEMON=false bun nx run @semio-tech/ui-styling-tokens:check-generated --skip-nx-cache` | **exit 0** — "generated artifacts are fresh" |
| 8b | wgpu TS lint | `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-wgpu:lint` | **exit 0** (5 tasks) |
| 8c | regenerate browser boot | `… :generate-browser-boot --skip-nx-cache` | **exit 0**, 22.4s |
| 8d | regenerate frame worker | `… :generate-frame-worker --skip-nx-cache` | **exit 0**, 21.1s |
| 8e | browser worker bundles | `… :check-browser-worker --skip-nx-cache` | **exit 0** — bundles byte-fresh after 8c/8d |
| 8f | frame worker bundle | `… :check-frame-worker --skip-nx-cache` | **exit 0** |
| 8g | wgpu TS unit suites | `… :test-browser-worker` | **79 passed / 2 failed** (`w1int-ts-testbw-2.txt`), was 78/3 before §2.9; both remaining proven pre-existing (§3.C) |
| — | ui-render (shader contract) | `cargo test -p semio-framework-ui-render --lib -j 4 -- --test-threads=1` | **129 passed / 0 failed** — the structural WGSL validator accepts the aligned `WORLD3D_SHADER` twin (§2.7) |

Gate 6's `renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`
is `--skip`ped because it does not merely fail, it **aborts the whole test binary**: its assertion
`semio_framework::mesh_from_glb(&valid).expect("legacy glTF oracle")` panics with
`gltf: buffer index 0 out of range`, and unwinding then trips
`WorldAssetFetchOwner`'s drop witness (`♾️infinite/🌍️world/🦀️.rs:12302`), which is a
**non-unwinding panic in a destructor → SIGABRT**. On the first run that hid every test after
`async_boundary_tests` (47 of 784 ran). Both the test file
(`🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`) and the oracle crate
(`🔨️modules/🏗️mesh-engine`) are byte-identical to HEAD, so this is pre-existing. **Worth its own
ticket**: a drop witness that converts a test failure into a process abort makes every later test
unobservable.

---

## 2. What I changed, and why

### 2.1 Board2d golden JSON — gate 5's only real Wave 1 failure
`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs:338`
(`GOLDEN_SCENES_JSON`) — added `"gridVisible":true` and
`"selectableNodes":true,"selectableEdges":true,"selectableHandles":true` to the `Board2dScene`
object. The struct gained those four fields (`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1924-1937`), and they are
React-parity-correct: `🖱️ui/🎬️scene/🟦️.ts:970-976` declares all four as optional props and
`🖥️Board2dHost/🟦️.tsx:1110,1119` reads each as `scene.<prop> !== false`, i.e. default `true` —
exactly what `board2d_default_true` answers.

### 2.2 `tree_section_header_height` — unused-import warning in the lib build
`🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:29` imported it unconditionally, but its only caller
`paint_tree_widget` is `#[cfg(test)]` (`:2298`). Split into a `#[cfg(test)] use`.

### 2.3 `world3d_pointer_down_law` — two wrong assertions, one abort
`🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:311-312`. The law indexed
`args["targets"][0]` as a nested array, but `targets` is a **JSON text** arg — the world authority
writes it through bounded string credits (`♾️infinite/🌍️world/🦀️.rs:6343`
`INTERACTION_TARGETS_OPEN`, `:6358` `builder.string_joined(Some("targets"), …)`), which the producer's
own docstring at `:3360` states explicitly. So both assertions read `None` and the failure tripped
`World3dState`'s drop witness (`🌍️world/🦀️.rs:1735`) into a second SIGABRT. Fixed by parsing the
text, and by correcting the expected granularity from `"handle"` to `"object"`:
the shared oracle `🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` gives
`granularity: "object"` for all seven `interactionSelect` cases (`instance-pick-*`,
`marquee-release-replaces`) and `"handle"` only for `instance-hover` — which is what
`WORLD_PICK_GRANULARITY_ID` (`🌍️world/🦀️.rs:11072`) and its docstring already encode. The id is also
the bare channel id (`extrude@solid`, not the render id `extrude@solid#0`), matching
`interactionTargetsForInstances`, so the law now strips the `#n` suffix.

### 2.4 Two stale source-shape laws in `🔬️wgpu-renderer-async-boundary`
* `:509` `native_binary_owns_exactly_one_entrypoint_driver` expected `drive_entrypoint(` twice;
  `⌨️native-entrypoint/🦀️.rs` has three call sites (`:83`, `:136`, `:149`) — and had three at HEAD
  too. The law's real content ("exactly ONE `block_on(`") still holds; the count is now 3. **Passes.**
* `:77` sliced `DRAW_SOURCE` between `"pub fn ensure_raster_step"` and
  `"pub fn get(&self, key: &str) -> Option<&RasterTexture>"`. The first marker has not matched since
  the function became `pub(crate) fn ensure_raster_step` (`🖍️draw/🦀️.rs:1677`; already `pub(crate)` at
  HEAD), so `find` fell back to `draw.len()` and the slice **inverted** — the test panicked
  `byte range starts at 187645 but ends at 74292` instead of asserting anything. Marker corrected; the
  law now evaluates (and still fails on 14 further stale markers — §3.A2).

### 2.5 Dock: silhouette clip count, and the staged-vs-published hit registry
* `🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs:247` expected `clip.scissors.len() == 3`; it is 2. W1i made the
  tab chips paint **edge to edge** (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` — "`layout_stack_cap` advances by
  the chip width alone"), so the two chips form ONE silhouette top span and
  `content_clip_rects()` = body + 1 span. That is the React-parity-correct choice: React's tab bar is
  `flex min-w-0 items-stretch justify-start overflow-x-auto` with **no gap utility**
  (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1108`). Expectation updated to `Some(2)`. **Passes.**
* `:266` `resize_hits_win_over_later_scroll_region` called `input.hit_at(…)` without ever publishing
  the frame's registry. `register_hit` only **stages**; `hit_at` resolves the **published** registry
  (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:312` vs `:346`) — identical at HEAD, so the law had been
  asserting against an empty registry. Added `input.publish_hits();`. **Passes.**

### 2.6 `#[cfg(test)]` import that production now calls — a real Wave 1 break
`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:10-12` gated
`use crate::scenes::queue_canvas_image_upload_sized;` behind `#[cfg(test)]`, while the production
UI-image path calls it at `:1763`. Merged into the ungated
`use crate::scenes::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with, render_component_scene_step};`
(this also collapsed two separate `use crate::scenes::` lines into one). This was the last native
compile error; gate 2 was red on it.

### 2.7 The `WORLD3D_SHADER` twin (W1f gap 4) — aligned across all four backends
W1f gave the wgpu `WORLD3D_SHADER` a per-vertex colour so terrain tints, and left the
`🖌️render/✨️shader-contract` copy (which webgpu/d3d12/metal/vulkan all build from) without it.
Aligned:
* `🖱️ui/🖌️render/✨️shader-contract/🦀️.rs:546-549` — `VertexInput` gains `@location(2) color: vec4<f32>`;
  `:580` `out.color = instance.color * vertex.color;`
* `:600-609` — `WORLD3D_MESH_VERTEX_BUFFERS` vertex stride `24 → 40` with
  `shader_location: 2, Float32x4, offset: 24`, matching the canonical wgpu layout
  (`🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2441-2443`, `:2493-2495`) and `World3dVertex`'s three fields (`:227`).
* `🖌️render/🎯️targets/🧊️webgpu/🗃️resources/🦀️.rs` — the second `World3dVertex` gains `color: [f32; 4]`,
  `upload_mesh` fills identity white (this path has no per-vertex colour source), docstring updated.
* `🖌️render/🎯️targets/🪟️d3d12/✨️hlsl/🦀️.rs` — `VSInput` gains `float4 vertex_color : ATTRIB2`,
  `out_v.color = input.color * input.vertex_color;` (named `vertex_color` because the instance tint
  already owns `color` at ATTRIB7).
* `🖌️render/🎯️targets/🍎️metal/✨️msl/🦀️.rs` — same, `[[attribute(2)]]`.

`cargo test -p semio-framework-ui-render --lib` is 129/0, so the contract's own structural WGSL
validator and its `pipeline_entry_points_exist_in_shader` law accept the result.

### 2.8 Committed slot-table budget receipts re-synced (twice)
`🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` —
`engine_canvas::EngineSurfaceRegistry` `elementSizeBytes` `67528 → 74608 → 75032` and
`scenes::AdmittedSurfaceMap<World3dState>` `21952/47008 → 22464/48032`. Wave 1 grew both slot
structs. **This receipt is volatile**: it moved twice inside this pass because packet owners kept
adding scene fields, so whoever lands the next scene field must re-measure it.

### 2.9 Boot-query capacity law re-pointed at its new home
`🧪️tests/📨️browser-frame-transport/🟦️.ts:423` asserted
`bootSource.indexOf("location.search.length") < bootSource.indexOf("new URLSearchParams")`. W1d moved
boot-query parsing out of `🚀️browser-boot/🟦️.ts` into the schema-owned
`🧭️boot-descriptor/🟦️.ts`, where the same bound is enforced by `boundedLocation`
(`:117-120`, `WGPU_BOOT_LOCATION_CAPACITY`) **before** `new URLSearchParams`
(`:157`) — so the guard survived, only its address changed. The law now reads the descriptor module,
and additionally asserts the page boot no longer parses the query itself. **Passes** (gate 8g went
78/3 → 79/2).

### 2.10 Chord laws ported to the one remappable shortcut table
`🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs:331-358` called `shell_role_chord` /
`shell_mode_step_chord`, which W1c deleted along with the open-coded `meta && Char(..)` ladder they
read. There is exactly one successor: `shell_shortcut_for(action, modifiers) -> Option<ShellShortcut>`
over `SHELL_SHORTCUT_ROWS` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13256`, `:13164-13218`), the Rust twin of
React's `SHELL_KEYBINDINGS`. Ported to `ShellShortcut::SurfaceRole(role)` /
`ShellShortcut::ModeStep(step)`; the two "a role chord is not a mode chord" halves became
`!matches!(…, Some(ShellShortcut::ModeStep(_)))` so the disjointness intent survives the merge.
**Passes.**

### 2.11 Reviewer items folded in

1. **`ICON_CHEVRON` token + tree chevron call site — no change needed; the premise does not hold.**
   The token already exists under the name W1o gave it: `chrome::SIZE_TINY`
   (`🎯️targets/🧊️wgpu/🖥️chrome/🦀️.rs:20`) = `UI_SPACING_COMPACT_PX × chrome::SIZE_TINY_UI_SPACING`
   = 3.2 × 3.0 = **9.6 px**, emitted from `🎨️styling/🔣️.json:353 "sizeTinyUiSpacing": 3.0`. It is
   already the size both tree **row** toggles use (`🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:207`). The two
   `ICON_TINY` (16 px) call sites the review flagged — `🪀️widgets/🦀️.rs:391` and
   `🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:174` — are both **section headers**
   (`WidgetNode::Section` at `PANEL_HEADER`, and `render_tree_section_header`), and React draws those
   at `size-small` = 16 px: `treeSectionChevronClassName = "size-small flex-shrink-0 …"`
   (`🌳️Tree/🟦️.tsx:251`). The row toggle is `size-tiny` (`:4576`). So all four call sites were
   already correct and the codegen needs nothing. What WAS wrong is the helper's docstring, which
   invented a non-existent `chrome::ICON_CHEVRON`: `🪀️widgets/🦀️.rs:498` now names `SIZE_TINY` /
   `ICON_TINY` with both React lines. (`ICON_TREE_ROW` = 12 px is the third, distinct box — a row's
   leading icon; `🔬️targets-wgpu-widget-metrics/🦀️.rs:26` pins all three apart.)
2. **`w1c`'s native check** — confirmed covered by gate 2 (0 errors, `Checking …` present).
3. **Commented-out `push_window_silhouette_border`** — deleted
   (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`, end of `render_stack`), together with the two bindings that
   existed only to feed it (`let _stroke = theme.stroke_hairline;` and the seven-line `let _border =
   …` ladder, plus its now-unused `stack_hovered`). The function itself stays: `🐚️Shell` imports it
   (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:10`) and `🔬️wgpu-chrome-overlays-tour/🦀️.rs:393` asserts on it.
4. **Six blank lines in `📐️flex`** — squeezed; the file had two runs (one of six around
   `//#region 🎨️StyleMapping`, one of two before `fn gap_size`), both collapsed to one.
5. Inline-comment hygiene left to a separate packet as instructed.

### 2.12 One `[DEBUG]` print demoted
`⌨️native-entrypoint/🦀️.rs:118` — `eprintln!("[DEBUG] native boot descriptor rejected: {error}")`
immediately precedes `std::process::exit(1)`, so it is a permanent fatal diagnostic, not temporary
instrumentation; prefix dropped. **Not touched:** the 93 `[DEBUG]` lines in
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (Wave 1's net contribution is 3) and the per-frame world3d dump at
`♾️infinite/🌍️world/🦀️.rs:1516` (byte-identical to HEAD, and it floods gate 6's log with ~4 KB per
paint). Both are that file's established diagnostic channel, pre-date this wave, and belong to the
hygiene packet — but the world3d one should be gated, it is the single largest source of noise in the
renderer test log.

---

## 3. Pre-existing failures, with evidence

`git stash` is forbidden, so every claim below is either "the file is byte-identical to HEAD"
(`git status --porcelain` / `git diff HEAD`) or "the law's inputs evaluate to the same result against
`git show HEAD:<file>`" (I exported the HEAD blobs to a scratch dir and re-ran the laws' own logic in
Python over both trees).

### 3.A ui crate (gate 5) — 5 failures, one root cause, structurally impossible at HEAD too

```
wgpu::engine::retained_document_hostile_fixtures::max_plus_one_stale_aba_… (ArenaFull)
wgpu::prepared::tests::preparation_yields_at_the_configured_item_budget
wgpu::prepared::tests::receiver_survives_worker_ownership_of_the_job
wgpu::prepared::tests::retained_codec_source_moves_once_and_retires_one_page_per_governed_step
wgpu::reconcile::document_tree_reconcile_tests::a_surfaces_second_document_reaches_the_arena_…
```

* **Files:** `🧬️contract/🎟️resident/🦀️.rs`, `🧬️contract/📃️document/🦀️.rs`, `🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs`
  and all five test files are **unmodified vs HEAD**. The only two `🧬️contract` diffs in the whole wave
  are a const **string value** (`TREE_WINDOW_PATH_SEPARATOR` `"\u{1f}"` → `"␟"`) and a new
  **standalone** `struct EdgePx` — neither changes `size_of::<UiNodeRecord>()`, so
  `UI_RESIDENT_DOCUMENT_BYTES` and `UI_RESIDENT_AGGREGATE_BYTES` are bit-identical to HEAD.
* **Root cause:** `UiDocumentArena::reserve` (`📃️document/🦀️.rs:608`) prices EVERY document at the
  per-surface **ceiling** — `UiResidentLimits { items: UI_RESIDENT_SURFACE_ITEMS /*4097*/, bytes:
  UI_RESIDENT_SURFACE_BYTES /*8 MiB*/ }` — against an aggregate of
  `UI_RESIDENT_SLOTS × UI_DOCUMENT_NODES × size_of::<UiNodeRecord>()` = `64 × 128 × size_of(record)`.
  The item ledger alone caps admissions at `floor(131076 / 4097) = 31 < UI_RESIDENT_SLOTS = 64`, so
  the hostile fixture's loop over all 64 slots **cannot** succeed on any tree. This is the exact
  category error the aggregate's own docstring (`🎟️resident/🦀️.rs:22-31`) says was fixed one level up
  ("a maximum a surface may reach, never a price") and that `reserve` still commits.
* **Cascade:** the hostile test leaks its ~31 reservations when it panics, which is why the three
  `prepared` cases and the `reconcile` case **pass in isolation and fail in-suite** (verified: each
  green under `-- --exact <name>`). That is the "rotating `UiResidentPermit` refusals" the packets
  reported — the names rotate because which test is starved depends on run order.
* **Recommendation (own ticket):** price a document reservation from its census, as
  `try_reprice`'s docstring already prescribes, instead of reserving the ceiling up front.

The scene golden-JSON drift the packets also listed under "pre-existing" was **not** pre-existing —
it was W1's own Board2d field addition, and it is fixed (§2.1). The `mounted_layout` alignment
failures and the `large_layout_and_shaping_job_…_below_eight_ms` wall-clock law that W1g/W1l
reported are **no longer failing** at all.

### 3.A2 os renderer (gate 6) — 3 laws proven identical at HEAD

| Test | Evidence |
|---|---|
| `async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied` | I re-implemented `presenter_retirement_contract`'s 60 clauses in Python and ran it over the worktree and over the HEAD blobs of all six sources it reads. **Identical failing set, 7 clauses, zero new** (`glue`'s `packet.scene_revision() != expected.scene_revision …`, `runtime_presentation_authority_and_candidate_identity_change_independently`, three `matches(…).count()` laws, and two `draw` test-name markers). |
| `async_boundary_tests::raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete` | Same method over `retained_raster_contract`'s 101 clauses. **Identical failing set, 15 clauses, zero new.** Fourteen of them name APIs that exist nowhere in the tree (`validate_engine_target_texture_allocation`, `validate_engine_replacement_view_allocation`, `retain_engine_allocation_fault`, `surface.texture = texture`, …) and did not exist at HEAD either, so repairing this law means re-deriving it against a refactor that predates this ticket. My §2.4 fix removed its *panic*; the assertion itself still fails. |
| `deadlines::tests::caret_blink_toggles_and_rearms_on_fire` | `⏰️deadlines/🦀️.rs`, `🖌️render/⏱️schedule/🦀️.rs` (which owns `FrameScheduler::request_deadline`/`next_deadline`) and `🔬️wgpu-deadlines-unit/🦀️.rs` are **all unmodified**. `CaretBlink::fire` correctly arms `now + CARET_BLINK_SECONDS` = 1.0, but the scheduler is an append-only deadline list and the test never drains the 0.5 one it armed at `sync(0.0)`, so `next_deadline()` answers the earlier 0.5. Fixing it means either draining in the law or making `fire` supersede its own deadline — a repo-wide scheduler semantics decision I did not take unilaterally. |

### 3.B os renderer (gate 6) — the same fixed-pool exhaustion family, 15 failures

```
kernel_runtime::semantic_document_tests::command_batch_ninth_document_is_retained_…
shell::shell_document_retirement_tests::shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation
shell::shell_document_retirement_tests::shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement
shell::tool_run_panel_tests::tool_run_panel_of_a_running_run_paints_and_its_buttons_dispatch_the_run
shell::tool_run_panel_tests::tool_run_panel_buttons_are_keyboard_reachable
shell::window_measures_tests::window_measures_overlay_paints_and_dispatches_every_gesture_like_react
interpreter::render_plan_validator_tests::inline_svg_saturation_rejects_before_parse_or_source_copy
interpreter::render_plan_validator_tests::resolve_ui_image_decodes_inline_svg_data_url_at_natural_aspect_ratio
interpreter::render_plan_validator_tests::resolve_ui_image_decodes_plain_utf8_svg_data_url
scenes::raster_frame_cost_tests::admission_saturation_runs_no_hash_dimension_or_pixel_materialization
scenes::raster_frame_cost_tests::checked_out_drop_hands_back_exact_fifo_owner_and_rejects_aba
scenes::raster_frame_cost_tests::pending_raster_ring_is_fixed_fifo_and_returns_cap_plus_one_owner
scenes::raster_frame_cost_tests::queue_canvas_image_upload_redecodes_when_source_changes
scenes::raster_frame_cost_tests::queue_canvas_image_upload_requeues_stable_key_without_unbounded_digest
scenes::raster_frame_cost_tests::realm_close_retires_pending_rasters_before_terminal_and_allows_clean_reopen_fixture
```

Their messages are `(ArenaFull, SurfaceId(…))`, `Permit { fault: Capacity, items: 13, bytes: 85314 }`,
`"retained document permit failed: Capacity (resident capacity exhausted)"` and
`"raster producer process credits exhausted"` — i.e. §3.A's ledger plus the analogous fixed raster
slot pool, both in unmodified code. Two behave the two documented ways:
`shell_absolute_refusal_…` and `tool_run_panel_of_a_running_run_…` fail **alone** (deterministic
consequence of the 8 MiB-against-8 MiB ceiling); `queue_canvas_image_upload_redecodes_when_source_changes`
**passes alone** (starved by an earlier case in the same binary).

### 3.C Order- and env-dependent, pre-existing

* `shell::ui_prefs_themes_i18n_tests::env_lock_ignores_unset_and_empty` and
  `…::shell_pref_locks_reads_the_four_lockable_envs` — `left: Some("dark") right: None`: two laws
  setting the same process env vars in one test binary. Neither file is in the Wave 1 diff.
* Gate 8g's two remaining TS failures, both with unmodified test files **and** unmodified subjects:
  `⏱️wgpu-worker-step-budget` expects `🎞️frame-worker/🟦️.ts` to contain `phaseUs=${step.elapsedUs}` —
  **0 occurrences at HEAD** as well; `⏱️wgpu-ui-turn-budget`'s diagnostics-switch law reads
  `⏱️turn-budget/🟦️.ts`, which `git status` reports clean.

### 3.D os infinite (gate 7) — 7 failures, all outside the Wave 1 surface

```
world::tests::live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption
world::tests::prepared_world_resources_are_send_and_deduplicate_uploads
world::tests::world_authority_retains_front_plan_across_output_saturation_and_retries_in_order
world::tests::world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry
world::tests::world_component_marquee_publish_merges_before_one_atomic_set_selection
world::tests::world_object_registry_enforces_capacity_revision_and_aba
world::tests::world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer
```

None of these seven functions appears in `git diff HEAD` of
`♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`, and W1f's 567-line diff of `🌍️world/🦀️.rs` is confined to
terrain (`@@ 6932-7450`), the camera plan / `setCamera` shape (`@@ 6275-6412`), the context menu
(`@@ 4531`, `@@ 10876`), the gumball (`@@ 9131`), one pointer-button hunk (`@@ 10862`) and one new
`World3dState` field — no hunk touches the object registry, the marquee component-id path, the
saturation owner, the prepared-resources dedup or the `exact(glue, host, native, browser)` source law.
`terrain` (26) and `pointer_gesture` (7) are fully green. This matches W1f's own count of 7.

### 3.E Known other-lane failures the packets already named

* `shell::command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss`
  — flagged by both W1c and W1h as 🛂️SpaceAdministration/`DirectoryClient` bootstrap-epoch machinery.
* `shell::shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel`
  — `left: 151.2 right: 7.2`, exactly one status-pill width plus a gap, in the case that passes
  `panels: &[]`. W1h's analysis holds: the test's *flush* branch assumes no pill while its *reserved*
  branch assumes one, so it is internally inconsistent with the current
  `world3d_status_pill_for` visibility rule. Belongs to whoever changed pill visibility; I did not
  edit another lane's law to hide it.

---

## 4. Conflicts between packets, and how they were resolved

| Conflict | Resolution |
|---|---|
| W1a and W1b both owned `scene_action` / `render_placeholder` / `draw_ink_rect_outline` | No duplicate exists in the tree: W1a cut its copies from the include and left the production ones. Verified by grep — exactly one definition of each. |
| Pick granularity: `🧩️wgpu-engine-surfaces` said `"handle"`, `🌍️world` says `"object"` | `"object"`, cited to `🖱️pointer-gestures.json` (all seven `interactionSelect` cases) and `interactionTargetsForInstances`. §2.3. |
| `targets` as nested array (test) vs JSON text (producer) | JSON text — the producer's bounded-string-credit design is deliberate and documented at `🌍️world/🦀️.rs:3360`. §2.3. |
| Silhouette clip: 3 scissors (test) vs 2 (W1i's edge-to-edge chips) | 2, cited to `🎨️Canvas/🟦️.tsx:1108` (no gap utility in the tab bar). §2.5. |
| `DockState::remove_window` (W1i G2 kept it) vs `close_window` (W1i's new React-parity collapse) | Resolved by the packet owner **during** this pass: they removed `remove_window` and repaired all 13 call sites within 70 s. I deliberately did not race them. |
| Tree chevron size: review said `ICON_TINY` (16 px) on a row, W1o says `SIZE_TINY` (9.6 px) | W1o is already right in the code; only the docstring was wrong. §2.11.1. |
| `WORLD3D_SHADER` wgpu copy (vertex colour) vs `✨️shader-contract` copy (none) | wgpu copy wins; the contract and its HLSL/MSL/webgpu mirrors were brought up to it. §2.7. |
| Boot-query guard: law in `🚀️browser-boot`, code in `🧭️boot-descriptor` | Code wins — the schema-owned parser is the React-parity home; the law follows it. §2.9. |
| Chord helpers: bespoke `shell_role_chord`/`shell_mode_step_chord` vs the `SHELL_SHORTCUT_ROWS` table | Table wins (React's `SHELL_KEYBINDINGS`). §2.10. |

---

## 5. Unresolved, and what the next pass must do

1. **Gate 6 is 752/32, not 752/0.** 20 of the 32 are classified above as pre-existing (§3.A2, §3.B,
   §3.C) or other-lane (§3.E). The remaining ~12 are in files their packet owners were **actively
   writing during and after this run** — `engine_canvas::paint2d_engine_tests` (3),
   `engine_canvas::board2d_engine_tests::transient_and_flush_now_tables_match_the_react_sets`,
   `engine_canvas::saturated_graph_and_board_wheel_queues_preserve_cameras`,
   `scenes::canvas2d_tests::canvas2d_pan_drag_schedules_a_settled_camera_dispatch`,
   `scenes::production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only`,
   `scenes::virtual_file_system_tests::chevron_press_toggles_expansion_and_dispatches_nothing`,
   `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`,
   `runtime_publication_tests::runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation`.
   They need one more integration sweep **after the fleet quiets** — they are not stable enough to fix
   from underneath their authors.
2. **Gate 5's final number is 506/5, measured before the last churn.** The ui lib-test binary is, at
   the time of writing, broken by an in-flight peer edit:
   `🔬️targets-wgpu-engine-unit/🦀️.rs:468,470` calls `ui.focused_node("w")`, a method that exists
   neither now nor at HEAD. Re-run gate 5 once that lands.
3. **Three `cargo check` runs in this pass produced bogus errors** and must not be trusted from their
   logs: one hit a corrupt incremental cache
   (`internal compiler error: encountered incremental compilation error with evaluate_obligation`,
   `Found unstable fingerprints`) and reported 27 phantom "cannot find struct `UiButtonNode`" errors
   against re-exports that are byte-identical to HEAD. `cargo clean -p semio-framework-os-renderer-wgpu`
   (906 files, 901 MiB) cleared it. Two others compiled a Dock/Shell snapshot taken mid-write by a
   peer. **Always re-run a red check once before believing it while the fleet is live.**
4. **Own tickets recommended:** (a) the resident-reservation pricing bug in §3.A — it is the single
   largest source of test noise across both crates; (b) the drop-witness-turns-failure-into-SIGABRT
   pattern that hid 737 of 784 renderer tests; (c) re-deriving `retained_raster_contract`'s 14 dead
   markers or deleting the law; (d) gating the per-frame world3d `[DEBUG]` dump.
