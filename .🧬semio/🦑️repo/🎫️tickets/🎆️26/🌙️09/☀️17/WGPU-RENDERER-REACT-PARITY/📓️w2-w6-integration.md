# 🧩️ Waves 2–6 integration — wgpu renderer React parity

Integrator pass over the packets `📓️w2a` … `📓️w6a`, continuing `📓️w1-integration.md`. Every command
below ran in the foreground, one `cargo` at a time, plain env, `-j 4`. Logs under `🗑️generated/w6int-*.txt`.

> ⚠️ **Two workers were live throughout.** W5c owns the GPU world pass (`🧊️gpu/🦀️.rs`, `🖍️draw/**`,
> `🧊️renderer/🦀️.rs`'s world-pass code and the wasm activation) and W7a owns the Shell overlay/tour,
> the chord-glyph formatter and the control ids. Failures whose evidence lives in those files are
> named and left alone (§4). A peer also began restructuring `✏️s/🔌️plugins/🌊️wfc` mid-pass, which
> broke `cargo metadata` for the whole workspace for a while — the final gate numbers were taken
> around that window and each one says which run it came from.

---

## 1. Gate results

| # | Gate | Command | Before | After |
|---|---|---|---|---|
| 1 | ui crate, wgpu engine | `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | 0 errors | **0 errors**, 4 warnings, `Finished` |
| 2 | os renderer, native + tests | `cargo check -p semio-framework-os-renderer-wgpu --lib --tests -j 4` | 0 errors | **0 errors**, 89 warnings, `Checking semio-framework-os-renderer-wgpu` present, `Finished` |
| 3 | os renderer, wasm32 | `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | 0 errors | **0 errors**, 69 warnings, `Finished` in 1m 07s |
| 4 | ui crate tests | `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4 -- --test-threads=1` | **506 / 5** (W1) | **557 / 0** |
| 5 | os renderer tests | `cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- --test-threads=1` | **771 / 33** (brief), 819 / 31 on my first run | **852 / 7** |
| 6 | os infinite world/terrain | `cargo test -p semio-framework-os-infinite --lib -j 4 -- --test-threads=1 world:: terrain` | 157 / 7 (W1) | **169 / 7** — the identical seven names (§5) |
| 7a | wgpu TS lint | `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-wgpu:lint` | exit 0 | **exit 0** |
| 7b | regenerate browser boot | `… :generate-browser-boot --skip-nx-cache` | exit 0 | **exit 0** |
| 7c | regenerate frame worker | `… :generate-frame-worker --skip-nx-cache` | exit 0 | **exit 0** |
| 7d | browser worker bundle | `… :check-browser-worker --skip-nx-cache` | exit 0 | **exit 0** |
| 7e | frame worker bundle | `… :check-frame-worker --skip-nx-cache` | exit 0 | **exit 0** |
| 7f | wgpu TS unit suites | `… :test-browser-worker --skip-nx-cache` | 89 / 2 (W6a) | **92 / 0** |
| 7g | styling codegen freshness | `NX_DAEMON=false bun nx run @semio-tech/ui-styling-tokens:check-generated --skip-nx-cache` | exit 0 | **exit 0** |
| 7h | nx `ui-rs:test-wgpu-engine` | `NX_DAEMON=false bun nx run @semio-tech/ui-rs:test-wgpu-engine` | — | red, **fail-fast on the wall-clock law of §6** (322 / 1 of 557 before the stop; gate 4 is the same binary at 557 / 0) |
| 7i | nx `framework-renderer-wgpu:test-wgpu-unit` | `NX_DAEMON=false bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit` | — | red, **fail-fast on the two `async_boundary_tests` source contracts** (§4.1), 52 / 2 of 859 before the stop |
| 7j | nx `semio-framework-os-infinite:test-wgpu-world-terrain` | `NX_DAEMON=false bun nx run semio-framework-os-infinite:test-wgpu-world-terrain` | — | red, **fail-fast on `live_renderer_retains_generation_wake…`** (§5), 82 / 1 of 177 before the stop |

W2h's three targets run under `nextest` with **fail-fast**, so 7h–7j stop at the first failure and never
report the whole suite; gates 4, 5 and 6 are the complete counts for those same three binaries. That is
worth knowing before anyone reads a `Summary [ … ] 83/177 tests run` as "94 tests are missing".

---

## 2. Root causes fixed

### 2.1 The resident-reservation pricing bug — gate 4's largest noise source
`🖱️ui/🧬️contract/📃️document/🦀️.rs`. `UiDocumentArena::reserve` priced EVERY cold document at the
per-surface **ceiling** (`UI_RESIDENT_SURFACE_ITEMS` = 4097 items, `UI_RESIDENT_SURFACE_BYTES` = 8 MiB)
against an aggregate of `UI_RESIDENT_AGGREGATE_ITEMS` = 131 076 items. The item ledger alone therefore
capped concurrent documents at `floor(131076 / 4097)` = **31** of the `UI_RESIDENT_SLOTS` = 64 slots the
slot ledger offers, so every hostile fixture that walks all sixty-four slots refused at the
thirty-second, leaked its ~31 reservations on the panic, and starved every later test in the same
process.

Fixed by giving the whole document contract ONE pricing arithmetic and pricing a cold root from its
census instead of the ceiling:

* `📃️document/🦀️.rs:451` — new `pub const fn ui_document_resident_limits(nodes: usize)`: the header, the
  terminal page and one record each at `size_of::<UiNodeRecord>()`, plus
  `UiDocumentAssembly::required_open_bytes()`, clamped at the per-surface ceiling. This is exactly the
  arithmetic `UiDocumentLease::try_publish` and the refresh laws' own `surface_limits` already ran.
* `:reserve` now reserves `ui_document_resident_limits(0)` (2 items / 14 738 bytes measured, against a
  ceiling of 4097 / 8 MiB).
* `UiDocumentSlot::climb_resident` (new) reprices the root UPWARD as records land — monotone, so a
  concurrent holder can never win the gap between two of one document's own pages — and `push` refuses
  with `ArenaFull` when the aggregate will not take the next record.
* `🎟️assembly/🦀️.rs` — `open_into`'s "cold convenience" stopped reserving the ceiling too, and
  `place_one` climbs through the same helper; `try_publish` and the refresh test's `surface_limits`
  now delegate to the one helper instead of repeating it.
* **Law**: `🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs`
  `cold_document_roots_are_priced_from_their_census_and_reach_every_slot` — opens a cold builder in every
  one of the `UI_DOCUMENT_LEASE_SLOTS`, proves the 65th is refused by the SLOT ledger (never by a price),
  and pins the aggregate cost at `slots × one-record price`.

Cleared `wgpu::engine::retained_document_hostile_fixtures::max_plus_one_stale_aba_…` and
`wgpu::reconcile::document_tree_reconcile_tests::a_surfaces_second_document_reaches_the_arena_…` in gate 4,
and the whole `shell::shell_document_retirement_*` / `window_measures_*` / `kernel_runtime` capacity
family in gate 5.

**Measured, not assumed:** the `semio-framework-ui-contract` crate's own `document::`/`resident::`
suites are red **at HEAD too**. I exported the HEAD blobs of the three files I changed, ran the suite
against them (14 passed / 29 failed), restored mine and re-ran (16 passed / 28 failed over 43 shared
cases): the change is +2 green, −1 red. That family's breakage predates this ticket and belongs to
whoever grew `UiNodeRecord`'s static backing (`contract_backing_bytes` reads 2 223 932 against a
committed 2 077 760).

