# B1 — Promote `🔄️fsm` to a core `🔄️machine` framework module

Wave B1, step 1 of the workstream-B sequence. **Status: PASS.**

## What changed

Created `🧰️framework/🔨️modules/🔄️machine/` as a new framework module with a proc-macro sibling, following the `🧬️schema` + `✨️derive` precedent. **The old `✏️s/🔌️plugins/🖍️draw/🔄️fsm/` crate pair is untouched and still a workspace member** — both pairs coexist, which is the whole point of the create-and-delete sequencing (see `📌️important.md`, "⛔ NEVER move a directory containing a `Cargo.toml`").

### Files created (11)

| Path (under `🧰️framework/🔨️modules/🔄️machine/`) | Content |
|---|---|
| `🦀️component.rs` | the 2,412-line statechart kernel, transcribed |
| `📦️packages/🦀️rust/📦️glue.rs` | `extern crate self as machine;` + `#[path]` mount |
| `📦️packages/🦀️rust/Cargo.toml` | `semio-framework-machine` |
| `📦️packages/🦀️rust/📋️project.json` | `@semio-tech/framework-machine`, `tags: ["scope:framework"]` |
| `📦️packages/🦀️rust/📜️script.ts` | test router → `runCargoTestBudgeted(["semio-framework-machine"])` |
| `✨️derive/🦀️component.rs` | the 1,520-line `statechart!` proc-macro, transcribed |
| `✨️derive/📦️packages/🦀️rust/📦️glue.rs` | thin proc-macro entry, `#[path]` mount |
| `✨️derive/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-machine-derive`, `proc-macro = true` |
| `✨️derive/📦️packages/🦀️rust/📋️project.json` | `@semio-tech/machine-derive-rs` |
| `✨️derive/📦️packages/🦀️rust/📜️script.ts` | test router |

Updated: root `Cargo.toml:91-92` — two members added, immediately after the existing `🧬️schema`/`🧮️math` framework-module entries in the "🧹️ Adopted" block. **Lines 66-67 (the old fsm members) are deliberately left in place.**

### The transcription, and how it was made exact

Rather than a hand-copy, the rename was derived from a measured inventory:

```
kernel  🦀️component.rs : 0 occurrences of `\bfsm\b`   → transcribes VERBATIM
                          3 occurrences of `fsm_macros` → `machine_derive`
derive  🦀️component.rs : 75 occurrences of `fsm`, of which 74 are `fsm::` paths
                          (the 75th is the line-1 module docstring)
```

The `\bfsm\b` word-boundary grep **missed `fsm_macros`** on the first pass — caught by a follow-up substring grep before compiling. Worth knowing for anyone doing a similar rename: `fsm_macros` is one word, so a word-boundary pattern silently skips it, and the resulting breakage would only appear at link time.

Kernel verified byte-identical to source immediately after copy (`diff -q` → identical), *then* the 3 `fsm_macros` refs renamed. Derive verified: 0 remaining `fsm` substrings, exactly 74 `machine::` paths.

### One deliberate deviation from the framework precedent

`🧬️schema/✨️derive` and `🗣️dsl/✨️derive` each keep **two byte-identical copies** of their macro source (`✨️derive/🦀️component.rs` and `✨️derive/📦️packages/🦀️rust/📦️glue.rs`) — cargo compiles the glue copy, so editing only the component file silently does nothing. At 1,520 lines that mirroring is a guaranteed drift bug.

**We did not copy that shape.** The old fsm macro crate already demonstrates the correct alternative in this repo — a thin `📦️glue.rs` that `#[path]`-mounts the component and exposes four `#[proc_macro]`/`#[proc_macro_derive]` wrappers — and we preserved it. Flagged here so a reviewer reading the "derive crates keep two copies" gotcha knows this one is intentionally different.

## Verification (real commands, real output)

**1. No dangling workspace member** — the failure mode that aborts cargo for every session on the machine, before compilation, and therefore cannot be hidden behind `--all-targets`:

```
$ # parse members[] out of root Cargo.toml, assert each has a Cargo.toml on disk
ALL MEMBER PATHS RESOLVE ✓

$ grep -n "fsm\|🔄️machine" Cargo.toml
66:    "✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust",
67:    "✏️s/🔌️plugins/🖍️draw/🔄️fsm/✨️macros/📦️packages/🦀️rust",
91:    "🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust",
92:    "🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust",
```

