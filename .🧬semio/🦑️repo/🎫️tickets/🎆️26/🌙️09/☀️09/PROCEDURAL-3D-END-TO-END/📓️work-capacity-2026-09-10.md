# Work Capacity — `retained command exceeds semantic work capacity`

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: retained-command work capacity / footprint.
2026-09-10. Session ⚪9f5f6952 (Fable 5.1). Repo MCP down all session
(`invalid initialize params`); ticket bookkeeping is on disk and no ticket was opened/closed/reopened.

Continues `📓️fold-contract-2026-09-10.md` (which made the store FOOTPRINT two rows) and answers
`📓️runtime-verification-2026-09-09.md` boot #5's last action-path blocker.

---

## 1. TL;DR

Two defects, one declaration split:

1. **The unit split.** A retained route declared its work THREE times, in TWO units, and nothing
   compared them: the store footprint its preflight declared (**rows** — 2 since the fold lane), the
   extent its work answered `ArtifactRetainedCommandPhase::Preflight` with (**items** — 1), and the
   `maximum_work_items` its payload carried (**items** — 32); plus a fourth copy of the proof
   contract repeated verbatim 28 times. Fixed by making them ONE declaration
   (`ArtifactRetainedWorkCapacity`), in the store's own unit — staged edit rows.
2. **The message that hid the real live fault.** Preflight reported an `extent()` of `None` — a work
   REFUSING a command — with the capacity message. The served app's only route that can answer
   `None` is `flowEvalTick`, and it does so on every boot: `pending_effects` arms the tick with a
   bare `Effect::DispatchAction` carrying **no window address**, so the shell redispatches it under
   the current window (the flow window `procedural-main`) and
   `Generation3dFlowEvalWindowWork::extent`'s preview-window gate refuses. Reproduced natively (§2.3).
   That refusal now has its own fault, so no future boot can read it as a capacity problem.

`setActiveExample` and `interactionSelect` were **never** over capacity — measured, not inferred
(§2.1/§2.2). Boot #5's console attributed one fault line to the `interactionSelect` observation while
the tick chain re-armed and re-faulted continuously in a buffer that drops thousands of lines a boot.

---

## 2. Native reproduction (task 1) — the measured values

New law module `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs`, five laws, all green.
Raw log `🗑️generated/cap-3-generation3d-suite.txt`; the `--nocapture` values below are quoted verbatim.

### 2.1 Every bounded retained route's extent vs its preflight ceiling

`every_bounded_retained_route_answers_an_admissible_extent` walks all 28 rows of
`GENERATION3D_RETAINED_TOOL_IDS` against the boot document and answers the EXACT value preflight
measures:

```
[DEBUG] generation3d work capacity: items=32 work_items=64 one-item rows=2 footprint rows=2
[DEBUG] retained route setActiveExample:  extent=2  maximum_work_items=64
[DEBUG] retained route nodeGraphEdit:     extent=2  maximum_work_items=64
… (20 more one-item routes, extent=2) …
[DEBUG] retained route addGeneration:     extent=18 maximum_work_items=64
[DEBUG] retained route removeGeneration:  extent=18 maximum_work_items=64
[DEBUG] retained route renameGeneration:  extent=18 maximum_work_items=64
[DEBUG] retained route updateGenerationValues: extent=18 maximum_work_items=64
[DEBUG] retained route selectGeneration:  extent=18 maximum_work_items=64
```

