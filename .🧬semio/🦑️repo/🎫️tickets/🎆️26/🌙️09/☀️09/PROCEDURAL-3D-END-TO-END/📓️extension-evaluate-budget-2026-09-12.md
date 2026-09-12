# Budgeted, Resumable `evaluate` — Watchdog Liveness and Shard Recovery (2026-09-12)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "extension evaluate budget". Repo MCP was down for
the whole run (`repo (-32602): invalid initialize params`, `semio: connection closed`); ticket
bookkeeping is on disk and no ticket was opened/closed/reopened. Outputs under
`🗑️generated/evaluate-budget/`.

Closes item **2** of `📓️preview-eval-cancellation-2026-09-12.md` §5 ("Budgeted `evaluate`") and its
§4.2 finding ("a single guest turn is uninterruptible at the door"), and the session-death half of
`📓️shard-termination-2026-09-10.md` §6's open note ("making it yield is guest work").

---

## 1. The defect, measured

`🗑️generated/mesh-delivery/coordinator-sphere/console-Sphere-Cut-With-Torus.txt:5824`:

```
70897 error [DEBUG] shard 0 terminated by the host watchdog: the worker was silent for 16271 ms;
outstanding: turn flow-extension-brep#request started 16299 ms ago; turn procedural#1 started 16271 ms ago.
70897 error [DEBUG] PluginRuntime: shard 0 lost, restoring actors: procedural#1, flow-extension-brep#request
70898 error [DEBUG] PluginRuntime: turn failed for actor procedural#1 Error: shard 0 terminated …
70902 error [DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-brep, capability: evaluate, req: 9n, …}
```

and then, in `console-Box-Fillet-Preview.txt`, every later gesture on the restored actor:

```
74102 error [DEBUG] action failed interactionHover … Error: [DEBUG] plugin procedural: command ingress fault: plugin.command-page-invalid
74643 error [DEBUG] action failed setActiveExample {exampleId: box-fillet-preview} Error: … plugin.command-page-invalid
```

Two distinct defects, in sequence.

### 1.1 One `evaluate` request was one uninterruptible kernel call

`brep.bool.cut` on `🍩️sphere-cut-with-torus` is one synchronous call inside one guest turn. Requests
1–8 answered with `turns: 1` in 1.6–7.6 s each; request 9 (the boolean) never answered, and 16.3 s of
worker silence tripped `evaluateShardLiveness`'s `missedLimit × heartbeatTimeoutMs` ladder.

**Where the time actually goes** — measured natively (debug, aarch64) by instrumenting
`exact_imprint_boolean` and `validate_body` with `[DEBUG]` timers and running
`sphere_cut_with_torus_evaluates_to_the_difference_volume`. Instrumentation removed afterwards; the
numbers are the design input for the unit decomposition below:

| stage | cost | what it is |
|---|---|---|
| `boolean_solid` fast paths | 0.19 ms | trivial/box shortcuts, never taken here |
| imprint (1 face-pair) | **63 ms** | SSI + clip + imprint-edge construction |
| apply pending imprints | 0.45 ms | 3 pieces per side |
| classify | 4.0 ms | 3 faces against the other solid |
| **stitch** | **1.50 s** | `group_shells` + `shell_signed_volume` per group + containment |
| **validate** | **3.00 s** | `validate_body` |
| whole exact boolean | **4.51 s** | |

and inside that 3.00 s validation, per check:

| check | cost |
|---|---|
| `check_loop_rings` | 5.8 µs |
| `check_edge_valence` | 6.7 µs |
| `check_tolerance_containment` | 39.5 µs |
| `check_missing_pcurves` | 1.0 µs |
| `check_same_parameter` | 41.5 µs |
| `check_shell_closure_and_orientation` | 18.7 µs |
| `check_face_loop_winding` | 171.5 µs |
| **`check_solid_orientation`** | **1.551 s** |
| **`check_degenerate_geometry`** | **1.614 s** |
| `check_self_intersection_probe` | 74.1 µs |

So the dominant cost is NOT the face-pair loop everyone assumes — it is two per-entity loops
(`shell_signed_volume` per face, `face_area` per face) inside the validator, plus the stitch's own
signed-volume probe. That is what decides where the yield points had to go.

