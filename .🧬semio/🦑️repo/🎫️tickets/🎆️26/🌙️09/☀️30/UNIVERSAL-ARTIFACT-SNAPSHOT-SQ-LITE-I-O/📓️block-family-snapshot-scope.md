# Block Family Snapshot Scope

Read-only inventory during the sole STL native gate. No Block artifact model, provider, declaration or runtime source was changed.

The actual owning AGENTS.md distinguishes kind definitions from Puzzle assemblies. Exact native snapshot coordinates from the authored schema/declaration constants are s.block.block2d@1/*, s.block.block3d@1/* and s.block.block5d@1/*. The three snapshot structs are independently owned; shared document records are genuinely reexported from Block2d's 🧬️schema/🧱️shared/🦀️.rs. No SQLite provider was present in the bounded path inventory. Presence alone is not a runtime capability proof.

Block2d persists schema, node_kind, presentation, handle_kinds, handles, compatibility, attributes, authors, camera2d and meta. Presentation owns optional shape/color/icon_kind strings and independently optional radius/width/height f64. Each handle kind owns id/name/label/color/default_wire_kind; each handle template owns id/handle_kind plus f64 angle/radius.

Block3d persists schema, object_kind, representations, independently persisted five-string catalog child identity, vortex_kind_extra, vortices, compatibility, attributes, authors, camera3d and meta. Vortex metadata owns id/name/label/color/default_cable_kind. Vortex templates own id/vortex_kind, position and direction triples, radius and optional label. The catalog handle is persisted independently of the vortex metadata: derived catalog reconstruction is not an export representation.

Block5d persists schema, part_kind, part_2d, part_3d, representations, grip_kinds, grips, compatibility, attributes, authors, camera2d, camera3d and meta. Part2d owns the same six independent optional presentation fields as Block2d. Part3d owns optional orientation quadruple and scale triple. Grip kinds own id/name/label/color/default_rope_kind; grip templates own id/grip_kind, angle/radius_2d, position/direction triples and radius_3d.

Genuinely shared records:
- Kind identity: required id/name/label/description and independently optional variant/icon/unit.
- Attribute: key/value and optional definition string.
- Author: id/name and optional email.
- Compatibility: id/source/target and bidirectional bool.
- Representation: id/name/description, optional mesh_url/lod, ordered tags and ordered attributes.
- Camera2d: x/y/zoom f64; Camera3d: position/target triples and zoom f64.
- Meta: independent description string.

Each final schema needs literal per-owner domain tables, ordered occurrence relationships, exact Option presence, duplicate/order retention and independently persisted strings. Floating fields need individually named REAL/IEEE-word/numeric-class companions for every native state; a REAL-only schema would repeat the audited geometry fidelity gap. Unresolved persisted source strings must not become mandatory foreign references without evidence of a real owning invariant. Native and TypeScript numeric domains and actual validators still need inspection before implementation. Canonical derived native record metadata/binding/emission can be reused only through genuine controlled producers and explicit preallocation row admission. No whole-value/native carrier or kind-definition-to-assembly conversion is proposed.

Verification is pending. This inventory does not count any Block dialect as covered.
