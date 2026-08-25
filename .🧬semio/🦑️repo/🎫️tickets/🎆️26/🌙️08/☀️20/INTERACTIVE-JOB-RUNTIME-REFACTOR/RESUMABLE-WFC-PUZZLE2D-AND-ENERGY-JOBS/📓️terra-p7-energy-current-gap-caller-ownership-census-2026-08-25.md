# Phase 7 Energy Current Gap, Caller, Ownership, and Test Census

Date: 2026-08-25  
Method: read-only source census against the approved master plan and the P7b/P7c1/P7c2/P7c3 contracts. No source edits or Cargo, Nx, Wasm, browser, or runtime command was run.

## Verdict

**P7 Energy is RED.** The 2026-08-21 report documents useful prior focused-engine evidence, but the current source still contains every blocker that P7c1, P7c2, and P7c3 were created to repair. `EnergyJob` is a test/batch-only persistent-job facade, not a bounded, admitted, retained, process-worker operation. It therefore cannot meet the master-plan Phase 7 gate.

P7b is not reassessed here beyond its contract boundary: its existing source/static report is historical evidence only. Its independently specified mounted retained-session repair and the final Phase 7 executable matrix remain separate blockers.

## Current Production Caller Census

| Question | Exact current result | Evidence |
| --- | --- | --- |
| Production `EnergyJob` constructor | Only `Engine::job` constructs it, with `RevisionId(0)` and `Generation(0)`. | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs:905-909` |
| Mounted caller/factory/session | None. An `rg --glob '*.rs' --glob '!**/tests/**' 'EnergyJob::(new|from_checkpoint)\|Engine::(job|run)' ✏️s` census found no product action, process registry, worker session, or preview consumer. | Same file; all remaining external call hits are unit tests at `:1076-1208`. |
| Product command/action surface | Energy action directories are `📌️empty.md`; the plugin has no simulation action. | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/.../🎬️actions/📌️empty.md` for editor/viewer structure and zones. |
| Batch route | `Engine::run` calls `run_to_completion` on the same job; keeping this as a headless adapter is consistent with the plan. | `.../🧪️sim/🦀️component.rs:911-925` |
| Worker/UI ownership | No `WorkerPool`/`WorkerJobSession` match exists in the Energy plugin source census. There is no live document revision, operation slot, or close hook. | Energy plugin `rg` census, plus source above. |

The direct implications are P7c3 blockers, not mere missing UI polish: there is no action-to-admission-to-P2a1 spawn route, no live freshness guard, no immutable view, no lossless consumer, and no document/window/application close drain.

## P7c1 Numerical and Admission Census

### Present foundations

- `EnergyJob` does implement `InteractiveJob`, checks operation/generation, cancellation, deadline/fuel, and advances weather/precompute/zone/surface initialization one outer record at a time (`.../🧪️sim/🦀️component.rs:529-608`).
- `PrecomputeBuilder` is an existing one-record cursor for zones, surfaces, normalization, thermostats, and fenestrations (`.../🧠️precompute/🦀️component.rs:99-244`).
- Surface, fenestration, PV, service-hot-water, refrigeration, and water traversal each have an outer cursor (`.../🌰️kernel/🦀️component.rs:268-338`, `:592-649`).
- Output samples are partly cursorized (`.../🧪️sim/🦀️component.rs:455-525`). These are foundations, not P7c1 acceptance.

### Still missing or unsafe