(18 = the boot fixture's 7 widgets + 2 framing steps = 9 items × 2 rows.)

**Before this lane the same walk read `extent=1 … maximum_work_items=32`** — a one-ITEM extent
measured against a 32-ITEM ceiling, while the store measured a 2-ROW footprint. Admissible by
accident, never by construction: nothing in the runtime could have caught a route whose footprint
outgrew its extent, because the two were never the same quantity.

### 2.2 The two routes boot #5 named, through the real job ladder

```
[DEBUG] setActiveExample preflight admitted: lanes=[Artifact, Config, Transient, Ui, Terminal]
[DEBUG] interactionSelect reserved work items=2 lanes=[]
```

`set_active_example_passes_the_retained_preflight` drives `handle_action` → the retained ladder
(wire pages → decode → **preflight** → work → publish) → the host's bounded publication/ACK, and
`interaction_select_passes_the_reserved_preflight` does the same for the framework-reserved route and
then reads the selection back. Both pass. Neither can fault at preflight: `interactionSelect` never
reaches `ArtifactRetainedCommandJob` at all — it is a `FrameworkInteractionSelectJob`
(`🔌️plugin/🦀️.rs` `framework_reserved_route_job`), and no reserved job raises that message.

### 2.3 The live fault, reproduced

`an_unaddressed_flow_eval_tick_is_refused_not_over_capacity` dispatches `FlowEvalTick` with a
`ViewModel` addressed to the flow window `procedural-main` — exactly what the shell does with
`pending_effects`' unaddressed `Effect::DispatchAction` — and the job faults:

```
[DEBUG] unaddressed flowEvalTick: Fault { origin: App, code: FaultCode("app.message"),
  message: "registered fixture typed operation fault:
            retained command work refused the command before any capacity was measured" … }
```

Before this lane the identical dispatch produced **`retained command exceeds semantic work
capacity`** — the browser's message, for an addressing miss with no capacity involved. That is
boot #5's blocker, and it is why the eval chain never starts and `extrusion-axis` stays `computing`.

---

## 3. Root cause, by file and line

| # | file:line | what |
|---|---|---|
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:446` (`ArtifactRetainedCommandPhase::Preflight`) | `!work.extent(…).is_some_and(\|extent\| extent != 0 && extent <= self.maximum_work_items)` — **one** predicate over two distinct failures: a work REFUSING the command (`None`) and a work exceeding capacity. The refusal borrowed the capacity's message. |
| 2 | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs:281` + `:309` + `:344`/`:562` + `:1156-1183` | four independent declarations: `GENERATION3D_RETAINED_WORK_ITEMS = 32` (items), `generation3d_one_item_footprint` → 2 (rows), `generation3d_bounded_extent` → `Some(1)` (items) / `Generation3dPreviewCommandWork::extent` → `widgets+2` (items), and 28 verbatim copies of `ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)`. |
| 3 | `✏️s/…/🌀️generation2d/…/✏️editor/🦀️.rs:211-215` + `:508-515` + `:680` + `:1145-1165` | the same split, already DRIFTED: two bare `ArtifactStoreOneItemFootprint { work_items: 2, … }` struct literals, `extent → Some(1)`, `WORK_ITEMS = 32`, and a proof `max_decoded_items` of **64** against the factory's own 64 but a work-unit budget of **1**. |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` `framework_reserved_work_items` (`_ => Ok(1)`) | every framework-reserved route (`interactionSelect`, `interactionHover`, `selectAll`, `clearSelection`, `setSelectionMode`, `setInteractionGranularity`, the history/clipboard routes) handed its job **one item** while its own `admit_bounded_config_mutation` declared a **two-row** footprint. |
| 5 | `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` `Generation3dFlowEvalWindowWork::extent` + `:1417 pending_effects` | the tick's extent gates on the addressed window being the preview window, but the effect that arms it carries no window address. **Not fixed here** — see §7. |

---

## 4. The unified declaration

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs`:

```rust
/// 🧮️ The ONE quantity a retained route declares, in the store's OWN unit: staged edit ROWS.
pub struct ArtifactRetainedWorkCapacity { invertible_items: usize }

impl ArtifactRetainedWorkCapacity {
    pub const fn for_invertible_items(invertible_items: usize) -> Self;
    pub const fn rows(self, items: usize) -> usize;                  // items × 2
    pub const fn work_items(self) -> usize;                          // → maximum_work_items
    pub const fn rows_for_items(self, items: usize) -> Option<usize>;// → extent()
    pub fn one_item_footprint(self, retained_bytes: usize) -> store::ArtifactStoreOneItemFootprint;
    pub const fn admits(self, extent: usize) -> bool;
}
```

`rows` is `items × store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`, so `rows(1)` IS what
`ArtifactStoreOneItemFootprint::for_one_invertible_item` declares and IS what
`ArtifactStore::fold_batch_item` measures (`forwards.len() + inverse.len()`). One quantity, four
readers.

Per route, exactly one line now carries the number:

| app | declaration | derived |
|---|---|---|
| generation3d | `const GENERATION3D_RETAINED_CAPACITY = ArtifactRetainedWorkCapacity::for_invertible_items(32)` | `GENERATION3D_RETAINED_WORK_ITEMS = …work_items()` (64 rows) · `generation3d_one_item_footprint` = `…one_item_footprint(bytes)` · both extents = `…rows_for_items(n)` · `generation3d_bounded_contract()`'s work units = `…work_items()` |
| generation2d | `const GENERATION2D_RETAINED_CAPACITY = …for_invertible_items(32)` | the same four, plus the two struct literals replaced by one `generation2d_one_item_footprint` |
| framework reserved | `const FRAMEWORK_RESERVED_ROUTE_CAPACITY = …for_invertible_items(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS / ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS)` | `framework_reserved_work_items` = `…rows_for_items(items)` over the unchanged per-route item count (`framework_reserved_route_items`), faulting rather than silently proceeding past the store's own ceiling |

The 28 + 21 verbatim proof contracts are gone: `bounded_first_step_tool_proofs!` already had a
`contract: <expr>, tools: [ … ]` form, and both apps now use it with their own
`generation{3,2}d_bounded_contract()` — the SAME fn the factory's `execution_contract()` returns, so
the proof catalogue can no longer drift from the factory it proves. `max_decoded_items` stays its own
named constant (`GENERATION{3,2}D_RETAINED_DECODED_ITEMS`): it counts decoded JSON items on the
wire, deliberately NOT staged edit rows.

Preflight is now two predicates with two messages:

```rust
let Some(extent) = work.extent(command, snapshot, interaction, self.context.as_deref()) else {
    return self.fault(cx, b"retained command work refused the command before any capacity was measured");
};
if extent == 0 || extent > self.maximum_work_items {
    return self.fault(cx, b"retained command exceeds semantic work capacity");
}
```

---

## 5. Framework law tests (task 2)

`🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs` + fixture `🧫️fixtures/🧮️work-capacity.json` (9 cases,
language-agnostic — the arithmetic is declared as data, not in Rust):

- `one_declared_capacity_answers_rows_ceiling_and_admission` — every fixture row's `rows`,
  `work_items`, `admits` and `rows_for_items` against the store's own invertible row cost.
- `a_route_footprint_of_n_rows_admits_an_extent_of_n` — **the law the task names**: for capacities of
  1 / 2 / 32 / 4 096 items, `one_item_footprint(0).work_items == rows(1)` and
  `admits(footprint.work_items)`.
- `an_item_counted_extent_cannot_stand_in_for_a_row_counted_footprint` — the hostile control:
  `rows(1) != 1`, and a zero-item capacity admits nothing rather than everything.

App-side, `generation3d_declares_one_work_capacity_for_extent_footprint_and_preflight` pins the same
identity for the concrete route, and `both_durable_lanes_declare_through_the_one_shared_footprint_builder`
(fold lane) still forbids a bare struct literal returning.

---

## 6. Runs (task 3)

`CARGO_TARGET_DIR=$S/target-cap`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK` per suite. Raw logs in
`🗑️generated/cap-*.txt`.

| gate | command | result | log |
|---|---|---|---|
| framework laws | `cargo test -p semio-framework-plugin --lib retained_command::` | **6 passed / 0 failed** | `cap-2-framework-laws.txt` |
| framework compile | `cargo check -p semio-framework-plugin --tests --keep-going` | **0 errors**, 85 warnings, `Finished dev in 2m 51s` | `cap-1-plugin-check.txt` |
| generation3d suite | `cargo test -p …-generation3d --features component-app-assembly --lib -- --test-threads=2` | **296 passed / 21 failed** (317) | `cap-3-generation3d-suite.txt` |
| generation2d suite | same, `-p …-generation2d` | **228 passed / 2 failed** (230) | `cap-5-generation2d-suite.txt` |
| procedural native | `cargo check -p semio-s-plugin-procedural --keep-going` | **0 errors**, `Finished dev in 10.65s` | `cap-4-procedural-native.txt` |
| procedural wasm | `--target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | **0 errors**, `Finished wasm-dev in 1m 57s` | `cap-6-procedural-wasm.txt` |

All 5 new work-capacity laws and all 13 fold-contract laws green.

generation3d: the fold lane's baseline was 290 passed / 22 failed of 312; this lane adds 5 tests and
takes the suite to 296/21 of 317. **None of the 21 remaining failures belongs to this lane** — each
is in a class `📓️fold-contract-2026-09-10.md` §8.2 already assigned: preview/geometry after kernel
install (tessellation lane, 11 of them), the mounted/retained authority laws (3), and the viewer
preview render laws (4). generation2d's 2 failures are both the declared fail-closed
`module.vcs` remote-merge class (`two_instances_converge_disjoint_widget_moves`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`), unchanged
by this lane and far ahead of the unit-suite lane's 179/46 of 225 baseline.

