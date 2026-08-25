# Terra P7c1 Energy Numerical Microcursor Fourth Independent Re-audit — 2026-08-25

## Verdict

**RED.** The two third-audit implementation defects are substantially repaired, including the production TimeSeries append path. The weather preflight, however, still uses logical record length for its independent weather item/max dimension. P7c1 expressly requires actual observed backing capacity and forbids logical length. Therefore Config capacity MAX+1 is not rejected by the declared `WeatherRecords` bound.

This new report preserves all three earlier Terra audits.

## Repaired third-audit residuals

### Weather storage and normal live path — GREEN

- `EnergyJob::admit` creates `FixedTable<usize, WeatherRecord>`, admits it before mounting `EnergyJobAuthority`, and releases the claimed operation slot on allocation refusal (`⚙️engine/🧪️sim/🦀️component.rs:980-1057`). The fixed table is boxed one-time slot storage, not a retained vector.
- Its fixed slots are charged to `observed_bytes` and hence pages (`290-292`). `ResolveWeather` inserts exactly one stable direct slot per fuel grant, records `WeatherFault::SlotRejected` before emitting a job fault on overflow, and all later weather users read direct indexes (`1885-1900`, `1926`, `1948`, `2004`, `2159`). Close uses the table tail-pop protocol.
- The source law materially verifies moved Config record-pointer identity on rejection/retry, fixed slot capacity before/after two copies, forced-overflow fault retention, and bounded close (`3617-3659`). It was not executed under the no-Cargo constraint.

### TimeSeries append — GREEN

`TimeSeries::append_admitted` is a production crate-private method, not `cfg(test)` (`⚙️engine/📤️output/🦀️component.rs:72-93`). Before either push it verifies both admitted backing capacities, paired current lengths, and the common maximum. With paired lengths and capacity at least the admitted maximum, both subsequent pushes are allocation-free; every error returns a discriminated `TimeSeriesAppendError` before mutation.

The aggregate path gives the series the exact admitted sample maximum during fixed-table publication (`⚙️engine/🧪️sim/🦀️component.rs:1613-1633`), calls `append_admitted`, retains its exact error in `AggregateZoneWork::series_fault`, and turns the aggregate work into a retained fault (`1635-1646`, `723-753`, `2193-2198`). The owned append law covers missing backing, length mismatch, exact MAX, and MAX+1/no-growth (`📤️output:214-239`). This closes the previous static missing-method error.

## Remaining blocker: weather item admission is length-based, not capacity-based

`observed_weather_records` returns `weather.records.len().max(1)` at `⚙️engine/🧪️sim/🦀️component.rs:379-387`. That value becomes both the `weather_records` numerical dimension and the `observed_items` contribution (`248-292`), then directly drives the fixed weather-table admission (`990-1005`).

The original Config owner can hold greater allocated backing capacity than its logical length. A Config containing `Vec::with_capacity(2)` with one weather record has `records.len() == 1` and `records.capacity() == 2`. The current census consequently reports one weather record and passes `EnergyNumericalBounds` with `weather_records == 1`, although the exact Config owner retains two WeatherRecord backing slots. A reserve-only Config capacity MAX+1 mutation likewise leaves `weather_records` unchanged.

`observed_model_bytes` does account for `weather.records.capacity()` at `🧪️sim:513`, so the byte subtotal may reject such a graph under a deliberately tight byte maximum. That does not repair the independent weather-record/item gate: P7c1 requires actual observed backing capacity, not logical length, and requires exact MAX+1 behavior per declared independent dimension. The contract states this explicitly in `📓️p7c1-energy-numerical-microcursor-repair-contract-2026-08-24.md:87-93`.

The new weather law uses a two-element literal (`🧪️sim:3617-3632`) whose length happens to equal its capacity; it does not mutate Config weather capacity while holding logical length fixed. It therefore cannot establish the required exact Config-owner capacity law.

## Regression census

The current scoped sources retain the prior structural GREEN repairs:

- Fixed boxed-slot `FixedTable`, one-time admission, binary/direct lookup, and one-slot tail close remain in `🔋️model/🦀️component.rs:24-164`.
- No scoped `HashMap`, `extract_if`, or `ObservedTable` occurrence remains.
- Public whole model/output/kernel/precompute/sizing/dispatch helpers stay test-gated/crate-private; `Engine` still consumes `Model` and `SimulationConfig` by value.
- Production checkpoint encoder/decoder remains `cfg(test)`; normal preview is scalar fixed 42-byte output.
- Retained typed reserve faults, fixed abandonment/recovery, 19 live MAX+1 source mutations, and actual nested cancel/deadline/stale cursor loops remain present. Those test bodies were not executed.

## Allowed gate results

| Gate | Result |
| --- | --- |
| Scoped edition-2021 `rustfmt --check` | GREEN |
| Scoped `git diff --check` | GREEN |
| Owned P7c1 law fixture parse/key check | GREEN |
| Scoped forbidden-source census | GREEN except test-only checkpoint JSON references; no production map/extract-if/weather-Vec/old TimeSeries call regression found. |

## Required closure

Use actual `config.weather.records.capacity()` for the WeatherRecords/item bound and reserve the fixed weather table to that capacity, while retaining a separate logical target cursor based on `len().max(1)`. Add a Config reserve-only MAX+1 mutation that proves the exact weather backing owner is rejected/retryable without relying on the byte maximum. Do not count compiler/runtime/parity/P7c2/P7c3 deferred gates as passed.
