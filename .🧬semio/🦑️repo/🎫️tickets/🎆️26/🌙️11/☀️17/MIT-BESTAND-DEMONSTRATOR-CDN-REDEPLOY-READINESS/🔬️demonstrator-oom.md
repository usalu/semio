# Demonstrator wasm release build: SIGKILL of the rustc unit

## Cause
The kill is not jetsam and not the native-build capacity machinery. It is the host warden
(`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden`), a per-machine daemon that signals
`local-unit` (rustc) subjects with SIGKILL. Its alarm log (`~/Library/Caches/semio/host-capacity/warden/alarms.jsonl`)
holds 11 cancels of `semio-build-scheduler rustc --crate-name semio_hub_demonstra*` / `semio_s_artifact_*` since yesterday,
the last two at 09:59:52 (2239 MB) and 10:28:46 (130 MB, matching the final log failure). Each one says
`level danger, reason pressure critical`, with 2189 to 3885 MB available (guard 1638, danger 819) and a headroom of
6488 to 12077 MB (danger line 2048). The host had plenty by every number; only the macOS kernel pressure flag was `critical`
(3 GiB swap busy, constant swap-out), and `decideHostWardenV1` made `pressure === "critical"` alone a `danger`.

In danger the deficit is `max(guard - available, headroomGuard - headroom, 1)`, which is 1 MB when only the flag is set, so
the loop cancels the first candidate by (rank, newest first, largest): the youngest `local-unit`. That is why a 130 MB unit
that had just started died, and why the next attempt of the same unit dies again (the scheduler holds it back
"cancelled by the host warden before", then restarts it at the same flag). Pass-level retries cannot cure a flag that
stays critical for hours.

The build tool itself (admission, `charged x/y GiB`) never kills: admission only holds units back.

## Fix (root, policy)
OS pressure alone never decides `danger` (which cancels) any more. `critical` now behaves like `warn`: it holds at `guard`
(suspend only, reversible) when available memory is below the resume line (15 percent, 2457 MB here), and is calm otherwise.
Danger stays purely numeric (available below the danger line, or headroom below the headroom danger line), so real memory
exhaustion still cancels the cheapest units. With available 2189 MB the host is still below the resume line, so a unit may be
suspended (not killed); with 3885 MB nothing happens.

Files (all below `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/`):
- `🟦️.ts`: level law, `triggerHostWardenV1` (new trigger `critical` at guard), doc.
- `🧬️schema/🔣️.json`: guard trigger texts `critical` (en, de).
- `🧪️tests/🐍️.py`: independent oracle, same law.
- `🧪️tests/🧫️fixtures/🔣️.json`: two critical-with-plenty cases now calm, new guard-by-critical case, one case moved to numeric danger; `expect` regenerated with the oracle (`--check` passes).
- `🧪️tests/🟦️.ts`: trigger words test and the mounted test (mounted ladder: the danger tick was driven by the critical flag with deficit 1; it now uses numeric danger, 505 MB available below the 512 MB danger line, with `guardMinimumMegabytes:520` for that run so the deficit is 15 MB and the cheapest kill still covers it, then the next subject is suspended; the guard tick uses 515 MB).

## How it takes effect
The warden restarts itself when the digest of its sources changes ("its own source changed"), the next caller starts the
new code, no manual step. `repo` builds started afterwards are protected once the new warden is up (check
`warden.log` for `started: pid ..., code ...`). Verified: Python oracle `--check` passes and the warden tests pass 27/27 (`repo tools exec bun -- test ./...warden/🧪️tests/🟦️.ts`). No cargo build was run.

## Not changed
The overdue rule remains: a unit suspended 120 s while the level is not calm is cancelled. Under a persistent guard
(available below 2457 MB) a suspended unit can still end cancelled after 120 s; lowering memory use of the unit (profile) is
the second lever if that happens. Profile flags of the demonstrator crates were not inspected as the first lever sufficed.
