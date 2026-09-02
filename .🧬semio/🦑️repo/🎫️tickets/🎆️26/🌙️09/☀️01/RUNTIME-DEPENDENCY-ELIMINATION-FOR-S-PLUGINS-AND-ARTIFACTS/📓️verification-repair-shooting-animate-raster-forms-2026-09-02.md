# Verification-and-repair pass: shooting, animate, raster, forms (2026-09-02)

Real `cargo check -p <crate> --message-format short` runs (foreground, warm default target
dir, `grep -cE ': error(\[|:)'` counts — anchored `^` was NOT used). No `CARGO_TARGET_DIR`
override.

## semio-s-plugin-shooting
Before: **550** errors. After fix: **549**.

**Ours (1, fixed):** `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️.rs` —
`ExportShots::handle` parsed a JSON string into `serde_json::Value` then called
`semio_framework_plugin::to_dsl_value(&value).ok()`, which resolves to the generic
`value::to_dsl_value<T: ToValue>` (not the `pack::json` one taking `&serde_json::Value`), so
`serde_json::Value: ToValue` was never satisfied. Fixed by using the framework's own infallible
`From<serde_json::Value> for DslValue` bridge instead: `.map(DslValue::from)`.

**Peer churn (549, left alone):** dominated by the repo-wide async-convention-debt wave
("is not a future"/"expected future"/E0053 incompatible-future-type — 473+ error lines), plus
`E0432` `…::mutation` module-structure churn (13), and a UI-contract API rewrite
(`Label`/`ActionDescriptor`/`UiValue`/`PanelTabDefinition` mismatches, ~72 lines). A handful of
`ToValue`/`FromValue`-not-satisfied errors (`ShootingCamera`'s `#[value(default = "...")]`
default-fns, etc.) are downstream fallout of those same functions having been made `async` by the
peer wave — not something wrong with the derive/attribute conversion itself.

## semio-s-plugin-animate
Before: **90** errors. After fix: **89**.

**Ours (1, fixed):** `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:80` —
production call built a `serde_json::json!(...)` literal directly around a `FigureTileSource`
value, but `FigureTileSource` was converted to `value_derive::ToValue/FromValue` with no serde
derive at all, so `FigureTileSource: Serialize` failed. Fixed by routing through
`dsl::ToValue::to_value(&x).into()` (the same `DslValue -> serde_json::Value` `From` bridge used
elsewhere in this wave) instead of the `json!` macro.

**Peer churn (89, left alone):** `PptxSnapshot`/`SemioPresentationSnapshot`/`SemioAnimationSnapshot`
losing `Serialize`/`Deserialize` in the stdio crate (the documented cross-cutting stdio blocker —
out of scope, different crate); `DESCRIPTORS`/`descriptor` missing on `Mutation` impls; `E0432`
`…::mutation` module churn (13 lines); the same UI-contract `Label`/builder-method rewrite as
shooting; async-convention `E0053`/"is not a future" fallout.

## semio-s-plugin-raster
Before: **119** errors. After fix: **118**.

The specific "3 files" this ticket names (`🚪️io/📤️export/🧵️serializers/…/🔣️json/…/🦀️.rs`,
`🚪️io/📥️import/🧩️deserializers/…/🔣️json/…/🦀️.rs`,
`🚪️io/📥️import/🧩️deserializers/…/🖊️dwg/…/🦀️.rs`) plus `🚪️io/🦀️.rs`'s retyped
`raster_document_json_to_svg`/`raster_document_json_from_dwg` (now passing `RasterSnapshot`
directly, no round-trip) compile with **zero** errors — confirmed by grepping the error log for
those exact file paths.

**Ours (1, fixed):** `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` —
`RasterArtifact`'s `Serialize`/`Deserialize` derive was `#[cfg_attr(test, derive(...))]` but its
`assets` field carried an unconditional `#[serde(serialize_with = "...")]`, which is meaningless
outside test cfg since no `#[derive(serde::Serialize)]` registers that attribute name there
("cannot find attribute `serde`"). First attempt (making the derive fully unconditional) was
**reverted** — it cascaded into a genuine peer-churn error (`ArtifactChild<SemioImageSnapshot>:
Deserialize` — the stdio serde-loss blocker) for a net +1, worse than before. Correct minimal fix:
gate the field attribute with the same `#[cfg_attr(test, serde(...))]` as the type-level derive.
Net: -1, no new errors.

**Peer churn (118, left alone) + 2 pre-existing/unrelated (not this wave, not touched):**
`semio_framework_value_derive` unresolved-crate errors across the whole `✏️editor/🎮️commands/*`
surface (confirmed in `📓️serde-to-value-dag-raster-sourcing-forms.md` as a *different*,
concurrent session's `component.rs → 🦀️.rs` rename wave referencing a crate never added to
raster's manifest — not this ticket's conversion); `E0432` `…::mutation` module churn; the same
UI-contract `BuiltNode`/builder-method rewrite; two pre-existing `dsl::to_dsl_value(&serde_json::
json!(...))` misuses in `semio_fixture_snapshot` (schema/🦀️.rs:398-399, git-diff confirmed
untouched by any live session, same root cause pattern as the shooting fix but a different call
site nobody has fixed yet — left alone since it predates and is outside this ticket's named scope).

## semio-s-plugin-forms
Before: **1** error. After: **1** (no change — nothing of ours to fix).

Cannot get a forms-specific signal at all: the single error is
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/../../🌿️vcs/🦀️.rs:2771` —
`E0502` borrow-check failure in `semio-framework-os-flow`'s vcs module, entirely unrelated to
serde/value work, blocking forms's own compilation before it starts (matches
`📓️serde-to-value-dag-raster-sourcing-forms.md`'s prior finding of the same upstream blocker).
Not touched — out of scope (different crate, unrelated feature).

## Summary
| crate | before | after | ours fixed | peer-churn (left alone) |
|---|---|---|---|---|
| shooting | 550 | 549 | 1 | 549 |
| animate | 90 | 89 | 1 | 89 |
| raster | 119 | 118 | 1 | 118 |
| forms | 1 | 1 | 0 (blocked upstream) | 1 |

No `serde_json::Value` round-trip was reintroduced anywhere; no serde derives were re-added to
`DslArtifact`/`MutationLeaf`/etc.; all fixes route through the framework's own
`DslValue <-> serde_json::Value` `From` bridge or `dsl::ToValue`/`dsl::FromValue`.

## Files fixed
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖨️export/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
