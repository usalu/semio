# glTF Independent Semantic Audit

## Confirmed Corrections

- The deleted geometry umbrella has no active glTF mount or `pub use geometry as bounds` forwarding alias.
- Aggregate contract ownership is consistently named `geometricAnalysis`/`geometric_analysis` across Rust, TypeScript, JSON Schema, GraphQL, Proto, and text representations.
- Binary and text codecs no longer remain in the schema inference tree; glue remains mechanical.
- The initial nested multi-responsibility `schema/inferences/modules/geometric-measurement` placement was invalid. The implementation lease corrected it to subset-LCA modules: `vector-operations`, `inference-measures`, and `mesh-topology`.
- No direct edits to generated outputs were observed.

## Release Blockers

1. The three surviving glTF modules need canonical collection/member manifests declaring their exact production consumers. Current report data declares none and resolves only glue, so the two-independent-component check fails.
2. The semantic graph must resolve the metric components' relative imports as terminal production edges instead of collapsing the graph at stdio glue. This is a central discovery-adapter defect, not evidence that the modules have one consumer.
3. The I/O inference-codec collection requires its canonical manifest/tree membership, and taxonomy/generator declarations still reference the retired `schema/inferences/{binary,text}` locations.
4. Retired empty geometry/text/binary directories must be removed after mounts/generator paths are migrated.
5. Rerun `cargo check -p semio-s-plugin-stdio`, the glTF test filter, scoped taxonomy report, and the workspace gate after the corrections. Earlier observations (Cargo check and 91 filtered glTF tests) predate the final tree and do not release this lease.

## Protocol Identifier Decision

Strings such as `s.stdio.gltf.geometry` are representation/provenance tags, not component names. They may remain only where protocol representation correctness requires stable wire values; the source-level aggregate field and collection identity must not retain a geometry alias.
