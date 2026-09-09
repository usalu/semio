# Extension Round-Trip — `Emit::extension_invocations` Lands, and Both Generation Editors Retain Their Session

Ticket 2026/09/09/PROCEDURAL-3D-END-TO-END, W1 lane "extension continuation wiring", continuation of
`📓️extension-continuation-2026-09-09.md` §5. Session ⚪9f5f6952 (Fable 5.1), second run after the
machine reboot killed the first at ~17:30. Repo MCP was down for the whole session (`invalid
initialize params`); ticket bookkeeping is on disk and no ticket was opened/closed/reopened.

---

## 1. What was already on disk, and what this run added

The pre-reboot lane had landed most of §5 uncommitted but had **never compiled or run any of it**.
This run reconciled that state, closed the remaining holes, and produced the first green evidence.

| area | state found on disk | this run |
|---|---|---|
| kernel `Effect::InvokeExtension` `#[non_exhaustive]` + `Effect::invoke_extension` | landed | verified, unchanged |
| `RequestRegistry::request_continuation` / `take_continuation` | landed | verified, unchanged |
| `reactor::queue_extension_invocation` / `extension_response_args` / `take_extension_response` | landed | verified, unchanged |
| `Event::Completed` continuation branch in `poll` | landed | verified, unchanged |
| `Emit::extension_invocations` + `ExtensionInvocation` + `mint_extension_invocations` fail-closed guard | landed | verified, unchanged |
| all four producers migrated off hand-minted `RequestId` | landed | verified, unchanged |
| `Generation3dInstanceOperationOwner` (Defect B) | landed | verified, unchanged |
| `Generation2dInstanceOperationOwner` (Defect B) | landed | verified, unchanged |
| **generation2d `flowEvalTick` still `BatchOnlyPendingRewrite`** | **hard-dead at dispatch** | **fixed** |
| **generation2d `flowEvalResolve` had no unit test** | missing | **added** |
| **every test written by the previous lane** | never executed | **all now run and pass** |

### 1a. Design decision recorded (deviation from §5's literal wording, kept deliberately)

§5 asked for "a parked guest task awaiting the `RequestFuture`". The implementation on disk instead
uses a **redispatch continuation slot** (`Slot::Continuation`) and this run keeps it, because the
parked-task shape is not reachable from a plugin:

- `ArtifactApp::handle` is a pure synchronous reducer, and `Emit::tasks` is `#[cfg(test)]`-only
  (`🔌️plugin/🦀️.rs:9645`) — `dispatch_emit` rejects a non-empty task lane in production. A plugin
  therefore has **no place to hold a `RequestFuture` across turns**.
- The continuation slot carries identical semantics (one effect out, one `Event::Completed` back,
  answered exactly once) with resumption expressed as a follow-up dispatch — Elm's Msg-from-Cmd.
- The id still comes from the **same counter** as `RequestRegistry::request`, so a continuation id can
  never collide with a parked future's, and `RequestFuture::poll` fail-closes on a continuation id
  rather than consuming it (`📮️requests/🦀️.rs:422`).

Nothing on the wire changed: `req` is a `u64` whoever mints it, so WIT, `🖼️wire-turn.ts`,
`PluginRuntime.wireEffectToFriendly`, ShellHost, the wasmtime host and every checked-in jco `.d.ts`
are byte-identical. Only the *origin* of `req` moved.

---

## 2. The mechanism as it now stands (three hops, all covered by tests)

1. **App declares, never mints.** A command handler pushes an `ExtensionInvocation { extension_id,
   capability, request_json, response_action }` onto `Emit::extension_invocations`. Pushing a raw
   `Effect::InvokeExtension` is impossible outside the kernel crate (`#[non_exhaustive]` variant,
   `🎠️kernel/🦀️.rs:537`) and is additionally rejected at drain time with an explicit fault
   (`🔌️plugin/🦀️.rs:19890`).
