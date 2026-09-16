# GIS 2D End-to-End — Verification Log

## Build / runtime fixes (this continuation)

1. **`genesis_gis_map_child_pack`** — derives `drawing` + `value` child packs from parent snapshot (`✏️s/.../🗺️gismap/🦀️.rs`).
2. **`Gis2dPlayApp` / `GisMapViewer`** — `child_restore_projection` + `genesis_child_pack`.
3. **GIS plugin** — `editor_with_members` / `viewer_with_members` over `SemioMembers`; `semio-s-artifact-stdio-semio` dep on `semio-s-plugin-gis`.
4. **Test harness** — `SemioMembers`, `Gis2dTestApp`, `render_tiled_map_scene` (binary tiled-map projection).
5. **View command tests** — assert on decoded `TiledMapScene` fields, not escaped JSON substrings.
6. **Framework** — typed retained publication uses `begin_outbound_apply_batch` + `flush_published_apply_batch` when a document backbone is attached.
7. **Example undo test** — `settle_framework_reserved_admission` + maintenance pump after `undo`.
8. **Retained mutation ids** — `gis2d-retained-{operation}-{sequence}` so hot-backbone peers do not collide on `gis2d-retained-1#0`.
9. **Framework** — `complete_store_replacement_genesis` on live envelope store replacement (`AwaitingMembers`) mints `genesis_child_pack` members like archive load.
10. **Convergence harness** — `attach_hot_backbone`, repeated `commitCheckpoint` + `tick_backbone`; `Gis2dTestApp::drop` skips teardown while panicking.
11. **Envelope drive** — reactor turn + `maintenance_step` each loop (`drive_gis_map_live_load`); treat `Progress` like `Pending` while decode is `Ready` and replacement is starting.

## Framework envelope ladder (2026-09-16, latest)

- **`reclaim_returned_ticket_now`** on field-decoder registry + inline reclaim in `release_step` / `close_step` (`env-20` ticket-reclaim gate).
- **`ActiveArtifactEnvelopeDecode`** — publish `Ready` as soon as the worker completes with a completion ticket (load API no longer blocked on worker-session teardown); `has_runnable_work` still pumps session/outcome close; `has_runnable_typed_operations` only counts decode while poll is `Pending`.
- **`try_adopt_completed`** — default missing `dialect` to `A::DIALECT` so schema-first ingress wires without a dialect field can start store replacement.
- **Composition envelope law fixture** — valid schema-first wire + `SlowFreshEnvelopeFieldDecoder` wrapping the real fresh decoder (bare slow decoder + `vcs:{}` was faulting with `schema-json.missing-required-field`).
- **`try_begin_artifact_store_replacement`** — pre-drain via `force_worker_session_terminal` + `drive_envelope_decode_jobs` + retirement pump before publish; `decode_worker_owners_are_terminal` gate; `terminal_state` close ladder in `force_worker_session_terminal`.
- **`advance_artifact_envelope_load`** — returns `Progress` until decode worker owners are terminal (no fault on partial drain).
- **`has_runnable_artifact_envelope_decode_worker_step`** — matches `has_runnable_work` so reactor turns pump the post-`Ready` worker close ladder.
- **`artifact_envelope_decode_worker_owners_are_terminal`** — public witness for tests and load drain.
- **`ActiveArtifactEnvelopeDecode::drive`** — `Complete` no longer skips session drop; `Ready` parks only when `session.is_none()`; drops terminal-empty session inline.
- **`try_begin_artifact_store_replacement`** (post-publish) — reuses `drain_ready_envelope_decode_worker_owners` (reactor drive + decode jobs + retirement pump) instead of a narrow `force_worker` loop.
- **Composition `pump_envelope_decode_worker_close`** — calls `drain_ready_envelope_decode_worker_owners` (not maintenance-only rotation), so `one_reactor_turn_pumps_*` finishes in bounded time instead of spinning on stage 11/24.

## Green (verified)

| Gate | Result |
|------|--------|
| `@semio-tech/framework-os-dev:activate-gis2d-wgpu-dev` | pass (2026-09-16) |
| `semio-s-plugin-gis` surface assembly test | pass |
| `set_active_example_empty_then_reuse_round_trips_document` | pass |
| `renders_gis_map_scene` | pass |
| `semio-s-artifact-gis-gismap` lib | **239 / 240** pass (2026-09-16) |
| `two_instances_converge_on_disjoint_route_edits` | pass |

## Still open

| Gate | Symptom |
|------|---------|
| `gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` | Re-run after worker-owner drain fix (blocked: **Xcode license** — `sudo xcodebuild -license`; stale test binary shows stack overflow from failed relink) |
| `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` | `drain_envelope_decode` now stops at decode `Ready` without calling `advance` (avoids accidental store replacement); cancel maintenance pumps worker retirements |
| `drawing_live_envelope_submit_*` | Not re-run after latest framework changes |
| `@semio-tech/gis-plugin:component-cold-map-patch-check` (source/AJV only) | pass (2026-09-16, no native link) |
| `component_cold_map_patch` (Wasmtime / native-check) | Blocked: Xcode license |
| Full `semio-s-artifact-gis-gismap` 240/240 | Blocked on live envelope test above |

**2026-09-16 (agent):** `xcrun --sdk macosx --show-sdk-path` still exits **69** (license). Background `one_reactor_turn_pumps_the_envelope_decode_worker` run **aborted** after ~44 min with no pass/fail — re-run via `verify-gis2d-envelope-gates` after license (or devcontainer).

**2026-09-16 (agent, CLT):** With `DEVELOPER_DIR=/Library/Developer/CommandLineTools` + `SDKROOT=…/MacOSX.sdk`, native link works without accepting Xcode.app license. `📜️verify-gis2d-envelope-gates.sh` now auto-falls back to CLT. `one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll` **passed** (~10 min) after stronger `drain_ready_envelope_decode_worker_owners` pumping. Remaining gates: `advance_artifact_envelope_load_*`, GIS live envelope, full gismap lib, `activate-gis2d-wgpu-dev`.

**2026-09-16 (agent):** Full `📜️verify-gis2d-envelope-gates.sh` run **aborted** (SIGTERM) after ~26 min still on the first framework test; log tee failed (`🗑️generated/` missing). Composition harness outer drain retries trimmed (production `drain_ready` unchanged) before re-run.

**2026-09-16 (agent):** `advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault` run **aborted** (SIGTERM) after ~25 min — same slow harness (pre-fix). `drain_envelope_decode` now pumps worker close once after `Ready` (not per reactor turn). Verify script reordered: fast gates first, `one_reactor_turn_*` last.

## Dev entry

Launch **`gis2d-wgpu`** → `bun nx run workspace:dev -- gis 2d` (port 6140). Stack: GIS plugin → `tiled-map` → `TiledMapHost`.
