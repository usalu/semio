# Surface capture lease panic (2026-11-27)

## Root cause

`Event::SurfaceVisible` (and `SurfaceHidden`) reach `SurfaceContexts::insert`/`remove` through `plugin_mount_surface`/`plugin_hide_surface`. Those assert `cached_capture.is_none()`. A cached view read (`OriginalSurfaceViewRead`, `🪟️surfaces/read.rs`) sets `cached_capture` in phase 0 and releases it only at its close, across several turns. A mount arriving in between aborted the guest ("original surface capture holds the registry mutation lease", wasm `unreachable`). Reads already answered "try later" (`Ok(false)`); the mutating events had no such path.

## Fix

- `SurfaceContexts::capture_held()` (`⚛️reactor/🪟️surfaces/🦀️.rs`).
- `plugin_surface_capture_held(runtime, instance)` and the pure decision `surface_mutation_waits(event, held)` (`⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs`).
- `select_turn_event` (`⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs`) selects no event while it waits. The event stays in the `events` slot (the existing yield path), `events_pending` keeps the turn reporting more work, and it replays in order once the capture is released.

## Laws (both green)

- `mutation_wait_tests::surface_mutation_events_wait_only_while_their_own_instance_holds_a_capture` (`🖥️surface-render/mutation-wait-tests.rs`, crate semio-plugin-runtime-ops-surface).
- `surface_context::tests::an_open_capture_is_reported_held_until_it_is_released_and_then_the_registry_mutates_again` (`🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs`, crate semio-plugin-plugin-program).

Caveat: a heavy rig law (open a real `OriginalSurfaceViewRead`, assert held, release) was attempted in `ops/🧪️tests/original-view-read.rs`; that test file hangs/fails at baseline (the existing tests there fail at admit with an unwrap on None, and `release_read` loops forever), so it was removed. The "failing first" step was a compile failure (the symbols did not exist); the pre-fix panic is the one in the requester console.

Not covered: `plugin_render_with_document` and `plugin_refresh_ui` also call `update_view`, which asserts the same lease; they are host-driven calls, not events, and were left alone.

## Status of the rebuild and probe (not reached)

`T/requester-rebuild-install.sh` is still running in the background (log `T/🗑️generated/requester-install-run.log`, builds `requester-build-1.log`, `requester-build-2.log`). Attempt 1 failed because another agent's in-flight `🧷️companions/🥥️tests-15/📦️packages/🦀️rust/Cargo.toml` was briefly malformed ("invalid type: map, expected a sequence"; fixed on disk by the time of attempt 2). Attempt 2 has sat for over 2000 s in "Reading the project graph slice ... waiting for host capacity" (the known graph-slice stall) with the host nearly idle (load 2). Once the script prints `requester ok` and the install line, run `T/activate-and-probe.sh react-journey-lease 1,4` style (rows 1 and 4), then the full 19-row run. No probe result exists yet; meshes > 0 is unverified.
