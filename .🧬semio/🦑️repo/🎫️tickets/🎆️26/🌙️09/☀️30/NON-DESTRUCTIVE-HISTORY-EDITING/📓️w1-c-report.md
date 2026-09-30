# 📓️ W1-C Report: 🛠️tool-machine Module

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W1-C (design §5, plan "W1-C"). Status: **done, verified**.
Nothing was committed. No ticket or goal was opened or closed.

## 1. What Was Built

The new framework module `🧰️framework/🔨️modules/🛠️tool-machine`, crate `semio-framework-tool-machine`
(lib `semio_framework_tool_machine`). It is pure and target-neutral, and it compiles for native, `wasm32-wasip2` and
`wasm32-unknown-unknown`.

| File | Role |
|---|---|
| `🦀️.rs` | Rust owner: `ToolYield<M>`, `ToolYieldKind`, `ToolTransactionState`, `ToolTransactionClosed` (`toolTransaction.closed`), the `ToolTransaction<M>` reducer, the `ToolMachine` trait (blanket impl), `ToolStep<M>`, `ToolMachineRunner<T, H>` and `TOOL_MACHINE_ACTOR` |
| `🟦️.ts` | TS twin of the same surface, driving the `@semio-tech/machine` kernel. `TransactionRef` and `mintTransactionRef` are imported from `@semio-tech/framework-replication`. The TS twin has no mint of its own. |
| `🧬️schema/🔣️.json` | Schema of record. It covers the yields, the transaction, `TransactionRef`, the HLC, `ToolStep`, and the example gesture's input, events, mutations and chart. The root fixture def is `TransactionLawFixture`. |
| `🧫️fixtures/🧫️transaction-law/🔣️.json` | Language-agnostic corpus with these parts: 3×4 matrix, 11 reducer cases, 7 id vectors, the example gesture chart plus 9 scenarios, and 10 invariants |
| `🧪️tests/🔬️unit/🦀️.rs` | 10 Rust tests. They include the example `drag_gesture` `statechart!` (idle → pressed → dragging) and the `probe`/`eager` law machines. The third-party `blake3` crate (dev-dependency) pins the ids. |
| `🧪️tests/🧪️conformance/🟦️.ts` | 14 bun:test tests. The oracles are **ajv** (schema), a **JS `Map`** (keyed first-insertion order), **xstate** (the fixture chart as an xstate machine) and **fast-check** (500 random yield sequences for reducer vs Map, and 300 random gestures for runner vs xstate). |
| `📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts, package.json}` | Scaffolding copied from `⏯️tool-run`. The nx project is `@semio-tech/framework-tool-machine-rs` with targets `test`, `test-quick`, `test-long`, `test-exhaustive` and `check`. `test` runs bun conformance and then the cargo tests. `check` runs native plus `wasm32-wasip2`. |

Shared-file edits, each made with the Edit tool on a unique anchor:

- Root `Cargo.toml`: added the member line after `⏯️tool-run` and the `[workspace.dependencies]` entry `semio-framework-tool-machine`.
- `LIB/🔣️taxonomy.json` `members-of-modules`: added `"🛠️tool-machine"` after `"⏯️tool-run"`.

Dependencies:

- `machine` (the path dependency `semio-framework-machine`, renamed because `statechart!` expands to `machine::…`) and `semio-framework-replication` (`protocol`).
- Dev-dependencies: `blake3` (already in `Cargo.lock` through `hash` and `replication`) and `serde_json`.
- No new runtime dependency on an external library.

## 2. Contract as Implemented

