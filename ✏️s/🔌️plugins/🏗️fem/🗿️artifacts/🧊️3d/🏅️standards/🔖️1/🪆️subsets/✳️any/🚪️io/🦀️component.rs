//! 🚪️ IO s.fem3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register(). `stdio.obj`/`stdio.stl` are
//! EXPORT-only (real geometry: `FemSolid` footprints, genuinely triangulated and extruded by
//! their own `height` — see `engine::meshing::build_semio_mesh_snapshot`); no honest IMPORT
//! exists (an arbitrary mesh carries no `FemMaterial`/`FemSection`/`FemSupport`/`FemLoadCase` to
//! reconstruct a `Fem3dSnapshot` from). `stdio.zip`/`stdio.png` were deleted outright in both
//! directions: fem3d has no real archive-bundle or raster-visualization capability to honestly
//! back them (see ticket w5a--report.md's `stdio_gaps`/rationale).
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.txt"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.md", "stdio.obj", "stdio.stl", "stdio.txt"] }
