# Native BIM Placement And Original FEM Pairing Checkpoint

## Result

`native_graph.rs::Builder::placement` now emits an explicit three-dimensional positive-Z IFCDIRECTION alongside RefDirection for every nonzero horizontal building rotation. Zero rotation retains both optional directions absent. The shared `Reader::axis` requires the exact three-field IFCAXIS2PLACEMENT3D tuple and rejects one-sided Axis/RefDirection before reading its vectors. Its existing CartesianPoint/Direction roles, three-coordinate requirement, finite values, positive horizontal Z and nonzero planar reference restrictions remain intact. BIM and reference FEM readers share this helper.

The actual original native BIM routes in `native_domains.rs` call those same functions via `original_mapping`, retaining their existing `IoRunControl`/decode/encode path; there is no second BIM placement builder. The original native FEM route has its own `native_fem_owned.rs::Graph::identity_axis`. It now requires the exact tuple and paired presence too, preserving original source owner/receiving wallet infrastructure and all existing identity, coordinate-role and dimension checks. Original FEM graph construction already emits both directions absent and was not changed. Native glTF construction likewise already emits paired absent identity frames; its separate axis evaluator and the external geometry engine were not edited by this worker.

## Tests Authored Before Production Changes

Native BIM tests consume root's language-neutral `io/🧪️tests/bim-axis-cases.json` angles -0.7, 0, 0.3, π/2 and π. They inspect every emitted axis pair, the rotated building's exact positive Z and planar reference, origin/elevation preservation and complete BIM roundtrip. A separate authored test invokes the real controlled BimIntoIfc/IfcIntoBim domain types with original native decode/encode recipients and an explicit fixture grant across all five cases, then drains any remaining original retirement owners.

Native negative tests consume the existing `axis-cases.json` pair, wrong-role, short-dimension, zero-axis and parallel-axis fixtures. They also remove each direction in turn from an actual rotated native building and require exchange.placement refusal. Original FEM tests build through the actual controlled native constructor, supply either an otherwise valid positive Z alone or positive X alone to its identity context, invoke the actual IfcIntoFem serializer with its original receiving/encoding control, require InvalidValue and drain the original recipients. These tests are authored and **not executed**.

## Executed Independent Evidence

`py -3.13 -X utf8 <ticket>/native-bim-axis-oracle.py` passes using existing IfcOpenShell 0.9.0.post1 behind a test-only PlacementOracle interface. The same five neutral angles produce valid IFC4X3_ADD2 placements with the prescribed paired presence; actual EXPRESS validation has no errors and its independent placement matrices agree with the expected rotation, Z and origin. Both otherwise-valid one-sided identity directions produce real EXPRESS validation errors. Output: `{"oracle":"IfcOpenShell 0.9.0.post1","schema":"IFC4X3_ADD2","pairedRotationExpectations":5,"oneSidedIdentityRefusals":2,"nativeExecution":false}` (whitespace normalized).

This checks independent neutral expectations, not generated native Rust output. Root owns the separately executed TypeScript builder and its full-graph oracle results. No native builder/reader success is inferred from the Python result.

Rustfmt parsed all four edited Rust files using `--emit stdout --config skip_children=true`; existing files were not reformatted or overwritten. `cargo test --manifest-path <IFC4x3>/📦️packages/🦀️rust/Cargo.toml --lib native_bim_five_rotations -- --nocapture` again refuses at the authentic `-vV` enforcing compiler ingress with `Original compiler ingress requires the repository launcher and authentic enforcing store`, before rustc/type checking. Native BIM/FEM tests, original wallet effects and retirement assertions have not executed. No bypass was attempted.

## Exact Changed Roster

- `io/native_graph.rs`: paired rotated native construction and strict shared horizontal-reader tuple/pair admission.
- `io/native_fem_owned.rs`: strict tuple/pair admission in the original native FEM identity reader.
- `io/🧪️tests/native_bim.rs`: five-angle direct/controlled native route and pair/role/dimension negative tests.
- `io/🧪️tests/native_fem_owned.rs`: actual original serializer rejection and funded-retirement assertions for both one-sided identity forms.
- Ticket: `native-bim-axis-plan.md`, `native-bim-axis-oracle.py`, this report and the shared coordination append.

No TS source, package registration, root-owned neutral fixture, external geometry, AGENTS, Git, goal or ticket lifecycle was changed. The oracle reads the retained shared dependency cache without deleting it and creates no persistent output file. Existing asynchronous/original-wallet limitations outside these narrow axis checks remain unchanged and are not represented as closed.
