# ⚖️ Wave W1-D: tool-run policy predicates

Lane W1-D of `📋️tool-run-contract.md` (§1, §2.4, §2.5, §3.2, §3.6, §3.7, §5 W1-D row, §6.5).

**Status: landed.**

- All four self-test suites pass: 110 cases in total.
- A predicate-break check shows none of the self-tests is vacuous (12 of 12 deliberate breaks caught).
- The live gate is red, as expected: 253 findings belong to this lane (listed in §5).
- `verify interactivity` still throws before printing its report. The throw comes from pre-existing foreign self-tests (§4.3).

Paths are relative to the repo root. `S` = `📜️script.ts`. `E3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 1. What changed

### 1.1 Root script `S`

Line numbers were taken at the time of writing; the file is edited concurrently.

| Where | Change |
|---|---|
| `S:28-31` | Imports. Removed: the envelope and preview-json self-tests. Added: run-job, trace and tool-run-policy self-tests. P4e is kept. |
| `S:9058` (removed) | Deleted the 13 `INTERACTIVITY_AUDIT_PUZZLE_FILL_*` / `…PUZZLE3D/5D_*` path constants. They are replaced by the `INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES` role table. |
| `S:9113` | `InteractivityAuditCategory` gains `"tool-run"`. Its findings print as `file:line`, and the existing `otherCategoryFindings` rule makes them count as deny failures. |
| `S:9407-9422` | Audit call site: P4e (`blocking-bridge`), run-job, trace, then the five repo-wide predicates (`tool-run`). Every self-test runs before its predicate. |
| `S:9480-9690` | Region `🪣️PuzzleFillToolRun`: `INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES`, `interactivityPuzzleFillSources`, `interactivityPuzzleFillRunJobFailures`, `interactivityPuzzleFillTraceFailures`, and a trimmed `interactivityPuzzleFillP4eFailures`. |
| `S:9692-10052` | Region `⏯️ToolRunPolicy`: requirement data table, code masking, declaration index, source walk, and the five predicates. |
| export list (was `S:25404-25418`) | Removed the deleted constants and functions. The new API is exported inline. |

Deleted from `S`:
- `interactivityPuzzleFillEnvelopeFailures`: its laws were all about the plugin-local fill envelope and job bridge that contract §3.6 removes.
- `interactivityPuzzleFillPreviewJsonFailures`: it required the preview JSON page that §3.6 deletes.

### 1.2 Tests

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts` (new, foreign folder, see §6) | Synthetic miniature repo: a run-declaring tool and utility, a discovered run tool, an undeclared legacy command, framework constants and a trace host. It has 38 cases: 5 clean-pass checks plus 33 planted or tolerated variants. |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts` (new) | Exports `PUZZLE_FILL_TOOL_RUN_FIXTURE`, a contract-model miniature of 3d fill, and `puzzleFillToolRunFixtureWith`. It has 32 cases. |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-trace/🟦️.ts` (new) | Reuses the run-job fixture. It has 13 cases. |
| `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts` (rewritten) | Synthetic FillBuilder miniature. It has 27 cases. |
| `…/🔬️interactivity-puzzle-fill-envelope/` and `…/🔬️interactivity-puzzle-fill-preview-json/` | Deleted. They are replaced by run-job and trace. |

### 1.3 Self-test model change

**Old model.**
- The fill self-tests mutated the live production source.
- They threw whenever the live baseline was not green.
- Since phase 3 this aborted the whole `verify interactivity` run at the envelope self-test (`🗑️generated/W1-D/verify-baseline.txt`).
- It also made every mutation vacuous while W1-A/B are mid-rewrite.

**New model.**
- Every self-test runs against a handcrafted clean fixture that embodies the contract model.
- Each self-test asserts two things:
  - the clean fixture passes;
  - every planted violation is reported (for repo-wide predicates, at the planted file), and every tolerated variant (comment-only, test-module-only, config coalescing, undeclared rows, framework-owned literals) stays silent.
- Live sources only produce findings; they never throw.

## 2. Public API as landed (`S`, exported)

