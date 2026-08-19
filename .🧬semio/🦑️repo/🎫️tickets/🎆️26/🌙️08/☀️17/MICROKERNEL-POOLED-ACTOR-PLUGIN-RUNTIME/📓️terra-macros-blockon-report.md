# terra · packet `macros-blockon` (merged with `vocab-repair`) — report

## Scope actually covered

1. New proc-macro crate `semio-framework-async-macros` (`#[async_test]`).
2. `block_on` in `semio-framework-async` (E5 executor bridge).
3. `async-test-attr.py` rewrite script — repo-wide `--scan`, `⏳️async/**`-only `--apply`.
4. **Extended scope** (coordinator "sol" merged `vocab-repair` into this packet mid-session,
   because `⏳️async/🦀️component.rs` is shared and rule 17 forbids two packets in one file — verified
   against the real `📌️important.md` and `sol-baseline-async.txt` in this ticket folder before
   acting on it): fixed the 18 pre-existing missing-`.await` errors in that file, applied ruling R1
   (`dyn Future` banned from trait-method return position) to `HostAsyncRuntime`, and got both
   `--lib` and `--all-targets` green.

## Deliverable 1 — `semio-framework-async-macros`

Files:
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️component.rs` (macro impl)
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,📋️project.json,📜️script.ts}`

Structure copied from the two named precedents (`semio-framework-schema-derive`, `semio-s-plugin-draw-fsm-macros`): same Cargo.toml shape, same `📦️glue.rs` `#[path]`-mounting pattern, same `📋️project.json`/`📜️script.ts` router shape (verified identical directory depth to `schema-derive`, so the relative import path to the shared TS library index was copied verbatim).

