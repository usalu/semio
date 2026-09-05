# Terra P7c1 Energy Numerical Microcursor Third Independent Re-audit — 2026-08-25

## Verdict

**RED.** The current remediation genuinely closes most of the two preceding source-level findings, but two live production defects remain: an unadmitted dynamically growing weather owner and a production call to a test-only `TimeSeries` method. The latter is a statically demonstrable non-test compilation failure. No Cargo/build/runtime gate was run, by instruction.

This is a new report; it preserves both earlier RED reports:

- `📓️terra-p7c1-energy-numerical-microcursor-independent-audit-2026-08-25.md`
- `📓️terra-p7c1-energy-numerical-microcursor-remediation-reaudit-2026-08-25.md`

## Read boundary and method

Read both Terra RED reports and the revised Sol report in full, then re-read the live Energy sim, model, precompute, kernel, sizing, dispatch, output, meters, results, glue surface, and owned P7c1 fixture. This was a read-only exact-tree audit. It ran only scoped rustfmt, diff, source census, and Bun fixture parsing—no Cargo, Nx, Wasm, browser, build, or runtime test.

## Repairs verified as genuine

| Requirement | Live source result |
| --- | --- |
| Fixed table authority | GREEN structurally. `FixedTable` owns `Box<[Option<(K,V)>]>`, admits exactly once, rejects repeat admission, box-converts the temporary allocator vector, uses binary lookup/direct index access, and tail-pops one slot (`🔋️model/🦀️component.rs:24-164`). It replaces validation, simulation state, precompute, time-series, and meter map storage. |
| No retained dynamic map/extract-if | GREEN in the scoped live sources. The census found no `HashMap`, `extract_if`, or prior `ObservedTable`. |
| Former whole APIs | GREEN structurally. Whole precompute/kernel/sizing/dispatch/model/output/meter helpers are `cfg(test)` plus crate-private; the production Engine adapter takes exact `Model` and `SimulationConfig` owners (`🧪️sim:3082-3094`). |
| Checkpoint/preview boundary | GREEN for production. Checkpoint rejection state, restore, and serialization are `cfg(test)` (`🧪️sim:217-243,1060-1167,1204+`); normal preview is scalar and fixed 42-byte (`63-71,1187-1201,2499-2511`). |
| Fault and close improvements | GREEN structurally for sizing/dispatch/output. Sizing/dispatch retain typed builder faults on reserve failure (`📏️sizing:161-175,260-267`; `🚦️dispatch:110-125`); output returns `OutputFault` (`🧪️sim:1752-1817`). Fixed tables and all examined retained owners use bounded tail/character close. |
| Live cancellation/deadline/stale law | GREEN structurally. The new law mounts and iterates the actual retained validation, precompute/surface, timestep-builder/timestep, zone/system/plant/schedule, warmup, both aggregation, sizing, backing, and output cursors (`🧪️sim:3736-3941`) rather than merely totaling taxonomy arrays. It injects cancel, deadline, and stale generation before `job.step`. |
| MAX+1/retry, abandonment, chronology | STATIC-PRESENT. The 19 heterogeneous live capacity mutations call `EnergyJob::admit` (`3540-3575`); checkpoint retry preserves a pointer in the test-only fixture (`3577-3606`); direct Drop/panic recovery and 1/2/4/default chronology are present (`3608-3680`). Execution is deferred by the no-Cargo boundary. |

## Blocking counterexamples

### 1. Live weather copies grow dynamically without an admitted backing stage

The numerical census computes `weather_records` from the supplied weather vector capacity or a fixed design-day count (`🧪️sim/🦀️component.rs:245-254`). But admitted `EnergyJobAuthority` starts its live weather owner as `Vec::new()` (`984`), and `ResolveWeather` then performs a normal `self.weather.push(...)` once per grant (`1851-1858`). There is no preceding `self.weather.try_reserve_exact(...)`, fixed page table, or capacity/length refusal for this owner anywhere in the production path.

This is not a mere test helper: `weather` feeds warmup and the run timestep (`2121-2126`) and is independently popped during close (`2319`). Thus every job with more than the inline zero capacity obtains one or more allocator growth grants after admission. It violates the required fixed/page/observed-capacity live storage and "one admitted allocation/copy unit" discipline even though each individual push is cursored.

### 2. Production calls a method compiled only for tests

`TimeSeries::push` is explicitly `#[cfg(test)]` at `⚙️engine/📤️output/🦀️component.rs:71-76`. The normal production aggregation path unconditionally calls it at `⚙️engine/🧪️sim/🦀️component.rs:1599-1605`:

```rust
let series = self.time_series.series.get_index_mut(...).expect(...);
series.push(self.hour_index as f64, zone_state.air.temp_c);
```

Under a normal non-test build that inherent method does not exist, so the crate cannot type-check this live path. Rustfmt validates parse/format only; it cannot clear the missing method. This independently prevents a P7c1 GREEN result.

## Additional adversarial observation: ordered-table precompute refusal must be intentional

The new table's binary lookup requires monotonic insertion (`FixedTable::insert`, `🔋️model:62-79`). Validation covers zones/materials/constructions/surfaces, but not fenestration IDs (`🧪️sim:1407-1415`). Precompute discards several `FixedTable::insert` results (`🧠️precompute:281-309,337-375,470-486`). A non-ascending valid-surface fenestration sequence reaches that code; the table becomes faulted. The outer precompute path does currently notice `backing_rejected()` and faults on the same grant (`🧪️sim:1864-1875`), so this is not recorded as a separate silent-fallback blocker. It is nevertheless a live compatibility/integrity constraint: model vector order must be specified as ascending for every binary-keyed family, or insertion failures must be carried as a precise typed validation rejection before precompute begins.

## Gate results

| Gate | Result |
| --- | --- |
| Scoped edition-2021 `rustfmt --check` | GREEN — sim, kernel, precompute, sizing, dispatch, model, output, meters, results all exit 0. |
| Scoped `git diff --check` | GREEN — same sources plus fixture exit 0. |
| Owned P7c1 law fixture | GREEN syntax/key parse — Bun parsed JSON and found `schema`, `admission`, `step`, and `terminal`. |
| `HashMap` / `extract_if` / `ObservedTable` live-source census | GREEN — no occurrence in scoped Energy authority sources. |
| Production cfg surface | RED — static `cfg(test)`/call mismatch for `TimeSeries::push`. |
| Actual mutation, recovery, chronology runtime evidence | DEFERRED — source tests are materially present but intentionally unrun. |

## Required closure

1. Add a dedicated weather-backing admission stage that reserves/fixes the exact weather record owner before the first copy, validates insertion length, and keeps its close one-record-at-a-time.
2. Make the live `TimeSeries` append a production crate-private bounded method (or move the write inline) and reserve/check its exact sample slot before every append; do not leave the only method behind `cfg(test)`.
3. Clarify/validate the required stable order for all binary-keyed model families, especially fenestrations and faults, before precompute. Retain a discriminated fault rather than relying on an ignored `insert` result plus generic later fault.

## Deferred gates

Compilation, test execution, 19 mutation execution, checkpoint fixture runtime, Drop/panic saturation, 1/2/4/default worker runtime chronology, allocation/watchdog evidence, numeric/reference parity, native/Wasm integration, and P7c2/P7c3 remain deferred. They cannot turn the two source-level blockers into GREEN.
