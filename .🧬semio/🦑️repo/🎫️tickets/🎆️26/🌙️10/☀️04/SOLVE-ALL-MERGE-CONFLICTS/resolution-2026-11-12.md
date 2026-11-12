# Resolution 2026-11-12

Stash conflict markers (`Updated upstream` / `Stashed changes`) were in nine TypeScript files. Each resolution keeps the current axonometric and environment wiring together with the stashed level-of-detail behavior.

- Contracts import both `PlanarViewpoint` and `ShapeRole`. `ExportFormat` stays unused here and is not imported.
- Catalog load takes the optional `only` asset-folder set the body already filters on.
- Depiction keeps the axonometric camera, section markers on elevation and plan, and axonometric dimensions, and filters drawn planar shapes by `options.roles` while extents still reach every shape.
- Transport parcels use the camera for nearness and the transport-density stride.
- Revision imports both the portable cube root and `LodId`.
- Cast export imports axonometric loading and the logical-extent helpers. An axonometric cast measures its level-independent figure on the side elevation, because `prepare` is planar.
- Cast-export tests keep the current manifests, including the rejection of transport layers without `transportAgeS`.
- Plan-projection asserts `logical.heightM` (the index field) against both the side-elevation figure and the plan depth.
- The energy authoring script keeps the pinned Python environment, UTF-8, tool path and build constraint, and sets `SEMIO_TEACHING_PROFILE` to `final` for tests.

`repo tools exec bunx -- tsc --noEmit` on `@teaching/staffage` exits 0. The `<<<<<<< ANCHOR` blocks under the non-destructive history-editing ticket are fixture text, not merge conflicts, and were left in place.

GitKraken still listed the nine files because the index kept all three stages. Those paths are now staged as resolved (`git add` of the working tree only). `git ls-files -u` is empty and no merge is in progress. Nothing was committed.
