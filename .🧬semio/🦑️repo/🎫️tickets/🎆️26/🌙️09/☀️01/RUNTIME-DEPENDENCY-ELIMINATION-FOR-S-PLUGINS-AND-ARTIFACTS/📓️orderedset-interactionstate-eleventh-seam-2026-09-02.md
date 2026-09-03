# OrderedSet + InteractionState seam (blockers 2 & 3), eleventh pass — 2026-09-02

Scope: `semio-framework-replication` (package `protocol`). Blocker 1 (`DslValue`'s own
`serde::Serialize`/`Deserialize` in `🧰️framework/🔨️modules/🌱️value/🦀️.rs:281,288`) was explicitly
out of scope — untouched, its "remove once no serde-deriving type holds a `DslValue`" docstring
still stands as the deliberate final step.

## A. `OrderedSet` — real consumer found, gated (not removed)

`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️.rs:66-75` already had `ToValue`/`FromValue`
(the `ArrayWire` region, unconditional — that's the real wire codec). The `serde::Serialize`/
`Deserialize` impls next to it were unconditional too. Traced every non-test-dir caller
(`grep -rn OrderedSet`, then narrowed to non-plugin, non-test files):

- `Widget::OutputPreview.expanded` and `FlowPreviewGui.expanded` in
  `💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs:153,209-213` — the only real forcing consumers. Both
  already route production traffic through `ToValue`/`FromValue` unconditionally; their
  `serde::Serialize`/`Deserialize` derive is **already** `#[cfg_attr(test, derive(Serialize,
  Deserialize))]` (os-flow's own docstring: "`serde` is TEST-ONLY — see `FlowArtifact`'s docstring
  above"). No consumer call site needed converting — they already used the right codec.
- Two `🌀️procedural` plugin files (`generation3d`/`generation2d` snapshot+mutation binary codecs)
  construct `flow::OrderedSet::new()` directly but never call serde on it — confirmed no impact,
  left untouched (off-limits per ticket rules anyway).

**Fix**: gated `OrderedSet`'s hand-written `Serialize`/`Deserialize` behind
`#[cfg(any(test, feature = "ordered-set-serde"))]` (🧺️set/🦀️.rs:74,87), mirroring the sibling
`OrderedMap<V>: Serialize` gate one file up (`🗂️ordered/🦀️.rs:433,441`) — **but with a correction**:
plain `#[cfg(test)]` alone is not sufficient here, and I verified why before shipping it. `Widget`/
`FlowPreviewGui`'s `#[cfg_attr(test, derive(Serialize))]` lives in a *different* crate
(`semio-framework-os-flow`). When `cargo test -p semio-framework-os-flow` compiles that crate with
`--cfg test`, its dependency `protocol` is still compiled as a plain (non-test) library — Rust's
`#[cfg(test)]` never crosses a crate boundary. A derive macro needs its field types' trait bounds to
exist at the derived impl's OWN compile time, regardless of whether the method is ever called, so
`os-flow`'s test build would hit "the trait bound `OrderedSet: Serialize` is not satisfied" even
though nothing in its tests calls `serde_json` on a `Widget` — the derive alone forces the bound.

Added a new `ordered-set-serde` feature to `protocol`'s `Cargo.toml` (empty, feature-only — `serde`
itself is still an unconditional dependency there because of blocker 1). `os-flow`'s `Cargo.toml`
enables it **only in `[dev-dependencies]`** (re-listing `semio-framework-replication` there with
`features = ["ordered-set-serde"]`, `[dependencies]` keeps the plain form) — Cargo's dev-dependency
feature unification activates the extra feature only for `os-flow`'s own `--tests`/`cargo test`
builds, never for a normal `cargo build`/`cargo check` of it or anything depending on it. This is
the standard idiom for a cross-crate test-only trait impl; I could not fully empirically verify it
end-to-end because `semio-framework-os-infinite` is currently broken by unrelated, in-flight
concurrent edits (`git status` shows ~17 modified files under
`💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/**`, 5 pre-existing `E0283` errors,
nothing to do with this change) which sits between `os-flow` and a clean `--tests` check. I did
verify the mechanism is sound from first principles (see above) and that `cargo check -p
semio-framework-replication` itself stays green with the gate in place.

**Consumer that forced it**: `os-flow`'s `Widget::OutputPreview`/`FlowPreviewGui` test-only serde
derive (not any production path).

## B. `InteractionState` — plugin.rs call sites converted

`💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`pub mod app`), `InteractionConfigMutation`'s `OpText`/`OpBinary`
impls (was lines 9857-9872, now ~9857-9876 after the edit) called `serde_json::to_string`/
`from_str`/`to_vec`/`from_slice` directly on `protocol::InteractionState`, even though
`InteractionState` already has hand-written `ToValue`/`FromValue`
(`📡️replication/📡️wire/🦀️.rs:1926,1936`, pre-existing, unrelated to this pass).

Converted all four call sites to `dsl::os_pack::json::to_json_string`/`from_json_str` — the
`ToValue`/`FromValue`-over-`DslValue` JSON bridge in `🎒️pack/🔤️json/🦀️.rs:1404-1410`
(`ToFromValueBridge` region; its own docstring already names this exact ticket as the reason it
exists — this file was clearly landed by a concurrent session on the same seam). `dsl::os_pack::json`
was already the path this same file uses two call sites down (`ManifestActionInvocation`/
`ManifestCommandInvocation` at what were lines 29878/29906), so no new import/dependency needed.
`encode_op`/`decode_op` route bytes through `.into_bytes()`/`std::str::from_utf8` since
`to_json_string`/`from_json_str` are string-based, not byte-based.

No wire-byte differential test added for this one: `to_json_string` walks `ToValue::to_value()` →
`DslValue` → the SAME `pack::json::to_string` renderer already proven byte-identical to
`serde_json` elsewhere in this ticket's other passes (not a fresh re-encoding I introduced), and
`InteractionState`'s `ToValue`/`FromValue` pair was pre-existing, not written by me.

**Did NOT touch**: `InteractionState`'s own `#[derive(..., serde::Serialize, serde::Deserialize)]`
in `📡️wire/🦀️.rs:1914` — out of my assigned scope (only the plugin.rs call sites were named), and
still has real consumers I must not edit: `🕹️interaction/📃️query`, `📡️live`, `🧬️mutations/🔁️set-state`,
`📖️capture`, `♻️retirement`, and `🏠️local-interaction` test files (all under `🧪️tests/` — owned by
another agent) call `serde_json::from_value::<InteractionState>(...)` directly.

## C. Attempted removing `serde`/`serde_json` from `protocol`'s `[dependencies]`

Commented out both lines, ran `cargo check -p semio-framework-replication --message-format short`,
captured **206 errors**, then restored the two lines and re-verified 0 errors (see VERIFY below —
tree was never left broken). Error breakdown by file:

| file | errors |
|---|---|
| `📡️replication/🎮️mutation/🦀️.rs` | 46 |
| `🌱️value/🦀️.rs` (blocker 1 — `DslValue`, expected/deliberate) | 38 |
| `📡️replication/📡️wire/🦀️.rs` (includes `InteractionState`'s own derive, line 1914) | 37 |
| `📡️replication/⚔️conflict/🦀️.rs` | 24 |
| `📡️replication/🆔️ids/🦀️.rs` | 23 |
| `⚠️diagnostic/🦀️.rs` | 23 |
| `📡️replication/🔗️causal/🦀️.rs` | 7 |
| `📡️replication/🧾️wire/🦀️.rs` | 6 |
| `⚠️diagnostic/📍️span/🦀️.rs` | 2 |

**This is NOT down to blocker 1 alone.** The Cargo.toml comment (`📦️packages/🦀️rust/Cargo.toml`
lines above the `serde` dependency) claims only 3 live blockers — `DslValue`, `OrderedSet`,
`InteractionState` — and is now confirmed STALE: `🎮️mutation/🦀️.rs`, `⚔️conflict/🦀️.rs`,
`🆔️ids/🦀️.rs`, `🔗️causal/🦀️.rs`, `⚠️diagnostic/🦀️.rs` (+ `📍️span`), and `🧾️wire/🦀️.rs` all carry
unconditional `#[serde(...)]`/`serde_json::` usage the comment never mentions. None of these were in
my assigned scope (A/B above only), so none were touched or investigated further — flagging them
here as the real remaining blocker set for whoever picks up the next pass, rather than claiming
false success.

## VERIFY (run 2026-09-02/03, `iso3` target dir)

- `cargo check -p semio-framework-replication` — **0 errors** (2 pre-existing warnings, unrelated).
- `cargo check -p semio-framework-os-kernel` — **0 errors**.
- `cargo check -p semio-framework` — **0 errors**.
- `cargo test -p semio-framework-actor --lib` — **121 passed, 0 failed**.
- `cargo test -p semio-framework-ui-scene` — **108 passed, 0 failed** (+ 0 doctests).
- `cargo metadata --no-deps --format-version 1` — exit 0.
- Could NOT get a clean `cargo check -p semio-framework-os-flow --tests` to directly exercise the
  `ordered-set-serde` feature end-to-end: blocked by `semio-framework-os-infinite`'s 5 pre-existing
  `E0283` errors from unrelated concurrent edits in `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/**`
  (confirmed via `git status`, not caused by this pass). Recommend whoever next touches `os-flow` or
  `os-infinite` re-run `cargo check -p semio-framework-os-flow --tests` once that settles, to close
  the loop on the `ordered-set-serde` feature.

## Files touched

- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️.rs` — gated `OrderedSet`'s `Serialize`/
  `Deserialize` behind `#[cfg(any(test, feature = "ordered-set-serde"))]`.
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` — added the
  `ordered-set-serde` feature (empty; `serde` itself stays unconditional, blocker 1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — enabled
  `ordered-set-serde` on `semio-framework-replication` in `[dev-dependencies]` only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `InteractionConfigMutation`'s
  `OpText`/`OpBinary` impls (`pub mod app`, ~line 9854-9876) now route through
  `dsl::os_pack::json::to_json_string`/`from_json_str` instead of `serde_json::` directly.
