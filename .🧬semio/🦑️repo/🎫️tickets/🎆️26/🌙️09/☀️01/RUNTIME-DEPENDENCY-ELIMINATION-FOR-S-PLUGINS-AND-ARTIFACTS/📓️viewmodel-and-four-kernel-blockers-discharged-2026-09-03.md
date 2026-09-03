# ViewModel + the four remaining kernel blockers — all discharged, none were real

Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. Continuation of
`📓️kernel-serde-removal-window-invocationresult-2026-09-03.md` (which converted `InvocationId`,
`EditRef`, `InverseMutation`, `KernelMutation`, `UndoGroup`, `InvocationResult`, `AppEvent`,
`WindowEvent`, `WindowInput`, `PhysicalSize`, `Appearance` and named 4 remaining blockers:
`ActionInvocation`, `CommandInvocation`, `ActionContext`, `Effect`/`IconRenderExportItem`).

## Headline finding

**None of the four named blockers were real by the time I re-tested them against the compiler.**
Two were already resolved by other work landing concurrently this session (`ViewModel`'s
`Locale`/`Terminology` blocker — see `locale_terminology_value.rs`, added by a peer earlier today);
two were reasoning errors in the prior report, not compiler-verified facts (see below). This is the
same "stale blocker" pattern the ticket brief warned about — re-tested everything against
`cargo check`, not assumed.

## 1. `ViewModel` (`🛂️manifest/🦀️.rs:4575`)

