# W2 — Open App-Schema Registry API (component.rs)

File owned: `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
Crate: `semio-framework-schema` (Cargo.toml at `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`)

## Scope

Additive-only: make the open, general-purpose app-schema registry fully usable by
external plugin crates without touching the closed catalog (`register_all_app_schema_descriptors()`,
~lines 350-1014, unchanged) or the parked `catalog-integration`-gated regions
(unchanged — those still reference `semio_s_plugin_*::register_*` functions that
don't exist yet; confirmed via `cargo check --features catalog-integration`, which
still fails with the same pre-existing `E0433 cannot find module` errors for every
plugin crate name — that failure is expected/parked, not something this wave fixes).

## Findings — visibility audit of the open API

Before changing anything I checked what was actually private vs. public (did not
assume the task description's list was accurate as given):

- `AppSchemaDescriptor` (struct + all 3 fields `id`/`config`/`presence`) — already `pub`. No change needed.
- `FacetLeaves` (struct + all 5 fields `rust`/`typescript`/`graphql`/`json_schema`/`proto`) — already `pub`. No change needed.
- `AppSchemaRegistry` (struct + `new`/`register`/`get`/`iter`/`len`/`is_empty`) — already `pub` throughout. No change needed.
- `register_app_schema_descriptor` — already `pub fn`. No change needed (only added/extended its doc comment).
- `app_schema_descriptor_registered`, `with_app_schema_registry`, `with_app_json_schema_catalog`, `app_schema_graphql_sdl` — already `pub fn`. No change needed.
- **`validate_registered_app_descriptor` — was NOT part of the open API at all.** It existed only as a
  private `fn` (no `pub`) defined inside `#[cfg(test)] mod tests { ... }` (old location: lines ~1506-1538,
  used by two tests in the `AppSchemaRegistryParity` subregion). Being inside a `#[cfg(test)]` module means
  it was never compiled into the crate for external callers regardless of visibility keyword — a plugin
  crate could not have called it under any circumstances. This is the one real blocker the ticket's premise
  pointed at.

## Change made

Relocated `validate_registered_app_descriptor` out of `mod tests` into the main
`GlobalAppSchemaCatalog` region (right after `app_schema_graphql_sdl`, before the
region's `//#endregion`), unchanged in behavior, now `pub fn`. The two test call
sites (`app_schema_registry_registers_and_validates_all_thirty_nine_owners` and
`app_schema_registry_accepts_placeholder_owner_for_wave_structure`) needed no edits —
they already resolve the symbol via the module's existing `use super::*;` glob import,
now pointing at the promoted top-level `pub fn` instead of the old test-local one.

Added a doc-comment block above `register_app_schema_descriptor` (the primary
entry point) listing all six open-API functions, one line each with a unique emoji,
naming it as the contract plugin crates should call from their own `🔧️setup`/init
code — this is the surface the later per-plugin fan-out wave will target:

- 📎 `register_app_schema_descriptor`
- 🔎 `app_schema_descriptor_registered`
- 📚 `with_app_schema_registry`
- 🔣 `with_app_json_schema_catalog`
- 🔗 `app_schema_graphql_sdl`
- ✅ `validate_registered_app_descriptor` (new doc comment on its own promoted definition too)

## Not touched (per instructions)

- `register_all_app_schema_descriptors()` and its ~390 `include_str!` roster (lines ~350-1014, unchanged, only shifted downward by the inserted lines above it — no edits inside it).
- Both `catalog-integration`-gated regions (`register_all_plugin_artifact_schema_descriptors` etc.) — untouched, still parked, still calling nonexistent `semio_s_plugin_*::register_*` fns.
- `ArtifactSchemaDescriptor`/`ArtifactSchemaRegistry`/`register_artifact_schema_descriptor` (the artifact-schema twin API) — already fully `pub`, out of this ticket's named scope (which named the *app*-schema registry), left as-is.

## Verification

- `cargo check -p semio-framework-schema` — clean, no errors, no warnings attributable to this file. (Finished in ~3s.)
- `cargo check -p semio-framework-schema --tests` — clean, no errors. Remaining warnings in output all originate from the `semio-framework-os-kernel` dependency (unused imports/dead code in unrelated modules — pre-existing, not caused by this change, not touched).
- `cargo check -p semio-framework-schema --tests --features catalog-integration` — still fails with the same pre-existing `E0433 cannot find module semio_s_plugin_*` errors as before my change (confirmed this is the known-parked state described in the ticket, not a regression I introduced — I did not modify that code).

## Files changed

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
