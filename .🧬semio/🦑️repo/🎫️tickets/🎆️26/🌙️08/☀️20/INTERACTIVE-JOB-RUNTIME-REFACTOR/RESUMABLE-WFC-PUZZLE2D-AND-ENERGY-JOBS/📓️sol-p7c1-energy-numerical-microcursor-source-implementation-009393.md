# Sol P7c1 Energy Numerical Microcursor Source Implementation

Date: 2026-08-25  
Owner: Sol Extra High source executor  
Status: Fourth Terra RED remediated; source-only fifth independent re-audit candidate, not yet accepted

## Boundary

This packet changes only Energy-owned numerical simulation sources and an Energy-owned hostile-law
fixture. It does not change the shared scheduler script, actor/shard/renderer/Puzzle2d/P8 sources,
or claim P7c2/P7c3. No Cargo, Nx, Wasm, browser, or product build ran while the shared Rust tree was
active.

## Implemented

### Terra RED remediation

- Corrected weather admission accounting to use the actual owned Config weather `Vec` backing
  capacity, with a logical-one zero policy, rather than collapsing it to `len().max(1)`. The fixed
  job weather table is admitted to that full backing credit while its copy target remains the
  logical record count. Observed items charge every owned capacity slot; observed bytes charge both
  the Config `WeatherRecord` backing and the fixed job `Option<(usize, WeatherRecord)>` slot, and
  pages are recomputed from the checked byte total.
- Added a reserve-only hostile mutation whose logical length is unchanged while capacity grows. It
  proves the weather-record comparator rejects before mount, preserves the exact Config/weather
  allocation pointer, and retries it. A separate mutation holds the record comparator open, lowers
  only observed-item credit, proves the independent `ObservedItems` rejection and exact-owner retry,
  and checks exact item/byte deltas plus page arithmetic. This directly guards against substituting
  `len().max(1)` for owned backing capacity again.

- Replaced the last live growable weather copy with an exact one-time admitted
  `FixedTable<usize, WeatherRecord>`. Its fixed boxed slots are allocated from the checked
  `weather_records` census before the job authority is mounted; the fixed slots are also charged to
  observed bytes/pages. `ResolveWeather` inserts one record per fuel grant, rejects any slot beyond
  the admitted maximum into a retained `WeatherFault`, all numerical reads use stable direct slots,
  and close pops one record per grant. The hostile law proves weather MAX+1 returns the exact Config
  weather allocation, retries it, preserves slot capacity during copy, rejects a forced extra live
  insert without changing records/capacity, retains the typed fault, and bounded-closes the owner.
- Added the production crate-private `TimeSeries::append_admitted` path. It verifies timestamp and
  value backing against the admitted sample maximum, verifies paired lengths, rejects MAX+1 before
  either mutation, and only then performs the two allocation-free pushes. Aggregate-zone work calls
  this production method and retains the discriminated `TimeSeriesAppendError`; its admitted sample
  maximum is the exact timestep count. The owned mutation law fills exact MAX, rejects MAX+1 with no
  value/capacity change, and covers missing-backing/length/full taxonomy at the live API boundary.

- Replaced the interim growable `ObservedTable` authority with an Energy-private `FixedTable` whose
  retained backing is an exact boxed slot array. Admission happens exactly once from the observed
  census, a second admission is a typed fault, insertion cannot exceed the admitted slot count,
  keyed tables require sorted incremental insertion and use binary search, chronology tables use
  direct stable slot indexes, and close retires exactly one occupied slot at a time. The temporary
  allocator vector is converted immediately into the fixed box and is never retained or regrown.
- Made every fixed table and all precompute/state/result/meter table fields crate-private. The live
  authority no longer exports a mutable `Vec`, linear query, capacity-growth, or string-key map
  surface. Time-series and meter string keys move into the retained character-close owner before
  their fixed slot is retired.
- Removed production reachability of `Model::{validate,zone_by_id,construction_by_id,
  material_by_id,surfaces_for_zone}`, output wildcard/CSV/reduction helpers, output registration,
  meter aggregate helpers, tariff/LCCA whole passes, and sizing whole-slice peak helpers. The
  remaining reference helpers are `cfg(test)` and crate-private.
- Disabled checkpoint publication and restore together in production for P7c1. The JSON checkpoint
  state, decoder, exact rejected-owner retry, and serializer exist only under `cfg(test)` as a
  hostile ownership/admission fixture until P7c2 supplies the retained fixed-page wire parser.
  Production preview remains a fixed 42-byte scalar record with no dynamic zone/result owners.
