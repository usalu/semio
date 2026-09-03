# `semio-framework-2d`'s `serde` — put behind an off-by-default feature (change applied)

Scope: `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml`, `⚙️engine/🦀️.rs`, and the one real
consumer's manifest (`semio-framework-os-flow`). Positive result — change made, verified compiling
and testing clean on both the default (plugin) and `serde`-enabled (os-flow) paths.

## Part 1 — facts

1. **What `os-flow` needs serde for.** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️.rs`
   derives `Serialize`/`Deserialize` (non-`#[cfg(test)]`) on `DrawingNode`, `SceneNode`, and one
   private struct, over fields typed `Vec<semio_framework_2d::PathSegment>` /
   `Option<Vec<semio_framework_2d::PathSegment>>` (lines 180, 197, 258, 354). This was already
   documented in-repo by a prior pass on this same ticket (comment at `⚙️engine/🦀️.rs:24-34` before
   this edit, citing a confirmed `cargo check -p semio-framework-os-flow` → 15× E0277).

2. **Plugin-reachability, checked by real call sites, not just edges.** Two plugin crates depend on
   `semio-framework-2d`:
   - `semio-s-plugin-draw` (`✏️s/🔌️plugins/🖍️draw`) — its only use is in
     `🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
     (`to_kernel_segment`/`from_kernel_segment`), which hand-converts between the plugin's own
     `PathSegment` (its own `ToValue`/`FromValue`) and `semio_s_2d::PathSegment` field-by-field, plus
     calls to `semio_s_2d::booleans::boolean_paths_many` / `trace::trace_bitmap_paths` (pure
     functions). No `serde::Serialize`/`Deserialize` call on the kernel type anywhere.
   - `semio-s-plugin-flow-extension-draw` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs:6`) only
     imports `DrawingError` and `Vec2` — neither is `serde`-derived (`DrawingError` has
     `#[derive(Clone, Debug, PartialEq)]` only; `Vec2` is a bare `type Vec2 = [f64; 2]`).
   - **Correction to the task's premise:** this second plugin crate *does* also depend directly on
     `semio-framework-os-flow` itself (as `flow_extension_sdk`), and os-flow genuinely appears in its
     `wasm32-wasip2` component tree (`cargo tree -p semio-s-plugin-flow-extension-draw --target
     wasm32-wasip2 --edges normal --prefix none | grep -i os-flow` → hits). So "os-flow never ships
     in a plugin component" is not universally true — it is true only for the specific package named
     in the task's payoff check, `semio-s-plugin-draw`. This does not block the change: neither
     plugin's *own code* needs serde on 2d types, and Cargo feature unification means
     `flow-extension-draw`'s build already carries serde regardless (via its direct `os-flow` dep,
     unconditional in `os-flow`'s own manifest) — enabling 2d's new `serde` feature there via
     unification adds nothing new.

3. **`os-flow` absent from `semio-s-plugin-draw`'s component — confirmed both before and after.**
   `cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 --edges normal --prefix none | grep -i
   os-flow` → no output, both before and after this edit.

4. **Existing feature-flag convention on `semio-framework-2d`.** Two feature flags already existed,
   `booleans` and `trace` (both `= []`, pure code-inclusion — `#[cfg(feature = "…")]` on `#[path]`
   module mounts in `📦️packages/🦀️rust/🦀️.rs`, neither gates a dependency). No existing
   `serde`-shaped feature to imitate inside this crate, so the change follows the sibling
   `semio-framework-ui` `wgpu`/`wgpu-engine` precedent's *shape* instead: mark the dependency
   `optional = true` and add a `serde = ["dep:serde"]` feature line, with the one consumer opting in
   via `features = ["serde"]` on its manifest edge — the same style `os-flow`'s own manifest already
   uses one line above for `ui_wgpu`'s `features = ["wgpu"]`.

   Also confirmed the crate's *only* `serde` usage anywhere is the single `PathSegment` derive in
   `⚙️engine/🦀️.rs` — grepped `booleans/🦀️.rs`, `trace/🦀️.rs`, and the rest of `⚙️engine/🦀️.rs`
   (`DrawingError`, `Vec2`, `EngineCache`/`semio_framework_hash` content-addressing) for
   `Serialize`/`Deserialize`/`serde::` — zero further hits. `DrawingError` derives only
   `Clone, Debug, PartialEq`; `EngineCache`'s content-addressed cache keys use
   `semio_framework_hash::hash`, unrelated to `serde`.

