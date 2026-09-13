# 🧪️ Remaining Suite Reds — 2026-09-13 (lane: remaining-suite-reds)

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`. Acts on `📓️flow-eval-session-retirement-2026-09-13.md`
§5.2/§5.3 and `📓️lib-suite-deadlock-delivery-2026-09-13.md`.

`repo` MCP did not connect this session (`-32602 invalid initialize params`), `semio` MCP
`CONNECTION_CLOSED` — no ticket lifecycle call was made, `📓️status.md` and `🎫️ticket.json` were not
edited, no modifying git command was run, no dev server was started, stopped or restaged, and
nothing under `🗑️generated/` that this lane did not create was read-modified or deleted. This lane's
logs live in `🗑️generated/suite-reds/`.

## TL;DR

| suite | before (this lane's own run) | after (this lane's own run) |
|---|---|---|
| `-p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1` | **432 passed / 7 failed**, 59.1 s (`gen3d-before.txt`) | **446 passed / 3 failed**, 34.9 s (`gen3d-after.txt`) |
| `-p semio-framework-os-flow --lib -- --test-threads=1` | **162 passed / 58 failed**, 1.9 s (`flow-before.txt`) | **184 passed / 37 failed**, 6.6 s (`flow-after.txt`) |
| `-p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry` | (regression gate for the `Atom` change, §4.5) | **18 passed / 0 failed**, 3.6 s (`example-geometry.txt`) |
| `-p semio-framework-os-kernel-neural-engine --lib` | (same gate) | **54 passed / 0 failed** (`neural.txt`) |

Four of the seven generation3d reds and **21 of the 58 flow reds** are fixed, at five owning layers.
**No test that passed in a before-run fails in an after-run** — the flow failure list after is a
strict subset of the list before (`comm` diff, §5.1). Both remaining lists are attributed in §3
and §4.6 with `git log --date=iso` evidence.

Two of the three remaining generation3d reds turned out to be **unbuilt features declared as if they
existed**, not regressions — §3.1 and §3.2 name exactly what is missing.

---

## 1. The generation3d featured `--lib` suite

### 1.1 Before — 7 failures, all reproduced

`🗑️generated/suite-reds/gen3d-before.txt` (foreground, `--test-threads=1`, 59.1 s, no abort). The
coordinator's brief expected 11 from the flow-eval-session-retirement lane's earlier run; peers had
already fixed four of those (the keyboard fixture, the `generation3d-publication.contended` lease and
the five viewer preview tessellation laws are green, and one new one appeared). The seven measured
here:

| law | message | verdict |
|---|---|---|
| `unit_tests::declared_actions_bridge_to_commands` | `action nodeGraphViewport failed to bridge: nodeGraphViewport requires viewport` | **fixed** (§2.1) |
| `component::work_capacity::every_bounded_retained_route_answers_an_admissible_extent` | `nodeGraphViewport decodes from its own action id` | **fixed** (§2.1, same root cause) |
| `viewer::…eval_chain_tests::a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted` | `flow.registry-retirement-full` | **fixed** (§2.2) |
| `unit_tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | `Generation3d preview operation did not finish` | **half fixed** — the 30 s stall is gone (§2.3); the law now fails in 0.04 s on a real, unbuilt feature (§3.1) |
| `unit_tests::each_example_loads_distinct_fixture_and_preview_geometry` | `registered fixture typed operation exceeded its exact maintenance grant` | **green in every run after §2.4's diagnostic landed**; the diagnostic that would name the stage is now in place if it returns |
| `unit_tests::two_instances_converge_disjoint_widget_moves` | `module.vcs` remote-snapshot merge fail-closed | **not mine** (§3.3) |
| `unit_tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `P3 production envelope load did not reach terminal` | **not fixed, root cause narrowed** (§3.2) |

### 1.2 After — 3 failures

`🗑️generated/suite-reds/gen3d-after.txt`:

```
test result: FAILED. 446 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 34.88s
    editor::generation3d::component::unit_tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows
    editor::generation3d::component::unit_tests::two_instances_converge_disjoint_widget_moves
    editor::generation3d::component::unit_tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed
