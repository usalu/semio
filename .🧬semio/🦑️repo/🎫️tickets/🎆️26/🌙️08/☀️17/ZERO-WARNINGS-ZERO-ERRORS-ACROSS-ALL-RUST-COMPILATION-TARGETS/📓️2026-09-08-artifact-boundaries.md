# Native Artifact Boundary Corrections

## Pass539 — App Declarations

Sequence, Wires and DAG now accept an app fleet that implements the artifact's concrete editor and viewer conversions. The artifact, standard and subset declarations share that type parameter. Parent plugins retain their closed app enums; extracted artifacts no longer refer to a nonexistent parent plugin module. This follows the same declaration boundary already used for Drawing and VCS.

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🦀️.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`

## Pass540 — Duplicate Snapshot Exports

Removed the redundant early exports for Din18599Snapshot and Block3dSnapshot. The canonical exports from each artifact's standard schema remain.

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️.rs`

## Pass541 — FEM 2D Engine Ownership

The FEM plugin now uses the extracted artifact crates directly. Its 2D engine and existing numerical tests therefore import document types from their owning artifact crate root. The test fixture uses document_dsl. The numerical model always uses the dispatch macros, so that existing compile-time dependency is required independently of UI assembly. No numerical formulas or test expectations changed.

- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🎵️modal-buckling/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🗺️mesh-preview/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🎵️modal-buckling/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🗺️mesh-preview/🧪️tests/🔬️unit/🦀️.rs`

## Pass542 — Existing Test Ownership

The unchanged Lowpoly, Procedure and CAD demo tests are now mounted in the artifact crates they test. Their old parent-plugin mounts were removed. CAD's sole viewer command derives Default on its Noop variant in the defining artifact crate, replacing the invalid test-only implementation in the plugin crate. The surface laws remain mounted and unchanged.

- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🧪️tests/🔬️surface/🦀️.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`

Native532 supplied the failing compiler evidence. Concurrent work had already corrected the Puzzle attraction-geometry paths and Drawing FSM library path when inspected, so those files were not edited in these passes. Syntax, compiler and runtime validation of passes539–542 remain required.


Pass543 rustfmt syntax validation: 26/26 files parsed with unchanged input hashes. This does not establish compiler or runtime success.
