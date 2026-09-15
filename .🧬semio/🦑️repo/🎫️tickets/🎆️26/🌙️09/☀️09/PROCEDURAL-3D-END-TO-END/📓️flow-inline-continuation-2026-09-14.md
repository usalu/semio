# Flow Inline Continuation — One Owning Publication Layer, One Hop Per Lane (2026-09-14)

Opus execution lane `flow-inline-continuation`. Lands §7 item **4** of
`📓️flow-tick-coalescing-2026-09-14.md`: the `flowEvalResolve` → `flowEvalTick` round-trip pair the
preview chain still paid **per dependency level**, and the routing obstacle that kept the coalescing
lane from removing it.

---

## 1. The hop that was left, and why it could not simply be deleted

After wave coalescing, one chain of N contributed dependency levels cost N+1 `flowEvalTick` hops: one
per level, plus a terminal walk. Each hop is a full host round trip measured at **1 391 ms** on React
(`📓️react-hop-latency-2026-09-14.md` §2.1 — `invoke` 442 ms + 2.09 × `refresh` 435 ms + `commit`
51 ms + `arm` 20 ms).

The hop is redundant by construction. When the LAST answer of a wave settles,
`FlowEvalSession::settle_window_extension` already leaves the window owing exactly one hop, and the
answer's own fold already holds everything that hop needs — the retained session, the document, the
config. The only thing it did NOT hold was the addressed preview window's **transient**:

* `flowEvalTick` was routed through `Generation3dFlowEvalWindowWork`, which owns
  `context.window_transient` and publishes the evaluation into it;
* `flowEvalResolve` / `flowTessellateResolve` were routed through the session-only work with
  `ArtifactToolPublicationLane::HostOnly` — no store lane at all.

So an inline continuation on the fold route would have advanced the chain and published **no
intermediate geometry and no advanced status pill** — and intermediate publication is exactly the
thing a user cancels out of. That is the obstacle §7 item 4 named.

---

## 2. The design: move the fold to the publisher, not the publication to the fold

**One window-addressed work owns every route of the chain that may publish.** The two folds joined
`Generation3dFlowEvalWindowWork` (and its viewer twin `Generation3dViewFlowEvalWindowWork`); nothing
was adapted and no second publication path exists.

| | before | after |
|---|---|---|
| `flowEvalTick` | window work, `WindowTransient` | unchanged |
| `flowEvalResolve` | session work, `HostOnly` | **window work, `WindowTransient`** |
| `flowTessellateResolve` | session work, `HostOnly` | **window work, `WindowTransient`** |
| `flowEvalRelease` | session work, `HostOnly` | unchanged |
| `flowTessellateCancelResolve` | session work, `HostOnly` | unchanged |

The last two stay off the layer deliberately: they address an extension **actor**, not a window, and
`retained_window_transient_target` refuses a window that has left the roster — a release must still
reach the geometry kernel after its window is gone.

`retained_window_transient_target` now answers for all three window-addressed routes off one helper
(`generation3d_flow_eval_window_address`), which is sound because
`reactor::extension_response_args` echoes the request's own `windowId`/`windowKindId` back onto the
response action: an answer names its window for exactly the reason a tick does.

### 2.1 The admission rule — the run job's own question, asked by the fold

`FlowEvalSession::inline_continuation_admitted(window_id, turn_started_us, now_us)`:

1. **not cancelled.** `begin_window_tick` retires the `cancelled` banner (work resuming is the one
   thing that may), so a continuation on a cancelled chain would not merely compute one wave too
   many — it would **un-cancel the run the user stopped**, from inside an answer that was already
   crossing when they stopped it. A cancel also defaults every latch, so `window_tick_owed` already
   answers `false`; the explicit guard is kept because those two facts must not be one accident apart.
2. **`window_tick_owed`** — literally `next_preview_eval_hop`'s `Dispatch` question. A fold may only
   take over a hop the scheduler would otherwise have dispatched, never invent one. The chain's
   SHAPE therefore cannot change; only who runs the hop.