- **Reducer** (`ToolTransaction::apply`). An upsert replaces an entry by key and keeps its first-insertion slot; a new key is appended. A retract removes its key; retracting an absent key does nothing, and a later upsert of that key appends it. A commit closes the transaction and keeps its entries. An abort closes it and clears the entries, so nothing remains. Any yield on a closed transaction returns `Err(ToolTransactionClosed)` and changes nothing. `into_parts()` returns `(TransactionRef, Vec<M>)`.
- **Runner** (`ToolMachineRunner::start(tool, actor, input, host)`, then `send(event, clock)` and `timer_elapsed(timer, clock)`):
  - A transaction opens at the first `Upsert` while none is open. Its id comes from `TransactionRef::mint(actor, clock of that event, tool)`, which W1-A owns.
  - `Retract`, `Commit` and `Abort` without an open transaction do nothing, so a click, or an escape while only pressed, leaves no trace at all.
  - One event closes at most one transaction. A yield after the close within the same event is refused, and the event publishes nothing (fail-closed); the runner stays usable. This rule rules out two transactions in one event minting the same id from the same clock.
  - A yield while the initial configuration is being entered is refused, because a transaction only opens in response to an event.
  - Commands that are not yields (timers and invokes) are routed to the `Host` through `machine::route_command`. `Send` targets are dropped, because a tool is a single actor.
  - `ToolStep` is one of `Idle`, `Open`, `Committed(ref, mutations)`, `Aborted(ref)` or `Empty(ref)`. A commit with no entries is `Empty`, meaning no edit.
- **`ToolMachine`** is defined as `trait ToolMachine: Machine<Effect = ToolYield<<Self as ToolMachine>::Mutation>>`, with a blanket impl. Every `statechart!` that declares `effect: ToolYield<M>;` is therefore a tool machine without extra code. I compiled the supertrait-cycle form on its own with rustc before relying on it.

## 3. Deviations and Decisions (Within §5)

1. `ToolTransaction` stores `reference: TransactionRef` instead of separate `id` and `tool` fields. It is the same data and avoids rebuilding the ref at commit. The fields are private so that only `apply` can change the state.
2. The Rust constructor is `start`, not `new`, to match the TS twin, where a constructor cannot return a result union.
3. `ToolStep::Open` carries no data. The host reads `runner.transaction()` for the ref and the provisional entries (the overlay), which avoids cloning two strings on every pointer move.
4. My own mint was removed at the coordinator's request. Rust uses `protocol::TransactionRef::mint` and TS uses `mintTransactionRef`. The fixture's `ids` section pins three things together: the Rust mint, the TS mint, and a reconstruction of the material with the third-party `blake3` crate. The ids themselves were generated independently with the first-party TS BLAKE3 plus a hand-written LEB128.
5. Tool state is ephemeral. It lives in the runner's `Snapshot` (the configuration and the context) and has no persistence API here; history can edit only the mutations and the ref.

## 4. Verification (All Run in the Foreground, Gated)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-tool-machine --tests` | clean. The first run raised a type-dependent `dead_code` warning (unused `Noop`), which proves the crate was really type-checked; I fixed it and the check is now 0 warnings. |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1-c cargo test -p semio-framework-tool-machine` | **10 passed, 0 failed** |
| `cargo check -p semio-framework-tool-machine --target wasm32-wasip2` | Finished, 0 warnings |
| `cargo check -p semio-framework-tool-machine --target wasm32-unknown-unknown` | Finished, 0 warnings |
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | **14 pass, 0 fail** (~9.5k expects) |
| package router `bun ./📜️script.ts test` (in the package dir, private target) | bun 14/14, then nextest **10/10 passed** |
| package router `bun ./📜️script.ts check` | native plus wasip2 both finished |
| Negative control: a TS twin mutated so that upsert appends instead of replacing, run through a copy of the conformance test | 4 tests failed as expected: the fixture cases, the Map fast-check, the scenarios and the xstate fast-check |
| `tsc` on the twin and the test (temporary tsconfig, `include: []`) | 0 errors in tool-machine files. The only error is in a peer's `🛂️manifest/🟦️.ts:117` (`DialogChoice` not yet exported, W1-E in flight). |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/🛠️tool-machine` | **clean=true errors=0 warnings=0** |
| `bun ./📜️script.ts verify dependencies literal-external` | red across the repo, as before (`literal-external=258`, 2 oracle conflicts on `image` and `serde_json` elsewhere). No `tool-machine` or `blake3` finding. |

