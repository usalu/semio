# 📓️ terra-R1-async-iface report — new PURE crate `semio-framework-async`

## ⚠️ Packet-id collision, read first

This packet's brief calls itself "**R1**" and names the report path `📓️terra-R1-report.md`. That
exact packet id was already used and FINALIZED earlier in this same ticket for a completely
different scope: "R1 native manifest" (committed-descriptor wiring into `🏃️run/🦀️component.rs`,
accepted in `📓️status.md` at "✅ R1 finalized — and a systematic naming hazard now has four
instances"). That report file already holds real, coordinator-verified content for that other
packet.

I did **not** overwrite `📓️terra-R1-report.md` — doing so would have destroyed the finalized
native-manifest packet's only record. This report lives at `📓️terra-R1-async-iface-report.md`
instead (named after my executor id). **Sol: please reconcile the id collision** (this new crate
packet needs a fresh id — R5/R6/whatever is next — rather than reusing R1) and treat this filename
as the durable record for the async-interface work until you do.

## Delivered

New pure crate `semio-framework-async` at `🧰️framework/🔨️modules/⏳️async/`, mirroring `🎭️actor`'s
layout exactly:

```
⏳️async/
  🦀️component.rs                                  # 781 lines, all regions
  🟦️component.ts                                  # ts-rs re-export + hand-written WebAsyncScope seam
  📦️packages/🦀️rust/{Cargo.toml, 📦️glue.rs, 📜️script.ts, 📋️project.json}
  📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts, 📦️index.ts}
```

Regions in `🦀️component.rs` (one `//#region` per type, per-item docstrings each starting with a
unique emoji, no comments inside definitions):

- **`🪪️OperationContext`** — `{ actor: u64, generation: u16, trace: TraceId, lane: u8,
  deadline_ms: Option<u64>, cancel: CancelToken, capability: Option<CapabilityTokenId> }`, exactly
  as specified. `TraceId(u64)` and `CapabilityTokenId(u64)` are separate serde+ts-rs newtypes.
  **Deliberately NOT `Serialize`/`Deserialize`**: it embeds a live `CancelToken` (an `Arc`), so it is
  an in-process value passed by reference within one host, never wire data — documented inline, same
  reasoning `🎭️actor`'s own seam docstrings use elsewhere in this ticket.
- **`🛑️CancelToken`** — `Arc<CancelNode>` where `CancelNode { local: AtomicU8, parent:
  Option<CancelToken> }`. `CancelState` is `Live < Park < Cancelled` (`PartialOrd`/`Ord` derived, so
  severity comparison is a plain `max`). `state()` is a max-severity fold of `local` with the
  parent's own `state()` — so `child()`-derived tokens observe ancestor cancellation transitively
  with no child registry to walk, and a child's own `park`/`cancel` never propagates upward. `cancel`
  is terminal: `park`/`unpark` become no-ops once cancelled (tested explicitly).
- **`🌳️Scope`** — `ScopeOwner { Actor(u64), Package(String), Service(&'static str) }`,
  `ScopeId(u64)`, `ScopeHandle { id, owner, cancel }`, `ScopeDrainReport { finished, cancelled,
  leaked: u32 }`. `ScopeOwner` intentionally skips `Deserialize`/ts-rs (see "Judgment calls" below).
  `ScopeHandle` skips `Serialize` for the same live-handle reason as `OperationContext`.
- **`🚰️ChannelPolicy`** — `LatestWins | Coalesced{key} | LosslessBounded{cap} | ByteCredit{bytes}`,
  full serde + ts-rs, exactly as specified.
- **`🧵️ThreadPlan`** — pure arithmetic `thread_plan(cores) -> ThreadPlan { kernel:1, shards:
  clamp(ceil(N/2),2,8), tokio_workers: clamp(ceil(N/4),1,4), compute: max(1, N-shards-tokio_workers-1),
  epoch_ticker:1 }`, plus `ThreadRole` and `ThreadBudget::{from_plan, checkout, remaining}` with
  `debug_assert!` on over-draw (a release build lets the atomic counter wrap instead of panicking —
  matches "debug-panics on over-draw" literally). See "Genuine finding" below for the invariant's
  actual valid domain.
- **`🎛️HostAsyncRuntime`** — `HostFuture<T> = Pin<Box<dyn Future<Output=T> + Send + 'static>>` and
  the trait with the exact six methods from the brief (`open_scope`, `spawn_scoped`, `run_blocking`,
  `sleep_until`, `cancel_scope`, `now_ms`), including `spawn_scoped`'s mandatory `&ScopeHandle` (no
  detached-spawn entry point exists on this trait).
