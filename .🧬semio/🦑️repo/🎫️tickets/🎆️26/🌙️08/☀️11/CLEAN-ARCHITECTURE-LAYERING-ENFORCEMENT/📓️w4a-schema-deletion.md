# Wave 4a — Schema Closed-Catalog Deletion

## Scope
Files owned:
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`

## Step 1 — Confirmed self-registration is in place before deleting
- Read `w3-verify-summary.md`: explicit **GO** verdict — "Step A (`register_app_schema()`) is now wired for every app that has one, across all 15 slices, confirmed against the framework's own parked `catalog-integration` expected-fn-path list."
- Counted 39 `id: "s.…"` entries in the closed catalog (matches the documented "39 owners").
- Spot-checked 7 plugins' own `register_app_schema()` fn exists at the path the closed catalog's doc-comment / the parked `catalog-integration` call-site list expected:
  - `📐️cad` (`apps/cad/config/schema/component.rs:103`) — present
  - `✒️writer` (`apps/writer/config/schema/component.rs:25`) — present
  - `🌊️flow` (`apps/flow/config/schema/component.rs`) — present
  - `🌀️procedural` `◻2d` and `🧊️3d` apps — present
  - `📕️norm` (`config/schema/component.rs:14`) — present
  - `🪵️sourcing` `🗂️curate` (`apps/curate/config/schema/component.rs:22`) — present
- All 7/7 spot-checks confirmed. Proceeded.

## Step 2 — Deleted `register_all_app_schema_descriptors()`
Deleted the full function (doc comment + body, ~665 lines, all ~390 `include_str!` calls and `AppSchemaDescriptor` literals for the 39 owners). Updated the now-stale doc comment on `register_app_schema_descriptor` (previously said "This is the contract the next wave's per-plugin fan-out (the parked `catalog-integration` call sites below) follows" — rewritten since those call sites are now gone: "Every app owner self-registers via `register_app_schema_descriptor`; there is no closed framework-side catalog.").

## Step 3 — Deleted both `#[cfg(feature = "catalog-integration")]` regions
The `//#region 🔖️CatalogIntegration` block contained two dead sub-parts behind the same never-enabled feature:
- `register_all_plugin_artifact_schema_descriptors()` (~54 calls to `semio_s_plugin_*::…::register_artifact_schema()`, artifact-catalog scope, unrelated to my ticket but inside the same dead region) + its helper fns (`facet_formats_resolved`, `json_property_keys`, `persistent_property_keys_from_artifact_json`, `assert_json_states_parse`, `validate_registered_artifact_descriptor`) + the `#[test] artifact_schema_catalog_registers_and_validates_all_fifty_four_artifacts`.
- `register_all_plugin_app_schema_descriptors()` (~39 calls to `semio_s_plugin_*::apps::…::register_app_schema()`).
Deleted the whole region (all cfg-gated, all calling functions/paths that are either never-created or simply unreachable since the feature is permanently parked). Also removed the now-unused `#[cfg(all(test, feature = "catalog-integration"))] use std::collections::BTreeSet;` import at the top of the file (only consumer was inside the deleted region).

## Step 4 — Cargo.toml
Removed the `catalog-integration` feature declaration (`catalog-integration = []` and its comment) from `[features]`. No `optional = true` plugin deps existed for it — the crate's `[dependencies]` list (`jsonschema`, `schemars`, `serde`, `serde_json`, `thiserror`, `semio-framework-os-kernel`, `semio-framework-schema-derive`) has nothing feature-gated, so nothing else to drop.

## Step 5 — Deleted the framework 39-owner parity test
Deleted `#[test] fn app_schema_registry_registers_and_validates_all_thirty_nine_owners()` (asserted `registry.len() == 39` after calling the now-deleted `register_all_app_schema_descriptors()`). Kept the sibling `empty_app_facet_leaves()` helper and `#[test] fn app_schema_registry_accepts_placeholder_owner_for_wave_structure()` — unrelated, still needed. Did not attempt to relocate the parity check (out of scope — registry codegen's job per instructions).

## Step 6 — Public API preserved
Confirmed still `pub`: `register_app_schema_descriptor`, `AppSchemaRegistry`, `validate_registered_app_descriptor`, `AppSchemaDescriptor`, `FacetLeaves` (plus untouched siblings `app_schema_descriptor_registered`, `with_app_schema_registry`, `with_app_json_schema_catalog`, `app_schema_graphql_sdl`).

## Step 7 — Verification
- `cargo check -p semio-framework-schema` → **clean** (only pre-existing unrelated warnings surfaced from `semio-framework-os-kernel`'s own build, nothing schema-crate-specific).
- `cargo check -p semio-s-plugin-cad -p semio-s-plugin-flow -p semio-s-plugin-procedural` → all three hit the known concurrent "document" module churn: `couldn't read .../🎛️apps/<app>/📌️panels/📄️document/🦀️component.rs` (flow, cad, procedural `◻2d`). This is exactly error-class (a) documented in `w3-verify-summary.md` ("Concurrent 'document' refactor... dominant blocker in nearly every per-crate cargo check"), not caused by this change. Grepped the full cargo output for `register_app_schema|AppSchemaDescriptor|AppSchemaRegistry|FacetLeaves|register_all_app_schema|catalog-integration|semio-framework-schema` — **zero hits**, confirming no new error class was introduced by this deletion.

## Out-of-scope observations (not acted on, files not in my ownership)
Several plugin files have stale docstring comments pointing at the now-deleted function/feature (comments only, not compiled calls — verified they don't break compilation):
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs:102` — `/// see ...::register_all_app_schema_descriptors()`.
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs:19` — `mirrors the parked catalog-integration call site's exact fn path`.
- ~20 other plugin app schema files reference `register_all_app_schema_descriptors` in doc comments (grep: `register_all_app_schema_descriptors` across `✏️s/🔌️plugins/**`).
Left untouched — outside my file ownership for this ticket slice.

## Files touched
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs` (edited)
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` (edited)
