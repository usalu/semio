# 🧵️ `handle_action(interactionSelect)` stack overflow — root cause, fix, gates (2026-09-09)

Packet: the **selection path** only. `📓️editor-gaps-2026-09-09.md` §7 owed two follow-ups; this one
closes the first (`handle_action(INTERACTION_SELECT_ACTION_ID)` aborts the process). The second
(`VcsArtifactApp::snapshot()`'s ordered-map retirement abort) is a sibling lane's and is untouched
here — §6 reports what is still red because of it.

## 0. TL;DR

| | before | after |
|---|---|---|
| **the failure** | `fatal runtime error: stack overflow, aborting` inside `app.handle_action(INTERACTION_SELECT_ACTION_ID, …)` — reproduced on `context_menu_reads_the_framework_owned_graph_selection` AND on the untouched pre-existing `generation3d_interaction_selection_owns_its_persisted_history` | both run green at the default thread stack |
| **the cause** | **not recursion.** One non-recursive descent that reserves 3–4 MiB of stack, against the 2 MiB a Rust-spawned thread (libtest's test thread) gets | 1.75–2.0 MiB, inside the default |
| **the dominant frame** | `VcsArtifactApp::dispatch_framework_reserved_action::{closure#0}` reserved **1 890 336 bytes in ONE resume frame** | **220 864 bytes** (−88 %) |
| **the second frame** | `<VcsArtifactApp<…> as PluginApp>::close_step` reserved **692 080 bytes** | **470 320 bytes** (−32 %) |
| **guard** | none | `one_framework_reserved_route_fits_a_bounded_thread_stack` — drives one real `interactionSelect` on a deliberately 2 MiB thread |

No stack-size workaround anywhere: `RUST_MIN_STACK` was used **only as a diagnostic** to prove the
descent was finite, and is not set by any test, script or `launch.json` entry this packet touched.

---

## 1. Reproduction, and why the "infinite recursion" premise was wrong

Private target dir throughout: `CARGO_TARGET_DIR=$S/target-sel` (APFS clone of `target/debug`),
`RUSTC_WRAPPER=""`. Binary:
`target-sel/debug/deps/semio_s_artifact_procedural_generation3d-aac96b3af30be31e`.

```
$ RUST_BACKTRACE=1 …/semio_s_artifact_procedural_generation3d-… --test-threads=1 --nocapture \
      editor::generation3d::component::tests::context_menu_reads_the_framework
running 1 test
test …::context_menu_reads_the_framework_owned_graph_selection ...
thread '…::context_menu_reads_the_framework_owned_graph_selection' (1565318) has overflowed its stack
fatal runtime error: stack overflow, aborting
rc=134
```

`RUST_BACKTRACE=1` prints nothing: Rust's stack-overflow handler `abort()`s without unwinding, so the
overflow backtrace has to come from somewhere else. Two independent instruments were used.

### 1.1 The descent is FINITE — so it is not a cycle

Raising only the thread stack (diagnostic, not a fix) makes the test pass unchanged:

| `RUST_MIN_STACK` | 2 048 KiB | 3 072 KiB | 4 096 KiB | 128 MiB |
|---|---|---|---|---|
| result | overflow | overflow | **ok** | **ok** |

An infinite recursion cannot be bought off with 4 MiB. So there is no `handle → interaction_select →
render/pending_effects → handle` cycle, no `Deref`/`with_session` re-entry and no context-menu
delegate loop: `context_menu`/`context_menu_with_request_context`/`context_menu_body` form a strict
one-way delegation and never re-enter `handle_action`. The premise recorded in
`📓️editor-gaps-2026-09-09.md` §7 ("stack-overflows … recurses") is corrected here.

### 1.2 The real call chain (10 frames, not 10 000)

A temporary `[DEBUG]` `std::backtrace::Backtrace::force_capture()` probe was placed in
`Generation3dPlayApp::interaction_topology` (the app-side callback the dispatch reaches deepest) and
the test re-run at 128 MiB. Verbatim, framework part:

```
 1: <Generation3dPlayApp as ArtifactEditor>::interaction_topology
 2: <EditorApp<Generation3dPlayApp> as ArtifactApp>::interaction_topology::{closure#0}
 3: VcsArtifactApp<EditorApp<…>>::resolve_domain_topology::<…>::{closure#0}
 4: VcsArtifactApp<EditorApp<…>>::dispatch_interaction_action::{closure#0}
 5: VcsArtifactApp<EditorApp<…>>::commit_framework_shared_host_route::{closure#0}
 6: VcsArtifactApp<EditorApp<…>>::dispatch_framework_reserved_action::{closure#0}
 7: VcsArtifactApp<EditorApp<…>>::dispatch_action::{closure#0}
 8: <VcsArtifactApp<EditorApp<…>> as PluginApp>::handle_action::{closure#0}
 9: …::context_menu_reads_the_framework_owned_graph_selection::{closure#0}
10: …::__semio_async_test_block_on
```

Ten frames. Every frame appears **once**. The probe was removed again before the gates below.

The executor is not the culprit either: `semio_framework_async_macros::async_test`'s inlined
`block_on` (`⏳️async/✨️macros/🦀️.rs:52-71`) is a `loop { poll; park }` with a `Thread::unpark` waker,
and `plugin_job_yield_once` (`🔌️plugin/🦀️.rs:15259`) is `poll_fn` + `wake_by_ref` + `Pending` — a wake
that unparks, never one that re-polls from inside the waker. Yielding does not nest.

### 1.3 The frames are the problem — measured off the machine code

Each function's stack reservation is in its own prologue. The stripped-debuginfo binary still carries
it (`objdump -d`, LLVM — an instrument entirely outside this codebase):

```
0000000100132070 <…VcsArtifactApp<EditorApp<Generation3dPlayApp>>::dispatch_framework_reserved_action0…>:
  100132070: stp  x28, x27, [sp, #-0x20]!
  100132074: stp  x29, x30, [sp, #0x10]
  100132078: add  x29, sp, #0x10
  10013207c: sub  x9, sp, #0x1cd, lsl #12   ; = 0x1cd000        ← 1 888 256 bytes
  100132080: sub  sp, sp, #0x1, lsl #12                          ← probe loop
  …
  100132090: sub  sp, sp, #0x820                                 ← + 2 080 bytes
```

**1 890 336 bytes = 1.80 MiB in one frame.** The whole `interactionSelect` descent measured:

| frame | bytes |
|---|---|
| `handle_action::{closure#0}` | 112 080 |
| `dispatch_action::{closure#0}` | 190 480 |
| **`dispatch_framework_reserved_action::{closure#0}`** | **1 890 336** |
| `run_framework_reserved_job::<ArtifactReservedToolJob>::{closure#0}` | 63 824 |
| `commit_framework_shared_host_route::{closure#0}` | 33 328 |
| `dispatch_interaction_action::{closure#0}` | 58 512 |
| `revalidate_and_persist_interaction_state::{closure#0}` | 29 168 |
| **sum** | **2 377 728** (+ the test's own 401 200-byte generator frame) |

against 2 MiB. `-Zprint-type-sizes` corroborates that no single *type* is huge (the largest future in
the crate is ~55 KiB, the largest struct 30 KiB) — the 1.8 MiB is the sum of many separately-slotted
temporaries in one body, not one fat value.

---

## 2. The cycle — stated as asked, in two lines

There is **no cycle**. `handle_action` → `dispatch_action` → `dispatch_framework_reserved_action`
(`🔌️plugin/🦀️.rs:21294`) → `run_framework_reserved_job` → `commit_framework_shared_host_route` →
`dispatch_interaction_action` (`:20666`) → `resolve_domain_topology` (`:20552`) →
`Generation3dPlayApp::interaction_topology` (`✏️editor/🦀️.rs:1349`) is a **straight, ten-frame, non-recursive
descent**; it aborted because `dispatch_framework_reserved_action`'s single generator resume frame
reserved 1.80 MiB of the 2 MiB a Rust-spawned thread owns, and the teardown's
`PluginApp::close_step` (`:23173`) reserved a further 692 KiB.

---

## 3. Root cause and fix

### 3.1 `dispatch_framework_reserved_action` — 19 awaits of the same future type, 19 slots

`🔌️plugin/🦀️.rs:21294`. The body was one `match action { … }` with **19 arms**, each spelling its own

```rust
self.run_framework_reserved_job(action, &raw, decoded_items, ArtifactReservedToolJob::new(FrameworkXJob::new(raw.clone(), work_items)), meta, None).await?
```

Every arm erases its job into the *same* concrete `ArtifactReservedToolJob`, so all nineteen call the
*same* monomorphization `run_framework_reserved_job::<ArtifactReservedToolJob>` — whose future is
~64 KiB. But nineteen `.await`s are nineteen distinct suspend points, and an unoptimized build
(`opt-level = 0`, no LLVM stack colouring, no MIR slot reuse) gives each its own **non-overlapping**
slot: 18 × ~64 KiB ≈ 1.15 MiB, plus the four `commit_framework_*_route` futures and the
`qualified_tool_proof`/`framework_reserved_work_items`/`build_artifact_reserved_action_job` slots.

**Fix.** The route→job `match` is not async and does not need to be inside the generator. A new
module-level `framework_reserved_route_job(action, raw, work_items) -> Result<ArtifactReservedToolJob, Fault>`
(`🔌️plugin/🦀️.rs:13935`, right under the `framework_reserved_job!` invocations that define the nineteen
job types) holds all nineteen arms; each arm's temporary is now a `FrameworkXJob` of ~100 bytes in a
plain, small, non-generator frame. `dispatch_framework_reserved_action` keeps exactly one clipboard
branch (`copy`/`cut`/`paste` still get the app's own job first through
`build_artifact_reserved_action_job`) and **one** `run_framework_reserved_job` await:

```rust
let job = if CLIPBOARD_ACTION_IDS.contains(&action) {
    match self.build_artifact_reserved_action_job(action, args, raw.clone(), meta).await? {
        Some((job, completion)) => { app_completion = Some(completion); job }
        None => framework_reserved_route_job(action, raw.clone(), work_items)?,
    }
} else {
    framework_reserved_route_job(action, raw.clone(), work_items)?
};
let permit = self.run_framework_reserved_job(action, &raw, decoded_items, job, meta, None).await?;
```

Semantics are byte-identical, including the `interactive-job.unknown-reserved` fault for an
unrecognised route (now raised by the helper, still before any job is dispatched).

**Measured:** 1 890 336 → **220 864** bytes (−1 669 472, −88 %).

After this alone the test's requirement fell from 3–4 MiB to 2.05–2.25 MiB — still just over the
2 MiB default, and the abort moved to teardown (§3.2).

### 3.2 `PluginApp::close_step` — 35 KiB operations moved by value inside a 30-stage body

`🔌️plugin/🦀️.rs:23173`. `close_step` is one 520-line sequential bounded-close state machine (~30
`if`-guarded stages, each `return`ing). Two of its stages own
`MountedTypedCommandFullOperation<A>`, which `-Zprint-type-sizes` puts at **35 552 bytes** for
`EditorApp<Generation3dPlayApp>`; each stage moves it by value several times (`get_mut(..).ok_or_else(..)?`
→ `ControlFlow<Result<Infallible, Fault>, …>`, `remove(..)` → `Option<…>` → `Result<…, Fault>` → the
local → `drop`), and again each move is its own slot.

The macOS crash report for the 2 MiB run named the faulting frame exactly:

```
…VcsArtifactApp<EditorApp<Generation3dPlayApp>> as PluginApp>::close_step   (+0x400c)
…testkit::close_registered_fixture_app::<VcsArtifactApp<EditorApp<…>>>
…context_menu_reads_the_framework_owned_graph_selection::{closure#0}
```

**Fix.** The two operation-owning stages became their own `#[inline(never)]` methods on the inherent
`impl`, verbatim bodies, guards left in place at the call site:

- `close_latest_wins_command_step` (`🔌️plugin/🦀️.rs:19323`) — the head keyed (latest-wins) command,
- `close_typed_operation_step` (`🔌️plugin/🦀️.rs:19345`) — the head mounted typed operation,

so their 35 KiB moves live in a frame that exists only while that stage runs. `close_step`'s own two
stages collapse to

```rust
if let Some(operation_id) = self.latest_wins_order.items.front().copied() {
    return self.close_latest_wins_command_step(operation_id, maximum_items, maximum_bytes);
}
if !self.tool_operations.is_empty() {
    return self.close_typed_operation_step(maximum_items, maximum_bytes);
}
```

Retirement/drop discipline is untouched — every witness check, every `terminal_is_empty` assertion,
every fault code and every `drop(...)` moved with its stage, in order.

**Measured:** 692 080 → **470 320** bytes (−221 760, −32 %).

### 3.3 Why this is a runtime fix, not only a test fix

`PLUGIN_WASM_STACK_BYTES` is `8388608` (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:134`,
`-C link-arg=-zstack-size=8388608`) and the `wasm-dev` profile is unoptimized too, so the same
frames exist in the shipped plugin. Before this packet ONE `interactionSelect` consumed ~2.4 MiB of
that 8 MiB shadow stack before any host frames — 3.4× headroom for a single pick, and generation3d's
two-domain menu re-entered `resolve_domain_topology` twice per dispatch. Native, any plugin driven
from a plain `std::thread` (2 MiB) was already dead, which is exactly what the test harness proved.
After the packet a full construct-select-close lifecycle measures ~1.75 MiB.

---

## 4. Tests

### 4.1 New guard (framework level, where the defect is)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:2632`
`one_framework_reserved_route_fits_a_bounded_thread_stack` — builds the real registered
`VcsArtifactApp<TestApp>`, dispatches the real framework-injected `interactionSelect` on the declared
`items` domain and closes it, all on a `std::thread` with an explicit **2 MiB** stack (exactly what a
Rust-spawned thread gets; tighter than the 8 MiB wasm shadow stack). It is a *behavioural* guard, not
an assertion about source shape, so any future re-inflation of a frame on this route fails it.

```
test component::plugin_runtime::plugin_builder_contract_tests::one_framework_reserved_route_fits_a_bounded_thread_stack ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 601 filtered out
```

Threshold sweep on the same binary (`SEMIO_DEBUG_STACK_BYTES` override, removed before the final
build): 1 536 KiB → overflow, **1 792 KiB → ok**. So the guard passes with ~256 KiB of margin, and
would have failed by >1.3 MiB before §3.1.

The independent oracle for the property this test asserts is the **LLVM toolchain's own machine
code**: the `sub sp` prologue read out of the linked binary with `objdump` (third-party, outside this
codebase) is a second, static measurement of the same number the test exercises dynamically — the two
agree on every step of the table in §0.

`block_on` reaches the spawned thread through a **dev-only** feature edge added to
`🔌️plugin/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-async = { …, features = ["entrypoint"] }` under
`[dev-dependencies]` — the exact opt-in `block_on`'s own doc names for test consumers. No
interactive-reachable build sees the gate open (dev-dependencies are not built for
`cargo check -p semio-s-plugin-procedural`, native or wasm).

### 4.2 The tests named in the brief — exact filters, default stack, no `RUST_MIN_STACK`

`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --exact <name> --test-threads=1`
(run through the built binary directly, same effect):

```
editor::generation3d::component::tests::context_menu_reads_the_framework_owned_graph_selection        ok (1 passed)
editor::generation3d::component::tests::generation3d_interaction_selection_owns_its_persisted_history ok (1 passed)
editor::generation3d::component::tests::graph_selection_splits_into_node_and_edge_domains             ok (1 passed)
editor::generation3d::component::tests::no_pointer_down_route_survives_the_framework_owned_selection_domain ok (1 passed)
editor::generation3d::component::tests::context_menu_grouped_disclosure_stays_within_budget           ok (1 passed)
```

`context_menu_reads_the_framework_owned_graph_selection` prints its own witness:

```
[DEBUG] generation3d context menu unfolded 16 rows for one framework-owned graph selection
```

### 4.3 `grep INTERACTION_SELECT_ACTION_ID` under the three plugins' tests

- **generation3d** — 2 sites (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:531`, `:572`), both in the two tests above,
  both green.
- **generation2d** — **0 sites**. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/**` contains no
  `INTERACTION_SELECT_ACTION_ID`/`interactionSelect` reference at all; nothing to run.
- **puzzle3d** — 1 site (`🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1266`, inside
  `app_definition_labels_resolve_german_reuse_branded_for_aggregator`, which skips the six framework
  interaction action ids when auditing German labels — it declares the id, it does not dispatch):

```
editor::puzzle3d::component::tests::app_definition_labels_resolve_german_reuse_branded_for_aggregator ok (1 passed)
editor::puzzle3d::component::tests::app_definition_labels_stay_english_native_without_brand_locks     ok (1 passed)
```

---

## 5. Gates

Private target dirs (`$S/target-sel`, `$S/target-sel-wasm`), `RUSTC_WRAPPER=""`, `--keep-going`.
Warnings are reported because a zero error count means nothing if a crate aborted at expansion — all
four runs emit warnings, so all four are real type-checks.

| gate | result | raw |
|---|---|---|
| `cargo check -p semio-framework-plugin --keep-going` | **0 errors**, 0 warnings, `Finished dev` | `🗑️generated/selection-native-check-framework-plugin.txt` |
| `cargo check -p semio-s-plugin-procedural --keep-going` | **0 errors**, 7 warnings, `Finished dev in 30.44s` | `🗑️generated/selection-native-check-procedural.txt` |
| `cargo check -p semio-s-plugin-puzzle --keep-going` | **0 errors**, 7 warnings, `Finished dev in 28.83s` | `🗑️generated/selection-native-check-puzzle.txt` |
| `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev --keep-going` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`, own `target-sel-wasm`) | **0 errors**, 7 warnings, `Finished wasm-dev profile in 50.75s` | `🗑️generated/selection-wasm-check.txt` |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run` | **0 errors**, 17 warnings | — |
| `cargo test -p semio-framework-plugin --lib --no-run` | **0 errors** | — |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib --no-run` | **0 errors** | — |

None of the warnings is this packet's: `unused import: SpaceMember`
(`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs:24`), `unused extern crate`
(`🌀️generation2d/🦀️.rs:7`, `🧊️generation3d/🦀️.rs:7`), `unnecessary qualification`
(`🌀️generation2d/…/🧬️mutations/🦀️.rs:171`) — all pre-existing, all in files this packet did not open.
The one warning this packet *did* introduce (`unnecessary qualification` on
`crate::testkit::close_registered_fixture_app`) was fixed in place; the framework test build is back
to 0 owned warnings.

**Peer churn note.** The first wasm run at 22:2x died on
`error[E0592]: duplicate definitions with name 'with_children'` at `🔌️plugin/🦀️.rs:7275`/`:7283` — a
concurrent lane's in-flight `ArtifactView` refactor, repaired by its author within minutes. Every
figure above is from the re-run against the repaired tree.

---

## 6. Still red — attribution, not this packet's

- `editor::generation3d::component::tests::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`
  **no longer aborts** (it used to die on the same overflow), and now fails on a real assertion at
  `✏️editor/🧪️tests/🔬️unit/🦀️.rs:16` — the left preview window's `preview_eval_text` is empty after
  `drain_flow_eval_ticks_with_view`. That is the tessellation/preview lane's, not the selection path's.
- `component::plugin_runtime::plugin_builder_contract_tests::ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs`
  (framework, untouched) fails on `artifact store reached Drop without its exact terminal-empty
  shallow-shell witness` (`🏪️store/🦀️.rs:17671`) — the 2026-09-08 retirement invariant against a test
  that never closes its app. The sibling lane owns it; the new guard in §4.1 sidesteps it by closing
  its app through `testkit::close_registered_fixture_app`.
- `editor::puzzle3d::component::tests::document_and_kinds_trees_use_german_reuse_section_labels`
  (out of scope, run opportunistically) still overflows, at a completely different site owned by
  puzzle3d: `Puzzle3dPlayApp::initial_snapshot` → `Puzzle3dPrecomputeSession::drop` →
  `pump_fill_terminal_step` → `fill_envelope_registry`'s `OnceLock` initialiser. Same *class* of
  defect (an oversized unoptimized frame), different owner and different file; worth its own packet.
- `editor::generation3d::component::tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows`
  still overflows at the default stack, at a **generation3d-owned** site: the crash frame is
  `Generation3dPreviewCommandWork` (a 29 960-byte struct, `-Zprint-type-sizes`) moved twice inside
  `retained_command::ArtifactRetainedCommandJob<EditorApp<…>>::close_step`, reached through
  `drive_preview_operation`. Not this packet's frames: it needs 2.0–2.5 MiB, and this packet's §3.2
  split is stack-neutral on that path by construction (`close_step` 470 320 + `close_typed_operation_step`
  219 680 = 690 000, against the 692 080 the single body used to reserve — −2 080 bytes). Given 4 MiB
  it clears the overflow and then panics on the sibling lane's `ordered-map root must be explicitly
  retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:81`), so it is blocked twice over by other lanes.
- `editor::generation3d::component::tests::two_instances_converge_disjoint_widget_moves` also
  overflows — its own `assert_two_registered_instances_converge` generator frame is 229 152 bytes on
  top of a 788 096-byte test-closure frame (`🗑️generated/selection-frame-sizes-before.txt`). Testkit,
  not selection.
- The remaining `FAILED` rows in the same module (`all_bundled_examples_emit_preview_meshes`,
  `command_ids_are_unique_and_cover_every_row`, `document_from_mesh_returns_valid_default_snapshot`,
  `each_example_loads_distinct_fixture_and_preview_geometry`,
  `generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`, `preview_payload_has_meshes_and_instances`,
  `rectangle_wire_preview_emits_edge_only_mesh`, `retained_route_dispositions_are_exact_and_exhaustive`,
  `sun_measures_are_exposed_on_preview_windows`) are assertion failures in the mesh/preview/example
  lanes, none of them an abort and none of them on the selection path. Full module transcript:
  `🗑️generated/selection-generation3d-component-suite.txt`.
- `VcsArtifactApp::snapshot()`'s ordered-map abort-on-drop (`📓️editor-gaps-2026-09-09.md` §7) is
  untouched by design — the sibling lane owns it.

---

## 7. Files changed

| path | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | **new** `framework_reserved_route_job` (`:13935`); `dispatch_framework_reserved_action` (`:21294`) collapsed from 19 `run_framework_reserved_job` awaits to one job build + one await; **new** `close_latest_wins_command_step` (`:19323`) and `close_typed_operation_step` (`:19345`) extracted verbatim out of `PluginApp::close_step` (`:23173`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | **new** `one_framework_reserved_route_fits_a_bounded_thread_stack` (`:2632`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` | `[dev-dependencies]` edge on `semio-framework-async` with `features = ["entrypoint"]`, for that test's `block_on` |

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` carried a temporary `[DEBUG]`
backtrace probe in `interaction_topology` during §1.2 only; it was removed and the file is back to
its incoming content. No generation3d, generation2d or puzzle source was changed by this packet.

Raw logs: `🗑️generated/selection-*.txt`.