### 2.2 The prepared-render process permit leak — gate 4's other three
`🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` + `🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs`. The three
`wgpu::prepared::tests::*` failures passed alone and failed in-suite. The panic message named nothing, so
I gave `PreparedRenderInput::new`'s `#[cfg(test)]` panic the rejection fault and the live ledger:
`prepared render process permits exhausted (held items 4/64 pages 12288/16383 backing-units 6168)` —
**three tests had each leaked 4 096 pages**, and the fourth claim of 4 096 overflowed the 16 383 ceiling
by exactly one page.

The leakers are the four tests that DROP a `PreparedRenderJob`/`PreparedRenderPacket` instead of closing
it: a dropped owner re-arms itself in the abandonment registry and waits for someone to drive
`close_abandoned_step`, which in a test binary is nobody. Each now calls the file's own
`drain_abandoned_preparations()` after the drop, so it leaves the process ledger as it found it:
`receiver_survives_worker_ownership_of_the_job`, `preparation_yields_at_the_configured_item_budget`,
`preparation_rejects_a_stale_generation_before_publication`,
`preparation_observes_cancellation_without_replacing_a_packet`. Module went 35 / 3 → **38 / 0**.

### 2.3 The raster producer ledger — seven gate-5 failures in one cascade
Two saturation laws (`inline_svg_saturation_rejects_before_parse_or_source_copy`,
`admission_saturation_runs_no_hash_dimension_or_pixel_materialization`) demanded all **256**
`PreparedRasterReservation` slots. The producer ledger is process-wide and every neighbouring test that
resolved a ui image or queued an upload still holds one, so both `expect`-panicked mid-loop — and the
panic then leaked the reservations they had already taken, which is why the entire
`scenes::raster_frame_cost_tests` + `interpreter::render_plan_validator_tests` family failed in-suite and
passed alone. Both now saturate FROM WHEREVER THE LEDGER IS (reserve until refusal, assert the refusal,
assert the set is non-empty so the law cannot pass vacuously) and release exactly what they took.