**2. Workspace graph loads:**

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo metadata --no-deps --format-version 1
WORKSPACE LOADS ✓
```

**3. The promoted crate builds and its own suite passes** (`scratch-machine-test-1.txt`):

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-machine
   Compiling semio-framework-machine v0.1.0
    Finished `test` profile [unoptimized] target(s) in 7m 59s

running 26 tests
test component::checkout_integration::dsl_machine_walks_cart_to_receipt ... ok
test component::checkout_integration::dsl_machine_cancel_resume_round_trips_via_shallow_history ... ok
test component::checkout_integration::dsl_machine_coverage_reaches_every_declared_state ... ok
test component::host::tests::test_host_advance_fires_due_timers_only ... ok
test component::host::tests::test_host_cancel_timer_removes_pending ... ok
test component::host::tests::test_host_records_effects_and_task_lifecycle ... ok
test component::inspect::tests::null_inspector_observes_nothing_observable ... ok
test component::inspect::tests::trace_inspector_records_one_microstep_per_transition ... ok
test component::kernel::tests::flat_machine_toggles_and_counts ... ok
test component::kernel::tests::guard_blocks_transition_when_false ... ok
test component::kernel::tests::hierarchical_machine_enters_default_descendant ... ok
test component::kernel::tests::hierarchical_machine_transitions_into_compound_default ... ok
test component::kernel::tests::parallel_done_bubbles_only_once_every_region_finishes ... ok
test component::kernel::tests::parallel_regions_enter_together ... ok
test component::kernel::tests::shallow_history_restores_last_active_child ... ok
test component::persist::tests::persist_then_restore_round_trips_active_state ... ok
test component::persist::tests::restore_applies_migration_chain_until_fingerprint_matches ... ok
test component::persist::tests::restore_rejects_fingerprint_mismatch_without_migration ... ok
test component::runtime::tests::actor_system_drains_sent_events_through_one_macrostep_each ... ok
test component::testing::tests::conformance_fixture_passes_for_matching_sequence ... ok
test component::testing::tests::conformance_fixture_fails_with_descriptive_message ... ok
test component::testing::tests::invariant_reports_violation_by_name ... ok
test component::testing::tests::explore_reaches_both_toggle_states ... ok
test component::tests::bitset_iter_ones_spans_words ... ok
test component::tests::bitset_set_clear_contains ... ok
test component::tests::bitset_clear_all_and_is_empty ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Why this result is stronger than a `cargo check`.** The three `checkout_integration` tests are `statechart!` *invocations* — they only compile if the derive crate built, the `machine::` path rename is correct in every emitted path, and the self-alias in `📦️glue.rs` resolves. A green check would not have exercised the macro at all. Also passing: `persist_then_restore_round_trips_active_state` and the migration-chain test, which are the primitives workstream B depends on for event-sourced machine state, and `explore_reaches_both_toggle_states` / the conformance fixtures, which are the acceptance apparatus for the 53 handcrafted statecharts to come.

**4. Toolchain probe settling open risk #8** (`probe-atd.rs`, `probe-atd-result.txt`):

```
$ rustc --edition 2021 --crate-type bin -o probe-atd-bin probe-atd.rs   # exit 0, no warnings
$ ./probe-atd-bin                                                        # RAN OK
```

`#![feature(associated_type_defaults)]` **works** on `nightly-2026-07-07` (`rustc 1.99.0-nightly (c4af71034 2026-07-06)`). So `type Machine: AppMachine<Self> = NoMachine<Self>` is available, and the per-app migration cost for a not-yet-converted app is **zero lines** rather than 57 one-line edits. The fallback is no longer needed.

**5. Old crate still compiles side by side** (`scratch-old-fsm-test-1.txt`):

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-s-plugin-draw-fsm
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Both crate pairs green simultaneously, 26/26 each.** This is the invariant the create-and-delete sequencing exists to preserve: at no point between now and the eventual deletion is there a workspace member pointing at a path that does not exist, and draw keeps building against the old crate until it is explicitly repointed.

## Files touched

- **Created**: the 11 files above, plus `probe-atd.rs`, `probe-atd-result.txt`, `scratch-machine-test-1.txt`, `scratch-old-fsm-test-1.txt`, `scratch-metadata-err.txt` in this ticket folder.
- **Updated**: root `Cargo.toml` (2 lines added at :91-92).
- **Removed**: nothing. Deletion of the old pair is a later step, gated on draw being repointed.

## sharedFileRequests

None. Root `Cargo.toml` is not in any peer's claimed set, and the addition is purely additive.

## Concurrent-churn observations

