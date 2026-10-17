# 🔏️ Result RENDERER-PRIVACY (slice `rpv`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command (read-only `git diff --stat/--numstat`, `git grep`, `git ls-files`), no ticket tool. `F` = this folder. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `EL` = `E/🧱️elements`. Tools: `F/rpv-tools/` (section 9) over the round-2 oracle `F/pos-tools/` (read only, run with `POS_PREFIX=rpv`, sandboxes in `scratchpad/rpv-work/`). Logs, JSON and reports: `F/🗑️generated/rpv-*` (nothing deleted). Every repo edit was an anchored replacement with an asserted count through `rpv-tools/ed.py`, journalled in `F/🗑️generated/rpv-edits.log` (51 edits, 26 files). Crate names below drop the prefix `semio-framework-os-renderer-wgpu-` / `semio_framework_os_renderer_wgpu_`.

## 0. Verdict

**GREEN against the oracle, UNCOMPILED.** On the `ri2` family (76 crates) the round-2 oracle reaches body type-check in every run (canary E0308, E0616, E0624 raised) and reports **298 privacy errors in lib and 525 in `--test` before, 0 and 0 after**. Also 0 on the 92-crate family `ri3bins` (the 76 plus the 14 `shell-tests-*` bins, `frame-tests-1`, `fixtures`), lib and test. Six seeded mutations are all caught (section 5.3).

1. **Attribution (section 2):** the 298 lib rows (525 test) sit in 12 (14) owner crates and 119 members. 263 of the 298 disappeared when slice `rw2t` widened the WP2 files while I worked (evidence in 2.2); the 35 that remained at my start were 25 first-run rows of five crates outside WP2 plus 10 cascade rows, and every later round added the rows that had been hidden behind error-typed receivers.
2. **Fix (section 3):** per member one of five forms, never a blanket `pub`: plain `pub` where the member is cross-crate API (records, DTOs, identity fields), a **narrow owner method** where a consumer reached into internals (`renderer_io_sessions_pending`, `RetainedSurface::has_published`), `pub(crate)` where the consumer is a sibling module of the same crate (4 methods), a **`testkit`-gated `#[doc(hidden)] pub`** for test-only hooks (24 members, with the feature plumbed and checked), and an unchanged private field where nothing reaches it. API growth per crate in section 4: 40 public members and 24 gated hooks in 14 crates.
3. **New finding (section 6): the oracle is blind to E0451** (private field in a struct literal or struct pattern) because rustc runs that pass only when no other error exists (micro tests). A static scan (`rpv-tools/lit.py`) found **17 sites** (4 lib, 13 test) in the tree; all are fixed, the scan reports 0 now, and its self-test and the before/after prove it can see them.
4. A diff of the non-stub rows (`z1` against `a1`) caught one real defect of my own edit (the facade `pub(crate) use` list of host-base names lacked the new function); fixed, and the diff now shows only rows in code I did not touch.

Nothing was compiled by cargo (compile admission is refused this wave). Real-check commands: section 7.

## 1. Found state

* Input: `RESULT-PRIVACY-ORACLE-SWEEP-2.md` (298 privacy rows in `ri2`, untouched by it), `RESULT-RENDERER-INTEGRATION-2.md` (every `pub(crate|super|in)` of the flipped crates became `pub`; the crate roots own explicit `pub(crate) use` lists of host-base names), `RESULT-RENDERER-INTEGRATION-3.md` (5 `SceneSurfaceState` rows handed off; `cfg(test)` items used across crates handed to `rtb`), `RESULT-RENDERER-WP2-TESTS.md` (written at 22:01, section 3.4 and 6.3).
* Run `a1` (`cd F/pos-tools && POS_PREFIX=rpv python3 pos.py run families/ri2.json --cfgs lib,test --label a1 --no-base`, 21:13 to 21:18, tree of 21:13, absolute rows, canary `E0308 E0616 E0624` reached in both configurations): **lib privacy 298, test privacy 525** (plus the 2 canary rows each), identical to the sweep's 298. Other categories of that run for orientation: lib resolve 118, type 461, other 1; test resolve 149, type 602, other 2 (stub cascades and documented artefacts, not touched).
* Concurrency: `rw2t` widened WP2 members between 21:15 and 21:30 (its list `F/rw2t-tools/touched.txt`); my second run `a2` therefore started from a different tree than `a1`. Every file I edited was re-read by `ed.py` in the same call that wrote it.

## 2. Attribution of the 298 (lib) and 525 (test) rows