2. **SDK mints and parks.** `mint_extension_invocations` drains the lane through
   `reactor::queue_extension_invocation`, which allocates the id via
   `RequestRegistry::request_continuation`, records `{response_action, request_json}` against it and
   queues the `Effect::InvokeExtension` onto the registry's own outbound queue — drained into
   `turn-result.effects` later in the SAME turn.
3. **SDK answers by dispatch.** `Event::Completed { req, result }` first asks
   `take_extension_response`; a continuation id yields `(instance, response_action, args)` and is
   dispatched through `plugin_dispatch_response_action` into the same app instance. `args` is the
   ORIGINAL request object's own fields (`nodeHash`, `handle`, …) merged with `ok` plus either
   `outputJson` or `faultCode`/`faultMessage` — the SDK never invents a key the app did not send. A
   non-continuation id falls through to `RequestRegistry::resolve` exactly as before.

---

## 3. Diff summary, file:line, per crate

### 3a. `semio-framework` — kernel (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`)

| file:line | change | author |
|---|---|---|
| `:529-537` | `Effect::InvokeExtension` doc rewritten + `#[non_exhaustive]` — no crate outside the kernel can write the struct literal | pre-reboot lane |
| `:686-693` | `Effect::invoke_extension(..)` — the single in-repo constructor | pre-reboot lane |

### 3b. `semio-framework-plugin` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`)

| file:line | change | author |
|---|---|---|
| `⚛️reactor/📮️requests/🦀️.rs:30` | `Slot::Continuation { response_action, request_json }` | pre-reboot lane |
| `⚛️reactor/📮️requests/🦀️.rs:170` | `pub struct ExtensionContinuation { instance, response_action, request_json }` | pre-reboot lane |
| `⚛️reactor/📮️requests/🦀️.rs:235` | `RequestRegistry::request_continuation` — same id counter as `request` | pre-reboot lane |
| `⚛️reactor/📮️requests/🦀️.rs:251` | `RequestRegistry::take_continuation` — answers once, `None` for a parked future | pre-reboot lane |
| `⚛️reactor/📮️requests/🦀️.rs:422` | `RequestFuture::poll` fail-closes on a continuation id instead of cancelling it | pre-reboot lane |
| `⚛️reactor/🦀️.rs:709` | `queue_extension_invocation` | pre-reboot lane |
| `⚛️reactor/🦀️.rs:721` | `extension_response_args` — request fields ∪ outcome | pre-reboot lane |
| `⚛️reactor/🦀️.rs:745` | `take_extension_response` | pre-reboot lane |
| `⚛️reactor/🔄️turn/🦀️.rs:284-296` | `Event::Completed`'s continuation branch → `plugin_dispatch_response_action` | pre-reboot lane |
| `🦀️.rs:9618` | `Emit::extension_invocations` lane | pre-reboot lane |
| `🦀️.rs:9687` | `pub struct ExtensionInvocation` + `::new` | pre-reboot lane |
| `🦀️.rs:19885-19893` | `mint_extension_invocations` + fail-closed rejection of a hand-pushed `Effect::InvokeExtension` | pre-reboot lane |
| `🦀️.rs:30772` | `plugin_dispatch_response_action` | pre-reboot lane |
| `⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs` (NEW, 69 lines) | 4 SDK unit tests | pre-reboot lane |
| `⚛️reactor/📮️requests/🧪️tests/🔬️unit/🦀️.rs:128-178` | 4 registry continuation tests | pre-reboot lane |
| `🦀️.rs:15181` | `ArtifactFixedRegistry::insert_admitted` → `pub(crate)` | **this run** |
| `🦀️.rs:16861 / :16876 / :16895 / :16911 / :16913 / :16922` | `ActiveArtifactStoreReplacementState`, `ActiveArtifactStoreReplacement`, its `active_member_open`/`committed`/`state` fields and `::new` → `pub(crate)` | **this run** |
| `🦀️.rs:17669 / :17754 / :17798` | `VcsArtifactApp::window_transient_store` / `store_replacement_jobs` / `child_content_generation` → `pub(crate)` | **this run** |
| `🧪️tests/🧩️composition/🦀️.rs` (21 sites) | `super::X` → `crate::app::X`; `ComposedParentApp::<true>::` turbofish at `:189/:192/:195` | **this run** |