1. **No numerical admission envelope.** `EnergyJob::new` takes complete dynamic `Model` and `SimulationConfig` by value, creates ordinary dynamic collections, and has no checked observed-capacity/item/page/op/process census (`.../🧪️sim/🦀️component.rs:161-205`). No MAX/MAX+1 rejected-owner retry authority exists.
2. **Validation remains whole-model.** `Validate` calls `self.model.validate()` and consumes its diagnostics in a single granted turn (`:542-545`).
3. **Timestep construction is whole `collect` + `sort` every timestep.** `TimestepWork::new` collects and sorts the complete surface and fenestration key sets before work can yield (`.../🌰️kernel/🦀️component.rs:225-257`). It is invoked synchronously by both warmup and run stage (`.../🧪️sim/🦀️component.rs:619-625`, `:662-676`). Even the otherwise useful precompute builder has a hidden per-surface material `collect` (`.../🧠️precompute/🦀️component.rs:148-184`), so P7c1 must audit its helpers rather than merely retaining the outer cursor.
4. **Zone work is monolithic.** One `prepare_zone` call scans daylight/fenestration, people, lighting, equipment, infiltration, airflow nodes/links/solve, mechanical ventilation, thermostat, and humidistat, creating dynamic `Vec`/network backing en route (`.../🌰️kernel/🦀️component.rs:340-450`).
5. **System substep is monolithic.** One turn performs prediction/advance, an unbounded ideal-load loop with a nested fault scan, and an unbounded equipment loop (`.../🌰️kernel/🦀️component.rs:452-560`).
6. **Secondary systems hide reductions/builds.** Plant collects dispatch priorities and reduces all zones in one turn (`:575-590`); `Dispatcher::dispatch` then clones and sorts that set (`.../🚦️dispatch/🦀️component.rs:59-89`). Battery reduces all zones in one turn (`.../🌰️kernel/🦀️component.rs:606-619`).
7. **Warmup convergence is whole-collection.** Two `.all()` scans followed by two full `HashMap` rewrites execute after a timestep (`.../🧪️sim/🦀️component.rs:610-649`).
8. **Aggregation/finalization are dynamic or whole-collection.** Per-zone aggregation constructs four strings and can grow maps/series (`:377-398`); facility adds names/maps (`:400-408`); summaries/metrics/economics reduce full tables/history and construct rows (`:410-433`); result construction transfers dynamic graphs and clones metadata (`:435-453`).
9. **Dynamic `HashMap` authorities and close are not exact.** Surface and timestep aggregate maps use ordinary `HashMap` insertion (`.../🌰️kernel/🦀️component.rs:218-220`, `:318-320`). The Energy close path pops a few vectors/entries but then drops whole `precompute`, `pre`, `state`, `timestep_work`, `result`, and builder graphs in a single grant (`.../🧪️sim/🦀️component.rs:764-838`). This violates one-owner/page close and cannot give exact backing-credit evidence.

## P7c2 Checkpoint, Publication, and Terminal Census

1. **Whole serde/JSON checkpoint.** `from_checkpoint` strips a header and deserializes the full graph before identity validation (`.../🧪️sim/🦀️component.rs:219-266`). `encode_state` clones every retained graph and `serde_json::to_vec`s it in one call (`:327-370`). The source imports `serde` at `:19`; no page arena, binary schema, retained builder/decoder, cap, or rejected-owner API exists.
2. **Unbounded preview.** `publish_preview` collects/sorts every zone and allocates three full-zone vectors; `encode_preview` then allocates an unrestricted byte vector, and only temperatures are encoded despite typed heating/cooling fields (`:287-321`, `:878-892`). There is no fixed latest-wins slot, replacement-retirement cursor, or byte authority.
3. **No bounded lossless queues.** Checkpoint and terminal outputs are directly returned as freshly allocated `Vec<u8>` in `StepOutcome` (`:323-325`, `:756`), with no queue capacity, `take`, retry/resume, or retained full-queue packet.
4. **Terminal is still computational.** `Complete` repeats full checkpoint encoding and transfers `commit_output` (`:756`), rather than atomically exposing an already-prepared page-backed commit.
5. **Output itself remains partially unbounded.** Header paths append three dynamic metadata strings in one grant and later append `format!(\"{:?}\", ...)` strings while growing `commit_output` (`:455-525`).

## P7c3 Mounted-Product Census

All P7c3 requirements are absent, not just unverified:

- no schema-owned start/cancel/retry/discard/adopt event or localized English/German action label;
- no fixed generation-tagged Energy slot/retirement arena or P2a1 process registration;
- no retained snapshot-to-model/config admission cursor;
- no nonblocking reconcile/spawn or process WorkerPool factory;
- no immutable, try-only latest projection; no checkpoint/commit consumer;
- no revision/config/operation/sequence/tier monotonicity validation before install/consume;
- no provisional four-tier progress window, keyboard/screen-reader busy/cancel state, or accessible fault/result path;
- no close-step coverage for pending admission, current job, queues, results, faults, or lost-handle/panic recovery.

## Ownership Inventory To Replace

