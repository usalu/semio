# manifest / io / interaction — closing the last 13 (2026-09-02, session 2)

Scope: the 3 named modules only, all under `🧰️framework/🔨️modules/`: `🛂️manifest`, `🚪️io`
(+ `🧬️schema`), `🕹️interaction` (+ `🧬️schema`). Did not touch `action-bus::optional_json_to_dsl`.

## Baseline re-test of the two stale `🚧️ BLOCKED` claims

Both were re-tested by actually compiling, per instruction, not by trusting the in-source comments.

1. **`io::ArtifactDialect`** — the old comment said manifest's `AppDefinition.dialect`/
   `IoEntryDescriptor.owner`/`counterpart`/`ComposerEntryDescriptor.writes`/`reads` still derived
   plain unconditional `Serialize, Deserialize`. Verified: all three now derive `ToValue, FromValue`
   only. Blocker gone. Dropped `Serialize, Deserialize` + `#[serde(rename_all = "camelCase")]`,
   deleted the stale comment, replaced with an accurate one.

2. **`io_schema::IoPayload`** — the old comment said `semio-framework-plugin`'s
   `🔌️plugin/🖥️host/🦀️.rs` test module calls `serde_json::to_vec(&IoPayload::…)`/
   `serde_json::from_slice::<IoPayload>` at 5 call sites (~8306-8414), and that `#[cfg_attr(test,
   …)]` can't fix it because `cfg(test)` never activates for a normal (non-dev) dependency's own
   test compilation. Confirmed this reasoning is correct (textbook per-crate `cfg(test)`
   semantics). Fix: converted those 5 test call sites in `🖥️host/🦀️.rs` to
   `dsl::os_pack::json::to_json_string(...).into_bytes()` / `dsl::os_pack::json::from_json_str(...)`
   — the exact pattern the SAME file's production `io_run`/`io_sniff` methods already use two
   fields above. This is outside my 3-module scope but a small, mechanical, low-risk fix (5 call
   sites, one file), not the kind of wide cross-crate atomic wave `optional_json_to_dsl` is.
   Blocker fully resolved: dropped `Serialize, Deserialize` from `IoPayload`, deleted the comment,
   removed the now-unused `use serde::{Deserialize, Serialize};` import from `io_schema/🦀️.rs`
   (nothing else in that file used it).

## `🛂️manifest` — 2 refs

- `pub payload: Option<serde_json::Value>` on `DescriptorEntry` → `Option<DslValue>`. Verified
  nothing in the repo constructs a `DescriptorEntry` yet (doc comment says so; grep confirms) — a
  pure retype, no call-site fallout.
- `use serde::{Deserialize, Serialize};` — NOT removable. A concurrent peer session is actively
  converting this same file this session; at last read it carries an explanatory comment saying
  `ViewModel` and the `MediaVocabulary` family still derive plain `Serialize, Deserialize`
  unconditionally because sibling `#[path]`-mounted modules (`🎠️kernel`, `🔁️workflow`, both owned
  by other agents) embed them by value in plain-serde types. Out of my scope; left alone.

## `🚪️io` — 4 refs — ALL RESOLVED, 0 serde left in either file

`io/🦀️.rs` and `io/🧬️schema/🦀️.rs` now contain zero `serde`/`Serialize`/`Deserialize` outside the
crate's own `Serializer`/`Deserializer` trait names (unrelated custom traits, not serde).

## `🕹️interaction` — 7 refs — 2 resolved, 5 still genuinely blocked (new reason, not the old one)

- `InteractionRef(String)` — no blocking dependency (just wraps a `String`). Converted:
  `#[derive(..., Serialize, Deserialize)] #[serde(transparent)]` →
  `#[derive(..., ToValue, FromValue)] #[value(transparent)]`.
- `InteractionDefinition`/`GranularityDefinition` — genuinely blocked, confirmed by compiling with
  `ToValue, FromValue` added: both embed `label: LocalizedLabel` (`GranularityDefinition` also
  `icon_id: IconName`), and neither `LocalizedLabel` (`🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/
  🦀️label.rs`) nor `IconName` (`🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`, re-exported via
  `ui_wgpu`) implement `ToValue`/`FromValue` yet — only `Serialize, Deserialize`. Compile error:
  `E0277: the trait bound 'LocalizedLabel: ToValue' is not satisfied`. Both types are outside my 3
  modules (`🖱️ui`, `🖼️assets`) and are large separate in-flight conversions per the ticket's own
  scoreboard (`🖱️ui` alone carries ~900 refs). Updated the in-source `🚧️ BLOCKED` comments to name
  the correct current blocker (was stale, blamed `🛂️manifest`, which no longer applies) instead of
  deleting them, since the block is real.
- Added `use dsl::{FromValue, ToValue};` to `interaction/🦀️.rs` (same crate-wide `dsl` alias
  `🛂️manifest/🦀️.rs` uses) for `InteractionRef`'s new derive.

## Compile verification

`cargo check -p semio-framework --message-format short`, `CARGO_TARGET_DIR` = the shared iso3 dir,
`RUSTC_WRAPPER=""`:
- Before any edit this session: not re-measured (session inherited a red tree from concurrent
  manifest work — see below).
- After every edit, filtered the error list by owning module: 100% of errors are in `🛂️manifest`
  and `🔁️workflow` (peer-owned, unrelated types: `LocalizedLabel`, `IconName`, `WindowLayout`,
  `MediaType`, `ActivationEvent`, `ViewModel`, etc.), confirmed by grepping the error log for
  `ArtifactDialect`, `IoPayload`, `InteractionRef`, `io_schema`, `🚪️io`, `🕹️interaction`, `🖥️host`
  — zero hits at every check. Error count fell across rechecks purely from peer progress (143 →
  80), never rose from my edits.
- Also checked `cargo check -p semio-framework-plugin --tests` (the crate I touched outside my 3
  modules) — same result: all errors trace to the same peer-owned manifest churn; zero mention my
  edited symbols or `🖥️host/🦀️.rs`'s lines.
- Neither crate is fully green right now — both are blocked on the concurrent manifest/workflow
  wave, unrelated to this work.

## Payoff step — NOT attempted

`cargo tree -i serde` on `semio-s-plugin-draw`'s wasm32-wasip2 graph still shows
`serde`/`serde_core`/`serde_derive`/`serde_json` linked via `semio-framework` itself. Root cause:
`🛂️manifest`'s own remaining serde-only types (`ViewModel`, `MediaVocabulary` family,
`ActionDescriptor`, plus everything blocked on `🖱️ui`'s `LocalizedLabel`/`IconName`/`WindowLayout`/
`SurfaceKind`/`NamedLayout`/`WindowOptions`) and `🎠️kernel`/`🔁️workflow`'s `MediaType`/`MediaForm`/
`MediaWireFormat`/`MediaPortSpec` — none of which are in my 3-module scope, none touched by
`optional_json_to_dsl` either. Per the standing rule, did not clear `semio-framework`'s
`Cargo.toml` serde lines — the crate still genuinely needs them for code outside my 13 refs.

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (1 field retype only)
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs` (5 test call sites only, to unblock
  `IoPayload`)

No `Cargo.toml` edited anywhere.
