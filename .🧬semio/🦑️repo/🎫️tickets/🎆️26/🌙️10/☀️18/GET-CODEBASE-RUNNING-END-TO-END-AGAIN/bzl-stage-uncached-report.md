# Unbounded cache declarations (cache without outputs) for manifest owners

Source: `bazelUncachedReportV1` over the static whole-graph harness (1,466 projects, 8,021 task rules). Rule: `cache: true` (or plugin class `deterministic`) with no `outputs` array is `unbounded` and is NOT cached by the Bazel adapter (fail-closed, consistent with bzl-label). Fix per target: add the real `outputs`, or an explicit `outputs: []` for a pure check (stays cacheable).

- actions 8021, cached 3052, uncached by reason: {"continuous": 1308, "declared": 355, "driver-owned": 1560, "live": 624, "side-effecting": 313, "unbounded": 809}
- unbounded targets: 809 in 278 projects; declared via `cache: true`: 809, via plugin class only: 0

## Most frequent target families

| family | targets |
|---|---|
| `test` | 608 |
| `verify` | 142 |
| `mutation` | 14 |
| `check` | 8 |
| `source` | 5 |
| `generate` | 3 |
| `native` | 3 |
| `metadata` | 3 |
| `child` | 2 |
| `tiles` | 2 |
| `mcp` | 2 |
| `io` | 2 |
| `conformance` | 1 |
| `shared` | 1 |
| `utility` | 1 |
| `add` | 1 |
| `geometry` | 1 |
| `semantic` | 1 |
| `catalog` | 1 |
| `composition` | 1 |
| `jack` | 1 |
| `preview` | 1 |
| `port` | 1 |
| `untrusted` | 1 |
| `package` | 1 |

## Dangling `dependsOn` (refused by the planner)

- `@semio-tech/gis-gismap-rs:verify-inference-mcp` depends on `semio-hub-gis-component:component` (project absent)
- `@semio-tech/gis-plugin:inference-discovery-check` depends on `semio-hub-gis-component:component` (project absent)
- `@semio-tech/gis-plugin:inference-discovery-oracle` depends on `semio-hub-gis-component:component` (project absent)
- `@semio-tech/s-dev-composition:typecheck` depends on `@semio-tech/trinity-jack-lsp:wasm` (project absent)

## Per project (root, targets)