Logs are in `🗑️generated/w1-c/` (safe to delete at close).

## 5. Ticket Inputs Kept

- `🧪️w1-c-transaction-ids.ts`: the independent id generator (TS first-party BLAKE3 plus LEB128).
- `🧪️w1-c-transaction-law-fixture.py`: writes the hand-authored fixture expectations. It reproduces the committed fixture byte for byte (checked with `cmp`).

## 6. Notes for the Coordinator and Downstream WPs

- **Launch rows**: none were added. They are generated centrally.
- **Reusable-module rule**: the taxonomy requires two production consumers. Today there are none. The planned ones are W2-D (the puzzle 2d select tool) and W2-A (`Emit.transaction` stamping). Draw's vendored `🔄️fsm` gesture machine is a natural third candidate.
- **W2-A integration**:
  - Per event, call `runner.send(event, hlc)`. On `Committed(ref, mutations)`, publish exactly one edit with every op's `MutationMeta.transaction = Some(ref)`.
  - On `Open`, render `runner.transaction().entries()` as the overlay.
  - On `Aborted` or `Empty`, drop the overlay.
  - Keep the runner in window-transient state.
- **Tool author contract**: yield at most one closing `Commit` or `Abort` per event, and never yield from entry actions of the initial state.

## 7. Follow-Up (Audit M-1, M-2 and the W2-D Persistence API)

The audit this follows is `📓️audit-modules-ui.md`. Status: **done, verified**.

### 7.1 M-1: Host Abort and Reset

- **`abort(reason: ToolAbortReason) -> ToolStep<M>`** (TS: `abort(reason)`).
  - It drops the open transaction with zero trace.
  - It cancels, on the host, the timers and invokes of the configuration it leaves.
  - It re-enters the initial configuration from the `start` input.
  - It returns `Aborted(ref, reason)`, or `Idle` when no transaction was open.
- **`reset() -> Option<TransactionRef>`** does the same silently and returns the ref it dropped, if any.
- **Aborts now carry a reason.** `ToolStep::Aborted(TransactionRef, ToolAbortReason)`:
  - A tool's own `Abort` yield reports `Tool`.
  - The reasons are `tool`, `blur`, `captureLost`, `baseMoved`, `frozen` and `retired` (Rust enum, TS const, schema enum).
- **Input must be cloneable.** The runner keeps the start input to re-initialize the statechart, so the runner's impl requires `T::Input: Clone`.

### 7.2 M-2: The Rest Law

The rest law is enforced as a refusal, which I picked over an implicit abort as the safer choice:

- Both options leave zero trace, but a refusal reports the tool bug instead of hiding it.
- "At rest" means the root's initial state is active (`at_rest()` / `atRest()`).
- An event that leaves the tool at rest with its transaction still open returns `Err(ToolRefusal::Unclosed)` (`toolTransaction.unclosed`) and drops the transaction. The next gesture therefore opens a new transaction and can never join the old one.
- The earlier refusal type `ToolTransactionClosed` is replaced by `ToolRefusal { Closed, Unclosed }`. Both codes are pinned in the fixture and the schema.
- Fixture coverage: the chart variant `leaky_gesture` (release without commit) with 2 scenarios, one of which is the audit's leak reproduction.

### 7.3 Persistence API for W2-D (Coordinator Request)

