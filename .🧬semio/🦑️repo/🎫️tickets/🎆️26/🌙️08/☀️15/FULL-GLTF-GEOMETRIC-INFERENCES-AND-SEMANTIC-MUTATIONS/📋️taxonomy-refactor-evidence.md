# GLTF Taxonomy Refactor Evidence

## Outcome

The GLTF inference and mutation implementations now follow the schema taxonomy instead of accumulating unrelated behavior in bounds and set-snapshot.

## Inference Taxonomy

- Removed the obsolete 💡️inferences/📦bounds Rust and TypeScript leaves.
- Moved the aggregate contract and computational kernel to 💡️inferences/📐️geometry.
- Moved the reusable typed measurement contract to 💡️inferences/🧾️measure.
- Added paired Rust and TypeScript leaves for the fourteen independent indicator groups:
  📦️size, 🧱️area-volume, ⚪️compactness, 📏️proportion,
  ⚖️mass-distribution, 🌀️curvature, ↕️thickness, 🕳️concavity,
  ↔️clearance, 🔗️adjacency, 🧭️orientation, 🪞️symmetry,
  🌊️roughness, and 🕸️topology.
- The inference root is now assembly and reexports; it does not own geometry algorithms.

## Mutation Taxonomy

- All 28 frozen mutation variants have a named semantic command folder.
- Every command folder owns the same complete six-leaf triad:
  Rust and TypeScript 🦠️mutation, 🔺️diff, and ↩️inverse.
- The closed root enum uses named payload types and preserves frozen tags 0..27.
- Cross-command reference validation, remapping, and diff planning live in 🧬️mutations/🧭️planning.
- Text and binary codecs live in the existing 📝️text and 💾️binary codec folders.
- 📄set-snapshot contains only set-snapshot mutation, diff, and inverse behavior.

## Integration

- The Rust glue module mounts every inference group and every mutation command triad.
- TypeScript roots reexport the same taxonomy.
- libz-sys is now an explicit stdio Rust dependency because the existing stdio deflate implementation calls it directly; this removes an undeclared-build dependency exposed by the clean rebuild.

## Verification

- rustfmt over all GLTF Rust component leaves: passed.
- TypeScript runtime import of both GLTF roots: passed.
- Focused TypeScript tsc no-emit: passed.
- Deterministic conformance audit: passed with 67 indicators, 14 groups, geometry-only inference root, 28 mutations, and tags 0..27.
- cargo test -p semio-s-plugin-stdio --no-run: passed with RUSTC_WRAPPER cleared to avoid the restricted sandbox sccache.
- SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/stdio-plugin:test-quick -- gltf: passed; 91 tests run, 91 passed, 3367 skipped.
- Scoped git diff --check: passed.

No AGENTS.md file or unrelated ticket target was edited. No modifying git command was used.
