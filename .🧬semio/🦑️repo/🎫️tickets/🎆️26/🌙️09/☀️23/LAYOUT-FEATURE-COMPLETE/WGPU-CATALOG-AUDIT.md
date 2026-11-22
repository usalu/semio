# WGPU Layout Catalog Audit

The port 6179 console reports `plugin.descriptor-unavailable` for the animate descriptor. The specific WGPU composition `✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts` supplies the complete composed catalog and native catalog to `createCompletedWgpuConfigurationV1`. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts` injects both into HTML; the selected Layout activation controls staged artifacts, not this inventory.

`🚀️browser-boot/🟦️.ts` selects variant mode when the boot descriptor names a variant but supplies no explicit plugin modules. `🌐️browser-host/🟦️.ts` forwards that absence. `🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts` substitutes every catalog target and fetches every descriptor before invoking `PlaygroundBootPlanner`. Consequently animate fails before the Layout variant can be selected. The separate native catalog is installed in the renderer before this step.

The generated native catalog has Layout as a non-host plugin and its playground specifies app `s.layout.layout@1/*#editor`. Its catalog planner therefore selects the Layout dependency closure rather than host mode. Plan the implicit variant against admitted catalog rows before fetching descriptors, then keep the descriptor-based graph validation; preserve explicit modules and all-selection semantics. Missing or inconsistent descriptor dependencies must remain errors.

Existing language-neutral vectors live at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🧩️plugin-modules/🔣️.json`; `🧪️tests/🪆️embedded-mount/🟦️.ts` exercises those vectors against the independent graphlib oracle and descriptor preparation.
