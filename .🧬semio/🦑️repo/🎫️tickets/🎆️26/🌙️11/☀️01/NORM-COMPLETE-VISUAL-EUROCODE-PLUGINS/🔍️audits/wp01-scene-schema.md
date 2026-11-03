# WP-01 Scene Schema (A1) - status: WRITTEN, NOT YET COMPILED OR TESTED

Exit gate NOT met: both `mcp__repo__show_projects` and `mcp__repo__cargo` (test --lib on the crate manifest) hung for 1800 s with no response, twice. No build or test ever ran. Code below is unverified; expect compile fixes.

## Built (crate `semio_s_plugin_norm_structural_scene`, `NORM/🎬️structural-scene/`)
- `🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml` (deps: semio-framework-value, -value-derive; dev: serde_json). Registered in `NORM/Cargo.toml` members. No `📋️project.json`/`📜️script.ts` added (the sibling compliance crate has only a Cargo.toml); check `show_projects` sees it.
- `🧬️schema/🦀️.rs` mounts: `🪄️macros` (records!/variants!/choices! = ToValue/FromValue derives, camelCase, variants tagged `{kind,data}`), `✅️validation`, `🧾️canonical-json`, `📐️geometry`, `🎨️appearance`, `🧱️section-shapes`, `🏹️load-glyphs`, `🧩️entities`, `🏷️annotations`, `🔺️scene-diff`, plus A2's `📈️diagram2d` and `🕹️handles`.
- Proto/GraphQL/TS twins generated from the Rust declarations by `TICKET/wp01-gen-twins.py` (88 types; run `python3 -I wp01-gen-twins.py [--check]`). They import A2's part files.
- Tests: `🧪️tests/🔬️unit/🦀️.rs` (round trip, canonical JSON vs serde_json, wire shape, section areas vs closed forms, validation rules, hash, delta, twin parity) and `🧪️tests/🥒️.feature`.

## Public API
Validate/Violation, canonical_json, SCENE_SCHEMA_ID, StructuralScene, SceneRevision, EntityRef, EntityKind, SceneEntity (compute_hash, sealed), Geometry (+Member/Plate/Shell/Solid/Fastener/Support/Surface geometries, ShellShape, SolidShape, FastenerKind, Pattern, SurfaceShape), LoadGlyph, LoadProfile, SupportKind, SupportGeometry, LoadCaseRef, Appearance, MaterialRole, ColourChannel, StyleState, UtilisationBinding, DeformationOverlay, Layer, ColourMapRef, Annotation, Legend, LegendScale, SectionShape (outline, area), ProfileKind, SceneDelta, EntityPatch, AnnotationPatch, geometry primitives, LocalizedCopy, Quantity.

## Deviations
- `content_hash` is a 16-hex-digit String (FNV-1a of canonical JSON), not u64, to survive JS numbers. `opacity` is f64. Tuples replaced by named structs; fixed arrays by Vec with validated length.

## Open items
1. Golden `🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` is EMPTY: after the first successful run, fill it with `canonical_json(&scene())` (test `golden_scene_matches_committed_bytes` fails until then).
2. A2 stubs: `📈️diagram2d/🦀️.rs` and `🕹️handles/🦀️.rs` are A1 placeholders (FieldBinding{path}); A2 must overwrite. See `📌️important/requests.md`. The twins' import lines expect A2's `{🛰️.proto,🔗️.graphql,🟦️.ts}` there.
3. Unchecked assumptions: Option encodes as null in the TS twin; derive handles Box<SectionShape>, unit variants in adjacent tagging, u8/u64/i32; `use super::*` glob of pub(crate) helpers; docstring emoji uniqueness not machine-checked.
4. Repo MCP hang needs investigation by the caller.

## Takeover update (A1 successor)
- Fixed `Cargo.toml` of the crate: `workspace` was `../../../..` (one level too many; siblings use `../../..`) and the package name now follows the sibling convention `semio-s-plugin-norm-structural-scene`. Added matching `[workspace.dependencies.semio-s-plugin-norm-structural-scene]` in `NORM/Cargo.toml`. Discovery needs no `project.json`/`script.ts`: workspace `member-manifests` glob `**/📦️packages/🦀️rust/Cargo.toml` covers it (compliance sibling has none either).
- A2 has replaced the `diagram2d` (412 lines) and `handles` (233 lines) stubs; twins not yet re-checked.
- Build NOT run to completion: `repo run '@semio-s/plugin-norm-structural-scene:check'` (background, log `🗑️generated/wp01/check.txt`) has waited >50 min in the capacity queue (memory pressure; FIFO blocked by ~10 stuck `repo show projects` requests of the fleet that each need 6.4 GB). Killing those was denied, so it stays queued.
- Still open: compile+test, golden fixture fill, twin re-check (`python3 -I wp01-gen-twins.py --check`), full norm workspace check.