`validate_body` runs **twice** per preview (once inside the boolean, once as the preview's own
validate gate), and the whole `sphere_cut_with_torus` example test costs 22–25 s natively, of which
the boolean is 4.5 s and the rest is tessellation and the preview gate.

### 1.2 Restoring the ACTOR is not restoring the APP

`ActivationRegistry.restoreActor` (`🎠️kernel/🟦️.ts:2157`) bumps the generation, cancels the queue and
`resume()`s the actor; `resume` restores `this.checkpoints.get(actorId)` **if one exists**. A watchdog
kill takes no checkpoint, so the guest came back EMPTY — the console shows the restored actor at
`turn=1` with `bytes=19136512 delta=19136512`, i.e. a fresh linear memory.

`createApp` was never re-run, so `runtime.guest_lifetimes` has no instance 1, and
`⚛️reactor/🔄️turn/🦀️.rs:884`'s very first command-page predicate
(`!runtime.guest_lifetimes…is_some_and(|slot| slot.cell.is_live())`) refuses every later page with
`plugin.command-page-invalid`. The session answers nothing until the user reloads. `ShellHost` had a
one-shot retry for exactly this fault — but only on the BOOT path, and nothing told it a LIVE session
had died.

---

## 2. Budgeted, resumable `evaluate`

### 2.1 The contract (schema-first)

**Fixture:** `✏️s/…/🧊️generation3d/…/🪆️subsets/✳️any/🧫️fixtures/⏱️evaluate-budget.json` — 11 named
laws, 6 fold rows (each an envelope plus the exact status the preview window must publish), a
`jobPhaseTags` projection table, and a `steppedOperator` block naming `brep.bool.cut`, its phase order
and the minimum round trips a tight budget must produce.

**Framework wire fixture:** `🧰️framework/…/🌊️flow/🧩️extensions/🕸️wasm/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json`
— the `evaluate` capability's own wire, now declaring `optionalRequestFields`
(`nodeHash`/`budget`/`wallMicros`) and `envelopeFields`, with per-row `done`/`phase`.

**Request:** `{operatorId, inputJson, nodeHash?, budget?, wallMicros?}`.
**Answer:** one envelope, for every flow extension:

```
Working  {done:false, cancellable:true,  phase:"<job tag>", unitsDone, unitsTotal, outputJson:""}
Complete {done:true,  cancellable:false, phase:"complete",  unitsDone, unitsTotal, outputJson:"<out dict>"}
Cancelled{done:true,  cancellable:false, phase:"cancelled", unitsDone, unitsTotal, outputJson:""}
```

A refused evaluation is a `complete` whose `outputJson` is the `{"error": …}` body the unbudgeted call
always produced — a stepped answer and a one-shot answer are indistinguishable downstream. A response
body that is not an envelope at all is read as a finished evaluation whose output IS that body; it is
the only reading that cannot lose an answer.

### 2.2 The four layers

```
  ┌──────────────────────────────── generation3d app (wasm guest) ─────────────────────────┐
  │  flowEvalTick ──► pending_extension_eval ──► ExtensionInvocation{                      │
  │                                                capability:"evaluate",                  │
  │                                                request:{…, nodeHash, budget=8,         │
  │                                                          wallMicros=2_000_000},        │
  │                                                response_action:"flowEvalResolve"}      │
  │  flowEvalResolve ──► FlowEvalSession::resolve_preview_eval(nodeHash, envelope)          │
  │        Working  ──► publish progress, re-arm the SAME request                           │
  │        Complete ──► seed_node_cache(outputJson), re-arm                                 │
  │        Cancelled──► seed nothing, arm nothing                                           │
  └──────────────────────────────────────┬──────────────────────────────────────────────────┘
  ┌──────────────────────────────────────▼──────────────────────────────────────────────────┐
  │  flow-extension-brep :: bundle().handler("evaluate")                                    │
  │     flow_extension_sdk::evaluate_invoke_json                                            │
  │       ├─ retained job for (operatorId, nodeHash)?  ──► resume it                        │
  │       └─ Registry::dispatch_job(operatorId, input)                                      │
  │            None       ──► Registry::dispatch (one call, as before)                      │
  │            Some(job)  ──► step while working AND the wall deadline has not passed        │
  │  bundle().handler("evaluateCancel")  ──► cancel_evaluation / cancel_all_evaluations     │
  └──────────────────────────────────────┬──────────────────────────────────────────────────┘
  ┌──────────────────────────────────────▼──────────────────────────────────────────────────┐
  │  neural_engine (domain-NEUTRAL)                                                          │
  │     trait OperatorJob { step(budget) -> Working|Done(Dictionary)|Cancelled;              │
  │                         progress(); cancel(); }                                          │
  │     Operator::step_plan(&input) -> Option<Box<dyn OperatorJob>>   (default: None)        │
  │     Registry::dispatch_job / Registry::finish_job                                        │
  └──────────────────────────────────────┬──────────────────────────────────────────────────┘
  ┌──────────────────────────────────────▼──────────────────────────────────────────────────┐
  │  semio-s-artifact-stdio-semio :: the kernel                                              │
  │     BooleanJob   Imprint(face pair) → ApplyA/B(face) → ClassifyA/B(face) → Stitch        │
  │                  → Validate(BodyValidationJob unit) → Complete | Cancelled               │
  │     BodyValidationJob  8 cheap whole-body checks + ONE FACE per shell-volume unit        │
  │                        + ONE EDGE / ONE FACE per degeneracy unit                          │
  └──────────────────────────────────────────────────────────────────────────────────────────┘
```