`rpv-tools/attr.py` reads the privacy diagnostics of a run (`*-priv.err`), takes the owner crate from the `__m_<crate>` head of the struct path (E0616) or from the "defined here" span (E0624, E0603), finds the definition line in the repo file, and counts rows per user crate. Result for `a1`: `F/🗑️generated/rpv-a1-attr.json` (827 rows, 119 owner members plus the 2 canary members; 22 rows whose definition the tool could not place = the 4 canary rows plus 18 rows of the tuple field `ShellUtilityPath.0`, resolved by hand), `rpv-a1-table.md`.

### 2.1 Owner crates (user crates carry their row counts; in the member lists `name L/T` = lib rows / test rows and `*` marks a member fixed by me, the rest by `rw2t`)

| owner crate | lib rows | test rows | members | users (rows, lib and test together) | fixed by |
| --- | ---: | ---: | ---: | --- | --- |
| `scene_state` | 247 | 356 | 62 | `scene_render` 146, `scene_ink` 137, `engine_attach_tests` 81, `scene_feeds` 74, `canvas2d_host` 68, `scene_canvas2d` 50, `scene_tables` 22, `scene_ink_tests` 19, `scene_text_vfs` 6 | rw2t |
| `scene_feeds` | 5 | 52 | 14 | `scene_ink_tests` 47, `scene_render` 10 | rw2t |
| `scene_tables` | 3 | 23 | 6 | `scene_render` 26 | rw2t |
| `engine_registry` | 9 | 13 | 7 | `engine_text` 18, `engine_attach_tests` 4 | rw2t |
| `runtime_mailbox` | 9 | 12 | 6 | `runtime_mailbox` 18, `runtime_mailbox_tests` 3 | rpv |
| `shell_view_paint` | 9 | 9 | 1 | `shell_view_chrome` 18 | rpv |
| `kernel_protocol` | 0 | 14 | 4 | `kernel_runtime` 14 | rpv |
| `engine_text` | 3 | 10 | 2 | `engine_attach_tests` 7, `frame_transaction` 6 | rw2t |
| `host_base` | 2 | 11 | 4 | `top` 11, `app_runtime` 2 | rpv |
| `scene_text_vfs` | 4 | 8 | 4 | `scene_render` 12 | rw2t |
| `frame_actions` | 3 | 8 | 3 | `runtime_mailbox` 6, `runtime_mailbox_tests` 5 | rpv |
| `kernel_retained` | 2 | 4 | 2 | `kernel_pool` 4, `kernel_runtime` 2 | rpv |
| `engine_attach` | 2 | 2 | 2 | `scene_render` 4 | rw2t |
| `ui_engine_dump` | 0 | 3 | 2 | `ui_engine_tests` 3 | rw2t |

**`scene_state`**
* `SceneSurfaceState`: `drag` 20/41, `pointer_was_down` 23/28, `viewport` 10/39, `ink_edit` 16/17, `ink_marquee_points` 10/15, `ink_overrides` 6/19, `map_marquee_active` 6/13, `map_marquee_points` 5/13, `ink_camera` 4/8, `paint_stroke_active` 4/8, `ink_gesture_open` 5/5, `last_click_ms` 4/6, `last_click_target` 4/6, `camera_dispatch_controller_id` 3/6, `canvas_click` 4/4, `map_last_hover_json` 3/5, `vfs_expanded_ids` 4/4, `touch_gesture` 3/3, `canvas_framing_revision` 2/2, `map_hover_owner` 2/2, `map_interaction_owner` 2/2, `hovered_control_id` 1/1, `last_pointer_pos` 1/1
* `SceneListTransferAuthority`: `active` 13/13, `next_generation` 3/3
* `GumballSlot`: `gesture` 9/9, `pointer` 5/5, `host_id` 4/4, `pointer_id` 4/4, `window_id` 4/4, `view` 3/3, `document_generation` 2/2, `cancellation_requested` 1/1, `controller_id` 1/1
* `(methods and items)`: `stage` 8/8, `take` 5/5, `enabled` 1/1, `has_capacity_for` 1/1
* `CanvasGestureSlots`: `slots` 7/7
* `CanvasStrokeJson`: `color` 4/4, `width` 4/4, `dash` 2/2, `cap` 1/1, `join` 1/1
* `SceneListTransferSession`: `source_rect` 4/4, `source` 3/3, `driver_drag` 1/1, `source_host_id` 1/1, `source_window_id` 1/1, `start_x` 1/1, `start_y` 1/1
* `CanvasImageFieldJson`: `src` 2/2, `height` 1/1, `width` 1/1
* `CanvasTextFieldJson`: `content` 1/1, `font_family` 1/1, `size` 1/1
* `PendingRasterCheckedOut`: `queue` 0/2, `surface` 0/1
* `SceneAccessibilityPresentation`: `accepted` 1/1, `candidate` 1/1, `queued_candidate` 1/1

