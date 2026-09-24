# WP-C8 — Checkpoint Job, Peer Panel Refresh, Collab 10/10, Zero-Touch

Session 10 slice C8. Ports: hubs 7880–7889, serves 6380–6389. Captures `wp-c8/generated/`.

## Status

| Item | Status |
|---|---|
| 1. `commitCheckpoint` guest panic → async job across turns + native law | DONE natively (laws green); guest rebuild requested from W1 (gis, draw first) |
| 2. Peer inspector panel refresh after remote edit | DONE natively (law green); live proof pending guest rebuild |
| 3. Scenario 10/10 + STEP 14 peer cursors | BLOCKED on W1 catalog A (hub-bound guests are served from the hub catalog; 7820's is stale). Partial run c8f: 1a–1e, 5, 7, 8 PASS |
| 4. Zero-touch clean-state run, timed | NOT RUN: needs catalog A publish (cold cache after the 12:43 sweep) |
| 5. `browser-document-open-check` last stage | DONE: `open-plan-server-check` 8/8 laws (the stage C7 could not finish); a second full-chain run hung once in one law (below) |

## Log

### Item 1 — reserved commits are a queued job driven across turns

Root cause (measured): `JobCompleted` for a framework-reserved route ran the whole commit as ONE future through
`resolve_ready` (`⚛️reactor/🔄️turn/🦀️.rs` → `plugin_complete_reserved_spawned_job` → a second `resolve_ready` inside
`with_instances_mut`). The commit routes contain real suspension points: `commit_children_for_checkpoint` and
`cascade_checkout_to_children` `yield_once` per composed child, and the revert walk yields per undo. gis2d is composed
(`SemioMembers`, canonical child handles), so its first checkpoint suspended → panic → wasm `unreachable`. Probe
`generated/probe-3.txt`: the uncomposed contract app finishes with 0 pendings, which is why no native law caught it.

Fix (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `…/⚛️reactor/🔄️turn/🦀️.rs`):
- `JobCompleted` now only hands the job to the app: `PluginApp::admit_reserved_spawned_job` (sync) moves it into a FIFO
  `reserved_commits` queue (`FrameworkReservedCommit { stage, applied, total, permit }`).
- `advance_typed_operation_publication` (the reactor's per-turn typed-operation continuation) runs ONE unit per call through
  `step_framework_reserved_commit`: Validate+route, one unit per checkpoint child, one per checkout pin, one per revert
  undo. Each unit re-reads the operation's cancellation lease; `reserved_commit_progress()` answers `applied/total`.
  All `yield_once` points in these routes are gone, so every unit is ready by construction.
- The finished result is framed by `advance_typed_operation_output_with_leftover` (`Invocation{in_reply_to:0}` or
  `Error`) before that operation's `OperationCompleted`; the JobCompleted arm routes via `route_exchange_output` (typed
  result pages were dropped there before).
- Close ladder, `Drop`, close conjunction and `has_{pending,runnable}_typed_operations` own the queue and outcome.
- Test helpers `settle_framework_reserved_admission` / `drive_framework_reserved_spawn_like_host` drive the same units
  (`settle_framework_reserved_commit`).

Laws (`🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`):
`a_composed_checkpoint_commit_is_driven_one_ready_unit_per_turn_with_progress` (every unit polled once with a no-op waker
must be Ready; 2 children → 3 units, progress (1,3),(2,3), pins recorded) and
`a_composed_checkpoint_commit_cancels_between_units` (cancel after unit 1 → `interactive-job.cancelled`, no parent checkpoint).

| Check | Result | Capture |
|---|---|---|
| `cargo test -p semio-framework-plugin --lib -- checkpoint reserved revert history undo redo` | 76/76 | `test-3.txt` |
| `cargo test -p semio-framework-plugin --lib` | 833/835; the 2 failures are tool-run timing/capacity laws that pass serially (2/2), unrelated | `test-lib-1.txt` |

### Item 2 — a remote merge re-projects every view of the document

Root cause: the browser actor delivers hub frames to the guest as `Event::Message{source: backbone}`
(`🏪️store/👷️worker/🟦️.ts` invokePoll). The reactor arm dirtied `dirty_background_surfaces`, and
`SurfaceContexts::background_surfaces()` answers the mounted WINDOWS only (panels only when no window exists) — so the
canvas re-rendered while the inspector panel and history sections kept their pre-merge trees.

Fix: `DocumentBackboneTurnOutputV1.document_changed` (= the delivery produced merge reports);
`SurfaceContexts::document_surfaces()` (every bound window, panel and section); the backbone arm dirties
`dirty_document_surfaces` when the document changed, background surfaces otherwise (acks, presence). Job progress/completion
keep the window-only background set.

Law: `document_surfaces_name_every_window_and_every_panel` (`⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle`),
green with the item-1 laws (`test-4.txt`, 51/51 of the filtered set).

### Log
- 04:38 hub mutex: `open-plan-server-check` queued (detached pid 68516, capture `generated/open-plan-server-check.txt`), behind h4.
- 04:45 W1 accepted the rebuild: gis → draw describe+materialize-dev, then `activate s react dev`, then catalog
  `stdio,gis,note,draw,writer,puzzle` → `.🧬semio/🌐hub/w1-catalog-a` (ETA gis/draw ~05:20, catalog ~06:20).

### Item 5 — `browser-document-open-check`

- 04:38–05:10 `open-plan-server-check` (the stage C7's run never reached) under the hub mutex: oracle + production parity
  passed and **8/8** exact server laws passed, exit 0 (`generated/open-plan-server-check.summary.txt`).
- 05:10 full chain rerun (`generated/browser-document-open-check.txt`): oracle, Chromium runtime, OS test-quick 5/5,
  plan oracle, parity and 2 server laws passed; then the os-hub test binary running
  `document_open_plan_admin_revocation_invalidates_session_and_share_bindings` sat at 0 % CPU for 70 min (the same
  law took 0.07 s in the 04:38 run). `sample` could not attach. I killed my tree at 06:31 so H4 could take the hub mutex.
  This is an intermittent hang in that law; its root cause is not diagnosed.
- 06:25 C8 serve on 6380 (s lane, activation 06:23 that already contains items 1+2): pid 63655, vite child (see `serve-6380-s.txt`).
- 07:05 BLOCKED on gis: W1's gis describe (07:01) and native `cargo test -p semio-s-plugin-gis --lib` fail with
  `artifact-definition.runtime-capability: no declared composer capability owns the runtime claims`
  (`generated/gis-lib-1.txt`). W1 traced it to a peer's 06:17–06:19 gis io edit: `compose_export_txt` ComposerEntry was added to
  gismap `🚪️io/🦀️.rs:206` and gisterrain `:184` with no declared `composer.txt` capability, and the svg/pdf/png/las/ply/dwg
  deserializers were dropped. The owner is unknown. It's routed to the coordinator (not a one-line fix, so C8 leaves it alone per rule 11).
  No gis guest, gis2d activation or catalog A (which includes gis) can be built until it's fixed.
- 07:2x t3 (owner of the gis io edit) removed the undeclared txt composers; gis assembles again (t3's law 1/1). W1 re-describes gis.
- 08:40 **Item 1 live, local lane (gis2d serve 6381, W1 guest 08:29):** Add Feature → Commit Checkpoint → Add Feature → Add Feature,
  0 faults, history `Add Feature / Commit Checkpoint / Add Feature`, `Check In (1)` after the checkpoint (`generated/c8e-run.txt`).
  Before the fix the same click trapped (`wp-c7/generated/c7ag-*`).
- 08:45 Local inspector refreshes after a local edit: Positions 152→153 within 2 s (`c8i-run.txt`).
- 08:50 The hub-bound lane (c8f/c8g on hub 7820) still panics with `resolve_ready` on commitCheckpoint, because the browser actor loads
  its guest from the HUB's trusted catalog (`/execution-target/component`), and 7820's c7-boot catalog carries the OLD gis.
  So the collab proof needs catalog A (W1) on the hub. c8f's step 2/3 "PASS" came from the old ledger witness counting my own
  panel-tab rows. I replaced it with an inspector-only witness (c8g: A's own inspector also stayed at 152 on the stale guest).
- 11:45 W1's catalog A publish was killed at 11:45 (rc=137, wall 6649 s, `wp-w1/generated/catalog-a.txt`). My wait loops were killed at the same moment, so it looks like an external sweep. Waiting for W1's retry.

- 13:32 Rule 13: `cargo check -p semio-framework-plugin` on the current tree: EXIT 0 with 85 warnings (`check-plugin-rule13.txt`).
- 13:32 Catalog A is still not published (W1 holds the wasm mutex since 13:10, cold cache). My hub/serves died in the 11:45 sweep, and I hold
  no processes or locks now. **Resume recipe** once `.🧬semio/🌐hub/w1-catalog-a/trusted-catalog/current.json` exists:
  1. hub: `bun wp-c7/hub-hold.ts 7880 <APFS clone of w1-catalog-a> .tmp-ticket/wp-h5/bin/os-hub` (nohup), credentials per `wp-c7`;
     serve: `zsh wp-c8/serve.sh gis2d 6381 http://127.0.0.1:7880`;
  2. `C3_TAG=c8h bun wp-c8/c8-collab-scenario.mjs http://127.0.0.1:6381 127.0.0.1:7880 <space> <doc>`. Steps 2/3 now pass only on the
     peer's INSPECTOR changing (witness fixed), and step 4 (undo) depends on the checkpoint fix;
  3. `S_COLLAB_OUT=.tmp-ticket/wp-c8/generated S_COLLAB_HUB_BINARY=.tmp-ticket/wp-h5/bin/os-hub zsh 📜️fleet-mutex.sh wasm c8 -- bun nx run
     @semio-tech/framework-os-dev:collab-e2e` (14 steps incl. STEP 14 writer/draw/puzzle3d cursors);
  4. zero-touch: `rm -rf` a fresh `OS_HUB_DATA`, then time `dev s` (`ensureTrustedCatalog` → `os-hub:trusted-catalog-bootstrap`) under the wasm mutex.

## Coordinator note (17:5x)
- Zero-touch hazard: `🌎️hub/📦️packages/🦀️rust/dist/build-dev/os-hub` (mtime 2026-09-23 19:37) is a pre-H2 binary that panics at boot (overlapping `GET /scopes/{scope}/document/ws`). When proving zero-touch `dev s`/▶️start, verify ensureDevLocalHub rebuilds or refuses a stale dist hub by content/source freshness, and never boots it.