```

The binary also declares 10 more laws than the before-run (449 vs 439): one is this lane's
(§2.1), the rest are the graph-keyboard-nav lane's `🧭️navigate-graph` command, which landed
mid-session (§6).

---

## 2. The generation3d fixes

### 2.1 `nodeGraphViewport` could not bridge from its own action id — two laws, one root cause

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:211`

`parse_flow_viewport` was the ONE decoder in this app's 34-row `command_from_action` that hard-fails
on an absent argument:

```rust
let value = args.get("viewport").cloned().ok_or_else(|| Fault::from("nodeGraphViewport requires viewport"))?;
```

Every sibling defaults — `setLodMode` `.unwrap_or_default()`, `setSunIntensity` `.unwrap_or(1.0)`,
and the twin camera decoder `parse_preview_camera_json` falls back to
`Generation3dPreviewCamera::default()`. The contract both failing laws state is that a DECLARED
action bridges from its own id under the shell's staged args alone:

* the framework's `assert_declared_actions_bridge_to_commands` (`🔌️plugin/🦀️.rs:6946`) stages
  `effective_action_args(&action.args, &empty_args, None)` — and `nodeGraphViewport` is declared
  `ActionDefinition::new("nodeGraphViewport", …)` (`✏️editor/🦀️.rs:2263`) with **no args at all**, so
  the shell can stage nothing for it;
* `every_bounded_retained_route_answers_an_admissible_extent`
  (`✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs:44`) calls `command_from_action(tool_id, None)` for every
  retained route.

An action that declares no args and a command that requires one cannot both be right. Fixed at the
decoder: **absent → the identity camera, present-but-malformed → still a fault.**

```rust
let Some(value) = args.get("viewport").cloned() else { return Ok(semio_framework_os_kernel::Viewport2d::default()) };
dsl::from_dsl_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))
```

`Viewport2d::default()` is `{x: 0, y: 0, zoom: 1}` (`🖱️ui/🪟️viewport/◻️2d/🧬️schema/🦀️.rs:61`), and the
runtime always supplies the key anyway (`nodeGraphViewportActionArgs`,
`🧱️elements/🕸️NodeGraph/🟦️.tsx:146`), so no runtime path changes shape.

Both halves are now stated by a law of their own, because the generic bridge law only ever exercises
the absent half — `node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one`
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1189`): an argless invocation decodes to the identity camera, an
authored one round-trips exactly, and a `zoom: 0` viewport still faults (`crate::zoom` rejects a
non-positive zoom, `🖱️ui/🪟️viewport/🦀️.rs:36`).

### 2.2 `flow.registry-retirement-full` — nothing in the repository pumps that queue

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs:260` (new `reclaim_free_retired_registries`), `:279` (its call site)

The viewer eval-chain law passes alone and fails at law 398 of the suite. The queue that refuses it
is `FlowExtensionRegistryState::retired`, a `VecDeque` of `RETIRED_REGISTRY_CAPACITY = 16`
(`📔️registry/🦀️.rs:60`); `begin_flow_registry_replacement` refuses the 17th replacement with
`flow.registry-retirement-full`, and the only thing that gives a slot back is
`retire_flow_extension_registries_step`.

**That pump has no caller outside test code anywhere in this repository.** Measured:

```
grep -rn "retire_flow_extension_registries_step|retireFlowExtensionRegistries" --include=*.rs --include=*.ts --include=*.tsx --include=*.wit .
```

returns 14 hits: the definition, a `#[cfg(test)]` drain helper, and 12 call sites all inside
`🧪️tests`. So this is not a test-harness artifact — **a served process is refused its 17th
`setContributions` / extension install / uninstall and every one after it, forever.** The procedural
3d shell pushes a contributions closure per boot.

The lib-suite-deadlock-delivery lane had already MEASURED that draining the queue from the test
serial guard is wrong (`🧪️tests/🔬️serial/🦀️.rs:74-89`: `443 passed; 7 failed` without the drain,
`375 passed; 75 failed` with it) and wrote "the queue's depth belongs to whoever owns the registry
retirement frontier, not to a test guard". This lane took that literally and fixed it at the
frontier's own door instead:

```rust
if state.retired.len() >= RETIRED_REGISTRY_CAPACITY {
    reclaim_free_retired_registries(state);
}
if state.retired.len() >= RETIRED_REGISTRY_CAPACITY { return Err("flow.registry-retirement-full"); }
```

