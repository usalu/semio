# Extracted Artifact Test Imports

Pass544 resolves current native532 diagnostics: Lowpoly's descriptor law imports ArtifactSchemaFields and parse_state_class_kebab from framework_schema; Equation's scene-owner law uses the OS pack JSON API; Procedural's constraint and solver tests use the existing local flow and solver_graph modules. The related solver documentation link was repaired. Procedural's dispatch macro dependency is required by its constraint enum without UI assembly, so it is no longer optional. Test inputs, expectations and solver algorithms are unchanged.

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔳️solver-grid-2d/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧱️solver-grid-3d/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕸️solver-graph/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔍️search/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/📦️packages/🦀️rust/Cargo.toml`

Pass545 moves the unchanged Process3D demo laws into the owning artifact crate and removes the empty example grouping left in the parent plugin:

- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs`

The Norm EN1990 and EN1994 fixture-test paths had already been repaired by concurrent work when checked. Parent Puzzle, Trinity and FEM manifest paths now point to their plugin roots; native532 loaded the removed old paths earlier. Those stale diagnostics require a fresh compiler pass. Syntax and compiler validation of passes544–545 remain pending.


Pass547 rustfmt syntax validation for passes544–546: 27/27 files parsed with unchanged input hashes. This does not establish compiler or runtime success.
