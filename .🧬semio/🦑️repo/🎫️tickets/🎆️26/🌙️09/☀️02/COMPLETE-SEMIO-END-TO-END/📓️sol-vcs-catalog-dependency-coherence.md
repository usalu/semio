# VCS Catalog Dependency Coherence

## Recovered failure

Hub catalog build receipt `exact-cargo-laws-Gt5a6S/00` reached `semio-s-plugin-vcs` and stopped with three dependency diagnostics: `VcsDemoConfigMutation` and `VcsDemoPresenceMutation` lacked the required static mutation roster/instance descriptor mapping, and the history window referenced `semio_framework_ui_scene::GraphTimelineScene` without declaring that first-party scene crate.

## Repair

- Added exact, nonempty per-variant mutation descriptors for config snapshot, locale change, and presence noop, including truthful inversion, diff, outcome, composition, and required-language metadata.
- Bound each aggregate variant to its exact descriptor and extended the native config/presence tests to prove the roster and instance mapping.
- Declared the existing first-party `semio-framework-ui-scene` dependency and imported `GraphTimelineScene` through that dependency before passing it to the existing typed `scene_surface` encoder.
- Extended the registered VCS native-codec oracle with a neutral dependency-coherence check. Its oracle-only Nx execution is GREEN: `receipts=1 hostile=9 ajv+node+webcrypto=1 dependency-coherence=3`. This is source/schema evidence; the next Hub/VCS native build remains the compile/runtime authority.