### 2.3 Kernel — `BodyValidationJob`

`🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs`. `validate_body` is now an **unbudgeted
façade over the job** (`BodyValidationJob::new(body).run_to_completion(body)`) — exactly the relation
`tessellate_solid` has to `TessellationJob`, so there is no second pass to drift from.

Units: the eight microsecond checks stay one whole-body unit each; `solidOrientation` accumulates
`mass_properties::face_volume_contribution` **one face at a time** across every solid's outer and void
shells (and emits its verdicts, in the original solid-then-void order, once the sums are complete);
`degenerateEdges` is one edge per unit and `degenerateFaces` one `face_area` per unit. The two
whole-pass functions those replaced are gone.

`face_volume_contribution` became `pub` for exactly this reason.

### 2.4 Kernel — `BooleanJob`

`🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`. `exact_imprint_boolean` is replaced by a cursor state machine over
the SAME code: the inner face-pair body was extracted verbatim into `imprint_face_pair`, and every
other stage reads its own cursor. `boolean_solid` is now

```rust
match boolean_job(body, a, b, op, tol, rec)? {
    BooleanAdmission::Answered(id) => Ok(id),
    BooleanAdmission::Job(job) => job.run_to_completion(body, rec),
}
```

so the budgeted and unbudgeted paths are one implementation. The microsecond-cheap trivial/box fast
paths are taken during ADMISSION (they mutate the body and produce the result in place, so they can
never be re-run) and come back as `Answered`.

`units_total` is the plan known so far and is revised upward exactly once, when the stitch lands and
the result's validation plan can finally be read — nothing about the post-stitch body is knowable
before the stitch. `units_done` is monotone throughout.

🚧️ **Honest limitation, stated in the type's own doc:** a cancelled or failed job leaves the imprints
it already applied in the body — exactly what a boolean returning `Err` mid-pipeline has always left.
The kernel has no transaction; nothing is rolled back.

### 2.5 Kernel — `Brep`

`boolean_job_sync` / `step_boolean_job_sync` (+ `BrepBooleanJob`, `BrepBooleanAdmission`,
`BrepBooleanStep`), the exact analogue of `tessellate_job_sync`. `BrepBooleanJob` **owns** its
`OpRecorder`, because the recorder must outlive every step. `Brep::boolean_sync(a, b, op)` is the new
one-shot entry every `fuse`/`cut`/`intersect` goes through, so the two roads cannot diverge.

### 2.6 Framework — the domain-neutral operator job

`🧠️neural/⚙️engine/🦀️.rs`: `OperatorProgress`, `OperatorJobStep`, `trait OperatorJob`,
`Operator::step_plan` (default `Ok(None)`), `Registry::dispatch_job`, `Registry::finish_job`.

The framework knows only units, a phase tag and three outcomes. Which units an operator has is the
domain extension's business. Every operator that does not answer `step_plan` is evaluated in one call
exactly as before — no behaviour change for the ~400 non-boolean operators.

### 2.7 Extension SDK — the budget, once, for all ten extensions

`🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs`: `evaluate_invoke_json` now parses `EvaluateRequest` and delegates
to `evaluate_step_envelope_json`, plus a bounded (16-slot, LRU) process-global job registry keyed by
`(operatorId, nodeHash)` with `cancel_evaluation` / `cancel_all_evaluations` / `evaluation_progress`.
`EVALUATE_STEP_BUDGET = 8` units, `EVALUATE_STEP_WALL_MICROS = 2_000_000`.

**Why 2 s:** it is below the shard watchdog's ORDINARY 5 s heartbeat window, not merely below its
16 s termination threshold. A guest turn that never yields blocks the worker's event loop and its
progress ticker with it, so the only thing that distinguishes a busy worker from a dead one is the
turn boundary — and a 2 s ceiling puts one inside every window.

A step runs **without the registry lock held** (the job is taken out and parked back), so a second
concurrent request for the same key starts its own job instead of deadlocking.

### 2.8 Extension — the three set operations

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`: `Fuse`/`Cut`/`Intersect` are the only operators in
the extension that answer `step_plan`, through `BrepBooleanOperatorJob`. Admission ALWAYS returns a
job — even a fast-path answer, because admission has already mutated the body and a `None` there would
make the caller evaluate the whole boolean a second time. New `evaluateCancel` handler.

### 2.9 Guest — the fold, the ledger and the status

- `🌊️flow/🖥️host/🦀️.rs`: `eval_progress_by_hash` on `FlowEvalSessionState` (wired into construction,
  `clear`, the retirement ladder and `terminal_is_empty`), `PreviewEvalPhase` (+`from_job_tag`,
  `tag`, `labels` en+de, `is_cancellable`), `PreviewEvalProgress`, `PreviewEvalStatus` (+`ratio`),
  `PreviewEvalOutcome`, `FlowEvalSession::resolve_preview_eval`, `preview_eval_status`,
  `preview_eval_cancel_invocation_request_json`. `cancel_preview_evaluation` EMPTIES the evaluation
  ledger (unlike the tessellation ledger, whose frozen counters are what the surface keeps showing).
- `🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`: the tick sends `budget`/`wallMicros`; `resolve_eval` folds
  the envelope (working → re-arm and seed nothing, cancelled → owe nothing, complete → seed);
  `cancel_preview_eval_for` now emits **two** invocations — `evaluateCancel` and `tessellateCancel`,
  two doors because there are two registries of retained kernel work and each names its own.
- the status projection publishes `inFlight` counting BOTH kinds of work, new `evalUnitsDone`/
  `evalUnitsTotal`, a `ratio` taken from the evaluation while it is the only work outstanding, and
  the evaluation's own localized phase instead of `idle`.

---

## 3. Watchdog liveness

`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` (the generated shard worker): a `heartbeat("turn-step")` is
now emitted at the STEP BOUNDARY — the instant `actor.api.poll(…)` (and `stepJob`) hands control back,
before the reply is posted. The start-of-request beat proves only that the request was received; the
step beat is what proves a guest running a budgeted job is alive, because such a guest blocks the
worker's event loop (and the while-busy ticker with it) for the whole of each step.

The 16 s threshold and every policy number are unchanged: `SHARD_LIVENESS_POLICY` is schema-owned
(`📮️shard-client/🧬️schema/🔣️.json`) and this lane did not touch it. What changed is that a worker
doing exactly what it was asked to do now crosses an observable boundary every ≤ 2 s.

---

## 4. Shard recovery

### 4.1 The runtime retires and announces

`🔌️PluginRuntime/🟦️.tsx`: a module-level `instanceRecoveryByActor` map, written by `createApp` after
the open bound a UI owner and cleared by `releaseInstanceMaps`. `handlePluginShardLost` now:

1. retires the bookkeeping of every instance those actors hosted (`releaseInstancesForLostActors`),
2. restores the actors as before,
3. announces the losses (`notifyPluginInstancesLost`) with a `[DEBUG] PluginRuntime:
   plugin.actor-instance-lost — …` line.

New exports: `onPluginInstancesLost`, `LostPluginInstance`, `PLUGIN_ACTOR_INSTANCE_LOST_FAULT`
(`"plugin.actor-instance-lost"`). An extension's request actor owns no app instance, so its loss
announces nothing — it is a retryable request failure, not a lost session.

### 4.2 The shell re-establishes, or says so

`🏛️ShellHost/🟦️.tsx`: an effect subscribes to `onPluginInstancesLost` and, when the lost instance is
the current session's, re-runs `establishPrimaryWithShardRetry` on the rebuilt shard — the same
`createApp` + layout seed the boot path uses, which re-opens the document. If that also fails, the
shell sets a typed, localized error from the new `ui.common.workerLost` label
(en: "The session was terminated — please reload", de: "Die Sitzung wurde beendet — bitte neu laden"),
added to `📚️I18n`'s `common` block and to BOTH language tables.

### 4.3 The outstanding request gets a typed fault

`runCapturedExtensionEffect` maps `isShardLostError(error)` onto the new
`EXTENSION_WORKER_LOST_FAULT = "extension.worker-lost"`, `retryable: true`, instead of the anonymous
`extension.invoke-failed`. That code travels through `note_eval_answer_fault` into the preview
status's `fault.faultCode`, so the surface can distinguish "the geometry kernel refused this" from
"the geometry kernel's worker died".

---

## 5. Tests — all run in the foreground, results verbatim

### 5.1 The example-geometry gate (the required one)

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --test example-geometry
running 17 tests
…
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.63s
```

