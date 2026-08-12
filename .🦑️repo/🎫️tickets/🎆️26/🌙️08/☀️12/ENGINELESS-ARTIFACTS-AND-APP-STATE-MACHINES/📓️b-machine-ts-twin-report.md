# Workstream B — `🔄️machine` TypeScript twin

Scope: add `🧰️framework/🔨️modules/🔄️machine/🟦️component.ts`, the TS twin of `🦀️component.rs` (2579
lines), following the existing multi-implementation convention. Rust side untouched.

## Convention mirrored — and why the ticket brief's assumption was wrong

The brief said to mirror `🧬️schema`, `🎠️kernel`, `🎯️action-bus` and expected each to have its own
`📦️packages/🟦️typescript/` (package.json/tsconfig.json/project.json/script.ts/index.ts). **That isn't
what those three modules actually do.** Checked all three on disk:

- `🎠️kernel/` and `🎯️action-bus/` contain *only* `🟦️component.ts` + `🦀️component.rs` — no TS package
  folder at all, no per-module tests.
- `🧬️schema/` has `📦️packages/🦀️rust/` (its own Rust crate, like `machine` has) but **no**
  `📦️packages/🟦️typescript/` — same bare `🟦️component.ts` at the module root.

All six framework-root modules with a TS twin (`schema`, `kernel`, `action-bus`, `platform`, `mesh`,
`manifest`) are wired into **one shared package**: `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`
re-exports every module's `component.ts` with `export * from "../../🔨️modules/<name>/🟦️component.ts"`,
and `🧪️vitest.config.ts` there only ever includes `🟦️glue.ts` itself (`include: ["🟦️glue.ts"]`,
`includeSource: ["🟦️glue.ts"]`) — none of the six `component.ts` files contain `import.meta.vitest`
blocks; **every test for every framework-root module lives centrally in `glue.ts`**. The
`📦️packages/🟦️typescript/` *per-module* pattern (package.json + tsconfig.json + project.json +
script.ts + index.ts) is real, but it belongs to a different tier: `math`, `assets`, `ui`, `2d`, `3d` —
modules published/consumed as standalone units, not siblings of `machine`.

**What I actually did:** added `🟦️component.ts` at `🧰️framework/🔨️modules/🔄️machine/🟦️component.ts`
(module root, no package scaffolding), registered it in `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`
(`export * from "../../🔨️modules/🔄️machine/🟦️component.ts";`), and added all new tests to `glue.ts`'s
existing `if (import.meta.vitest)` block, exactly where `schema`/`kernel`/`action-bus`'s tests already
live. No new `project.json`/`package.json`/`script.ts` — `@semio-tech/framework`'s existing ones already
cover the new file. No `launch.json` change needed — I didn't add a new nx target, only extended the
existing `@semio-tech/framework:test` target's file coverage.

Checked for export-name collisions between `machine`'s new exports and the other five modules' before
wiring `export *` (`grep`-swept every exported identifier name across all six `component.ts` files) —
none found (`step`/`init` false-positive hits were parameter names in `platform`/`kernel`, not exports).

## Ported symbols

Full surface from the ticket brief, all present in
`🧰️framework/🔨️modules/🔄️machine/🟦️component.ts`:

- **Ids**: `NodeId`, `EventId`, `TransitionId`, `GuardId`, `ActionId`, `InvokeId`, `TimerId`, `ActorId`,
  `ROOT` — branded `number` types (`type NodeId = number & { readonly __machineId: "NodeId" }`) with a
  same-named constructor function each, since Rust's newtype tuple structs have no zero-cost TS
  equivalent; branding still catches accidental id-space mixups at compile time.
