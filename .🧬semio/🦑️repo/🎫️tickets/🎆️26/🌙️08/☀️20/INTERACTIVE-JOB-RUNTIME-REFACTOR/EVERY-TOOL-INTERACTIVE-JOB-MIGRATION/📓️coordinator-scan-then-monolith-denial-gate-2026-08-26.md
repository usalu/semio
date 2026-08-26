# Scan-Then-Monolith Denial Gate

Date: 2026-08-26

## Outcome

The authoritative `verify interactivity tool-jobs` gate now rejects a resumable `ArtifactCommandWork` implementation when its yielded steps only scan inputs under a `*-command-scan` stage and its terminal step calls a helper that invokes the original one-shot `command.dispatch` reducer. Input scanning can support validation, but it does not bound the mutation, reconstruction, serialization, or publication work and therefore cannot satisfy the interactivity contract by itself.

The proof schema now records whether each owner-local contract is `bounded_first_step` or `resumable`. Only resumable proof rows are subject to this denial. Rejected rows remain in `remainingCommands` with the reason `resumable route scans inputs then invokes a monolithic reducer`; the JSON report also exposes the exact `scanThenMonolithRows` ledger.

## Fresh evidence

Command:

```sh
bun ./📜️script.ts verify interactivity tool-jobs --format json --output '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📊️coordinator-official-tool-jobs-current-2026-08-26.json'
```

Result: expected RED, exit 1, with 468 hostile/static self-tests clean. The new ledger contains 100 rejected rows:

- Lowpoly: 47 via `lowpoly_retained_reduce`
- Process3d: 26 via `process3d_retained_reduce`
- Procedural2d: 18 via `procedural2d_retained_reduce`
- Sourcing Curate: 9 via `sourcing_curate_retained_reduce`

The overall truthful remaining-command count increased from 681 to 781 because these 100 scan-only routes no longer receive false migration credit. Separate live blockers remain 36 process-global payload stores, 35 app-owned import-media routes, and two Puzzle3d proofs for framework-owned action constants that must be removed from the app-owned catalog.

## Subsequent checkpoint

Lowpoly replaced its scan-only terminal reducer with a typed, operation-owned segmented workspace. A fresh authoritative run now reports 53 scan-then-monolith rows and 734 remaining commands. The 47 Lowpoly rows left the denial ledger; Process3d contributes 26, Procedural2d 18, and Sourcing Curate 9. The two forged Puzzle3d framework-action proofs were also removed, so they no longer appear as catalog failures. Lowpoly runtime timing and kernel-finalization proof is still pending and static removal from this ledger is not a runtime completion claim.

## Hostile law

The verifier self-test constructs a resumable fixture whose progress stage is `fixture-command-scan` and whose terminal helper calls `command.dispatch`; the detector must return exactly one rejected row. Replacing that terminal call with `workspace.step_one(command)` must remove the rejection. This raised the authoritative self-test count by one.

## Closure condition

A rejected cohort becomes eligible only when its operation-owned mutable workspace performs real bounded mutation/reconstruction work during yielded steps, binds checkpoint restore to the exact workspace identity, and hands back or incrementally closes every retained owner. Renaming the progress stage is not an accepted repair; runtime max+1, interruption/replay, cancellation, close, native/Wasm, and timing laws remain required.
