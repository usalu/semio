# Terra P7c1 Energy Numerical Microcursor Remediation Re-audit — 2026-08-25

## Verdict

**RED — the remediated tree closes several prior structural defects, but it does not yet meet the P7c1 live bounded-work contract.** This is a source/static audit of the exact current tree. It did not run Cargo, Nx, Wasm, browser, build, or runtime/parity work.

This report is new and preserves the earlier RED audit at `📓️terra-p7c1-energy-numerical-microcursor-independent-audit-2026-08-25.md`.

## Scope and method

Read in full: the P7 master plan, P7c1 contract, earlier Terra RED/gap census, the updated Sol implementation report, the current simulation/kernel/precompute/sizing/dispatch/model/output/meters/results sources, the glue export surface, and `🪨️tests/p7c1-energy-numerical-laws.json`. I followed production callees rather than names and used only the requested scoped static gates.

## Repairs that genuinely hold

- The public `Engine::job` and batch `Engine::run` consume `Model` and `SimulationConfig` by value at `⚙️engine/🧪️sim/🦀️component.rs:3013-3028`; the interactive engine no longer clones either input. The run-to-completion loop is confined to the explicit batch adapter.
- The former whole-operation helpers are test-gated: precompute build (`🧠️precompute:85`), kernel initialize/warmup/advance (`🌰️kernel:511-536,1352-1404`), sizing `size` (`📏️sizing:286`), and dispatch `dispatch` (`🚦️dispatch:58`).
- The normal job does not emit a checkpoint. `encode_state` is `#[cfg(test)]` at `🧪️sim:1154`; the normal preview path at `1126-1147` builds the scalar/empty-vector P7c1 preview and sends a fixed 42-byte encoding. This is an appropriate P7c2 deferral for checkpoint **emission**.
- The generated 64-slot operation/generation abandonment registry remains fixed, and the true `Drop` and panic recovery tests take/recover the exact authority once (`🧪️sim:3572-3611`). Close paths now use cursor/tail disposal rather than `extract_if`.
- The direct 19-dimensional MAX/MAX+1 source fixture is no longer merely a string assertion: `p7c1_live_owner_capacity_mutations_reject_the_declared_dimension` exercises `EnergyJob::admit` for zones, surfaces, fenestrations, people, lighting, equipment, infiltrations, mechanical ventilations, thermostats, humidistats, ideal loads, faults, zone equipment, plant loops, PV, battery, SHW, refrigeration, and water (`🧪️sim:3472-3507`). The checkpoint rejection/pointer retry and 1/2/4/default chronology tests are also materially present. They were not executed under this audit's no-Cargo constraint.
- Normal success-path sizing and dispatch have fine-grained cursors: one surface/vertex/name character/result or one dispatcher equipment per `step` (`📏️sizing:130-208`, `🚦️dispatch:108-159`).

## Blocking counterexamples

### 1. The supposed fixed table is a repeat-growable, linear-scan `Vec`

`ObservedTable<K,V>` is a public `Vec<(K,V)>` at `⚙️engine/🔋️model/🦀️component.rs:22-25`. Its public `try_reserve` accepts a fresh amount every call, invokes `Vec::try_reserve_exact`, and adds that amount to `observed_capacity` (`34-38`). Thus a holder can repeatedly increase the table's authority; it is not an immutable observed-capacity admission.

More importantly, `insert`, `get`, `get_mut`, and `entry` perform `iter().position`, `iter().find`, `iter_mut().find`, and another `iter().position` (`41`, `52`, `56`, `64`). Validation, simulation state, precompute tables, time series, and meters call those live lookup paths. Replacing `HashMap` with an insertion-ordered vector has removed the hash map but introduced hidden unbounded scans in the per-grant authority. This falsifies the fixed/page/observed-capacity/stable-cursor requirement.

### 2. Production-reachable whole APIs remain public and exported

The crate glue exposes all model symbols through `📦️packages/🦀️rust/📦️glue.rs:98,171`. Therefore `Model::validate` is production reachable, not an internal test helper. It builds four `HashSet`s with `collect`, builds another `HashSet`, scans the full model, clones names, and formats diagnostics at `⚙️engine/🔋️model/🦀️component.rs:702-823`. `Model::surfaces_for_zone` is likewise public and allocates a full collected vector at `838-840`.

These are precisely former whole-validation/query authority that P7c1 required to be test-only or hard UI-forbidden/unreachable. No such guard exists.

### 3. `from_checkpoint` rejects only a subset after a whole JSON decode

