# Wave 0 R2-C Media Consumers

## Outcome

The procedural, GIS, and S Studio plugin roots consume the frozen R2-A typed builder surface directly. Every root remains fallible through `try_build()`; no setup callback or global mutable registrar remains in this lane.

| Owner | Frozen declaration | Removed legacy path |
| --- | --- | --- |
| `🌀️procedural` | `HostMediaHandlerDeclaration::mesh_dwg_bridge(...)` | `register_dwg_mesh_bridge`, `ensure_linked_flow_extensions`, callback installers, contribution JSON bag, contribution command, flow-extension dependency aliases |
| `🌍️gis` | `HostMediaHandlerDeclaration::two_d_svg_export(...)` | `gismap::io::register_host_io` and root setup registration |
| `🪐️space` | `foreign_document_codec::<SpaceApp>(OS_SPACE_SCHEMA)` | `register_s_exports` and root setup registration |

Procedural's former linked extensions supplied immutable metadata and native executable identity, so they are now seven direct `FlowExtensionDeclaration::new(...)` builder entries: brep, math, primitive/core, logic, dictionary, list, and text. No callback fallback was required.

The procedural 2D/3D config leaves now return pure `app_schema_descriptor()` values, and their `ArtifactApp::app_schema()` overrides are authoritative. GIS and S Studio already follow that same descriptor pattern and were verified. The procedural contribution field was removed consistently from Rust, TypeScript, GraphQL, Proto, and JSON Schema artifact/config/diff mirrors.

## Source Validation

- Legacy-symbol scan returned zero matches for `.setup(`, void media/config registrars, procedural ensure/sync paths, extension aliases, and contribution JSON/mutation names in the three owned plugin trees.
- Confirmed builder declarations: one procedural mesh-DWG handler, seven procedural flow extensions, one GIS 2D-SVG handler, and one S Studio foreign document codec.
- `rustfmt --edition 2021 --check` passes for the three edited plugin roots after formatting them.
- The three edited procedural JSON Schemas parse successfully with Bun.

## Deferred Integration Validation

Cargo and Nx checks were intentionally not run. The inference lane owns the active workspace/Cargo transition; integration validation resumes only at that barrier.
