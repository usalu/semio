# Flow Tick Coalescing — One Hop Per Dependency Level, One Monotone Ledger (2026-09-14)

Opus execution lane `flow-tick-coalescing`. Owns §3 items **3** (reduce the flow graph's own per-node
hop count) and **4** (fix the backwards progress ratio) of
`📓️react-perf-ceilings-audit-2026-09-14.md`.

---

## 1. The defect, at file:line

### 1.1 One host round trip per CONTRIBUTED NODE, not per dependency level

`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs`,
`Evaluator::evaluate_channels_budgeted` — the topological walk every `flowEvalTick` runs:

```rust
match dispatch(&neuron.kind, &merged) {
    Err(EvalError::PendingExtension { extension_id, operator_id, node_hash }) => {
        return Ok(BudgetedEval { … pending_extension: Some(PendingExtensionEval { … }) });
    }
```

The walk **returned at the first park**. `BudgetedEval.pending_extension` was an `Option`, the flow
host's `FlowHost::pending_extension_eval` was an `Option`, and `evaluate_tick`
(`✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs`) turned that one `Option` into one
`ExtensionInvocation`. So a graph with N contributed operators cost N park hops plus a terminal one
— **whatever the graph's shape**. Two operators that could not possibly interfere (`sphere` and
`torus`, `rect` and `vector`) still waited for each other.

This is not a cheap loop: every brep AND math operator in generation3d is CONTRIBUTED by a plugin at
runtime (`setContributions`) and never linked into the guest, so each park is a full host round trip
— measured at roughly **1 s apiece** once guest compute and mesh transfer were both fixed to
sub-second cost (`📓️react-perf-ceilings-audit-2026-09-14.md` §2a, §2b, §2d). §1's table there:
`sphere-cut-with-torus` 7 hops / 8.0 s, `face-sweep-extrude` 8 hops / 10.1 s, against
`rectangle-wire-preview` 3 hops / 4.1 s. Wall time tracked hop count almost exactly.

`FLOW_EVAL_TICK_STEP_BUDGET = 512` was never the limiter — a seven-node graph is far under 512. The
`Option` was.

### 1.2 The census that could not move, so the ratio could not either

`🌊️flow/🖥️host/🦀️.rs`, `build_flow_status_json`:

```rust
if dirty.contains(id) && !remaining.is_empty() {
    widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Stale));
    continue;
}
```

`FlowEvalSession::preview_chain_status` counts a node DONE when its census row is anything but
`queued`/`computing`/`stale`. But `dirty` is measured against the chain's **frozen** baseline —
`evaluate_step` only advances `previous_snapshot` when the whole walk completes — so every node this
evaluation touches stays dirty until the very last hop. A node the walk had already recomputed was
therefore `stale`, not `ok`, and `nodes_done` was **the count of nodes the evaluation never touched**:
constant from the first hop to the last, then a jump to `nodes_total`.

The wgpu progress lane's chain ledger (`📓️wgpu-progress-visibility-2026-09-14.md` §5.1) had already
made the published fraction **monotone** — it was just monotone and FLAT. The user-visible reading of
a flat bar is the same one the backwards bar produced: nothing is happening.

The backwards half is genuinely fixed and stays fixed: the budgeted-eval ledger
(`preview_eval_status`, `🦀️.rs:3471`) still aggregates only its live rows, but
`preview_progress_status_json_for` (`🧵️preview-eval/🦀️.rs`) no longer prices the ratio off it while
the chain has a census. This lane did not re-open that; it made the census the ledger actually
advance, and pinned the whole sequence with laws in both languages.

---

## 2. The fix

### 2.1 A topological WAVE per hop (`🧠️neural/⚙️engine/🦀️.rs`)

* `BudgetedEval.pending_extension: Option<PendingExtensionEval>` → `pending_extensions: Vec<…>`.
* `PendingExtensionEval` gains `neuron_id`: a request now names the node it belongs to, so a census
  can say which nodes are outstanding at their plugin instead of guessing from the head of a list.
* The walk **gathers instead of returning**: a park records the request, marks the neuron `parked`,
  charges one unit of `spent` (a park IS routed work, so the existing dispatch/deadline budget still
  bounds a wave) and continues to the next neuron in topo order.