3. **the turn still has wall** — `flow_eval_inline_continuation_fits`, pure in both instants.

A refused continuation **touches nothing**: the window still owes the hop it owed and the
`previewEval` run job dispatches it on its next step. Park is not a state — it is the absence of the
call. That is why there is no parked-continuation bookkeeping anywhere.

### 2.2 The 8 ms hold: one turn, one deadline

`flow_eval_tick_budget(turn_started_us)` replaces the nullary version. A walk that OPENS its own turn
passes `None` and gets the whole `FLOW_EVAL_TICK_ELAPSED_CEILING_US` (6 ms); a walk running inline
inside a fold's turn passes that turn's start and **inherits the same deadline** rather than opening
a fresh allowance. Without that, a chained continuation would stretch the interactive hold by one
whole ceiling per wave.

`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US = FLOW_EVAL_TICK_ELAPSED_CEILING_US / 3` (2 ms) is the
floor a walk needs to be worth starting: `EvalStepBudget::exhausted` always dispatches at least one
node before a deadline can stop it, so a continuation admitted with a sliver left would overrun by a
whole operator rather than yield.

`EvalStepBudget::deadline_us()` was added so a law can state the shared-deadline rule instead of
inferring it.

### 2.3 One wave per turn, structurally

A continuation that parks extension work stops there: the answers must cross to the host before the
next wave can be walked at all. So one guest turn can never run two waves of the same chain — the
bound is the chain's own shape, not a counter.

---

## 3. Hops per example

`hops` = `flowEvalTick` actions the HOST dispatches for one cold-cache example load in the edit lane.
One chain of N waves is N+1 hops: one per wave plus a terminal walk. With the continuation admitted
throughout, hop 1 is dispatched and hops 2..N+1 all run inline.

| example | contributed nodes | waves | before (u64 journey) | after coalescing | after inline |
|---|---:|---|---:|---:|---:|
| Rectangle Wire Preview | 1 | `[1]` | 3 | 2 | **1** |
| Hexagonal Mushroom Column | 3 | `[2,1]` | 3 | 3 | **1** |
| Box Fillet Preview | 2 | `[1,1]` | 6 | 3 | **1** |
| Box Shell Preview | 2 | `[1,1]` | 6 | 3 | **1** |
| Sphere Box Fuse | 3 | `[2,1]` | 7 | 3 | **1** |
| Rectangle Extrude Volume | 4 | `[2,1,1]` | 7 | 4 | **1** |
| Sphere Cut With Torus | 4 | `[2,1,1]` | 7 | 4 | **1** |
| Face Sweep Extrude | 4 | `[2,1,1]` | 8 | 4 | **1** |
| **eight examples** | 23 | — | **47** | **26** | **8** |

§8 measures this in a browser. The measured floor is **2** dispatched hops per edit example and
**1** per viewer example, not 1 — the edit lane's gesture (`setActiveExample`) arms one hop of its
own before the run job's, and both then run their chains inline. What the model gets exactly right is
the SHAPE: the count no longer depends on the graph's depth at all.

The tessellate half matters as much as the evaluate half: a mesh body arrives one CHUNK per round
trip and each chunk used to cost its own `flowEvalTick`. That is why `flowTessellateResolve` joined
the same layer on the same terms.

---

## 4. The fixture law

`🧫️fixtures/⏱️evaluate-budget.json` gains an `inlineContinuation` section — the language-agnostic
source of record — and its `answersArmNothing` law was rewritten to the new truth:

> A fold never DISPATCHES a hop. […] What it may do — and now does — is RUN that owed hop itself,
> inline, inside the answer's own guest turn. […] The admission question is the run job's own
> scheduling question, so the chain's shape cannot change; only who runs the hop.

The section declares `owningPublicationLayer`, the four admission clauses, `oneWavePerTurn`,
`hopIndexing`, the eight per-example wave shapes with their three hop columns, and three interference
rows:

