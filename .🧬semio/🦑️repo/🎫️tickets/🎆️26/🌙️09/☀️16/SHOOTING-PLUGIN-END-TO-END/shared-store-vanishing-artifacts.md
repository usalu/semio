# Shared store: dependency artifacts that "vanish" mid-build

## Symptom

`repo cargo --store=shared ... test -p ... --lib` failed twice with
`extern location for <dep> does not exist: .../u/l/p07d6f3/prototype/build/<crate>/<hash>/out/lib<dep>-<hash>.rmeta`
(run 1: `semio-framework-job-capture` and `-return`; run 2: `semio-os-store-ledger`).
An earlier lane (`q0c9bae`) shows the same failure for `equivalent` (indexmap).

## What it is not

- Not the replica daemon: `.cache` is git-ignored and nothing was ever written, then removed.
- Not the unit/lane collector: the `out` directories of the missing units were never modified after
  Cargo created them (birth time equals modification time); units are only collected after 30 minutes.
- Not a rustc ordering problem: a real rustc writes the `.rmeta` before it announces it (measured).
- `build/<crate>/<hash>/out` is Cargo's new build-directory layout, not a relocation by a wrapper.

## Evidence

- The unit cache event log of the lane (`.cache/b/r/m/u/x/p07d6f3.jsonl`) has no event at all for the
  missing units (`capture`, `return`, `interaction`, `ledger`), not even the `remote-miss` that every
  wrapper emits within milliseconds; no rustc incremental session exists for them in run 1.
- Cargo nevertheless moved on to the dependent within one second and wrote no `dep-lib-*` fingerprint
  for the missing unit: the compiler wrapper process ended with status 0 without compiling.
- No "waiting for other jobs" line: no job was still running.
- The same silence appears in lane `q0c9bae` (`anyhow`, `semver`, `bitflags`, `equivalent`).

## Root cause

`unit-cache/🛰️fleet/📡️client/🟦️.ts`: `call()` rides through a fleet cache outage ("a restart takes a few
seconds") with `await pause(wait)`, and `pause` created its timer with `timer.unref()`. In a wrapper
process the request that failed was the last thing the event loop held, so the loop emptied during the
pause and Bun ended the process with exit status 0 (reproduced: a script awaiting an unref'd timer
prints nothing and exits 0). Cargo reads status 0 as "unit compiled", releases the dependents, and the
first dependent finds the extern missing. The condition needs the fleet to be reachable within the last
five minutes and to fail once (LAN blip, host under memory pressure with load 14), which is why it hits
after minutes of building, in bursts of consecutive units, and only when the fleet tier is on.

A second defect made it silent: the wrapper entries set the exit status only when the work settled, so
any early end of the process was a success.

## Fix

- `pause` no longer unrefs its timer: a promise somebody awaits must keep the process alive.
- `📜️script.ts` and `🏁️entry.ts` set `UNFINISHED_EXIT_STATUS_V1` (70) before the work starts and replace
  it with the real status (entry uses `.then`, because Bun ends an unsettled top-level await with 0
  even when an exit code is preset: measured).

## Tests (unit-cache fleet suite)

- ride-through in a child process that has nothing else to wait for returns the 404 answer after one
  retried failure (fails with the unref'd timer: no output, status 0/70).
- a process ending before its work settled exits 70.

Run: `repo run repo:test-cargo-unit-cache-processes`.
