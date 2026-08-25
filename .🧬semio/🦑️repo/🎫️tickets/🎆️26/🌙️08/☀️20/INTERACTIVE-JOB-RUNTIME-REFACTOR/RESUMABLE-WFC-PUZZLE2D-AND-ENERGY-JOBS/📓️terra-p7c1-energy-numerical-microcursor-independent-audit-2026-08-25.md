# P7c1 Energy Numerical Microcursor Independent Terra Audit — 2026-08-25

## Verdict

**RED**

The current tree adds a real cursored EnergyJob path, but P7c1's boundedness and
admission contract is not closed. Public production entry points still execute
whole operations; live authorities use dynamic HashMaps; preview/checkpoint/output
perform whole scans, clones, formatting, and unchecked growth; sizing and dispatch
retain their former whole work. The purported hostile-mutation evidence is
test-only counting/string evidence, rather than executable mutations of live
authorities.

## Scope and method

Read in full:

- the master plan;
- the P7c1 repair contract;
- the earlier Terra gap census;
- Sol's P7c1 source report;
- the Energy plugin instructions;
- live sim, kernel, precompute, sizing, dispatch, model, output/result ownership,
  and P7c1 fixture source.

This is a read-only audit of the exact shared tree. No Cargo, Nx, Wasm, browser,
build, or runtime simulation was run.

## Live counterexamples

### 1. Direct production APIs still run whole operations

The public API surface bypasses EnergyJob rather than making every batch path an
adapter for that job:

- PrecomputedModel::build loops until a PrecomputeBuilder completes at
  ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️component.rs:86-91.
- TimestepWork::new contains an unbounded construction loop at
  ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️component.rs:511-520.
- SimulationKernel::initialize iterates all zones and every HashMap surface at
  kernel/component.rs:1350-1361; warmup performs hour, state-map, and all()
  scans at lines 1362-1395; advance_timestep then loops until completion at
  lines 1398-1404.
- SizingManager::size loops the builder to completion at
  ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📏️sizing/🦀️component.rs:135-142.
- Dispatcher::dispatch clones, sorts, and returns collected vectors at
  ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🚦️dispatch/🦀️component.rs:58-91.

These are public production bodies, not test helpers. They falsify the master
rule that an interactive operation is a persistent, cancellable, bounded state
machine and P7c1's requirement to eliminate the formerly whole
precompute/timestep/warmup/sizing/dispatch paths.

### 2. Dynamic HashMap authorities remain in the live job

The P7c1 contract forbids dynamic HashMap authority. It remains in every central
job owner:

- ValidationWork owns four HashMaps at sim/component.rs:545-553. Its reserve
  stages merely call HashMap::try_reserve and later inserts/contains lookups
  at lines 1144-1310; this is not a fixed/page-backed ID table.
- SimulationModel owns dynamic zones and surfaces HashMaps at
  kernel/component.rs:141-154. EnergyJob reserves them and mutates them in
  InitializeZones/InitializeSurfaces at sim/component.rs:1757-1789.
- PrecomputedModel exposes nine dynamic HashMaps at
  precompute/component.rs:65-80. The resumable builder inserts into them at
  lines 231-355; its close path uses extract_if at lines 105-130, whose next()
  need not have a bounded bucket scan or stable removal order.
- The active results path continues to use dynamic HashMap TimeSeriesTable and
  MeterTable through sim/component.rs:1457-1527 and 1535-1576.

The stable Vec order sidecars do not convert those maps into fixed-authority
storage, and map iteration/removal is still present in close and in the legacy
kernel APIs.

### 3. Preview, checkpoint, and output are whole and allocating

The job's own published paths violate the requested no-scan/no-growth/no-format/no-clone
rule:

- publish_preview allocates three fresh Vecs and scans every pre.zone_order
  record in one EnergyJob::step at sim/component.rs:1043-1077. It neither
  reserves nor carries a preview cursor. encode_preview repeats a whole
  zone-temperature loop at lines 2330-2345.
- encode_state clones weather, validation, precompute, pre/state maps,
  histories, results, sizing, summaries, and commit bytes before serializing
  all of it with serde_json at lines 1084-1137. This is a whole checkpoint
  copy, not one admitted copy/page unit.
- encode_output_step appends metadata strings in one stage, constructs
  format! strings, and grows commit_output without a capacity authority at
  lines 1622-1689.

These paths are reachable from the interactive EnergyJob: previews are returned
from ResolveWeather, InitializeSurfaces, WarmupConvergence, PublishTimestep,
and BuildResults; checkpoints are returned during warmup and publication.
P7c2 may own a later publication/checkpoint redesign, but this current P7c1
tree cannot satisfy the requested preview and result/fragment boundedness while
these bodies remain.

There is also a static type-consistency warning that was not compiler-checked:
EnergyJobPreview declares no run_backing_stage at sim/component.rs:64-75, yet
publish_preview initializes that field at line 1069. EnergyCheckpointState
requires run_backing_stage at lines 693-744, but encode_state does not provide
it at lines 1084-1134. Rustfmt parses this source but does not type-check it.

### 4. Admission is bypassable and the batch constructor copies the owner

EnergyJob::admit correctly observes then checks a census before accepting its
moved Model and SimulationConfig (sim/component.rs:861-934). Two reachable
entry points defeat the full requirement:

