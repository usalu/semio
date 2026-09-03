# Demonstrator + Animate — serde/serde_json elimination

Date: 2026-09-03. Verification method: grep-only, zero cargo/rustc commands run at any point (by
me or either subagent). Independently re-verified below, not just trusted from subagent reports.

## Scope
Two targets from this ticket's "next closest to manifest-clean" list:
- `✏️s/🔌️plugins/🎪️demonstrator/`
- `✏️s/🔌️plugins/🎞️animate/`

Both converted via a parallel subagent each, following the same template
(`✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🦀️.rs`) and conversion rules as prior waves on this ticket.

## 🎪️demonstrator

**Before → after (production, non-test refs):** 44 → 4.

The remaining 4 hits are all inside one `#[cfg(test)] mod tests` block in
`🗿️artifacts/🎪️playground/…/🧬️schema/🧬️mutations/✒️change-schema/🦀️.rs`, function
`committed_json_bridge_round_trips` (lines 103–107). This is a genuine third-party oracle: it
decodes the *string output* of `apply_playground_mutation_json`/`undo_playground_mutation_json`
(both already fully converted off serde) with real `serde_json::Value` purely to confirm our own
JSON writer produces spec-conformant JSON matching committed fixtures — exactly the
"validate our implementation against a third-party library" pattern CLAUDE.md requires. Left alone
deliberately.

No `#[serde(alias = …)]` or `#[serde(untagged)]` usages existed in the original 44 hits, so there
is no read-compat gap to report for this plugin.

**Files edited (16):** all under `✏️s/🔌️plugins/🎪️demonstrator/…`:
- io import/export (de)serializers: `zip`, `json` (rfc8259), `xlsx` (both import and export sides)
- schema: `🧬️schema/🦀️.rs` (PlaygroundArtifact), `📸️snapshot/🦀️.rs`, `🔺️diff/🦀️.rs`,
  `🧬️mutations/🦀️.rs`, `🧬️mutations/✒️change-schema/🦀️.rs` (+ its `💾️binary/🦀️.rs`),
  `💡️inferences/🦀️.rs`