```ts
export const INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES: { precompute; fill; geometry; editor; tool; terminology; setFillCount; fillBuildTick; editorTests; runFixture; previewFixture; schema; transport; renderer; puzzle5dPrecompute; puzzle5dWindow } // repo-relative paths
export type InteractivityPuzzleFillSources = Readonly<Record<keyof typeof INTERACTIVITY_AUDIT_PUZZLE_FILL_FILES, string>>;
export function interactivityPuzzleFillSources(repoRoot: string): InteractivityPuzzleFillSources;
export function interactivityPuzzleFillRunJobFailures(sources: InteractivityPuzzleFillSources): string[];
export function interactivityPuzzleFillTraceFailures(sources: InteractivityPuzzleFillSources): string[];
export function interactivityPuzzleFillP4eFailures(precomputeSource: string, fillSource: string, geometrySource: string): string[];

export type InteractivityToolRunRequirement = { toolId; root; scope: readonly string[]; actions: readonly string[]; verbs: readonly string[]; measures: readonly string[]; lane; inventory };
export type InteractivityToolRunSource = { path: string; text: string };
export type InteractivityToolRunFinding = { file: string; line: number; text: string };
export const INTERACTIVITY_TOOL_RUN_REQUIREMENTS: readonly InteractivityToolRunRequirement[];
export function interactivityToolRunPolicySources(repoRoot: string, requirements: readonly InteractivityToolRunRequirement[]): InteractivityToolRunSource[];
export function interactivityToolRunAmendFailures(sources, requirements): InteractivityToolRunFinding[];          // (1)
export function interactivityToolRunLocalLifecycleFailures(sources, requirements): InteractivityToolRunFinding[]; // (2)
export function interactivityToolRunLegacyTraceFailures(sources): InteractivityToolRunFinding[];                  // (3)
export function interactivityToolRunDeclarationFailures(sources, requirements): InteractivityToolRunFinding[];    // (4)
export function interactivityToolRunReservedActionFailures(sources): InteractivityToolRunFinding[];               // (5)
```

The self-test entry points each return their case count:
- `interactivityToolRunPolicySelfTests(): number` in `⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts`;
- `interactivityPuzzleFillRunJobSelfTests()`, `interactivityPuzzleFillTraceSelfTests()` and `interactivityPuzzleFillP4eSelfTests()` in the puzzle test folders.

### 2.1 Shared machinery

**Code masking (`interactivityToolRunCode`).**
- Line-aligned.
- Comments are blanked; string literals are kept.
- Rust `#[cfg(test)]` modules and items are emptied.
- Files under `🧪️tests/`, `🧫️fixtures/`, `tests/`, `benches/` and `examples/` are excluded by `interactivityIsRuntimeSource`.

**Declaration index.**
- It finds every `ToolDefinition::new(id, …)` and `UtilityDefinition::new(id, …)` under `✏️s/`.
- An `id` may be a string literal or a `const X: &str`. The const is resolved in the same file first, then uniquely within the plugin.
- A declaration counts as declaring `run` when:
  - its enclosing `XDefinition { … }` struct literal contains `run: Some(`, or
  - its enclosing fn body contains `.run = Some(`.
- A run declared under a different root never satisfies a row.

**Source walk.**
- Rust and TS runtime sources under `✏️s` and `🧰️framework`.
- A file is kept when it lies under a row root or scope, or when it carries a trigger token.
- About 5.3k sources; the walk takes 9-14 s warm.

### 2.2 The five repo-wide predicates

1. **amend.** Scope: every requirement row, plus every run-declaring tool no row names (scoped to its declaring directory, non-recursively). Reported:
   - `Emit::amend(` / `ActionEmit::amend(` in the tool's scope;
   - an `Emit { … }` literal in scope with `coalesce_key: Some(` and non-empty `artifact_mutations`;
   - anywhere under the row root, a `coalesce_key = match` arm naming one of the row's `actions` and not mapping to `None`;
   - anywhere under the row root, a block guarded by `== "<action>"` that coalesces non-empty artifact mutations.

   Config-only coalescing is tolerated.
2. **local-lifecycle.** Applies only once the tool declares `run`. Reported:
   - the row's `verbs` anywhere under its root (token-bounded, `-` counts as part of a token);
   - the row's `measures` in its scope;
   - these generic patterns in its scope:
     - `fn *_progress_measure` / `fn *_cancel_measure`;
     - quoted `cancel|abort|stop|retry|discard|adopt|pause|resume|finalize…` action ids;
     - quoted `…Tick|Cancel|Abort|Adopt|Discard|Retry|Finalize` ids;
     - quoted kebab `start-/cancel-/…` verbs.