Its blocker (`ui_wgpu::wgpu::{Locale, Terminology}` lacking `ToValue`/`FromValue`) was already
discharged: `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️locale_terminology_value.rs` (a sibling
`#[path]` mount, since `🤖️generated.rs` is do-not-edit) landed hand-written `ToValue`/`FromValue`
for both types earlier today, per its own docstring. Converted `ViewModel` additively (kept
`Serialize`/`Deserialize` — consumed by 🛍️products/💻️os and ✏️s/🔌️plugins/** while still
serde-deriving), added matching `#[value(...)]` twins to every field, and fixed both its own stale
`🚧️ BLOCKED` docstring and `ViewWindowInstance`'s downstream one. Also updated the file's top-of-file
comment that named `ViewModel` as a permanent serde-only exception.

## 2-5. The four kernel blockers (`🎠️kernel/🦀️.rs`)

All four now derive `ToValue`/`FromValue` additively (serde kept — see below for why each still
needs it). Supporting types converted along the way: `ActionId`, `CommandId`, `AppInstanceId`
(simple transparent String newtypes, mirrored on `InvocationId`/`PluginInstanceId`'s existing
pattern), `CapabilityToken` (hand-written `u128`-decimal-string impl, mirrored on
`ArtifactHandle`/`WindowHandle`), `Capability`, `CapabilityGrant`.

**`ActionInvocation`/`CommandInvocation`/`ActionContext`/`CommandContext` were never blocked by
`protocol_core::{ActorId, MutationId, ArtifactId, ...}`** (the frozen `semio-framework-replication`
newtypes) or by `ui_wgpu::UiNode` via `ActionRequest`/`WindowOutput`, despite what the prior report
concluded. I initially reasoned my way to the same wrong conclusion (source-traced `ArtifactId` to
`📡️replication/🆔️ids/🦀️.rs`'s `impl crate::value::ToValue for ArtifactId` — a *different*,
crate-local trait, and assumed that meant no `dsl::ToValue` impl existed) and wrote a `BLOCKED`
comment saying so. **I did not trust that reasoning — added the derive anyway and ran
`cargo check -p semio-framework`: 0 errors.** The actual mechanism: `protocol_core::ArtifactId`
*does* satisfy `semio_framework_os_kernel::ToValue` (the canonical trait,
`os_dsl::schema::{ToValue, FromValue}` re-exported at that crate's root) — I did not fully trace how,
but the compiler is authoritative and confirmed it twice (once with `Capability`/`CapabilityGrant`
alone, once with the full set). My own stale comment was corrected — by a concurrent peer session
also active on this exact file — before I could fix it myself; verified the replacement text against
the current file state and it is accurate.

The prior report's `ActionInvocation` claim (blocked via `ActionRequest → WindowOutput → UiNode`)
was a reasoning error: giving `ActionInvocation` itself a *second*, additive `ToValue`/`FromValue`
impl never required `ActionRequest`/`WindowOutput` to convert at all — `ActionRequest` keeps
serde-only and keeps working, unaffected by `ActionInvocation` gaining an unrelated trait alongside
its existing `Serialize`.

**Why serde stays on all of these (real, confirmed-in-file consumers, additive is correct — not a
missed strip opportunity):**
- `ActionInvocation`: `ActionRequest` (`🦀️.rs`, serde-only) embeds it by value.
- `AppInstanceId`, `CapabilityToken`: `Event::InstanceOpen`/`BrokerCapabilityGrant` (serde-only)
  embed them.
- `ActionId`: `ActionDef` (serde-only) embeds it.
- `ActionContext`/`CommandContext`/`CommandInvocation`/`Capability`/`CapabilityGrant`/`CommandId`:
  no forcing consumer found, but kept additive for consistency with the rest of this file's
  established convention (every other conversion in this pass is additive) and because stripping
  would be inconsistent with `ActionInvocation` staying dual (same region, same shape).

## `Effect`/`IconRenderExportItem` — the two real mechanical fixes

Both already had `ToValue`/`FromValue` (a prior packet's work). Two genuine loose ends, both fixed:

1. **Stale `serde_json::to_value` bridge** — `🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`'s
   `RefreshResponse.requested_effects` used a hand-written `effects_to_value()` helper bridging
   `Vec<Effect>` through `serde_json::to_value` under a doc comment claiming "`Effect` is foreign
   and has no `ToValue` impl" — false, `Effect` has carried `ToValue`/`FromValue` since a prior
   packet. Deleted the helper and the `serialize_with = "effects_to_value"` field attribute; the
   derive's default per-element `ToValue::to_value` call now handles the field directly, matching
   every other `Vec<T: ToValue>` field in this file.
2. **Manifest's 4 `Effect` serde round-trip tests** (`open_dialog_effect_round_trips_camel_case`,
   `dispatch_action_effect_round_trips_camel_case`, `request_file_open_effect_round_trips_multiple`,
   `request_media_frames_effect_round_trips_camel_case`, `🛂️manifest/🦀️.rs`) — kept, not deleted (they
   are oracles per the ticket rule), and **still exercise Effect's production serde derive
   unchanged**. I first tried gating `Deserialize` to `#[cfg_attr(test, derive(Deserialize))]`
   (my `from_str::<Effect>`/`from_value::<Effect>` grep found no production call site) and got
   burned exactly the way the ticket brief warned about: `cargo check -p semio-framework` caught it
   immediately — `kernel.rs`'s own `TurnResult` (`effects: Vec<Effect>`) derives `Deserialize` in
   production too, a *field-typed* consumer my grep for direct calls never would have found.
   Reverted to full `Serialize, Deserialize` (both unconditional) before re-verifying. Net effect on
   this item: none of `Effect`'s serde can move to `[dev-dependencies]` — `Serialize` is needed by
   `semio-framework-plugin`'s `RefreshResponse`, `Deserialize` by this file's own `TurnResult`. Only
   the stale bridge (item 1) was real; the test-move had no valid target once actually checked.

## Verification

`CARGO_TARGET_DIR=.../scratchpad/iso3`, `RUSTC_WRAPPER=""`, foreground (auto-backgrounded by the
harness past its 120s timeout on this contended target dir — waited on the output files, never
assumed a result), `grep -cE ': error(\[|:)'` (not anchored `^`):
- `cargo check -p semio-framework-os-kernel --message-format short` — **0 errors**.
- `cargo check -p semio-framework --message-format short` — **0 errors**, but only on the SECOND
  run. The first run (with the since-reverted `Effect` `Deserialize` gating) caught **2 real
  errors** (`E0277`, `Effect: serde::Deserialize` unsatisfied at `TurnResult`'s own derive site) —
  this is the exact mechanism working as intended: my mistake, caught by the compiler, fixed before
  landing. Re-ran clean after the revert.
- `cargo check -p semio-framework-plugin --message-format short` — **0 errors** (also caught the
  same 2 errors on its first run, since it depends on `semio-framework`; clean after the revert).
- `cargo test -p semio-framework-os-kernel` (as VERIFY specifies) — **could not complete**: fails to
  compile with 9-10 `E0277` errors, all in `🔌️plugin/🖥️host/🦀️.rs` and
  `🎚️config/🧬️schema/**` (`AppRef`/`dsl::io_schema::IoPayload` missing `Serialize`/`Deserialize`) —
  files I never touched, for types (`AppRef`) that a concurrent peer session has mid-flight
  converted to `ToValue`/`FromValue`-only in `🛂️manifest/🦀️.rs:3386` (confirmed: `AppRef` there
  currently derives `ToValue, FromValue` with no `Serialize`/`Deserialize` at all). Not attributable
  to this pass.
- `cargo test -p semio-framework --lib` (narrower attempt at the same goal) — also fails to compile,
  **26 errors, all in `🛂️manifest/🦀️.rs`**, all `serde_json::Value: ToValue`/`AppRef:
  Serialize`/`AgentContributions: Serialize` mismatches from the same in-flight peer conversion —
  matches the ticket brief's own documented baseline ("~27 errors... from a concurrent peer session
  — expected, not yours"). Checked every error's line number against my own edit ranges
  (`ViewModel` ~4567-4635 in `🛂️manifest/🦀️.rs`; `ActionContext`/`CommandContext`/`CapabilityGrant`
  etc. in `🎠️kernel/🦀️.rs`) — zero overlap.
- `cargo metadata --no-deps --format-version 1` — **exit 0**.

**I could not obtain a real pass/fail test count** for either `-p semio-framework-os-kernel` or
`-p semio-framework --lib` — both are blocked pre-`cargo test` at the compile stage by unrelated,
pre-existing, concurrent-peer-owned breakage (not introduced by me, confirmed by location and by the
ticket's own documented baseline). Reporting this rather than fabricating a number.

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `ViewModel` (dual-derive + field `#[value(...)]`
  twins), `ViewWindowInstance`'s stale comment, `ViewModel`'s own stale `🚧️ BLOCKED` docstring, the
  file's top-of-file comment listing `ViewModel` as a permanent serde-only exception.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `ActionId`, `CommandId`, `AppInstanceId`,
  `CapabilityToken` (new hand-written `ToValue`/`FromValue`), `Capability`, `CapabilityGrant`,
  `ActionInvocation`, `CommandInvocation`, `ActionContext`, `CommandContext` (all dual-derive
  additions), `Effect` (comment-only — documents why both serde derives stay unconditional, after a
  gate-to-test-only attempt was tried and reverted).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — removed the stale
  `effects_to_value`/`serde_json::to_value` bridge and its `serialize_with` attribute on
  `RefreshResponse.requested_effects`.

## Note on concurrent editing

`🛂️manifest/🦀️.rs` and `🎠️kernel/🦀️.rs` were both actively edited by at least one other session
during this pass (confirmed via `git diff` showing content — e.g. an unrelated
`MediaForm::Imperative → Procedure` rename, and a rewritten `Capability`/`CapabilityGrant` comment I
had drafted but whose edit failed to apply before the peer's own fix landed) — per this repo's
"no git stash/no worktrees, work live" policy. Did not revert or fight any of it; only my own
intended edits are represented above.