| Current owner | Current retirement | Required destination |
| --- | --- | --- |
| `Model`, `SimulationConfig`, weather and precompute | Whole ordinary ownership; job `Drop`/`take` paths | P7c1 observed-capacity census and fixed/page-backed input/admission authority |
| Timestep vectors/maps/work and warmup maps | Per-timestep `Vec`/`HashMap`; whole `timestep_work.take()` | P7c1 stable precomputed order plus explicit construction/zone/system/warmup close cursors |
| Preview vectors and bytes | `last_preview: Option`, freshly returned byte `Vec` | P7c2 fixed latest-wins packet slot plus bounded replacement retirement |
| Checkpoint/restored bytes | Whole serde clone/decode | P7c2 versioned binary pages, retained encoder/decoder, rejected exact input/retry authority |
| Checkpoint/commit/fault publication | Direct `StepOutcome` allocation | P7c2 fixed lossless queues and generation-tagged terminal state |
| Result/output metadata | `Results`, dynamic strings, growing `commit_output` | P7c1 record builders followed by P7c2 pre-admitted prepared commit packet |
| Job/session/window state | None in production | P7c3 P2a1 slot, cancel authority, immutable view, close/retirement arena |

## Bounded Implementation Packets

These packets must wait for P2c/P3mn source quiescence and must remain serialized **within Energy** because they touch the same protocol/engine seam. P7c3 may prepare read-only schema/UI mapping in advance but cannot merge before P7c1/P7c2 acceptance.

1. **P7c1 — numerical microcursor and admission (Sol High).**
   - Own: `.../⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs`, `.../🌰️kernel/🦀️component.rs`, `.../🧠️precompute/🦀️component.rs`, plus Energy-owned permanent fixtures.
   - Replace whole validation, construction order, zone/system/plant/battery/warmup/finalization paths with retained one-unit cursor substages; precompute stable order once; fixed/page or observed-capacity numerical envelope and exact rejected retry owner; one semantic-owner/page close.
   - Required tests/mutations: every independent MAX/MAX+1, fuel-one boundary for all substages/close, cancel/generation/panic close, high-cardinality components and chronology at 1/2/4/default.

2. **P7c2 — retained wire, channels, and terminal (Sol High; after P7c1 Terra acceptance).**
   - Own: Energy simulation wire/schema and `.../🧪️sim/🦀️component.rs`; add only schema-first Energy-owned files if the taxonomy needs them.
   - Replace serde/JSON checkpoint/restore, all-zone preview, direct outcomes, and terminal encode with versioned fixed-page build/restore, deterministic bounded projection, latest-wins preview retirement, fixed lossless checkpoint/commit queues, and prepared exact-once commit.
   - Required tests/mutations: schema rejection matrix, MAX/MAX+1 by item/page/op/aggregate, one fragment/page grant, saturation exact identity/order, stale slot reuse, cancel/panic/drop close requeue, restored-byte parity and 1/2/4/default ordering.

3. **P7c3 — mounted Energy operation (Sol High; after P2a1/P7c1/P7c2 Terra acceptance).**
   - Own: Energy artifact command/schema/registration/editor/viewer/transient/action/window files and a dedicated product session file; do not reopen P7c1/P7c2 engine ownership except through their public interface.
   - Register one process job kind and fixed P2a1 arena; add event-sourced start/cancel/retry/discard/adopt semantics; staged snapshot/model admission; worker-only stepping; immutable projection view; English/German accessible four-tier window; freshness/tier checks; bounded close/drain.
   - Required tests/mutations: no direct UI `step`/whole materialization, process-slot MAX/MAX+1 exact retry, spawn only after admission, worker count replay, stale/queue-full/cancel/close/lost-handle cases, localized provisional/accessibility and final single-transfer.

4. **Phase-7 matrix owner (serialized, after P7b and P7c source acceptance).**
   - Run the final immutable-tree repository gates only then: focused/product debug/release, strict warnings, WorkerPool 1/2/4/default replay, native interaction console evidence, both Wasm targets, timing/cadence, cap/saturation/panic/stuck/close, and Energy tolerance parity. None was run for this census.

## Dispatch Blockers

- P7c1 is immediately necessary and source-disjoint from active P2c/P3mn only after those packets quiesce.
- P7c2 is blocked by P7c1 source acceptance; P7c3 is blocked by P2a1 plus P7c1/P7c2 source acceptance.
- Phase 7 cannot close until P7b's mounted repair and the final serialized executable matrix also pass.