3. **legacy-trace.** Any occurrence of `*_TRIED_RING`, `*_TRIED_MAX`, `FillTried*`, `WorldFillTried*`, `push_tried`, `fillBuildPreview`, `FillBuildPreview`, `fill_build_preview`, `*FILL_PREVIEW_JSON*`, `FillPreviewJson*` or `fill_preview_json*`, in both `✏️s` and `🧰️framework`.
4. **declaration.** Each row needs a Tool/UtilityDefinition with the row's `toolId` under its root, declaring `run: Some(ToolRunDefinition)`. The table itself is also checked; these report as stale:
   - duplicate rows;
   - a root with no sources;
   - no surviving scope entry.
5. **reserved-action.** Only in `✏️s/`. Reported:
   - quoted `toolRun[A-Z]…` literals;
   - `ActionDefinition::new(_catalog)(TOOL_RUN_*_ACTION_ID`;
   - `TOOL_RUN_*_ACTION_ID =>` match arms.

   Referencing the constants in order to dispatch (for example `Effect::DispatchAction { action: TOOL_RUN_START_ACTION_ID.into() }`) is allowed.

### 2.3 Puzzle fill predicates

**Run-job.** The live laws are:
- The fill `ToolDefinition(TOOL_ID)` literal declares `run: Some(`.
- The run definition has `mutating: true`, `Revalidate`, `Resume`, `Instance3d`, `run_job`, `revalidate_job`, localized stages, counters and reasons, and Danger, Warning and Success verdicts.
- The job has `impl InteractiveJob for Fill…`, `ToolRunTickWriter`, `consume_fuel(1)`, and `PreviewReady`, `CheckpointReady` and `Complete` outcomes.
- The job issues `.upsert(` with Testing, Danger, Warning and Success verdicts and the `Instance3d` subject.
- Provisional results use `.append_op(`, `.append_entity(` and `.retract_to(`.
- A stall is a `ToolRunStepKind::Warning` step.
- The job has `begin_close`, `close_step` and `terminal_is_empty`.
- None of these plugin-lifecycle tokens remains:
  - job bridge: `enqueue_fill_job`, `cancel_fill_job`, `fill_job_identity`, `poll_fill_job`;
  - lock-is-commit: `take_fill_locked_chunk`, `take_locked_into_fixture`, `take_locked_mutations`, `FILL_LOCK_PLACEMENTS_PER_TICK`;
  - plugin spawn: `kind: FILL_JOB_KIND`;
  - plugin verbs and the history coalesce key: `"fillBuildTick"`, `"cancelFillBuild"`, `"fill-count"`.
- The `fill-build-tick` file is gone.
- The §5 red→green successor fixtures exist, and the two lock-is-commit fixtures are gone.
- The `🎞️fill-run.json` law pins:
  - `opsPerPlacement ≥ 1`;
  - a `parryOracle` entry;
  - `delivery.candidates ≥ 5000`;
  - `interactive` = Nakagin, `turns ≥ 771`, `budgetUs === 2000`;
  - `verdict:reason` prefixes on every case.

**Trace.**
- No `candidate_page`, `candidate_ghost`, `FillDiagnostic` or `world_fill_preview_json` in the 3d/5d producers.
- No `WorldFillDiagnosticRecord`, `FillDiagnosticOverlay`, `latestFillIdentityRef`, `data-fill-` or `WORLD_FILL_` in World3dHost.
- No `reveal_index`, `revealIndex`, `revealCutoff(s)` or `reveal_cutoff(s)` anywhere in those files.
- World3dHost mounts `<ToolRunTraceLayer`.
- The legacy `🧫️fixtures/🔣️.json` preview fixture is gone.

**P4e (kept, FillBuilder-intrinsic).**
- Kept unchanged: cooperative preparation, the fixed-owner preflight with cap/+1 evidence, the resumable spatial index, the "no whole-state checkpoint" law, and the fixture names.
- Rewritten:
  - the capacity-refusal law now requires a `ToolRunStepKind::Danger` step before `StepOutcome::Fault`;
  - the clone law is now file-wide.
- Removed: the transport, renderer and diagnostic-page laws (moved into the trace law as prohibitions).
- Changed: the fill-side prohibition is narrowed to `spatial_index.upsert(`, because the run job legitimately calls `writer.upsert(`.

