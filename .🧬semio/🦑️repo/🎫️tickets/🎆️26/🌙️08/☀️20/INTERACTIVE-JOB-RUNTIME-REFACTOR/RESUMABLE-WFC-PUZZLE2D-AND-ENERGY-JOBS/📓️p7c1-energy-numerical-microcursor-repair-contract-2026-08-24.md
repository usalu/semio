# P7c1 Energy Numerical Microcursor Repair Contract

Date: 2026-08-24  
Owner: next Sol High source executor  
Status: prepared, not accepted  

## Packet Boundary

This packet repairs the numerical and model-traversal half of P7c. It must leave the current
`EnergyJob` and `TimestepWork` route with one semantically bounded unit of work per granted turn.
It does not claim P7c checkpoint/publication acceptance or mounted-product acceptance. P7c2 must
replace retained checkpoint, preview, commit, and restore graphs; P7c3 must mount the operation on
the process `WorkerPool` with revision/generation freshness and product close drain.

The implementation must preserve `Engine::run` only as a headless adapter that drives this same
job. It must not add an Energy-owned pool, thread, scheduler, runtime dependency, permanent script,
legacy adapter, default locale, or compatibility route.

## Exact Current Blockers

### Admission and validation

- `EnergyJob::new` accepts complete dynamic `Model` and `SimulationConfig` graphs without an exact
  admitted item/page/byte envelope. The later P7c2 packet owns the retained byte authority, but
  P7c1 must expose an exact preflight numerical census and reject before numerical construction.
- `EnergyJobStage::Validate` calls whole-model `Model::validate()` in one turn and then consumes the
  complete diagnostics collection to find a fatal message.
- Weather target selection may expose an arbitrary input record count; every subsequent weather
  record must still be one admitted unit.

### Timestep construction

- `TimestepWork::new` collects every `pre.surfaces` key, sorts the complete vector, then repeats the
  operation for fenestrations in the turn that starts each warmup or run timestep.
- Solar/context construction is coupled to those whole collections, so there is no cancellable
  construction state between timestep admission and surface processing.
- The ordered surface and fenestration authority is reconstructed for every timestep instead of
  being built once, deterministically, during admitted precompute.

### Zone preparation and system substeps

`TimestepWork::prepare_zone` performs all of these operations in one turn for one zone:

- full fenestration-area reduction for daylight;
- complete people, lighting, and equipment scans;
- infiltration lookup and construction;
- airflow-network node collection, link construction, and complete network solve;
- full mechanical-ventilation reduction;
- thermostat and humidistat searches;
- dynamic node/link `Vec` construction.

`step_system_substep` then performs a complete ideal-load scan, a complete fault lookup for each
ideal load, and a complete zone-equipment scan in a single turn. One zone or one system substep is
not a work bound when it can contain arbitrary component counts.

### Secondary systems and warmup

- `step_plant` reduces all zones, collects every equipment priority, constructs a dispatcher, and
  dispatches the complete list in one plant turn.
- `step_battery` reduces all zone delivered loads in one battery turn.
- Daily warmup convergence performs two whole-zone `all` scans and then rewrites both complete
  previous-value maps in the same Energy turn.
- Standard `HashMap` insertion in timestep aggregate maps and warmup maps is not an exact backing
  model. Rehash or replacement work can occur inside a nominal single-record turn.

### Aggregation and finalization

- `aggregate_zone` formats four dynamic names and may create four dynamic map/series authorities in
  one turn; `aggregate_facility` may create two more.
- `finalize_summaries` performs meter reductions and a full floor-area reduction in one turn.
- `finalize_metrics` performs meter reductions and a full temperature-history resilience scan.
- `finalize_economics` performs tariff and life-cycle iterations plus dynamic row construction in
  one turn.
- `build_results` transfers several whole dynamic graphs and clones three dynamic metadata strings
  in one turn.

P7c2 owns the final bytes and channel authority, but P7c1 must leave each numerical reduction and
record construction resumable so P7c2 can wrap it without concealing monolithic work.

## Required Production Design

### One admitted numerical envelope

Add one P7c-owned schema-first bounds definition covering at least zones, surfaces, fenestrations,
people, lighting, equipment, infiltration, airflow nodes/links, mechanical ventilation,
thermostats, humidistats, ideal loads, faults, zone equipment, plant loops and equipment, PV,
batteries, water/refrigeration/hot-water systems, weather records, timesteps, meters, series,
samples, history values, summary rows, and dynamic identifier/name bytes.

Preflight must use actual observed backing capacity or fixed/page-backed authorities, checked
arithmetic, and one simultaneous-working-set model. Logical length, decorative token bytes, and
credits that are returned before the corresponding owner is retired are forbidden. MAX succeeds;
MAX+1 returns the exact original input graph with pointer/identity-preserving retry authority.

### Persistent construction stages

Replace `TimestepWork::new` with a persistent constructor. Prefer storing the stable ordered
surface/fenestration IDs once in `PrecomputedModel` while its admitted builder already visits each
record. If an order transformation remains necessary, use a persistent radix/bucket or other
fixed-work cursor with a bounded number of inspected/moved IDs per grant. Never call whole `sort`,
`sort_by*`, `collect`, or a hidden equivalent from a mounted turn.

