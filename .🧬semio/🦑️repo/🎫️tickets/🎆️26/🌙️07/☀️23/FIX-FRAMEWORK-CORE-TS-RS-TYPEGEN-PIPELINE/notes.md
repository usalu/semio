# Findings

## `ts(optional)` bug — already fixed, no longer reproduces
`framework/core/rs/lib.rs`'s `IntroductionStepDefinition.cutouts` (`Vec<IntroductionAnchor>`) no longer
carries `#[cfg_attr(feature = "typegen", ts(optional))]` — someone (a concurrent session) already removed
it before this ticket started. Swept the whole file for other `Vec<T>` fields still wrongly paired with
`ts(optional)` (ts-rs only allows that attribute on `Option<T>`, confirmed via ts-rs wiki/docs) — none found.
Two full `cargo test --features typegen exports_typescript_bindings` runs (direct + via
`bun nx run @semio-tech/framework-core:generate`) show zero `ts(optional)` errors.

## "failed to parse serde attribute" warnings — cosmetic, not version drift
Confirmed via ts-rs docs (v10.1.0, the version pinned in `Cargo.lock`): ts-rs only understands a fixed
serde-attribute allowlist (rename, rename_all, tag/content, untagged, skip, flatten, default). Anything
outside that (`skip_serializing_if`, `serde(transparent)` on tuple structs, etc.) triggers a **warning**,
not a hard error, unless the `no-serde-warnings` feature is off (it's off here, so warnings print — harmless).
Spot-checked `ActionRef(String)` with `#[serde(transparent)]`: generated binding is already correct
(`export type ActionRef = string;`) because ts-rs natively unwraps single-field tuple structs regardless of
that attribute. No ts-rs version bump needed.

## Real current blocker: unrelated, actively in-progress `ui_wgpu` refactor
`bun nx run @semio-tech/framework-core:generate` still fails — not from anything in `framework/core/rs`,
but because framework-core's `typegen` Cargo feature unconditionally pulls in `ui_wgpu/typegen`
(`framework/core/rs/Cargo.toml`: `typegen = ["dep:ts-rs", "ui_wgpu/typegen"]`), and `ui/wgpu/rs/lib.rs`
currently has uncommitted, mid-flight changes threading a new `waiting: Option<bool>` field through
`UiButtonNode`/`UiStackNode`/`UiSectionNode`/`UiTreeItemNode`/`UiTreeNode`/`UiTreeSectionNode` — not yet
fully propagated (constructor/field-name mismatches, `E0063`/`E0560`). This is ticket
`26/07/23/WAITING-STATE-FOR-ALL-UI-ELEMENTS` (open, session `⚪️2fd49f6762ed4f1aa0ca2dfa4806e5bc`,
plan `/Users/ueli/.claude/plans/introduce-waiting-state-for-precious-pnueli.md`). Two consecutive runs of
this ticket showed *different* `ui_wgpu` errors each time — that session is actively iterating.

Two full attempt logs saved alongside this ticket for reference.

## Not attempted here
Did not touch `ui/wgpu/rs/lib.rs` — it's another session's active, uncommitted work; editing it now would
race/conflict with that session and is out of scope for the ts-rs typegen bug this ticket was opened for.
Manifest regeneration (and diffing against the hand-patched `framework/core/js/generated/manifest.ts`) is
deferred until `WAITING-STATE-FOR-ALL-UI-ELEMENTS` lands and `ui_wgpu` compiles clean again.
