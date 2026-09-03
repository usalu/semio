# Phase 2 — actor / replication / os-kernel-dsl-derive serde removal attempt (2026-09-02)

## Result: all three crates END with serde exactly where they started (none stripped)

### `semio-framework-actor` — attempted, REVERTED, blocked outside the crate

Applied the sanctioned `#[cfg_attr(test, derive(Serialize, Deserialize))]` +
`#[cfg_attr(test, serde(...))]` pattern (proven by `🔺️mesh-engine`/`🪵️sourcing`/`-3d`) across all four
files (`🦀️.rs`, `🚪️lifetime/🦀️.rs`, `🚪️lifetime/🩹️patch/🦀️.rs`, `📤️return/🦀️.rs`; 45 derive sites +
the `decimal_generation`/`request_sequence` hand-written codec modules), moved `serde` to
`[dev-dependencies]`. `cargo check -p semio-framework-actor` — 0 errors. `cargo test -p
semio-framework-actor --lib` — 121 passed.

**But** `cargo check -p semio-framework` then broke with 28 errors, all `E0277`
(`X: serde::Serialize`/`Deserialize` not satisfied) from
`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — a module OUTSIDE the three crates in scope. Its `Event`
enum (line ~981) and sibling types derive `#[derive(Clone, Debug, PartialEq, Serialize,
Deserialize)]` UNCONDITIONALLY (not test-gated) and embed `ActorInstanceOpenRequest`,
`ActorInstanceCloseRequest`, `ActorInstanceLifecycleAck`, `ActorInstanceLifecycleReceipt`,
`ActorUiPatchReceipt`, and `JobCheckpoint` (re-exported from `semio_framework_actor::instance_lifetime`)
directly as fields — real production code, not a test oracle.

Per rule 5 ("if NOT fixable from inside the crate, RESTORE the dependency... a partial honest result
beats a broken tree") and the standing rule not to touch crates outside the three: reverted all five
touched files (`🦀️.rs`, `🚪️lifetime/🦀️.rs`, `🚪️lifetime/🩹️patch/🦀️.rs`, `📤️return/🦀️.rs`, and the
crate's `Cargo.toml`) via `git show HEAD:<path>` back to tracked content (no modifying git command
used — read-only `git show`, confirmed via `git status --short` these 5 files were the ONLY diff
against `HEAD` before revert, so this is an exact restore, not a guess). Post-revert: `cargo check -p
semio-framework` — back to 0 errors.

**Conclusion**: `semio-framework-actor`'s `serde` cannot move to `[dev-dependencies]` without also
fixing `🎠️kernel`'s `Event`/co-located types (outside this ticket's three-crate scope). `serde` stays
in `[dependencies]`, unchanged from session start.

### `semio-framework-replication` — verified still blocked, left untouched

The crate's own `Cargo.toml` already carries a detailed "tenth seam pass" note (dated 26/09/01,
apparently from an earlier session) naming three live blockers. Re-verified all three fresh today
rather than trusting the comment:
1. `🧰️framework/🔨️modules/🌱️value/🦀️.rs:281` — `impl serde::Serialize for DslValue` (and
   `:288` `impl<'de> serde::Deserialize<'de> for DslValue`, `:218` `impl From<&DslValue> for
   serde_json::Value`) — confirmed present, a deliberate bridge for other serde-deriving types
   holding a `DslValue` field (e.g. `ui_wgpu`'s `ActionDescriptor`) plus real JSON-export bridges.
2. `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️.rs:66-75` — `impl serde::Serialize for
   OrderedSet` / `impl<'de> serde::Deserialize<'de> for OrderedSet` — confirmed present, real callers
   in `os-flow`/`procedural` plugins.
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `protocol::InteractionState` hit
   directly via `serde_json::to_string`/`from_str`/`from_slice` at (confirmed, line numbers shifted
   slightly from the note but same call sites) 9857/9861/9872, never through a `pack`/`ToValue` seam.

All three consumers live in `🌱️value` (a different framework module, and per the note's own scope
this crate `EXPORTS` the value type system to that module) and `💻️os/🔌️plugin` — both outside the
three crates in scope for this ticket. No edit made to `replication`'s `Cargo.toml` or sources.
`cargo check -p semio-framework-replication` — 0 errors (baseline, unchanged).

### `semio-framework-os-kernel-dsl-derive` — proc-macro-only, sanctioned to leave, no change made

`[lib] proc-macro = true` (verified in the crate's own `Cargo.toml`). All `serde_json::` usage in
`🦀️.rs` (~30 call sites) runs INSIDE the proc-macro's own expansion-time logic (parsing
`🔣️.json` taxonomy/fixture files off disk while generating code for a caller) — this is host-side
build tooling, never code that ships in the caller's compiled output, since a `proc-macro = true`
crate is never linked into any downstream artifact (wasm component included) — only invoked by
`rustc` on the host during a dependent crate's build.

`cargo tree -p semio-framework-os-kernel-dsl-derive` confirms `serde_json` is a direct
`[dependencies]` edge of this proc-macro crate itself (not a transitive edge through some other
runtime dependency), and `serde` is `[dev-dependencies]`-only (used by the crate's own `#[test]`s).
No downstream crate re-exports or forwards this dependency — it terminates here. Left unchanged, as
instructed ("a proc-macro-only dep is fine to leave").

## Verification (iso3, RUSTC_WRAPPER="")

- `cargo check -p semio-framework-actor` — 0 errors
- `cargo check -p semio-framework-replication` — 0 errors
- `cargo check -p semio-framework-os-kernel-dsl-derive` — 0 errors
- `cargo check -p semio-framework-os-kernel` — 0 errors
- `cargo check -p semio-framework` — 0 errors
- `cargo test -p semio-framework-actor --lib` — 121 passed; 0 failed
- `cargo metadata --no-deps --format-version 1` — exit 0
- `git status --short` on all three crate directories — clean (no diff against `HEAD`)

## Files touched (net effect: none — attempted-and-reverted only)

- `🧰️framework/🔨️modules/🎭️actor/🦀️.rs` — edited then reverted to `HEAD`
- `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs` — edited then reverted to `HEAD`
- `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs` — edited then reverted to `HEAD`
- `🧰️framework/🔨️modules/🎭️actor/📤️return/🦀️.rs` — edited then reverted to `HEAD`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` — edited then reverted to `HEAD`
- `semio-framework-replication`, `semio-framework-os-kernel-dsl-derive` — inspected only, no edits

## What would unblock actor and replication (out of scope for this pass)

- Actor: gate or convert `🎠️kernel/🦀️.rs`'s `Event` enum (and any sibling types embedding actor
  wire types) to `ToValue`/`FromValue`, or `#[cfg_attr(test, derive(Serialize, Deserialize))]` it too
  — that module is not one of this ticket's three crates.
- Replication: give `DslValue` and `OrderedSet` first-party `ToValue`/`FromValue` (already exists for
  `DslValue`'s own shape trivially; the serde impls are for OTHER types embedding them) and convert
  `ui_wgpu::ActionDescriptor`, the `os-flow`/`procedural` `OrderedSet` callers, and
  `💻️os/🔌️plugin/🦀️.rs`'s `InteractionState` call sites to a `pack`/`ToValue` seam instead of
  `serde_json` directly — none of that lives in `replication` itself.