* New `waits_on_parked(tree, parked, neuron_id)`: a neuron drawing an input from a parked — or
  already blocked — neuron is itself marked parked and **never dispatched**. This is the
  correctness half, not an optimisation: `collect_neuron_input` silently skips a source with no
  output (`else { continue }`), so dispatching a node behind a parked answer would hand the operator
  a half-built input and cache a wrong answer under a hash claiming to describe the real one.
* `budgeted_remaining_from` → `budgeted_remaining(order, from_index, parked, dirty)`: parked nodes
  sit BEFORE the stop index, so they are named explicitly and **unconditionally** — the old
  `dirty.contains(id)` filter would have dropped a parked non-dirty node from `remaining` and let
  the chain call itself converged while an answer was still crossing.

Independence is what makes a wave safe, and it is structural: no member of a wave consumes another
member's output, because a member is only walked once every one of its inputs was already produced.

### 2.2 The flow host carries the wave (`🌊️flow/🖥️host/🦀️.rs`)

`FlowHost::pending_extension_evals: Vec<…>`, `take_pending_extension_evals() -> Vec<…>`, and the
retirement ladder drains the vector one request per page.

**No latch change was needed** — and that is the tell that this was the design all along.
`FlowEvalSession`'s per-window tick latch already counted a fan-out: `note_window_extensions_in_flight`
takes a `count`, and `settle_window_extension` hands the re-arm to whichever answer lands **last**
("so the LAST answer of a fan-out emits exactly one re-arm and the earlier ones emit none"). The
`Option` in the evaluator was the only thing keeping the fan-out at one.

### 2.3 The census advances (`build_flow_status_json`)

