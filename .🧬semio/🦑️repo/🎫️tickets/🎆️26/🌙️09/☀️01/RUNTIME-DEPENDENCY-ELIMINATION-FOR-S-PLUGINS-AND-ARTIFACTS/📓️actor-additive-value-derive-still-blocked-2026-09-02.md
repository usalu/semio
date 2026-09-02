# 🎭️actor additive ToValue/FromValue — still architecturally blocked, even after the crate= fix

## Task
Additive `ToValue`/`FromValue` pass over `semio-framework-actor`'s 57 serde-only types, using the
newly-landed `#[value(crate = "…")]` escape hatch documented in
`📓️value-derive-crate-attribute-and-variant-field-fixes-2026-09-02.md`, pointed at whatever value
root actor can reach without a Cargo cycle. No Cargo.toml edits permitted.

## Baseline count (verified, not assumed)

```
grep -rln 'derive(.*Serialize.*Deserialize\|derive(.*Deserialize.*Serialize' --include="🦀️.rs" \
  "🧰️framework/🔨️modules/🎭️actor" | grep -v "/🧪️"
```
→ 4 files (`🦀️.rs`, `📤️return/🦀️.rs`, `🚪️lifetime/🦀️.rs`, `🚪️lifetime/🩹️patch/🦀️.rs`), **57
derive sites** total (matches the brief's count exactly).

## Finding: no reachable path, confirmed two independent ways

**1. `cargo tree -p semio-framework-actor -e normal`** — full normal-dependency graph is:
```
semio-framework-actor
├── semio-framework-job → semio-framework-async → semio-framework-trace
│                       → semio-framework-trace
└── serde (+ serde_core, serde_derive, proc-macro2, quote, syn, unicode-ident)
```
No `semio-framework-value-derive` (the proc-macro crate itself), no `semio-framework-replication`
(owns `🌱️value/🦀️.rs`, the only crate that mounts `DslValue`/`ToValue`/`FromValue`/`ValueError`),
no `semio-framework-os-kernel`, and no `serde_json` (only present as a `[dev-dependencies]` entry,
invisible to a normal `cargo check`). `job`→`async`/`trace` is a dead end; wasm-only `async` and
`wasm-bindgen` are cfg-gated to `target_arch = "wasm32"` and carry no value-crate path either.

**2. Real compile-error proof** (not inferred): temporarily added
`::semio_framework_value_derive::{ToValue, FromValue}` to `ActorReturnOrigin`'s derive list in
`📤️return/🦀️.rs` plus a matching `#[value(rename_all = "camelCase", deny_unknown_fields)]`, then ran
`cargo check -p semio-framework-actor --message-format short`:

```
error[E0433]: cannot find `semio_framework_value_derive` in the crate root: could not find
  `semio_framework_value_derive` in the list of imported crates
error: cannot find attribute `value` in this scope
```

This is the important distinction from the crate= fix: the override only changes which path the
**generated code** references for `DslValue`/`ToValue`/`FromValue`/`ValueError` — it does nothing for
reaching the **`#[derive(ToValue, FromValue)]` proc-macro itself**, which still requires
`semio-framework-value-derive` as an actual Cargo dependency to be invocable at all. Actor has zero
edge to that crate, so the crate= escape hatch cannot help here regardless of what path is passed to
it: there is no path to try.

Reverted via `git apply -R` on the saved diff; confirmed `git status --porcelain` on `🎭️actor/` was
clean afterward, then re-ran both baseline checks below to confirm 0/0 unchanged.

## Ruled out (same reasoning as the prior blocked report, `📓️mcp-additive-value-derive-actor-blocked-
2026-09-02.md`, still valid)
- `os-kernel` is out: it already depends on `semio-framework-actor` (`features = ["ureq"]`) — a
  back-dependency is a manifest-level cycle Cargo rejects outright.
- `#[path]`-mounting `🌱️value/🦀️.rs` straight into `actor` without touching Cargo.toml was
  considered again and rejected again, for a stronger reason than "nominally distinct type": that
  file's unconditional (non-test, non-cfg-gated) `impl From<&DslValue> for serde_json::Value` /
  `impl serde::Serialize for DslValue` bridges (lines ~218-298) require `serde_json` as a real
  dependency, which `actor` does not have outside `[dev-dependencies]` — mounting the file as-is
  would not even compile, before the type-identity problem is reached. Splitting the file to mount
  only `🔁️codec/🦀️.rs` would dodge that but still requires `actor` to locally define its own
  `DslValue`/`Number` (the `codec` module's `use super::{DslValue, Number}` expects them in scope) —
  reproducing the type-identity dead end the prior report already ruled out, and touching a shared
  file (`🌱️value/🦀️.rs`) outside this module's ownership either way.

## Verification (real, run)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework-actor --message-format short   # 0 errors, baseline unchanged
cargo check -p semio-framework --message-format short         # 0 errors, baseline unchanged
```
Both counted with `grep -cE ': error(\[|:)'` (anchored `^` avoided per the ticket's own warning).

## Outcome
- **Before: 57 serde-only derives. After: 57 serde-only derives (unchanged).**
- **0 files under `🎭️actor/` edited** — the one experimental edit was reverted via `git apply -R`
  and confirmed clean via `git status --porcelain`.
- **No Cargo.toml touched anywhere.**
- **No `Serialize`/`Deserialize` removed or otherwise modified** — nothing in `🎭️actor/` was changed
  at all in the final state.
- The `#[value(crate = "…")]` fix does not unblock `actor`: it solves *which path the generated code
  points at*, not *whether the derive macro crate itself is reachable*. Actor has no dependency edge
  to `semio-framework-value-derive` and none is addable without a `Cargo.toml` edit, which remains out
  of scope for this ticket's rules. This is a hard architectural blocker, not a missed technique.
