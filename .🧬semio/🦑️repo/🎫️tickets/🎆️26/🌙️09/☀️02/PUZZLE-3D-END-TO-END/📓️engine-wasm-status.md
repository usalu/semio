# Engine WASM Build Status — `dev:puzzle:3d`'s three crates

Checked 2026-09-03, ~01:00–01:08 CEST, via `RUSTC_WRAPPER="" cargo check ... --message-format short`.

The shared `/Users/ueli/Documents/semio/target` directory was locked by dozens of concurrent
`cargo check` processes from other live sessions (some running for 1h+, including a peer already
running the exact `semio-framework-os-flow --target wasm32-wasip2` check). Runs against the shared
target dir blocked indefinitely ("Blocking waiting for file lock on build directory"). Switched to
an isolated `CARGO_TARGET_DIR` (a scratch dir under this ticket's session scratchpad) to get a
clean, unblocked answer — this trades cache reuse for isolation, so timings below are a mostly-cold
build, not representative of steady-state.

## Package names

| Engine crate | Path | Cargo package |
|---|---|---|
| surface | `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust` | `semio-framework-surface` |
| editor | `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust` | `semio-framework-editor` |
| flow-core | manifest at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (not under `🫀️core/📦️packages/🦀️rust`, which has no `Cargo.toml`) | `semio-framework-os-flow` |

## Result summary (isolated `CARGO_TARGET_DIR` run, current as of 01:08 CEST)

| Crate | Native `cargo check` | `--target wasm32-wasip2` |
|---|---|---|
| `semio-framework-surface` | **5 errors** (all transitive, see below) | not requested (only flow-core needed per task) |
| `semio-framework-editor` | **5 errors** (same transitive cause) | not requested |
| `semio-framework-os-flow` | not run standalone (native check for this package would pull the same transitive dependency; skipped since the wasm check — the one that matters for `dev 3d` — was run directly) | **2 errors** (same transitive cause, subset) |

**All three are currently blocked by the same single root cause**, not by anything in their own
source: `semio-framework-os-infinite` (pulled in transitively by all three) fails to compile with

```
error[E0283]: type annotations needed
```

at three sites in one file:
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1701:24`,
`:1702:24`, and `:5343:40` (each a `Value::from(<ambiguous numeric>)` call whose target numeric
type can no longer be inferred). Native check reports 5 errors total (surface/editor); the
wasm32-wasip2 check reports 2 (some of the surrounding code is cfg'd out for that target).

**This is a live in-flight edit, not touched.** The file is currently `git status` dirty
(uncommitted `M`), and its mtime was ~17–25 minutes before this check (well inside the ~30-minute
"another session is actively working on this" window). It sits outside all three target engine
crates' own source trees (it's a shared `board`/`infinite` dependency), so per the task's explicit
instruction this was left alone rather than attempting to finish someone else's edit. Record only;
no fix attempted.

## Superseded earlier reading (do not use)

An earlier check against the contended **shared** target dir (before switching to the isolated
one, ~00:15–00:32 CEST) had shown a completely different error class — `E0277` "trait bound ...
serde::Serialize/Deserialize ... not satisfied" on `JobCheckpoint`/`ActorInstanceOpenRequest`/
`ActorInstanceCloseRequest`/`ActorInstanceLifecycleAck`/`ActorInstanceLifecycleReceipt`/
`ActorUiPatchReceipt` (surfaced through `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`'s `Event` enum) for
surface, and on `PropertyValue`/`manifest::Manifest` (surfaced through `semio-framework-graph`) for
editor/flow-core. Both were independently confirmed (by this check and by a sibling agent's
`📓️value-derive-sweep.md` in this same ticket folder) to be transient fallout from another
session's live, uncommitted edits to `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs` and
`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` (both had mtimes seconds-to-minutes old at the
time, under the same parent ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
ARTIFACTS`). That state had fully settled/moved on by the time of the isolated-target-dir re-check
above — those specific errors no longer reproduce. Nothing was fixed for this class either; it
resolved itself as the other session's edit progressed.

## What was fixed

Nothing. Every error observed across both check passes traced to another session's in-flight,
uncommitted edit in a shared dependency outside the three target crates' own source — not to a
small, unambiguous, isolated bug (missing import, stale path, half-applied `#[value(...)]`) inside
`surface`, `editor`, or `flow-core` themselves.

## What was deliberately left

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:1701,1702,5343`
  — `E0283` ambiguous numeric type in `Value::from(...)` calls. Blocks native builds of `surface`
  and `editor`, and the `wasm32-wasip2` build of `flow-core` (i.e. blocks `dev:puzzle:3d` right
  now). Left alone: file is mid-edit by another session (uncommitted, ~20 min old).
- (Superseded, no longer reproducing) `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs` /
  `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` serde-derive gaps — left alone at the time for
  the same reason; had already resolved by the next check.

## Bottom line for `dev:puzzle:3d`

As of 01:08 CEST, the `wasm32-wasip2` build of `semio-framework-os-flow` (the package `dev 3d`
actually builds) **fails with 2 errors**, both inherited from `semio-framework-os-infinite`'s
`🕸️dag/🦀️.rs`, which is currently being edited live by another session. Re-check once that file's
edit lands/commits.
