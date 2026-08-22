# Terra Audit — P0–P2 and P6–P8

Date: 2026-08-22. Scope is static/Bun validation only; Cargo was deliberately not run because a separate worker owns that surface. No ticket marker was cleared.

## Current verdict

| Phase | Status | Evidence / unmet gate |
| --- | --- | --- |
| P0 observability and freeze | Current static gates pass; prior child ticket is closed | `verify interactivity` passes in DENY mode with one approved blocking bridge; dependency ratchet passes (177 current / 238 baseline). Existing P0 report records the trace and inventories. Runtime/wasm proof was not rerun in this audit. |
| P1 one pool | **Open; cannot close** | The P1 ticket remains open. Its latest runtime-gate report records six production threads outside `WorkerPool`; subsequent source inspection shows some cited renderer/HTTP paths have changed, but this audit did not run the Cargo/runtime census needed to overturn the documented blocker. No current all-production thread-cardinality proof exists in the ticket. |
| P2 job/progress | **Open; cannot close** | The protocol, watchdog driver, progress vocabulary, commit validation, actor bridge and torture tests are present in source. The P2 bridge report explicitly leaves the plugin-host mounted native/test/clippy gate blocked upstream; this audit cannot revalidate it without Cargo. |
| P6 FEM | **Not gateable / no phase ticket** | No P6/FEM child ticket folder or gate report exists. Static source contains `MeshJob`, `AssemblyJob`, `PcgJob`, `LdltJob`, `SubspaceIterationJob`, and `FemJobGraph`; `rg 'async fn|\\.await'` finds no residual declarations in `✏️s/🔨️modules/🏗️fem/⚙️engine`. The required coarse-preview latency, cancellation, reference-tolerance and <8 ms runtime gates have no current ticketed execution evidence. |
| P7 WFC/Puzzle 2D/Energy | **Not gateable; WFC integration is unmet** | No P7 child ticket or gate report exists. `WfcJob` and `EnergyJob` implement `InteractiveJob`; Puzzle 2D has a current `BoardFillJob` continuation owned by another worker and was untouched. However, the Assembly inference entry point still declares and directly calls `compile_and_solve`, which builds a `GraphSolver` and invokes `solver.solve(seed)` synchronously. The source also calls this `async fn` without `.await`; Cargo ownership prevents a compile rerun here, but the call shape itself needs repair. That UI/inference-facing route is not wired to `WfcJob`, so P7's “all three” resumable-progress gate is not met. |
| P8 all tools | Static catalog gate passes; **phase remains open** | `tool-jobs` reports 769 unique production rows: 765 bounded/migrated and six `BatchOnlyPendingRewrite`; zero unclassified. The UI rejection backstop is statically verified. The six batch-only commands are deliberately unavailable from UI, but they are remaining rewrites within P8's stated scope; P8 also has an unverified strict native plugin Clippy blocker in its own report. |

## Current static verification

```text
$ bun ./📜️script.ts verify interactivity
[verify interactivity] severity=deny ...
[verify interactivity] 1 finding(s) total: blocking-bridge: 1
[verify interactivity] DENY mode — clean.

$ bun ./📜️script.ts verify interactivity tool-jobs --format json
macroHostFiles=50, macroInvocations=50, commandRows=771, uniqueCommandRows=769
boundedRows=765, batchOnlyRows=6, forbiddenRows=0, deletedRows=0
productionFactories=1, productionRegistrations=1, productionDispatches=1, failures=[]

$ bun ./📜️script.ts verify dependencies
baseline=238, current=177; clean — no new third-party dependencies.
```

The one interactivity finding is allowlisted; the verifier reports it clean in DENY mode. It must not be interpreted as a newly reachable forbidden call.

## Material blockers

1. **P1:** retain the open ticket until a current, production-only thread/process census proves the literal gate, including `!Send` DB/store paths. Existing historical evidence is insufficient to close it, even though several former sites are now gone or test-only.
2. **P2:** rerun the plugin-host mounted native/test/clippy gate after the upstream mesh-engine/compiler cohort is stable. This worker did not run Cargo.
3. **P6:** open/restore a phase ticket and record executable gates for coarse preview (<50 ms), immediate stale-solve cancellation, reference numerical tolerance, and watchdog step duration. Source presence is not timing or numerical evidence.
4. **P7:** replace the live Assembly `compile_and_solve` / `solver.solve(seed)` route with a persisted `WfcJob` operation driven by the shared job protocol; attach WFC preview/progress publication and generation-validated commit. Coordinate Puzzle 2D work with its active owner; do not overwrite its continuation. Then add executable Energy/WFC/Puzzle-2D timing, cancellation, preview, and worker-count determinism evidence under a P7 ticket.
5. **P8:** complete or explicitly delete/reclassify the six remaining batch-only commands: Remodel `runReconstruction`, `retryStage`, `runStage`; Draw `canvasPointerDown`; Flow `duplicateWidget`; Forms `setTryValue`. Also rerun the strict native framework-plugin gate once the OS-kernel lint cohort is repaired.

## Source anchors

- P2 protocol and watchdog: `🧰️framework/🔨️modules/🧵️job/🦀️component.rs` (`InteractiveJob`, `drive_step`, progress events, torture conformance).
- P2 actor bridge: `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs` (`JobTurnBridge`, `JobStepOutcome`, replay records).
- P6 jobs: `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️component.rs`, `🧮️analyses/🦀️component.rs`, and `🔢️sparse/🦀️component.rs`.
- P7 WFC blocker: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (`compile_and_solve`); resumable implementation exists separately at `…/🧩️wfc-engine/🧵️job/🦀️component.rs`.
- P7 Energy job: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️component.rs`.
- P8 coverage: `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8-production-tool-job-coverage.json`.

## Non-actions

- No Cargo, git-modifying, marker-clearing, renderer Rust, FEM Rust, or Puzzle 2D edits were made.
- No TypeScript/script defect independently within scope was found; the existing static verifier already enforces the P0 and P8 catalog checks.