`reclaim_free_retired_registries` is **front-only and never rotates**. That is the whole difference
from the drain that was measured to break 68 laws: a blocked front means
`RegistryRetirement::close_step` could not take `Arc::get_mut` on the version's `final_root`
(`🧠️neural/⚙️engine/📔️registry/🦀️.rs:102`), i.e. a live `SharedRegistry` reader still holds it, and
reclaiming stops there. Back-pressure is paid at the door; nothing is ever force-closed. It runs
ONLY when the queue is already full, so a green lane pays nothing.

The promise this must not cost is
`registry_replacement_admission_preserves_roots_on_capacity_and_generation_exhaustion`: a caller that
PINS a version and then fills the queue is still refused, with every root intact. That law measured
exhaustion by an exact install count (`for _ in 1..capacity`), which is now the queue depth BEHIND
the pinned version rather than `capacity`; it fills by pressure instead
(`📔️registry/🧪️tests/📔️registry/🦀️.rs:113-120`) and states exactly what it stated before.

New law: `a_full_retirement_queue_reclaims_its_free_versions_before_refusing_but_never_past_a_live_reader`
(`📔️registry/🧪️tests/📔️registry/🦀️.rs:156`) — 64 installs with no pump at all all succeed, the queue
never grows past its declared capacity, a live reader is never retired past, the refusal leaves the
pinned root byte-identical, and releasing the reader makes the slot reclaimable again.

Registry laws in isolation: **12 passed / 0 failed** (`flow-registry-2.txt`), from 10/2.

### 2.3 `Generation3d preview operation did not finish` — a second copy of a bounded protocol

`✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:756`

`drive_preview_operation` was a hand-rolled copy of the framework's own settle ladder that never
drained `take_typed_operation_completion`. The terminal witness lands in its own outbox and
`has_pending_typed_operations` counts it (`🔌️plugin/🦀️.rs:27418`), so the loop spun to its 30-second
deadline and reported that an operation which had in fact finished "did not finish". The framework's
`settle_registered_typed_operation` grew that drain earlier in this ticket and its docstring names
this exact trap — a second copy of a bounded protocol is how a fix lands in one of them only.

`drive_preview_operation` now delegates to `settle_registered_typed_operation` and counts the lanes
of its receipt. The law terminates in **0.04 s** instead of 30 s, and now fails on what it is actually
about (§3.1): `artifact=1 config=1 transient=0`. The message carries those counts now, because
"did not publish … exactly once" without them says nothing.

### 2.4 Two framework testkit defects found on the way

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

1. **`:6752` — `settle_registered_typed_operation` under-drained presented pages.** It took at most
   ONE result page per turn, but `has_pending_typed_operations` does not count a presented,
   un-ACKed page — so a turn that leaves a second page queued can be the turn the loop exits on, and
   that page is never seen. Now `while let Some(page) = …`. (This is not what §3.1's `transient=0` is:
   the transient page is never published at all. The under-drain is a real defect found while
   proving that.)
2. **`:6739` — the maintenance-grant refusal named nothing.** `registered fixture typed operation
   exceeded its exact maintenance grant` did not say which stage or by how much, while the atomic
   that knows (`crate::app::LAST_MAINTENANCE_STAGE`) was already being written on every
   `maintenance_step`. It now reports `stage N released I items / B bytes against a grant of …`.

---

## 3. The three generation3d reds this lane did NOT fix, and why

### 3.1 `generation_preview_is_one_app_transient_shared_by_two_generation_windows` — the app transient is declared, never written, never read

This is **not a regression**. Measured, with no ambiguity:

* `addGeneration` DECLARES the transient lane —
  `ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[Artifact, Config, Transient] }`
  (`✏️editor/🦀️.rs:786`), and so do `setActiveExample`, `removeGeneration`, `renameGeneration`,
  `updateGenerationValues` and `selectGeneration`.
* The work that serves those tools, `Generation3dPreviewCommandWork::step` (`✏️editor/🦀️.rs:479`),
  only ever returns `ArtifactCommandWorkStep::Complete(emit)` — never `CompleteWithEphemeral` — and
  the computed preview it holds is **thrown away**: `if let Some(fixture) = result.preview_fixture { fixture.retire_cold(); }`
  (`✏️editor/🦀️.rs:503`).
