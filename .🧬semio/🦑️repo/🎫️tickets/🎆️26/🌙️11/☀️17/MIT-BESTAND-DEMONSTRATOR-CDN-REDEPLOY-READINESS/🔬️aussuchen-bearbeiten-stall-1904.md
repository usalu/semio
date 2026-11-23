# Aussuchen and Bearbeiten `setContributions` stall (1904 matrix)

Status: probable cause found by reading plus live probing, fix and law written and green in a standalone Rust and Python run. Not yet proven in the browser: it needs a Demonstrator module rebuild. See "Confidence".

## Live evidence (current core, built 18:08, which already contains the 06:42 lease fix)

Probe: `bearbeiten-ingress-probe.cjs` and `stall1904-console-shapes.cjs` through `overlay-serve.mjs` on the site `5dc1b077e86de2aa/out` (logs `🗑️generated/stall1904-aussuchen-probe.log`).

- Aussuchen and Bearbeiten behave identically. Boot command (seq 2) completes; sequence 4 pages are accepted; typed operation slots drain within about 45 turns (`live=0/64 peak=0`). After that the only more-work source is `command_ingress` for every remaining turn (about 630 turns/s), `effects=0`, no console error, status `command-pending` for 16,000+ replies.
- Every turn spends from the original turn grant without moving the command (`1/89/89/0/0` is the dominant receipt; one 1632-byte allocate and release plus a 26,240-byte construction repeat about 35 times per second). The `os-shell` watchdog reports `setContributions has held demonstrator#1's lane for 5000 ms`, the shell banner later says 118 s.
- The earlier dynamic finding (`🔬️aussuchen-current-ingress.md`, `🔬️contributions-command-lease.md`): lease arbitration at address 129741632 followed by `leave` on the same address in the same turn.

## Cause

`Lease::arbitrate` (host-event routing, `📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs`) grants a free lease and answers `false` on the granting turn by design ("the granting turn itself answers false"). The holder is confirmed only when the lane's next claim answers `true`.

Every command handler (`handle_command`, `handle_action`, `handle_action_invocation`, `handle_command_frame`, `handle_intent_frame`) starts with `capture_original_host_event_route`, which claims the lease. On the granting turn the handler returns `Ok(None)` before it stages any source, so nothing is pending: `shared_command_source_pending()` and `ingress.action_source_pending()` are both false.

The background typed-operation publication of the same original turn (`advance_typed_operation_publication`, app impl) then calls `original_command_scope.release_idle(cx, command_source_pending)`. The 06:42 fix keeps the holder only while a command source is pending. A grant that no claim has confirmed yet has no source, so it is released at once. The next turn grants again and answers `false` again. The lane never holds the lease, the command never gets past its first claim and answers `command-pending` forever, while each turn pays a claim (grant spends, no progress).

This also explains why the 06:42 fix (keep a pending holder) passed its fixture but did not change the browser: its fixture always had the holder confirmed first.

## Fix

`Lease` gains an `armed` flag (`🦀️.rs`, same file): set when a free lease is granted, cleared by the holder's own confirming claim (`arbitrate` answers `true`), by `leave`, and by `clear`. `leave_if_idle` never releases an armed holder. Explicit completion, error and close releases (`release`, `leave`, `clear`) are unchanged, waiters are unchanged, and the stall watchdog and turn limits are untouched.

Law (language-agnostic, `🧪️tests/🔣️.json`):
- new scripts `aFreshGrantSurvivesIdlePublicationUntilItsFirstClaim`, `aFreshGrantStillYieldsToAnExplicitLeave`, `everyRegrantIsArmedAgain`;
- script `idlePublicationKeepsThePendingCommandHolderAndWaiter` now confirms holder `b` with a claim before its idle release (law change: an unconfirmed grant is no longer idle-releasable).

The existing Rust test `every_scripted_lease_sequence_follows_the_law` runs every script, so no Rust test file changed.

## Tests

- Independent oracle (Python holder plus deque plus fresh-grant flag) against the actual lease source compiled with the pinned rustc, no cargo: `stall1904-command-lease-law.py red` (fresh grant not armed: fails at `aFreshGrantSurvivesIdlePublicationUntilItsFirstClaim:1`) then `green` (`Rust and Python agree on all 10 fixture sequences`).
- Cargo: `repo cargo --store=shared --configuration=prototype -- test -j 2 -p semio-plugin-instance-dispatch --lib host_event_routing`, log `🗑️generated/stall1904-lease-cargo.log` (see result at the end of this file).

## Rebuild needed

Source changed: `semio-plugin-instance-dispatch` only. Rebuild the Demonstrator module (`demonstrator`, which hosts the sourcing and process artifacts, so Aussuchen, Bearbeiten and Koordinator) and then the site (the site embeds the module core). No other module links this crate in the boot matrix except through modules that mount the same instance runtime (procedural, energy, gis, cad, fem, flow, process, puzzle, sourcing also link `semio-plugin-instance-dispatch`; they only need a rebuild for the next full release, the fix does not change their behavior unless they hit the same grant window).

## Issuer clock refusal

`🔬️issuer-clock-refusal.md` did not exist when this was written. That refusal (poll_wit, "original actor context changed its issuer clock/deadline or renewed work") is a hard error; the Aussuchen and Bearbeiten stall has no error, so it is a different, silent mechanism. If the other agent's cause is in `OriginalSourceBinding::matches` (`validate` of the emission, `pending_configuration_source`), it would turn a retried command from a stall into a refusal once the lease is held, and must be re-checked after the rebuild.

## Confidence

The grant-release livelock follows from reading and matches the dynamic lease-arbitration observation, but the guest has no names and no diagnostic copy was built (the build is running), so the loop was not observed in a debugger after the fix. If the rebuilt Demonstrator still stalls, next evidence: a `[DEBUG]` line in `claim_original_command_scope` (key held, armed, pending) and in `release_idle`.

## Cargo run result

At the time of writing the cargo test was still compiling dependencies (the Demonstrator build is running and shares the machine); it was left running in the background. Read `🗑️generated/stall1904-lease-cargo.log` for the `test result` line of `every_scripted_lease_sequence_follows_the_law`. The standalone rustc plus Python result above is the completed evidence.