- **Configuration**: `Configuration` interface, `ConfigurationIter`, `BitSet` class (backed by
  `Set<number>`, ascending-sorted iteration to match Rust's `ConfigurationIter`).
- **Traits**: `StatechartEvent`, `Machine<M>`, `MachineSpec` (new — see below).
- **Schema/tables**: `GuardFn`, `ActionFn`, `InputFn`, `OutputFn`, `MachineDefinition`, `NodeKind`,
  `NodeDef`, `Trigger`, `TransitionKind`, `TransitionDef`.
- **Commands**: `Command` (discriminated union), `CommandSink`.
- **Snapshot**: `Status`, `Snapshot` class (`matches`, `historyFor`/`recordHistory`/`historyEntries` in
  place of Rust's `pub(crate)` history field, `branchForExploration` for `explore`'s BFS), `StepReport`.
- **Lifecycle**: `init`, `macrostep`, `timerElapsed`, `MICROSTEP_LIMIT` — full behavioral port of the
  SCXML-style microstep/macrostep algorithm (domain computation, conflict resolution, history
  shallow/deep, parallel regions, `on_done` bubbling), not a stub.
- **Inspection**: `InspectionEvent`, `Inspector`, `NullInspector`, `MicrostepTrace`, `TraceInspector`.
- **Host**: `Host`, `NativeHost` (wall-clock via `Date.now()`), `TestHost` (caller-driven simulated
  clock).
- **Persistence**: `persist`, `restore`, `PersistedSnapshot`, `Migration`, `RestoreError`.
- **Routing**: `route_command` → `routeCommand`, `ActorSystem` (`spawnRoot`/`send`/`drain`/
  `timerElapsed`/`snapshot`).
- **Step pair**: `start`, `step` — same guarantee as Rust: the live `Snapshot<M>` is created and
  discarded inside one call; only `PersistedSnapshot` and plain `MachineStep` data cross the boundary.
  `MachineStep` class with `isActive`.
- **Verification**: `explore`, `check_invariants` → `checkInvariants`, `run_conformance` →
  `runConformance`, `Model`, `Coverage`, `Invariant`, `ConformanceStep`, `FsmError`.

## Rust constructs that don't map cleanly

1. **`Machine::definition() -> &'static MachineDefinition<Self>` (static/associated-function
   dispatch).** TS has no static dispatch over a type parameter. Every kernel entry point
   (`init`/`macrostep`/`timerElapsed`/`persist`/`restore`/`explore`/`runConformance`/`start`/`step`)
   takes an explicit `Machine<M>` value instead of inferring it from `M`. Decision: this is the
   idiomatic TS shape (pass data, don't fake static polymorphism), not a workaround.
2. **`M: Machine` associated types (`M::Context`, `M::Event`, …).** No associated-type syntax in TS.
   Added `MachineSpec` — one generic parameter bundling `Context`/`Event`/`Input`/`Output`/`Effect`,
   accessed as `M["Context"]` etc. This is the standard TS idiom for this exact situation and reads
   very close to the Rust call sites.
3. **`BitSet<const W: usize>`.** Rust's `statechart!` macro sizes a fixed word-array bitset per
   machine at compile time. TS has no const generics. Collapsed to one dynamically-sized `BitSet`
   (backed by `Set<number>`) shared by every machine — `Machine::Config` is therefore not a per-machine
   associated type in the TS twin, it's always `Configuration`/`BitSet`. Rust's
   `bitset_iter_ones_spans_words` test (asserting ascending order across a `u64` word boundary) is
   ported as an ascending-order assertion only — "spans two words" is meaningless for a `Set`-backed
   twin, noted inline in the ported test.
4. **`fingerprint: u64`.** `number` loses precision above 2^53. Used `bigint` for
   `MachineDefinition.fingerprint`/`PersistedSnapshot.fingerprint`/`Migration.sourceFingerprint`.
5. **`Result<T, E>` / `thiserror`.** No `Result` type in TS and CLAUDE.md forbids pulling in a runtime
   library (e.g. `neverthrow`) for this. Used plain discriminated-union returns:
   `{ok:true; value} | {ok:false; error}` for `restore`/`step`/`runConformance`, and a
   `FsmError = {kind:"violation"; message}` object instead of a `thiserror` enum.
6. **`M::Context: Clone` bound on `explore`.** TS has no trait-bound equivalent enforced by the type
   system. `Snapshot.branchForExploration()` uses `structuredClone` on the context at runtime instead
   — works for plain data (the common case) but not for contexts holding functions or class instances
   with private state, unlike Rust's compile-time-checked `Clone`. Documented on the method; this is a
   real, if narrow, behavioral gap the report flags rather than hides.
7. **`ActorLogic` trait + its blanket `MachineLogic<M>` impl.** Purely a marker abstracting over an
   actor's associated types so a hypothetical non-machine `ActorLogic` could exist later. TS's
   structural typing means `MachineSpec` already provides exactly those types to anything that needs
   them — the marker layer has no work to do and was omitted (documented in `ActorSystem`'s docstring)
   rather than added as dead ceremony.
8. **`StatechartSchema` trait (`SCHEMA_JSON: &'static str` associated const).** Its own Rust docstring
   says its purpose is "feeds TypeScript generation tooling" — i.e. it exists so *Rust* struct field
   metadata can drive *TS* type/schema generation. On the TS side there is nothing to generate: the
   hand-written TS types already are the target. Omitted; noted here rather than in-code to avoid
   dead-code ceremony per CLAUDE.md's "concise code" rule.
9. **`#[cfg(feature = "macros")]` (`statechart!`, `export_wasm_machine!`, `StatechartEvent`/
   `StatechartSchema` derive macros).** TS has no macro/derive system. Every hand-authored test fixture
   below therefore writes its `NodeDef`/`TransitionDef` tables directly instead of through a DSL — this
   is the same tradeoff Rust itself takes when `feature = "macros"` is off.

## WasmBridge decision

Rust's `wasm_bridge` module (`WasmHost`, `#[cfg(target_arch = "wasm32")]`) is **not ported**. Reasoning:

`WasmHost` exists to run a `statechart!`-compiled machine *inside* a wasm binary, receiving JS timer
polls and forwarding effects out to a JS callback via `js_sys`/`wasm_bindgen`. That whole bridge only
has a caller when some Rust crate compiles a concrete machine to wasm (`export_wasm_machine!`) for a
JS/TS host to drive it. `🔄️machine` itself never does this outside its own `wasm_smoke` test — it's a
library other crates build machines with. The framework already has exactly the JS-side consumer this
bridge exists to talk to: `🎠️kernel/🟦️component.ts`'s `PluginWasmHandle`/`PluginWorkerClient`/
`loadPluginModuleViaWorker` region drives component-model wasm plugins over a binary `exchange` ABI —
the real "TS talks to a wasm machine" story runs through *that* existing bridge, not through a
reimplementation living inside `machine`'s own twin.

**Decision: the TS twin is the consumer, not a reimplementation** — exactly the hint in the brief. If
and when an app compiles its own `statechart!` machine to wasm via `export_wasm_machine!`, its TS side
drives it through the existing kernel-module plugin bridge, using the *pure-TS* `Host`/`ActorSystem`
machinery ported here for the (more central, per the ticket's own thesis — "apps are the TS side") case
where the app runs its state machine natively in TS with no wasm boundary at all. No `WasmHost` class,
no `wasm_smoke`-equivalent, added to the twin.

## Tests

Extended `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`'s existing `if (import.meta.vitest)` block —
no new test file, per convention and per CLAUDE.md. Every Rust test case (`🧪️Tests` region, lines
2299–2579, plus the per-submodule `🧪️Tests` regions earlier in the file) has a TS counterpart:

| Rust test | TS counterpart |
|---|---|
| `test_host_advance_fires_due_timers_only`, `test_host_cancel_timer_removes_pending`, `test_host_records_effects_and_task_lifecycle` | `describe("machine: TestHost")`, 3 cases |
| `trace_inspector_records_one_microstep_per_transition` | `describe("machine: TraceInspector")` |
| `flat_machine_toggles_and_counts`, `guard_blocks_transition_when_false` | `describe("machine: kernel")`, `ToggleMachine` fixture |
| `hierarchical_machine_enters_default_descendant`, `hierarchical_machine_transitions_into_compound_default`, `shallow_history_restores_last_active_child` | same describe, `PlayerMachine` fixture |
| `parallel_regions_enter_together`, `parallel_done_bubbles_only_once_every_region_finishes` | same describe, `RecorderMachine` fixture |
| `persist_then_restore_round_trips_active_state`, `restore_rejects_fingerprint_mismatch_without_migration`, `restore_applies_migration_chain_until_fingerprint_matches` | `describe("machine: persist/restore")` |
| `actor_system_drains_sent_events_through_one_macrostep_each` | `describe("machine: ActorSystem")` |
| `explore_reaches_both_toggle_states`, `conformance_fixture_passes_for_matching_sequence`, `conformance_fixture_fails_with_descriptive_message`, `invariant_reports_violation_by_name` | `describe("machine: testing (…)")` |
| `bitset_set_clear_contains`, `bitset_iter_ones_spans_words` (order-only, see mapping note 3), `bitset_clear_all_and_is_empty` | `describe("machine: BitSet")` |
| `dsl_machine_walks_cart_to_receipt`, `dsl_machine_cancel_resume_round_trips_via_shallow_history`, `dsl_machine_coverage_reaches_every_declared_state`, `start_produces_a_persistable_initial_configuration`, `step_round_trips_through_persisted_state_only`, `step_reports_entered_and_exited_states`, `step_with_a_blocked_guard_leaves_the_configuration_untouched`, `step_rejects_a_persisted_snapshot_from_another_machine_shape` | `describe("machine: checkout DSL twin (integration)")` — hand-compiled `CHECKOUT_MACHINE` fixture reproducing the Rust `statechart!` DSL's node/transition tables directly (no macro on the TS side, see mapping note 9); node/transition ids chosen by me (documented inline), not required to match Rust's macro-assigned ids since only observable behavior needs to agree |
| `null_inspector_observes_nothing_observable` | not ported — a Rust compile-only smoke test with no TS analogue (TS has no separate "does this compile without a concrete type" check); `NullInspector` is exercised indirectly by every other test that passes it |

30 new `it(...)` cases added (`43 → 73` in `glue.ts`, confirmed via `git show HEAD` diff of the test
count).

## Verify — real command output

```
$ bun nx run @semio-tech/framework:test
...
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript
 ❯ |@semio-tech/framework| 🟦️glue.ts (73 tests | 2 failed) 41ms
     × stores a function-typed init as the current value (not as a lazy factory) 8ms
     × stores a no-op function init without invoking it 0ms
 ❯ |@semio-tech/framework| 🟦️glue.ts (73 tests | 2 failed) 45ms
     × stores a function-typed init as the current value (not as a lazy factory) 9ms
     × stores a no-op function init without invoking it 4ms

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 4 ⎯⎯⎯⎯⎯⎯⎯
 FAIL  |@semio-tech/framework| 🟦️glue.ts > ephemeralBox > stores a function-typed init as the current value (not as a lazy factory)
ReferenceError: ephemeralBox is not defined
 ❯ 🟦️glue.ts:463:19
 FAIL  |@semio-tech/framework| 🟦️glue.ts > ephemeralBox > stores a no-op function init without invoking it
ReferenceError: ephemeralBox is not defined
 ❯ 🟦️glue.ts:473:19

 Test Files  2 failed (2)
      Tests  4 failed | 142 passed (146)
   Duration  736ms
```

Full output: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/scratch-machine-ts-vitest-run.txt`.

The suite runs twice per invocation (an existing quirk of this vitest config, not something I changed —
73 tests × 2 = 146, 2 failures × 2 = 4), so the real per-run picture is **73 tests, 71 passed, 2
failed**. **All 30 new `machine`-prefixed tests pass, in both runs.** The 2 failing tests
(`ephemeralBox` describe block) are **pre-existing and unrelated to this change** — confirmed via
`git show HEAD:"🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts"`, which already calls `ephemeralBox(...)`
without importing it, before any of my edits landed. I did not fix it (out of my assigned scope, and
this is a shared multi-session tree); flagged it separately via `spawn_task`
(`task_78aba787`, title "Fix missing ephemeralBox import in framework glue.ts tests"). Because of this
pre-existing failure, `bun nx run @semio-tech/framework:test` exits non-zero overall — **not** a clean
green run, and I am not claiming otherwise.

Also ran a scoped `tsc --noEmit` against just `🟦️component.ts` + `🟦️glue.ts` (ad-hoc tsconfig in the
scratchpad, since the package has no dedicated `typecheck`/`lint` nx target — confirmed
`bun nx run @semio-tech/framework:lint` errors with "Cannot find configuration for task", and neither
`kernel`/`action-bus`/`schema` have one either). Zero errors originate in
`🔄️machine/🟦️component.ts`. `glue.ts` showed only pre-existing issues (the same missing
`ephemeralBox`/`PluginRegistryEntry`/`PluginSourceEvent` imports, confirmed pre-existing via the same
`git show HEAD` check) plus `TS5097` "import path can only end with .ts" noise from my scratch
tsconfig lacking `allowImportingTsExtensions` (a flag the project's real vite/vitest pipeline sets;
every sibling `component.ts` import triggers the same false positive under my ad-hoc config, so this is
a verification-harness artifact, not a real error). One real bug this caught and I fixed before the
vitest run: two of my own checkout-integration tests read `system.host.startedTasks()` /
`.cancelledTasks()`, but `ActorSystem.host` is typed as the `Host<M>` interface (which only has
`startTask`/`cancelTask`), not the concrete `TestHost<M>` — fixed by holding a separate
`const host = new TestHost<CheckoutSpec>()` reference for those assertions.

## Files touched

- **Created**: `🧰️framework/🔨️modules/🔄️machine/🟦️component.ts` (1132 lines).
- **Updated**: `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` — added the `export *` line, an import
  block for `machine`'s symbols, and ~430 lines of fixtures + 30 new tests inside the existing
  `if (import.meta.vitest)` block.
- **Scratch** (this ticket folder, `.txt`): `scratch-machine-ts-vitest-run.txt` (full vitest output).

## Gaps / honesty check

- Did **not** achieve a fully green `bun nx run @semio-tech/framework:test` — blocked by a pre-existing,
  unrelated `ephemeralBox` import bug, flagged separately (see above), not fixed by me.
- No `typecheck`/`lint` nx target exists for this package to run (matches sibling convention — none of
  `kernel`/`action-bus`/`schema` have one either); verified via a scoped ad-hoc `tsc --noEmit` instead,
  see above.
- `explore()`'s context-cloning (`structuredClone`) is a real, narrower guarantee than Rust's
  compile-time `M::Context: Clone` bound — documented on `Snapshot.branchForExploration`, not silently
  papered over.
- Rust's `null_inspector_observes_nothing_observable` (a compile-only smoke test) has no TS analogue;
  noted rather than force-fit.
