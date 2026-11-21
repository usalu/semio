# GIS Terrain Mutation Build Repair

The managed WebAssembly build stopped in the terrain operations crate before linking the GIS component. Three invalid mutation branches called the current diagnostic constructor without its target path. The elevation replacement payload lacked a canonical JSON tree for its nested authored source.

The shared terrain source now supplies `exaggeration`, `importedMap`, and `elevationSource` as the diagnostic paths. It derives `CanonicalJsonTree` for `ElevationSource` through the existing Pack JSON owner. I verified these changes in the live files before editing. The production repairs were already present, so my changes add only test coverage and its fixture.

The operations tests verify that all three invalid mutations produce one field-targeted diagnostic, refuse admission, and yield an empty diff. The neutral JSON fixture fixes the diagnostic contract. A second test traverses the borrowed canonical tree for the existing authored elevation fixture and checks every bound and sample against serde_json. I checked the Rust syntax and the fixture paths and dimensions. The native Cargo run was deferred to reserve compiler capacity for the managed GIS launch.
