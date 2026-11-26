# Capacity Sizing: Refused Component Pass (2026-11-26)

## Symptom
`repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies`: run 2 was admitted at 512 MB ("sized at 46% of the default from 1 earlier run(s)"), the grant grew only to 1638 MB, and the component pass threw `ParentComponentMemoryRequiredErrorV1` (needs 3757 MiB). Logs: `🗑️generated/component-build-2.log`.

## Root causes
1. Learning (`recordHostOutcomeV1` and the `learn` in `host-capacity/🟦️.ts`) only ever sees the measured peak. A run that is refused or fails for lack of memory teaches nothing (`learnableHostRunV1`), so the class keeps the small estimate of an earlier cheap run and the next run is sized the same way. The run knew its need (3757 MiB) but had no channel to tell the holder.
2. `ParentComponentMemoryV1` froze the allowance read at start. `runPassesInJobSlotsV1` threw as soon as `required > allowance` with no fleet placement, although the holder grows the grant while the host frees memory (log: 1638 -> 2574 MB after 435 s).

## Fix
- `📐️estimates`: `hostRunAfterNeedV1` (pure, language-neutral) lifts a run to the need it declared and marks it `limited`, which the existing learn rule accepts for failed or cancelled runs. A run can raise the learned size, never shrink it.
- `🎫️gate`: `declareHostCapacityNeedV1` writes `<grant>.need` (largest declaration wins); the holder reads it when the run ends and passes `needMegabytes` in `HostRunV1`; the file is removed with the grant. Protocol const `needSuffix`.
- `host-capacity/🟦️.ts` (repo product): `learn` applies `hostRunAfterNeedV1`.
- `🧵️job-slots`: `ParentComponentMemoryV1` takes an optional `ParentComponentGrowthV1` (`ceilingMegabytes`, `waitMilliseconds`, `refresh`, `declare`). Every attempt refreshes the live allowance (never lower). A requirement above the grant that the ceiling can carry declares its need once and waits (cancellable, bounded by `ADMISSION_POLICY_V1.growth.waitMilliseconds`); an unreachable requirement or an expired wait still refuses with the same error.
- `🧩️component-pass`: wires refresh to the live reservation, declare to `declareHostCapacityNeedV1`, ceiling to the host memory above the reserve.

## Tests (red first, then green)
- `🏗️build/🌡️host-capacity/🧪️tests/📣️need/🟦️.ts` with language-agnostic `🧫️fixtures/🔣️.json` (4 rules) plus the end-to-end laws: a refused need re-sizes the class to at least that need; the gate hands the declared need to `learn` for a failed body.
- `📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🧪️tests/🌱️growth/🟦️.ts`: waits and starts when the live grant grows, refuses at once past the ceiling, bounded wait, cancellation.
- Regression: job-slots `🎟️parent`, estimates, upgrade gate suites pass.

## Limits
- The holder still grows the grant by its own rules (free memory, queue); the declared need drives learning for the next run and the wait polls the live reservation, it does not force the holder to grow to a target.
- No third-party library oracle exists for the need rule (a one-line max); the existing estimates Python oracle was not extended.
