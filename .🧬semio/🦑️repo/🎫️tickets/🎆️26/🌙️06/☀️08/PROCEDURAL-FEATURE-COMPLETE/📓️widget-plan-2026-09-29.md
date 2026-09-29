# Widget Contract and Mesh Inspection Plan

The installed BRep extension covers nearly every public BRep kernel operation and already exposes indexed mesh creation, transforms, cutting, repair, conversion and analysis. The procedural catalogue enumerates the registered roster through windowed groups. Remaining gaps include mesh component inspection and ambiguous port codes and abbreviations derived from similar channel names (for example grid pattern dirX/dirY).

This slice owns the BRep extension mesh module, its shared fixtures and tests, BRep metadata helpers and packaged descriptor parity. The coordinator owns inspector and selected BRep UX, and the catalogue executor owns catalogue/add-widget interaction.

1. Author portable fixtures and a query schema for indexed vertex, halfedge and face inspection.
2. Write Rust widget dispatch tests and TypeScript fixture/oracle tests first. Confirm failing tests through Bun/Nx.
3. Implement mesh vertex-position, edge-endpoints/length and face vertex-list/normal/centroid widgets using existing kernel methods and independent TypeScript equivalents.
4. Require every registered port to declare value schemas and unique code, abbreviation, name and full name within its direction. Test packaged metadata against live registration.
5. Run existing procedural TypeScript and BRep extension Rust Nx targets. Record actual outcomes here, preserving unrelated concurrent edits.

Face centroids mean the arithmetic mean of polygon corners, explicitly stated in descriptions. Face normals use the polygon Newell normal and reject zero-area input. Edge identifiers are preview halfedge ids, matching existing selection conventions. These widgets inspect polygon data and claim no analytic BRep fidelity.

## Implemented Slice

- Added three query widgets for vertex position, halfedge endpoints/length, and polygon corner indices/normal/corner mean, with portable query schema and concave polygon fixture. Independent Three.js Vector3, Line3 and polygon normal computations verify the TypeScript counterpart.
- Added nine modeling widgets wrapping validated kernel methods: bevel, dissolve edges, dissolve vertices, merge vertices, proportional move, grid snap, one-sided mirror, shortest-edge decimation, and coplanar face merge. Descriptions state convex bevel limits, one-sided mirror semantics, and decimation's approximate early-stop behavior.
- Typed formerly undeclared BRep channels, including topology lists, transform vectors, serialized text, closest-parameter numbers, and geometry outputs. Ambiguous shorthand expands only for collisions, preserving unambiguous concise identifiers.
- Added schema/oracle coverage for channel metadata and native/package manifest equality; the latter catches omitted registrations and stale defaults as well as missing types.
- Native inspection dispatch passed through the Nx BRep target. Initial TypeScript red run failed on the missing implementation as expected. Modeling red dispatch failed on unregistered bevel as expected. Latest full native/package verification remains in progress.

Nx daemon graph calculation hung during the concurrent workspace runs. Reissued the affected owned commands with NX_DAEMON=false and NX_ISOLATE_PLUGINS=false; no shared daemon or other developer processes were stopped.
