# Puzzle 2D Clipboard Handle Names

`Puzzle2dRecorder::clone_fragment` generated a free node ID independently of its handle IDs. A destination handle named `node-10:v1` can belong to `elsewhere`. The old paste chose `node-10` anyway and could give that existing destination handle and a pasted handle the same identity.

The recorder now builds one set of destination node and handle IDs for the fragment. It checks each proposed node ID and all its prospective handle IDs against that set, then reserves the new endpoints only when the clone is recorded. Subsequent clones use the same set. Successful clones keep the `node-N` and `node-N:vI` naming rules, and the edge remap keeps each original endpoint and its owner. The change replaces the per-clone linear node-ID check with one scan of the destination endpoints for the fragment.

The shared topology fixture covers a destination handle owned by a different node, a destination node named like a prospective handle, and a destination handle named like a prospective node. The baseline native core passed the ordinary case and failed the new case, choosing `node-10` and `node-11` where the independent SQLite oracle expects `node-13` and `node-14`. The updated native core passed both cases. The collision case checks that all endpoints are unique, the pasted edges identify the expected cloned owners and ports, and the existing edge retains its original owners. All ten Bun source tests passed using SQLite and Ajv.

The standalone native verifier extracts the current recorder definitions and resolves its pack, value and Serde dependencies from one warm pack metadata graph under `cargo-puzzle-clipboard-build/debug/build`. It checks host memory and workspace storage before and during linking. It does not build the whole editor. Native logs are `🗑️generated/clipboard-2d-handle-namespace-before.log` and `🗑️generated/clipboard-2d-handle-namespace-after.log`.

## DWG dependencies in the production recorder probe

Puzzle 2D declared the aggregate `semio-s-artifact-stdio-dwg` crate in 30 manifests, although its Rust library and test targets had no references to that crate. The cleanup removes those dependencies and their workspace catalog entry. The DWG read and geometry dependencies used by the format tests and the shared drawing exporter used by the DWG, DXF, SVG, PNG, and PDF serializers remain in place.

The four live locks now reflect the removal of these edges. The Puzzle 2D and bridge locks also lose the unused aggregate DWG and DWG inferences package records. Hub and fixture-sweep retain the aggregate because other packages depend on it. The historical ticket probes were left intact.

Offline locked dependency resolution passed for Hub and the regenerated recorder probe. The Puzzle 2D workspace’s explicit `acadrust` test dependency is missing from the offline index. Bridge and fixture-sweep both need broader lock refreshes. Offline dry runs reported 132 and 571 additions respectively, including unrelated framework dependencies, and left the locks untouched beyond the scoped DWG edits.

The clone recorder now reads the destination host view once. It indexes node and handle identifiers, edge identifiers and labels for the kinds in the fragment. Accepted clones update those indexes in place, keeping subsequent labels and endpoints consistent while the typed recorder applies each mutation. The shared label allocator preserves the first authored root, accounts for unlabeled peers and handles a numeric catalog name on a kind's first two placements.

Both native core cases passed on the final source. They execute the production clone function with a scratch mutation sink and verify exactly one read of the destination host view, occupied identifiers, exact new endpoints and both label sequences. Eleven Bun tests passed, including the independent SQLite label calculation and the existing topology, naming and selection checks.

The standalone recorder build compiled through the native host and format crates. Its 3D editor dependency reached code generation after the shared Store binary comparison method had landed. The earlier Store build still held the preceding interface and rustc refused that dependency with E0599. The recorder is queued on the fresh Store build. The probe also exercises the real typed mutation and inverse replay for cut and paste.

Cargo’s locked unit graph for the recorder probe has 1,310 units and excludes the aggregate DWG and DWG inferences targets. The previously killed target, `semio-s-artifact-stdio-dwg-snapshot-objects`, remains in that graph. The shortest path runs through `semio-s-artifact-puzzle-2d-mut-any-10`, `semio-s-artifact-stdio-semio-drawing-export`, and `semio-s-artifact-stdio-dwg-snapshot`. The shared export crate is required by Puzzle 2D’s format routes. No code generation ran during this check. Dependency logs and the unit graph are under `🗑️generated/2d-dwg-*`.

### Production recorder without drawing export

The five serializer files are mounted in the `semio-s-artifact-puzzle-2d-text-inverse` mirror. The full artifact re-exports them through that mirror. No target in the `mut-any-10` dependency closure mounts those files. The only Rust references to the exporter in Puzzle 2D are the five format leaves. The sliced unit graph contains 1,307 units, including 15 direct exporter declarations from Puzzle 2D. Those declarations are in board-host, engine, graph-manifest, inferences, mut-any, mut-any-2, mut-any-3, mut-any-4, mut-any-10, mutations, records, schema, snapshot, tambour-end-storeys, and tambour-single-storey. Omitting the 15 edges in a graph simulation leaves 1,285 units and removes all 20 DWG units in the slice. Keep the exporter dependency in text-inverse for the public format routes.