The Energy job must retain the partially constructed timestep and expose cancellation between every
construction unit. Warmup and run must share this same constructor state and deterministic order.

### Zone microcursor

Replace `prepare_zone` with a substage enum and retained indices/accumulators. Each grant may inspect
or commit at most one admitted component, airflow node/link, or fixed numerical operation. The
minimum substages are daylight-area reduction, daylight evaluation, people, lighting, equipment,
infiltration, airflow-node build, airflow-link build, airflow solve micro-iteration, mechanical
ventilation, thermostat, humidistat, and zone-work publication.

Replace `step_system_substep` with retained ideal-load, fault, and zone-equipment cursors. The air
prediction/advance and each equipment simulation must be distinct bounded units. A fault lookup
must use a precomputed fixed/page index or its own cursor; nested full scans are forbidden.

### Secondary-system microcursors

Plant processing must retain a zone-load reduction cursor, equipment-priority build cursor,
dispatch cursor, and final loop simulation unit. Battery processing must retain its zone-load
reduction. Any library helper used by these stages must be audited down to its actual loop; wrapping
a whole helper call in a newly named stage does not satisfy this contract.

### Warmup and finalization microcursors

Daily warmup convergence must use separate retained temperature-check, load-check,
temperature-history update, and load-history update cursors. A convergence decision may publish
only after all cursors complete for the same day/generation.

Aggregation/finalization must retain separate cursors for meter totals, floor area, history
resilience, tariff periods, life-cycle years, summary rows, result sections, and metadata bytes.
Each dynamic name/row/result owner must be constructed through the admitted backing authority, one
bounded fragment per turn. `BuildResults` may only perform constant-time field handbacks after all
sub-builders are complete.

### Deterministic chronological semantics

Chronological simulation state is authoritative and single-writer. Parallel-ready zone/component
packets may be introduced only as immutable inputs plus generation-tagged indexed result slots;
their reduction must be deterministic and chronological. This packet must produce bit-identical
checkpointable numerical state for WorkerPool sizes 1, 2, 4, and default, except explicitly
documented floating-point tolerance fields at the final Energy parity gate.

All numerical substages must check cancellation and deadline before taking the next unit and after
returning from any helper whose cost is not statically constant. No turn may retry a partially
applied numerical mutation.

## Terminal and Close Requirements

Every partially constructed timestep, zone substage, secondary-system substage, warmup scan, and
finalization builder must have a persistent close cursor. Ordinary `Drop`, `clear`, `truncate`, map
replacement, generation replacement, panic unwinding, or session loss must not recursively retire
an admitted dynamic graph. Close must release one owned record/page at a time and return the exact
credit only after that owner is gone. A handle dropped during partial `Closing` must durably requeue
the same generation/cursor without resetting or duplicating retirement.

P7c1 is not accepted if these close states exist only in tests or if batch `Engine::run` is the only
caller that can finish them.

## Hostile Permanent Fixtures and Mutations

Add permanent source fixtures that prove:

1. exact MAX admission succeeds and every independent MAX+1 dimension rejects before transfer;
2. rejected input preserves the exact owned graph and can be retried after credit release;
3. one-fuel stepping advances every construction, component, warmup, finalization, and close
   substage without performing the adjacent record;
4. cancellation at every substage leaves no partially applied state and reaches bounded close;
5. large per-zone people/lighting/equipment/ideal-load/equipment-assignment sets cannot be hidden
   behind a single zone step;
6. large plant equipment, airflow node/link, meter, sample, history, tariff, and LCCA inputs cannot
   be hidden behind a single named helper;
7. ordered timestep construction is deterministic at 1/2/4/default workers and after restore;
8. panic/session-drop during each partial close stage reclaims every real backing exactly once;
9. generation replacement cannot mutate or publish an older chronological state;
10. no Energy-owned scheduler, pool, thread, blocking wait, unbounded channel, whole `collect`,
    whole `sort`, recursive close, or production terminal drain is reachable.

Mutations must remove or bypass each cursor advance, cancellation check, exact MAX+1 rejection,
generation check, deterministic ordering step, close requeue, and credit handback independently.
The focused mutation target must kill every mutation.

## Acceptance Evidence

Source acceptance requires an independent Terra read-only audit of the final diff plus exact caller
and retained-owner census. No broad Cargo/Nx/Wasm/browser command may run while overlapping Rust
packets are active. Once the tree is source-quiescent, the serialized build owner must capture:

- focused debug/release and strict-warning builds through Bun/Nx-owned repository commands;
- real process WorkerPool replay at 1, 2, 4, and default workers;
- maximum/MAX+1, cancellation, retry, panic, stuck-job, saturation, and close-drain evidence;
- allocation-pressure evidence and watchdog max/p99 for every new substage;
- first substantive preview under 50 ms, active cadence under 33 ms, and every mounted worker/UI
  turn below 8 ms;
- native plus `wasm32-unknown-unknown` and `wasm32-wasip2` gates;
- numerical parity with the Energy reference results and deterministic chronological output.

Passing P7c1 does not close Phase 7. P7c2, P7c3, P7b, and the final Phase 7 executable matrix remain
required.