| row | waves | interference | dispatched hops | inline waves |
|---|---|---|---:|---:|
| `cancel-between-waves-stops-the-chain` | `[2,1,1]` | cancel before hop 2 | 1 | 0 |
| `a-spent-turn-parks-and-the-host-round-trip-happens` | `[2,1,1]` | spent turn before hop 2 | 2 | 2 |
| `an-unadmitted-fold-leaves-the-latch-exactly-as-it-found-it` | `[1]` | spent turn before hop 2 | 2 | 0 |

Answered twice, sharing nothing but the fixture: Rust replays the ladder against the real
`FlowEvalSession` latch; the TypeScript twin rebuilds a `WindowLatch` from the fixture's admission
prose alone.

---

## 5. Files changed

| file | change |
|---|---|
| `🧰️framework/…/🧠️neural/⚙️engine/🦀️.rs` | `EvalStepBudget::deadline_us()` |
| `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` | `FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`; `flow_eval_tick_budget(turn_started_us)`; `flow_eval_inline_continuation_fits`; `FlowEvalSession::tick(host, turn_started_us)`; `FlowEvalSession::inline_continuation_admitted` |
| `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | `evaluate_tick(…, turn_started_us)`; fold docstrings state the continuation |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | `evaluate(…, turn_started_us)`; new `continue_inline` |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` | new `resolve` — fold + continuation |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/🔺️flow-tessellate-resolve/🦀️.rs` | new `resolve` — fold + continuation |
| `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | `Generation3dFlowEvalWindowWork` gains `tool_id` and both folds; `generation3d_flow_eval_window_address`; `GENERATION3D_FLOW_EVAL_WINDOW_TOOL_IDS`; `retained_window_transient_target`; both folds' lane → `WindowTransient` |
| `✏️s/…/🧊️generation3d/…/👁️viewer/🦀️.rs` | the read-only twin of all of the above, plus `generation3d_view_continue_inline`; the resolve work narrows to the two actor-addressed answers |
| `✏️s/…/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs` | `settle` takes the caller's `ActionMeta` — a window-addressed fold cannot be answered with a viewless meta |
| `✏️s/…/🌀️generation2d/…/🧵️preview-eval/🦀️.rs`, `✏️s/…/🌊️flow/…/⏱️flow-eval-tick/🦀️.rs` | `session.tick(host, None)` |
| `✏️s/…/🧊️generation3d/…/🧫️fixtures/⏱️evaluate-budget.json` | `inlineContinuation` section; `answersArmNothing` rewritten |
| `✏️s/…/✅️flow-eval-resolve/🧪️tests/🔬️budget/🦀️.rs` + `🟦️.ts` | the ladder law in both languages |
| `🧰️framework/…/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | three admission/cancel/wall laws, plus §7.1's settled-census law |
| `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` (§7.1) | `PreviewChainStatus::settled()` |
| `✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` (§7.1) | `computing` and the abort affordance stop reading a settled chain's stale run view |
| `✏️s/…/🧊️generation3d/…/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs` (§7.1) | the projection law over a `Running` run view |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🐍️run-settle-probe.mjs` | the two-pick browser reproduction of §7.1 |

### 5.1 One framework change this lane depends on and did not author

`🧰️framework/…/🔌️plugin/🦀️.rs`'s `plugin_dispatch_response_action` dispatched its continuation with
`view_state: None`. Because a response action is now a window-addressed route,
`retained_window_transient_target` faulted every fold with
`targeted window transient capture requires an exact ViewModel roster` — measured live on 6028 at
19:44 (`🗑️generated/flow-inline/console-boot/console.txt`). A peer landed the fix at **19:45:12**:
the continuation now carries the instance's LAST HOST VIEW, exactly as `pending_effects` does. This
lane's browser numbers are taken on a guest restaged AFTER that change.

---

## 6. Laws, with output

All Rust runs foreground, `RUST_MIN_STACK=33554432`, `--test-threads=1`.

### 6.1 New laws (all green)

`cargo test -p semio-framework-os-flow --lib -- host::tests::an_inline_continuation host::tests::a_cancelled_chain_is_never host::tests::every_dag_walk_of_one_guest_turn`