The existing `verify-2d-clipboard.rs` can compile the production recorder directly with the mounts below. `mut-any-10` re-exports the existing records, diff and mutation constructors from the same authorities as text-inverse. The production recorder already imports the play snapshot and the relevant editor helpers from `mut-any-10`. Keep its existing fixture, placement, typed mutation replay and endpoint assertions. The fixture’s `handleNamespace` case supplies a destination with colliding node and handle IDs. This route exercises recorder behavior and typed application. Retained dispatch, mounting and UI publication still require the editor tests.

```rust
extern crate puzzle2d_model as semio_s_artifact_puzzle_2d_mut_any_10;
extern crate semio_os_kernel as protocol;
extern crate pack_json as semio_framework_pack_json;
extern crate value as semio_framework_value;

pub use puzzle2d_model::{standards, Puzzle2dSnapshot};

#[path = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🦀️.rs"]
pub mod actual_recorder;

pub mod editor {
    pub mod puzzle2d {
        pub use crate::actual_recorder as recorder;
        pub use puzzle2d_model::editor::puzzle2d::snapshot;
    }
}
```

The standalone manifest now uses relative paths for its dependencies and selects the existing OS kernel, infinite board, plugin and diagnostic packages alongside the model. The verifier mounts the actual recorder and its real editor snapshot. Cargo resolved and locked the revised manifest offline. Its compile plan contains 1,286 units, including the probe and `mut-any-10`. The plan contains zero DWG units and excludes both the drawing exporter and text-inverse. Hub’s locked dependency graph still includes the exporter through text-inverse. All four live locks reflect the 15 removed declarations. Cargo resolved the probe source path and the path attribute resolves to the production recorder file.

The native run can reuse the previously populated clipboard cache. From the repository root, set `generated` to the ticket’s absolute `🗑️generated` directory and run the following after coordinating access to the cache.

```sh
CARGO_HOME="$generated/cargo-home" \
CARGO_TARGET_DIR="$generated/cargo-puzzle-clipboard" \
CARGO_BUILD_BUILD_DIR="$generated/cargo-puzzle-clipboard-build" \
CARGO_PROFILE_DEV_DEBUG=0 RUSTC=rustc RUSTC_WRAPPER='' RUST_MIN_STACK=134217728 \
ZIG_GLOBAL_CACHE_DIR="$generated/zig-clipboard/global" \
ZIG_LOCAL_CACHE_DIR="$generated/zig-clipboard/local" \
cargo run --offline --locked -j1 --manifest-path "$generated/clipboard-2d-probe/Cargo.toml"
```

The unit graph verification ran without code generation. The probe has yet to compile and execute.

### PDF dependency contingency

The DWG-free probe still reaches the PDF model through `semio-s-artifact-stdio-pdf`. Its locked compile plan has 15 incoming edges to this aggregate from the same lower Puzzle 2D crates listed above. Puzzle 2D has no Rust references to the aggregate. Its actual PDF serializer calls the shared Semio drawing exporter, which imports the aggregate’s typed records and encoder. The Semio drawing importer and Hub’s PDF composition also use the aggregate.

Omitting only the 15 lower edges from the recorder’s unit graph reduces it from 1,286 to 1,223 units and makes all 61 PDF targets unreachable, including the PDF model. The four live workspace locks each have the same 15 edges and other aggregate consumers, so the scoped lock change would remove those references without dropping the PDF package records. The text-inverse serializer mounts and the shared importer and exporter edges are outside this cut. This was a read-only graph simulation while the native probe was compiling.

### PDF and peer editor pruning

The recorder compiled the PDF model and reached the peer 3D editor, where its warm build exposed a stale Store library. The same 15 lower Puzzle 2D crates that carried the unused PDF aggregate were the recorder plan’s only direct consumers of the 3D aggregate. No 2D Rust source references either aggregate. Puzzle 2D’s inference implementation and tests import two real diagram constants from `semio-s-artifact-puzzle-3d-inferences`.

The 15 lower manifests and four live locks now omit both unused declarations. Each live lock still has unrelated consumers of both aggregates. The real 3D inference imports and the shared drawing serializer dependencies remain. Cargo refreshed the standalone recorder lock and produced an offline locked plan with 1,133 units, down from 1,286. That plan retains the 3D inference target and has zero PDF and DWG targets. It excludes the 3D aggregate and peer editor. No code generation ran for the graph verification.

The preserved manifest bootstrap derives all seven production dependency paths from the Puzzle 2D workspace metadata. A clean generated-directory check resolved the same seven paths and the same authored probe entry point as the active recorder manifest. [Fallback bootstrap](./prepare-2d-native-fallback.py)

The pruned native build compiled the real 2D mutation and play snapshot libraries. The recorder probe then passed after switching its inverse fixture to the current `ColdMutation` trait. It linked the exact retained Rust fingerprints and mounted the current production recorder. It checked relative positions, scoped distinct endpoints, edge data, incident lock lookup, the allowed Cut, and exact restoration after replaying the Paste and Cut inverses. Serde independently decoded the shared input and expected results. [Recorder probe source](./verify-2d-clipboard.rs)