Two further order dependencies in the same family, both process-wide registries the laws assumed were
theirs alone:
* `checked_out_drop_hands_back_exact_fifo_owner_and_rejects_aba` — `PendingRasterUploadCursor` walks the
  WHOLE pending registry, so the first checkout was a foreign surface's producer
  (`slot 0 epoch 1` where the law expected `slot 5 epoch 3`). It now retires the authority first
  (`begin_pending_raster_authority_close` + `reset_pending_raster_authority`, the idiom its own neighbour
  `realm_close_…` already uses).
* `canvas2d_pan_drag_schedules_a_settled_camera_dispatch` — `sweep_expired_scene_camera_dispatches` drains
  every surface's deadline, so a neighbour's armed camera was counted (`2` where `1` was expected). The law
  now clears `SCENE_CAMERA_DISPATCH_DEADLINES_MS` first, as its own sibling law does.

### 2.4 A real publication race: a completion visible without its scene invalidation
`📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`, `enqueue_runtime_completion`. The order was
`enqueue → drop(queue) → mark_scene_changed()`. A reader landing in that window took the mailbox lock and
read `ready.len() == 1` beside the OLD presentation witness — a completion the frame it is about to
present does not know it must rebuild for. `runtime_publication_tests::runtime_single_enqueue_reader_
cannot_observe_completion_without_its_scene_invalidation` observed exactly that,
`(count 1, witness {scene_revision 1})` against a committed `{scene_revision 2}`, deterministically.
`mark_scene_changed` is a single `fetch_add` on an `AtomicU64`, so moving it above `drop(queue)` publishes
the completion and its invalidation as one observation and adds no lock order. **Green.**

