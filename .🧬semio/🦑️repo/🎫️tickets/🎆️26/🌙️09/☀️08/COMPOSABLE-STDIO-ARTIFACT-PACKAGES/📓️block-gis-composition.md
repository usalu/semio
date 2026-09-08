# Block and GIS Composition

## GIS

The GIS plugin manifest now mounts its taxonomy-root composition source. The package-local Rust implementation mount tree was removed. The plugin and native-codec receipt registry refer directly to the map and terrain artifact crates, each with its app assembly feature enabled. The native stdio catalog remains an explicit plugin-level contribution because it assembles the complete codec catalog. Unused artifact-specific runtime dependencies were removed from this composition manifest.

The existing language-neutral artifact identity fixture, independent serde_json oracle, native-codec receipt tests, and component cold-map-patch integration test remain owned by the plugin integration boundary. Artifact examples and document tests are being mounted by the artifact execution owner in each leaf.

Validation pending: Cargo resolution, GIS component compile, fixture-based plugin assembly runtime test.

## Block

The Block plugin now mounts its taxonomy-root app enum, registration and surface tests. Its three artifact dependencies activate their app assembly features; the plugin invokes each generic artifact declaration with the parent `BlockApps` enum. Shared kind, metadata, author, representation and camera records are canonically owned by the block-2d artifact. The package-local monolithic Rust mount tree was removed. The optional entry symbol is default-enabled through `plugin-entry`. Component compiler and runtime surface checks remain pending.

## Files

- `✏️s/🔌️plugins/🌍️gis/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`
- Removed `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/🦀️.rs`

Additional Block files: `✏️s/🔌️plugins/🧱️block/🦀️.rs`, `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`, removed `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`.

## GIS Leaf Integration Corrections

The component compiler reached Block and GIS after compiling the selected full stdio catalog, then found five Block schema-alias errors and fourteen GIS source-boundary errors. The final dependency spelling is `semio-framework-schema`, matching the derive macro’s emitted `::semio_framework_schema::` paths. Handwritten framework references use that canonical absolute crate path; `crate::schema` remains the artifact domain module. No duplicate external-crate alias item is declared.

The coordinator repaired both GIS leaf duplicate snapshot exports, three mechanically corrupted Semio base-geometry import paths, and the map artifact's missing Surface dependency. Surface is optional for map app assembly and uses its prior no-session-bindgen selection in both GIS leaves, preventing duplicate component session exports. Unused direct framework geometry dependencies were removed. GIS check 3 isolated the canonical derive dependency spelling, which was repaired in both leaf manifests and sixteen mounted source files. Check 4 failed in shared PNG/glTF mutation derives while reading an empty authority JSON file; the current project and taxonomy documents both parse successfully. The current combined Block/GIS retry is `block-gis-composition-check-5.txt`; its result is pending.

Additional files are both GIS artifact roots and manifests plus gismap `🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema,🚪️io}/🦀️.rs`.

## Example Ownership

Removing the old GIS parent mount tree must preserve its two editor demo-session implementations. Both now have canonical mounts in the corresponding artifact editor examples module, with their existing tests. The two existing test asset includes were corrected to reach the adjacent example asset directory. The plugin no longer mounts those artifact implementations independently. Additional files are the gismap and gisterrain `✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️.rs` files, plus both artifact roots.

The current composition check also restored the `protocol`/`store` aliases required by the closed-app dispatch macro. Terrain selects its editor framework dependency only through component app assembly. Map's retained initializer now names the canonical kernel fault types, allowing its framework dependency to be optional and selected only for its editor/viewer feature. Existing schema use of Surface terrain tiles remains a real default dependency.

## Persistent GIS Law Routing

The two GIS native law groups now name the map artifact directly; the editor law explicitly enables component-app-assembly. Their exact test names include the taxonomy mounting component module. The map group oracle now inspects its extracted test source for neutral membership coverage, and the durable assembly oracle checks the current owned optional-sink expression. The map group oracle passed 26 checks; the durable assembly oracle passed AJV plus Node/WebCrypto hash agreement, three roles, four cancellation cases and three rejection cases. Native law execution is still pending the old Cargo queue.

Additional coordinator-owned test routing and fixture-reference files:

- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗺️mutate-gismap-1/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏔️mutate-gisterrain-1/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs`