## 3. Requirement table (`INTERACTIVITY_TOOL_RUN_REQUIREMENTS`, seeded from `📓️audit-p4-tool-inventory.md`)

The table has 28 rows. Tool ids for algorithms that are not tools yet are the expected future ids: the existing action id, or a noun for multi-verb lifecycles.

| toolId | root | lane | inventory |
|---|---|---|---|
| fill | puzzle 🧊️3d | W1-B | §1.1 |
| importFixture | puzzle 🧊️3d | unassigned | §1.1 |
| fill | puzzle ◻️2d | W2-A | §1.2 |
| fill (utility) | puzzle 🖐️5d | W2-B | §1.3 |
| importDocument, reorganize | procedural 🧊️generation3d | unassigned | §2.1 |
| reorganize | procedural 🌀️generation2d | unassigned | §2.2 |
| s.assembly.solve | procedural 🧩️assembly | W3 (3) | §2.3 |
| energySimulation | energy 🔋️model | W3 (1) | §3.1 |
| reconstruction | remodel 📸️remodeling | W3 (4) | §3.3 |
| decimate | lowpoly (+ framework mesh kernel in scope) | W3 (15) | §3.4 |
| formatDocument | writer | unassigned | §4.1 |
| reorganize | flow; sequence | W3-F | §4.3, §4.7 |
| runAnalysis, importProgram | architect 🏛️program | W3 (12) | §4.8 |
| forceLayout, reorganize | reasoning 🔌️wires | W3-F | §5.1 |
| importCadFile | cad | unassigned | §5.4 |
| run | imperative 📜️procedure | W3 (10) | §5.7 |
| reorganize | trinity ♻️rewriting (no scope); trinity 🔌️jack; dag | W3-F | §5.8, §5.9 |
| combineBoolean | draw | unassigned | §6.1 |
| inkApplyEvents | note | W3 (14) | §6.2 |
| exportStudioPack, exportMedia, reorganizeWorkflow | space ⚙️engine | W3 (13) | §6.4 |

Inventory rows deliberately left out:
- layout `exportPng/Svg/Pdf/Package` (`algorithmic-mutating/readonly`);
- shooting `export-shots` ("adjacent, side-effect only");
- architect `run-report/run-validation/search` (readonly);
- draw `updateLayerTraceParams` ("params only");
- the inner `WfcJob` capability, which is folded into the assembly row;
- generation3d preview eval and puzzle 3d brush suggestions (`mutating: false` runs).

When a lane converts a tool, it edits that tool's row (tool id, verbs, scope).

## 4. Tests run

All commands ran in the foreground. Logs are in `T/🗑️generated/W1-D/`.

### 4.1 Self-tests

| Command | Result |
|---|---|
| `bun T/🐍️w1d-policy-probe.ts --self-tests-only` | **pass**: `tool-run=38 run-job=32 trace=13 p4e=27`, 110 cases |
| `bun T/🐍️w1d-predicate-mutation-check.ts` | **12/12 predicate breaks caught** (`predicate-mutation-check.txt`), root script restored byte-identical (`cmp`). One temporary break per predicate; see §4.2 |

The breaks, one each, were:
- the amend regex;
- declaration accepting everything;
- the legacy regex;
- reserved skipping plugins;
- lifecycle never declared;
- lifecycle ignoring undeclared;
- comments kept;
- test modules kept;
- run-job tick command;
- trace mount;
- P4e refusal ordering;
- amend guard on empty mutations.

The P4e refusal-ordering break was missed on the first run. I added the `fault-before-danger-step` case, and it is caught now.

### 4.2 Harness incident

The first version of the break-check restored each break by first-occurrence string replacement.

- For the `continue;` break, that restored into `parseStorybookSegments` (`S:449`).
- For the `""` break, it prepended text at the top of the file.

Both were repaired within the same minute. `git diff HEAD -U0 -- 📜️script.ts` confirms that every remaining hunk lies in this lane's regions. The harness now restores by exact offset and verifies byte identity.

### 4.3 `bun nx run workspace:verify -- interactivity`

**Baseline before this lane: exit 1** (`verify-baseline.txt`). It aborted at the envelope self-test ("Puzzle fill envelope baseline was falsely rejected", 8 laws), so no report was printed.