⚠️ The last four rows are **repairs, not features**: a peer's composable-artifact refactor had moved
those declarations into `mod app` and left the crate's own `#[cfg(test)]` composition module unable
to name them, so `cargo test -p semio-framework-plugin` could not link **at all** (22 errors) — the
SDK crate is this lane's lease, so the lane repaired its test target rather than reporting the gate
as unrunnable. No item was made `pub`; every widening is `pub(crate)`, so the crate's public API is
byte-identical. (Note: an automated peer sweep reverted the `insert_admitted` widening once mid-run;
it was re-applied and is present on disk as of this report.)

### 3c. generation3d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

| file:line | change | author |
|---|---|---|
| `✏️editor/🦀️.rs:107-169` | `Generation3dInstanceOperationOwner { eval_session, closing }` + `with_session` + `maintenance_step`/`close_step`/`terminal_is_empty` | pre-reboot lane |
| `✏️editor/🦀️.rs:161` | `with_scratch_session` — the ownerless `render`/`handle` fallback, retired through `begin_close` + granted `close_step` | pre-reboot lane |
| `✏️editor/🦀️.rs:417-478` | `Generation3dFlowEvalWindowWork` — `flowEvalTick` on the RETAINED session | pre-reboot lane |
| `✏️editor/🦀️.rs:517-552` | `Generation3dSessionCommandWork` — retained-session twin of `BoundedArtifactCommandWork` | pre-reboot lane |
| `✏️editor/🦀️.rs:969` | `build_instance_operation_owner` | pre-reboot lane |
| `✏️editor/🦀️.rs:1063-1068` | work selection: `flowEvalTick` → window work, preview ids → preview work, rest → session work | pre-reboot lane |
| `✏️editor/🦀️.rs:2054-2078` | `preview_tessellate_invocations` — `ExtensionInvocation::new("brep","tessellate",…,"flowTessellateResolve")` replaces the `RequestId(105)` effect | pre-reboot lane |
| `✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:27` | `ExtensionInvocation::new(pending.extension_id,"evaluate",…,"flowEvalResolve")` replaces `RequestId(104)` | pre-reboot lane |
| `✏️editor/🦀️.rs:90-92 / :277-279 / :635-637 / :1131-1133 / :1288-1296 / :1459-1461 / :1535-1537` | both resolve commands declared, retained, published `HostOnly`, proofed, `Migrated` | pre-reboot lane |
| `✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs` | **rewritten**: explicit `ColdRetire` retirement of both `Dictionary`s and of the borrowed `Arc<NeuralCache>` before the session claims its cache root | **this run** |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs:27` | `close_registered_fixture_app(&mut *app)` — the `Box<VcsArtifactApp<…>>` needed a deref after a framework signature change | **this run** |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:4-22, :57` | `retire` rewritten against the CURRENT `RetirementCursor` API (`close_step(maximum_bytes) -> RetirementStep{Child,Bytes,Complete,BudgetExhausted}`, no `begin_close`) and the `include_str!` fixture depth corrected from `../../../../` to `../../` | **this run** (adopted repair) |

### 3d. generation2d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