**`scene_feeds`**
* `EventFeedLayout`: `inner` 4/22, `card_gap` 1/5, `card_padding` 0/1, `icon_size` 0/1, `time_font_size` 0/1
* `GraphTimelineLayout`: `inner` 0/6, `graph_width` 0/3, `label_track_width` 0/3, `row_height` 0/3, `selectable_width` 0/2, `avatar_overlap` 0/1, `avatar_size` 0/1, `graph_column_width` 0/1
* `EventFeedEntryJson`: `timestamp_ms` 0/2

**`scene_tables`**
* `TableMetrics`: `body` 2/9, `row_h` 1/4
* `BlockListPlan`: `targets` 0/7, `body_range` 0/1, `palette_target_step_id` 0/1
* `(methods and items)`: `table_editable_text_commit` 0/1

**`engine_registry`**
* `TextEditorDeliveryState`: `typing_end` 4/4, `typing` 3/3, `latest` 1/1, `publication` 1/1
* `EngineCanvasPacketReservation`: `destination` 0/2, `sequence` 0/1
* `EngineSurfaceSlot`: `exhausted` 0/1

**`runtime_mailbox`**
* `(methods and items)`: `owner_mut`* 6/6, `finish`* 1/1, `finish`* 1/1, `owner_mut`* 1/1
* `RuntimeRasterOperationAuthorityInner`: `next_operation`* 0/2, `exhausted`* 0/1

**`shell_view_paint`**
* `ShellUtilityPath`: `0`* 9/9

**`kernel_protocol`**
* `MountedProductReplayRecoverySlot`: `owner`* 0/8, `generation`* 0/4
* `MountedProductReplayRefusalSlot`: `generation`* 0/1, `owner`* 0/1

**`engine_text`**
* `TextEditorRetainedActionBatch`: `first` 2/8, `second` 1/2

**`host_base`**
* `RendererIoSlot`: `state`* 1/5
* `RendererIoHandle`: `slot`* 0/4
* `(methods and items)`: `try_take`* 1/1
* `RendererIoNode`: `rejected`* 0/1

**`scene_text_vfs`**
* `VfsMetrics`: `body` 2/3, `row_h` 1/2
* `VfsVisibleRow`: `row` 1/2, `level` 0/1

**`frame_actions`**
* `FrameActionEnvelope`: `descriptor`* 1/6, `cancelled`* 1/1, `receipt`* 1/1

**`kernel_retained`**
* `RetainedSurface`: `published`* 2/2
* `MountedReplayRecoverySlot`: `owner`* 0/2

**`engine_attach`**
* `RasterCompositeProgress`: `completed` 1/1, `done` 1/1

**`ui_engine_dump`**
* `DumpNode`: `path` 0/2
* `DumpAccessibility`: `window_ids` 0/1

Reading notes:
* 247 of the 298 lib rows are `SceneSurfaceState`, `GumballSlot`, `SceneListTransfer*`, `Canvas*Json` and `SceneAccessibilityPresentation` of **scene-state**, read by the five layers above it (scene-render 146, scene-ink 137, scene-feeds 74, canvas2d-host 68, scene-canvas2d 50). They are production reads across the crate edges, not test peeks.
* **Intra-crate rows:** `runtime_mailbox` `owner_mut` and `finish` (9 lib rows) are used by `runtime-mailbox` itself; the carve put the guard structs into `presentation-raster` and the users into `mailbox-impl`, two modules of one crate. Module privacy, not crate privacy.
* **Facade rows:** the only user outside the owner chain for `host_base` is the renderer facade (`top`): `native-entrypoint` read `RendererIoSlot.state` directly.
* Rows that need members hidden behind an error-typed receiver appear only after the first member is public (for example `SceneListTransferSession.pointer_id` behind `SceneListTransferAuthority.active`). That is why later runs list members the first run does not: a2 added `pointer_id`, `authority_generation`, `source_document_generation`, `promoted`, `TextEditorTypingRun.last_ms`, `BlockListTarget.*`.

### 2.2 Who fixed what (evidence)

Between `a1` and `a2` 263 of the 298 lib rows disappeared and 35 non-canary rows remained (`rpv-a2-attr.json`, `rpv-a2-members.txt`): 25 are first-run rows of `runtime-mailbox` 9, `shell-view-paint` 9, `frame-actions` 3, `host-base` 2, `kernel-retained` 2, and 10 are cascade rows (`SceneListTransferSession` 9, `TextEditorTypingRun.last_ms` 1). The 263 are in files that `rw2t` lists as touched (scene-state both files, engine-surface-registry, text-editor, table-and-paint2d, ink-canvas-render, event-feed-and-timeline, board2d-and-vfs, engine-surface-attach, pending-raster, introspection) and that carry the widening in `git diff` (`pub(crate)` lines changed to `pub`); I made none of those edits (my journal starts at 21:29:50 and has no entry for them). `RESULT-RENDERER-WP2-TESTS.md` 3.4 describes the same widening (`SceneSurfaceState` 23 fields, 98 line edits by `pos2widen.py`, 38 items named by the moved laws) and 6.3 hands me the members outside WP2 (`shell-view-paint`, `ui-engine`, `frame-latency`, `presentation-raster`, `kernel-retained-state`, `host-base`, Shell `icon-export`). Members: `rw2t` 99 (scene-state 62, scene-feeds 14, engine-registry 7, scene-tables 6, scene-text-vfs 4, engine-text 2, engine-attach 2, ui-engine-dump 2), `rpv` 20 of the 119 first-run members; the other members in section 3 are cascades of later rounds and the E0451 sites (section 6).