`EnergyJob::from_checkpoint` first confirms model/config census and the byte ceiling, then calls whole `serde_json::from_slice::<EnergyCheckpointState>` (`🧪️sim:1001-1018`). Its following restoration checks cover weather, root time-series/meter entry counts, selected history/sample/summary lengths, and output bytes only (`1019-1035`). It does not apply same-bounds verification to, among other live owned data:

- validation index tables;
- precomputed observed tables/orders;
- `SimulationState` zone/surface tables;
- `Results`, sizing builder/tables, and aggregate buffers;
- `last_preview`'s public vectors;
- ordering and intermediate per-stage cursors/strings not covered by the selected dimensions.

A byte-valid checkpoint can therefore mount oversized nested storage before rejection (or without rejection) even though normal production emission is forbidden. The rejection closure does preserve the exact `Model`/`SimulationConfig`, and the source pointer-retry test is good evidence for that half; the same-bound requirement is nevertheless false.

### 4. Preview is only conditionally bounded

The normal publisher is scalar, but `EnergyJobPreview` still exposes dynamic `Vec` fields (`🧪️sim:63-75`), and `from_checkpoint` installs `checkpoint.last_preview` without a vector-length bound (`1045+`). The restore counterexample above admits a dynamic preview through the public decode surface. P7c1 cannot call the preview fixed/bounded until restore validates or excludes it.

### 5. Allocation rejection silently converts output, sizing, and dispatch into success

`encode_output_step` returns `true` when its one-time output reserve is rejected, without storing an error or fault (`🧪️sim:1695-1701`). Its caller consequently proceeds toward an empty retained output. That is a silent fallback, not bounded refusal.

Sizing does the same: reserve failure sets `SizingStage::Complete` at `📏️sizing:117-126`; dynamic name reserve does likewise at `213-219`. The caller cannot distinguish an allocation failure from completed sizing. `DispatchBuilder::step` repeats the pattern at `🚦️dispatch:101-106`. The normal success cursor granularity cannot compensate for silent failed-allocation completion.

### 6. The every-substage interruption evidence is a taxonomy count, not execution

The named test only assigns each outer `EnergyJobStage`, injects cancel/deadline, and compares pre/post encoded state (`🧪️sim:3615-3694`). At `3695-3697` it merely adds `#[cfg(test)]` kernel/precompute stage-array lengths. It never positions a real `ValidationWork`, warmup, precompute, timestep, zone/system/plant/schedule cursor at every enumerated substage and injects cancel/deadline/stale there. The JSON laws fixture parses, but it is declarative and the in-source proof remains a count/sub-string-style coverage surrogate rather than the requested real authority mutation.

## Required remediation direction

1. Replace `ObservedTable` with a one-time admitted fixed/page owner and deterministic ID-to-slot lookup that neither scans nor grants additional capacity after admission. Make overflow an explicit rejected/fault state.
2. Remove, test-gate, or make inaccessible all remaining whole public `Model`/output/meter queries; in particular eliminate `Model::validate`'s hash/collect/format path from production authority.
3. Either hard-disable `from_checkpoint` pending P7c2 or validate every nested resident field against the exact observed census before installing it, with incremental/cancellable decoding instead of whole JSON materialization. Bound or remove `last_preview` vectors.
4. Propagate allocation refusal through a typed job fault/rejection from output/sizing/dispatch—never `Complete` or `true` as a successful fallback.
5. Turn the declared substage arrays into actual cancellation/deadline/stale injections at every live nested cursor, and make the law fixture executable rather than documentary.

## Static gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Scoped edition-2021 `rustfmt --check` | GREEN | sim, kernel, precompute, sizing, dispatch, model, output, meters, results all exit 0. |
| Scoped `git diff --check` | GREEN | Same live source set exits 0. |
| P7c1 law JSON syntax/schema-key parse | GREEN | `bun -e` parsed the fixture and found `schema`, `admission`, `step`, and `terminal`. |
| Source census | RED | Live `ObservedTable` scans/re-reservation; public `HashSet`/`collect`/`format!`; silent reserve branches cited above. |
| 19 heterogeneous MAX+1 mutations / checkpoint retry / Drop-panic / chronology | STATIC-PRESENT, NOT RUN | The actual source tests are present, but Cargo/test execution was prohibited. |

The first attempt to assume a top-level `mutations` field in the declarative fixture failed because the fixture instead has the four documented sections above; that was corrected to a syntax/key check and is not presented as a mutation execution result.

## Deferred gates

Compiler, unit/integration runtime, actual mutation execution, determinism/parity, retained-page transport, native/browser host delivery, P7c2 checkpoint protocol, and P7c3 finalization remain deferred by scope. Their deferral does not clear the source-level RED blockers above.
