# Per-crate before/after

Before: `🗑️generated/before-summary.tsv` (this ticket, first crates) or the previous round's notes; `—` = not run before. After: final sweep (`final-final-part-01..03`, `final2-part-00`, `final2-part-04a/b`), unit + integration + doc tests.

| Crate | Before | After |
|---|---|---|
| `semio-ui-tui-engine` | 4 pass / 4 fail | 8 pass |
| `semio-ui-tui-host` | 0 pass / 0 fail | 0 pass |
| `semio-ui-tui-frame` | 0 pass / 0 fail | 0 pass |
| `semio-ui-tui-vt` | 0 pass / 0 fail | 0 pass |
| `semio-ui-tui-vt-laws` | 79 pass / 0 fail | 79 pass |
| `semio-ui-tui-terminal` | 34 pass / 0 fail | 34 pass |
| `semio-ui-tui-widgets` | 8 pass / 0 fail | 8 pass |
| `semio-ui-tui-unit-laws` | 100 pass / 0 fail | 100 pass |
| `semio-ui-tui-laws` | 101 pass / 0 fail | 101 pass |
| `semio-ui-tui-core` | 59 pass / 1 fail | 60 pass |
| `semio-ui-locale` | did not compile | 17 pass |
| `semio-ui-retained-tree-tree` | 21 pass / 0 fail | 21 pass |
| `semio-ui-retained-tree` | — | 0 pass |
| `semio-ui-arena` | — | 4 pass |
| `semio-ui-theme` | — | 0 pass |
| `semio-ui-styling` | — | 1 pass |
| `semio-ui-action` | — | 30 pass |
| `semio-framework-text-splice-native` | did not compile | 4 pass (`a1-splice-14`); final sweep 1 pass / 3 fail, caused by another session's live `[DEBUG]` probes in `🌱️value/🛬️decode/🏭️construction/…/🎛️driver` (written 04:47 on 11-19) |
| `semio-ui-scene-surface` | — | 6 pass |
| `semio-ui-scene-world3d` | — | 22 pass |
| `semio-ui-scene-views` | — | 18 pass |
| `semio-ui-scene-doc` | — | 3 pass |
| `semio-ui-scene-node-graph` | — | 7 pass |
| `semio-ui-scene-plane` | 21 pass / 1 fail | 22 pass |
| `semio-ui-scene-modelling` | — | 16 pass |
| `semio-ui-scene-pass` | — | 4 pass |
| `semio-ui-scene-camera` | — | 40 pass |
| `semio-ui-scene-picking` | — | 43 pass |
| `semio-ui-scene-mesh` | 17 pass / 1 fail | 18 pass |
| `semio-ui-scene-algebra` | — | 2 pass |
| `semio-ui-scene-pack` | — | 11 pass |
| `semio-ui-scene` | — | 1 pass |
| `semio-ui-wgpu-engine` | lib test needs --features | 0 pass |
| `semio-ui-wgpu-engine-scene-laws` | did not compile | 14 pass |
| `semio-ui-wgpu-engine-laws` | did not compile | 106 pass |
| `semio-ui-wgpu-engine-surface` | — | 0 pass |
| `semio-ui-wgpu-events` | — | 0 pass |
| `semio-ui-wgpu-events-control-laws` | — | 14 pass |
| `semio-ui-wgpu-events-laws` | 88 pass / 1 fail | 89 pass |
| `semio-ui-prepared-staging` | — | 19 pass |
| `semio-ui-prepared` | — | 2 pass |
| `semio-ui-wgpu-mounted-layout` | — | 24 pass |
| `semio-ui-wgpu-mounted-layout-laws` | 1 pass / 2 fail | 3 pass |
| `semio-ui-wgpu-input` | — | 0 pass |
| `semio-ui-wgpu-input-laws` | — | 14 pass |
| `semio-ui-wgpu-controls` | — | 0 pass |
| `semio-ui-wgpu-paint` | — | 0 pass |
| `semio-ui-wgpu-paint-reference` | — | 0 pass |
| `semio-ui-wgpu-paint-laws` | — | 69 pass |
| `semio-ui-render-prepared-shaders` | — | 1 pass |
| `semio-ui-wgpu-device` | — | 3 pass |
| `semio-ui-draw-contract` | — | 23 pass |
| `semio-ui-wgpu-render` | — | 16 pass |
| `semio-ui-wgpu-draw-execution` | — | 23 pass |
| `semio-ui-wgpu-pipelines` | — | 1 pass |
| `semio-ui-wgpu-mesh` | — | 4 pass |
| `semio-ui-wgpu-raster` | did not compile | 32 pass |
| `semio-ui-wgpu-laws` | 36/45 lib, clipboard 0/2 | 48 pass |
| `semio-ui-wgpu-hosting` | needs --features | 44 pass |
| `semio-ui-wgpu-layout` | — | 37 pass |
| `semio-ui-wgpu-layout-laws` | did not compile | 16 pass |
| `semio-ui-wgpu-widgets` | — | 8 pass |
| `semio-ui-wgpu-widget-kit` | — | 4 pass |
| `semio-ui-wgpu-light` | — | 27 pass |
| `semio-ui-layout` | — | 31 pass |
| `semio-ui-declaration` | — | 37 pass |
| `semio-ui` | — | 3 pass |
| `semio-ui-text` | — | 30 pass |
| `semio-ui-border-presentation` | — | 6 pass |
| `semio-ui-render-shader-contract` | — | 11 pass |
| `semio-ui-backend-vulkan` | — | 0 pass |
| `semio-ui-backend-metal` | compile break under feature unification | 5 pass |
| `semio-ui-backend-webgpu` | — | 14 pass |
| `semio-ui-backend-d3d12` | — | 0 pass |
| `semio-ui-render` | — | 32 pass |
| `semio-ui-render-packet` | — | 42 pass |
| `semio-ui-render-device` | — | 1 pass |
| `semio-ui-render-interaction` | — | 44 pass |
| `semio-ui-render-text` | — | 18 pass |
| `semio-ui-host` | — | 13 pass |
| `semio-ui-host-tests` | — | 60 pass |
| `semio-ui-raster-owner` | — | 11 pass |
| `semio-ui-runtime-reconcile-cursor` | — | 32 pass |
| `semio-ui-runtime-reconcile` | — | 36 pass |
| `semio-ui-runtime-exchange` | — | 13 pass |
| `semio-ui-runtime` | — | 17 pass |
| `semio-ui-runtime-presentation` | — | 24 pass |
| `semio-ui-runtime-reconcile-primitives` | — | 3 pass |
| `semio-ui-testing-paint-policy` | — | 0 pass |
| `semio-ui-prepared-laws` | — | 38 pass |
| `semio-ui-contract-operations` | — | 15 pass |
| `semio-ui-contract-identity` | — | 1 pass |
| `semio-ui-contract-values` | blocked by source boundary | 1 pass |
| `semio-ui-contract-values-laws` | — | 43 pass |
| `semio-ui-contract-actions` | — | 5 pass |
| `semio-ui-contract-builder` | — | 29 pass |
| `semio-ui-contract-budget` | — | 1 pass |
| `semio-ui-contract` | — | 32 pass |
| `semio-ui-contract-text` | — | 4 pass |
| `semio-ui-contract-presentation` | — | 42 pass |
| `semio-ui-contract-validation` | — | 41 pass |
| `semio-ui-contract-components` | — | 3 pass |
| `semio-ui-contract-conformance` | 38 pass / 5 fail | 43 pass |
| `semio-ui-contract-typegen` | generated input missing | 1 pass |
| `semio-ui-contract-text-edit` | — | 7 pass |
| `semio-ui-select-behavior` | — | 19 pass |
| `semio-ui-surface-identity` | — | 1 pass |
| `semio-ui-viewport` | — | 12 pass |