- **`🧪️testkit::ManualRuntime`** — the in-crate `HostAsyncRuntime` test double, feature-gated
  `testkit` (also compiled under plain `cfg(test)` for this crate's own suite). A real manual poll
  loop (`drive()`) using `std::task::Waker::noop()` (stable since Rust 1.85, well under this crate's
  declared `rust-version = "1.88"` — confirmed by compiling, not assumed) — no `futures` crate, no
  hand-rolled `RawWaker` vtable needed. `sleep_until` only resolves once the caller advances the
  injected clock via `set_now_ms`; the runtime never reads a real clock anywhere.

Every non-handle data type carries `Serialize`/`Deserialize` + `#[cfg_attr(feature = "typegen",
derive(ts_rs::TS))]`: `TraceId`, `CapabilityTokenId`, `CancelState`, `ScopeId`, `ScopeDrainReport`,
`ChannelPolicy`, `ThreadPlan`, `ThreadRole`. No `pack_encode`/`pack_decode` anywhere — unlike
`🎭️actor`, this crate is not a wire codec; it is an in-process interface/vocabulary layer, so I did
not add a `pack` module (nothing in the brief asked for one, and no method on `HostAsyncRuntime`
takes or returns bytes).

`🟦️component.ts` re-exports the (not-yet-generated) ts-rs mirror plus a hand-written
`WebAsyncScope` interface — the documented, NOT-implemented future web-host seam, with an
English-then-German docstring as CLAUDE.md requires. It intentionally omits
`spawnScoped`/`runBlocking`/`sleepUntil`/`nowMs` from the seam (a web host would drive those through
its own event loop rather than exposing discrete calls) and hand-mirrors `ScopeOwner`/
`ScopeDrainReport`'s shapes locally as `WebAsyncScopeOwner`/`WebAsyncScopeDrainReport` rather than
promoting them into the generated mirror, since the real ones are intentionally excluded from ts-rs.

## Commands + exit codes (verbatim, run this session, foreground)

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo check -p semio-framework-async --all-targets
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
EXIT_CODE=0

$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo test -p semio-framework-async
running 16 tests
test component::tests::cancel_token_cancel_is_terminal_over_park ... ok
test component::tests::cancel_token_park_then_unpark_returns_to_live ... ok
test component::tests::parking_parent_does_not_downgrade_an_already_live_reading_below_park ... ok
test component::tests::cancel_token_root_starts_live ... ok
test component::tests::cancelling_parent_transitively_cancels_child_and_grandchild ... ok
test component::tests::child_cancel_never_propagates_upward_to_parent ... ok
test component::tests::manual_runtime_spawn_scoped_runs_a_ready_future_on_drive ... ok
test component::tests::manual_runtime_cancel_scope_reports_finished_and_cancelled ... ok
test component::tests::manual_runtime_sleep_until_resolves_only_after_injected_time_advances ... ok
test component::tests::thread_budget_checkout_debits_and_returns_remaining ... ok
test component::tests::scope_handle_child_scope_shares_cancellation_lineage ... ok
test component::tests::thread_plan_invariant_holds_once_floors_stop_binding ... ok
test component::tests::thread_budget_checkout_debug_panics_on_overdraw - should panic ... ok
test component::tests::thread_plan_is_deterministic ... ok
test component::tests::thread_plan_low_core_counts_oversubscribe_but_never_zero_a_role ... ok
test component::tests::thread_plan_shards_and_tokio_workers_never_exceed_their_ceilings ... ok
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests semio_framework_async
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
EXIT_CODE=0

$ CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo check -p semio-framework-async --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
EXIT_CODE=0
```

Also run separately and passing (not in the mandated list, but the one thing `📜️script.ts typegen`
would gate on): `cargo test -p semio-framework-async --features typegen exports_typescript_bindings`
→ `test component::tests::exports_typescript_bindings ... ok`, exit 0.

Also ran, not mandated but worth doing given this ticket's Z1 zero-warnings gate: `cargo clippy
-p semio-framework-async --all-targets --all-features -- -D warnings`. First run failed
(`manually reimplementing div_ceil` in my own `ceil_div_usize` helper) — fixed to
`numerator.div_ceil(denominator)` (stable since Rust 1.73, well under this crate's declared
`rust-version = "1.88"`); second run exit 0, zero warnings. All three mandated commands re-run
clean after that fix (line numbers in the purity grep below are post-fix).

## Purity evidence

```
$ grep -nE 'tokio|wasm_bindgen|web_sys|winit|rayon|std::thread|SystemTime|Instant::now|std::fs|std::net' 🧰️framework/🔨️modules/⏳️async/🦀️component.rs
1:  //! ... no `tokio`, no `std::thread`, no I/O,               ← doc comment
4:  //! [`HostAsyncRuntime`] ... never `tokio` directly ...      ← doc comment
5:  //! concrete executor (tokio today, ...) ...                ← doc comment
8:  //! 🪡 **Where tokio actually lives**: nowhere in this crate. The concrete `tokio`-backed  ← doc comment
12: //! downstream crates can unit-test ... without ever linking tokio.  ← doc comment
256:    pub tokio_workers: u32,                                   ← field name, see below
271-272: /// ... `kernel`/`tokio_workers`/`epoch_ticker` are park-dominated ...  ← doc comment
282-285: let tokio_workers = ...; ThreadPlan { ..., tokio_workers, ... }        ← field name, see below
306,316,326: tokio_workers: AtomicU32, ... ThreadRole::TokioWorker => &self.tokio_workers  ← field/variant name
351: /// Hides the concrete async executor (tokio in packet R2; ...)           ← doc comment
367: /// ... a real thread pool in the tokio implementation ...                ← doc comment
389: /// ... unit-test against the trait without linking tokio. ...            ← doc comment
650,655,659: assert!(... plan.tokio_workers ...); fn ..._tokio_workers_...()   ← field name / test name
GREP_EXIT_CODE=0
```

**Flagging honestly rather than hiding it**: the grep's literal exit-0/all-comments bar is not
fully met — `tokio_workers`/`ThreadRole::TokioWorker` are real code (field/variant names, not doc
comments), because the packet brief itself dictates that exact field name in `ThreadPlan`'s literal
shape (`tokio_workers: clamp(ceil(N/4), 1, 4)`). This is an accounting *label* — "how many threads
this plan reserves for whatever tokio runtime a LATER crate runs" — not a use of the `tokio` crate:
`Cargo.toml` has no `tokio` dependency, there is no `use tokio`, no `tokio::` qualified path, and
both `cargo check --target wasm32-unknown-unknown` above and this whole crate's dependency graph
(`serde`, optional `ts-rs`) confirm it. I chose to keep the coordinator-specified field name rather
than rename around the grep, since a sibling packet (R2) will almost certainly read `plan
.tokio_workers` by that exact name. If the grep's literal zero-code-matches bar must hold exactly,
the fix is a one-line rename (`tokio_workers` → e.g. `async_workers`) — flagged here rather than
done silently, since it changes a name the brief gave verbatim.

## Genuine finding: `thread_plan`'s invariant only holds for `cores >= 4`

The brief says to test `shards + compute + 1 <= cores` for `cores in 1..=64`. Simulated by hand
before writing the implementation, then confirmed by the passing test suite: `shards` has a hard
floor of 2 and `tokio_workers` a hard floor of 1, so at `cores` 1–3 the reserved budget
(`shards + tokio_workers + 1`) already exceeds `cores` before `compute`'s own floor of 1 is even
applied — e.g. at `cores=1`: `shards=2, tokio_workers=1, compute=max(1, 1-2-1-1)=1`, so
`shards+compute+1 = 4 > 1`. The invariant holds exactly from `cores=4` onward (equality at exactly
4: `shards=2, tokio_workers=1, compute=1` → `2+1+1=4`), and I verified it for every `cores` in
`4..=64` (`thread_plan_invariant_holds_once_floors_stop_binding`). Rather than silently narrowing
the test range without comment, or forcing a fabricated pass at 1–3, I split it into two tests: the
invariant proper over `4..=64`, and a separate `thread_plan_low_core_counts_oversubscribe_but
_never_zero_a_role` over `1..3` asserting the honest behavior there (deliberate oversubscription,
never a zeroed role). Documented in `thread_plan`'s own doc comment. This reads as intentional
design (the floors exist so even a one-core host gets a live actor system and a live async
runtime, and the OS scheduler already handles oversubscription safely) rather than a bug, but it is
sol's call whether `cores < 4` deserves different handling before R2 builds on it.

## Judgment calls

1. **`ScopeOwner` and `ScopeHandle` skip `Serialize`/`Deserialize`/ts-rs entirely** (not just
   `Deserialize`). `ScopeOwner::Service(&'static str)` cannot derive `Deserialize` — there is no
   `serde::Deserialize` impl for `&'static str` (nothing can borrow past the deserializer's input
   lifetime to manufacture a `'static` reference) — and `ScopeHandle` carries a live `CancelToken`
   for the same reason `OperationContext` does. Rather than split `ScopeOwner` into a serializable
   subset and a doc comment explaining the omission, I kept the type exactly as specified and
   documented the reasoning inline. `WebAsyncScopeOwner`/`WebAsyncScopeDrainReport` in
   `🟦️component.ts` are hand-written local mirrors for the seam instead.
2. **No `pack` module.** `🎭️actor`'s hand-rolled binary codec exists because that crate's types
   cross a real wire (wasm ↔ host). Nothing in this brief crosses a wire yet — `HostAsyncRuntime`'s
   methods take/return live Rust values, not bytes — so I did not invent a codec nobody calls.
3. **`📦️packages/🟦️typescript/📦️index.ts`** — one file beyond the brief's literal 3-file listing
   for the TS package. `package.json`'s `exports` map needs *some* real target; every sibling
   TS package in this repo (`🎭️actor`, `◻2d`, `🖼️assets`, `🧊️3d`) uses a `📦️index.ts`/similar entry
   point for exactly this, so I added the minimal one-line re-export rather than pointing `exports`
   at a file two directories away, which would be unconventional here even if technically valid.
4. **No `vitest.config.ts`, no `test` nx target for the TS package.** There is genuinely nothing to
   test yet: `🟦️component.ts` is hand-authored documentation + a not-yet-generated ts-rs import
   (mirrors `🎭️actor`'s own history — its `component.ts` also imported a nonexistent generated file
   before typegen ever ran, and is excluded from that crate's `vitest.config.ts` `include` list for
   exactly this reason), and `WebAsyncScope` has no implementation to exercise. I mirrored the
   `🖼️assets` TS package's precedent instead (no vitest config, no fabricated test) — added a single
   honest `info` nx target instead of a placeholder `test` target that would test nothing.
5. **`ManualRuntime::cancel_scope` matches every open scope, not just `owner`'s.** `ScopeOwner` has
   no stable id to index by, and `ManualScopeRecord` (deliberately minimal) does not store the
   owner. Documented in-line as an acknowledged simplification of the test double — a real
   `HostAsyncRuntime` implementation (packet R2) is expected to index scopes by owner directly. Not
   a correctness issue for anything in this crate's own test suite (each test only ever opens one
   scope at a time), but worth R2 knowing about before assuming `ManualRuntime`'s behavior generalizes.

## Lease-requests

**Mandatory — root `Cargo.toml` workspace membership** (per this packet's own instructions and the
established `F1-scale-fixture` pattern, `📓️status.md` "Leases applied by me" under F1):
1. Add member path `"🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust"` to the root `Cargo.toml`
   `[workspace] members` list.
2. Add a `[workspace.dependencies]` alias: `semio-framework-async = { path =
   "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust" }`.
3. Delete this crate's own `[workspace]` opt-out table in
   `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml`, and add `[lints] workspace = true`
   in its place (matching every other member).
4. Delete `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.lock` (generated only because the
   crate is standalone right now; workspace members do not carry their own lock file).

**Non-mandatory, flagged for awareness:**
5. See "Purity evidence" above — if the grep's literal zero-code-matches bar must hold exactly
   rather than "zero real `tokio`-crate usage", `tokio_workers`/`ThreadRole::TokioWorker` need a
   rename (e.g. `async_workers`/`AsyncWorker`). I left the coordinator-specified name in place and
   flagged it instead of unilaterally renaming a name the brief gave verbatim.
6. The packet id collision itself (see the top of this report) — this packet needs a real id
   distinct from the already-finalized "R1 native manifest".

## Same-name-different-type collision check

Checked against the four already-recorded collisions (`ActorId`, `exchange`, `IoEntryDescriptor`,
`Budget`) and the codebase generally: **no new collision found**. This crate names `ThreadBudget`
(not bare `Budget`), and `OperationContext.actor` is a bare `u64` rather than a new `ActorId` type,
specifically to avoid adding a fifth instance of either existing hazard. `CapabilityTokenId` and
`ScopeId`/`ScopeOwner`/`ScopeHandle`/`ScopeDrainReport`/`ChannelPolicy`/`ThreadPlan`/`ThreadRole`/
`CancelToken`/`CancelState`/`TraceId`/`OperationContext`/`HostAsyncRuntime`/`HostFuture`/
`ManualRuntime` — none of these names exist anywhere else in the tree (checked by reading, not
grep, since `CLAUDE.md`/the packet brief forbid the search tool; spot-checked the obvious
candidates `Scope`, `Budget`, `Context` by inspection of the sibling `🎭️actor` and `🎠️kernel` crates
already read in full for this packet).

## Honest gaps

- **TS side not built or typechecked.** I did not run `bun install`/`bun ./📜️script.ts` for either
  TS package, and did not run the `typegen` nx target (only the raw `cargo test --features typegen
  exports_typescript_bindings` it depends on, which passes). `🤖️generated/🟦️async.ts` does not
  exist yet — same state `🎭️actor` was in before its own first typegen run. Not part of this
  packet's mandated acceptance list, but worth running before anything actually imports
  `🟦️component.ts`.
- **`ManualRuntime`'s `cancel_scope` owner-matching is a known simplification** — see "Judgment
  calls" #5.
- **No wasm32-wasip2 check was run** (not requested by this packet — R2's job once a concrete
  runtime exists to actually compile a component against).
- **`ScopeHandle`/`OperationContext`'s exclusion from ts-rs means `🟦️component.ts` cannot describe
  them at all** for a future web host beyond the hand-written `WebAsyncScope` seam's own local
  types. That is a design consequence of the "live handle, not wire data" call in "Judgment calls"
  #1, not an oversight — but it means anyone implementing `WebAsyncScope` for real will need to
  invent its own wire shapes for whatever it actually sends over `postMessage`, rather than reusing
  these Rust types' automatic mirror.

## Files touched (all new; nothing pre-existing was edited)

- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
- `🧰️framework/🔨️modules/⏳️async/🟦️component.ts`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.lock` (generated; registrar should delete per lease-request #4)
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📦️index.ts`

No file outside `🧰️framework/🔨️modules/⏳️async/**` was touched. No git command of any kind was run
beyond read-only `git status`/`git diff --cached`/`git show :<path>` (used once, to confirm an
`AM` status on my own in-progress file was this ticket's known auto-commit bot staging my
work-in-progress, not a peer collision — confirmed: the staged blob matched an earlier, mid-edit
version of my own file, not another session's content).