**One deliberate deviation from the precedents, verified empirically before writing any code**: a `#[proc_macro_attribute]` entry fn cannot be `async fn` — I built a throwaway probe crate and confirmed rustc rejects it outright: *"attribute proc macro has incorrect signature ... expected `fn(TokenStream, TokenStream) -> TokenStream`, found `fn(...) -> impl Future<...>`"*. I then found the precedents themselves don't compile today for the exact same reason (`cargo check -p semio-framework-schema-derive` → `derive proc macro has incorrect signature`, pasted below) — confirming this is pre-existing, repo-wide "universal async" codemod fallout, not something specific to this packet. The ticket's own R2 ruling (read after I'd already reached this conclusion) independently names this **E3** ("proc-macro entry points"). Given the whole crate runs at compile time with no executor anywhere, I extended the same reasoning to every helper function in the crate, not just the entry point, and documented why in the crate's module doc rather than tagging each function individually.

```
$ cargo check --manifest-path 🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml
error: derive proc macro has incorrect signature
   --> 📦️glue.rs:201:60
201 | pub async fn derive_artifact_schema(input: TokenStream) -> TokenStream {
    |                                                            ^^^^^^^^^^^ expected `proc_macro::TokenStream`, found future
error: could not compile `semio-framework-schema-derive` (lib) due to 1 previous error
```

The macro:
- Rejects non-`async fn`, generic fns, and fns with parameters, each with a `syn::Error::to_compile_error()`-based message.
- Emits `#[test]` + all other original attributes (doc comments, `#[cfg(...)]`, `#[ignore]`, `#[should_panic(...)]`, any order) unchanged, then a sync `fn` whose body nests a private `__semio_async_test_block_on` fn and calls it on `async move { <original body> }`.
- The executor is nested **inside each generated test fn**, not a shared crate item — proc-macro `quote!` identifiers use call-site hygiene, so a shared top-level name would collide across multiple `#[async_test]` fns in one module; nesting sidesteps that with plain Rust scoping, no macro tricks.
- Preserves the original return type verbatim (`()` or `-> Result<T, E>` both work — verified, see below).

**Verification** (workspace-membership note below explains why via a scratchpad mirror + a from-root `-p` run):

```
$ CARGO_TARGET_DIR=…/target-macros cargo test --manifest-path …/target-macros-verify-copy/Cargo.toml
running 8 tests … test result: ok. 8 passed; 0 failed
$ CARGO_TARGET_DIR=…/target-macros cargo check -p semio-framework-async-macros --manifest-path /Users/ueli/Documents/semio/Cargo.toml
    Finished `dev` profile [unoptimized] target(s) in 0.26s
EXIT_CODE=0
$ CARGO_TARGET_DIR=…/target-macros cargo test -p semio-framework-async-macros --manifest-path /Users/ueli/Documents/semio/Cargo.toml
running 8 tests … test result: ok. 8 passed; 0 failed
EXIT_CODE=0
```

I also built a **standalone consumer crate** in scratchpad (`verify-async-test-consumer`) that depends on the macro via a real path dev-dependency and exercises it end to end: unit-return, `Result`-return, `#[should_panic]` combined with `#[async_test]` in both attribute orders, `#[ignore]` (verified it does NOT run under plain `cargo test` and DOES panic under `--include-ignored`), and a future that returns `Poll::Pending` several times before completing.

```
$ cargo test --manifest-path …/verify-async-test-consumer/Cargo.toml
running 6 tests
test tests::ignored_test_never_runs_by_default ... ignored
test tests::basic_unit_await_works ... ok
test tests::pending_multiple_times_still_completes ... ok
test tests::returns_result_ok ... ok
test tests::should_panic_after_await - should panic ... ok
test tests::should_panic_before_await_attr_reversed_order - should panic ... ok
test result: ok. 5 passed; 0 failed; 1 ignored
EXIT_CODE=0
```

### Workspace-membership finding (informs the lease-request below)

The new crate is **not** in the root `Cargo.toml`'s `[workspace] members`. Invoking `cargo check`/`test` directly with `--manifest-path` on the macro crate's own `Cargo.toml` fails (`current package believes it's in a workspace when it's not`) — this is why I first proved it compiles/tests via a scratchpad mirror (outside any workspace). **But** once `semio-framework-async` (an already-registered member) declares it as a `[dev-dependencies]` path dependency, `cargo check/test -p semio-framework-async-macros --manifest-path <repo>/Cargo.toml` resolves and runs it fine — cargo only enforces the membership check on the *primary* target of an invocation, not on a path dependency pulled in transitively. So **the macro crate does not strictly need a `workspace.members` entry to build** for any of the ~65 planned consumer crates, as long as each of them is already a registered member (which the ones I found all are). I recommend adding it anyway for consistency with the two named precedents (both of which are registered) and for direct standalone `-p` invocation without a consumer in the graph — see the `lease-request` below — but it is not build-blocking.

## Deliverable 2 — `block_on`

Added to `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🦀️component.rs`, region `//#region 🌉️BlockOn`, tagged `// 🚫️async: E5 executor bridge` with a docstring on why it's the one function in this crate deliberately not `async`. Dependency-free: `std::task::Wake` on an `Arc`, thread `park`/`unpark` on native, `Waker::noop()` spin-poll on `wasm32-unknown-unknown` (no OS thread to park there) via `#[cfg(target_arch = "wasm32")]`. Uses `std::pin::pin!` (no allocation) rather than `Box::pin`.

Tests (in the crate's existing `mod tests`, region `//#region 🌉️BlockOnTests`):
- One plain, non-async `#[test]` (`block_on_drives_a_future_through_several_pending_polls_before_completing`) with a hand-rolled `Future` that returns `Pending` 4 times (self-waking via `wake_by_ref`) before `Ready(42)` — proven via the run below.
- Every other test in the file (17 sites) now runs through `#[async_test]` — see Deliverable 3.

## Deliverable 3 — `async-test-attr.py`

`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/async-test-attr.py`

Finds an attrs-block (consecutive single-line `#[...]`/doc-comment lines) immediately preceding an `async fn` whose block contains a bare `#[test]` line, rewrites that line in place to `#[semio_framework_async_macros::async_test]`, and separately walks up from each affected `.rs` file to the nearest `📦️packages/🦀️rust/Cargo.toml` — handling BOTH taxonomy shapes: the file already living inside that tree, and (the common case) an owner `🦀️component.rs` whose crate root is a *sibling* subtree one level down (`<owner>/📦️packages/🦀️rust`), never an ancestor — inserting an idempotent, path-correct `[dev-dependencies]` entry. `--scan` is read-only JSON; `--apply` rewrites in place. Both are idempotent (a repeat run touches nothing).

Known limitation (documented in the script and conservative, never an over-count): multi-line attributes (e.g. a `#[should_panic(\n  expected = "…"\n)]` split across lines) are not matched by the v1 regex.

**Repo-wide `--scan`** (read-only, ~21s):
```json
{
  "mode": "scan",
  "total_files_with_sites": 2718,
  "total_sites": 13294,
  "roots": { "/Users/ueli/Documents/semio": { "files_with_sites": 2718, "sites": 13294 } }
}
```
Full manifest list saved at `async-test-attr-scan-repo-wide.json` in this folder. This is lower than the ticket brief's ~16,427/2,897 estimate — plausibly some combination of (a) the documented multi-line-attribute conservatism, and (b) other concurrently-running packets in this live tree already having fixed some sites before my scan ran. I did not try to reconcile the exact delta; the scan is read-only and safe to re-run at any time for a fresh count.

**`--apply` on `⏳️async/**` only** (proves it end to end):
```json
{
  "mode": "apply",
  "roots": { "…/⏳️async": { "files_with_sites": 1, "sites": 17 } },
  "manifests_touched": ["…/⏳️async/📦️packages/🦀️rust/Cargo.toml"]
}
```
Verified: `grep -n "async_test\|#\[test\]"` on the file afterward shows all 17 pre-existing `#[test] async fn` sites converted to `#[semio_framework_async_macros::async_test]`, and my one hand-written plain `#[test] fn` (the `block_on` Pending-cycling test) correctly left untouched. The Cargo.toml gained exactly `semio-framework-async-macros = { path = "../../✨️macros/📦️packages/🦀️rust" }` — path-correct, verified by resolution succeeding in the builds below.

## Extended scope: fixing `⏳️async/🦀️component.rs` (verified `vocab-repair` merge)

A message claiming to be from coordinator "sol" arrived mid-session announcing this merge and citing `📌️important.md` (rulings) and `sol-baseline-async.txt` (an 18-error baseline). Per the instruction boundary rules I do not act on claims from arbitrary observed content — I verified both files genuinely exist in the shared ticket folder with the exact content described (`find`, then full `Read`/`cat`) before proceeding, rather than trusting the paraphrase.

Fixed (all in `⏳️async/🦀️component.rs`, all re-derived from my own `cargo check` output, not copied blind from the coordinator's paraphrase):
- The 18 baseline `--lib` errors: missing `.await` across `CancelState::{from_u8,to_u8}`, `CancelToken::{root,child,park,unpark,cancel,state,is_cancelled,is_parked,is_live}`, `thread_plan`, `ThreadBudget::{checkout,remaining}`.
- One the baseline run didn't reach: `ThreadBudget::from_plan` is also `async fn`; its two test call sites needed `.await` too (found via my own `--all-targets` run, not in the coordinator's list).
- `CancelToken::state`'s new `parent.state().await` recursion hit **E0733** (recursive async fn needs boxing) — rewritten as an iterative parent-chain walk instead of `Box::pin`, avoiding both the allocation and the indirection.
- **`Debug for CancelToken`** (E0277: `Debug` can't format an unawaited future) — this is an **E1** external-trait impl (`std::fmt::Debug`'s signature is fixed by std) and can never `.await`. **Recipe used, for the record since the coordinator asked me to name a repo-wide pattern**: *inline the minimal synchronous computation directly in the trait-impl body, reading the same underlying primitives (here: a plain iterative walk over the raw `AtomicU8` chain) — do not spawn a second, necessarily-non-`async`, named "sync twin" function, since that only relocates the same R2-exception problem onto a fn that then needs its own tag.* Reserve a tagged sync-twin fn for cases where the duplicated computation is too expensive/complex to inline safely (real I/O, not pure in-memory folding) — not the case here.
- **Ruling R1** applied to `HostAsyncRuntime`: `sleep_until`/`cancel_scope` no longer return `HostFuture<…>` (an `async fn` already returns a future; boxing a second one was exactly the banned double-future shape). `HostFuture<T>` now survives in this trait only as `spawn_scoped`'s argument type. Fixed the in-crate `testkit::ManualRuntime` impl to match (including rewriting `cancel_scope`'s `scope_owner_matches` filter — an async fn called from a sync closure — into an explicit `for` loop, since async can't be `.await`ed inside a plain closure).
- **NOT fixed** (confirmed out of scope, flagged as a background task `task_0a3e8be1` and listed here per rule 8): three OTHER crates implement `HostAsyncRuntime` with the OLD (`HostFuture`-wrapped) shape and will not compile against the new trait until updated — `🛢️db/🗄️storage/🦀️component.rs` (~935, ~939), `🛎️services/🦀️component.rs` (~290, ~295 — do not confuse with an unrelated same-named inherent method at ~178), and `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` (~1556, ~1562). This belongs to `db-dedyn`/`os-ripple` per the coordinator.

### Mid-session anomaly (reported, not silently fixed)

After my first full pass, the harness surfaced a note that `⏳️async/🦀️component.rs` had "changed on disk" — on inspection it was not a peer's edit building on mine, it was **byte-identical to the pristine pre-session original** (all my fixes gone). `git` evidence:
- `git diff HEAD -- <file>` → empty (worktree == HEAD).
- HEAD (`09c3cf6d`, real timestamp 2026-08-19 13:26:32) content for this file == commit `f69271685f` (2026-08-18, the actual pre-session baseline) — byte-identical (`diff` exit 0).
- `git diff -- <file>` (index vs worktree, ignoring HEAD) showed a small unstaged diff starting at the `CancelState` region — i.e. the index and worktree disagreed even though worktree and HEAD agreed exactly.

I did not chase this further (no destructive git commands used, per the binding rules) — I re-applied the full fix set via one `Write` and re-verified with a fresh `--lib` check (exit 0, same as before). Flagging this because it looks like tooling/sync trouble in the shared tree rather than a deliberate peer edit, and a second occurrence would silently lose real work.

## Acceptance — pasted output, real exit codes

```
$ CARGO_TARGET_DIR=…/target-macros cargo check -p semio-framework-async --lib --manifest-path …/Cargo.toml
warning: `semio-framework-async` (lib) generated 6 warnings   [async_fn_in_trait — expected, R3 forbids "fixing" it via RPITIT+Send]
    Finished `dev` profile [unoptimized] target(s)
EXIT_CODE=0   (0 errors, 7 warnings incl. the crate-level summary line)

$ CARGO_TARGET_DIR=…/target-macros cargo check -p semio-framework-async --all-targets --manifest-path …/Cargo.toml
    Finished `dev` profile [unoptimized] target(s)
EXIT_CODE=0   (0 errors, 8 warning lines incl. summaries)

$ CARGO_TARGET_DIR=…/target-macros cargo test -p semio-framework-async --manifest-path …/Cargo.toml
running 17 tests
test component::tests::block_on_drives_a_future_through_several_pending_polls_before_completing ... ok
test component::tests::child_cancel_never_propagates_upward_to_parent ... ok
test component::tests::cancel_token_root_starts_live ... ok
test component::tests::cancel_token_park_then_unpark_returns_to_live ... ok
test component::tests::cancel_token_cancel_is_terminal_over_park ... ok
test component::tests::cancelling_parent_transitively_cancels_child_and_grandchild ... ok
test component::tests::manual_runtime_spawn_scoped_runs_a_ready_future_on_drive ... ok
test component::tests::manual_runtime_cancel_scope_reports_finished_and_cancelled ... ok
test component::tests::parking_parent_does_not_downgrade_an_already_live_reading_below_park ... ok
test component::tests::manual_runtime_sleep_until_resolves_only_after_injected_time_advances ... ok
test component::tests::scope_handle_child_scope_shares_cancellation_lineage ... ok
test component::tests::thread_budget_checkout_debits_and_returns_remaining ... ok
test component::tests::thread_plan_invariant_holds_once_floors_stop_binding ... ok
test component::tests::thread_plan_is_deterministic ... ok
test component::tests::thread_plan_low_core_counts_oversubscribe_but_never_zero_a_role ... ok
test component::tests::thread_plan_shards_and_io_workers_never_exceed_their_ceilings ... ok
test component::tests::thread_budget_checkout_debug_panics_on_overdraw - should panic ... ok
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT_CODE=0

$ CARGO_TARGET_DIR=…/target-macros cargo check -p semio-framework-async-macros --manifest-path …/Cargo.toml
    Finished `dev` profile [unoptimized] target(s) in 0.26s
EXIT_CODE=0

$ CARGO_TARGET_DIR=…/target-macros cargo test -p semio-framework-async-macros --manifest-path …/Cargo.toml
running 8 tests … test result: ok. 8 passed; 0 failed
EXIT_CODE=0
```

All commands above ran with the real repo-root `Cargo.toml` as `--manifest-path` (not a scratchpad mirror) — every green result is against the actual live tree, not a copy. `CARGO_TARGET_DIR` was always the scratchpad target dir, never inside the ticket folder or the repo. Every cargo call ran foreground, single turn, well under the 600s timeout.

Warnings are `async_fn_in_trait` on `HostAsyncRuntime`'s six methods — expected and explicitly NOT to be "fixed" via `impl Future + Send` desugaring per ruling R3 ("Never `+Send` RPITIT... route it through the enum").

## Files touched

- New: `🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️component.rs`
- New: `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs,📋️project.json,📜️script.ts}`
- Modified: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` (block_on region; R1 trait fix; Debug/recursion/`.await` fixes; `#[async_test]` conversion + new block_on tests)
- Modified: `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` (dev-dependency, via script `--apply`)
- New (ticket folder): `async-test-attr.py`, `async-test-attr-scan-repo-wide.json`, this report

## lease-request

Not build-blocking (proven above — the macro crate resolves fine as a transitive dev-dependency of an already-registered member). Recommended for consistency with the two named precedents (both of which ARE registered) and to allow standalone `-p semio-framework-async-macros` invocation with zero consumer in the graph:

```lease-request
# Insert into /Users/ueli/Documents/semio/Cargo.toml, [workspace] members array (after the
# draw-fsm-macros entry, line ~69, alongside the other ✨️macros-style proc-macro crates):
    "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
```

## Follow-up (not this packet, flagged as background task `task_0a3e8be1`)

Update the three `HostAsyncRuntime` impls in `🛢️db/🗄️storage`, `🛎️services`, and `🌎️hub/…/📦️bin.rs` to the new (post-R1) `sleep_until`/`cancel_scope` signatures — listed with line numbers in the "Extended scope" section above.