### 2.5 The VFS chevron — a root elision applied to every top-level row
`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, `build_vfs_visible_rows`. React's
`buildVirtualFileSystemVisibleRows` elides exactly ONE node — the synthesized root, `fileNodeKindId:
"root"` — and starts its DFS at that root's children (`⚙️VirtualFileSystem/🟦️.tsx:417-424`). The wgpu port
elided EVERY top-level row carrying `hasChildren`, so an ordinary expandable folder at level 0 lost its
own row and its chevron. React keeps it: in its own law
(`🧪️owned-locale-detector-retirement/🟦️.tsx:9348`) `f1` carries `hasChildren: true` and stays a visible
row. Now gated on a new `VFS_ROOT_FILE_NODE_KIND` const with both citations in its docstring.
`chevron_press_toggles_expansion_and_dispatches_nothing` is green.

### 2.6 The stale production-ingress guard
`🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs:60`. The guard asserted `#[cfg(test)] pub fn
text_editor_apply_key` (and fifteen siblings) appear in the PRODUCTION `⚙️EngineCanvas` target, but the
2026-09-08 sweep moved every one of those wrappers into
`⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs`, so it could never hold (verified: 16/16 wrappers live in
the standalone file, 0/16 in the production target). Re-pointed at a new `ENGINE_CANVAS_STANDALONE`
constant and **strengthened**: each name must now also be ABSENT from the production target, which is what
"must not remain production-capable" actually means.

### 2.7 Two paint2d fixtures that never reached the host
`⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`.
* `raster_document_json()` omitted `schema` and per-layer `mask`. `parse_document` refuses anything whose
  `schema` is not `"raster.document"`, and `LayerNodeJson::Pixel` declares `mask: Option<MaskJson>` with no
  serde default. The attach path swallows the refusal (`let _ = host.sync_document_json(…)`,
  `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2801`), so the host ended up with NO layers — which surfaced as
  "the composited document has a pickable pixel layer somewhere in the viewport" rather than as a parse
  fault. The raster plugin's own snapshots carry both keys
  (`🖨️raster/…/📸️snapshot/⬅️before/🔣️.json`: `['blendMode','height','id','imageKey','kind','mask','name','opacity','transform','visible','width']`).
* `text_editor_scene` declared `selectionJson {start:0,end:0}` while its own assertions read `"alpha!"`
  and a caret of 6. The caret is part of the SCENE, not an editor default, so the fixture now declares the
  caret at the end of the buffer — which is what "a host that has just loaded a document and focused it"
  publishes. Module 5 / 2 → **7 / 0**.

### 2.8 The two env-lock laws, re-pointed at the boot descriptor
`🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs`. Both set `SEMIO_LOCKED_*` with `std::env::set_var`
and then read `env_lock`. The process env is NOT the source of truth:
`resolve_environment_boot_descriptor` reads those names ONCE, when the thread first touches
`BOOT_DESCRIPTOR` (`🧊️renderer/🦀️.rs:15530`), and every later read goes to the installed descriptor
(`:15631`) — so a `set_var` after that point can never reach `env_lock`, which is why W1d, W4a and W6a all
recorded these two as stale. They now install a lock set through the public
`apply_boot_descriptor` and restore the descriptor they found (new `with_boot_locks` helper). Module
**13 / 0**.

### 2.9 The caret-blink deadline
`📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-deadlines-unit/🦀️.rs`. `fire` is only ever reached AFTER the event
loop has consumed the deadline that woke it — `FrameScheduler::should_render` drains every `due <= now`
entry into `dirty` (`🖌️render/⏱️schedule/🦀️.rs:133-142`) and only then does the frame call `fire`. The law
skipped that drain, so the append-only list still held the half-period it had just consumed and
`next_deadline()` answered the old `0.5` instead of the re-armed `1.0`. Added the drain the production
order performs. No scheduler semantics were changed — W1 declined to take that decision unilaterally and
it turns out none was needed. Module **7 / 0**.

### 2.10 A page-size law written for eight slots
`📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs`.
`command_batch_ninth_document_…` filled a nine-owner list and then admitted `UI_DOCUMENT_LEASE_SLOTS` of
them — which has been **64** since the lease slots followed `UI_RESIDENT_SLOTS`, so the tenth
`swap_remove` panicked "first page owner" before the law asserted anything. Rebuilt from the page size
itself: one lease yields `UI_DOCUMENT_LEASE_ALIASES` owners, so a page needs
`UI_DOCUMENT_LEASE_SLOTS / UI_DOCUMENT_LEASE_ALIASES` leases; renamed
`command_batch_slots_plus_one_document_…`. Module **34 / 0**.

### 2.11 Two TS laws whose production print had been stripped
Both gate-7f failures were source-reading laws that W1 §3.C and W6a proved were "written ahead of their
production code". They were not — the code had been written and then **stripped by a `[DEBUG]`-removal
sweep**, leaving the gate standing over nothing:
* `⏱️turn-budget/🟦️.ts` — `TurnLedger::trace` was `if (!turnDiagnosticsEnabled()) return;` followed by an
  empty body, so every recorded/sustained overrun was invisible, which is the one thing this ledger
  exists to surface. Restored the `console.debug` line behind the same gate, in the shape the law pins
  (`[DEBUG] ${scope} ${verdict} site=… executing=…ms budget=…ms consecutive=… worst=…`).
* `🎞️frame-worker/🟦️.ts` — the bootstrap loop parsed `step.elapsedUs` and threw it away, so no Rust boot
  phase reported what it executed for. Now printed once per admitted phase behind the worker's own
  `diagnosticsStamp === true` gate.

Gate 7f went 89 / 2 → **92 / 0**, and 7b–7e were re-run and re-checked after the frame-worker edit.

### 2.12 The overlay row's flush assertion
`🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs:976` — see §3's first row. The wgpu lead is
React-correct, so the law now asserts the cancel FOLLOWS the pill in the same row and cites
`🌐️World3dHost/🟦️.tsx:4001,4018`. This closes a failure five packets in a row declined to own.

### 2.13 Committed receipts re-synced
* `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — `scenes::AdmittedSurfaceMap<World3dState>`
  `elementSizeBytes` 22 464 → **22 512**, `ownerSizeBytes` 48 032 → **48 128**. Same volatile receipt W1
  §2.8 re-synced twice; whoever lands the next `World3dState` field must re-measure it again.