Baseline before any change, same command: `17 passed … finished in 50.43s`.

Re-run after every change in this lane, on a machine loaded by concurrent peer builds:

```
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 62.81s
```

### 5.1b Whole generation3d lib

```
$ RUST_MIN_STACK=67108864 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
test result: FAILED. 379 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 198.46s
```

All six are **known reds that predate this lane**: the five recorded in `📓️status.md:662`
(`add_generation_records_an_undoable_generation_operation`,
`generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
`two_instances_converge_disjoint_widget_moves`, `undo_redo_round_trips_flow_graph_edits`,
`vcs_artifact_app_…_fail_closed`) plus `refresh_pending_effects_arms_flow_eval_tick_chain`, which
five earlier reports already list with the identical message (`📓️close-ladder-2026-09-10.md:274`,
`📓️contributions-rearm-2026-09-10.md:195`, `📓️flow-host-ownership-2026-09-10.md:275`,
`📓️flow-eval-tick-address-2026-09-10.md:222`, `📓️guest-memory-2026-09-10.md:243` — "the first
`flowEvalTick` costs 25 s", a hot-path debt, not an evaluation one).

### 5.2 Kernel

```
$ ./semio_s_artifact_stdio_semio-<hash> brep::schema::diff::boolean
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 2502 filtered out; finished in 20.07s

$ ./semio_s_artifact_stdio_semio-<hash> brep::schema::inferences::validation_report
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 2499 filtered out; finished in 0.01s
```

### 5.3 The extension half — a real boolean, really yielding

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️evaluate-budget/🦀️.rs`, driving the REAL
`evaluate_invoke_json` against the REAL kernel over the kernel suite's own bored-box case (a 4×4×2
block centred on the origin with a radius-0.5 cylinder through it — the cheapest pair that still
reaches the general exact engine):

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-s-plugin-flow-extension-brep evaluate_budget
running 3 tests
test component::tests::evaluate_budget::a_long_set_operation_answers_within_the_wall_allowance_and_resumes ... ok
test component::tests::evaluate_budget::a_cancel_between_round_trips_retires_the_parked_evaluation ... ok
test component::tests::evaluate_budget::an_unbudgeted_operator_completes_in_one_round_trip ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 1.35s
```

The first law drives the operation at a ONE-MICROSECOND wall allowance and asserts, per round trip:
`unitsDone` never decreases, never exceeds `unitsTotal`, a working answer is `cancellable` with an
empty `outputJson` and a phase from the fixture's declared order — then that the run took at least the
fixture's declared minimum of round trips, and that the final `outputJson` is the operator's own out
dictionary, agreeing with an unbudgeted control run over fresh operands in a fresh kernel.

### 5.4 The requester half — the fold and the published status

`✏️s/…/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs`, replaying `⏱️evaluate-budget.json` through the
real command handler, the real `FlowEvalSession` and the real status projection:

```
$ RUST_MIN_STACK=67108864 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib budget
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 379 filtered out; finished in 0.18s
```

Covering: every fixture row end to end (seeds / re-arms / phase / labels en+de / inFlight /
evalUnits / ratio / cancellable / cancelAction), the whole declared job→surface phase projection
(including "an unknown tag is `computing`, never `idle`"), monotone progress across a real SEQUENCE of
round trips with a tick beginning between each, and that the cancel gesture reaches BOTH registries.

### 5.5 The cancel laws (updated for the second door)

```
$ RUST_MIN_STACK=67108864 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib cancel
running 7 tests
test standards::…::cancelled_and_stale_aba_initializers_retire_to_terminal_empty ... ok
test viewer::generation3d::component::status_contract_tests::the_viewer_declares_the_cancel_verb_its_status_names ... ok
test editor::generation3d::commands::cancel_preview_eval::tests::an_unaddressable_kernel_cancels_locally_and_emits_nothing ... ok
test editor::generation3d::commands::flow_tessellate_cancel_resolve::tests::the_cancel_acknowledgement_arms_nothing ... ok
test editor::generation3d::commands::flow_eval_resolve::budget::the_cancel_gesture_reaches_the_evaluation_registry_too ... ok
test editor::generation3d::commands::cancel_preview_eval::tests::the_cancel_gesture_obeys_its_fixture_end_to_end ... ok
test viewer::generation3d::component::status_contract_tests::a_viewer_cancel_dispatches_live_and_never_mutates_the_document ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 378 filtered out; finished in 0.21s
```

### 5.6 TypeScript twin (independent implementation)

`✏️s/…/✅️flow-eval-resolve/🧪️tests/🔬️budget/contract.ts` re-implements the fold, the ledger, the
latch and the status from the fixture's own PROSE — its phase vocabulary is written out by hand rather
than read from the fixture's `jobPhaseTags` table, because an oracle that read the answer key would
prove nothing — and replays the identical rows plus the identical monotone sequence.

```
$ bun ".🧬semio/…/PROCEDURAL-3D-END-TO-END/🔍️evaluate-budget-contract.ts"
OK generation3d evaluate-budget TS twin
```

### 5.7 Host laws (vitest)

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts "🔌️PluginRuntime" -t "shard-loss" --reporter=verbose
 ✓ … handlePluginShardLost delegates to ActivationRegistry.handleShardLost for EXACTLY the affected actorIds 2ms
 ✓ … handlePluginShardLost retires the app instances those actors hosted and announces exactly them 1ms
 ✓ … a lost shard that hosted no app instance announces nothing 0ms
 ✓ … buildShardClientOptions wires onShardLost to handlePluginShardLost … 0ms
      Tests  4 passed | 104 skipped (108)
```

