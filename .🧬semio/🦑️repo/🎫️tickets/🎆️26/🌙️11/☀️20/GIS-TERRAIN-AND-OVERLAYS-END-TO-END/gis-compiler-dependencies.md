# GIS compiler dependency audit

The scoped GIS editor compiles DWG through unused direct dependencies in the Terrain component manifests and the GIS Map package and terminology package. The Terrain root package has the same unused declaration. The project source uses Semio Mesh and the typed GeoJSON and ASCII codecs.

The Hub's full stdio plugin owns DWG registration and uses it in the CAD extension. That dependency belongs in the full Hub composition. The narrow GIS path can drop its unused DWG declarations and regenerate the affected workspace dependency lists with the repository lock tool.

The mesh exporter uses glTF, STL, LAS, OBJ, and PLY. Those formats remain reachable through the Semio Mesh export API. The current scoped compiler is still working in the captured dependency graph.

Once the GIS Map membership roster used its three own Store members, the remaining path to the full Semio aggregator was `semio-hub-gis-component` → `semio-hub-gis` → `semio-s-artifact-gis-gismap` → `semio-s-artifact-gis-gismap-terminology-tests` → `semio-s-artifact-stdio-semio`. The GIS Map library declares the terminology test package under regular dependencies. The Rust source search found no use of that package from the GIS Map library. The terminology crate has its own workspace package.

A conservative manifest traversal included normal, build and target-specific path edges. It found 916 reachable workspace packages. Omitting the GIS Map terminology dependency left 793 and removed the full Semio aggregator from the closure. Omitting the unused GIS DWG edges as well left 791. The GIS Map library now omits that ordinary dependency. A Bun TOML check confirmed the workspace still contains the standalone terminology test package. Cargo’s offline workspace update refreshed the Hub, GIS Map and GIS Terrain lockfiles. The GIS Map standalone terminology test remains in its own workspace lock. The Terrain lock removes 123 local packages and preserves the third-party package roster. Cargo reported zero newly resolved packages for Hub and GIS Map. The unused DWG declarations remain in place.