* `🎨️theme` token law — `🐚️Shell/…:LOGO_UNTINTED` added to the ALLOWLIST of
  `🔬️targets-wgpu-theme-token-parity`. `Rgba::new(1.0, 1.0, 1.0, 1.0)` there is an identity TINT
  MULTIPLIER, not a paint: the brand mark is the one icon cell rasterised in its own four hues
  (`rasterize_svg(svg, tint_mask = id != "semio-logo")`) and React paints `<SemioLogo>` untinted, so any
  token would multiply the mark down to a single hue — the black disc that const exists to prevent. The
  allowlist entry (not the Shell source) was the right side to move, and it keeps me out of W7a's file.

---

## 3. Cross-packet conflicts, and how they were resolved

| Conflict | Resolution |
|---|---|
| Overlay row: `🔬️wgpu-shell-chrome-parity:976` asserts the World3d cancel is flush at `7.2`; `surface_overlay_controls_for` leads it by the status pill to `151.2` (W1h's, W2c's, W3b's, W4b's and W6a's shared open hand-off, red since Wave 1) | **The production side wins.** React paints the cancel `<button data-slot="world-compute-cancel">` INSIDE the status container, after the phase and progress spans, in one `flex items-center gap-single` row (`🌐️World3dHost/🟦️.tsx:4001,4018`) — which is exactly the lead `surface_status_pills_for`'s own docstring already describes. The law now asserts `control.x == pill.x + pill.w + gap_standard` and cites those lines. **Green.** |
| VFS root elision: every `hasChildren` top-level row (wgpu) vs exactly the synthesized root (React) | React. §2.5. |
| Text-editor caret: fixture `{0,0}` vs assertions `"alpha!"` / caret 6 | The assertions — the caret belongs to the scene, so the fixture declares it. §2.7. |
| Ingress guard: production `⚙️EngineCanvas` target vs the `#[cfg(test)]` standalone sibling | The standalone sibling, and the guard additionally asserts absence from production. §2.6. |
| `scene_action` defined twice — `🎞️Scenes` production and `⚙️EngineCanvas`'s standalone test lane | One definition. The `🎞️Scenes` one is now `pub(crate)` with a docstring and the EngineCanvas lane calls `crate::scenes::scene_action`. |
| Document pricing: the ceiling (`reserve`, `open_into`) vs the census (`try_publish`, the refresh laws) | The census, through one shared `ui_document_resident_limits`. §2.1. |

---

## 4. Still red in gate 5 (7), with attribution

1. **`async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied`** and
   **`…::raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete`** — the two 60/101-clause
   source contracts. Their inputs are `🧊️gpu/🦀️.rs`, `🖍️draw/🦀️.rs` and `🧊️renderer/🦀️.rs`, i.e. **W5c's
   live GPU world-pass files**; repairing markers against a file being rewritten underneath would race
   them. W1 §3.A2 already proved the failing clause sets identical at HEAD (7 and 15 clauses, 14 of them
   naming APIs that exist nowhere in the tree) and recommended re-deriving or deleting
   `retained_raster_contract` as its **own ticket**. Unchanged.
2. **`shell::tool_run_panel_tests::tool_run_panel_of_a_running_run_paints_and_its_buttons_dispatch_the_run`**
   and **`…::tool_run_panel_buttons_are_keyboard_reachable`** — these were attributed to the resident-arena
   capacity family by W3b/W4b/W6a. That capacity fault is now gone and the **real** failure is visible:
   the panel paints (`instances > 0`, `[DEBUG] ui-doc ingress window=framework.panel.toolRun … nodes=11`)
   but `engine.window_hit_targets("framework.panel.toolRun")` answers **nothing**, so
   `[STATS] tool-run panel targets []` and the keyboard walk lands on Abort instead of Pause. The fixture's
   four buttons all carry keys and `activate` bindings, so this is the retained paint/hit registration for a
   panel-tab surface, in `🐚️Shell` + `🗣️Interpreter` — **W7a's live file**. Deterministic, fails alone,
   needs its own packet with a sharper diagnosis than "capacity".
3. **`shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`** —
   "gap baseline must sit under the cutout between tabs and controls". Window-cap silhouette geometry in the
   **Shell overlay/tour region W7a owns**; named by W4a §1 and W3b as not theirs either.
4. **`shell::command_registry_tests::directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss`**
   — "rebootstrap resets the frontier". `DirectoryClient`/bootstrap-epoch machinery in the
   🛂️SpaceAdministration lane; recorded by W1c, W1h and W2c, unchanged here.
5. **`engine_canvas::saturated_graph_and_board_wheel_queues_preserve_cameras`** — the final assertion,
   `!defers_descriptor_sync_from_js()` after the retry: the retried `puzzle_board_pointer_up_into` answers
   `Ok(true)` and its `commit_board_pointer` runs, yet the board engine still reports a live gesture
   (`transform_drag`/`region_drag`/`interaction`). That is a board-engine ownership question in
   `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:8798`, not a wgpu one — a refused publish must not
   strand the gesture the retry then completes. Deterministic; belongs to the board lane.

Every one of the seven fails **alone as well as in-suite**, so none is an ordering artefact.

`shell::shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel` — red since W1h
and named by W1i, W2c, W3b, W4b and W6a as somebody else's — **is fixed** (§3, first row): the production
lead is React-correct and the law was asserting the wrong side.

---

## 5. Gate 6 — the seven world failures, re-proven

```
world::tests::live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption
world::tests::prepared_world_resources_are_send_and_deduplicate_uploads
world::tests::world_authority_retains_front_plan_across_output_saturation_and_retries_in_order
world::tests::world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry
world::tests::world_component_marquee_publish_merges_before_one_atomic_set_selection
world::tests::world_object_registry_enforces_capacity_revision_and_aba
world::tests::world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer
```

Byte-for-byte the set `📓️w1-integration.md` §3.D proved outside the Wave-1 surface, at the same count,
with `terrain` (26) and `pointer_gesture` fully green and the pass count risen 157 → 169. Their messages
are world-authority semantics, not wgpu parity:
`exact(glue, host, native, browser)` source law; `1 != 2` upload dedup; `Pending != OutputBlocked`;
`ParseIntError` parsing a `screen_select_components` id as a `u32` (the legacy oracle returns
`Vec<String>` and no longer yields bare numbers); `ids == [int(7), int(8)]`; a resolved replacement token;
`0 != 1` FIFO transfer. Two of them (`world_component_marquee_*`) look like the same id-shape drift W1 §2.3
fixed one level up for pick granularity, but the fix belongs to the world lane and `🌍️world` sits beside
W5c's live pass. **Proven, not fixed.**

---

## 6. Known flake

`wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` is a
WALL-CLOCK law. It failed at 35.8 ms and 27.5 ms during two loaded runs and passes alone and in the final
gate-4 run (557 / 0). W2a recorded 174 ms under a full fleet. It measures the machine as much as the code;
under a live fleet it is not a signal.

---

## 7. Consistency sweep

* **Duplicate helpers across packets** — one real duplicate: `scene_action` (§3). The other repeated names
  are `cfg`-exclusive arms (`apply_clipboard_paste_requested` ×3, `write_os_clipboard` ×2), WGSL functions
  inside two shader strings (`sdf_rounded_rect`, `vs_main`, `fs_main`), per-test-module locals (`leaf`,
  `stack`, `hit`, `repo_root`, `now_ms`, `component_scene_ui`) and the deliberate
  `production_fn_into` / `#[cfg(test)] pub fn production_fn` wrapper pairs the §2.6 guard now pins.
* **`#[cfg(test)]` on production-called items** and **dangling `#[path]`** — both are compile errors in a
  non-test build, and gates 1, 2 and 3 are green with warnings present, so the tree is clean on both. (Gate 2
  builds `--lib --tests`, gates 1 and 3 build `--lib` alone, which is what proves the `cfg(test)` half.)
* **`[DEBUG]` prints outside the diagnostics gate** — every `[DEBUG]` line Waves 1–6 added to a production
  wgpu file goes through `log_debug_diagnostic` / `debug_log_diagnostic`
  (`semio_framework_trace::runtime_diagnostics_enabled()`) except one: the browser worker's panic hook,
  which is a permanent fatal diagnostic rather than temporary instrumentation. Prefix dropped
  (`🌐️browser-worker/🦀️.rs:735`), the same call W1 §2.12 made for the native entrypoint. The 93 `[DEBUG]`
  lines in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` are untouched: W7a's file.
* **`//` comments inside function bodies** — a brace-depth analyser over the 114 wgpu Rust files the wave
  diff touches (`git diff 1ca5d400ae`, the ticket's own start commit; W5c's and W7a's files excluded) found
  **65 wave-added comment blocks / 200 lines** genuinely inside a body. **64 were hoisted** into the
  enclosing function's docstring by an index-stable codemod: each block's text became `///` lines appended
  after the last existing doc line of that item (or at the top of its attribute run), and the body lines
  were removed. Verified: every removed line is a `//` comment and every added line is a `///` doc line
  (0 exceptions across all 26 files), and `cargo check --lib --tests` is green for both crates afterwards.
  The **first attempt corrupted the edits** — bottom-up splicing shifted the earlier blocks' indices by the
  lines the later block had inserted, which duplicated one block and deleted two innocent comment lines; all
  26 files were restored from a pre-edit snapshot and the codemod rewritten as a single marking pass. The
  one block NOT hoisted is in `🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs`, deliberately left for
  W7a.
* **Six-blank-line runs** — one found and squeezed, before `scene_action` in `🎞️Scenes`.

---

## 8. Two red runs that were not real

Both were re-run once before being believed, which W1 §5.3 already recommends and which this pass
confirms twice over:

* **Gate 3** failed once with `no method named 'fault' found for &mut PreparedRenderJob … private field,
  not a method` at `🧊️renderer/🦀️.rs:12904` — W5c's own `w7b` diagnostic line, compiled against a
  `🎟️prepared/🦀️.rs` snapshot taken a moment before they added `pub fn fault(&self) -> Option<&'static str>`
  (`:2331`). The immediate re-run is green with 69 crate warnings emitted, which is what proves the crate
  actually type-checked rather than replaying a stale success. This is also the one gate the native check
  cannot stand in for: the line is wasm-only.
* **Nx 7j** failed once with `cannot borrow 'producer' as mutable` in `semio-framework-os-infinite`, a
  crate this pass never opened. It compiled again four polls later and the target reached its tests.

A third interruption was not a false red but a real stop: a peer began restructuring
`✏️s/🔌️plugins/🌊️wfc` and `cargo metadata` refused the whole workspace
(`failed to read ✏️s/🔌️plugins/🌊️wfc/📦️packages/🦀️rust/Cargo.toml`) until they finished.

## 9. Housekeeping the fleet forced

The volume ran **out of disk three times** mid-pass — once from `rustc` (`failed to write … full.rmeta:
No space left on device`), once so completely that the tool harness could not even open its own output
file, and once more during the final gate-4 run. `⚡️cache` was 343 GB: `cargo/build` 307 GB
(`debug/incremental` 26 GB, `wasm32-unknown-unknown` 23 GB), `nx` 28 GB across 739 hash directories.

Pruning `cargo/build/**/incremental` at `-mindepth 2` and `nx`'s hash directories (never
`terminalOutputs`) recovered 1 → 12 GB per sweep at `-mmin +180`, then `+45`, then `+15` as the window
closed; a background loop repeated it every three minutes for the rest of the pass, and the last gate-4
run still had to go out with `CARGO_INCREMENTAL=0`. **No prune touches a live peer's build** — an
in-flight session is minutes old — but the cache regrows at roughly 10 GB/h under a test fleet, so this
wants a standing sweep rather than an integrator noticing the volume is full.

---

## 10. What the next pass must do

1. **`retained_raster_contract` and `presenter_retirement_contract`** (§4.1) — own ticket, after W5c lands.
   Fourteen of the clauses name APIs that have never existed in this tree.
2. **The tool-run panel's hit registration** (§4.2) — now a sharp, reproducible question rather than a
   capacity excuse.
3. **`semio-framework-ui-contract`'s own `document::`/`resident::` suites** — red at HEAD, 28 failures, the
   first one aborting the binary through a drop witness. The committed `contract_backing_bytes` receipt is
   stale by 146 172 bytes.
4. **The board gesture strand** (§4.5) and **the seven world-authority laws** (§5).
5. The remaining 151 in-body comment blocks that Waves 1–6 did NOT add (pre-existing, outside this sweep's
   mandate) are still there, plus the one left for W7a.