## 3. The fixes

Rule applied per member (AGENTS: prefer the narrowest correct visibility; no blanket widening, no shims):

| form | when | members |
|---|---|---|
| plain `pub` | the member is read or built across crates in production, or completes a record whose siblings are already `pub` (identity pairs, DTOs, ledger records) | section 3.1 |
| narrow owner method | a consumer reached into internals; the owner offers the question instead | `renderer_io_sessions_pending()`, `RetainedSurface::has_published()` |
| `pub(crate)` | consumer is a sibling module of the same crate | 4 guard methods |
| `#[cfg(any(test, feature = "testkit"))] #[doc(hidden)] pub` plus the private twin under `not(...)` | only test code in another crate touches it; the lib build keeps it private | section 3.3 |
| unchanged | nothing outside names it | all other private members |

### 3.1 Plain `pub` (cross-crate API)

| owner | member | users | why |
|---|---|---|---|
| host-base | `RendererIoHandle::try_take` | app-runtime lib | non-blocking poll of the handle, the `Future` impl calls it |
| host-base | `RendererIoHandle.slot` | renderer-io law | identity pair with the already `pub generation`; `renderer_io_with_node(slot, generation, ..)` and `renderer_io_generation_live` are `pub` and take both |
| host-base | `FrameLatencyObservation.{authority,stage}`, `FrameLatencyPhaseSummary.authority` | frame-tests-1 | `pub fn observe(FrameLatencyObservation)` cannot be called from outside otherwise; the summary is serialised JSON anyway |
| frame-actions | `FrameActionEnvelope` (struct) `.{descriptor,receipt,cancelled}` | runtime-mailbox lib, runtime-mailbox-tests | reached through the `pub slots` of `FrameActionOwners`; a private type behind a `pub` field was also a `private_interfaces` smell |
| shell-view-paint | `ShellUtilityPath.0` | shell-view-chrome lib (9 sites) | transparent newtype that only exists to give `[u16; 64]` a `Default` |
| engine-registry | `TextEditorTypingRun.last_ms`, `TextEditorDeliveryInFlight.{source,pending_members}`, `TextEditorDraftPublication.{source,restore}`, `TextEditorDeliveryPublication.{source,include_selection}` | engine-text lib (read and struct literals) | records whose other fields rw2t made `pub`; the literals are E0451 (section 6) |
| scene-state | `SceneListTransferSession.{pointer_id,authority_generation,source_document_generation,promoted}`, `GumballSlot.surface_id` | scene-render lib, canvas2d-host lib | completes records rw2t widened |
| scene-tables | `BlockListTarget` (struct) `.{rect,control_id,action,role}` (`paint` stays `pub(crate)`) | scene-render law | `BlockListPlan.targets` is `pub` |
| scene-feeds | `EventFeedEntryJson.{icon_id,title,detail,tone}`, `HistoryColumnJson.{checkpoint_id,labels,authors,parent_checkpoint_id,description,mutation_level,lane}` | scene-ink-tests laws (struct literals) | `*Json` DTOs, `timestamp_ms` and `id` were already `pub` |

### 3.2 Narrow methods and `pub(crate)`

* `host-base`: new `pub fn renderer_io_sessions_pending() -> bool` (docstring 🔭️); `native-entrypoint` (renderer facade) used `RENDERER_IO_SLOTS.iter().any(|slot| matches!(slot.state.load(..), LIVE | CHECKED_OUT))`, now `crate::renderer_io_sessions_pending()`; the unused `Ordering` import is gone; the name is added to the facade's explicit `pub(crate) use ..host_base::{..}` list in `T/🧊️renderer/🦀️.rs` (the new-rows diff of run `z1` against `a1` caught its absence).
* `kernel-retained`: new `pub fn RetainedSurface::has_published()` (docstring 🔎️); `kernel-pool` used `retained.published.is_none()` twice, now `!retained.has_published()`; the field stays private.
* `runtime-mailbox`: `owner_mut` and `finish` of `FrameMaintenanceExecutionGuard` and `FrameDeferredExecutionGuard` are `pub(crate)` (4 methods; no public API growth).