## Part 2 — change applied

- `⚙️engine/🦀️.rs`: `use serde::{...}` gated `#[cfg(feature = "serde")]`; `PathSegment`'s derive
  split into an always-on `#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]` plus
  `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` /
  `#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "camelCase"))]`. `ToValue`/
  `FromValue` (the plugin wire encoding) are untouched and stay unconditional.
- `📦️packages/🦀️rust/Cargo.toml`: `serde = { workspace = true, optional = true }` plus a new
  `[features] serde = ["dep:serde"]` line (not in `default`).
- `os-flow`'s `📦️packages/🦀️rust/Cargo.toml` line 33: `semio-framework-2d = { path = "…" }` →
  `{ path = "…", features = ["serde"] }` — same line-shape as the existing `ui_wgpu` entry two lines
  above it.
- `semio-s-plugin-draw` and `semio-s-plugin-flow-extension-draw` manifests: **untouched** — they get
  the feature-off default.

## Verification

`CARGO_TARGET_DIR=…/scratchpad/iso3`, `RUSTC_WRAPPER=""`, all foreground:
- `cargo check -p semio-framework-2d --message-format short` → exit 0, 0 errors (`grep -cE ':
  error(\[|:)'` → 0).
- `cargo check -p semio-framework-2d --features serde --message-format short` → exit 0, 0 errors.
- `cargo test -p semio-framework-2d` (default features, i.e. no `serde`) → **23/23 passed**, 0
  failed (`booleans`, `os_engine`, `trace` suites).
- `cargo check -p semio-framework-os-flow --message-format short` (the consumer) → 28 errors, **all**
  in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/…/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`
  (`DagFixture`/`DagError`/`JsonValue` vs `Value` mismatches — E0277/E0283/E0308). Zero mention of
  `PathSegment`, `drawing`, or `semio-framework-2d` anywhere in the error list; `semio-framework-2d`
  itself compiled clean before the error output starts. This crate was never touched by this change
  and matches the task's own warning that unrelated concurrent churn (elsewhere than `🔁️workflow`,
  in this case `♾️infinite`'s dag board) is expected — not introduced here.
- `cargo check -p semio-s-plugin-flow-extension-draw --target wasm32-wasip2 --message-format short`
  → 2 errors, both the *same* `dag/🦀️.rs:5343`/`:1701` E0283 lines as above (this plugin pulls
  `os-flow` → `infinite`). Same pre-existing, unrelated churn, not this change.
- `cargo metadata --no-deps --format-version 1 >/dev/null; echo $?` → `0` (checked after every
  manifest edit, per the rule).

## Payoff — before/after, `semio-s-plugin-draw`, `--target wasm32-wasip2`

```
cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 --edges normal --prefix none \
  | awk '{print $1}' | sort -u | grep '^serde'
```
Both before and after: `serde`, `serde_core`, `serde_derive`, `serde_json` — **unchanged**. Expected:
10 of the other 11 listed in-component crates (`semio-framework`, `-actor`, `-graph`, `-os-kernel`,
`-os-kernel-dsl-derive`, `-plugin`, `-replication`, `-ui`, `-ui-contract`, `-ui-scene`) still declare
`serde` unconditionally and remain in this plugin's tree regardless — `-2d` alone reaching zero was
never going to zero the family out.

**This change DOES remove `semio-framework-2d` from the list of 11.** `cargo tree -p
semio-s-plugin-draw --target wasm32-wasip2 -i serde --edges normal`, before the edit, showed
`semio-framework-2d` as a **direct** child of the `serde` root (its own unconditional dependency).
After the edit, that direct edge is gone — `semio-framework-2d` only still appears in the inverted
tree nested *under* `semio-framework-os-kernel` (`serde ← os-kernel ← 2d ← plugin-draw`), because
`2d` has a normal (unrelated) dependency on `os-kernel`, and `os-kernel` is itself one of the other
10 crates still on the list — not because `2d` requires `serde` anymore. So: **10 in-component
crates left**, `-2d` is off the list.

## Files changed
- `🧰️framework/🔨️modules/◻️2d/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
