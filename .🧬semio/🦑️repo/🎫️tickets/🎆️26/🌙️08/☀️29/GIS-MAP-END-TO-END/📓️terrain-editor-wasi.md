# Terrain Editor WASI Compilation

The managed compile reached the GIS terrain editor and stopped on three diagnostics. The terrain data panel called an internal label helper through the plugin facade, which does not export it. The elevation-grid action closure also needed a concrete type before applying unary negation to the maximum coordinate.

The panel now uses the public bounded UI label conversion and keeps the existing panel-stage refusal. The elevation-grid closure declares its limits and defaults as `f64`, matching the numeric action schema. Both files parse as Rust 2021 and the scoped diff check passed. Full compilation remains with the managed GIS build.
