# Fixing the 13-error coordination break: ArtifactDialect / InteractionRef (2026-09-02, session N)

## Baseline (real, measured)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework --message-format short | grep -cE ': error(\[|:)'
```
→ **13**, matching the ticket exactly: 11 × `ArtifactDialect` (`Serialize`/`Deserialize` not
satisfied, E0277) + 2 × `interaction::component::InteractionRef` (same). Anchored `^error` grep
undercounts on this codebase's multi-line rustc output — used the unanchored form throughout, per
instruction.

## Root cause, read from the actual E0277s, not assumed

`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs` had `Serialize, Deserialize` stripped from
`ArtifactDialect` (kept `ToValue, FromValue`). `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs` had the
same done to `InteractionRef`. Both diffs carried a doc-comment claiming the manifest-side
consumers had "migrated off serde" / "no longer blocks this" — i.e. Option A already done.

That claim is **false for the current tree**, verified by reading `🛂️manifest/🦀️.rs` directly, not
by trusting the comment:

- `AppDefinition` (~line 3423) and `WindowKindDefinition` (~line 3195) are **still**
  `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]` only — no `ToValue`/`FromValue` —
  each carrying its own `🚧️ BLOCKED` comment naming the real, unrelated blocker: `ui_wgpu`'s
  `LocalizedLabel`/`IconName`/`SurfaceKind`/`WindowOptions` (`WindowKindDefinition`) and the same
  plus `InteractionDefinition`/`ActivationEvent`/`CapabilityRequirement`
  (`AppDefinition`) — none owned by this ticket, `IconName`'s home file marked "do not edit".
  `AppDefinition.dialect: ArtifactDialect` and `WindowKindDefinition.interactions:
  Vec<InteractionRef>` therefore need the serde half regardless of what `ArtifactDialect`/
  `InteractionRef` themselves derive.
- `IoEntryDescriptor`/`ComposerEntryDescriptor` (manifest's own, ~line 4796/4810) are dual-derived
  (`Serialize, Deserialize, ToValue, FromValue`) and carry `owner`/`counterpart`/`writes`/`reads:
  ArtifactDialect` — their own in-source comment says why they stay dual: "referenced (directly or
  transitively) by a BLOCKED serde-only manifest type" (i.e. `AppDefinition` et al above).

So **Option A is not available** for either type today — the manifest-side blocker is real,
independently confirmed by an overlapping peer session (see below), not merely inconvenient.

## Fix applied — Option B (additive dual-derive) for both types

- `🚪️io/🧬️schema/🦀️.rs`: restored `use serde::{Deserialize, Serialize};` and
  `Serialize, Deserialize` + `#[serde(rename_all = "camelCase")]` on `ArtifactDialect`, alongside
  the existing `ToValue, FromValue` / `#[value(rename_all = "camelCase")]`. Replaced the false
  "unblocked" doc comment with a `🚧️ BLOCKED` comment naming `AppDefinition`/`WindowKindDefinition`
  (direct, serde-only) and `IoEntryDescriptor`/`ComposerEntryDescriptor` (dual-derived, transitively
  blocked) as the exact reason serde must stay.
- `🕹️interaction/🦀️.rs`: restored `Serialize, Deserialize` + `#[serde(transparent)]` on
  `InteractionRef`, alongside `ToValue, FromValue` / `#[value(transparent)]` (matches its pre-strip
  form exactly). Added a `🚧️ BLOCKED` comment naming `WindowKindDefinition.interactions` as the
  reason. Also corrected the adjacent, now-proven-false `InteractionDefinition` comment (it claimed
  "`🛂️manifest` itself no longer blocks this" — verified false by reading the file) so it doesn't
  mislead the next pass the way it misled this one.
- Neither type had a hand-written `Serialize`/`Deserialize`/`ToValue`/`FromValue` impl anywhere in
  the tree (checked) — no E0119 risk from the dual-derive.
- No `Cargo.toml` touched; `serde` was already a real dependency of both crates (the import line
  existed before the strip).

## Peer-churn cross-check

A peer session's own note, `📓️manifest-serde-to-value-conversion-2026-09-02.md` (same ticket
folder, saved ~5 min before this fix, effectively concurrent), independently reached the identical
conclusion from the manifest side: baseline 13, "all 13 attributable to peer churn, 0 attributable
to me", and "Both are one-line reverts (add `Serialize, Deserialize` back to the derive + matching
`#[serde(...)]`) for whoever owns those files". A third, now-stale note in the same folder
(`📓️manifest-io-interaction-final-13-2026-09-02.md`) had claimed the opposite (blocker gone,
serde safely removable) — that note's premise no longer matches the tree; do not trust it without
re-verifying by compilation, which is exactly what caused this break in the first place.

## Verify — real before/after

```
cargo check -p semio-framework --message-format short | grep -cE ': error(\[|:)'
```
- Before this fix: **13** (0 attributable to me at that point — I hadn't touched anything yet;
  100% the two-type coordination break described in the ticket).
- After this fix: **0**. `semio-framework` compiles clean.

```
cargo check -p semio-framework-plugin --message-format short | grep -cE ': error(\[|:)'
```
- **5**, all pre-existing peer churn, unrelated to `ArtifactDialect`/`InteractionRef` and **not
  caused by this fix** (the fix only adds derives, never removes one — it cannot introduce new
  errors elsewhere): `semio_framework::manifest::ActionInvocation` (Deserialize),
  `semio_framework::manifest::CommandInvocation` (Deserialize), `semio_framework::MediaFingerprint`
  (Serialize) — all three intentionally serde-free per their own doc comments in this same ongoing
  DslValue-conversion wave (`MediaFingerprint`'s tuple-struct impls are hand-written, no
  `Serialize`/`Deserialize` by design) — and `dsl::io_schema::IoPayload` (Deserialize, 2 sites in
  `⚛️reactor/💼️jobs/🦀️.rs`), which lost serde in the same original strip but was out of this
  ticket's named scope (`ArtifactDialect`/`InteractionRef` only) and is already flagged fixable by
  the peer note above (5 test call sites in `🖥️host/🦀️.rs`, not touched here). No regression: this
  crate could not have compiled any cleaner before my edit, since my edit is strictly additive.

## Files touched

- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs`

No `Cargo.toml` edited. No oracle/test/fixture files touched. No git history commands run.
