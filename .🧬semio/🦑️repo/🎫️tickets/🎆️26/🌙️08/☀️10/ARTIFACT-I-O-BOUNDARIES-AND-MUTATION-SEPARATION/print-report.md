# Chart Artifact I/O Boundaries

The Print chart semantic schema now contains its snapshot and mutation meanings without text, binary, or SQLite codec mounts. Native operation traits and streaming binary helpers live under I/O text/binary mutation facets. Document traits live under their snapshot facets. The complete SQLite provider, controlled native helpers, SQL schema, and source/native test suites live under I/O SQLite snapshot.

A neutral ownership fixture and feature verify the separation with an independent minimatch path oracle. The existing SQLite source suite imports that ownership regression through its registered Nx route.

`bun nx run @semio-tech/print-rs:test-snapshot-sqlite-source` passed: eight tests, no failures, 173 assertions. Runtime debug output confirms complete authored states, independently queried SQLite relationships, scalar edits, surrogate renumbering, cancellation, malformed companion refusals, and exact IEEE words. The suite uses independent Ajv, D3 numeric checks, and Bun SQLite.

`bun nx run @semio-tech/print-rs:test-snapshot-sqlite-native` is running. Compilation and native outcomes remain unconfirmed until the command completes.