```
test host::tests::a_cancelled_chain_is_never_continued_inline_by_an_answer_that_was_already_crossing ... ok
test host::tests::an_inline_continuation_is_admitted_exactly_where_the_run_job_would_have_dispatched_a_hop ... ok
test host::tests::every_dag_walk_of_one_guest_turn_shares_one_wall_deadline_and_a_spent_turn_parks ... ok
test result: ok. 3 passed; 0 failed
```

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- budget::`

```
test …::flow_eval_resolve::budget::a_cancel_between_waves_and_a_spent_turn_each_hand_the_round_trip_back_to_the_host ... ok
test …::flow_eval_resolve::budget::every_example_chain_costs_one_dispatched_hop_once_the_folds_continue_it_inline ... ok
test …::flow_eval_resolve::budget::every_declared_job_phase_tag_projects_to_its_declared_surface_phase ... ok
test …::flow_eval_resolve::budget::progress_is_monotone_across_the_round_trips_of_one_evaluation ... ok
test …::flow_eval_resolve::budget::the_evaluate_budget_envelope_obeys_its_fixture_end_to_end ... ok
test …::flow_eval_resolve::budget::the_kernel_release_reaches_the_evaluation_registry_too ... ok
test result: ok. 6 passed; 0 failed
```

```
[DEBUG] inline continuation row cancel-between-waves-stops-the-chain: ChainLadder { dispatched_hops: 1, inline_waves: 0, continuations_after_cancel: 0 }
[DEBUG] inline continuation row a-spent-turn-parks-and-the-host-round-trip-happens: ChainLadder { dispatched_hops: 2, inline_waves: 2, continuations_after_cancel: 0 }
[DEBUG] eight example loads: 47 dispatched hops -> 8
```

### 6.2 TS twins

```
bun 🔍️evaluate-budget-contract.ts
  OK generation3d evaluate-budget TS twin
  OK generation3d inline-continuation TS twin
