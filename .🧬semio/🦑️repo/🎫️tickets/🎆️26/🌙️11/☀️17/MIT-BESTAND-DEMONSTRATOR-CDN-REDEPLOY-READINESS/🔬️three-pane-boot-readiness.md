# Koordinator, Aussuchen and Bearbeiten boot readiness (Nov 21, 15:50 PST)

## Lease correction

Present and intact in the live source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs` holds `Lease::leave_if_idle`, `CommandLease::release_idle` and the shared `release`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs` computes `command_source_pending` once (shared command source or action ingress) and calls `release_idle` for idle publication. The background-resume admission uses the same predicate. Explicit completion paths still call `release`.
- Language-agnostic law: `📣️host-event-routing/🧪️tests/🔣️.json` (seven scripts). Rust test `🧪️tests/🦀️.rs`.

Evidence run now against the live file: `command-lease-law.py green` prints "Rust and Python agree on all 7 fixture sequences"; `command-lease-law.py red` prints "Legacy Rust releases the active holder against the Python oracle". The script extracts the generic `Lease` text from the live source, so it proves the current text.

## Blocking finding: `semio-plugin-instance-app` is over its source boundary

A locked, offline `-j2` `repo cargo ... test -p semio-plugin-instance-dispatch` was refused by source admission before the compiler:

`Rust crate semio-plugin-instance-app exceeds its source boundary before the compiler: 19 of 32 files, 267663 of 262144 bytes`

The crate was 259,992 bytes at HEAD and 260,662 bytes when the lease fix was measured. It is 5,519 bytes over now, because uncommitted work by other agents landed concurrently and keeps growing (files touched at 15:36):

- new `🔌️plugin-app-impl/🎟️natural-publication/🦀️.rs`, 4,700 bytes (natural-source publication, `natural_source_capability`, `advance_natural_source_format`),
- `plugin-app-impl/🦀️.rs` 94,529 to 97,394 bytes (close batch, natural-source close demands, TimeTravel residue diagnostic, `command` diagnostic string, lease change about 400 bytes),
- `close-terminal` +100 bytes.

Every Demonstrator release attempt that includes this live tree will be refused, so the queued browser module rebuild cannot link until this is cut by at least 5.6 KB (move cohesive code into a crate with headroom, most naturally the natural-publication owner into `semio-plugin-instance-docs`, where `NaturalFilePublication` already lives). I did not do this myself: it is another agent's active feature and the files were being rewritten while I read them. The lease fix is not the cause (about 0.4 KB).

The native cargo test of the real `semio-plugin-instance-dispatch` crate therefore could not run; the standalone rustc law above is the only lease evidence for the current text.

## Aussuchen console error

Source: `🗑️generated/0908-demonstrator-linked.log`, wakSUm7, both locales. The browser console assertion received:

`setContributions command failed ... original host classification lost its registered action` (`plugin.command-failed`, origin app, not retryable).

Cause: the addressed-command host path classified `setContributions` with `tools.registry.get(action)`, but the command is app-owned and lives in the app/mode command registry. Koordinator reported the same classification error; Bearbeiten the same pending state.

Source now: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs` `dispatch_command` resolves the kind with `manifest_command_kind` (app or active mode owner lookup through `registry.app_command` / `registry.mode_command`) for both the initial and the host-active resumed branch. The remaining `registry.get(action)` at line 162 is the plain action path, where the action is registry-owned, so it is correct. The structural test `🧪️tests/🔬️app-typed-command-full-operation/🧾️source-rejection-anchors-and-host-lane/🦀️.rs` already asserts: no `registry.get(` in the addressed command route, two `manifest_command_kind(address` calls, and two `drive_original_host_configuration(command_id,kind,` calls.

## Bearbeiten host parser

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🦀️.rs` is integrated: `advance` runs up to 32 consecutive funded `advance_with` steps in the caller's existing turn and exits on yield or zero paid progress. SHA-256 `c2e22302...fb47` is identical to the value in `🔬️bearbeiten-ingress-throughput.md`, which measured 2,550 host calls against 54,913 for the 26,538-byte Process fixture.

Gap fixed: the burst was guarded only by a ticket script. The checked-in test `🧪️tests/🦀️.rs` now counts host turns per case and asserts an optional `maximumTurns` in the neutral fixture `🧪️tests/🔣️.json` (schema `🧪️tests/📐️.json`).

## Status per pane

- Koordinator: lease fix and classification fix present in source. Not bootable until the instance-app size boundary is restored, a Demonstrator module is rebuilt and the official acceptance (`♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`) passes.
- Aussuchen: same, plus the console error cause is the classification lookup, fixed in source.
- Bearbeiten: same, plus parser burst integrated and now guarded by a fixture turn bound.

## Test evidence

- `repo cargo --store=isolated --configuration=prototype --parallelism=2 -- test --locked --offline -j2 -p semio-plugin-host-configuration`: 2 passed, 0 failed (all 27 neutral fixture cases plus cancellation), including the new per-case `maximumTurns` check. Measured turns: the 26,538-byte Process fixture needs 19,352 host turns under the fixture's quoted grant (bound set to 24,000); all other cases at most 1,821. This guards against a regression of the 32-step burst but is not a bit-for-bit replay of the browser grant (that figure, 2,550 against 54,913, stays in `🔬️bearbeiten-ingress-throughput.md`).
- `command-lease-law.py green` and `red`: see above.
- `semio-plugin-instance-dispatch` native test: refused by the instance-app size gate (see blocking finding). Log: `🗑️generated/three-pane-lease-cargo.log`.

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🧪️tests/📐️.json`