* The mutation that would publish it, `SetGenerationPreview`
  (`✏️editor/🫧️transient/🧬️schema/🧬️mutations/👁️set-generation-preview/🦀️.rs`), has **zero emitters**
  — `grep -rn SetGenerationPreview` returns only its own definition, its enum row and its own unit
  test.
* The field it writes, `Generation3dTransient::generation_preview_text`
  (`✏️editor/🫧️transient/🦀️.rs:10`), has **zero readers**: `render_with_request_context`
  (`✏️editor/🦀️.rs:2112`) resolves the preview text from the two WINDOW transients
  (`transient.window::<…PreviewWindowTransientOwner>()`) and never touches the app-level one.

So the app-shared generation preview was designed (schema, mutation, declared lane, law) and never
built. Finishing it means the preview work must EVALUATE the generation-patched fixture and publish
its eval JSON as the app transient — the `host`/`session` fields
`Generation3dPreviewCommandWork` already carries for that (`✏️editor/🦀️.rs:427-433`) are never
populated — and the generate-preview render must read the app transient as the shared source. That is
a feature inside three live lanes' area (generate-mode panels/interactions and the wgpu generation
publication lane; `✏️editor/🦀️.rs` last written `ac22984a03`, **2026-09-13 20:45:21**, minutes before
this lane read it), and it must be sized against the interactive-turn budget that
`hex_column_boot_stays_inside_the_interactive_turn_budget` already measures. **Not built here, and no
claim is made about how it should be budgeted.**

### 3.2 `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` — the decode worker never leaves `Active`

Root cause narrowed, not established. What was measured (all foreground, law run alone):

* The operation IS a decode job, not a stuck ingress: `poll_artifact_envelope_decode(handle)` reads
  `Pending` on every turn, and a missing job reads `Fault`
  (`🔌️plugin/🦀️.rs:18347`) — so its state is `ActiveArtifactEnvelopeDecodeState::Active`.
* **No maintenance turn ever reports `Blocked`** — instrumented across 2 000 turns, zero
  `PluginCloseStep::Blocked { reason }`. Nothing on the ladder names an authority it waits for.
