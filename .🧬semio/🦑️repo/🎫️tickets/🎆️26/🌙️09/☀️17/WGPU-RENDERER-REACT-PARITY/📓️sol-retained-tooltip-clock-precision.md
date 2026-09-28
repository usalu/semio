# Retained Tooltip and Window Clock Precision

## Scope

This packet resolves the three clock and tooltip failures reported by the seventh focused WGPU UI gate. It preserves the accepted retained interaction model and uses the existing neutral tooltip and number-stepper fixtures.

## Evidence

The neutral tooltip fixture sets `clockOriginSeconds` to `31536000.123456` and `dwellMs` to `400`. The actual React `ChromeControlHint` oracle advances fake timers by `399` ms and then `1` ms, and production schedules the reveal with the integer constant `CHROME_CONTROL_TOOLTIP_DELAY_MS = 400`.

The WGPU event router stores monotonic timestamps as `f64`, but the shared dwell constant was `f32`. Converting the `f32` value `0.4` to `f64` yields approximately `0.40000000596`. At the fixture's large monotonic origin, advancing by the fixture's exact `400 / 1000` seconds remained before the widened deadline. Both retained tooltip laws consequently observed `TooltipStep::Idle` instead of `Reveal`.

The stepper clock failure had a separate stale expectation. Pointer-down accepts the editable stepper caret and arms both its hold repeat and tooltip. After tooltip reveal at origin plus `0.4` seconds, the accepted caret blink at origin plus `0.5` seconds is the next deadline; the first repeat follows at origin plus `0.6` seconds. The old law expected the repeat immediately and sampled its purported no-op exactly at the due caret tick.

## Changes

- `TOOLTIP_DWELL_SECONDS` is now an `f64`, matching the router clock and the neutral/React `400` ms contract without f32 widening.
- The retained window-clock law proves a sealed no-op between the tooltip and caret deadlines, a single invalidation at the accepted caret deadline, and a second single invalidation at the first repeat deadline.
- No runtime deadline order or retained-state ownership changed.

## Verification

Fail-first receipt from the seventh UI gate (`🗑️generated/astra-runtime/gate34/ui-7.log`):

- `accepted_control_tooltip_reveals_after_dwell_paints_in_overlay_and_dismisses_immediately` failed because the exact fixture dwell still returned `Idle`.
- `tooltip_keeps_accepted_text_across_candidate_discard_and_retires_after_replacement` failed at the same exact-dwell `Reveal` assertion.
- `window_clock_keeps_a_noop_candidate_sealed_and_invalidates_it_once_when_hold_repeat_changes_state` expected `31536000.723456003`, while the accepted clock correctly returned the earlier caret deadline `31536000.623456`.

Passing actual React oracle:

- `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long bun nx run @semio-tech/ui-react:test --skip-nx-cache --excludeTaskDependencies -- '../../🧱️elements/💡️ChromeControlHint/🧪️tests/🧩️component/🟦️.tsx'`

Result: 1 file passed and 2 tests passed; Vitest 38.34 seconds and Nx 43.3 seconds.

Running focused WGPU law:

- `CARGO_BUILD_JOBS=2 NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-clock-tooltip/tmp' bun nx run @semio-tech/ui-rs:test-wgpu-engine --skip-nx-cache --excludeTaskDependencies -- accepted_control_tooltip_reveals_after_dwell_paints_in_overlay_and_dismisses_immediately`

The second retained tooltip law and corrected window-clock law will run through the same focused Nx target after the shared native build completes.

## Limits

The full UI, native renderer, WASM, and paired-shell gates remain owned by the root agent. This packet does not claim those gates.

## Accepted Document Replacement Follow-up

The tenth UI gate reached the later lifecycle assertions after the clock repair and exposed two distinct issues:

- The reveal law reconciled the presented interaction rebase and then attempted to paint immediately. Reconciliation correctly queued a fresh layout, so the frame stayed pending on `DIRTY_LAYOUT`. The law now drives that required layout before the production paint ladder.
- The replacement law accepted a new document that reused the same window accessibility generation and document node id. The candidate router therefore rebased the old revealed target, and `acknowledge_presented_input` preserved the tooltip across the accepted document revision change. It now clears the retained tooltip exactly when the acknowledged `presented_revision` changes. Same-revision presentation and interaction candidates continue to preserve accepted tooltip text and geometry.

The source checkpoint changes only the acknowledgement revision boundary and the focused law's scheduler sequence; it adds no retained slot or compatibility path. `git diff --check` is clean. The focused command

`CARGO_BUILD_JOBS=2 NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-tooltip-lifecycle/tmp' bun nx run @semio-tech/ui-rs:test-wgpu-engine --excludeTaskDependencies -- tooltip`

first ran all five matching laws: four passed, while the reveal law exposed that the newly required layout overwrote its manually placed candidate anchor. The law now reapplies the candidate anchor after layout, preserving its actual top-gap contract.

The exact focused rerun

`CARGO_BUILD_JOBS=2 NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR='<ticket>/🗑️generated/sol-tooltip-lifecycle/tmp' bun nx run @semio-tech/ui-rs:test-wgpu-engine --excludeTaskDependencies -- accepted_control_tooltip_reveals_after_dwell_paints_in_overlay_and_dismisses_immediately`

passed 1/1 with 714 skipped; nextest took 0.042 seconds and Nx took 2 minutes 32 seconds. The document-replacement law is running separately through the same target and is not yet claimed.