- `@semio-tech/animate-presentation-rs` (`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-presentation-document-contract`
- `@semio-tech/architect-js` (`✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript`): `test`
- `@semio-tech/architect-program-rs` (`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/📦️packages/🦀️rust`): `check-register-catalog`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-snapshot-sqlite-source-profile`, `verify-architect-window-ownership`, `verify-physical-codecs`, `verify-program-document-contract`, `verify-snapshot-sqlite-source`
- `@semio-tech/assets` (`🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`): `check-generated`
- `@semio-tech/bim-model-rs` (`✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust`): `test`, `test-complete-sqlite-native`, `test-complete-sqlite-source`, `test-costs-source`, `test-snapshot-sqlite`, `verify-snapshot-sqlite`
- `@semio-tech/block-2d` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/block-2d-rs` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/block-3d` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript`): `test`, `verify-physical-codecs`
- `@semio-tech/block-3d-rs` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/block-5d` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/block-5d-rs` (`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-snapshot-sqlite-source`
- `@semio-tech/block-catalog-rs` (`✏️s/🔌️plugins/🧱️block/🗂️catalog/📦️packages/🦀️rust`): `conformance`, `test`
- `@semio-tech/block-js` (`✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript`): `shared-schema`, `test`
- `@semio-tech/block-shared-rs` (`✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust`): `test`
- `@semio-tech/cad-cad-rs` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-activation-source`, `verify-cad-document-contract`, `verify-owned-target`, `verify-reference-owner-syntax`, `verify-snapshot-sqlite-source`
- `@semio-tech/cad-extension-aec-building-energy-rust` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust`): `test`
- `@semio-tech/cad-extension-aec-building-rust` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust`): `test`
- `@semio-tech/cad-extension-aec-building-structure-rust` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust`): `test`
- `@semio-tech/cad-extension-spatial-shape-rust` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust`): `test`
- `@semio-tech/cad-js` (`✏️s/🧑‍💻dev/📐️cad/📦️packages/🟦️typescript`): `test`
- `@semio-tech/cad-js-module-aec-building` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript`): `test`
- `@semio-tech/cad-js-module-aec-building-energy` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript`): `test`
- `@semio-tech/cad-js-module-aec-building-structure` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript`): `test`
- `@semio-tech/cad-js-module-spatial-shape` (`✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript`): `test`
- `@semio-tech/dag-dag` (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🟦️typescript`): `verify-snapshot-sqlite-source`
- `@semio-tech/dag-dag-rs` (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-dag-document-contract`
- `@semio-tech/dag-js` (`✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript`): `test`
- `@semio-tech/demonstrator-js` (`✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🟦️typescript`): `test`
- `@semio-tech/demonstrator-playground` (`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🟦️typescript`): `verify-snapshot-sqlite-source`
- `@semio-tech/demonstrator-playground-rs` (`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/draw-drawing-rs` (`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust`): `test`, `test-inspector-selection-actions`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-drawing-canvas-window-ownership`
- `@semio-tech/draw-js` (`✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript`): `test`, `utility-action-policy`
- `@semio-tech/energy-js` (`✏️s/🔌️plugins/🔋️energy/📦️packages/🟦️typescript`): `test`, `test-toolchain-authority`
- `@semio-tech/energy-model` (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🟦️typescript`): `test`
- `@semio-tech/energy-model-rs` (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust`): `test`, `test-document-history-native`, `test-optical-field-intent-native`, `test-optical-inspector-native`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-batch-native`, `verify-batch-source`, `verify-enclosure-native`, `verify-enclosure-source`, `verify-publication-native`, `verify-publication-source`, `verify-results-native`, `verify-results-source`, `verify-solar-native`, `verify-solar-source`, `verify-thermal-native`, `verify-thermal-source`, `verify-weather-native`, `verify-weather-source`
- `@semio-tech/energy-model-state` (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state`): `verify-native`, `verify-source`
- `@semio-tech/fem-2d` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/fem-2d-rs` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust`): `test`, `test-fixed-dofs-native`, `test-fixed-dofs-source`, `test-material-native`, `test-material-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-fem2d-invariants`, `verify-fem2d-invariants-source`, `verify-fem2d-mounted-native`, `verify-fem2d-numerical-native`, `verify-fem2d-presence-native`
- `@semio-tech/fem-3d` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/fem-3d-rs` (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`): `test`, `test-material-native`, `test-material-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-fem3d-numerical-child-native`, `verify-fem3d-numerical-child-source`, `verify-fem3d-results-native`, `verify-fem3d-results-source`, `verify-fem3d-transform-intent`, `verify-fem3d-transform-intent-source`
- `@semio-tech/fem-js` (`✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript`): `test`, `verify-combination-stress`
- `@semio-tech/fem-material-rs` (`✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/📦️packages/🦀️rust`): `test-native`, `test-source`
- `@semio-tech/fem-node-rs` (`✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/📦️packages/🦀️rust`): `test-native`, `test-source`
- `@semio-tech/flow-extension-bim-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-brep-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-core-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-dictionary-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-draw-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-list-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-logic-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-math-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-extension-text-rust` (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust`): `test`
- `@semio-tech/flow-flow` (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🟦️typescript`): `test`
- `@semio-tech/flow-flow-rs` (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust`): `add-widget-retained-check`, `child-edit-check`, `child-identity-check`, `test`, `test-contributions-receiver`, `test-contributions-syntax`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-source`, `verify-flow-window-ownership`
- `@semio-tech/flow-js` (`✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript`): `test`, `test-command-retirement`
- `@semio-tech/forms-forms-rs` (`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-forms-document-contract`
- `@semio-tech/forms-js` (`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🟦️typescript`): `test`, `verify-physical-codecs`
- `@semio-tech/framework-graph` (`🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust`): `check-generated`
- `@semio-tech/framework-os` (`🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`): `check-wgpu`, `generate-wgpu`
- `@semio-tech/framework-os-dev` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`): `check-distribution`, `generate-distribution`, `generate-scale-fixture`
- `@semio-tech/gis-gismap-js` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript`): `test`, `tiles-check`, `tiles-check-native`, `verify-physical-codecs`
- `@semio-tech/gis-gismap-rs` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-gis-map-window-ownership`, `verify-inference-client`, `verify-inference-client-native`, `verify-inference-mcp`, `verify-inference-native-service`, `verify-map-document-contract`
- `@semio-tech/gis-gisterrain` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🟦️typescript`): `test`
- `@semio-tech/gis-gisterrain-rs` (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-gis-terrain-window-config`, `verify-terrain-document-contract`
- `@semio-tech/gis-js` (`✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript`): `test`
- `@semio-tech/imperative-extension-control-rust` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust`): `test`
- `@semio-tech/imperative-extension-effect-rust` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust`): `test`
- `@semio-tech/imperative-extension-logic-rust` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust`): `test`
- `@semio-tech/imperative-extension-math-rust` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust`): `test`
- `@semio-tech/imperative-extension-text-rust` (`✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust`): `test`
- `@semio-tech/imperative-js` (`✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript`): `test`
- `@semio-tech/imperative-procedure` (`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🟦️typescript`): `test`
- `@semio-tech/imperative-procedure-rs` (`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-procedure-document-contract`
- `@semio-tech/layout-js` (`✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript`): `test`, `test-renderer-contract`
- `@semio-tech/layout-layout-rs` (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-layout-document-contract`, `verify-layout-frame-selection`, `verify-layout-window-ownership`, `verify-physical-codecs`
- `@semio-tech/lowpoly-js` (`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript`): `test`
- `@semio-tech/lowpoly-lowpoly` (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript`): `test`
- `@semio-tech/lowpoly-lowpoly-rs` (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/mathematical-equation` (`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🟦️typescript`): `test`
- `@semio-tech/mathematical-equation-rs` (`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/mathematical-js` (`✏️s/🔌️plugins/➗️mathematical/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-artifact-contract-rs` (`✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust`): `test`, `test-byte-property`
- `@semio-tech/norm-din16798` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-din16798-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-din18599` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-din18599-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-din4108` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-din4108-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1990` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1990-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1991` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1991-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1992` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1992-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1993` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1993-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1994` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1994-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1995` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1995-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1996` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1996-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1997` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1997-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-snapshot-sqlite-source`
- `@semio-tech/norm-en1998` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1998-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-en1999` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-en1999-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-iso16757` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-iso16757-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/norm-js` (`✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript`): `test`, `test-wire-twins`
- `@semio-tech/norm-vdi3805` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🟦️typescript`): `test`
- `@semio-tech/norm-vdi3805-rs` (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/note-note-js` (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🟦️typescript`): `test`, `test-document-contract`, `test-snapshot-sqlite`
- `@semio-tech/note-note-rs` (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust`): `test`, `verify-note-empty-config-ownership`, `verify-snapshot-guest`, `verify-sqlite-snapshot-guest`, `verify-sqlite-snapshot-native`
- `@semio-tech/playbook-extension-procedural-rust` (`✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust`): `geometry-contract`, `test`
- `@semio-tech/playbook-js` (`✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript`): `test`
- `@semio-tech/playbook-playbook` (`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🟦️typescript`): `test`
- `@semio-tech/playbook-playbook-rs` (`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-playbook-document-contract`
- `@semio-tech/procedural-generation2d` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🟦️typescript`): `test`, `verify-snapshot-sqlite-source`
- `@semio-tech/procedural-generation2d-rs` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-generation2d-window-camera-ownership`
- `@semio-tech/procedural-generation3d` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/procedural-generation3d-rs` (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust`): `semantic-wire-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-document-restoration-oracle`, `verify-generation3d-document-io`, `verify-generation3d-preview-window-transient`, `verify-generation3d-widget-inputs`, `verify-generation3d-widget-inputs-native`, `verify-geometry-context`, `verify-original-widget-output-native`, `verify-original-widget-output-source`
- `@semio-tech/procedural-js` (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript`): `test`, `test-renderer-contract`
- `@semio-tech/process-extension-concrete-rust` (`✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust`): `test`
- `@semio-tech/process-extension-metal-rust` (`✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust`): `test`
- `@semio-tech/process-extension-robotic-rust` (`✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust`): `test`
- `@semio-tech/process-extension-wood-rust` (`✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust`): `test`
- `@semio-tech/process-js` (`✏️s/🔌️plugins/🏭️process/📦️packages/🟦️typescript`): `test`
- `@semio-tech/process-process3d-rs` (`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust`): `check-snapshot-sqlite-source`, `test`, `test-diff`, `test-replay-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/puzzle-2d-rs` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust`): `test`, `test-presence-native`, `test-presence-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-store-close-native`, `test-store-close-source`, `verify-snapshot-sqlite-source`
- `@semio-tech/puzzle-3d-rs` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-original-fill-fixture`, `verify-original-fill-fixture-native`, `verify-original-work-construction`, `verify-snapshot-sqlite-source`
- `@semio-tech/puzzle-5d-react` (`✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript`): `test`
- `@semio-tech/puzzle-5d-rs` (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-original-work-construction`, `verify-snapshot-sqlite-source`
- `@semio-tech/puzzle-js` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript`): `test`, `test-renderer-contract`
- `@semio-tech/raster-js` (`✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript`): `test`, `test-renderer-contract`
- `@semio-tech/raster-raster-rs` (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/reasoning-js` (`✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript`): `test`
- `@semio-tech/reasoning-wires` (`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🟦️typescript`): `test`
- `@semio-tech/reasoning-wires-rs` (`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-wires-document-contract`, `verify-wires-window-transient`
- `@semio-tech/remodel-js` (`✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript`): `test`
- `@semio-tech/remodel-remodeling-rs` (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-physical-codecs`, `verify-remodel-window-ownership`, `verify-snapshot-sqlite-native-syntax`, `verify-snapshot-sqlite-source`
- `@semio-tech/repo-dashboard-rs` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust`): `test`, `test-exhaustive`, `test-long`, `test-quick`
- `@semio-tech/repo-lib` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`): `test-inventory-artifact-shards`
- `@semio-tech/s-cad-composition-rs` (`✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust`): `source-check`, `test`
- `@semio-tech/s-dev-composition` (`✏️s/🧑‍💻dev`): `catalog-check`, `check-generated`, `composition-check`, `jack-graph-check`, `preview-generated`, `source-check`, `test-catalog-publication`, `verify-catalog`
- `@semio-tech/s-fem-composition-tests` (`✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript`): `verify-fem-assembly-physical-owners`, `verify-fem-mesh-preparation-owners`, `verify-fem-numerical-microcursor`, `verify-fem-numerical-page-owners`, `verify-fem-pcg-publication-owners`, `verify-fem-scalar-owners-native`, `verify-fem2d-window-config-contract`, `verify-fem3d-window-config-contract`
- `@semio-tech/s-fem-mesh` (`✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🟦️typescript`): `test-neutral-surface`, `test-surface-contract`
- `@semio-tech/s-fem-mesh-rs` (`✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📦️packages/🦀️rust`): `test`, `test-surface`
- `@semio-tech/s-flow-composition-rs` (`✏️s/🧑‍💻dev/🌊️flow/📦️packages/🦀️rust`): `port-sides-test`, `source-check`, `test`
- `@semio-tech/s-forms-composition-tests` (`✏️s/🧑‍💻dev/📋️forms/📦️packages/🟦️typescript`): `verify-forms-try-window-ownership`
- `@semio-tech/s-gis-services-native` (`✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust`): `mcp-check`, `mcp-composition-source-check`, `native-check`, `source-check`, `untrusted-content-check`
- `@semio-tech/s-imperative` (`✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust`): `test`
- `@semio-tech/s-native-ownership` (`✏️s/🧪️tests/🗂️native-ownership`): `test`
- `@semio-tech/s-norm-composition-tests` (`✏️s/🧑‍💻dev/📕️norm/📦️packages/🟦️typescript`): `verify-norm-document-contract`, `verify-norm-results-window-ownership`
- `@semio-tech/s-note-composition-tests` (`✏️s/🧑‍💻dev/🗒️note/📦️packages/🟦️typescript`): `verify-note-document-contract`
- `@semio-tech/s-plugin-composition` (`✏️s/🧪️tests/🏘️plugin-composition`): `test`
- `@semio-tech/s-puzzle-composition-tests` (`✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript`): `verify-browser-contribution`, `verify-puzzle-fill-policy-self-tests`
- `@semio-tech/s-space-composition-tests` (`✏️s/🧑‍💻dev/🪐️space/📦️packages/🟦️typescript`): `verify-framework-ui-protocol-ownership`
- `@semio-tech/s-spatial-kernel-semio-session-rs` (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust`): `source-check`, `test`
- `@semio-tech/sequence-sequence` (`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🟦️typescript`): `test`
- `@semio-tech/sequence-sequence-rs` (`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust`): `io-descriptor-parity`, `io-descriptor-parity-oracle`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-sequence-window-ownership`
- `@semio-tech/sequence-tests` (`✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript`): `test`
- `@semio-tech/shooting-js` (`✏️s/🔌️plugins/🎥️shooting/📦️packages/🟦️typescript`): `test`
- `@semio-tech/shooting-shooting-rs` (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-snapshot-sqlite-source`
- `@semio-tech/sourcing-curation-rs` (`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-curation-document-contract`, `verify-snapshot-sqlite-source`
- `@semio-tech/sourcing-extension-beams-rust` (`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust`): `test`
- `@semio-tech/sourcing-extension-slabs-rust` (`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust`): `test`
- `@semio-tech/sourcing-extension-windows-rust` (`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust`): `test`
- `@semio-tech/sourcing-js` (`✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript`): `test`
- `@semio-tech/space-core-rs` (`✏️s/🔌️plugins/🪐️space/🫀️catalog-documents/📦️packages/🦀️rust`): `test`
- `@semio-tech/space-home` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🟦️typescript`): `test`
- `@semio-tech/space-home-rs` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-complete-app-document`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-home-host-panel-owner`
- `@semio-tech/space-js` (`✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript`): `test`
- `@semio-tech/space-space` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🟦️typescript`): `test`
- `@semio-tech/space-space-rs` (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-complete-app-document`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-artifact-contract-rs` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust`): `test`, `test-original-ownership`
- `@semio-tech/stdio-assembly-rs` (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust`): `test`
- `@semio-tech/stdio-avi` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-avi-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-bcf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-bcf-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-binary` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-binary-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-bmp` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-bmp-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust`): `test`, `test-oracle`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-csv` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-csv-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-snapshot-subset-validation-native`
- `@semio-tech/stdio-deflate` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-deflate-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-docx` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-docx-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`
- `@semio-tech/stdio-dwg` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-dwg-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-dxf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-dxf-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-epw` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-epw-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-gif` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-gif-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-gltf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-gltf-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust`): `native-schema-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-html` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-html-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-ifc` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-ifc-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-jpg` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-jpg-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-js` (`✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript`): `package-contract`, `test`, `verify-semio-member-store-producer-source`
- `@semio-tech/stdio-json` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-json-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust`): `native-schema-check`, `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-las` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-las-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-md` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-md-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-mp3` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🟦️typescript`): `metadata-native`, `metadata-oracle`, `metadata-source`, `test`
- `@semio-tech/stdio-mp3-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-mp4` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-mp4-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-obj` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-obj-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-pdf` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript`): `test`, `verify-physical-codecs`
- `@semio-tech/stdio-pdf-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-snapshot-sqlite-source`
- `@semio-tech/stdio-ply` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-ply-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-png` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-png-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-pptx` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-pptx-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`
- `@semio-tech/stdio-semio` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🟦️typescript`): `test`, `verify-physical-codecs`
- `@semio-tech/stdio-semio-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust`): `conversion-definition-check`, `test`, `test-full`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-text-receiving-native`, `test-text-receiving-source`, `verify-stdio-document-contract`
- `@semio-tech/stdio-snapshot-editing-js` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-step` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-step-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-stl` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-stl-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📦️packages/🦀️rust`): `test`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-svg` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-svg-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-tiff` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-tiff-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-tsv` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-tsv-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `test-snapshot-subset-validation-native`
- `@semio-tech/stdio-txt` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🟦️typescript`): `test`, `verify-physical-codecs`
- `@semio-tech/stdio-txt-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust`): `test`, `test-original-encoding-native`, `test-original-encoding-source`, `test-original-receiving-native`, `test-original-receiving-source`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-wav` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-wav-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-xlsx` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-xlsx-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`
- `@semio-tech/stdio-xml` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-xml-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/stdio-zip` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🟦️typescript`): `test`
- `@semio-tech/stdio-zip-rs` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/trinity-jack` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🟦️typescript`): `verify-snapshot-sqlite-source`
- `@semio-tech/trinity-jack-lsp-worker` (`✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript`): `test`
- `@semio-tech/trinity-jack-rs` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-jack-child-selection`, `verify-jack-document-contract`, `verify-jack-graph-query`, `verify-jack-native-composition`, `verify-jack-query-ownership`, `verify-physical-codecs`
- `@semio-tech/trinity-jack-shell` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell/📦️packages/🦀️rust`): `test`
- `@semio-tech/trinity-js` (`✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript`): `test`
- `@semio-tech/trinity-rewriting` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🟦️typescript`): `verify-snapshot-sqlite-source`
- `@semio-tech/trinity-rewriting-rs` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-rewriting-document-contract`, `verify-rewriting-map-ownership`, `verify-rewriting-window-config`
- `@semio-tech/ui-styling-tokens` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust`): `check-generated`
- `@semio-tech/vcs-js` (`✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript`): `test`
- `@semio-tech/vcs-vcs-rs` (`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-2d` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/wfc-2d-rs` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-3d` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/wfc-3d-rs` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-bitmap` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🟦️typescript`): `test`
- `@semio-tech/wfc-bitmap-rs` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-engine-rs` (`✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust`): `test`
- `@semio-tech/wfc-grid2d` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/wfc-grid2d-rs` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-grid3d` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🟦️typescript`): `test`
- `@semio-tech/wfc-grid3d-rs` (`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/📦️packages/🦀️rust`): `mutation-inventory`, `mutation-inventory-check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`
- `@semio-tech/wfc-js` (`✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript`): `test`
- `@semio-tech/writer-js` (`✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript`): `test`
- `@semio-tech/writer-writer` (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🟦️typescript`): `test`, `verify-snapshot-sqlite-source`
- `@semio-tech/writer-writer-rs` (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust`): `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`, `verify-writer-window-state`
- `workspace` (`.`): `artifact-field-parity-test`, `test-discover`, `test-report`, `verify-dependencies-freeze`, `verify-dependencies-literal-external`, `verify-gate`, `verify-interactivity`, `verify-layering`, `verify-taxonomy-enforce`, `verify-taxonomy-implementation-enforce`, `verify-taxonomy-implementation-report`, `verify-taxonomy-report`, `verify-typescript`, `verify-typescript-project`

`*` = cacheable only through the plugin class (no explicit `cache: true`).