* It is **not** the driver's synchronous shape. The framework's own driver for this protocol
  (`drain_envelope_decode`, `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:1881`) awaits
  `advance_typed_operation_publication` and a cooperative yield per turn; rewriting
  `drive_production_envelope` that way (300 000 turns, 286 s) left the poll at `Pending`. That
  rewrite was **reverted** — it was 8× slower for the same red — and the driver's docstring
  (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:568`) now records the measurement so the next lane does not repeat it.

The remaining suspects are inside `ActiveArtifactEnvelopeDecode::drive` →
`WorkerJobSession::pump_one` (`🔌️plugin/🦀️.rs:18334`, `🧵️job/🦀️.rs:2045`): a submitted step whose
`WorkerJobPoll` never leaves `Submitted`. The identical law exists in seven sibling artifacts
(`raster`, `writer`, `process3d`, `gismap`, `drawing`, `generation2d`, `jack`) — whether they pass is
**not measured here**, and if they do, the difference is the place to look.

### 3.3 `two_instances_converge_disjoint_widget_moves` — a framework capability that does not exist yet

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18079`:

```rust
fn merge_remote_snapshot(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), VcsError> {
    let _ = (pack, spr);
    Err(VcsError::ValidationFailed("remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized".into()))
}
```

`git log --date=iso -S` puts that guard at **`21fbcd3538`, 2026-09-02 12:19:02** — eleven days old,
deliberate, and named with its own future. It is unreachable-around for the fixture: `attach_backbone`
always flushes a full `BackboneMessage::Snapshot` (`🏪️store/🦀️.rs:18154`), so in
`paired_registered_apps` instance `a`'s attach puts a snapshot in the channel and instance `b`'s
attach pumps it straight into the stub (`🔌️plugin/🦀️.rs:6892`, `attach b`). **Every**
`two_instances_converge_*` law in the repository is blocked the same way. Implementing the streaming
envelope decoder plus the persistent candidate transaction is a workstream, not a red-fix, and it is
the framework store lane's. **Not touched.** This is the "store vcs guard" the brief asked about: it
is not a guard defect, it is an unbuilt capability failing closed exactly as written.

---

## 4. The flow `--lib` suite

### 4.1 The 58, classified

Reproduced this lane's own before-run (`flow-before.txt`, 162/58):

| count | family | verdict |
|---|---|---|
| 27 | `ordered-map root must be explicitly retired before drop` | test-side ownership debt — **18 fixed** |
| 13 | `final Dictionary ownership must be explicitly retired or owned by a cold boundary` | same — **12 fixed** |
| 7 | value assertions | mixed — **1 fixed** (§4.5), the rest attributed (§4.6) |
| 4 | `registry::tests` shared-singleton order dependence | **1 fixed**, 3 remain (§4.4) |
| 2 | `FlowEvalSession must finish explicit close before drop` | **both fixed** (§4.3) |
| 2 | `extrude solid output` | not this lane's (§4.6) |
| 3 | singletons (`mem::replace` guard, VCS bridge progress, oracle source law) | not this lane's (§4.6) |

The brief's instruction was: *if tests drop owners without driving a ladder and the contract says
owners must be retired, the tests are wrong — fix them to retire through the paid ladder.* That is
what the 30 ownership fixes do. Three of them needed a paid ladder that did not exist yet, so it was
added at the owning layer rather than open-coded in the tests (§4.2, §4.3).

### 4.2 Three new paid ladders at the owning layer

* **`FlowFixture::replace_widgets`** (`🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:560`) — a bare
  `fixture.widgets = vec![…]` drops every displaced `Widget`, each of which owns a
  `Dictionary`/`OrderedSet`/`Tree` that refuses a bare drop, so the assignment panics at the END of
  the caller naming nothing that points back at it. This is the paid form, the twin of the
  `std::mem::replace(&mut self.fixture, fixture).retire_cold()` the flow host's own `apply_fixture`
  already pays (`🖥️host/🦀️.rs:290`).
* **`FlowGui::retire_cold` and `FlowArtifact::retire_cold`** (`🧵️retained/🦀️.rs:572`, `:580`) —
  `FlowHost::document()` hands back an OWNED projection (`FlowFixture::to_artifact`) whose `Tree`
  neurons own `Dictionary` params and whose `FlowUi.nodes` is an `OrderedMap`, so a caller that only
  READS it still has to close it. There was no way to.
* **`retire_flow_store_cold`** (`🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:764`) — `ArtifactStore`'s
  `Drop` asserts a terminal-empty shallow shell and only the owner-supplied disposer empties it, so a
  store built for the length of an expression aborts with
  `artifact store reached Drop without its exact terminal-empty shallow-shell witness`. Drains the
  same `close_owned_store_step` ladder a retained caller drives, in one cold pass.

### 4.3 `FlowEvalSession::retire_cold`

`🌊️flow/🖥️host/🦀️.rs:3521`. Three sites in the flow tests open-coded the same
`begin_close` + bounded `close_step` loop and two more just dropped the session. One ladder now, the
twin of `FlowHost::retire_cold`.

### 4.4 The registry laws' order dependence, and what remains of it

`contributed_registry_replacement_preserves_readers_and_drains_old_versions`,
`registry_maintenance_retains_cursor_outside_a_faulted_worker` and
`registry_replacement_admission_preserves_roots_on_capacity_and_generation_exhaustion` each ended with
`assert!(flow_extension_state().lock().unwrap().retired.is_empty())`. The queue is ONE process-global
singleton and a version another live law still reads can never be freed, so that is an assertion
about the whole binary's history rather than about the law — all three pass alone and fail in suite
order. `drain_flow_extension_registry_retirements` now ANSWERS the depth it could not free
(`📔️registry/🦀️.rs:89`) and those laws judge themselves against that baseline.

That is necessary but not sufficient, and the residue is **named, not claimed fixed**:
`registry_maintenance_retains_cursor_outside_a_faulted_worker` publishes a registry whose operator
PANICS in `retire_step` and expects to observe `flow.registry-retirement-panicked`. In suite order it
observes `None`, because by the time it runs its 1 000-step loop the faulting version is sitting
behind other laws' queued versions and is never reached. Three registry laws still fail in suite
order for that reason; all pass alone.

### 4.5 One value assertion was a real exact-carrier defect

`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:287`

`flow_eval_session_seeds_its_retained_neural_cache` seeds a node cache from JSON and gets
`Atom::Integer(42)` back where it wrote `Atom::Decimal(42.0)`. Traced hop by hop:

* `to_json_string(&Dictionary)` emits `{"$schema":"number","value":42.0}` — measured with a
  `[DEBUG]` print, since removed. `pack::json`'s `write_float` is exact.
* `pack::json`'s parser distinguishes `42` from `42.0` **by the text** and answers
  `Number::Float(42.0)` — its own docstrings say it does this precisely so an integer stays an
  integer across the bridge.
* `FromValue for Atom` then threw that away:
  `DslValue::Number(Number::Float(value)) if value.fract() == 0.0 => Ok(Atom::Integer(value as i64))`.

One lossy hop, at the one place whose job is to preserve the carrier, and its docstring claimed it
was "the same convention `pack::json::Number` uses" when `pack::json` does the opposite. Introduced
`21fbcd3538`, 2026-09-02 12:19:02. The shortcut is removed: a `Float` decodes to `Atom::Decimal`,
always.

This is on the procedural 3d user path, not only in a law: every `flowEvalResolve` seeds a node cache
from exactly this JSON (`seed_flow_eval_node_cache`, `🌊️flow/📔️registry/🦀️.rs:608`), so a decimal
output came back from an extension hop as an integer and compared unequal to what the kernel
produced. That is a **code-path argument**; no browser measurement was taken.

Regression gates run for it, both green and both in the foreground:
`semio-framework-os-kernel-neural-engine --lib` **54 / 0**, and generation3d's
`--test example-geometry` **18 / 0** in 3.63 s — the same count and the same laws the
lib-suite-deadlock-delivery lane left green.

### 4.6 After — 37, and what they are

`flow-after.txt`: **184 passed / 37 failed**. By module:

| module | count | note |
|---|---|---|
| `vcs::flow_vcs_tests` | 21 | 20 are `FlowRetainedVcs` / `FlowVcsDocument` built and dropped by the test body without driving `close_retired_step`; 1 is `production Flow VCS bridge did not progress` |
| `wasm_session::domain_laws` | 8 | same ownership family plus two census/receipt assertions |
| `registry::tests` | 3 | §4.4's residue |
| `host::tests` | 3 | `delete_selection_removes_edge_selected_by_synapse_id_domain` (`synapse id s1 must map into engine edge selection`) and two `extrude solid output` — real evaluation behaviour, not ownership |
| `drawing::drawing_kernel_tests` | 1 | `derive_twice_same_node_is_same_handle`, `left: 1 right: 2` |
| `extensions::wasm::tests` | 1 | `evaluate_round_trips_dictionary`, ownership |

The 20 `retained_vcs_*` reds have one shape and one missing ladder: `FlowRetainedVcs` has
`begin_close` / `close_retired_step` / `terminal_is_empty` (`🌿️vcs/🦀️.rs:914-966`) but no cold
disposal, and a generic one cannot be written blind — `close_retired_step` answers
`FlowVcsFault::ClosePending` while `credits.operations > 0`, so a law that leaves an operation open
has to cancel it first, which differs per law. That is the next lane's cheapest 20 points; the
pattern to follow is §4.2/§4.3.

---

## 5. Runs

Every count in this report was produced by a foreground run in this session; `df -h /` before the
first build showed 95 Gi available. Logs under `🗑️generated/suite-reds/`.

| log | command | result |
|---|---|---|
| `gen3d-before.txt` | generation3d featured `--lib`, `--test-threads=1` | 432 / 7, 59.1 s |
| `viewport-laws.txt` | the three viewport-related laws | blocked by a peer's half-written module (§6) |
| `gen3d-2.txt` | after §2.1 | 444 / 5, 133 s (under peer load; `hex_column_boot…` overran its 8 ms interactive ceiling at 12 015 µs and is green in every other run) |
| `gen3d-3.txt` | after §2.2 | 446 / 3, 102 s |
| `gen3d-after.txt` | after §2.3, §2.4, §4.5 | **446 / 3, 34.9 s** |
| `flow-before.txt` | flow `--lib`, `--test-threads=1` | 162 / 58, 1.9 s |
| `flow-host-1/2/3.txt` | `host::tests` only | 86/13 → 88/11 → 92/7 |
| `flow-registry-1/2.txt` | `registry` only | 10/2 → **12 / 0** |
| `flow-mid.txt`, `flow-mid2.txt` | whole flow `--lib` | 180 / 41 |
| `flow-after.txt` | whole flow `--lib` | **184 / 37, 6.6 s** |
| `neural.txt` | `semio-framework-os-kernel-neural-engine --lib` | **54 / 0** |
| `example-geometry.txt` | generation3d `--features component-app-assembly --test example-geometry` | **18 / 0**, 3.63 s |

### 5.1 No regression, measured rather than asserted

```
comm -13 before.txt after.txt      # failures present after but not before
(empty)
```

The flow after-list is a strict subset of the before-list: 21 fixed, **0 new**. On generation3d, the
three remaining laws are three of the seven from the before-run; nothing that passed before fails now.

---

## 6. Peer work encountered, and not touched

* **`✏️editor/🎮️commands/🧭️navigate-graph/`** (graph-keyboard-nav-appearance-boot lane) was
  half-written at 21:04 — `mod tests;` declared, `🧪️tests/🔬️unit/🦀️.rs` not yet on disk — and the
  generation3d test build failed on it for ~12 minutes. **Waited, did not fix forward**; the file
  appeared at 21:09 and the build recovered on its own. This lane used that window for the flow
  suite.
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` and
  `📔️registry/🧪️tests/📔️registry/🦀️.rs` were both reformatted under this lane mid-edit by a peer's
  tooling (import lines and one literal rewritten). Those rewrites were taken as the current state
  and built on, per the brief.
* `✏️editor/🦀️.rs` was last written by a peer at **20:45:21** (`ac22984a03`); this lane re-read it
  immediately before each of its two edits there.
* No hunk of any peer's work was reverted, and no modifying git command was run.

## 7. Files

Source:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `parse_flow_viewport` (§2.1).
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the new viewport law (§2.1), `drive_preview_operation` (§2.3), the preview-count diagnostic, `drive_production_envelope`'s measured docstring (§3.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — presented-page drain and the maintenance-grant diagnostic (§2.4).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` — `reclaim_free_retired_registries`, the admission's back-pressure pump, and the drain helper's baseline answer (§2.2, §4.4).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs` — the new capacity law and the four baseline-relative assertions (§2.2, §4.4).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` — `FlowFixture::replace_widgets`, `FlowGui::retire_cold`, `FlowArtifact::retire_cold` (§4.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs` — `retire_flow_store_cold` (§4.2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowEvalSession::retire_cold` (§4.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — 18 ownership fixes plus the store-owner installation (§4.1-§4.3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs` — three ownership fixes (§4.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` — `FromValue for Atom` exact carrier (§4.5).

Ticket: this report and `🗑️generated/suite-reds/*.txt`.

## 8. What is NOT claimed

- **Neither suite is green.** 3 of 449 generation3d laws and 37 of 221 flow laws still fail. §3 and
  §4.6 attribute every one; a root cause was ESTABLISHED for §3.1 and §3.3, NARROWED for §3.2, and
  for the flow residue only the family is named.
- **No production or browser claim.** Everything here is the native `--lib`/`--test` harness. No dev
  server was started or restaged, no plugin was booted, no browser probe was run. The user-path
  arguments in §2.2 (a served process cannot replace the registry a 17th time) and §4.5 (every
  `flowEvalResolve` seeds through the lossy hop) are **code-path arguments**, not measurements.
- **The `Atom::Decimal` change is framework-wide and was gated on three suites, not the workspace.**
  `neural-engine --lib` (54/0), generation3d featured `--lib` (446/3) and `--test example-geometry`
  (18/0) are green with it; `cargo check --workspace` was NOT run, and other crates that relied on a
  whole-valued JSON float decoding to `Atom::Integer` would now see `Atom::Decimal`. No such consumer
  was found by grep, and no audit of the wasm/TS bridges was done.
- **§3.2's remaining suspects are suspects.** The decode worker's `WorkerJobPoll` was not
  instrumented; the seven sibling artifacts' identical laws were not run.
- **The seven sibling `advance_artifact_envelope_load` laws, `--features` variants other than
  `component-app-assembly`, and every wasm target were not built.**
- **Ticket lifecycle untouched**, per the coordinator's instruction to this lane: `📓️status.md`,
  `🎫️ticket.json` and `📌️important.md` were not read-modified, and no `ticket_*` call was made.
