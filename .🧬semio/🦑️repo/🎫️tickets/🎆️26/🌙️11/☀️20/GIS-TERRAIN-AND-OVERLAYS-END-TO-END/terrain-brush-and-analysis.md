# Terrain Brush Settings

The GIS Terrain editor registers Configure Terrain Brush as an s view action. It publishes a typed, reversible window configuration change through the same scoped Store path as camera changes. Each terrain window owns its own radius, elevation step, and smoothing strength. A document edit still uses the registered Edit Elevation mutation so the artifact publishes the diff and records the change in document history.

The plugin supplies the brush action and arguments in the World3d interaction lane. Both renderers read the first-party contract, get the sample location from the terrain surface, and supply its row and column. Native coverage includes using the declared action and parameters and rejecting a camera drag.

The first-party grid-edit arithmetic is owned by Terrain inferences. An isolated Rust probe of the production arithmetic passed seven edit cases and three refusals from the neutral fixture. The probe replaced the artifact and error carriers with minimal stand-ins. The same vectors also passed the independent math.js and Ajv checks.

The browser contract passed all ten neutral action vectors against Ajv. The window schema contract passed six accepted and nine rejected configurations. Four terrain host tests and six independent Three.js ray cases passed. Rust parsing passed for the edited command, configuration, diff, mutation, and test sources. The mounted window persistence and new Rust diff tests are pending the package build.
