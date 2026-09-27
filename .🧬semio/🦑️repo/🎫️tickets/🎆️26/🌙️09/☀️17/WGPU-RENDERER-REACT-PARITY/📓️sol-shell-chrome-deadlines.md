# Shell Chrome Deadlines

## Contract

The Shell now exposes `ShellState::next_chrome_deadline(monotonic_now_seconds, wall_now_ms)`. Tooltip hover and transient-notice state remain epoch-millisecond records because their existing paint paths compare against browser `Date.now()` or native `SystemTime`. The accessor selects the earliest pending deadline and translates only its remaining duration onto the process monotonic clock:

`monotonic now + max(0, wall deadline - wall now) / 1000`

Ready tooltips do not continue returning a deadline, so the 400 ms wake paints once without creating a wake loop. An overdue notice returns the current monotonic time so one frame can retire it. Non-finite samples are refused.

The neutral schema and corpus are:

- `Shell/🧪️fixtures/⏰️chrome-deadline/🧬️schema/🔣️.json`
- `Shell/🧪️fixtures/⏰️chrome-deadline/🔣️.json`

They cover tooltip-only, notice-only, both earliest orders, ready-tooltip quiescence, overdue notice wake, and idle state.

## Browser oracle

The actual `ChromeControlHint` is driven with fake browser timers at 399/400 ms. `ShellHost` now routes its real notice timeout through `scheduleShellTransientNoticeDismissV1`, which the same React fixture drives at 3999/4000 ms. The production duration is exported as `SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS`; the host no longer carries an unowned timeout literal.

`SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=@semio-tech/framework-renderer-react --excludeTaskDependencies -- bunx vitest run --config <renderer-react config> <shell-chrome-deadline test>` passed 2/2 on 2026-09-27.

## Accepted retained clock authority

`Ui::surfaces_next_clock_deadline` scans the exact accepted `(String, UiSurfaceToken)` roster once, rejects stale token aliases and non-finite deadlines, and returns the finite minimum. Visibility is captured as token-qualified state when a candidate seals. Seal and discard do not mutate the accepted clock. Exact ACK suspends hidden owners and clears repeat, tooltip dwell/reveal/dismiss state, and the accepted `PresentedTooltip` record. Reused IDs remain isolated by surface generation.

The Stepper clock laws now account for the accessible hovered control's legitimate 400 ms tooltip dwell before its 600 ms first repeat. They accept the tooltip transition, seal a fresh candidate, prove a later no-op preserves the seal, then prove the repeat invalidates it once.

## Text baseline correction

Field description and error text use an authoritative line-height baseline for wrapped line boxes. Single-line clipped controls already center a font-size box inside their control; applying the wrapped baseline to them pushed their glyph ink below a 24 px NumberStepper. `paint_retained_glyph_step_flowed` now selects the line-height baseline only for `Wrap` and preserves the centered font-size baseline for `Clip`.

## Native filters

- `shell_chrome_deadlines_follow_the_shared_wall_to_monotonic_contract`
- `shell_chrome_deadline_refuses_non_finite_clock_samples`
- `window_clock_keeps_a_noop_candidate_sealed_and_invalidates_it_once_when_hold_repeat_changes_state`
- `a_late_hidden_visibility_ack_cannot_suspend_a_reused_surface_id`
- `suspending_a_hidden_surface_clears_every_hover_tooltip_clock`
- `number_stepper_editing_and_hold_repeat_match_the_neutral_react_contract`
- `puzzle3d_settings_document_completes_retained_paint`

Native execution is coordinated by the root task because it owns shared Cargo sessions.

## Next timed Shell slices

The current accessor intentionally includes only ChromeControlHint dwell and transient notices. Remaining Shell-owned time-dependent presentation found during the source census:

- tutorial progression and tour reveal latches;
- caret blink at the frame build boundary;
- wheel-zoom settle deadlines;
- document-opening elapsed banners;
- plugin installation/progress flush cadence;
- agent bridge and presence maintenance timers.

These require separate authority and scheduling laws rather than being folded into the two-duration chrome slice.
