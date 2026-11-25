# Host Memory Guard (swap, compressed memory, caps, hold)

## Finding: the guard already existed, with four gaps
The host warden (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/`) already samples available memory, compressed memory, swap used/free/out-rate and the OS pressure level (`📡️sampling`, macOS sysctl, Linux PSI, Windows commit), classifies repo-owned process trees into ranked kinds and suspends (SIGSTOP) or cancels the cheapest ones in levels calm/guard/danger. It runs in `enforce` mode here (state: 8 suspends, 9 cancels). The level names guard/danger are what the request calls tight/critical; user-visible texts now say calm/tight/critical (de: ruhig/knapp/kritisch).

Why the host still swapped to 13 GB:
1. Swap was only a rate trigger (swap-out >= 16 MB/s) and via a headroom that counts growable swap as free room, so a deep swap with moderate `available` never raised the level.
2. Compressed memory was sampled but never read by the law.
3. Subject size was the resident set (`ps rss`), which macOS does not count for compressed pages: two headless Chromium trees at 17.6 GB compressed looked tiny. The `browser` kind also only matched the repo browser directory, not a probe launching Chromium/Firefox from elsewhere.
4. New commands were only gated by the native-build admission (new compile units); the host-wide capacity gate (`withHostCapacityV1`) ignored the warden state.

## Design (all schema-first in `🛡️warden/🧬️schema/🔣️.json`, law pure in `🛡️warden/🟦️.ts`)
- Signals: unchanged one-interface sampling per OS; the warden Host record already carries available, compressed, swapUsed/Free, swap-out rate, pressure.
- Level law (`decideHostWardenV1`, `swapDepthHostWardenV1`): swap in use and compressed memory have depth 1 (guard line) and 2 (danger line): swap-used 35/55 percent of RAM, compressed 35/50 percent of RAM, and, where the swap has a fixed size (Linux, free swap known, no growing volume), swap-share 45/60 percent of the swap itself (the requested "60 percent of swap total"; macOS swap grows, so its total says nothing). Depth only raises the line available memory must clear (depth 1: hold below the resume line; depth 2: critical below the guard line), so shedding stops as soon as memory is back; a deep swap alone never sheds (swap pages stay used long after the cause is gone).
- Trigger (`available|headroom|swap-used|swap-share|compressed|pressure|critical|swap`) is part of the decision, the alarm (both languages) and the state.
- Hold: `withHostCapacityV1` waits (queued, cancellable, progress line in en/de) while the fresh warden state says hold and the request needs >= `holdMinimumMegabytes` (512); light commands, a calm, absent, stale or unreadable warden never wait. Port `warden` for tests; production reads the machine-wide state.
- Shed: unchanged ladder (guard: suspend cheapest, keep the most expensive running; danger: cancel cheapest, suspended first), resume one per calm tick; never touches non-repo processes (roots/identities/never tables).
- Caps: kinds declare `capMegabytes` (remote/local unit 10240, pool-child/farm-worker/slice-worker/browser/headless-browser 4096, warm-deriver 6144, heavy-question/test/test-binary 8192), bounded by `capMaximumBasisPoints` (60 percent) of RAM. A subject over its cap is cancelled (SIGKILL tree, cancel record, alarm reason `cap`) at any level, even calm. The loop measures capped subjects by footprint (`observeTreeMegabytesV1`: macOS `footprint`, Linux smaps, Windows private working set) at most every `capMilliseconds` (5 s) for the `capProbeMaximum` (8) largest, never below the resident set; a failing probe falls back to rss.
- New kind `headless-browser` (rank 46): Chromium/Firefox with `--headless` below a repo root, from any path; renderers (`--type=`) belong to their browser tree.
- Top non-repo consumers: `others` in `state.json` (top 5 trees outside every subject, grouped by topmost ancestor), reported at non-calm levels, never signalled.
- Visibility: `repo capacity status` prints the guard lines (level, trigger, numbers, interventions, last action, foreign consumers) and `--json` carries `warden`; `repo fleet status` already shows level, interventions and last alarm per worker; events are `alarms.jsonl` + stderr (en/de) + `cancelled/<pid>.json`.

## Not done / limits
- Windows stays observe-only (`Platforms.observeOnly`): no SuspendThread/cancel implementation yet; Windows signals (commit charge) are sampled.
- Fleet-status worker memory facts do not yet carry `trigger`/`others` (closed fleet schema with fixtures); `repo capacity status` does.
- Processes of non-repo origin (a user's own browser) are only reported, by design.

## Tests
- `🛡️warden/🧪️tests/🟦️.ts` (36 tests): corpus `🧫️fixtures/🔣️.json` (decisions 71 incl. 19 new swap/compressed/cap vectors, classifications incl. headless browsers, `others`, `holds`) equals independent Python oracle `🐍️.py` (`--decide --classify --others --holds`, stored `expect` written by the oracle); watchdog unit tests with a fake process table (cap kill with tree and cancel record, footprint beating rss, failing footprint, no cap for uncapped kinds, observe mode, state `others`); alarm/status/hold texts en/de.
- `🧪️tests/🛡️warden-hold/🟦️.ts` (3 tests): gate holds heavy work, queue stays, cancellable, light/calm/absent/stale/unreadable never hold.
- Run: `bun test ./🟦️.ts` in the test folders (registered bun); gate, fleet-status, capacity command and admission-related suites pass (64 + 34 + 56 tests). The pre-existing MOUNTED real-children test is timing-sensitive at host load > 10 (2 of 8 runs failed at `b` not yet stopped; unrelated to the law, passes at normal load).

## Follow-up: the Mac ran out again (3254 MB available, compressed 6851 MB, pressure warn, reported calm)
Cause: not stale code. The law of the first change deliberately made OS pressure alone, and a deep swap or compressed memory above the resume line, calm ("pressure never holds"); compressed depth only raised the line to the resume line (2457 MB), and 3254 > 2457.
Fixes (schema-first, oracle-checked):
1. Level law: pressure warn is at least tight; pressure critical or compressed at its danger line is critical; compressed at its guard line is tight. Only a severe danger (available below the danger line, headroom below its danger line, deep swap/compressed with available below the guard line) cancels; pressure/compressed-only critical suspends (and cancels what stays suspended too long). Deficit is at least `pressureDeficitMegabytes` (1024) while pressure or compressed holds the level. Live: `repo capacity status` printed "Memory guard: tight (enforce), compressed memory is above its line ..." at pressure warn, 3685 MB available (it is calm again now that pressure is normal).
2. Reservations: `memory.interactiveReserve*` (policy.json: 25 percent of RAM between 2048 and 8192 MB = 4096 MB on 16 GiB) counts as taken by what is not granted at least, so admission divides total minus max(measured non-granted footprint, reserve). The non-granted footprint (raw `cargo`, Claude helpers, scripts, headless browsers) was already measured as total minus available minus granted; the reserve is the floor under it.
3. Hold: `holdOperations` (run, run-many, affected, exec, watch, cargo, command) are held and queued while the warden is tight or critical whatever they declare; other operations by `holdMinimumMegabytes`.
4. Session cap: `memory.sessionCapBasisPoints` 4000 (6553 MB of 16 GiB): a further request of a session that already holds reservations waits with reason `session-cap` (en/de text) while held plus its least need pass the cap; a session holding nothing is always served.
Tests: warden corpus +11 decision/hold vectors (oracle), `🧪️tests/🛋️reserve` corpus + Python oracle for reserve and session cap; `🧯️progress` oracle test now runs on the policy without reserve/cap (its oracle models the plain policy).
Limit: the warden still only acts on repo-owned trees; a raw non-repo `cargo` is measured (it shrinks admission room) and listed under foreign consumers, never signalled.