### 3.3 `testkit` hooks (24 members, all in lib code, all private in a normal build)

Pattern per field (tuple struct: two definitions): `#[cfg(any(test, feature = "testkit"))] #[doc(hidden)] pub f: T,` followed by `#[cfg(not(any(test, feature = "testkit")))] f: T,` (`pub(crate)` where it was `pub(crate)` before: `InkInteractionDocument.{spans,span_len}`).

| owner (feature added where missing) | members | user and enabling edge (checked by `rpv-tools/featchk.py`, section 5.4) |
|---|---|---|
| host-base (`testkit = []`) | `RendererIoSlot.state`, `RendererIoNode.rejected`, `FrameLatencyAuthority.{domain,generation}` | top crate law via a new dev-dependency `host-base[testkit]`; `frame-tests-1` dependency line got `features = ["testkit"]` |
| kernel-protocol | `MountedProductReplayRecoverySlot.{generation,owner}`, `MountedProductReplayRefusalSlot.{generation,owner}` | kernel-runtime unit law; kernel-runtime got a target dev-dependency `kernel-protocol[testkit]` |
| kernel-retained (`testkit = []`) | `MountedReplayRecoverySlot.owner`, `MountedReplayRecoveryOwner.generation` | kernel-runtime; dev-dependency `kernel-retained[testkit]`, and its own `testkit` feature forwards it |
| runtime-mailbox (had `testkit`) | `RuntimeRasterOperationAuthorityInner.{next_operation,exhausted}`, `RuntimeDispatchReturn.{mailbox,revision,cursor,interaction}` | runtime-mailbox-tests (already `[testkit]`) |
| scene-ink (had `testkit`) | `InkInteractionDocument.{spans,span_len}` | engine-attach-tests (already `[testkit]`) |
| shell-view-panels (had `testkit`) | `ShellChromeFrameCursor.{parked_opportunities,parked_reports,parked_phase}` (the law uses `..Default::default()`) | shell-tests-windows-1 (already `[testkit]`) |
| ui-engine (had `testkit`) | `RetainedInputReceiptReservation` (type and tuple field) | shell-tests-chrome-1 (already `[testkit]`) |
| shell-doc-open (`testkit = []`) | `IconExportBatch.{initializing,saving}` | shell-tests-docs-3 dependency line got `features = ["testkit"]` |