| file:line | change | author |
|---|---|---|
| `✏️editor/🦀️.rs:59-107` | `Generation2dInstanceOperationOwner` + `with_session` + close protocol | pre-reboot lane |
| `✏️editor/🦀️.rs:217-255` | `Generation2dSessionCommandWork` | pre-reboot lane |
| `✏️editor/🦀️.rs:618` | `build_instance_operation_owner` | pre-reboot lane |
| `✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:27` | `ExtensionInvocation::new(…,"flowEvalResolve")` replaces `RequestId(101)` | pre-reboot lane |
| `✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` (NEW) | the resolve route: `seed_node_cache` + re-arm | pre-reboot lane |
| **`✏️editor/🦀️.rs:181`** | **`GENERATION2D_BOUNDED_TOOL_IDS` += `"flowEvalTick"` (13 → 14)** | **this run** |
| **`✏️editor/🦀️.rs:403`** | **`PUBLICATION_CONTRACTS` += `flowEvalTick`, lane `HostOnly` alone (the tick emits no mutation lane)** | **this run** |
| **`✏️editor/🦀️.rs:710`** | **`bounded_first_step_tool_proofs!` += `"flowEvalTick"`** | **this run** |
| **`✏️editor/🦀️.rs:1073`** | **`flowEvalTick` `BatchOnlyPendingRewrite` → `Migrated`** | **this run** |
| `✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs:26-30` + `🧪️tests/🔬️unit/🦀️.rs` (NEW) | resolve unit test mounted, mirroring generation3d | **this run** |
| `✏️editor/🧪️tests/🔬️testkit/🦀️.rs:36-56` | `retire_flow_eval_session` (delegates to the editor's own `close_flow_session`) + `empty_history_view` | **this run** |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs:181-183` | counts 13 → 14 | **this run** |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs:187` | `"flowEvalTick"` removed from the blocked-route list | **this run** |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs:271` | `ids.len()` 22 → **21** — the assertion was already wrong at HEAD (20 rows asserted as 21) and the previous lane's +1 preserved the off-by-one | **this run** |

**Why 3d's fourth block matters:** per
`📓️…/project-interactive-job-classification-gates-dispatch`, a `BatchOnlyPendingRewrite` command is
hard-dead at dispatch, not merely unoptimised. generation2d's whole evaluation chain starts at
`flowEvalTick`, so with the tick dead its brand-new `flowEvalResolve` route could never fire and the
extension round-trip was end-to-end unreachable in 2d regardless of the framework fix. It is now
`Migrated` **and** routed through `Generation2dSessionCommandWork`, i.e. onto the app instance's
retained `FlowEvalSession`.

### 3e. flow artifact (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/…/✏️editor/`)

| file:line | change | author |
|---|---|---|
| `🎮️commands/⏱️flow-eval-tick/🦀️.rs:53-63` | `ExtensionInvocation::new(…,"flowEvalResolve")` replaces the `RequestId(106)` effect | pre-reboot lane |
| `🦀️.rs:1627-1633` | resolve runs inside `FlowInstanceOperationOwner::with_session` | pre-reboot lane |

No change was needed in this run; two compile errors observed here at 18:02
(`_context`/`NoConfig {}` at `🦀️.rs:858`/`:1631`) were repaired by a peer before 18:30 and the crate
now checks clean.

---

## 4. Evidence — every number below is from a run in this session

Lane: private `CARGO_TARGET_DIR=$S/target-rt` (APFS clone of `target/debug`, 92 GB, seeded in 74 s),
`RUSTC_WRAPPER=""`, `--keep-going`. The shared `target/` was never touched (the react boot lane owns
it).

### 4a. Checks — all four green

| crate / target | result | warnings |
|---|---|---|
| `semio-framework-plugin` (native) | `Finished dev in 21.62s`, **0 errors** | 0 |
| `semio-s-plugin-procedural` (native) | `Finished dev in 52.48s`, **0 errors** | 1, all in `semio-framework-os-flow` (peer's `SpaceMember` unused import) |
| `semio-s-plugin-flow` (native) | `Finished dev in 56.39s`, **0 errors** | 1 (`semio-framework-os-flow`) + 11 (`semio-s-artifact-flow-flow`, peer-owned unnecessary-qualification / unused-import) |
| `semio-s-plugin-procedural` **`--target wasm32-wasip2 --profile wasm-dev`** (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | `Finished wasm-dev in 1m 24s`, **0 errors** | 1 (`semio-framework-os-flow`) |

The wasm lane is the one that matters for `#[cfg(target_arch = "wasm32")]` code, and it compiled
`semio-s-artifact-procedural-generation2d`, `…-generation3d` and `semio-s-plugin-procedural` in
sequence with no diagnostics of their own.

Raw logs: `🗑️generated/extension-round-trip-check-{sdk,procedural,flow,wasm}.txt`.

> On warning counts as a type-check witness: in `check` mode these crates are genuinely
> warning-clean, so the witness is the `lib test` build instead —
> `semio-framework-plugin (lib test) generated 56 warnings` and
> `semio-s-artifact-procedural-generation3d (lib test) generated 5 warnings` both appear alongside a
> successful link, i.e. neither crate aborted at module expansion.

### 4b. SDK unit tests — 8/8 pass

```
running 8 tests
test component::reactor::extension_continuation_tests::an_unknown_id_is_not_claimed_by_the_continuation_branch ... ok
test component::reactor::requests::tests::resolve_does_not_consume_or_corrupt_a_continuation ... ok
test component::reactor::requests::tests::take_continuation_answers_once_and_never_claims_a_parked_future ... ok
test component::reactor::extension_continuation_tests::a_queued_invocation_allocates_a_registry_slot_and_one_effect ... ok
test component::reactor::requests::tests::cancel_instance_sweeps_a_continuation_of_that_instance_only ... ok
test component::reactor::extension_continuation_tests::a_completion_for_a_minted_id_dispatches_the_response_action_with_the_outcome ... ok
test component::reactor::requests::tests::request_continuation_queues_one_effect_and_shares_the_request_id_counter ... ok
test component::reactor::extension_continuation_tests::a_faulted_invocation_dispatches_the_response_action_with_the_fault ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 590 filtered out; finished in 0.01s
warning: `semio-framework-plugin` (lib test) generated 56 warnings
```

These cover both halves of the §5 acceptance criterion verbatim: a drained `Emit` entry allocates a
real registry slot and queues exactly one effect carrying that minted id; a subsequent
`Event::Completed` for that id dispatches `response_action` with the payload — and, additionally,
that a fault still reaches the app as `ok:false` + typed fault, that a continuation answers exactly
once, that an unknown id falls through to `resolve`, and that `cancel_instance` sweeps continuations
per-instance.

Raw log: `🗑️generated/extension-round-trip-test-sdk.txt`.

### 4c. generation3d — 5/5 pass

```
running 5 tests
test editor::generation3d::commands::flow_tessellate_resolve::tests::tessellate_result_resolves_the_pending_handle ... ok
test editor::generation3d::commands::flow_tessellate_resolve::tests::unknown_node_hash_resolves_nothing ... ok
test editor::generation3d::component::tests::command_ids_are_unique_and_cover_every_row ... ok
test editor::generation3d::component::tests::retained_route_dispositions_are_exact_and_exhaustive ... ok
test editor::generation3d::commands::flow_eval_resolve::tests::eval_result_seeds_the_node_cache_and_rearms_the_tick_chain ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 276 filtered out; finished in 0.01s
warning: `semio-s-artifact-procedural-generation3d` (lib test) generated 5 warnings
```

Raw log: `🗑️generated/extension-round-trip-test-generation3d.txt`. Two real defects were found and
fixed here by *running* the previously-unrun test: the `Dictionary` comparison dropped a live neural
owner (`final Dictionary ownership must be explicitly retired or owned by a cold boundary`), and the
test-held `Arc<NeuralCache>` prevented `Arc::into_inner` from claiming the cache root during session
retirement (`final NeuralCache must be explicitly retired`).

### 4d. generation2d — 7/7 pass

```
running 7 tests
test editor::generation2d::commands::flow_eval_resolve::tests::eval_result_seeds_the_node_cache_and_rearms_the_tick_chain ... ok
test editor::generation2d::component::tests::command_ids_are_unique_and_cover_every_row ... ok
test editor::generation2d::component::tests::declared_actions_bridge_to_commands ... ok
test editor::generation2d::component::tests::every_command_round_trips_through_text_and_binary ... ok
test editor::generation2d::component::tests::every_printed_op_line_starts_with_the_rows_wire_keyword ... ok
test editor::generation2d::component::tests::retained_route_dispositions_are_exact_and_exhaustive ... ok
test editor::generation2d::component::tests::the_manifest_stitches_every_taxonomy_node ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 218 filtered out; finished in 0.02s
```

`declared_actions_bridge_to_commands` passing is the specific proof that the reclassified
`flowEvalTick` is registry-consistent end to end (declaration ↔ command ↔ bounded factory).

### 4e. Two editor-suite tests that abort — both pre-existing and peer-owned, NOT this lane

Running the *whole* `component::tests` module of either artifact aborts the test binary. Attributed,
not hand-waved:

| test | panic | owner |
|---|---|---|
| `generation3d::…::context_menu_grouped_disclosure_stays_within_budget`, `generation2d::…::add_widget_materializes_declared_kind_default_into_an_operation` | `artifact store cursor disposer reached Drop before terminal-empty ownership` (`🏪️store/🦀️.rs:2117`) raised from `ArtifactStoreCursorDisposer<…Config, …ConfigMutation>::drop` while the test merely drops its `VcsArtifactApp` | peer — `🧰️framework/…/🏪️store/🦀️.rs` is uncommitted-modified by the composable-artifact/store refactor; the tests call no `close_registered_fixture_app` |
| `generation3d::…::all_bundled_examples_emit_preview_meshes` | `ordered-map root must be explicitly retired before drop` (`🌱️value/🗂️ordered/🦀️.rs:81`, committed `de617a7c17`, 2026-09-08 23:25) | peer — the value/ordered-map retirement refactor; the test is untouched by this ticket's diff |

Neither is reachable from any line this lane wrote, and neither touches the eval session, the
request registry or the continuation branch. They are the standing "explicit retirement discipline"
debt landing across the repo, and they belong to whoever owns `🏪️store` and `🌱️value/🗂️ordered`.

---

## 5. What remains owed

1. **Descriptor regeneration for the procedural plugin.**
   `bun nx run @semio-tech/os-plugin-describe-rs:describe -- procedural` was **not run**: its
   `dependsOn: ["build"]` forces a full component-wasm build in the SHARED `target/`, which the react
   boot lane owns, and `lsof -nP -iTCP:6018 -sTCP:LISTEN` showed **no listener** at 18:35 — i.e. the
   boot lane's serve was not up, so the shared dir could not be entered under the agreed protocol.
   `✏️s/🔌️plugins/🌀️procedural/🛂️.descriptor.semio` and `…/🔣️.json` therefore still describe the
   pre-continuation action set (27 generation3d actions, and generation2d's `flowEvalTick` still
   classified `BatchOnlyPendingRewrite`). They MUST be regenerated before the plugin registry or the
   native `run` host is trusted (`🏃️run/🦀️.rs:1687` faults on a stale/missing descriptor).
2. **Runtime confirmation of the whole loop.** Every hop is now unit-tested, but no console log from
   a live `dev` boot has yet shown a `brep` `tessellate` outcome arriving as a `flowTessellateResolve`
   dispatch and painting a mesh. That is the react boot lane's gate, and it needs (1) first.
3. **The two peer-owned abort classes in §4e.** They block running either artifact's *full* editor
   suite in one process; exact-filter runs are unaffected.
4. **The peer's `semio-framework-os-flow` unused-import warning** (`🖥️host/🦀️.rs:24`, `SpaceMember`)
   is the single warning riding along in three of the four checks; left untouched as peer-owned.