One transient red on the way: `cargo check -p semio-s-plugin-procedural` failed once with
`E0425: cannot find value FLOW_EVAL_PUBLICATION_LEDGER` in
`🌊️flow/🖥️host/🦀️.rs:2562`. That file's `thread_local!` was being moved by a peer session mid-check
(the constant is at :2421 and the call at :2579 on disk now, so the line numbers the error quoted no
longer exist). Re-run: green. Not this lane's, and not a real break.

---

## 7. The one defect this lane did NOT fix, and why

`Generation3dFlowEvalWindowWork::extent` refuses every tick that is not dispatched with the PREVIEW
window addressed, and `Generation3dPlayApp::pending_effects` arms the chain with
`Effect::DispatchAction { action: "flowEvalTick", args: None, … }` — no window address at all. The
same three self-redispatch sites (`⏱️flow-eval-tick`, `✅️flow-eval-resolve`,
`🔺️flow-tessellate-resolve`) re-arm it the same way. `Generation3dPlayApp` also leaves
`ArtifactApp::retained_window_transient_target` at the framework default `None`, so the host captures
the window transient from whatever `ViewModel` the shell attached rather than from the tick's own
target — and `FlowEvalTick {}` carries no window id for it to name.

That is the eval-continuation lane's surface (`📓️eval-continuation-runtime-2026-09-10.md`), not the
capacity path this lane was scoped to: the fix is either a window id on the command payload plus a
`retained_window_transient_target` implementation, or an app-level tick that publishes no window
transient. What this lane guarantees is that the next boot's console says
`retained command work refused the command before any capacity was measured` instead of blaming a
capacity that was never exceeded, and that `an_unaddressed_flow_eval_tick_is_refused_not_over_capacity`
holds the reproduction in the suite.

