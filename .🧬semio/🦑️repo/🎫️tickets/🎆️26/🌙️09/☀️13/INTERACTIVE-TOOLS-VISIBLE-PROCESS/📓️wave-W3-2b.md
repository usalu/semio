# 📓️ Wave W3-2b — Generation2d Preview Eval as a Tool Run

Lane **W3-2b generation2d preview eval** (`📋️wave-3-lane-brief.md`, mirror of `📓️wave-W3-2.md`). Abbreviation:

- `A` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any`

## 1. What Changed

### The self-rearming `flowEvalTick` chain is a read-only `previewEval` tool run

- **Source of record:** `A/🧵️preview-eval/🔣️.json` (new).
  - `mutating: false`, `rebase: restart`, `reconfigure: restart`, trace `entity`, `runJob` `generation2d.previewEval.run`.
  - One stage, `evaluate`. Counters `evaluated`, `failed`, `blocked`, `hops`.
  - 8 reasons in EN and DE: `queued`/`computing` (testing), `evaluated`/`settled` (success), `blocked`/`settledWithFailures` (warning), `failed`/`extensionFault` (danger).
  - Policies:
    - Read-only: the run only evaluates into retained sessions.
    - `restart`: an evaluation of a moved graph is a different result, and the session baseline keeps a restart down to the dirty nodes.
- **Transport:** `A/🧵️preview-eval/🦀️.rs` (new). It is mounted at the subset level in the crate root `🦀️.rs`, exactly where generation3d mounts its own.
  - The hop payloads `FlowEvalTick` (:31) and `FlowEvalResolve` (:40) now carry `windowId` and `windowKindId`.
  - `PreviewEvalTarget {Document, Generation}` (:60).
  - `tick_effect` (:80), `may_rearm` (:86), `attached_preview_windows` (:100).
  - `owe_attached_previews{,_for_mutations,_carrying}` (:105–:140).
  - `evaluate_tick` (:158), `settle_empty_tick` (:189), `resolve_eval` (:200), `preview_eval_digest` (:221).
- **Run module:** `A/🧵️preview-eval/⏯️tool-run/🦀️.rs` (new).
  - Vocabulary enums (:15–:116), `preview_eval_tool_definition` (:131), `preview_eval_node_entity` (FNV-1a 64, :140).
  - `observe_preview_eval` (:187): merges every attached target's status; the least settled reason wins.
  - `PreviewEvalSessions` (:196) and `next_preview_eval_hop` (:229).
  - `PreviewEvalRunLink` (:244), with a request latch, `job_run` and the new per-target `evaluated` digests.
  - `owe_moved_targets` (:317) and `preview_eval_run_effects` (:330).
  - `PreviewEvalRunJob` (:377) with its `InteractiveJob` impl (:472):
    - one hop per `consume_fuel(1)`;
    - waits on the port while a hop is outstanding;
    - closing an owned, unsettled job calls `cancel_preview_evaluation` on every attached window.
- **Two retained sessions:** `A/✏️editor/🦀️.rs` `Generation2dInstanceOperationOwner` (:63).
  - It holds `eval_session` (document: the flow and edit preview render it), `generation_session` (the generate preview's patched fixture) and the `run_link`.
  - Why two: a single session held one evaluation, so evaluating the generate preview would have overwritten what the edit preview renders.
  - The close ladder retires both sessions. `PreviewEvalRunOwner` impl at :106.
- **Editor wiring:** `A/✏️editor/🦀️.rs`.
  - `GENERATION2D_PREVIEW_TARGETS`, `generation2d_preview_target` and `generation2d_preview_windows` (:257–:267).
  - `Generation2dFlowEvalWork` (:371) and `generation2d_flow_eval_hop` (:412):
    - the tick evaluates the target its window kind names and records that target's digest;
    - the generate target publishes `SetGenerationPreview` into the app transient when its evaluation changed;
    - the resolve folds the answer into the target's session;
    - both wake the job.
  - The `flowEvalTick` publication lane went from `HostOnly` to `Transient` (:550).
  - Every other session route now owes previews through `owe_attached_previews_for_mutations`.
  - `setContributions` invalidates both sessions and, if anything was invalidated, carries the run start.
  - `pending_effects` → `preview_eval_run_effects` with the current target digests (:1637).
  - `build_tool_run_job` (:1653). `.tool(preview_eval_tool_definition())` (:1788). `flowEvalTick` is declared as a runtime command.
  - Both modes reference the tool: `🎭️modes/✏️edit/🦀️.rs` and `🎭️modes/🧬️generate/🦀️.rs`.
- **Command bindings:**
  - `🎮️commands/⏱️flow-eval-tick/🦀️.rs`: `evaluate` and `target_digest`.
  - `✅️flow-eval-resolve/🦀️.rs`: the answer fold, which arms nothing.
  - `🧩️set-contributions/🦀️.rs`: `install(payload, sessions) -> Result<bool, Fault>`.
- **Deleted:**
  - the `flow_eval_tick::rearm` self-redispatch, and the re-arms in `flowEvalResolve`, `setContributions` and `pending_effects`;
  - the scratch-session `pending_effects` sync;
  - `Generation2dPreviewCommandWork`, the in-guest synchronous `FlowEvalSession::tick` loop inside generation commands, together with its Progress label `generation2d-preview-evaluation`;
  - the `Transient` lane on the five generation commands.
- **Crate `Cargo.toml`:** added `semio-framework-tool-run`, plus dev-dependency `fnv = "1.0.7"` (the oracle).

### Inventory classification (corrects `📓️audit-p4-tool-inventory.md` §2.2)

| Tool | Verified kind | Result |
|---|---|---|
| Flow evaluation (`flowEvalTick`/`flowEvalResolve`) | `algorithmic-readonly` | Converted |
| Generation command preview (`Generation2dPreviewCommandWork`) | `algorithmic-readonly`; missing from the inventory, and in-guest it cannot serve contributed operators | Converted: the generate preview is now the run's `Generation` target |
| `reorganize` | `algorithmic-mutating` | Left to W3-F (unassigned W1-D row) |
| Generation CRUD, canvas and pointer commands | one-shot / direct manipulation | Out of scope |

## 2. Public API (as landed)

- **`crate::preview_eval` transport:**
  - `tick_effect(&str, &str) -> Effect`
  - `may_rearm(&FlowFixture) -> bool`
  - `attached_preview_windows(Option<&ViewModel>, &[(&'static str, PreviewEvalTarget)]) -> Vec<PreviewEvalWindow>`
  - `owe_attached_previews(PreviewEvalSessions, &mut PreviewEvalRunLink, &[PreviewEvalWindow])`
  - `owe_attached_previews_for_mutations<M,C,D>(…, servable: bool, &mut Emit<M,C,D>) -> bool`
  - `owe_attached_previews_carrying<M,C,D>(…)`
  - `evaluate_tick(window_id, window_kind_id, &FlowFixture, &mut FlowEvalSession, Option<&str>) -> FlowEvalTickOutcome`
  - `settle_empty_tick(&str, &mut FlowEvalSession)`
  - `resolve_eval(&FlowEvalResolve, &mut FlowEvalSession)`
  - `preview_eval_digest(&[&str]) -> u64`
- **`crate::preview_eval` run:**
  - `observe_preview_eval(&[&str]) -> PreviewEvalObservation`
  - `next_preview_eval_hop(&PreviewEvalSessions, &[(String, &'static str, PreviewEvalTarget)]) -> PreviewEvalHop`
  - `owe_moved_targets(&mut PreviewEvalSessions, &PreviewEvalRunLink, &[PreviewEvalWindow], &BTreeMap<PreviewEvalTarget, u64>)`
  - `preview_eval_run_effects(PreviewEvalSessions, &mut PreviewEvalRunLink, &[PreviewEvalWindow], &BTreeMap<PreviewEvalTarget, u64>, Option<&ToolRunView>, bool) -> Vec<Effect>`
  - `trait PreviewEvalRunOwner { fn preview_eval_parts(&mut self) -> Option<(PreviewEvalSessions<'_>, &mut PreviewEvalRunLink)> }`
  - `PreviewEvalRunJob<O>::new(ArtifactInstanceOperationOwnerHandle, ToolRunJobPort, ToolRunIdentity) -> Result<Self, Fault>`
- **Commands:** `flowEvalTick {windowId, windowKindId}` and `flowEvalResolve {windowId, windowKindId, nodeHash, outputJson, extensionId, ok, faultCode, faultMessage}`.

## 3. Tests Run (Foreground, Logs in `🗑️generated/W3-2b/`)

| Command | Result |
|---|---|
| `cargo check -p semio-s-artifact-procedural-generation2d --features component-app-assembly --tests` | clean, 31 warnings, none in lane files — `check-tests-2.txt` |
| `… --target wasm32-wasip2` (lib) | clean, 12 warnings — `check-wasm.txt` |
| `cargo test … --lib -- preview_eval demo_run_job closing_an_unsettled eval_result_seeds every_command command_ids retained_route_dispositions the_manifest_stitches` | 18 pass, 1 fail — `test-preview-eval-final.txt`. The fail is the close-only pre-existing red (§6). |
| `cargo test … --lib` (full) | 213 pass, 35 fail — `test-lib-full.txt`. All 35 are pre-existing (§6). |
| `bun T/🐍️w3-2b-preview-eval-contracts.ts` (TS twin of the fixture) | OK — `ts-twin.txt` |
| `bun T/🐍️w1d-policy-probe.ts` | amend 0, local-lifecycle 0, legacy-trace 0, reserved-action 0. The generation2d `previewEval` declaration is found; only the unassigned `reorganize` rows remain — `w1d-probe.txt` |

### Laws

**Fixture laws** (`A/🧫️fixtures/⏯️preview-eval-run.json`, run by `🧵️preview-eval/🧪️tests/🔬️unit` and the TS twin `🔬️unit/🟦️.ts`):

- `hopRequest`, `targets`, `vocabulary`;
- `entityKeys`, with the `fnv` crate as the third-party oracle;
- `observations`, including merging two targets;
- `scheduling` (10 rows), `runEffects` (10), `documentMoved` (4, new), `gestureRearm` (9), `jobSupersession` (4).

**Run-job law:** `the_demo_run_job_traces_every_node_settles_and_stays_under_the_interactive_ceiling` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:577`, fixture `demoRun`).

- Measured on the bundled demo: 2 ticks, 1 hop, **worst `drive_step` 128 µs** against the 8 000 µs ceiling.
- Trace verdicts were Success and Danger (`rect` is `failed` because `math.add` is uncontributed); the settle step was `settledWithFailures`.

**Abort law:** `closing_an_unsettled_run_job_quiesces_its_window` (:646).

**App-level laws**, which drive the real framework ledger through `pending_effects` and `handle_action`:

- `a_generation_command_carries_the_run_start_and_the_run_publishes_the_shared_generate_preview` (:418):
  - `addGeneration` publishes lanes (1, 1, 0) and carries exactly one `toolRunStart`;
  - the run hops the generate previews, publishes the shared app transient, and both windows render the same output.
- `the_preview_eval_run_finalizes_nothing_and_an_abort_leaves_the_document_byte_identical` (:482):
  - finalize leaves the document pack, the config pack and the generation unchanged;
  - `toolRunAbort` ends the run as `aborted`, no poll restarts it, and the document pack/spr and generation are byte-identical.
- Both laws' bodies returned `Ok(())`; this was verified with a temporary `[DEBUG]` print, since removed (`test-app-laws.txt`). They still report FAILED only because of the pre-existing fixture-close red (§6).

**Brief items:**

- **"one undo entry":** not applicable; a read-only run authors none, and the law asserts exactly that.
- **Third-party oracle for evaluation results:** none beyond `fnv`, because evaluation is the contributed extension's.

## 4. `launch.json`

To register, beside the generation3d preview entries:

- `cargo test -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib -- preview_eval`
- `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/🐍️w3-2b-preview-eval-contracts.ts`

## 5. Deviations

- **Two sessions and targets instead of 3d's per-window transients.**
  - generation2d's edit preview and flow window render the retained session directly.
  - The generate preview renders the app transient.
  - Each window kind therefore maps to a target session rather than to a window transient.
- **Poll digest (`documentMoved`), which 3d does not have.**
  - The old generation2d `pending_effects` re-evaluated after undo, redo and remote edits.
  - A gesture-only debt would have regressed that.
  - Each hop records the digest of the fixture (plus the generation state and selection for the generate target) it evaluated. A poll owes the windows of a moved target. An unmoved target owes nothing, so an aborted run stays down.
- **No release hop and no status rows.**
  - generation2d requests no budgeted kernel job, so there is nothing to release.
  - Its previews are Canvas2d scenes, which have no compute-status contract; the abort affordance is the framework ToolRun panel plus `toolRunAbort`.
- **No finalize action from the surface in the lifecycle law.** A read-only run is finalized by the framework as soon as it completes (W0-I). Complete runs are still finalized by `preview_eval_run_effects`.

## 6. Foreign Edits and Pre-Existing Reds

- **Foreign edit:** root `📜️script.ts` `INTERACTIVITY_TOOL_RUN_REQUIREMENTS`.
  - New row: `previewEval` at the generation2d root.
  - Scope `🧵️preview-eval/⏯️tool-run`.
  - Verbs `rearm`, `Generation2dPreviewCommandWork`, `generation2d-preview-evaluation`.
  - The docstring was updated to match.
- **Pre-existing reds, not caused by this lane:** every generation2d registered-app test fails at `close_registered_fixture_app`, with either "document store close awaits a retained reader or owner" or "returned snapshot-read disposer is waiting on external ownership".
  - **Proof it predates this lane:** with every lane file temporarily restored to `HEAD`, `an_unknown_body_key_renders_a_diagnostic_instead_of_panicking` and `add_widget_undo_redo_round_trip` failed identically (`test-baseline-head.txt`). The lane files were restored afterwards.
  - **Not framework-wide:** generation3d close tests pass against the same framework (`test-3d-close-probe.txt`).
  - **The 35 reds:** 31 close-witness failures, 2 of the fixture-store terminal witness, 1 `retirement.terminal_is_empty`, and `declared_actions_bridge_to_commands` ("nodeGraphViewport requires viewport"), which touches no lane code.
- **Peer edits observed:** W0-I edited `P/⏯️tool-run/🦀️.rs` at 10:37. Nothing from it was touched.

## 7. Open Items

- **Fix the generation2d registered-fixture close witness** (document store reader / snapshot-read disposer). It masks every generation2d app law, including the two new lifecycle laws.
- **Dependent generated manifest:** `✏️s/🔌️plugins/🌀️procedural/🔣️.json` still lists `flowEvalTick` as an action and must be regenerated by its owner.
- **Document reload:** after one, the app transient resets while an unmoved generation session keeps its evaluation, so the generate preview shows its hint until the next gesture or the next move of the generation digest.
- **Mounting the trace:** node highlighting from entity trace records in the node graph belongs to the UI lanes. `reorganize` belongs to W3-F.