The two new rows pin: an extension's request actor contributes nothing to announce; retiring is
idempotent; an instance that closed normally is forgotten so its later shard loss is silent.

`🧪️tests/📥️inbound-request/🟦️.ts` gains two laws for the typed fault (a shard-loss invocation answers
`extension.worker-lost` with `retryable: true`; a genuine guest refusal still answers
`extension.invoke-failed` with `retryable: false`).

```
$ SEMIO_TEST_LEVEL=full bunx vitest run --config vitest.config.ts "📥️inbound-request"
 Test Files  1 passed (1)
      Tests  20 passed (20)
```

(18 before this lane; the two new ones are the worker-lost pair.)

```
$ SEMIO_TEST_LEVEL=full bunx vitest run --config vitest.config.ts "🔬️engine-contract"
 Test Files  1 failed (1)
      Tests  1 failed | 582 passed (583)
```

Six of the seven failures this suite had when the lane started were the PREVIOUS lane's: the shell
now hands `invoke` an `AbortSignal` and the assertions still expected `{ originInstanceId }` alone.
Fixed here (`signal: expect.any(AbortSignal)`, with the reason in a comment). The one remaining
failure is a peer's in-flight `noteShellCommand` change (`inverseArgs` added to the action args) and
is untouched by this lane.

### 5.7b The `evaluate` wire, and the other extensions

```
$ cargo test -p semio-framework-os-flow --lib the_evaluate_wire
test extensions::wasm::tests::the_evaluate_wire_answers_every_fixture_row ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 218 filtered out

$ cargo test -p semio-s-plugin-flow-extension-brep -- --exact \
    component::tests::extension_bundle_extends_flow_and_evaluates_box \
    component::tests::evaluate_budget::{a_long_set_operation_…,a_cancel_…,an_unbudgeted_…}
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 3.84s
```