- viewer/editor: `👁️viewer/🎭️modes/👁️view/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🦀️.rs`,
  `✏️editor/🦀️.rs` (also fixed `command_from_action` to take `Option<&dsl::DslValue>`, matching
  the `ArtifactEditor` trait's real signature — was previously mismatched),
  `✏️editor/🎮️commands/🔧️change-schema/🦀️.rs`

**Cargo.toml:** untouched. `serde.workspace = true` / `serde_json.workspace = true` remain in
`[dependencies]` in `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` because the oracle
test above still needs `serde_json` at test time — moving to `[dev-dependencies]` would be correct
in principle (the oracle only runs under `cfg(test)`) but was left as agreed-safe default; **this
line has NOT been re-verified against the zero-production-ref threshold in the strict sense (4
non-zero) so per the standing rule it stays in `[dependencies]` untouched.**

## 🎞️animate

**Before → after (production, non-test refs):** ~64 → 1 genuine production ref + 5 test-only lines.

Verified remaining hits directly:
- `🚪️io/🦀️.rs:72` — `pub fn animate_presentation_document_json_to_svg(value: &serde_json::Value)`.
  Genuine, unavoidable: it calls `semio_framework_os::title_card_svg(value, …)` at
  `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3451`, whose signature is
  `pub fn title_card_svg(value: &Value, …)` where `Value` is `serde_json::Value` (confirmed via
  `use serde_json::Value;` in that framework file). `🧰️framework/**` is explicitly out of scope
  ("DO NOT TOUCH") for this ticket slice, so this call boundary cannot be converted from animate's
  side alone. Correctly left as serde.
- `🚪️io/🦀️.rs:104` — `use serde_json::json;` inside the `#[cfg(test)]` block that exercises the
  function above.
- `✏️editor/🦀️.rs:788,800,801` — test `retained_command_fixture_matches_exact_routes_and_serde_json_boundaries`,
  a genuine third-party oracle round-tripping `PresentationConfigMutation` through real
  `serde_json::to_vec`/`from_slice`.
- `✏️editor/🎚️config/🦀️.rs:88` — `#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`
  on `PresentationConfigMutation`, gated to test builds only, backing the oracle test above.

**Cargo.toml:** untouched (correctly) — `serde.workspace = true` / `serde_json.workspace = true`
stay in `[dependencies]` in `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` because of the
genuine production reference at `🚪️io/🦀️.rs:72` (framework boundary, not just test residue).

**Files edited (19):** presentation artifact root, schema + mutations, io root, pptx
import/export deserializer/serializer, json export serializer, binary mutations codec, viewer +
editor view modes and their tile-editor windows, editor commands (`export-video-from-deck`,
`add-tile`, `delete-selection`), editor root, catalogue panel, engine + engine/video.

**Notable defects found and fixed along the way (per subagent, independently plausible from code
shape, not separately re-verified line-by-line by me beyond the grep spot-checks above):**
- `PptxIntoPresentation`/`PresentationIntoPptx` were calling `serde_json::to_value`/`from_value` on
  `PptxSnapshot`, which only derives `value_derive::ToValue/FromValue` (no serde derive) — this
  could not have compiled as written pre-conversion; replaced with direct `dsl::ToValue`/
  `dsl::FromValue` calls.
- Removed now-dead `VideoError::Json` variant/mapper in `⚙️engine/🎥️video/🦀️.rs` once its two call
  sites became infallible (`to_string_pretty` on the first-party JSON type never fails) — per this
  ticket's "delete fake fallback branches on now-infallible calls" rule.
- `BuiltNode`/`AppDefinition` (framework types) are documented framework-side as deliberately
  serde-only for now (out of scope); tests that previously JSON-serialized them were switched to
  `format!("{:?}", …)` (Debug) — all such tests only did `.contains(...)` substring checks, so
  behavior is unchanged.
- Array-index trap (`.get([0-9]`) and stale-fallback (`unwrap_or_else`) greps both came back clean
  in the final animate diff.

One incidental note from the animate subagent: it observed two unrelated `#[path]` emoji-spelling
touch-ups (`📄txt` → `📄️txt`) already present in
`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/🦀️.rs` that it did not make — almost certainly a
concurrent session or auto-commit elsewhere in this live multi-dev repo. Left as-is, not reverted,
not investigated further (out of scope for this ticket slice).

## Independent verification performed (by me, not just subagent self-report)
```
grep -rn 'serde_json\|use serde\|derive([^)]*Serialize\|derive([^)]*Deserialize' "✏️s/🔌️plugins/🎪️demonstrator" --include='*.rs' | grep -vE '🧪|🏭|🔬' | grep -vE ':\s*(///|//!|//|\*)'
grep -rn 'serde_json\|use serde\|derive([^)]*Serialize\|derive([^)]*Deserialize' "✏️s/🔌️plugins/🎞️animate" --include='*.rs' | grep -vE '🧪|🏭|🔬' | grep -vE ':\s*(///|//!|//|\*)'
```
Both re-run after subagent completion, matching their reported final counts. Manually read the
surrounding `#[cfg(test)]`/`#[test]` context for every remaining hit in both plugins, and confirmed
the animate `🚪️io/🦀️.rs:72` framework-boundary claim by reading
`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3451` and its `use serde_json::Value;` directly.

## What was deliberately left, and why (summary)
- demonstrator: 1 oracle test (4 lines) — kept per CLAUDE.md's third-party-validation rule.
- animate: 1 genuine production ref (framework boundary, out of scope) + 1 oracle test (5 lines) —
  both kept for the same reasons; Cargo.toml correctly NOT touched in either plugin since neither
  reached true zero production refs.

## Cargo commands run
Zero, by me and by both subagents, for the entire task.