- Made output encoding reserve/missing-authority rejection an immediate retained job fault. Sizing
  and dispatch now retain typed backing/order/name faults on their mounted builder authority;
  sizing remains mounted for bounded close, dispatch has bounded one-result close, and no reserve
  rejection is converted to successful completion.
- Replaced taxonomy-count interruption assertions with live mutations of actual mounted
  authorities. The law drives cancel, deadline and stale generation before mutation for validation
  (16 stages), precompute (11), surface precompute (4), timestep admission (7), timestep (12), zone
  preparation (15), system substep (6), plant (4), schedule lookup (8), warmup convergence (5),
  zone/facility aggregation (5 each), sizing (18), finalization (13), result build (5), fixed backing
  cursors (3+7), and output sections (6), then restores the original cursor and bounded-closes the
  same authority.

- Removed production reachability of the legacy run-to-completion `PrecomputedModel::build`,
  `TimestepWork::new`, `SimulationKernel::{initialize,warmup,advance_timestep}`,
  `SizingManager::size`, and `Dispatcher::dispatch` adapters. They are test-only and crate-private;
  production drives the retained builders/job.
- Changed `Engine::{job,run}` to consume exact `Model` and `SimulationConfig` owners. There is no
  borrowed-input clone before admission.
- Added exact checkpoint restore rejection ownership to the test-only P7c2 fixture boundary.
  `from_checkpoint` checks the complete `EnergyNumericalBounds`, checkpoint byte credit, operation
  identity, and process-job slots, and returns the exact retryable `Model` and `SimulationConfig` on
  rejection; production cannot call the whole JSON decoder.
- Replaced live `ValidationWork`, `PrecomputedModel`, `SimulationModel`, `TimeSeriesTable`, and
  `MeterTable` `HashMap` authorities with Energy-owned observed-capacity, stable-insertion tables.
  Their backing is explicitly admitted before insertion and close uses stable `pop` cursors; the
  former `extract_if` close scans are gone. Monthly summary ownership is likewise insertion-ordered.
- Rebuilt sizing as reserve, surface, vertex, Unicode-name, and single-result substages. No
  `surfaces_for_zone`, whole geometry reduction, `format!`, or two-result emit remains in the live
  sizing turn.
- Added a retained dispatch builder with reserve, stable-priority validation, capacity accumulation,
  and one-equipment emission substages. The test adapter uses the same builder without clone, sort,
  collect, or multi-owner emit.
- Fixed the preview/checkpoint field mismatch (`run_backing_stage` belongs to checkpoint state, not
  the typed preview). P7c1 previews are deliberately scalar, fixed-size, and one retained page; they
  do not scan or allocate zone arrays. Production checkpoint wire emission is hard-forbidden until
  P7c2 supplies the retained-page protocol; the restore decoder and test-only round-trip serializer
  remain isolated for ownership/admission laws.
- Replaced the final `Vec<u8>` commit handoff with one-page-per-grant retained payload streaming.
  The provisional P7c1 output omits arbitrary-length names and debug formatting, reserves admitted
  backing once, emits one meter/series sample/summary record per turn, and does not claim P7c2's
  final wire format.
- Removed the string-count hostile-law test. Live tests now mutate real admission/restore owners,
  compare exact pointer identity on MAX+1 rejection/retry, gate every declared job substage before
  mutation for cancellation/deadline, recover direct Drop and panic owners, bound close, and compare
  1/2/4/default chronology.

- Added a checked, observed-capacity numerical census and independent maxima for model topology,
  weather/timesteps, result samples/history/rows, identifiers, items, bytes, pages, operations, and
  process jobs.
- Admission rejects before numerical construction and returns the exact owned `Model`,
  `SimulationConfig`, and `Operation`; retry consumes that same authority.
- Replaced whole validation with retained reserve/index/reference-check stages.
- Retained stable zone/surface/fenestration order and model ID indexes during precompute.
- Replaced surface precompute with retained area, normal, material, and publication substages.
- Replaced timestep `collect`/`sort` construction with an admitted reserve/fill cursor over the
  precomputed stable order.
- Replaced the zone turn with daylight/config/area/evaluation, people, lighting, equipment,
  infiltration, airflow node/link/solve, mechanical ventilation, thermostat, humidistat, and
  publication substages.
- Replaced the system turn with prediction, ideal-load, fault, application, equipment, and
  completion substages.
- Replaced plant and battery whole reductions with retained zone-load, priority, dispatch,
  simulation, and battery-reduction cursors.
- Replaced warmup whole-map convergence with retained temperature check, load check, temperature
  history, and load history cursors.
- Replaced final numerical reductions with meter, floor-area, history, tariff, LCCA, summary,
  environmental, resilience, and metadata substages.
