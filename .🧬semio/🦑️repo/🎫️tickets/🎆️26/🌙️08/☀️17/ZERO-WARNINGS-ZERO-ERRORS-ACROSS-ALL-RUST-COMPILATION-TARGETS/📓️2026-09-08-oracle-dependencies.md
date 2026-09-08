# Oracle Dependencies After Artifact Extraction

Pass550 adds the existing Geo oracle as a GIS map dev dependency and the existing framework async test executor as a FEM 2D dev dependency. GIS binary tests use document_dsl for document parsing. The region-membership corpus now resolves relative to its test source; realpath verified the unchanged fixture file. The artifact test no longer tries to call a nonexistent parent plugin module. The existing plugin surface identity law already assembles the full plugin and verifies both artifacts and all four app roles, while the artifact law retains service registration and declaration assertions.

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs`

Pass551 preserves the existing Block 3D and 5D Serde comparisons across the extracted Block 2D dependency boundary. Eight shared record types expose their existing oracle derives and field attributes through a test-serde feature. The feature is enabled only by the dependent crates' dev dependency declarations; default features remain empty. No runtime serialization API or record fields changed. 35 existing conditional attributes were extended in the shared-record file.

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🧬️schema/🧱️shared/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/Cargo.toml`

Native532 contains the failing cross-crate Serde and missing oracle dependency diagnostics. Fresh syntax, metadata, compiler and runtime validation remain required.


Pass552 validation of passes550,551 and553: metadata exit 0, 0 Cargo warning lines, and 7/7 Rust files parsed with unchanged source hashes. Compiler and runtime verification remain required.