* `Computing` is **every member of the live wave** (`host.pending_extension_evals`' `neuron_id`s),
  falling back to `remaining.first()` when nothing is parked (a budget-exhausted local walk).
* A dirty node the walk has already passed is `Ok`. The `Stale` branch is gone: a node leaves
  `remaining` exactly once per chain and never returns to it, so "absent from `remaining` while
  dirty" is precisely "recomputed by this chain". `NodeEvalStatus::Stale` stays in the wire schema —
  it is still parsed by `set_node_statuses_from_json` and still matched by `preview_chain_status` —
  but the walk-derived census no longer produces it.

### 2.4 The tessellate branch

`evaluate_tick` emitted `tessellate` in the `else` of a single `if let Some(pending)`. It now reads
`if extension_invocations.is_empty() && !more` — same rule ("tessellate only once the evaluation owes
nothing"), expressed over a wave instead of an `Option`.

### 2.5 Call sites

The identical wave loop lands in all three surfaces that drive the chain: generation3d
(`🧵️preview-eval/🦀️.rs`), generation2d (`🌀️generation2d/…/🧵️preview-eval/🦀️.rs`) and the flow plugin's
own `⏱️flow-eval-tick` command.

---

## 3. Hops per example, before and after

`hops` = `flowEvalTick` settles for one example load in the EDIT lane. Two independent before-columns,
both on this tree: `📓️react-perf-ceilings-audit-2026-09-14.md` §1 (from
`🗑️generated/react-u64/journey/console.txt`) and the freshly instrumented
`📓️react-hop-latency-2026-09-14.md` §2.1 (from `🗑️generated/react-hop/before/`, which also prices a hop
at **1 391 ms wall**: `invoke` 442 ms + 2.09 × `refresh` 435 ms + `commit` 51 ms + `arm` 20 ms — three
serialized guest turns and nothing else).

**After** is the count the coalesced chain is now STRUCTURALLY bounded by: one park hop per contributed
dependency LEVEL that still has a cache-missed node, plus one terminal hop. Levels are read off each
example's own graph (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`); every brep AND math operator is
contributed, so every one of them is a round trip. It is a model, not a browser measurement — see §7.

| example | contributed nodes | levels | before (u64 journey) | before (hop-latency) | after (model, cold cache) |
|---|---:|---:|---:|---:|---:|
| Rectangle Wire Preview | 1 (`rect`) | 1 | 3 | 3 | **2** |
| Hexagonal Mushroom Column | 3 (`profile`, `extrusion-axis`, `extrude`) | 2 | 3 | 3 | **3** |
| Box Fillet Preview | 2 (`box`, `fillet`) | 2 | 6 | 6 | **3** |
| Box Shell Preview | 2 (`box`, `shell`) | 2 | 6 | 6 | **3** |
| Sphere Box Fuse | 3 (`sphere`, `box`, `fuse`) | 2 | 7 | 7 | **3** |
| Rectangle Extrude Volume | 4 (`rect`, `vector`, `extrude`, `volume`) | 3 | 7 | 8 | **4** |
| Sphere Cut With Torus | 4 (`sphere`, `torus`, `cut`, `volume`) | 3 | 7 | 7 | **4** |
| Face Sweep Extrude | 4 (`rect`, `vector`, `face`, `extrude`) | 3 | 8 | 8 | **4** |
| **eight examples** | 23 | — | **47** | **48** | **26** |

Hexagonal Mushroom Column is already at 3 before the change because the BOOT example is Hexagonal: its
nodes are cache hits by the time the edit lane switches back to it. That row is the shape every other
row now takes for free.

Waves for the two examples the audit named as the worst:

* `sphere-cut-with-torus`: `{sphere, torus}` → `{cut}` → `{volume}` — **3 waves for 4 nodes**.
* `face-sweep-extrude`: `{rect, vector}` → `{face}` → `{extrude}` — **3 waves for 4 nodes**.

**Priced at the peer lane's measured 1 391 ms/hop**, the eight example loads drop from 48 hops
(≈66.8 s) to 26 (≈36.2 s): **22 hops, ≈30 s off a 79.3 s ten-step run**. That arithmetic is the honest
upper bound of this change on its own — it assumes only that a removed hop costs what a measured hop
costs, and it does not touch the 1 391 ms itself, which lane `react-guest-turn-cost` owns.

**The ≤3 target is met for five of eight examples, not all eight.** Three examples are LINEAR chains of
three contributed levels, and a wave cannot compress a linear chain: `cut` cannot start before `sphere`
answers. `levels + 1` is the floor this fix can reach, and those three sit on it. §7 item 4 names the
one design that goes below it and why this lane did not land it.

---

## 4. The ratio-sequence law

The chain census is the numerator; the rule is stated once, in the fixture, and answered twice.

**`🧫️fixtures/🛑️preview-cancel.json` → new `chainCensus` section** — the `sphere-cut-with-torus`
chain replayed hop by hop, with `censusRule`, `unitsRule` and `waveRule` written as prose an
implementation in any language can re-derive:

| step | working | nodesDone | nodesTotal | inFlight | wave | units | ratio |
|---|---|---:|---:|---:|---|---|---:|
| armed | true | 2 | 6 | 0 | — | 2/6 | 0.333 |
| wave-1-parked | true | 2 | 6 | 2 | `sphere`, `torus` | 2/6 | 0.333 |
| wave-2-parked | true | 4 | 6 | 1 | `cut` | 4/6 | 0.667 |
| wave-3-parked | true | 5 | 6 | 1 | `volume` | 5/6 | 0.833 |
| settled | false | 6 | 6 | 0 | — | 6/6 | 1.000 |

The law, asserted identically in Rust and TypeScript:

1. `ratio[i+1] >= ratio[i]` across the whole sequence — **monotone**;
2. `ratio[last] > ratio[first]` — it **moves** (a monotone bar that never moves is the §1.2 defect);
3. `nodesDone` never shrinks;
4. a **working** chain never publishes `1.0` — "done" is the one answer live work may not give;
5. `inFlight == wave.length` at every step — every parked node of a wave is one answer in flight;
6. the wave widths are `[2, 1, 1]` — four contributed nodes across three levels cost THREE waves.

**One owning ledger, both renderers.** Nothing in the projection was forked for React: the fraction
comes from `FlowEvalSession::preview_chain_status` through `preview_progress_status_json_for`, and
both shells read it off the same `World3dComputeStatusV1` contract (`phase`, `unitsDone`,
`unitsTotal`, `inFlight`, `ratio`, plus `nodesDone`/`nodesTotal`). The Rust law drives the real
`PreviewChainStatus`; the TS twin rebuilds `units()`/`ratio()` from the fixture's prose alone and
shares nothing with the guest but the fixture.

---

## 5. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` | wave gather in `evaluate_channels_budgeted`; `BudgetedEval.pending_extensions: Vec`; `PendingExtensionEval.neuron_id`; new `waits_on_parked`; `budgeted_remaining_from` → `budgeted_remaining` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | `pending_extension_evals: Vec`; `take_pending_extension_evals`; retirement ladder; `build_flow_status_json` wave-`Computing` + no walk-derived `Stale` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/🧵️preview-eval/🦀️.rs` | `evaluate_tick` emits one invocation per wave member; tessellate branch over the wave |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/🧵️preview-eval/🦀️.rs` | same wave loop |
| `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` | same wave loop |
| `✏️s/…/🧊️generation3d/…/🧫️fixtures/🛑️preview-cancel.json` | new `chainCensus` section (the ratio-sequence law's fixture) |
| `✏️s/…/🧊️generation3d/…/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs` | Rust ratio-sequence law + `ChainCensus` fixture binding |
| `✏️s/…/🧊️generation3d/…/🧪️tests/🔬️status-contract/🟦️.ts` | TS twin: `ChainLedger` model + the same law |
| `✏️s/…/🧊️generation3d/…/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🟦️.ts` | budget twin taught the chain-phase rule the fixture already declared (it had gone stale) |
| `🧰️framework/…/🧠️neural/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` | two wave laws |
| `🧰️framework/…/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | three laws: census advances, wave paints `computing`, wave-sized cancellation |
| `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🔍️evaluate-budget-contract.ts` | runner path repaired (`contract.ts` → `🟦️.ts`; it had not been runnable) |

---

## 6. Laws, with output

All Rust runs foreground, `RUST_MIN_STACK=33554432`, `--test-threads=1`. Raw output under
`🗑️generated/flow-coalesce/`.

### 6.1 New laws (all green)

`cargo test -p semio-framework-os-kernel-neural-engine --lib`

```
test component::tests::a_budgeted_walk_parks_one_whole_wave_and_never_a_node_behind_a_parked_answer ... ok
test component::tests::a_two_level_contributed_graph_converges_in_two_waves ... ok
[DEBUG] flow eval wave: parked=["left", "right"] remaining=["join", "left", "right"]
[DEBUG] flow eval waves: [["left", "right"], ["join"]]
```

`cargo test -p semio-framework-os-flow --lib`

```
test host::tests::the_node_census_advances_as_a_chain_walks_and_never_calls_a_recomputed_node_stale ... ok
test host::tests::a_coalesced_tick_parks_a_whole_wave_and_paints_every_member_computing ... ok
test host::tests::cancelling_a_coalesced_wave_retires_every_parked_answer_and_late_settles_arm_nothing ... ok
[DEBUG] flow wave census: parked=["left","right"] census=[("add","ok"),("left","computing"),("preview","ok"),("right","computing"),("slider","ok")]
[DEBUG] flow node census nodes_done per hop: [2, 3, 4]
[DEBUG] wave cancel: retiredTessellations=0 rearms=0
```

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib`

```
test viewer::generation3d::component::status_contract_tests::the_published_progress_ratio_is_monotone_and_advances_across_one_evaluation ... ok
```

### 6.2 Regression suites

| suite | result | note |
|---|---|---|
| `semio-framework-os-kernel-neural-engine --lib` | **56 passed, 0 failed** | was 54/0 (`🗑️generated/suite-reds/neural.txt`, 09-13); +2 are this lane's |
| `semio-framework-os-flow --lib -- host::` | 113 passed, **3 failed** (107/4 on the last re-run) | the same 3 fail with the walk toggled back to park-and-return (§6.3) and 2 of the 3 fail in ISOLATION with an unrelated `🌱️value/🗂️ordered` panic — pre-existing. The 4th appeared late: `connect_ports_replaces_existing_incoming_on_same_input` now faults `IncompatiblePortTypes { source: "note@text", target: "add@a" }`, i.e. the peer port-type validation that arrived with `NodeGraphOperatorChannelRecord.value_types` (§8 item 2) — graph editing, no evaluator path |
| `semio-s-artifact-procedural-generation3d --lib` (full) | 456 passed, **3 failed** | byte-identical failure set to `🗑️generated/suite-reds/gen3d-after.txt` (09-13, 446/3): `generation_preview_is_one_app_transient…`, `two_instances_converge_disjoint_widget_moves`, `vcs_artifact_app_non_empty_retained_maintenance_swap…` — the latter two fault inside a peer's in-flight VCS envelope-decoder refactor |
| `--lib -- a_late_contributions_install` (native twin of the served shape) | **2 passed, 0 failed** | editor + viewer |
| `--test example-geometry` (kernel oracle) | **18 passed, 0 failed** | geometry unchanged by the coalescing |
| `cargo check -p semio-s-artifact-procedural-generation2d --features component-app-assembly` | **0 errors** | |
| `cargo check -p semio-s-artifact-flow-flow` | **0 errors** | |

### 6.3 The counterproof for the three flow-host reds

Rather than assert they were pre-existing, the walk was temporarily toggled back to
park-and-return-at-the-first-request and the same suite re-run: identical 3 failures, identical
messages. The toggle was then removed. `delete_selection_removes_edge_selected_by_synapse_id_domain`
is pure DAG edge selection and cannot be reached by an evaluator change at all.

### 6.4 TS twins

```
bun 🔍️preview-status-contract.ts
  generation3d preview-status surfaces=editor:procedural-preview generate:generation3d-generate-preview
  viewer:procedural-view-preview states=idle,computing,faulted-evaluate,faulted-unaddressable-kernel,cancelled
  cancelAction=toolRunAbort
  OK generation3d preview-status TS twin

bun 🔍️evaluate-budget-contract.ts
  OK generation3d evaluate-budget TS twin
```

The evaluate-budget twin was **not runnable on arrival**: its ticket runner imported
`🔬️budget/contract.ts`, a path that no longer exists (the twin is `🟦️.ts`). With the path repaired
it failed immediately — the wgpu progress lane had rewritten three fixture rows from
`phase: "idle", ratio: 1.0` to the chain phase (`📓️wgpu-progress-visibility-2026-09-14.md` §5.4)
without teaching the TS twin the chain-ledger rule. The twin now models it from the fixture's own
prose (a working chain with no node census publishes `computing` and `0.0`, never `idle`/`1.0`) and
the budget fixture holds in both languages.

---

## 7. What is NOT claimed

1. **No browser measurement — though the change IS staged and served.** The `hops after` column in §3
   is a model derived from each example's graph, not a `🐍️journey-probe.mjs` run. This lane did not
   restage: the gate (`react-batter[y]|-pro[b]e\.(mjs|ts)` quiet) never cleared across its whole
   window — peers ran `react-battery`, `react-hop-cost-probe`, `menu-dump-probe`, `flow-window-probe`,
   `io-surface-probe`, `status-parity-probe`, `react-gap-probe`, `interaction-matrix-probe` and
   `graph-keyboard-nav-probe` back to back, and all seven React serves (6018, 6021–6026) are claimed
   by other lanes. The **coordinator** restaged at 19:00–19:01 (`📓️status.md`,
   `🗑️generated/restage-retry.txt`) and that restage carries this guest change, so the wave is live on
   every React serve — **but nobody has yet counted hops against it.** The measurement is one command:
   `SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d bun 🐍️journey-probe.mjs`, then count
   `performInvocation settled {"actionId":"flowEvalTick"}` per step in `console.txt`, exactly as §1 of
   the perf-ceilings audit and §2.1 of the hop-latency report did. Nothing in §3's "after" column has
   been observed in a browser by this lane.
2. **No wgpu twin run.** `bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n,examples` was not run: it
   needs its own `activate-generation3d-wgpu-dev` restage plus a quiet 6118 shared by three wgpu
   lanes, and the React gate never cleared to take the restage slot first. The wgpu shell reads the
   ratio through the same `World3dComputeStatusV1` contract and the same `preview_chain_status`, so
   the change is renderer-neutral **by construction** — but that is an argument, not a measurement.
3. **≤3 hops for every example is NOT achieved.** Five of eight reach it; the three three-level
   chains land at 4. A wave cannot compress a linear dependency chain.
4. **The remaining hop per level is a `flowEvalResolve` → `flowEvalTick` pair, and a PEER LANE is
   removing it on top of this change.** When the last answer of a wave settles,
   `settle_window_extension` already arms the next tick; the guest could run that tick INLINE inside
   the `flowEvalResolve` command and park the next wave with no host round trip. This lane did not
   land it — `flowEvalResolve` is routed `ArtifactToolPublicationLane::HostOnly` while the tick
   publishes into the addressed preview window's retained transient through
   `Generation3dFlowEvalWindowWork`, so an inline continuation there would advance the chain but
   publish no intermediate geometry, which is exactly what a user cancels out of. While this lane
   ran, a peer landed `flow_eval_tick_budget(turn_started_us: Option<u64>)`,
   `FlowEvalSession::tick(host, turn_started_us)`, `FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US` and
   `inline_continuation_admitted` in the same file — an inline continuation that shares ONE turn
   deadline across however many waves it chains — and wired it through
   `take_pending_extension_evals`, i.e. on top of the wave. The two compose: waves cut the number of
   LEVELS, the inline continuation cuts the round trips per level. Test call sites in
   `🖥️host/🧪️tests/🔬️unit/🦀️.rs` were updated to the new two-argument signature (`None`, a walk that
   opens its own turn) as part of this lane so the suite builds again.

5. **`nodesDone`/`nodesTotal` are not in the fixture's `progressKeys` list.** The projection publishes
   them (`preview_progress_status_json_for`) and the new `chainCensus` section pins their behaviour,
   but `statusContract.progressKeys` still names only the six original keys. Both twins assert
   `includes`, so nothing fails; a later lane should decide whether the census keys belong in that
   declared list.
6. **Wave WIDTH is bounded only by the existing budget.** A park costs one unit of
   `EvalStepBudget.dispatches` (512 per tick) and is consulted against the same
   `FLOW_EVAL_TICK_ELAPSED_CEILING_US` wall deadline, so the 8 ms reactor hold is respected — a park
   does no compute. What is NOT measured is whether a very wide wave (say 50 independent operators)
   would exceed a worker's admission slots downstream of the tick. Today's widest example wave is 2.
7. **`NodeEvalStatus::Stale` now has no Rust producer.** It remains in the wire schema, is still
   parsed by `set_node_statuses_from_json`, and `preview_chain_status` still matches it. Whether the
   variant should be retired outright is a schema decision this lane deliberately left alone.

---

## 8. Two environment incidents this lane hit and cleared

1. **The build volume filled to 100 %** (`382 Mi` free, `ENOSPC`) mid-run: every cargo invocation in
   the fleet was failing. `.🧬semio/🦑️repo/⚡️cache/cargo/build/debug/incremental` was **91 GB**
   (the shared build dir was 352 GB total). Pruning that one directory returned **38 GB**; later runs
   used `CARGO_INCREMENTAL=0` so it does not regrow. Nothing but the incremental cache was removed —
   no artifact, no `🗑️generated` folder, no peer output.
2. **Transient peer breakage, twice.** `semio-framework-plugin` briefly failed to compile
   (`ToolRunPanel::panel` gained a `ready_tool` argument) and cleared on its own;
   `semio-s-artifact-procedural-generation3d` then failed on two errors in files this lane never
   touched — `NodeGraphOperatorChannelRecord` gained a `value_types` field
   (`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:183`) and `preview_eval::preview_kind` changed
   return type (`👁️viewer/🦀️.rs:1254`). Both were mid-refactor by other lanes and **both cleared
   within two minutes**. The crate was re-checked (0 errors) and every law re-run on the current tree:

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib \
  -- status_contract_tests a_late_contributions_install --test-threads=1
  test result: ok. 9 passed; 0 failed
  [DEBUG] chain census ratios=[0.333, 0.333, 0.667, 0.833, 1.0] nodesDone=[2, 2, 4, 5, 6] waves=[2, 1, 1]
  [STATS] before the install: {"height":"ok","radius":"ok","sides":"ok","profile":"queued",
          "extrusion-axis":"computing","extrude":"queued","column-preview":"ok"}
```

   That last line is the new census vocabulary running inside the real served-chain law: one node
   `computing`, its siblings `queued`, nothing `stale`.