- Replaced hidden schedule lookup scans with a retained constant/annual/holiday/rule/daily/weekly/
  time-series lookup cursor shared by every scheduled component family.
- Added retained, observed-capacity result-backing admission. Zone/facility names are reserved and
  copied into map-key, record-key, and order owners one Unicode character per turn; series samples,
  meters, history, and summary rows cannot insert beyond the observed admitted backing.
- Added retained reserve stages for validation, precompute, simulation-state, and result maps and
  vectors before their first insertion.
- Added field-wise close for partial `SizingBuilder` rows and design-day names. Result map keys move
  into a retained scratch owner and retire one Unicode character per close grant.
- Added a fixed 64-slot generation-qualified Energy abandonment registry. Ordinary direct drop,
  panic unwind, and partial-close session loss transfer the exact authority without allocation;
  bounded recovery is exact-operation and single-owner.
- Extended explicit close across timestep/precompute/state/input/config plus preview, time-series,
  meter, result, summary, and sizing result owners at one record or Unicode character per visible
  close grant.
- Preserved `Engine::run` as the headless adapter over `EnergyJob`.
- Added exact MAX, every independent comparator MAX+1, pointer-preserving rejected-owner retry,
  1/2/4/default grant chronology, bounded close, cancellation/freshness, direct-drop/panic recovery,
  exhaustive declared-substage cancel/deadline gates, restore, and hostile-law source fixtures.

## Files

- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📏️sizing/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🚦️dispatch/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📤️output/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧮️meters/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/💰️economics/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️component.rs`
- `✏️s/🔌️plugins/🔋️energy/🪨️tests/p7c1-energy-numerical-laws.json`

## Source Evidence

- Scoped `rustfmt --edition 2021` completed.
- Scoped `rustfmt --edition 2021 --check` completed with no output.
- Scoped `git diff --check` completed with no output.
- Production census found no `ObservedTable`, `extract_if`, `HashMap`, public production whole-run
  numerical adapter, public production model/output whole query, retained growable map authority in
  validation/precompute/state/results/meters, dispatch `collect`/`sort`, sizing `format!`/surface
  collection, whole zone-preview loop, production checkpoint restore/emission, or
  `serde_json::to_vec` in a production mounted step. Remaining `HashSet`, `collect`, formatting, and
  checkpoint JSON occurrences are `cfg(test)` hostile/reference helpers.
- The fourth narrow census found no production `weather: Vec<WeatherRecord>`, `weather: Vec::new()`,
  `self.weather.push`, or production call to the `cfg(test)` `TimeSeries::push`. It found the two
  weather authorities (checkpoint fixture and live job) as fixed tables, the pre-mount exact weather
  admission, the non-test admitted append definition and production call, and the retained typed
  weather/time-series fault fields.
- Scoped `git diff --stat` after the fourth RED remediation reports 5,363 insertions and 927
  deletions across ten Energy Rust sources, excluding the existing fixture/report entries.

These are source-only parse/format/census statements. No compile, runtime, performance, mutation,
or platform pass is claimed.

## Final Self-Census

The original six self-census blockers and both subsequent independent Terra RED findings are
repaired in source: legacy whole production entry points are isolated; exact input owners cross
admission without cloning; checkpoint restore/publication are consistently test-only; mounted maps
and unstable close scans are retired in favor of private fixed slots; sizing and dispatch use
retained microcursors and typed retained faults; scalar preview and provisional output use retained
page authority; and live nested authority gates replace taxonomy counts.

No additional P7c1 source blocker was found in the final scoped census. The packet remains **not
accepted** until an independent Terra audit confirms the diff and the serialized build owner later
establishes compilation, runtime, allocation, mutation, parity, watchdog, and platform evidence.

## Quiescent Verifier Additions Needed

The next serialized verifier should add or run focused laws that:

- compile debug/release with strict warnings through the Bun/Nx repository route;
- expand faithful real-input MAX+1 mutations from the live zone/restore authority law to every
  heterogeneous model/config family (the independent comparator remains exhaustive);
- pause/cancel/panic at every retained enum value and verify identical state plus bounded close;
- pressure schedule rules, aggregate names, maps, samples, tariffs, LCCA, and sizing rows beyond the
  8 ms watchdog threshold;
- prove no allocation occurs after the corresponding result backing admission;
- measure one-fuel and default-grant parity at worker counts 1, 2, 4, and default after P7c3 mount;
- verify exact credit return and no duplicate retirement during panic/session-drop close recovery;
- run native, `wasm32-unknown-unknown`, and `wasm32-wasip2` focused gates and Energy reference parity.

P7c2 checkpoint/publication authority, P7c3 mounted WorkerPool ownership, P7b, and the final Phase 7
matrix remain separate packets.