```

### 6.3 Regression suites

| suite | result | note |
|---|---|---|
| `semio-framework-os-kernel-neural-engine --lib` | **56 passed, 0 failed** | unchanged from the coalescing lane's 56/0 |
| `semio-s-artifact-procedural-generation3d --lib` (full) | **459 passed, 4 failed** | the coalescing lane's 3 (`generation_preview_is_one_app_transient…`, `two_instances_converge_disjoint_widget_moves`, `vcs_artifact_app_non_empty_retained_maintenance_swap…`) plus `every_panel_publishes_its_body_in_generate_mode_as_well_as_edit`, which lives in `📌️panels/🗿️artifact` — a file this lane never touched, modified in the working tree by the flow-window-artifact-tree lane |
| `--lib -- a_late_contributions_install` (native twin of the served shape) | **2 passed, 0 failed** | editor + viewer |
| `--test example-geometry` (kernel oracle) | **18 passed, 0 failed** | geometry untouched |
| `semio-framework-os-flow --lib -- host::` | **119 passed, 4 failed** | 2 are the coalescing lane's documented pre-existing set (`connect_ports_replaces_existing_incoming_on_same_input`, `delete_selection_removes_edge_selected_by_synapse_id_domain` — pure DAG edge selection, unreachable from an evaluator change); the other 2 (`hexagonal_mushroom_fixture_reports_extruded_solid_output`, `rectangle_extrude_fixture_evaluates_solid_output`) reach the kernel only through the UNBUDGETED `FlowHost::evaluate`, which no part of this change touches, and the authoritative geometry gate (`--test example-geometry`, 18/18, includes `hexagonal_mushroom_column_evaluates_to_the_analytic_prism`) is green — they sit in `📐️brep-geometry`/`🧠️neural`, both under heavy peer edit in the working tree |
| `cargo check` gen3d / gen2d / flow-plugin / flow-host | **0 errors** each | |

### 6.4 Two laws this change had to repair, and why that is the finding

`hex_column_boot_stays_inside_the_interactive_turn_budget` and
`a_viewer_tick_emits_extension_work_or_re_arms_but_never_settles_silently` both went red on
`targeted window transient capture requires an exact ViewModel roster`. The cause was the fixture,
not the change: `brep_extension::settle` answered extension invocations under `meta("local")` with no
view, modelling a shell that does not exist (its sibling `settle_with_meta` existed precisely because
"the shell answers a continuation with its own live view attached"). The two helpers are now ONE that
takes the caller's meta. **That red was the early warning for §5.1** — the same viewless-continuation
defect, in the harness first and in the real React shell minutes later.

---

## 7. What is NOT claimed

1. **§3's `after inline` column is a MODEL and §8's is the measurement, and they differ.** The model
   says 1 dispatched hop per example; the browser measures 2 in the edit lane and 1 in the viewer
   lane, because `setActiveExample` arms a hop of its own before the run job's. What the model gets
   right is that the count stops depending on graph depth. Quote §8, not §3, for hops.
2. **The native `--lib` harness cannot exhibit the contributed hop ladder.** A `--lib` binary LINKS
   the brep and math operator packs, so `evaluate` never parks an extension request there and only
   the tessellate half of the chain round-trips. Measured on `hex_column_boot`: with the continuation
   forced off, `turns=2 round_trips=2 eval_steps=2` (three runs); with it on, `turns=2 round_trips=2
   eval_steps=3` — the walk moved from a hop into the fold's own turn, but the dispatched count was
   already at its floor for that chain. The contributed-operator ladder is modelled by the fixture law
   and must be measured in a browser.
3. **Wave WIDTH is still bounded only by the existing budget** (`flow-tick-coalescing` §7 item 6);
   this lane did not narrow that.
4. **`nodesDone`/`nodesTotal` are still absent from `statusContract.progressKeys`**
   (`flow-tick-coalescing` §7 item 5) — untouched here.
5. **The framework continuation-view fix (§5.1) is not this lane's work.** It is load-bearing for
   every number in §8.
6. **The `previewEval` RUN still does not reach a terminal state promptly after a chain settles.**
   §7.1 fixes what a surface PUBLISHES about that, with a law; it does not fix the run's own
   finalize handshake, which is host-paced and belongs to `⏯️tool-run`. Measured symptom that
   remains: `toolRunFinalize` never appears in a whole journey's console.
7. **The wgpu twin was measured only by its own battery (§8.3), not by the journey probe**, which is
   React-only — so there is no wgpu HOP count, only a per-example geometry and status verdict.
8. **`accessibility:live` on wgpu is red and unattributed** (§8.3). It may or may not be downstream of
   §7.1; this lane did not chase it.

---

## 7.1 The defect the speed-up exposed: a spinner that outlived its work

Landing §2 turned the last thing that happens in a chain from a DISPATCHED `flowEvalTick` into an
extension answer's own guest turn. After it the guest has nothing further to say, so no further
`refreshUi` runs, so the `previewEval` run's terminal state is never rendered back — and
`preview_scene_status_json` / `preview_progress_status_json_for` raised `computing` and the
`toolRunAbort` affordance straight off that stale `ToolRunView`:

```
window:procedural-preview  phase=idle ratio=1 inFlight=0 nodesDone=7 nodesTotal=7
                           computing=true cancellable=true      ← forever