I edited no test source file (`rw2t`'s or any other); the only bin edits are two dependency lines (`frame-tests-1`, `shell-tests-docs-3`). Where a law could be switched to a constructor (`ShellChromeFrameCursor::at(phase)`, `RuntimeDispatchReturn::new`) that is cleaner than a hidden field; I kept the fields because those laws are not mine to edit (hand-off 8.3).

## 4. Public API growth per crate

Ungated public members added (new item or visibility raised to `pub`), gated hooks, `pub(crate)`:

| crate | public | `testkit` hooks | `pub(crate)` | feature / manifest |
|---|---:|---:|---:|---|
| host-base | 6 (1 new fn, 5 widened) | 4 | 0 | `testkit = []` |
| frame-actions | 4 | 0 | 0 | |
| shell-view-paint | 1 | 0 | 0 | |
| engine-registry | 7 | 0 | 0 | |
| scene-state | 5 | 0 | 0 | |
| scene-tables | 5 | 0 | 0 | |
| scene-feeds | 11 | 0 | 0 | |
| kernel-retained | 1 (new fn) | 2 | 0 | `testkit = []` |
| kernel-protocol | 0 | 4 | 0 | |
| runtime-mailbox | 0 | 6 | 4 | |
| scene-ink | 0 | 2 | 0 | |
| shell-view-panels | 0 | 3 | 0 | |
| ui-engine | 0 | 1 | 0 | |
| shell-doc-open | 0 | 2 | 0 | `testkit = []` |
| **total (rpv)** | **40** | **24** | **4** | 3 features; 3 new dev-dependency lines, 1 feature forward, 2 `features = ["testkit"]` requests |

`rw2t` widened 99 members in scene-state (62), scene-feeds (14), engine-registry (7), scene-tables (6), scene-text-vfs (4), engine-text (2), engine-attach (2), ui-engine-dump (2) and 38 laws-named items (its section 3.4). Manifests I edited (7): host-base (+feature), top crate (+dev-dependency host-base[testkit]), kernel-retained (+feature), kernel-runtime (+feature forward, +2 dev-dependency lines), shell-doc-open (+feature), shell-tests-docs-3, frame-tests-1 (one `features = ["testkit"]` each). `Cargo.lock` needs no edit: every new dev-dependency edge already exists in the root `Cargo.lock` (checked per edge); the `🧰️framework/Cargo.lock` does not list these crates at all (stale, as in earlier sweeps).

## 5. Verification

### 5.1 Oracle runs (all `POS_PREFIX=rpv`, `pos.py run <family> --cfgs <c> --label <L> --no-base`; absolute rows; the privacy column counts non-stub rows, canary rows excluded; "canary" = E0308, E0616 and E0624 of the probe module raised, so bodies were type-checked)

| run | family | tree | lib privacy | test privacy | canary |
|---|---|---|---:|---:|---|
| `a1` | ri2 (76) | 21:13, before everything | **298** | **525** | yes |
| `a2` | ri2 | 21:23, after rw2t's widening, before my first edit | 35 | 80 | yes |
| `a3` | ri2 | after round 1 (host-base, kernel-*, runtime-mailbox, frame-actions, view-paint, engine-registry, scene-state, block-list) | 0 | 2 (`MountedReplayRecoveryOwner.generation`) | yes |
| `a4` | ri2 | after round 2 | 0 | 0 | yes |
| `b1` | ri3bins (92) | same tree as a4 | 0 | 8 (frame-latency 3, ui-engine tuple field 3, `IconExportBatch` 2) | yes |
| `c1` | ri3bins | after round 3 (E0451 fixes, hooks) | 0 | 3 (`FrameLatencyAuthority.{domain,generation}`) | yes |
| `c2` | ri3bins, test | after round 4 | n/a | **0** | yes |
| `z1` | ri2 | after round 4 | **0** | **0** | yes |
| **`z2`** | ri2 | final tree (after the facade fix) | **0** | **0** | yes |

`z2` details: lib 20,890 errors (20,353 stub-layer noise), test 28,619 (27,936); resolve 84 / 99, type 452 / 582, other 1 / 2, trait-scope 0 / 0 (all unchanged classes of the sweep). Rows of `z2` that are not in `a1` (non-stub, keyed by code, crate and message): lib 2, test 4, all in code I did not touch: `engine-graph` `🕸️node-graph:561` and `ui-engine-tests` (the `infinite` free-function migration, rw2t 6.2) and `scene-state:66` (`isolated_cell` per-thread cell added by rw2t 3.3, an `if/else` type clash behind a stub type). Rows gone: 343 lib, 599 test.

### 5.2 Defects the run of the new rows found in my own work

* `z1` against `a1` (the diff of the non-stub rows keyed by code, crate and message) listed one new row: `cannot find function renderer_io_sessions_pending` in the renderer facade. The facade lists host-base names one by one; added, run `z2` shows the row gone.
* Order effect: the first oracle run on `ri3bins` found 8 test rows that `ri2` cannot see (the bins); two further rounds were needed because `FrameLatencyAuthority.{domain,generation}` sits behind `FrameLatencyPhaseSummary.authority`.

### 5.3 Seeded mutations (lib, `ri2`, run `m1`; sandbox only: `--mutate` makes one definition private again in the sandbox copy; the repo files were re-read after the run and are unchanged)

| # | mutation (sandbox) | expected | result (`rpv-ri2-m1-*`) |
|---|---|---|---|
| M1 | `pub fn try_take` to `fn try_take` (host-base) | E0624 in app-runtime | caught: `app-runtime:98` |
| M2 | `pub fn has_published` to `fn has_published` (kernel-retained) | E0624 in kernel-pool | caught: `kernel-pool:1802, 1813` |
| M3 | `pub descriptor` to `descriptor` (`FrameActionEnvelope`) | E0616 in runtime-mailbox | caught: `mailbox-impl:1443` |
| M4 | `pub(crate) fn owner_mut` to `fn owner_mut` (maintenance guard) | E0624 in the sibling module | caught: `mailbox-impl:1504` |
| M5 | `ShellUtilityPath(pub [u16; 64])` to `([u16; 64])` | E0616 in shell-view-chrome | caught: 9 rows `chrome-paint-and-navbar:543..580` |
| M6 | facade again reads `RENDERER_IO_SLOTS..state.load(..)` instead of the narrow method | E0616 (the testkit twin is off in the lib build) | caught: `native-entrypoint:15`, proves the gate |

`m1` totals: privacy 15 non-stub rows (the 15 above) plus the 2 canary rows; unmutated `z2` has 0. E0451 has its own controls (6.2).

### 5.4 testkit wiring (`rpv-tools/featchk.py`: Cargo resolver-2 feature propagation of `cargo test -p <user>`, dev-dependencies of the user count)

The oracle cannot check this: its `test` configuration turns `test` on for every crate. Result per hook pair (test build / lib build): top -> host-base OK / not enabled; frame-tests-1 -> host-base OK; kernel-runtime -> kernel-protocol OK / not enabled; kernel-runtime -> kernel-retained OK / not enabled; runtime-mailbox-tests -> runtime-mailbox OK; shell-tests-windows-1 -> shell-view-panels OK; shell-tests-chrome-1 -> ui-engine OK; shell-tests-docs-3 -> shell-doc-open OK; engine-attach-tests -> scene-ink OK. The "not enabled" lib lines are the negative control (a lib build must not see the hooks).

### 5.5 Static gates

| check | command | result |
|---|---|---|
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout <file>` on the 19 edited `.rs` files | 0 failures |
| manifests | `bun -e 'Bun.TOML.parse(...)'` on the 7 edited `Cargo.toml` | all parse |
| paths | longest edited repo-relative path | 228 bytes (limit 240) |
| new files | none in the repo (tools only in `F/rpv-tools/`) | n/a |
| losslessness | every edit asserted its anchor count; `git diff --numstat` of the 26 files is proportional (cumulative with peers' edits in shared files) | ok |
| debug logs | none added | ok |

## 6. The E0451 blind spot (finding, tool, results)

**Finding.** Micro tests (`scratchpad/rpv-work/micro/m1.rs..m3.rs`, `rustc +nightly-2026-07-20 --crate-type lib --emit=metadata -o /dev/null --error-format=short`): `a::S { f: 1 }` and `let a::S { f } = ..` with a private `f` raise E0451 only when the crate has no other error (m2). One E0308 (m1) or one E0616/E0624 (m3) elsewhere and the privacy pass never runs (it sits behind the abort after type-check). The union oracle always has thousands of stub errors, so **E0451 was never reported**, in this slice. A private tuple-struct constructor (E0603) and the field reads are fine (m1 reports E0603). No E0451 row appears in any of my 11 oracle runs, and the okwp8 reports of the previous sweep (`pos2-okwp8-*-report.txt`) contain none either.

**Tool.** `rpv-tools/lit.py <family.json>`: parses every brace-struct and its field visibilities (attributes and `cfg(.. testkit ..)` honoured), builds the module tree from `mod x;`, `#[path]` and `include!`, finds literals and patterns `Name { a: .., b, ..base }` outside `struct`/`impl`/`->` heads, and flags those whose struct (unique by name among candidates, same-crate candidates first) has a field not visible at the use site: not `pub` for another crate, private or `pub(super)` outside the defining module (same file by inline-module nesting, descendants of the defining file count as inside), and `..base` over invisible fields. Read-only.

**Results.** First scan (`ri3bins`, cross-crate only): 17 sites in 10 structs: lib `GumballSlot.surface_id` (canvas2d-host), `TextEditorDeliveryInFlight`, `TextEditorDraftPublication`, `TextEditorDeliveryPublication` (engine-text); test `FrameLatencyObservation` x3, `EventFeedEntryJson` x6, `HistoryColumnJson`, `InkInteractionDocument`, `RuntimeDispatchReturn`, `ShellChromeFrameCursor` (`..Default::default()` over the private `parked_*`). Fixed as in 3.1 and 3.3. Final scan with the module-aware version (intra-crate included): **0 sites** on `ri3bins` (815 brace structs, 825 definitions, 745 files in the module tree) and on `ri2`. Controls: the 17 before and 0 after on the same sources; `LIT_SELFTEST=1` injects a probe (private `f` used from two sibling inline modules, `pub(super)` used from a descendant, private used from a child) and reports exactly the two illegal literals and not the legal ones.

## 7. Real-check commands (not run; the compile ingress refuses compile admission this wave)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-host-base -p semio-framework-os-renderer-wgpu-kernel-retained -p semio-framework-os-renderer-wgpu-kernel-protocol -p semio-framework-os-renderer-wgpu-kernel-pool -p semio-framework-os-renderer-wgpu-kernel-runtime -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-frame-actions -p semio-framework-os-renderer-wgpu-shell-view-paint -p semio-framework-os-renderer-wgpu-shell-view-chrome -p semio-framework-os-renderer-wgpu-shell-view-panels -p semio-framework-os-renderer-wgpu-shell-doc-open -p semio-framework-os-renderer-wgpu-ui-engine -p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-renderer-wgpu-engine-text -p semio-framework-os-renderer-wgpu-scene-state -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-scene-render -p semio-framework-os-renderer-wgpu-canvas2d-host -p semio-framework-os-renderer-wgpu-app-runtime -p semio-framework-os-renderer-wgpu --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```
1. The same with `--tests` and the test hosts added: `-p …-runtime-mailbox-tests -p …-frame-tests-1 -p …-shell-tests-docs-3 -p …-shell-tests-chrome-1 -p …-shell-tests-windows-1 -p …-engine-attach-tests -p …-scene-ink-tests -p …-ui-engine-tests` (this is where the dev-dependency `testkit` lines are exercised).
2. The negative build, the hooks off: the first command's crates are already checked without `--features`; it must pass without any `testkit` (the `not(any(test, feature = "testkit"))` twins).
3. The hooks on, per owner: `… check -p semio-framework-os-renderer-wgpu-host-base --features semio-framework-os-renderer-wgpu-host-base/testkit` and the same for `kernel-protocol`, `kernel-retained`, `runtime-mailbox`, `scene-ink`, `shell-view-panels`, `ui-engine`, `shell-doc-open`, `engine-board`.
4. `--target wasm32-unknown-unknown --lib` for `shell-view-paint`, `shell-view-panels`, `ui-engine`, `scene-state`, `scene-feeds`, `scene-tables`, `engine-registry`, `kernel-retained` (the touched items that are not `cfg(not(wasm32))`); host-base's new function and the I/O slots are native-only.
Expected first faults: the `infinite` free-function family (rw2t 6.2) in engine-board/engine-graph/engine-attach, then `unused import` or `dead_code` warnings for fields that are now `pub` in private structs.

## 8. Not done, residual risks, hand-offs

1. **Not compiled.** The oracle checks name resolution and the privacy of paths, fields and methods; it does not check borrowck, trait coherence, the `cfg` twins' agreement on every target, or feature unification beyond `featchk.py`.
2. **Blind spots of the oracle** besides E0451: privacy findings hidden behind an error-typed receiver appear one round late (4 rounds here); fields behind stub-typed values are never seen (20,353 stub errors in `z2` lib). `lit.py` covers literals and patterns only for structs with a unique definition among candidates and ignores macro-generated structs and enum struct variants (their fields are always `pub`).
3. **Test-law changes that would be cleaner than hooks** (laws are not mine): `ShellChromeFrameCursor::at(phase)` for `shell-tests-windows-1` (`wgpu-window-pane-chrome:302`), `RuntimeDispatchReturn::new` for `runtime-mailbox-tests` (`async-boundary:2663`), a `FrameLatencyAuthority` accessor pair for `frame-tests-1`, a `renderer_io_slot_state(index)` helper for the renderer-io law. Each replaces 2 to 4 hidden fields.
4. **rw2t's widening is blanket in places** (its section 7.4): `SceneSurfaceState` shows 23 public fields; `ink_marquee_points` and the `map_*` fields are read for assertions only by the standalone law and could become a `testkit` accessor.
5. **The oracle tool** should adopt `lit.py` (an E0451 pass) and `featchk.py` (testkit wiring); both are read-only.
6. **Facade lists:** `T/🧊️renderer/🦀️.rs` re-exports host-base names explicitly; a new `pub` function in a lower crate is invisible to the facade until listed (one case found).
7. The 4 `pub(crate)` guard methods leave `FrameMaintenanceExecutionGuard` and `FrameDeferredExecutionGuard` private types returned from `pub fn begin`; that is a `private_interfaces` warning, not an error, and unchanged.
8. Concurrent peers: `rw2t` finished during my run; `rtb` and `ri3` bins were stable. Rows that appear after a later peer edit are not covered by `z2`.

## 9. Files

Created: `F/RESULT-RENDERER-PRIVACY.md`; `F/rpv-tools/` (`attr.py` attribution, `q.py` row query, `table.py` owner table, `ed.py` anchored editor, `featchk.py` testkit wiring, `lit.py` E0451 scan); logs and results `F/🗑️generated/rpv-*` (oracle runs `a1..a4`, `b1`, `c1`, `c2`, `z1`, `z2`, `m1`; `rpv-*-attr.json`, `rpv-a1-table.md`, `rpv-edits.log`, `rpv-*-e0451.json`); scratch `scratchpad/rpv-work/` (sandboxes, micro tests).
Edited (repo, 26 files): host-base `📥️lifted/🔁️asset-decode-jobs` and `⏱️frame-latency`, `⌨️native-entrypoint`, the facade `🧊️renderer/🦀️.rs`, `kernel-retained-state`, `kernel-pool`, `kernel-replay-client`, `presentation-raster`, `frame-actions`, Shell `view-paint/chrome-frame-and-main-window`, `view-panels/chrome-frame-and-main-window`, `doc-open/icon-export`, `Interpreter/retained-ui-engine`, `EngineCanvas/engine-surface-registry`, `Scenes/scene-state`, `Scenes/block-list-and-diff`, `Scenes/event-feed-and-timeline`, `Scenes/ink-canvas-model-state`, `T/🎭️scene-state`, and the `Cargo.toml` of host-base, the top crate, kernel-retained, kernel-runtime, shell-doc-open, shell-tests-docs-3, frame-tests-1. Deleted: none.