**After this lane: exit 1.**
- The run passes all four new self-tests and the new predicate block.
- It then aborts at the **pre-existing** `interactivityLiveReconcileSelfTests`: "live reconcile self-test per-surface-credit-cap made no source mutation". Run in isolation, that test throws the same way; its file was last changed on 2026-09-08.
- Behind it is a second pre-existing throw in `interactivityMountedLayoutTextSelfTests`: "P5c mutation dynamic-surface-registry did not alter source".
- Neither file is owned by this lane.

**Full report.** To see the whole report, `bun T/🐍️w1d-verify-past-foreign-throw.ts` ran the nx gate once with those two throws plus `interactivityMountedFrameTransactionSelfTests` temporarily replaced by `void 0`; the root script was restored byte-identical (`verify-past-foreign-2-plain.txt`). The result was **DENY, 1252 failures**:

| Source | Count | Owner |
|---|---|---|
| all-app discovery / launch registration | 782 | pre-existing |
| unlisted `blocking-bridge` | 177 | 176 pre-existing (db storage/engine, live reconcile, P5a…); **1 W1-D** (P4e capacity refusal) |
| `sync-fs` | 33 | pre-existing |
| `sync-process` | 1 | pre-existing |
| `thread-pool` | 7 | pre-existing |
| `tool-run` | 252 | **all W1-D** (§5) |

## 5. Red list (live, expected red until waves 1–3 land)

From `probe-live-3.txt`; the `verify` report shows the same 252 `tool-run` findings.

### (1) amend: 4 findings

- `E3/🦀️.rs:3426`: `"setFillCount" | "fillBuildTick" => Some("fill-count")` coalesce arm (W1-B).
- `E3/🦀️.rs:6977`: the `setFillCount` retained publish coalesces `fill_mutations` under `"fill-count"` (W1-B).
- `…/📸️remodel/…/🎮️commands/🏗️run-reconstruction/🦀️.rs:561` and `:886`: `emit_step` and the terminal commit coalesce `artifact_mutations` under `reconstruction:<generation>` on every tick (W3 remodel).
  - The inventory rated remodel's transaction "YES", but under contract §1 this is per-tick durable amending.

### (2) local-lifecycle: 0 findings

No tool declares `run` yet. The row verbs start to count as soon as a lane adds `run`.

### (3) legacy-trace: 190 findings

| File | Count |
|---|---|
| `E3/⏳️precompute/🪣️fill/🦀️.rs` | 178 (`FillPreviewJson*` ×~140, `FillBuildPreview` 18, `FILL_PREVIEW_JSON_*` caps, `FILL_TRIED_RING`, `FillTriedCandidate`, `push_tried`) |
| `…/🧊️3d/…/🧬️schema/🦀️.rs` | 5 |
| `E3/⏳️precompute/🦀️.rs` | 3 |
| `🖐️5d/…/🧠️precompute/🦀️.rs` | 2 |
| `E3/…/🪟️windows/🧊️main/🦀️.rs` | 1 |
| `🖐️5d/…/🪟️windows/🧊️3d/🦀️.rs` | 1 |

World3dHost is already clean (W1-C).

### (4) declaration: 28 findings, all 28 rows red

- "ToolDefinition without `run`": 3d fill (`🛠️tools/🪣️fill/🦀️.rs:19`) and 2d fill (`:21`).
- "UtilityDefinition without `run`": 5d fill (`☑️…/🪛️utilities/🪣️fill/🦀️.rs:15`).
- "no Tool/UtilityDefinition": the other 25 rows.
- No stale rows.

### (5) reserved-action: 0 findings

### Puzzle fill run-job: 19 findings

- No `run` on the fill ToolDefinition.
- Run definition policy fields are missing.
- Stage, counter and reason vocabulary is missing.
- 11 plugin-lifecycle tokens are present: `enqueue_fill_job`, `cancel_fill_job`, `fill_job_identity`, `poll_fill_job`, `take_fill_locked_chunk`, `take_locked_into_fixture`, `take_locked_mutations`, `FILL_LOCK_PLACEMENTS_PER_TICK`, `"fillBuildTick"`, `"cancelFillBuild"`, `"fill-count"`.
- `fill-build-tick` still exists.
- 2 successor fixtures are missing.
- 2 lock-is-commit fixtures survive.

W1-A's in-flight run job already satisfies the stepping, verdict, provisional, stall, retirement and fixture-law checks.

### Puzzle fill trace: 11 findings