⚠️ The FULL `--lib` suites of `flow-extension-brep` (23 failed), `flow-extension-bim` (8) and
`flow-extension-math` (8) are currently red with `final Dictionary ownership must be explicitly
retired or owned by a cold boundary`. This is **not this lane**: it reproduces identically in
`flow-extension-list` (`component::tests::get_reads_index`) and `flow-extension-math`
(`component::tests::schema_component_round_trips_vector`), crates this lane never touched, and on
tests that never evaluate anything (`manifest_lists_bim_operators`). Every test this lane owns or
changed retires its dictionaries explicitly and passes.

### 5.8 Typecheck

```
$ bunx tsc -p tsconfig.json --noEmit      # react engine target
861 errors repo-wide, ZERO on any line this lane touched.
```

The errors reported in the three files this lane edits are all pre-existing and elsewhere:
`ShellHost` 2021/8332/8333/9044 (this lane edits ~1682–1765 and ~3486–3520), `PluginRuntime`
2664/2890 (this lane edits ~353–435, 2013, 2444), `engine-contract` 5256/7331/8577/10470 (this lane
edits 1285/1300). The count matches the ~1021 the cancellation lane recorded for the same program.

---

## 6. Runtime on 6018

`🐍️mesh-delivery-probe.mjs` with `SEMIO_PROBE_PICK="Sphere Cut With Torus,Box Fillet Preview"`,
outputs in `🗑️generated/evaluate-budget/recovery-2`.

### 6.1 What could and could not be proven live

The host TypeScript (§3, §4) ships in the served bundle and is live on reload — **that half is
proven below**. The budget (§2) is NOT: the served `procedural` and `flow-extension-brep` wasm are
the ones already staged, and the two halves are **one contract**. Republishing the brep extension
ALONE would make the served guest read a `{done:true,…}` envelope as the operator's out dictionary
and break every preview, so it was deliberately NOT republished. §2 therefore stops at native proof
(§5.2–§5.6), exactly as the brief anticipates for guest changes.

### 6.2 The headline: the session no longer dies

```
83294 error [DEBUG] shard 0 terminated by the host watchdog: the worker was silent for 17951 ms;
      outstanding: turn flow-extension-brep#request started 17976 ms ago; turn procedural#1 …
83294 error [DEBUG] PluginRuntime: shard 0 lost, restoring actors: procedural#1, flow-extension-brep#request
83294 error [DEBUG] PluginRuntime: plugin.actor-instance-lost — procedural#1 died with shard 0 and must be re-created
83294 warning [DEBUG] ShellHost: procedural#1 died with shard 0 — re-establishing the session on the rebuilt shard
…
84166 debug [DEBUG] cooperative-maintenance instance=2 turn=1 generation=1 status=1->1 …
```

The watchdog line is still there — correctly, because the served wasm is still the unbudgeted one
and that turn really was silent for 17.9 s. What is gone is everything that came after it:

```
$ grep -c "command-page-invalid" console.txt
0
```

against the P0 baseline's `action failed interactionHover … plugin.command-page-invalid` and
`action failed setActiveExample … plugin.command-page-invalid` on every later gesture. The session
came back as **instance 2** and kept working.

### 6.3 Both examples deliver meshes

```
[DEBUG] boot:                        [["window:procedural-main",0,0,…],["window:procedural-preview",3,3641,"idle",1,null]]
[DEBUG] pick:Sphere Cut With Torus:  [["window:procedural-main",0,0,…],["window:procedural-preview",1,88230,"idle",1,null]]
[DEBUG] pick:Box Fillet Preview:     [["window:procedural-main",0,0,…],["window:procedural-preview",1,19231,"idle",1,null]]
```

(`[surfaceId, meshCount, meshesJsonLen, phase, ratio, fault]`.) Final samples:

| pick | t | meshes | meshesJson | phase | ratio | fault |
|---|---|---|---|---|---|---|
| boot | 49.9 s | 3 | 3 641 | idle | 1 | — |
| Sphere Cut With Torus | 159.3 s | **1** | 88 230 | idle | 1 | — |
| Box Fillet Preview | 203.4 s | **1** | 19 231 | idle | 1 | — |

The P0 baseline was `meshCount: 0` for BOTH, with the second pick failing outright. `meshes ≥ 1` for
both is met.

The re-issued round trips after the recovery, from the console:

