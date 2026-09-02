# 🌱️ Manifest/interaction: unwinding the 34+3 `🚧️ BLOCKED` types now that the ui_wgpu seven are unblocked

Owner: single session for both `🛂️manifest/🦀️.rs` and `🕹️interaction/🦀️.rs` (interdependent, per the
prior 13-error incident from splitting them across two agents).

## Starting state

Both files already carried a large uncommitted WIP from an earlier pass (stale — written before the
ui_wgpu seven landed, confirmed via its own comments claiming `LocalizedLabel` still lacked
`ToValue`/`FromValue`). That WIP had already correctly converted `Platform`/`ActionKind`/`ArgFormat`/
`ArgPresentation`/`MediaClass`/`MediaForm`/etc. (referenced-but-not-directly-blocked transitive types)
to dual (`Serialize, Deserialize, ToValue, FromValue`) and `InteractionRef` to dual+transparent.
Baseline `cargo check -p semio-framework --message-format short`: **0 errors** (confirmed by an
actual compiler run before touching anything).

## What was done

Converted the 34 manifest types + 3 interaction types named `🚧️ BLOCKED` (on the now-resolved
ui_wgpu seven: `LocalizedLabel`/`IconName`/`ActionDescriptor`/`NamedLayout`/`WindowLayout`/
`WindowOptions`/`SurfaceKind`) to `#[derive(..., ToValue, FromValue)]` + `#[value(...)]` twins of
every `#[serde(...)]` attribute (container and field-level), via an automated script (hand-editing
6977 lines was infeasible) followed by manual cleanup of a comment-duplication bug the script
introduced on its first pass (fixed with two corrective passes, verified line-by-line against
`git diff`).

Per the ordering rule (types consumed outside these two files by `🛍️products/💻️os`
plugin/renderer/shell modules or any `✏️s/🔌️plugins/**` must keep serde additively), grepped real
external consumers for every target type before deciding strip-vs-keep. **Every single one of the 34
+ 3 has a real external consumer** (`🔌️plugin/🦀️.rs`, `📺️renderer/…/Shell`, and/or dozens of
`✏️s/🔌️plugins/**` editor files) — either directly, or transitively via `AppDefinition`/
`WindowKindDefinition`/`ModeDefinition`/`PluginManifest` which are consumed by hundreds of files. **No
type in this pass had serde fully strippable.** Net result: `Serialize`/`Deserialize`/`#[serde(...)]`
ref count is unchanged (additive-only), `ToValue`/`FromValue`/`#[value(...)]` grew substantially.

Also fixed a stale/false doc comment on `ResourceSelector` (not in the original 34-list, discovered in
passing) claiming `#[value(...)]` "has no tuple-struct support at all" — disproved by the
already-dual `ActionRef`/`UtilityRef`/`ToolRef` tuple structs sitting right above it in the same file.
Initially added a derive, which conflicted (E0119) with `ResourceSelector`'s existing hand-written
`ToValue`/`FromValue` impls — reverted the derive, corrected the comment to explain the REAL reason
(hand-written impl exists, not a tuple-struct limitation), left the type otherwise untouched.

## 7 of the 34 could NOT be unblocked this pass — reverted back to serde-only after real compiler failures

Adding `ToValue`/`FromValue` to all 34 first, then compiling, surfaced 5 genuinely different residual
blockers (none are the ui_wgpu seven — all out of scope for this pass):

| Type | Blocked on | Owner |
|---|---|---|
| `UtilityDefinition` | `ui_wgpu::wgpu::UtilityCategory` (not one of the seven) | 🖱️ui |
| `ViewModel` | `ui_wgpu::wgpu::{Locale, Terminology}` (not one of the seven) | 🖱️ui |
| `PluginManifest` | `kernel::CapabilityRequirement` | 🎠️kernel |
| `WindowKindDefinition` | `kernel::CapabilityRequirement` | 🎠️kernel |
| `ExtensionPointDeclaration` | `kernel::ActivationEvent` | 🎠️kernel |
| `PackageDescriptor` | `kernel::ActivationEvent` (transitively via `ExtensionPointDeclaration`/`ContributionSet`) | 🎠️kernel |
| `AppDefinition` | transitively embeds `WindowKindDefinition`/`UtilityDefinition` above | (both above) |

These 7 were reverted to `#[derive(Serialize, Deserialize)]` only, with fresh `🚧️ BLOCKED` docstrings
naming the exact real blocker (not the stale generic "seven ui_wgpu types" text). `InteractionDefinition`/
`GranularityDefinition` (interaction.rs) unblocked cleanly with no residual issue.