```

Reproduced deterministically on 6024 in two picks (boot → `No example` → `Hexagonal Mushroom Column`,
`🐍️run-settle-probe.mjs`, raw in `🗑️generated/flow-inline/run-settle2/`), and it cost 2 of 23 journey
rows in one run and 4 of 23 in another.

**The rule, at its owner.** `PreviewChainStatus::settled()` (`🌊️flow/🖥️host/🦀️.rs`): a chain that owes,
awaits and runs nothing, and whose own census accounts for every node it published, has settled. The
chain is the LIVE ledger; a `ToolRunView` is a host artifact delivered on a render and therefore lags
its own job. So the run may raise `computing`/`cancellable` only while the chain has NOT settled —
before the first hop publishes a census the run legitimately knows more (it is the only thing that
knows a gesture started one), and after the last one it knows less. A census of ZERO nodes is
deliberately not settled, which is exactly that opening window.

This is not cosmetic: `toolRunAbort` offered beside `7/7, ratio 1.0, inFlight 0` stops nothing,
because nothing is running. A false affordance is as much a defect as a false spinner.

**Laws** (both green, output in §8.4):
* `a_settled_chain_census_outranks_a_lagging_run_view_and_an_empty_one_does_not`
  (`🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`) — the predicate itself, including the empty-census counter-case.
* `a_settled_chain_publishes_no_spinner_and_no_abort_however_stale_the_run_view_is`
  (`👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs`) — the real projection, driven with a `ToolRunState::Running`
  view across all three windows (no census → working → settled).

---

## 8. Measured in a browser

React, port 6024, guest restaged from this tree, `🐍️journey-probe.mjs`.
**23/23 converged, every mesh oracle green** (`🗑️generated/flow-inline/journey4/`).

`hops` = `performInvocation settled {"actionId":"flowEvalTick"}` counted per journey row — the same
instrument and the same probe as the before-column, which is re-counted here from the stored
`🗑️generated/react-u64/journey/` run rather than quoted.

### 8.1 Hops per example

| journey row | before | after |
|---|---:|---:|
| boot | 7 | **2** |
| edit: No example | 2 | 2 |
| edit: Hexagonal Mushroom Column | 3 | **2** |
| edit: Rectangle Extrude Volume | 7 | **2** |
| edit: Sphere Cut With Torus | 7 | **2** |
| edit: Box Fillet Preview | 6 | **2** |
| edit: Sphere Box Fuse | 7 | **2** |
| edit: Face Sweep Extrude | 8 | **2** |
| edit: Rectangle Wire Preview | 3 | **2** |
| edit: Box Shell Preview | 6 | **2** |
| generate-added | 2 | 2 |
| viewer-role | 5 | **1** |
| view: No example | 1 | 1 |
| view: Hexagonal Mushroom Column | 6 | **1** |
| view: Rectangle Extrude Volume | 7 | **1** |
| view: Sphere Cut With Torus | 6 | **1** |
| view: Box Fillet Preview | 5 | **1** |
| view: Sphere Box Fuse | 6 | **1** |
| view: Face Sweep Extrude | 7 | **1** |
| view: Rectangle Wire Preview | 2 | **1** |
| view: Box Shell Preview | 5 | **1** |
| **whole 23-step journey** | **108** | **32** |
| eight edit examples | 47 | **16** |
| eight viewer examples | 44 | **8** |

**The count is now FLAT.** Before, a row cost 3–8 hops depending on how deep its graph was; after, it
is 2 in the edit lane and 1 in the viewer lane for every example — `Face Sweep Extrude` (three
contributed levels) costs exactly what `Rectangle Wire Preview` (one) costs. That is the shape §3
predicted; the constant is 2 rather than 1 in the edit lane because `setActiveExample` arms a hop of
its own before the run job's, and the chain then runs inline from whichever ran first.

Priced at the peer lane's measured 1 391 ms/hop (`📓️react-hop-latency-2026-09-14.md` §2.1), the 76
removed hops are ≈**106 s** off one 23-step journey.

### 8.2 Seconds per row (same two runs, probe wall clock)

| row | before | after |
|---|---:|---:|
| boot | 11 | **8** |
| edit: Hexagonal Mushroom Column | 4 | **4** |
| edit: Rectangle Extrude Volume | 9 | **6** |
| edit: Sphere Cut With Torus | 8 | **6** |
| edit: Box Fillet Preview | 7 | **5** |
| edit: Sphere Box Fuse | 8 | **6** |
| edit: Face Sweep Extrude | 10 | **6** |
| edit: Rectangle Wire Preview | 4 | **4** |
| edit: Box Shell Preview | 6 | **5** |
| eight viewer examples | 4 each | **3–4 each** |

The probe samples once per second and requires three stable samples, so ~3 s of every row is the
instrument, not the app; the edit lane's deep examples are where the change is visible (9→6, 10→6).

### 8.3 wgpu

`activate-generation3d-wgpu-dev` restaged from this tree (12m11s, 5/40 cache hits — the wgpu guest
had not been rebuilt for a while, so this run also carries a batch of peers' changes), then
`bun 🐍️wgpu-battery.mjs --only=examples,status-a11y-i18n` on 6118 behind the
`wgpu-batter[y]|wgpu-.*-pro[b]e` quiet gate (which took ~10 min to clear). Raw in
`🗑️generated/wgpu-verify/` (the battery's own root; it ignores `SEMIO_PROBE_OUT`).

* **`examples`: every row green, `pageerrors=0`.** Both surfaces, all eight examples, each checked
  against its committed mesh ids and bounding box — e.g. `viewer: sphere-cut-with-torus publishes the
  committed meshes and box … box {min:[-2.1006,-2.1357,-2.2], max:[2.1475,2.1357,2.2]}`, `viewer:
  box-shell-preview … ids:["eval-shell@solid#0"]`. Editor rows likewise. The chain is renderer-neutral
  by construction and this is the measurement that says so.
* **`status-a11y-i18n`: 8 of 9 steps green.** The two that matter here both pass:
  `status:pill-while-computing` (`Computing · 0/1 (0%)`, `computing: true`, a published ratio) and
  `status:settled` (`phase: "idle"`). So §7.1 did not flatten the live pill — it still goes computing
  → settled on wgpu.
* **One red: `accessibility:live`.** After the example-switch gesture the accessibility mirror is
  byte-identical — `nodeCount 50 → 50`, `labels 30 → 30`, `arrived: []`, `left: []` — and the probe
  requires the mirror to move. This step was green in the previous full battery (22:42) and is red
  now. It is **not attributed**: the mirror carries window controls (`LOD`, `Show`, the two canvas
  labels), which an example switch does not change, and this run restaged a large batch of peer work
  alongside §7.1 — but §7.1 does remove a `computing` publication sooner, so a mirror that carried a
  transient pill node would sample identically at both ends. The a11y-live lane
  (`📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md`) owns that mirror; this lane flags it rather than
  guessing.
* The scoreboard's other three reds (`port-fit`, `generate-add`, `world3d-editor`) are stale entries
  from the 22:42 full battery that this two-lane run did not re-execute, not results of it.



### 8.4 §7.1's laws, with output

```
cargo test -p semio-framework-os-flow --lib -- host::tests::a_settled_chain_census
test host::tests::a_settled_chain_census_outranks_a_lagging_run_view_and_an_empty_one_does_not ... ok
[DEBUG] chain settled: working=false half=false censusFree=false done=true

cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- a_settled_chain_publishes_no_spinner
test viewer::…::status_contract_tests::a_settled_chain_publishes_no_spinner_and_no_abort_however_stale_the_run_view_is ... ok
[DEBUG] settled-vs-stale-run status: {"cancellable":false,"progress":{"inFlight":0,"nodesDone":3,"nodesTotal":3,"ratio":1.0,…}}
```

Live before/after of the same reproduction (`🐍️run-settle-probe.mjs`, boot → `No example` →
`Hexagonal Mushroom Column`, 6024):

```
before (🗑️generated/flow-inline/run-settle2/)  computing=true  phase=idle ratio=1 nodesDone=7/7 cancellable=true   ← 40 s, never clears
after  (🗑️generated/flow-inline/run-settle4/)  computing=null  phase=idle ratio=1 nodesDone=7/7 cancellable=false
```

### 8.5 Regression suites after §7.1

| suite | result |
|---|---|
| `semio-s-artifact-procedural-generation3d --lib` | **459 passed, 4 failed** (the 3 pre-existing + 1 peer-owned panel red; see §6.3) |
| `semio-framework-os-flow --lib -- host::` | **119 passed, 4 failed** (see §6.3) |
| `semio-framework-os-kernel-neural-engine --lib` | **56 passed, 0 failed** |
| `--test example-geometry` | **18 passed, 0 failed** |
| `bun 🔍️evaluate-budget-contract.ts` | evaluate-budget twin OK, inline-continuation twin OK |
| `bun 🔍️preview-status-contract.ts` | OK |
