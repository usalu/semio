# Process Extension Proof

The editor's `process3d-play` payload ID is its internal contribution filter; `s.process.process3d@1/*#editor` is the canonical editor command address. They serve different purposes.

The previous demonstrator pack fixture serializes model catalogs and does not read the four published descriptors. The descriptor test reads all four actual components, distills their contributions, and compares every delivered machine and capability against the editor's installable catalog using `serde_json` as an independent decoder. The generic catalog is built in; optional catalogs come from active contributions.

The editor test also pushes the descriptor contributions to a registry-backed editor, adds a machine from each catalog to one document, and adds a step using each machine's capability. The dev-route test separately exercises `readGeneratedCatalogProjection` and `buildPlaygroundSession` with the four contributors and their consumer.

The first native editor test run failed before reaching the editor on `semio-plugin-tool-machine` errors E0109 and E0618. Its admitted sources preceded the BRep refactor and the final test edits. Two subsequent runs were cancelled after their admitted dependency graphs were superseded by BRep and tessellation manifest edits. A coordinated current-source run owns the native checks.

The mounted descriptor test checks that removing an optional extension removes its catalog, preserves the authored workshop and steps, and prevents adding the removed provider's machine again. Saved process examples and authored documents embed workshop machines and capabilities. This native test has not yet completed.
