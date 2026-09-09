# Wave X — puzzle3d test suite: unblocking the harness and triaging what it found

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Scope: test infrastructure only
(`✏️editor/🧪️tests/**`, the 3d subset's other unit-test files, and the crate's Rust package script).
No production file was edited. Everything below was **run**, not read.

---

## 1. TL;DR

- **The suite now runs.** It did not before: the binary aborted on the very first app-driving test with
  `fatal runtime error: stack overflow` and never printed a summary. Four independent faults had to be
  removed to get there ([§2](#2-why-the-binary-aborted--four-root-causes)).
- **The harness was rebuilt on the framework's own contracts** ([§3](#3-what-changed-in-the-harness)):
  `#[semio_framework_async_macros::async_test]` instead of `resolve_ready`, one registry-backed and
  instance-bound fixture, a `settle` step that runs the host's real continuation turn, and a `Drop`
  that never panics.
- **`cargo test` is a usable gate again, and it is red for production reasons.** Measured with one
  process per test: **577 tests, 405 pass, 172 fail, 0 abort.** Every failure buckets into one of
  **10 named production defects**, none of them in a file this wave owns
  ([§4](#4-test-results), [§5](#5-production-bugs-found-do-not-paper-over-these)). The three largest:
  the canonical-JSON/fixture float mismatch (87), the Config publication lane admitting exactly one
  mutation variant (20), and `setActiveTool`/`setActiveUtility` having no tool proof (17).
- **One whole-suite invocation still aborts**, from two production bugs of its own: `⏳️precompute` drops
  framework job owners without their incremental close, and it only reaches that state because the
  process-global fill registry saturates across tests. Until both are fixed, this crate must be measured
  per test, not per suite.

---

## 2. Why the binary aborted — four root causes

Reproduced first, exactly as the brief asked, with
`RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-p3d cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4`:

```
running 561 tests
… add_object_kind_hostile_static_law_rejects_whole_catalog_conversion ... FAILED
… add_brush_object_hostile_static_law_rejects_engine_run_to_completion ... FAILED
… command_envelope_round_trip_holds_for_an_applied_operation ... FAILED

thread '…::tests::add_object_kind_honors_drop_origin' has overflowed its stack
fatal runtime error: stack overflow, aborting
… (signal: 6, SIGABRT: process abort signal)
```

### (a) The registry-less fixture cannot exist for this plugin
`testkit::new_app` builds `VcsArtifactApp::with_registry(app, AppActionRegistry::default())`, and
`with_registry_on_bus` then runs
`registry.tool_job_registration::<A>(&app_id, …)` (`🔌️plugin/🦀️.rs:16641`). puzzle3d declares
`bounded_first_step_tool_proofs!` (`✏️editor/🦀️.rs:6707`), so that join has no migrated generated
declarations to match and fails closed with `interactive-job.catalog-authority` before any dispatch.
**Fix:** the fixture is now `new_app_with_registry(puzzle3d_manifest_for_testkit)` + `bind_instance_id(1)`,
and the registry-less constructor is gone from this crate's harness entirely.

### (b) `resolve_ready` cannot drive a framework-reserved verb — by design
The six interaction verbs go `handle_action` → `dispatch_framework_reserved_action` →
`run_framework_reserved_job`, which calls `plugin_job_yield_once()` (`🔌️plugin/🦀️.rs:14282`). That
helper returns `Poll::Pending` exactly once, on purpose, after `cx.waker().wake_by_ref()`.
`semio_framework::io::resolve_ready` (`🚪️io/🦀️.rs:891-900`) polls **once** and panics on `Pending` —
so this was never a bug to fix in the app, it was the wrong executor. The framework's own answer is
`#[semio_framework_async_macros::async_test]` (`⏳️async/✨️macros/🦀️.rs`), a thread-park `block_on`,
which is what `🔱️trinity`, `🌊️flow` and `📕️norm` already use.
**Fix:** every app-driving helper is an `async fn` and every test that touches one runs on `async_test`.

### (c) libtest's 2 MiB per-test stack is too small for this crate's futures
Even after (a) and (b) the binary aborted with `fatal runtime error: stack overflow` inside
`add_object_kind_honors_drop_origin`. One `VcsArtifactApp` dispatch/render future plus the retained
tool-job poll chain under it does not fit an unoptimized 2 MiB frame.
**Fix:** `RUST_MIN_STACK` is set in the package's own `📜️script.ts` (see [§3](#3-what-changed-in-the-harness)),
so `nx test`, `bun ./📜️script.ts test` and the launch.json entries that call them inherit it on every
platform. An externally supplied value still wins.

### (d) A fixture `Drop` that asserts turns any failure into a process abort
A registry-backed `VcsArtifactApp` installs framework-owned `ArtifactStoreCursorDisposer` members whose
own `Drop` asserts terminal-empty ownership (`🏪️store/🦀️.rs:2017`). The `📸️process3d` fixture pattern
(`assert!(std::thread::panicking(), …)`) therefore converts the *first failing assertion in any test*
into a second panic during unwinding — a non-unwinding abort that kills the binary and hides the suite
summary. Observed verbatim:

```
assertion `left == right` failed
  left: 1
 right: 2
…
🏪️store/🦀️.rs:2017: artifact store cursor disposer reached Drop before terminal-empty ownership
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```

**Fix:** `Puzzle3dApp::drop` never panics and never asserts. It drains through the real close state
machine, and if it cannot reach the witness it `std::mem::forget`s the raw app — leaking a fixture inside
a test process costs nothing, and the close contract is instead stated **once, explicitly**, by the new
`close_witness` helper and the `fixture_app_reaches_its_terminal_empty_close_witness` test. That test is
currently RED, and its message is production bug [P1](#p1-puzzle3d-can-never-close).

---

## 3. What changed in the harness

| File | Change |
| --- | --- |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` | rebuilt (see below) |
| `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 305 helper call sites awaited, 40 `resolve_ready` wrappers unwrapped to `.await`, 3 remaining `#[test]`s moved to `async_test`, 11 hostile-static-law mutations repaired, 3 stale source literals repaired, 2 re-arity'd call sites, 1 explicit import, 1 new test |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | `process.env.RUST_MIN_STACK ??= 128 MiB` before the shared router runs |

### 3.1 The fixture

`Puzzle3dApp` is now a wrapper (`Option<Puzzle3dRawApp>` + `Deref`/`DerefMut`), so every existing
`&mut Puzzle3dApp` signature in the tests kept working unchanged. `app()` is the ONLY constructor:
registry-backed and instance-bound. `app_with_registry()` is deleted — there is no second kind of
puzzle3d app, and keeping two names for one thing was the inconsistency that hid fault (a) for a wave.

### 3.2 `settle` — one real host actor turn

The decisive finding. **A registry-backed `VcsArtifactApp` does not apply a retained tool job inline.**
`dispatch_typed` mints a `ToolOperationSpec`; its worker, store publication, result page and the three
typed outboxes are advanced only by later continuation turns. The registry-less fixture used to see
mutations land synchronously, which is why ~100 assertions in this file were written as if dispatch were
atomic. `dispatch()` now ends with a `settle` loop that mirrors `advance_typed_operation_output`
(`🔌️plugin/🦀️.rs:28550`) exactly:

```
maintenance_step(1, 16_384)             // worker + retirement stages
advance_typed_operation_publication()   // one publication unit
take_typed_operation_result_page(1)     // + acknowledge_typed_operation_result  ← without the ACK the app never retires
take_typed_operation_effect/event/ui_scope()
take_local_interaction_query_reply()    // + acknowledge_local_interaction_query
```

The drained effects/events/UI scope are folded back into the returned `InvocationResult`, so a test reads
the same triple a client would after the continuation turns. A result page on
`TypedOperationResultLane::Fault` is asserted against, with its body in the message — that is what turned
a silent hang into the precise production faults in [§5](#5-production-bugs-found-do-not-paper-over-these).

Three things were tried and rejected before this shape, recorded so nobody repeats them: exiting on
`has_runnable_typed_operations` alone never terminates (the outboxes and the local-interaction query
keep it true); `advance_typed_operation_publication` alone makes no progress at all (the operation sits
in the `Worker` stage, which only `maintenance_step` advances); and taking the result page without
`acknowledge_typed_operation_result` leaves the operation in `AwaitingAck` forever.

### 3.3 Reserved-verb routing

The old reserved list carried verbs that do not exist (`"checkpoint"`, `"alternative"`,
`"historyFilter"`) and was missing `setActiveUtility`/`setActiveTool`. It is now exactly the framework's
own `skip` set (`🔌️plugin/🦀️.rs:6362-6395`) — every verb the framework injects and handles itself.
(Routing those two to `handle_action` does not make them work; see [P2](#p2-setactivetool--setactiveutility-are-hard-dead-under-a-registry).)

### 3.4 Hostile-static-law repairs

All eleven `*_hostile_static_law_*` tests were failing. Two independent reasons, both test-side:

1. **`replacen(marker, …, 1)` cannot flip its own predicate.** Every stage marker occurs at least twice
   in `✏️editor/🦀️.rs` — once in the `enum` declaration, once in the `match` arm — so deleting the first
   occurrence leaves the second and `source.contains(marker)` stays true. Measured, per marker:
   `Puzzle3dScaleStage::ObjectSelection` ×2, `Puzzle3dSetActiveExampleStage::Publish` ×3,
   `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` ×17. Changed to `replace(marker, …)` in all 11 loops, which
   is the mutation the law actually means: *this stage is absent from the source*.
2. **Three pinned source literals went stale** when production reformatted the `build_tool_job` arms.
   `| "suggestionsTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))` and
   `| "engagementInput" => Box::new(Puzzle3dScalarConfigWork::new(tool_id))` both had a new alternative
   inserted between the id and the `=>` (and the `=>` moved onto its own line), and
   `Puzzle3dEngagementSubmitStage::UtilityConfig` was renamed to `…::Parse`
   (`✏️editor/🦀️.rs:5574-5583`). Repinned whitespace-independently (id half and `=> Box::new(…)` half
   asserted separately) so a reformat cannot break them again; the routing they check is unchanged and
   correct in production. `engagement_submit` also gained the per-stage hostile loop its ten siblings
   already had.

All 15 static-law tests now pass: `test result: ok. 15 passed; 0 failed`.

---

## 4. Test results

Command (from the repo root):

```
RUST_MIN_STACK=134217728 CARGO_TARGET_DIR=…/target-p3d RUSTC_WRAPPER="" \
  cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 -- --test-threads=1
```

A single whole-suite invocation still cannot print a summary, because two production defects
([P7](#p7-precompute-drops-framework-job-owners-without-their-incremental-close)) abort the process from
a destructor once earlier tests have saturated the process-global fill registry ([P8](#p8-fill-worker-admission-is-refused-after-earlier-tests-in-the-same-process)).
The authoritative measurement is therefore **one process per test** — no cross-test pollution, no
aborting neighbour. Reproduce with `🔍️isolate-puzzle3d-tests.py`, kept beside this report:

```
tests 577
total 577 ok 405 failed 172        (0 aborted)
```

Every one of the 172 buckets into a named production defect below. Counted by root cause:

| n | root cause |
| ---: | --- |
| 87 | [P9](#p9-mutation-fixtures-vs-the-canonical-json-writer) canonical-JSON / committed-diff mismatch on integral floats |
| 20 | [P3](#p3-the-config-publication-lane-admits-exactly-one-mutation-variant) Config lane rejects its mutation, or the lease is cancelled behind it |
| 17 | [P2](#p2-setactivetool--setactiveutility-are-hard-dead-under-a-registry) `interactive-job.missing-factory` on `setActiveTool` / `setActiveUtility` |
| 7 | [P10](#p10-worker-pump-rejects-the-reserved-mounted-worker-transition) `interactive-job.worker-pump` — reserved mounted worker transition rejected |
| 4 | [P6](#p6-a-standalone-puzzle3dstore-cannot-record-an-edit) `edit history insertion requires its exact mutation retirement factory` |
| 3 | [P5](#p5-the-inspection-panels-fixed-ui-map-overflows-on-a-live-object-selection) `ui.fixed-capacity` admission failures (inspection panel, outliner) |
| 2 | [P4](#p4-the-document-one-item-publication-has-no-preinstalled-capacity-for-a-large-example) `one-item publication requires preinstalled fixed applied and revision capacity` |
| 2 | [P7](#p7-precompute-drops-framework-job-owners-without-their-incremental-close) job owner dropped without its incremental close |
| 1 | [P1](#p1-puzzle3d-can-never-close) `close-owned-disposer-missing` for `draft-store` |
| 1 | [P8](#p8-fill-worker-admission-is-refused-after-earlier-tests-in-the-same-process) fill-worker admission refused |
| 28 | single-test behavioural failures, enumerated in [§7](#7-the-remaining-28-single-test-failures) |
| **172** | |

By module: `standards::…::mutations` 87 (all P9), `editor::puzzle3d::component::tests` 58,
`editor::puzzle3d::precompute::component::tests` 15, `…::precompute::fill::tests` 6,
`…::panels::document::tests` 2, four others 1 each.

For contrast: a **shared-process** run of the same suite reaches
`test result: FAILED` for 43 tests and then aborts at
`🧵️job/🦀️.rs:2440 rejected worker session admission requires exact incremental close` →
`panic in a destructor during cleanup` → `SIGABRT`. Any future measurement of this crate has to isolate
until P7 and P8 are fixed.

Compile gate, for the record:

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) — 0 errors
```

---

## 5. Production bugs found — do not paper over these

Every item below is reachable from a plain `app()` + one dispatch. None is in a file this wave owns, and
none was worked around in the tests.

### P1 puzzle3d can never close
`✏️editor/🦀️.rs` — `Puzzle3dPlayApp` declares only `build_document_store_disposer` (6699) and
`build_config_store_disposer` (6703). `VcsArtifactApp::close_step` walks seven owned lanes in a fixed
order (`🔌️plugin/🦀️.rs:21509-21516`) and every lane is mandatory —
`drive_artifact_owned_disposer` (`:12387`) faults the moment `A::build_<lane>_disposer()` returned `None`:

```
Fault { code: "interactive-job.close-owned-disposer-missing",
        message: "app owner did not provide the required bounded disposer for draft-store" }
```

So `draft-store`, `presence-store` and `transient-store` are unreachable, the `interaction-store` lane
behind them never runs, and no puzzle3d instance can reach its terminal-empty witness — in a test process
OR in a host, where the same `close_step` runs on shutdown and the framework-installed
`ArtifactStoreCursorDisposer` members then panic in `Drop`. `🌊️flow` shows the whole fix, three lines
(`✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs:1801, 1813, 1817`), each delegating to an existing framework
helper (`bounded_document_store_disposer::<NoDraft, NoDraftMutation>()`,
`NoTransientStoreDisposer::new()`).
**Test:** `fixture_app_reaches_its_terminal_empty_close_witness` (new, RED).

### P2 `setActiveTool` / `setActiveUtility` are hard dead under a registry
```
Fault { code: "interactive-job.missing-factory",
        message: "typed command 'setActiveTool' has no exact controller/owner/factory/tool/schema proof" }
```
`dispatch_action` (`🔌️plugin/🦀️.rs:19608`) routes a framework-injected host-configuration verb through
`A::host_configuration_mutation(action, args)`. **puzzle3d does not implement that hook at all** — the
trait default returns `None` — so both verbs fall to `admit_command_json` and fail closed. Neither is in
`PUZZLE3D_RETAINED_TOOL_IDS` (deliberately: `retained_command_catalog_excludes_framework_owned_shared_actions`
asserts it), so there is no proof to find. Note the asymmetry: `Puzzle3dCommand::SetActiveTool` **is**
declared (`✏️editor/🦀️.rs:1889`) while `setActiveUtility` has no variant at all.
`🏭️process3d` shows the shape of the fix (`✏️editor/🦀️.rs:1581`).
**17 tests blocked**, including every fill-tool test (they all begin by selecting the fill tool) and
every transform/gumball/utility test.

### P3 the Config publication lane admits exactly one mutation variant
`✏️editor/🦀️.rs:6544-6552`:

```rust
fn puzzle3d_config_store_mutation_bytes(mutation: &Puzzle3dConfigMutation) -> Option<usize> {
    match mutation {
        Puzzle3dConfigMutation::Snapshot { config } => puzzle3d_config_store_bounded_bytes(config).ok(),
        _ => None,
    }
}
```

`Puzzle3dConfigStorePreparation::advance` (`:6600`) rejects anything this returns `None` for:

```
retained operation faulted: Puzzle3d Config preparation rejected its exact mutation envelope
```

and when the preparation errors the operation's publication lease is cancelled, which surfaces on other
routes as the sibling message

```
retained operation faulted: typed-operation cancelled before its next publication unit
```

Every non-`Snapshot` variant is therefore unpublishable: `SetWindowCamera`, `SetWindowSun`,
`SetWindowGridSpacing`, `SetWindowVoxelDims`, `SetOverlapBudget`, `SetSuggestionMenu`,
`SetBrushCandidateIndex`, `SetWindowEngagementInput`, `SetActiveUtility`, `SetObjectKindWeights`,
`SetVortexKindWeights`. That is the **entire `Puzzle3dScalarConfigWork` family** (setCamera, setProjection,
setProjectionParam, toggleSun, setSun*, setLod*, setGrid*, setSelectableKind, setProximityRadius,
setChunkSize, setVoxelDims, setTransformGumballFlag, setVortexShow, setVortexDirection,
setBrushPlacementOverlapBudget, closeVortexSuggestions, hoverSuggestion, engagementControlSelect,
engagementInput) plus the kind-weight and engagement routes. **20 tests blocked in isolation** (more in a shared
process, where a cancelled lease cascades). This is exactly the class of defect the ticket exists to eliminate: the route
compiles, dispatches, and publishes nothing.

### P4 the document one-item publication has no preinstalled capacity for a large example
`nakagin_example_loads_via_operations`:
```
retained operation faulted: validation failed:
one-item publication requires preinstalled fixed applied and revision capacity
```
`Puzzle3dArtifactStorePreparationFactory` / `build_document_store_owners` do not preinstall enough fixed
applied+revision capacity for the Nakagin example's edit count.

### P5 the inspection panel's fixed UI map overflows on a live object selection
`selected_object_inspector_renders_that_object_field_group`:
```
render: Fault { origin: Plugin, code: "plugin.internal",
        message: "ui.inspection.fields: puzzle3d inspector action map entry admission failed" }
```
`✏️editor/📌️panels/🔍️inspection/🦀️.rs:73` — the `UiMapBuilder::try_new()` fixed capacity is smaller than
the object field group this wave's sibling (W-S) rewrote it to emit.

### P6 a standalone `Puzzle3dStore` cannot record an edit
`command_envelope_round_trip_holds_for_an_applied_operation`:
```
apply: ValidationFailed("edit history insertion requires its exact mutation retirement factory")
```
`Puzzle3dStore::new(create_document_envelope(…))` yields a store with no mutation retirement factory, so
`store.dispatch(ArtifactCommand::Apply { … })` fails. Either the store constructor must install the
factory or the CW7 law needs a different entry point.

### P7 `⏳️precompute` drops framework job owners without their incremental close
Two sites, both `debug_assert!` violations that abort the whole test binary from a destructor:

- `✏️editor/⏳️precompute/🦀️.rs:472-476` — on a rejected fill-worker admission it calls `begin_close()`
  then **one** `close_step(…)` and drops the value regardless of whether it reached
  `terminal_is_empty()`. `WorkerJobSessionAdmissionRejected::drop` (`🧵️job/🦀️.rs:2438-2441`) then fires
  *"rejected worker session admission requires exact incremental close"*.
- `✏️editor/⏳️precompute/🦀️.rs:1237` — `self.fill_rejected_worker = None;` drops a non-terminal rejected
  session the same way.
- The same class, third site: `RetainedJobPayload::drop` (`🧵️job/🦀️.rs:660-666`) fires *"RetainedJobPayload
  requires one-page close to terminal-empty"* from `precompute::fill::tests::stale_generation_faults_without_progress`.

These are what still prevent one whole-suite invocation from printing a summary.

### P8 fill-worker admission is refused after earlier tests in the same process
`set_fill_count_step_work_advances_a_real_admitted_fill_plan` fails on
`a fresh session with no live job must admit a fill worker` — it passes in isolation. The fill worker
registry is process-global (`fill_envelope_terminal_intents()`, `FILL_ENVELOPE_MAX_OPERATIONS`) and is
not released when a session ends, so it saturates across the suite. This is also what *produces* the
rejections P7 then mishandles.

### P9 mutation fixtures vs. the canonical JSON writer
Every `standards::…::mutations::<verb>::tests_<case>::{committed_json_is_canonical,
committed_diff_is_canonical, produces_committed_diff}` fails on integral floats:

```
left  (dsl::json::to_json_string of the decoded snapshot):  "gap": Number(1.0), "origin": [Number(0.0), …]
right (the committed 🔣️.json fixture, reparsed):            "gap": Number(1),   "origin": [Number(0), …]
```

The codec emits `1.0` for an `f64` field; the committed fixtures store `1`. **87 tests, one root cause**, spread across every one of the ~32
mutation verbs at once (`committed_json_is_canonical`, `committed_diff_is_canonical` and
`produces_committed_diff` for each). **Not fixed here on purpose:** these fixtures
are the language-agnostic oracle input shared with the Python second implementation
(`🧪️tests/🧊️mutate-puzzle-3d-1/🐍️.py`), so rewriting them would change the oracle, and if the writer
is the side that regressed it would cement the bug. Needs a decision from whoever owns `🧬️mutations`
before either side moves.

### P10 `worker-pump` rejects the reserved mounted worker transition
Seven tests, all on a plain `select_id` / `hover_id`:
```
Fault { code: "interactive-job.worker-pump",
        message: "framework reserved mounted worker transition was rejected" }
```
i.e. `run_framework_reserved_job`'s mounted worker for the reserved interaction verbs is refused a
transition. Every vortex/marker selection test in the suite goes through this.

---

## 6. The mutation harness (task 4)

`🧪️tests/🧊️mutate-puzzle-3d-1/{🥒️.feature, 🐍️.py, 🦀️.rs}` is **not a cargo test target**: no `[[test]]`
stanza in `📦️packages/🦀️rust/Cargo.toml`, and a repo-wide grep for `mutate-puzzle-3d-1` in `.rs` sources
returns only a docstring mention. It is a parity-runner case whose subject half deliberately replays the
committed vectors without linking the plugin crate (its own `🔮️oracle/🔣️.json` rationale says so).
`bun …/📦️packages/🦀️rust/📜️script.ts test` routes to `runArtifactRustTests` → `runCargoTestBudgeted` for
this crate — it never reaches the parity case. Run for real:

```
Summary [0.237s] 28/286 tests run: 25 passed, 3 failed, 0 skipped
  FAIL …::add_object_vortex::tests_appends_vortex_3_to_object_b::committed_json_is_canonical
  FAIL …::add_object_vortex::tests_appends_vortex_3_to_object_b::committed_diff_is_canonical
  FAIL …::add_object_vortex::tests_appends_vortex_3_to_object_b::produces_committed_diff
warning: 258/286 tests were not run due to test failure
error: test run failed
```

Two findings from that run, both worth the coordinator's attention:

1. **It is cargo-nextest, which isolates each test in its own process.** So the canonical gate is immune
   to [P7](#p7-precompute-drops-framework-job-owners-without-their-incremental-close)'s destructor abort,
   and it inherits the `RUST_MIN_STACK` this wave added to the package script. It does fail fast — the
   first three failures stop it after 28 of 286.
2. **The gate compiles the crate with DEFAULT features, so it runs none of the app tests.** 286 tests
   there versus 577 under `--features component-app-assembly`. The entire `editor` tree is
   `#[cfg(feature = "component-app-assembly")]` (crate root `🦀️.rs:14-21`), so
   `nx test @semio-tech/puzzle-3d-rs` has never compiled, let alone run, a single one of the 291
   `editor::puzzle3d::component::tests` — which is how a harness that cannot construct its own app
   survived a whole wave. Not changed here: `artifactRustCargoArguments` adds no `--features` for any
   artifact crate in the fleet (`🧰️framework/🛍️products/🦑️repo/…/🦀️cargo/📜️script.ts:61-67`), so this is a
   fleet-wide decision in a framework-owned script, not a puzzle3d one.

The 87 `standards::…::mutations::…` failures in [P9](#p9-mutation-fixtures-vs-the-canonical-json-writer)
are the cargo-side fixture tests for the very vectors that oracle compares against, and they are what to
fix before the parity case can mean anything. They are also the only puzzle3d failures the current gate
can see at all.

---

## 7. The remaining 28 single-test failures

- `component::tests::document_and_kinds_trees_use_german_reuse_section_labels` — `document tree objects section`
- `component::tests::fill_build_tick_work_spawns_the_isolated_planner_and_persists_the_checkpoint` — the completion emits no `Effect::SpawnJob` at all (`got []`)
- `component::tests::local_interaction_query_return_does_not_fault_the_next_maintenance_step` — the query never produces a terminal page within 2000 turns
- `component::tests::set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin` — `step()` does not reach `Complete` within its own declared extent
- `component::tests::two_instances_converge_disjoint_object_edits_via_backbone` — `attach_backbone` is fail-closed: *"remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized"*
- `component::tests::vortex_show_window_option_defaults_to_selected_and_switches_to_always` — `main window measures` absent
- `component::tests::vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards` — `main window measures` absent
- `component::tests::window_options_are_local_to_the_window_instance_not_shared_across_split_panes` — `base measures` absent
- `precompute::component::tests::dispatch_set_scene_then_apply_and_compose_fill_count_round_trip` — the base scene's host object does not survive `compose_fill_display(0)`
- `precompute::component::tests::fill_first_substantive_preview_arrives_below_fifty_ms_and_every_step_below_eight_ms` — no substantive preview at all (`None; stage=Some(PrepareCandidates); rejected=0`)
- `precompute::component::tests::fill_lane_advances_while_brush_targets_remain_queued` — `seed scene must schedule brush targets`
- `precompute::component::tests::fill_options_paths_are_millisecond_scale` — `applied fill objects must survive weight edits`
- `precompute::component::tests::fill_worker_actual_owner_census_rejects_cap_plus_one_with_exact_handback` — panics `index out of bounds: the len is 0 but the index is 0`
- `precompute::component::tests::fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close` — `left != right` failed
- `precompute::component::tests::fill_worker_cross_generation_restore_preserves_dropped_closing_handle_and_zero_credit` — `retained close cursor`
- `precompute::component::tests::fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase` — `the same terminal intent cannot mount twice after readiness is cleared`
- `precompute::component::tests::fill_worker_malformed_token_faults_exact_raw_owner_not_wrong_context_owner` — same terminal-intent message
- `precompute::component::tests::fill_worker_wrong_context_identity_faults_decoded_producer_before_drive` — same terminal-intent message
- `precompute::component::tests::fill_worker_session_drop_during_partial_close_rearms_the_same_cursor_once` — `partial retirement cursor`
- `precompute::component::tests::precompute_session_native_wrapper_exercises_public_methods` — the fixture no longer contains an object with id `host`
- `precompute::component::tests::set_scene_with_applied_fill_projection_preserves_slider_session` — `decreasing after sync`
- `precompute::component::tests::set_scene_with_identical_json_preserves_precompute_progress` — `precompute_step should have drained some queue items`
- `precompute::component::tests::update_kind_weights_soft_replans_tail_without_rebuilding_queue` — `queue_len_after_step < queue_len_after_seed` fails
- `precompute::fill::tests::all_fill_fixed_collections_store_max_entries_in_the_credited_page_and_return_plus_one` — `body`
- `precompute::fill::tests::constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently` — `body`
- `precompute::fill::tests::capacity_refusal_publishes_generation_qualified_no_ghost_diagnostic_before_fault` — `builder.preview.sequence > 0` fails
- `precompute::fill::tests::empty_fill_transition_stays_below_watchdog_ceiling` — `left == right` failed
- `precompute::geometry::tests::spatial_index_close_retains_bucket_values_and_retires_one_credited_owner_per_grant` — `bounded credit`

Twenty of the twenty-eight sit in `⏳️precompute/**` — W-F's area, and several of them
(`fill_worker_*`, `capacity_refusal_*`, `constructor_cap_*`) are the very tests
`📓️2026-09-08-wave-F-fill-capacity.md` §NOT-verified listed as written-but-never-executed. They are
executed now, and they are red. The four `component::tests` window/measure failures
(`main window measures` / `base measures` absent) are the same symptom as [P3](#p3-the-config-publication-lane-admits-exactly-one-mutation-variant)
one level up: the window never registers because its registering dispatch could not publish.

---

## 8. Not verified

1. **No runtime/browser confirmation.** Nothing here was observed in a running app.
2. **P9's direction is undetermined** — this report states the mismatch, not which side is authoritative.
3. **The batched runner is not a substitute for a green suite.** Until P7 is fixed, one whole-suite
   invocation still aborts, and any future measurement has to use the same batching.
4. **Cross-test pollution is real** (P8). Counts from per-test isolation and counts from a 24-test batch
   can differ for the fill tests specifically.
5. **`settle`'s effect/event/scope folding is a modelling choice.** The host forwards these to the shell;
   folding them into the returned `InvocationResult` is what makes the pre-existing
   `result.requested_effects` / `result.ui_scope` assertions meaningful again, but no test yet pins that
   the folded triple equals what a real client observes.
