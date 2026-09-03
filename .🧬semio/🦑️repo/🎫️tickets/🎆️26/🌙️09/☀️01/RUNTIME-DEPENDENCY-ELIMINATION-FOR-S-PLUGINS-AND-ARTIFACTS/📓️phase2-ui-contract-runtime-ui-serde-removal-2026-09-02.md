# Phase 2 — serde removal: ui-contract, ui-runtime, ui (2026-09-02)

Scope: `semio-framework-ui-contract`, `semio-framework-ui-runtime`, `semio-framework-ui`. Followed
the mandated loop (grep → strip → `cargo check` → fix-or-restore) per crate, foreground, no
sub-agents, no worktrees, `iso3` target dir.

## semio-framework-ui-contract — RESTORED, serde stays (documented, intentional)

The crate's own header docstring (`🦀️.rs:11-15`) already names serde as one of exactly three
things this crate is allowed to depend on ("Dependency-free. serde, the styling tokens, and
(additively) `protocol::value`'s `ToValue`/`FromValue`... — nothing else"). Confirmed empirically
rather than trusting the docstring alone: commented out the `serde = {...}` dependency line and ran
`cargo check -p semio-framework-ui-contract` → **248 errors**. Restored the line immediately;
crate is back to 0 errors, `cargo metadata` exit 0.

Root cause matches the ticket's own prior notes: `UiValue`/`UiFixedList`/`UiFixedMap`/`Component`/
`UiSnapshot`/`UiPatch*` and everything that embeds them (most of `document.rs`, `component.rs`,
`builder.rs`, `surface.rs`) are the deliberately-uncovered family — `UiValue`'s docstring forbids
adding `DslValue` conversions here (that bridge belongs in os-kernel, already implemented as
`ui_value_to_dsl_retained`). serde is these types' only wire format. No action needed; this is the
crate's permanent, intended end state.

## semio-framework-ui-runtime — STRIPPED, serde moved to `[dev-dependencies]`

`grep -rn serde` found exactly one non-test symbol: a hand-written
`impl serde::Serialize for TransactionPatch` in `🦀️transaction.rs:159-161`, module-level (not
`#[cfg(test)]`). Repo-wide grep confirmed `TransactionPatch`/`Transacted` have zero consumers
outside this crate's own `#[cfg(test)] mod tests` (which is where every `serde_json::to_string(...)`
call on it lives — `🦀️transaction.rs` lines ~1152/1196/1225). Every other `serde`/`serde_json` hit
in `🦀️reconcile.rs` was already inside `#[cfg(test)]` fns (`reconcile`, `snapshot`) or the test
module.

Changes:
- `🦀️transaction.rs`: gated the `impl serde::Serialize for TransactionPatch` block with
  `#[cfg(test)]` (docstring added explaining why).
- `Cargo.toml`: removed `serde` from `[dependencies]`; added it alongside the existing
  `serde_json` in `[dev-dependencies]`.

Verified: `cargo check -p semio-framework-ui-runtime` (lib) → 0 errors.
`cargo check -p semio-framework-ui-runtime --tests` → 1 error, but it is
`could not read .../🖱️ui/📤️output/🧪️test/🦀️s.rs: No such file or directory` from an
`include!(...)` at `🦀️reconcile.rs:3790` inside `mod output_pool_tests` — a file this session never
touched, in code this session never touched, referencing a path that does not exist on disk. This
is concurrent/in-progress peer work (per prior session note "Concurrent Cargo Workspace Churn"), not
caused by the serde strip; not chased per the standing instruction.
`cargo metadata --no-deps --format-version 1` → exit 0.

## semio-framework-ui — ALREADY CORRECT, no change needed

`grep -rn serde` only matches files under `🎯️targets/🧊️wgpu/` (`🤖️generated.rs`,
`🦀️icon_name_value.rs`, `🦀️locale_terminology_value.rs`, `🦀️label.rs`, `🦀️component.rs`) — the
sibling-file `ToValue`/`FromValue`-via-serde bridge for machine-generated `IconName`/
`Locale`/`Terminology`, exactly the precedent the ticket brief names. All of it is reachable only
through the `wgpu` feature, which already lists `serde`/`serde_json` as `optional = true` deps
pulled in solely by `dep:serde`/`dep:serde_json` in the `wgpu` feature array. With default features
(what a plugin wasm component builds), `cargo tree -p semio-framework-ui --no-default-features -e
no-dev --depth 1` shows this crate's own direct deps as only `ui_contract`/`ui_scene`/`ui_styling`
— no serde. Confirmed both configurations compile: `cargo check -p semio-framework-ui` (default) →
0 errors; `cargo check -p semio-framework-ui --features wgpu-engine` → 0 errors.

## Final verification (all green)

```
cargo check -p semio-framework-ui-contract --message-format short   → 0 errors
cargo check -p semio-framework-ui-runtime  --message-format short   → 0 errors
cargo check -p semio-framework-ui          --message-format short   → 0 errors
cargo check -p semio-framework-ui --features wgpu-engine             → 0 errors
cargo check -p semio-framework   --message-format short             → 0 errors
cargo metadata --no-deps --format-version 1 >/dev/null; echo $?      → 0
```

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️transaction.rs` — gated
  `impl serde::Serialize for TransactionPatch` under `#[cfg(test)]`.
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/Cargo.toml` — moved `serde` from
  `[dependencies]` to `[dev-dependencies]`.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — no net change (strip
  attempted and reverted; restored to identical original content).
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` — no change (already correct).