## Verification

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework --message-format short
```
Final result: **0 errors**, 11 warnings (all pre-existing, unrelated to this change). Confirmed by
real compiler run, not assumed.

## Ref counts (real, comment/`#[cfg(test)] mod`-stripped, via a Python script)

| File | Serialize+Deserialize+`#[serde(...)]` before this session | after | ToValue+FromValue+`#[value(...)]` before | after |
|---|---|---|---|---|
| `🛂️manifest/🦀️.rs` | 542 (session start, incl. prior WIP) / 581 (repo HEAD) | 542 (unchanged — additive only) | ~unknown (prior WIP partial) | 123+119+263 = 505 |
| `🕹️interaction/🦀️.rs` | 11 | 11 (unchanged) | 0 | 4+4+3 = 11 |

The manifest serde ref count differs between "session start" (542) and "repo HEAD" (581) because a
large concurrent agent fleet is actively editing this ticket's other files; HEAD moved during this
session via the repo's auto-commit process (confirmed via `git log --date=iso`, not by trusting
commit message text — see memory note on fake auto-commit dates). My own delta is exactly additive:
0 serde refs removed, 0 serde refs added, only `ToValue`/`FromValue` grew.

## Types left additive (kept BOTH derive families) with their external consumer

All 27 successfully-unblocked manifest types + `InteractionDefinition`/`GranularityDefinition`/
`InteractionRef` (interaction.rs) are kept additive — external consumer for every one is
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (the OS plugin loader) and/or dozens of
`✏️s/🔌️plugins/**/✏️editor/🦀️.rs` files (individually verified via `grep -rl` per type name, not
assumed):

`Keybinding`, `ArgSchema`, `ActionArgOption`, `ActionArgControl`, `ActionArgDef`, `ActionSemantics`,
`ActionDefinition`, `CommandDefinition`, `OsDefinition`, `ToolDefinition`, `IntroductionDefinition`,
`IntroductionStepDefinition`, `TutorialDefinition`, `TutorialChapter`, `TutorialBase`,
`TutorialTracks`, `TutorialNarrationCue`, `TutorialCaption`, `TutorialUiKeyframe`, `TutorialUiSample`,
`TutorialUiSnapshot`, `TutorialUiChange`, `DialogDefinition`, `ModeDefinition`, `PanelTabDefinition`,
`ExampleDefinition`, `ContributionSet`, `InteractionDefinition`, `GranularityDefinition`,
`InteractionRef`.

Plus the pre-existing (peer WIP, unchanged by me) additive set: `Platform`, `PlatformKeybinding`,
`ActionKind`, `ArgPresentation`, `ActionRef`, `ToolRef`, `UtilityRef`, `MediaClass`, `MediaForm`, and
~15 more transitive dependents — all documented in-place with "🚧️ Needed in serde form too" comments.

## Not touched (explicitly out of scope)

`🎠️kernel/🦀️.rs`, `🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs`, `🖱️ui` (including
`ui_wgpu::wgpu::{Locale, Terminology, UtilityCategory}`), `🚪️io/🧬️schema/🦀️.rs` (flagged in the
prior research note for re-audit once `AppDefinition`/`WindowKindDefinition` convert — they still
haven't, so it's still needed). No Cargo.toml edits. No `🧪️oracle/`/`🧪️test/`/`🧪️tests/`/
`🔬️probes/`/`🏭️generator/`/`🧫️fixtures/` edits.

## Note: `--tests` build (not the required gate) has pre-existing, unrelated failures

`cargo check -p semio-framework --tests --message-format short` (the required gate per this ticket's
VERIFY section is the plain `cargo check`, without `--tests`, which is 0 errors) surfaces 28 errors in
`#[cfg(test)] mod` code — but every one traces to types NOT touched this pass: `AppRef`,
`manifest::CommandInvocation`, `manifest::AgentContributions`, `manifest::ActionInvocation` (already
`ToValue`/`FromValue`-only from an earlier wave, no `Serialize`/`Deserialize` — test code calling
`serde_json::to_string` on them was already broken before I started), plus
`semio_framework_ui_contract::PresenceUpdate` in `🎠️kernel/📤️return/📦️content/🧪️dialects/🦀️.rs`
(a `🎠️kernel` file, not owned by this pass, likely mid-edit by another concurrent agent). None of the
34+3 types this pass converted or reverted appear in this error list — confirmed by reading every
error line before concluding it's pre-existing, not assumed. Left untouched per the "do not touch
test dirs" rule; flagging for whichever wave owns `AppRef`/`CommandInvocation`/`ActionInvocation`/
`AgentContributions`'s test fixtures and `🎠️kernel`'s `PresenceUpdate`.

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs`