```
85746  extension request answered {capability: evaluate,     req: 13, turns: 1, bytes: 234}
…
119040 extension request answered {capability: evaluate,     req: 19, turns: 1, bytes: 162}
126654 extension request answered {capability: tessellate,   req: 20, turns: 1, bytes: 38749}
162250 extension request answered {capability: evaluate,     req: 21, turns: 1, bytes: 234}
171544 extension request answered {capability: tessellate,   req: 24, turns: 1, bytes: 8576}
```

### 6.4 Why the second boolean did NOT trip the watchdog

Request 19 (the re-issued `brep.bool.cut`) spent 98.1 s → 119.0 s — **21 s in one turn** — and no
second watchdog line appears. That is not this lane: a REBUILT shard's actors are fresh activations,
so `ShardClient.actorsPastFirstTurn` is empty and `evaluateShardLiveness` measures them against
`firstTurnTimeoutMs` (30 s) instead of `heartbeatTimeoutMs` (5 s) — the first-turn ladder
`📓️shard-termination-2026-09-10.md` §2.1 added. The unbudgeted boolean survives its FIRST turn on a
rebuilt shard by 9 s, and would not survive a second. This is exactly why §2 is still owed at
runtime: the budget is what removes the coin flip.

### 6.5 `extension.worker-lost` was not observed live, and why

The failing request 12 was logged by `dispatchInvokeExtensionEffect`'s own catch
(`[DEBUG] invokeExtension dispatch failed … req: 12n`), not by `runCapturedExtensionEffect`'s: the
REQUESTER's actor died on the same shard as the callee, so the completion could not be published at
all. The typed fault covers the case where the callee's worker dies and the requester survives (they
are on different shards); the same-shard case is answered by §4.2's whole-session recovery, which is
the better outcome. The fault mapping itself is pinned by the two vitest laws in §5.7.

---

## 7. What is still owed

1. **Restage the procedural plugin.** The requester half (the envelope fold, the evaluation ledger,
   the status projection, the second cancel door, the `budget`/`wallMicros` request fields) is proven
   natively and by the TS twin but is not in the served wasm. The brep extension half CAN be
   republished on its own — but the two halves are one contract, so until the guest is restaged the
   served app would read a `Working` envelope as an out dictionary. **Both must be staged together.**
2. **Fillet, offset/shell and sweep are still atomic.** They offer no `step_plan`, so they are
   evaluated in one call exactly as before. `Box Fillet Preview` costs 0.3 s natively, so nothing is
   urgent there; the extension point exists (`Operator::step_plan`) and costs no contract change.
3. **A cancelled boolean leaves its imprints in the body** (§2.4). Giving the kernel a transaction is
   a lane of its own.
4. **`check_solid_orientation` / `check_degenerate_geometry` are 3.2 s of the 4.5 s boolean** and are
   dominated by re-tessellating the same faces at a probe tolerance. Budgeting them made them
   preemptible; making them CHEAP (a shared probe-tolerance mesh cache) would make the whole preview
   an order of magnitude faster and is the single highest-value follow-up this measurement exposes.
5. **The coarsest single step is one face** of `face_area`/`face_volume_contribution` — ~500 ms for
   the three faces of `🍩️sphere-cut-with-torus` natively. That is the atomic unit of the algorithm
   (abandoning a face mid-quadrature throws its work away), so 8 ms interactivity is not reachable
   without item 4.

---

## 8. Files

Created:
- `✏️s/…/🧊️generation3d/…/🪆️subsets/✳️any/🧫️fixtures/⏱️evaluate-budget.json`
- `✏️s/…/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs` + `contract.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️evaluate-budget/🦀️.rs`
- `<ticket>/🔍️evaluate-budget-contract.ts`, this report

Changed:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🦀️.rs` + `🧪️body/🦀️.rs`
- `✏️s/…/🧊️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs`
- `✏️s/…/🧊️brep/🧬️schema/🔺️diff/🔀️boolean/🦀️.rs`
- `✏️s/…/🧊️brep/🧬️schema/⚙️engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` + `🧫️fixtures/🔁️extension-invocation-wire/🔣️.json` + `🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` + `🧪️tests/📥️inbound-request/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` + `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/…/🧊️generation3d/…/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`
- `✏️s/…/✅️flow-eval-resolve/🦀️.rs`
- `✏️s/…/🧫️fixtures/🛑️preview-cancel.json` + `🛑️cancel-preview-eval/🧪️tests/🔬️unit/🦀️.rs`