- from_checkpoint only observes the census and claims a slot; it never calls
  first_exceeded or takes an EnergyNumericalBounds at
  sim/component.rs:954-1023. A restored MAX+1 Model/Config therefore bypasses
  the normal exact admission rejection.
- Engine::job clones its complete borrowed Model and SimulationConfig before
  admission at lines 2865-2869. That is an unmetered whole owner copy, not the
  exact moved Model/Config allocation whose identity can be returned on
  rejection.

The fixed 64-slot abandonment registry and equality check on operation plus
generation (sim/component.rs:807-860 and 935-940) are a positive partial
repair. They do not cure the bypass/copy paths above.

### 5. Sizing still scans, collects, formats, and emits multiple owners per step

SizingBuilder::step calls Model::surfaces_for_zone, which creates a Vec through
filter(...).collect() in model/component.rs:717-719, then computes all areas in
the same step. It formats and pushes both heating and cooling SizingResult
owners in that one call at sizing/component.rs:73-81. It has no P7c1 admission
reservation for either sizing result vector or its component strings. The
equipment branch also formats/pushes dynamically at lines 83-88.

This falsifies the specific sizing, allocation, copy, and one-semantic-owner
requirements even when EnergyJob reaches its Size stage.

### 6. Close is partially cursored but not fixed or stable

The close path usefully releases Vec/string contents one owner/character at a
time in many branches, including preview, sizing, result, and model/config
fields (sim/component.rs:2116-2288 and 2454-2820). It nevertheless delegates
map retirement to extract_if in validation, time series, meters, precompute,
and summaries. Those dynamic maps have no fixed ID/page cursor and do not
provide the required stable one-owner close authority. This is not cured by a
one-item response counter.

## Required-path assessment

| Contract area | Source result |
| --- | --- |
| Validation/precompute cursors | Partial: cursors exist, but live direct whole APIs and dynamic maps remain. |
| Zone/system/plant/battery/warmup/finalization cursors | Partial: named cursors exist in kernel and finalization; legacy full kernel paths remain reachable. |
| Stable order | Partial only: sidecar order vectors help main output, but HashMap traversal/removal and direct dispatcher paths remain. |
| Exact observed Model/Config MAX and MAX+1 | RED: main moved admission is partial; checkpoint bypasses bounds and Engine::job clones the owner. |
| No collect/sort/scan/growth/format/clone/dynamic maps | RED: direct counterexamples above. |
| Fuel before work; cancel/deadline/stale | Outer EnergyJob checks identity/cancel/yield then consumes one fuel unit at sim/component.rs:1697-1708, but entire reachable preview/checkpoint/output/legacy calls make the every-substage conclusion false. |
| Fixed generation-qualified Drop/panic/session recovery | Partial structural evidence: fixed 64 slots and qualified recovery exist; executable saturation/panic/drop evidence was not run. |
| Batch adapter uses same job | Partial: Engine::run drives EnergyJob, but clones input before admission and legacy public APIs retain different run-to-completion paths. |

## P7c1 law and mutation evidence

The fixture itself is valid JSON, but it is not an executable adversarial gate:

- p7c1-energy-numerical-laws.json is declarative text. Its listed dimensions
  are incomplete relative to the live EnergyNumericalCensus (for example it
  says components rather than the independent live people/lighting/equipment,
  etc.).
- P7C1_HOSTILE_MUTATION_LAWS is an array of 26 nonempty strings under
  cfg(test) at sim/component.rs:3005-3035. The associated test only checks
  length and non-emptiness at lines 3498-3501; it applies none of those
  mutations to source or a live authority.
- The claimed every-max-plus-one test creates a synthetic census and calls
  first_exceeded, not EnergyJob::admit on a MAX+1 Model/Config owner, at
  lines 3247-3328. The earlier exact-owner test covers only zones.
- Kernel/precompute stage arrays are cfg(test) declarations
  (kernel/component.rs:233-288; precompute/component.rs:153-168). The
  cancellation test counts those values but does not drive each nested cursor
  with cancel/deadline/stale state (sim/component.rs:3405-3495).
- The language-agnostic test is also cfg(test); it parses the included JSON and
  extracts its own schema substring at lines 3504-3512. It does not validate
  executable behavior or hostile mutations.

Consequently, P7c1 baseline, all required faithful mutations, and the
1/2/4/default chronology are **not GREEN evidence** in this audit. No Cargo
test was run by scope restriction.

## Allowed static gates run

| Gate | Result |
| --- | --- |
| edition-2021 rustfmt --check on sim, kernel, precompute, sizing, dispatch | PASS |
| Bun JSON.parse of the owned p7c1-energy-numerical-laws.json fixture | PASS |
| Scoped git diff --check for those sources and fixture | PASS |
| Live source census for HashMap, loop, collect/sort, format!, clone | RED, with the production occurrences cited above |

Rustfmt is a parse/format check only; it neither type-checks the two static
field inconsistencies noted above nor demonstrates runtime, determinism, or
numerical parity.

## Deferred gates

Deferred by the audit scope, not treated as passed:

- Cargo/compiler/type checking and the Energy unit suite;
- runtime watchdog, cancellation/deadline/stale injection at every nested
  cursor, Drop/panic/session-loss and registry-saturation behavior;
- 1/2/4/default worker chronology and batch parity;
- Energy reference/numerical parity;
- P7c2 streaming checkpoint/publication and P7c3 mounted-session host
  integration.

Production source must first remove the RED counterexamples before those
runtime gates can establish a GREEN P7c1 result.