---

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs` | `ArtifactRetainedWorkCapacity` (the one declaration); Preflight split into a refusal fault and a capacity fault |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧫️fixtures/🧮️work-capacity.json` | new — 9 language-agnostic capacity cases |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs` | the three framework capacity laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `FRAMEWORK_RESERVED_ROUTE_CAPACITY`; `framework_reserved_work_items` declares rows through it over a new `framework_reserved_route_items` |
| `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | `GENERATION3D_RETAINED_CAPACITY`; ceiling/extents/footprint/contract all derived; 28 proof contracts collapsed onto the shared `contract:` form; `GENERATION3D_RETAINED_DECODED_ITEMS` named |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs` | new — the five app laws, including the live reproduction |
| `✏️s/…/🌀️generation2d/…/✏️editor/🦀️.rs` | `GENERATION2D_RETAINED_CAPACITY`; both bare footprint struct literals replaced by `generation2d_one_item_footprint`; 21 proof contracts collapsed; `GENERATION2D_RETAINED_DECODED_ITEMS` named |

---

## 9. Restage (task 4)

```
CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTC_WRAPPER="" \
  SEMIO_BUILD_BUDGET_MS=14400000 SEMIO_CMD_BUDGET_MS=14400000 NX_DAEMON=false SEMIO_RENDERER=react \
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev
```

Nothing held `target/wasm32-wasip2` (no `.cargo-lock`, no peer cargo on that target dir) at launch.
The serve on 6018 was NOT restarted.

Freshness proof — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🌀️procedural/semio_s_plugin_procedural_component.core.wasm`:

| | mtime | size | sha256 |
|---|---|---|---|
| before (boot #5's stage) | `Sep 10 04:02` | 80 590 821 | `aab242190c478eb46518aef0c133c14d5b9fc736bfe073803aa99dbcf14f6f1c` |
| after | **`Sep 10 05:22`** | 80 597 509 | **`34682c9d2bf6f17ba34f3723fee891904b54e6c70bf7d3876e4c00ca450f0e87`** |

```
✔  nx run @semio-tech/procedural-plugin:component-dev
✔  nx run @semio-tech/procedural-plugin:materialize-dev
Activated generation3d react dev: 11 completed components (changed)
NX   Successfully ran target activate-generation3d-react-dev … and 38 tasks it depends on
```

Log `🗑️generated/cap-11-activation.txt`; the first (17m 29s) build run is `cap-7-restage.txt`.

It took four launches, none of them this lane's fault — each intermediate failure was a peer session
mid-refactor in a crate this lane never touched, and every one compiled clean on the next attempt:

| launch | failed task | peer's in-flight break |
|---|---|---|
| 1 (17m 29s) | `puzzle-plugin:wasm` | SIGINT during the shared wasm build |
| 2 | 9 × `flow-extension-*:component-dev` | `🌊️flow/🖥️host/🦀️.rs:2185` `E0308` — `kind_infos` becoming `Arc<HashMap<…>>` |
| 3 | the same, then `procedural-plugin:component-dev` | `🌊️flow/🖥️host/🦀️.rs:1039` `E0599 drain_displaced`/`displace_eval_state`, then `…/🌀️generation2d/…/🧬️schema/🦀️.rs:208` `E0599 flow_neuron_kind_info_map` — the same peer's `flow_neuron_kind_infos_json` → `flow_neuron_kind_info_map` sweep, mid-way through its call sites |
| 4 | — | green |

The procedural component itself built successfully on launch 1 already (`04:02 → 05:05`, sha
`7a8752ae…`); launch 4 rebuilt and re-staged it on the then-current tree (`05:22`, sha `34682c9d…`)
and ran the activation the earlier launches never reached.
