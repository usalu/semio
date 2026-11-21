# Terrain GeoJSON Rust Edition

The WASI retry stopped in terrain inferences because the GeoJSON importer and exporter used two Rust 2024 let chains. The crate builds as Rust 2021.

Both conditions now filter their options before binding. A valid source ID enters the taken set only after passing the existing whitespace and length checks. An exported label still overrides source metadata only when it differs from the original label.

Pinned rustc parsed the complete GeoJSON source as Rust 2021 and the scoped diff passed its whitespace check. The managed build will verify the artifact with the full WASI dependency graph.