- `candidate_page` / `candidate_ghost` in fill, precompute and schema (5).
- `world_fill_preview_json` in the 3d main window and the 5d window (2).
- `reveal_index` / `revealIndex` in fill, precompute and schema (3).
- The legacy preview fixture still exists (1).

### P4e (`blocking-bridge`): 1 finding

The capacity refusal does not publish a danger step before the fault. The old code publishes a diagnostic page instead (W1-A).

## 6. Commands to register in launch.json

No new nx target. The predicates run inside the existing `⚖️gate⚡️interactivity` gate (`bun nx run workspace:verify -- interactivity`). Registering that gate is itself one of the pre-existing launch findings.

## 7. Deviations from the brief and contract, with reasons

1. **Test folder layout.**
   - Deleted: `🔬️interactivity-puzzle-fill-envelope`, `…-preview-json`.
   - Added: `🔬️interactivity-puzzle-fill-run-job`, `…-trace`.
   - Reason: both old folders encoded mechanisms that contract §3.6 deletes (the plugin fill envelope/job bridge and the preview JSON page). A rename matches the new laws better than old names with new contents.
2. **The repo-wide self-test lives in `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/`**, not under puzzle. The predicates are domain-neutral, and the domain-driven taxonomy puts their test next to the tool-run module. This is a foreign folder (W0-A module); the edit is additive, one new file.
3. **Self-tests run against synthetic contract fixtures instead of live source**, so live code that is mid-rewrite produces findings instead of aborting the audit (§1.3).
4. **New audit category `tool-run`** instead of reusing `blocking-bridge`. The findings then print with `file:line` and count through `otherCategoryFindings`; P4e stays `blocking-bridge`.
5. **Rows use expected tool ids** for algorithms that are not tools yet: the existing action id, or `energySimulation`, `reconstruction`, and the pre-existing `s.assembly.solve`.
6. **(2) applies only to tools that declare `run`**, as the brief says. Verbs of undeclared rows are already covered by the (4) red.

## 8. Foreign edits

- **New file** `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts` (W0-A module). It is not referenced by the module's own `📜️script.ts test`, which runs an explicit path.
- **Root `S` outside the `…Failures()` predicates** (all within the interactivity audit):
  - the import lines;
  - the deleted path constants;
  - the category union;
  - the `interactivityAuditRun` call site;
  - the export list.
- **Temporary edits to `S`, all restored byte-identical and verified:**
  - predicate breaks during the mutation check;
  - three foreign self-test calls replaced by `void 0 /* [DEBUG] */` for the full-report run.

## 9. Open items

- **Coordinator / owners of the two foreign self-tests.**
  - `interactivityLiveReconcileSelfTests` (`per-surface-credit-cap` anchor gone) and `interactivityMountedLayoutTextSelfTests` (`P5c dynamic-surface-registry` anchor gone) throw and abort `verify interactivity` before its report.
  - Before this lane they were hidden behind the envelope throw.
- **W1-A / W1-B / W1-C / W2-A / W2-B / W3.** Drive the §5 red list to green, and edit your own table row when converting.
  - W1-B: remove the `"setFillCount" | "fillBuildTick" => Some("fill-count")` arm and the `setFillCount` publish coalescing.
  - W1-A: publish a `Danger` step before the preparation-capacity `Fault`, and add the two successor fixtures.
- **Remodel.** Per-generation coalesced reconstruction commits are an amend under contract §1. This is new evidence against the inventory's "gold standard" rating.
- **Performance.** The tool-run source walk adds about 9-14 s warm to the audit. It walks `✏️s` and `🧰️framework` separately from `interactivityAuditScan`'s `.rs` walk. Sharing one walk would need a change in foreign audit plumbing, so it was not done here.
- **Heuristics that may need tightening once real conversions land.**
  - Discovered run tools are scoped to their declaring directory, non-recursively.
  - A fn-level `.run = Some(` marks every declaration in that fn.
  - The generic lifecycle regexes apply only inside scope.
- **Scripts kept in `T`:** `🐍️w1d-policy-probe.ts`, `🐍️w1d-predicate-mutation-check.ts`, `🐍️w1d-verify-past-foreign-throw.ts`.
- **Logs in `T/🗑️generated/W1-D/`:** `verify-baseline.txt`, `verify-past-foreign-2-plain.txt`, `probe-live-3.txt`, `predicate-mutation-check.txt`.
