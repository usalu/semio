# P7c1 Energy Numerical Microcursor Fifth Independent Re-audit

Date: 2026-08-25
Auditor: Terra (independent read-only source/static audit)
Scope: fourth-RED weather census remediation plus prior P7c1 regression surface

## Verdict

**GREEN — source/static scope.** The fourth audit's sole blocker, a weather census based on logical length rather than the owned `Config` weather backing capacity, is closed on the current tree. The new production `TimeSeries` append authority also remains correctly admitted, non-growing, and typed on failure. No regression was found in the fixed-table, forbidden-structure, checkpoint, or legacy-whole-helper census.

This is a new report and preserves the four preceding Terra reports. No production source or shared verifier was edited. No Cargo, Nx, Wasm, browser, build, or runtime test was run.

## Weather Census and Mount Trace

The repair meets the actual-capacity rule in the contract (repair contract lines 90-93), rather than merely renaming the prior logical count.

- `observed_weather_records` selects `weather.records.capacity()` for supplied weather, with only the stated zero-capacity/logical-one fallback. `weather_copy_target` is deliberately separate and selects `records.len().max(1)` for copy work (`⚙️engine/🧪️sim/🦀️component.rs:379-397`). Thus a reserve-only expansion changes admission even when it changes no logical record, while an empty input still has a one-slot synthetic-record policy.
- The capacity value is an independent `weather_records` entry in the checked item sum (`🧪️sim/🦀️component.rs:248-291`) and in the explicit `WeatherRecords` comparison before `ObservedItems`, bytes, and pages (`332-375`). The Config backing is charged as `records.capacity() * size_of::<WeatherRecord>()` (`522-524`); the simultaneous fixed job-table backing is separately charged as `weather_records * size_of::<Option<(usize, WeatherRecord)>>` (`290-292`); page count derives from that checked byte total.
- After every numerical comparison succeeds, admission makes exactly one `FixedTable` of the observed weather capacity before constructing the mounted authority. An allocation refusal releases only the temporary abandonment slot and returns the same moved `Model`/`Config` in the typed `WeatherRecords` rejection (`986-1016`). `FixedTable` itself allows one exact admission, box-converts its slots, and rejects a full or repeat insertion (`🔋️model/🦀️component.rs:24-88`).
- `ResolveWeather` uses the logical copy target and direct `Vec::get(index)` source access, then inserts exactly one weather record per grant. A table-overrun becomes retained `WeatherFault::SlotRejected`, not a grow or fallback (`🧪️sim/🦀️component.rs:1212-1223, 1875-1911`). The source census found no production weather `Vec::new`, `push`, or reserve growth path.
- The live test laws now specifically reserve weather capacity without adding a record, assert that census capacity is greater than logical length, prove `WeatherRecords` MAX+1 returns the exact `Config.records` pointer, copy only the two logical records in two grants, and reject the next fixed slot (`3627-3678`). The companion law proves the reserve-only delta independently changes `ObservedItems`, charges both Config and job-table bytes, derives pages from bytes, and returns the exact Config weather pointer on `ObservedItems` MAX+1 before mount (`3681-3709`). These test bodies were inspected, not executed.

## Time-Series and Prior-RED Regression Trace

- Production `TimeSeries::append_admitted` requires both timestamp and value capacities to meet `admitted_samples`, checks length parity, then checks the maximum before either write. Only then does it append the pair; each refusal has a retained typed `TimeSeriesAppendError` (`📤️output/🦀️component.rs:73-99`). The live aggregate receives that error, retains it in `AggregateZoneWork::series_fault`, marks the retained work rejected, and the aggregate stage faults without trying another mutation (`🧪️sim/🦀️component.rs:1646-1657, 2201-2213`). The older unrestricted `push` is `cfg(test)` only.
- The associated source law covers unadmitted backing, parity mismatch, exact maximum, maximum+one no-mutation, and unchanged capacities (`📤️output/🦀️component.rs:214-239`). It was inspected only.
- The scoped live-source census found no `HashMap`, `BTreeMap`, `extract_if`, or `ObservedTable`. The remaining headless `Engine::run` is not a counterexample: the contract expressly preserves it only as an adapter over this job (repair contract lines 15-17), and the live body drives `EnergyJob` through `run_to_completion` (`🧪️sim/🦀️component.rs:3128-3166`). Legacy model/precompute/kernel/sizing/dispatch whole helpers remain absent from the production-public census; checkpoint serialization/restoration stays test-gated as established by the prior audit.
- Fixed boxed tables, one-time admission, stable direct insertion/binary lookup, and bounded tail disposal remain in the live table implementation (`🔋️model/🦀️component.rs:24-164`). The prior retained typed reserve-fault, generation-qualified abandonment/recovery, cancellation/deadline/stale, and one-owner close structures were re-censused and no forbidden regression was found.

## Allowed Evidence Gates

| Gate | Result |
| --- | --- |
| Scoped `rustfmt --check --edition 2021` for sim/output/model/precompute/kernel/sizing/dispatch/meters/results | GREEN (no output) |
| Scoped `git diff --check` for those sources and the P7c1 fixture | GREEN (no output) |
| Fixed-table / dynamic-map / weather-growth / production-whole-helper `rg` census | GREEN (no prohibited scoped production result) |
| `bun` JSON parse of `🪨️tests/p7c1-energy-numerical-laws.json` | GREEN (`semio.energy.numerical-laws/1`, 14 admission dimensions) |

## Deferred Gates

Compiler/type checking, the inspected unit laws and 19 MAX+1 mutations, allocation-failure paths, Drop/panic/session-loss recovery, cancellation/deadline/stale execution, 1/2/4/default chronology, batch parity, product mounting, checkpoint/publication replacement, and runtime close behavior were not executed. P7c2 checkpoint/publication and P7c3 WorkerPool/product integration remain outside P7c1 and this static verdict.

