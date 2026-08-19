# 📓️ `db-trait-flip` — FINISHED TO GREEN (the interrupted atomic packet)

**Owner decision received: finish, not revert.** This is the packet `📓️status.md`'s
"🔴️ `semio-framework-os-kernel-db` is RED — 84 errors" entry flagged upward. It is now closed out.

## Result

| gate | before | after |
|---|---|---|
| `cargo check -p semio-framework-os-kernel-db --lib` | **83 errors** | **exit 0** |
| `… --all-targets` (rule 26) | **361 errors** | **exit 0** |
| `cargo test -p semio-framework-os-kernel-db --lib` | could not build | **424 passed / 0 failed / 0 ignored** |
| `cargo check -p semio-hub --all-targets` | 3 errors | **exit 0** |
| `cargo test -p semio-hub` | could not build | **11 + 20 passed / 0 failed** |

(The handover said 84; the tree measured 83 — one had already been absorbed by drift. Not a missed fix.)

## The decision the packet was missing: where the sync/async boundary sits

`db_storage`'s trait family was already flipped to `DbFuture<'a, T>`; **no caller had been converted at
all** (`grep -c "async fn"` was 0 in every db component). The packet stopped exactly between its two
halves, so the boundary had never been chosen. It is now:

- **Pure-logic layers are `async fn`** — `db_snapshot`, `db_wal`, `db_index`, `db_compact`, `db_sync`,
  `db_cluster`, `db_projection`, `db_query`. They own no threads. `async fn` on inherent methods is
  `wasm32`-clean and needs no boxing, unlike the trait family (which needs `DbFuture` to stay dyn-compatible).
- **Thread-owning layers keep their sync signatures and bridge once** with `db_actor::block_on` —
  `db_artifact` (bodies run on `ArtifactAuthority`'s actor thread), `db_engine` (per-submit bridge
  threads), `db_cli` (single-shot process), and every `#[cfg(test)]` module.
- **`🌎️hub` is genuinely async** and simply `.await`s (`handle_frontier_advertise`) — no bridge.

This honours the handover's hard constraint verbatim: **no db-actor thread was converted and no
`db_engine` per-submit bridge thread was deleted.** Blocking moved *outward one level* — out of each
storage backend body, into the one thread that already owned that call. This is the same blocking that
previously lived inside the old `block_on` bridges, which is why the async bodies already existed.

## Two dyn-compatibility escapes the cascade forced

`db_query::ConsistencyResolver` and `db_query::FullTextLookup` are consumed as `&dyn`, so AFIT was not
available (the same constraint `db_storage`'s module doc documents for its own family). Both now return
`db_storage::DbFuture<'a, T>` with `Box::pin(async move …)` impls — mirroring the storage family rather
than inventing a second shape. Their test doubles return `Box::pin(std::future::ready(…))`.

## `inline_fs_runtime` (the `E0425`) — resolved by deduplication, not by writing it

`db_engine::Database::open_at` referenced a function that was never written. `db_cli` already carried a
private `CliRuntime` doing precisely that job. Per CLAUDE.md ("if code is repeated it MUST be close to
each other"), the single implementation now lives **beside the `FsStorage` that requires it** as
`db_storage::InlineRuntime`, with `FsStorage::open_inline(owner, root)` as the one-call bridge.
`CliRuntime` is deleted. `db_engine::open_at`, `db_cli::open_fs_storage` and `db_testkit`'s FS law test
all go through it. It carries the same gating as `FsStorage` (`feature = "fs"`, not `wasm32`).

That helper also absorbed two stale 1-argument `FsStorage::open(root)` call sites (in `db_cli` at HEAD
and in `db_testkit`) that the interrupted packet had left behind — they could not have compiled.

## One real defect found by finally being able to RUN the suite

`db_preview::tests::preview_crate_never_references_wal_shaped_symbols` — the crate's single most
important law ("previews are never durable") — **failed**. Not from this work: the W6 packet had added
prose to `📦️packages/🦀️rust/Cargo.toml` explaining the sync/async boundary, and that prose names
`db_storage`. The guard did a raw `manifest.contains("db_storage")`, so a *comment* tripped a
*dependency* law. It had been invisible because the crate has not compiled since.

Fixed at the guard: comment lines are stripped before the check, so the law tests what it means to test.
**This is rule 26's own point one level up — a green `--lib` is not a passing test suite, either.**

## `wasm32-unknown-unknown`: red, and NOT a regression from this work

`--target wasm32-unknown-unknown --lib` reports 66 errors. Verified pre-existing, not assumed:

- `db_artifact` calls `Receiver::recv_blocking` / `Address::ask_blocking`, both
  `#[cfg(all(not(target_arch = "wasm32"), feature = "thread"))]` in `db_actor`. `git diff` shows **zero**
  working-tree changes from me in that file, and `git log -S` dates those calls to **2026-08-10**, nine
  days before this session.
- `db_engine` / `db_cli` name `FsStorage`, which is correctly `wasm32`-gated. `git show HEAD:…` confirms
  `db::storage::FsStorage` was already referenced there before my edit.

So `db_artifact`/`db_engine`/`db_cli` have never been `wasm32`-clean; they are the thread- and
fs-owning layers. The module doc's `wasm32` claim is scoped to `db_storage` itself ("`FsStorage` …
compiles to an effectively-empty module on a `wasm32-unknown-unknown` target check"), and that still
holds. My bridging added further `block_on` call sites **inside those same already-native-only modules**
and none anywhere else — every pure-logic layer went `async fn` precisely so it stays `wasm32`-clean.
Making the thread-owning trio `wasm32`-clean is the pending runtime/db refactor's job, not this packet's.

## Method — the fixpoint scripts (kept in this folder)

The cascade was ~450 mechanical edits, so it was driven by the compiler rather than by grep, in four
shapes, each looping to a fixpoint on `--message-format=json` spans:

| script | shape it fixes |
|---|---|
| `asyncify-db.py` | `E0277` `?`-on-future → `.await`; `E0308` tail → `.await`; `E0728` → make the enclosing `fn` `async`. Allowlisted to the pure-logic modules only |
| `bridge-db.py` | the same two shapes in the thread-owning modules → wrap in `block_on` |
| `bridge-scrutinee.py` | diagnostics that point at a `match` arm / `if let` binding rather than at the future → wrap the scrutinee |
| `bridge-receiver.py` | `.unwrap()`/`.is_ok()` straight on a future: walks the receiver chain backwards over balanced parens and wraps exactly the receiver |

⚠️ **Two traps worth recording, both of which the compiler caught and neither of which grep would have:**
1. A non-greedy "wrap the tail expression" regex swallows `assert_eq!(`'s opening paren, producing
   `block_on(assert_eq!(x)).unwrap(), y)`. The paren structure is identical to the correct form, so the
   repair is a *swap* of the two prefixes, not a re-parse.
2. A "wrap this bare binding" pass keyed on a variable NAME (`result`) is not scoped to the test region
   and will hit production code with the same name. It did — one site in `db_compact::Compactor::run`,
   caught by `Result<…> is not a future` and reverted. **Name-keyed edits need an explicit line-range
   guard**; the test-side script has one (`#[cfg(test)]` line number), the ad-hoc pass did not.