- UCAS (#2548) initially reported a **live** `🔄️fsm` relocation collision with APA. Measured independently: the directory is at its original path, mtime **Aug 9 03:31** (3 days), both members intact, exactly one `*fsm*` hit under draw. UCAS re-measured and withdrew the report, attributing it to a transient mid-move state sampled hours earlier and generalised past its shelf life. **Nothing was racing.** This is the third instance in this tree of a derived observation outliving its validity — the standing rule (ask the session, never infer from files) held up again.
- Ownership of `🔄️fsm` settled without a negotiation: APA's relocation destination was draw's *artifact* engine tree, which this ticket abolishes outright, so both tickets agree on direction and only ours survives the taxonomy change. UCAS relayed and endorsed the framing.
- `🔌️plugin/🦀️component.rs` queue agreed as **APA → us → others**. UCAS is out except to repair composition items (`ChildEmit`, `ArtifactChildren`, `dispatch_group`, `SpaceMember` — ours to ping, never fix).
- Build times are long (~8 min for one small crate) due to ~29 concurrent rustc processes across six sessions. Expected; not a fault.

## Result

**PASS.** The core `machine` module exists, compiles, and passes 26/26 of the kernel's own tests including three full `statechart!` macro integrations. Workstream B is unblocked at the crate level.

---

# B1 step 2 — `MachineStep` + `start`/`step`

**Status: PASS.** Added `mod step` to `🔄️machine/🦀️component.rs` (regions `🔖️StepInspector`, `🔖️MachineStep`, `🔖️StepEntryPoints`), re-exported at the crate root as `pub use step::{start, step, MachineStep};`.

## The contract

```rust
pub struct MachineStep<M: Machine> {
    pub entered: Vec<&'static str>,     // stable ids, union across the macrostep's microsteps
    pub exited:  Vec<&'static str>,
    pub active:  Vec<&'static str>,     // the settled configuration — project from this
    pub commands: Vec<Command<M>>,      // what the machine DESCRIBED; it never executes
    pub report:   StepReport,
    pub persisted: PersistedSnapshot,   // the new logical state, ready to write to a lane
}

pub fn start<M: Machine>(input: M::Input) -> MachineStep<M>;
pub fn step<M: Machine>(prior: &PersistedSnapshot, context: M::Context, event: M::Event,
                        migrations: &[&dyn Migration]) -> Result<MachineStep<M>, RestoreError>;
```

**The load-bearing property:** `Snapshot<M>` is created and dropped inside `step`. Only `PersistedSnapshot` — plain data — crosses the call boundary. That is what lets a host keep machine state in one of its four state lanes rather than in a durable struct field or a `thread_local!`, satisfying DKM tier (d) by construction rather than by convention.

Two entry points rather than one `Option`-taking function, because `init` derives its own context from `M::Input` while `restore` is handed one: collapsing them would force callers to supply both and let one be ignored.

`entered`/`exited` are collected by a private `StepInspector` wired in place of `NullInspector` — the kernel already emitted `InspectionEvent::Microstep { exited, entered }`, so no kernel change was needed. They are the **union across microsteps in execution order**, so a node touched twice in one macrostep appears twice; this is documented on the struct, and `active` is the field to project from.

## Verification (real output)

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-machine --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1.20s        # zero warnings

$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-machine
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
                                                                  # 26 pre-existing + 5 new, 0 warnings
```

The five new tests, all in the existing `checkout_integration` module (no new test file, per the ticket rules), all passing:

```
start_produces_a_persistable_initial_configuration ... ok
step_round_trips_through_persisted_state_only ... ok
step_reports_entered_and_exited_states ... ok
step_with_a_blocked_guard_leaves_the_configuration_untouched ... ok
step_rejects_a_persisted_snapshot_from_another_machine_shape ... ok
```

They were chosen to pin the properties workstream B actually depends on, not to raise coverage: that a full read-transition-write cycle threads **only** `PersistedSnapshot` values between calls; that a `cart -> payment` confirm reports both the compound and its initial child as entered (what a host projects from); that a **rejected guard still settles** with an untouched configuration rather than erroring; and that a foreign fingerprint is **refused** rather than silently reinterpreted, which is the safety property behind evolving a machine's shape against an existing history.

## One implementation note worth carrying forward

The first compile failed with `unresolved imports crate::kernel, crate::persist`. Because the component file is `#[path]`-mounted as `mod component` inside `📦️glue.rs`, `crate::` from inside it resolves to the **glue crate root** (where `pub use component::*` re-exports the public surface), not to the component module. Private sibling modules are reached with `super::`. Compounding it, `crate::persist` *does* resolve — to the re-exported `persist` **function**, not the module — so a naive fix produces a confusing second error. Use `super::kernel` / `super::persist` for module paths and `crate::` only for re-exported items. Every module added to this file will hit this.

## Files touched (step 2)

- **Updated**: `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs` (+~100 lines: new `mod step`, one re-export line, five tests).
- **Created**: `scratch-step-check-1.txt`, `scratch-step-check-2.txt`, `scratch-step-test-1.txt`.

## Next

1. `🟦️component.ts` twin for the module (every sibling framework module has one; it should read `MachineDefinition.manifest_json`, never become a second interpreter).
2. Workstream C's `Emit` fourth lane and the `AppMachine` protocol — both land in `🔌️plugin/🦀️component.rs` and are **gated on APA finishing with that file**. Queue agreed as APA → us.
3. Workstream A's F1 (repeal the artifact-engine mandate) — independent of the above, but needs the `📜️script.ts` / `🔣️taxonomy.json` write-queue renegotiated first, since the standing agreement names a taxonomy path that does not exist.