1. `ToolTransaction::resume(reference, entries) -> Self`, TS `ToolTransaction.resume`. The result is Open. Entries are upserted in order, so first-insertion order is kept, and a repeated key keeps its first slot with its last mutation.
2. `ToolMachineRunner::resume(tool, actor, input, snapshot, transaction, host) -> Result<Self, ToolRefusal>`, TS `ToolMachineRunner.resume(machine, …)`. It refuses in two cases:
   - a committed or aborted transaction, with **`Closed`**;
   - a resting snapshot that holds an open transaction, with **`Unclosed`**.
   - The brief read "`Unclosed`" for both; I kept `Closed` for a closed transaction because it matches the code's meaning. Callers that only check `is_err()` are unaffected.
   - Timers scheduled before the persist stay with the host that scheduled them.
3. `ToolMachineRunner::into_parts(self) -> (Snapshot<T>, Option<ToolTransaction<M>>)`, TS `intoParts()`.

Fixture coverage:

- A `resume` admission table with 10 rows (state × none/open/committed/aborted).
- A `{host: "resume"}` scenario row kind, used by the scenario "a gesture persisted and resumed between events is still one transaction".
- Both languages also replay every scenario with a persist and resume after **every** row, and must get the same outcomes as the uninterrupted replay.

### 7.4 Fixture, Schema and Twins

- **Fixture** (still reproduced byte for byte by `🧪️w1-c-transaction-law-fixture.py` plus `🧪️w1-c-transaction-ids.ts`):
  - 8 id vectors (one added for clock 1007);
  - `refusals` and `abortReasons` lists;
  - `gesture.resume` (10 rows);
  - `gesture.variants`: `drag_gesture` with 13 scenarios, which adds host abort, idle abort, reset and resume, and `leaky_gesture` with 2;
  - 13 invariants (new: `restClosesTransaction`, `hostCancelLeavesZeroTrace`, `resumeContinuesTheGesture`).
- **Scenario rows** now come in five kinds: event→step, event→refusal, host abort, host reset and host resume. The schema is updated accordingly.
- **xstate oracle** is extended with the rest law (the chart's initial state), host abort and reset (re-entering the initial snapshot), and resume as a no-op.
- **fast-check** covers both chart variants. It mixes in host abort, reset and resume, and an arbitrary biased around the magnet so that empty commits are hit (audit N-13b).
- **Audit emoji findings**: the audit lists none for tool-machine (N-5 is time-travel, N-24 is the UI contract). I re-checked all four of my files after the additions and found no duplicate docstring emoji.

### 7.5 Consumer Touched

`✏️s/🔌️plugins/🧩️puzzle/…/🪛️utilities/🖱️select/🦀️.rs:176` changed the pattern `ToolStep::Aborted(_)` to `ToolStep::Aborted(..)`, one token, because the variant gained its reason.

- `cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly` finished with no warnings in the select tool or in tool-machine. The 11 warnings are in peers' files.
- The select tool obeys the rest law: its single `idle` state upserts and commits within one event.

### 7.6 Verification (Foreground, Gated)

| Command | Result |
|---|---|
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1-c cargo test -p semio-framework-tool-machine` | **13 passed, 0 failed** |
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | **18 pass, 0 fail** (~11.4k expects) |
| package router `bun ./📜️script.ts test` | bun 18/18, nextest **13/13** |
| `cargo check -p semio-framework-tool-machine` on native, `--target wasm32-wasip2` and `--target wasm32-unknown-unknown` | all finished, 0 warnings (`--tests` also 0 warnings) |
| `tsc` (twin and conformance) | exit 0 |
| TS negative controls: no rest law; abort without re-init; resume admitting unclosed; abort keeping timers | each mutation fails 1–3 tests |
| `verify taxonomy report --scope 🧰️framework/🔨️modules/🛠️tool-machine` | clean=true errors=0 warnings=0 |

### 7.7 Still Open From the Audit (Not in This Brief)

- **N-10**: a clock is still required on every event.
- **N-11**: there is no `InertHost`.
- **N-12**: the schema's `minLength` constraints are not checked in code.
- **N-13**: TS `entries()` still returns the live array.
- **N-13b**: the probe charts are not yet in the fixture.
