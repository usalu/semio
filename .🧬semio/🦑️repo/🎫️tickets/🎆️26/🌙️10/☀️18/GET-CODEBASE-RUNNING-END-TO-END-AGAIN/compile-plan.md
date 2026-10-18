# Compile plan: GET-CODEBASE-RUNNING-END-TO-END-AGAIN

Read-only consolidation (no cargo, no build, no git write). Sources: the `result-*.md` files of this ticket that carry compile lists (c6flow, cg1, glob, nxneutral, wgpu), `RESULT-CENSUS5-A/C7/C8/C9/D/E/F/G/PAI.md`, `RESULT-BASELINE-DEFECTS-12.md`, and the deferred items of `COMPILE-DEFECT-LEDGER.md`, `📓️COMPILE-RUNBOOK.md` and `📓️COMPILE-WINDOW-CHECKLIST.md` (all in `🪓️monolith-split`). Batch files named by the runbook (`pi1-tools/flipped.json`, `okeng-tools/b2.crates`, `okwp0-tools/touched.txt`, the stdio contract set) were expanded by name and verified.

Totals: 784 crates in 34 workspaces. 526 get `--lib` and `--tests`. 258 framework members (the runbook's phase 1c lib sweep) get `--lib` only. All 784 names exist as package names in the tree (`semio-tech` and `semio-mut-family-2` are not crates; see section 5).

## 0. Gates before any call

- Runbook section 2.1 blockers B1 (image receipt has no `mountDirectory`) and B2 (headroom 4,262,842,368 B against 4,294,967,296 B) were recorded at 2026-10-16 and are not re-probed here. Run the probe first: `pl check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml --message-format=short` (runbook 2.2). Any launcher refusal means stop.
- Take the fleet mutex around every launcher call (runbook 2.3): `mkdir <F>/🗑️generated/cargo-mutex.lock`, write the owner file, release after the call. One launcher call at a time.
- Only `--lib` and `--tests` are used. `--all-targets` is refused by the source gate for crates with the oversized dev-dependency `semio-framework-replication` (runbook section 1).

## 1. Command form (checked against the launcher source)

Launcher: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts`, schema `…/🚪️launcher/🧬️schema/🔣️.json`.

- `--store=shared|isolated` and `--configuration=prototype|release` are both required before `--`. `--parallelism=<n>` is optional.
- The first Cargo argument after `--` must be in an admitted class: `compile` (build, check, test, bench, run, doc, clippy, rustc, rustdoc), `resolve` (metadata, tree, fetch, ...), or `lock` (generate-lockfile, update). `update --workspace --offline` is therefore admitted.
- Manifest: the launcher reads `--manifest-path` from the Cargo arguments and defaults to `Cargo.toml` in the working directory. Root crates therefore work with or without the flag. This plan passes it explicitly for every workspace, as the lead's form requires.
- Working directory: the helper `cd`s to the repo root, so manifest paths are repo-relative.

```sh
pl() { cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@"; }
```

Each batch below is one `pl` call. Per runbook section 1 the `--lib` line runs first, then the `--tests` line. Batches are at most 25 `-p` names. The workspace order is the runbook phase order: framework leaves, root OS and plugin host, plugin workspaces, teaching, hub.

## 2. Offline lock refresh (mandatory first, one call per workspace)

Runbook section 2.4 and checklist step 1. After each refresh, `metadata --locked --offline` must be a no-op. Run the refresh before any batch of that workspace.

```sh
pl update --workspace --offline --manifest-path 🧰️framework/Cargo.toml
pl update --workspace --offline --manifest-path Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml
pl update --workspace --offline --manifest-path ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml
pl update --workspace --offline --manifest-path 🎓️teaching/Cargo.toml
pl update --workspace --offline --manifest-path 🌎️hub/Cargo.toml
```

Workspace order for the refresh is the same as in section 3. Lock files that the runbook names as hand-edited (L1 to L11) are all covered by these lines.

## 3. Ordered batches

### 3.1 framework workspace (`🧰️framework/Cargo.toml`): 359 crates, 101 with `--tests`, 258 lib-only

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-3d-brep-boolean-unit-tests -p semio-framework-3d-brep-engine-tests -p semio-framework-3d-brep-polynomial -p semio-framework-3d-mesh-tests -p semio-framework-actor-pack -p semio-framework-artifact-reference-borrow -p semio-framework-async-macros -p semio-framework-canvas-font-assets -p semio-framework-dispatch-macros -p semio-framework-dsl-record-derive -p semio-framework-expression -p semio-framework-font-anta-latin-ext-regular -p semio-framework-font-anta-latin-regular -p semio-framework-font-anta-math-regular -p semio-framework-font-anta-symbols-regular -p semio-framework-font-kelly-slab-cyrillic-regular -p semio-framework-font-kelly-slab-latin-ext-regular -p semio-framework-font-kelly-slab-latin-regular -p semio-framework-font-noto-emoji-activities-regular -p semio-framework-font-noto-emoji-faces-regular -p semio-framework-font-noto-emoji-flags-regular -p semio-framework-font-noto-emoji-food-regular -p semio-framework-font-noto-emoji-joined-forms-regular -p semio-framework-font-noto-emoji-nature-regular -p semio-framework-font-noto-emoji-objects-regular --lib --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-font-noto-emoji-people-regular -p semio-framework-font-noto-emoji-regions-regular -p semio-framework-font-noto-emoji-supplement-regular -p semio-framework-font-noto-emoji-symbols-regular -p semio-framework-font-noto-emoji-travel-regular -p semio-framework-font-share-tech-mono-latin-regular -p semio-framework-geometry-primitives -p semio-framework-hash -p semio-framework-intrinsic-size -p semio-framework-io-base64 -p semio-framework-job-unit-tests -p semio-framework-machine-derive -p semio-framework-number -p semio-framework-pack-intrinsic-media-laws -p semio-framework-plugin-child-group-wire-laws -p semio-framework-plugin-command-page-laws -p semio-framework-plugin-surface-carrier-laws -p semio-framework-process-routing-workspace -p semio-framework-schema-composition -p semio-framework-schema-registry -p semio-framework-source-projection -p semio-framework-surface-tiled-map-mvt -p semio-framework-trace -p semio-framework-ui-arena -p semio-framework-ui-contract-budget --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-geometry-primitives -p semio-framework-hash -p semio-framework-io-base64 -p semio-framework-job-unit-tests -p semio-framework-number -p semio-framework-pack-intrinsic-media-laws -p semio-framework-process-routing-workspace -p semio-framework-schema-registry --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-contract-conformance -p semio-framework-ui-render-prepared-shaders -p semio-framework-ui-render-shader-contract -p semio-framework-ui-scene-pack -p semio-framework-ui-styling -p semio-framework-ui-surface-identity -p semio-framework-ui-testing-paint-policy -p semio-framework-unicode-mappings -p semio-framework-unicode-properties -p semio-framework-value-control-contract -p semio-framework-value-resident -p semio-framework-value-testing-allocator -p semio-framework-2d-compute -p semio-framework-3d-testing-support -p semio-framework-composition-admission -p semio-framework-geometry-vector -p semio-framework-io-base64-tests -p semio-framework-machine -p semio-framework-native-build-compiler-resources -p semio-framework-number-tests -p semio-framework-test-fixtures -p semio-framework-trace-memory-testing -p semio-framework-ui-render-device -p semio-framework-value-allocation-return -p semio-framework-geometry-bulge --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-contract-conformance -p semio-framework-ui-testing-paint-policy -p semio-framework-value-resident -p semio-framework-value-testing-allocator -p semio-framework-3d-testing-support -p semio-framework-io-base64-tests -p semio-framework-machine -p semio-framework-native-build-compiler-resources -p semio-framework-test-fixtures -p semio-framework-trace-memory-testing --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-geometry-skeleton -p semio-framework-value-derive -p semio-framework-value-paged-list -p semio-framework-geometry-solids -p semio-framework-value-retirement -p semio-framework-geometry -p semio-framework-value-factory -p semio-framework-value-native-encoding -p semio-framework-value-original-aliases -p semio-framework-value-retirement-queue -p semio-framework-value-shared-issuance -p semio-framework-canvas-camera -p semio-framework-math-sampling-vocabulary -p semio-framework-raster -p semio-framework-typeset -p semio-framework-ui-render-packet -p semio-framework-ui-scene-algebra -p semio-framework-value-clone-source -p semio-framework-value-native-decoding -p semio-framework-math-sampling-config -p semio-framework-math-sampling-diffusion -p semio-framework-ui-render-text -p semio-framework-value-clone-source-testing -p semio-framework-value-ordered -p semio-framework-value-paged-text --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value-derive -p semio-framework-value-paged-list -p semio-framework-value-retirement -p semio-framework-value-factory -p semio-framework-value-native-encoding -p semio-framework-value-original-aliases -p semio-framework-value-retirement-queue -p semio-framework-value-shared-issuance -p semio-framework-value-clone-source -p semio-framework-value-native-decoding -p semio-framework-value-clone-source-testing -p semio-framework-value-ordered -p semio-framework-value-paged-text --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-math-sampling-workspace -p semio-framework-value-intrinsic -p semio-framework-math-sampling-constraints -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-testing -p semio-framework-mutation-diff -p semio-framework-value-retained-clone -p semio-framework-math-sampling-engine -p semio-framework-value-retained-map -p semio-framework-math-sampling-decoding -p semio-framework-value -p semio-framework-2d-engine -p semio-framework-2d-retire -p semio-framework-2d-text -p semio-framework-2d-text-font-testing -p semio-framework-3d-brep-tolerance -p semio-framework-3d-brep-vector -p semio-framework-3d-mesh-contract -p semio-framework-3d-rigid -p semio-framework-abi -p semio-framework-async -p semio-framework-deflate -p semio-framework-graph-algorithms -p semio-framework-icon-contract -p semio-framework-io-binary-source --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value-intrinsic -p semio-framework-math-sampling-constraints -p semio-framework-math-sampling-processors -p semio-framework-mutation-diff -p semio-framework-value-retained-clone -p semio-framework-value-retained-map -p semio-framework-value -p semio-framework-abi -p semio-framework-async --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-io-control -p semio-framework-io-sqlite-model -p semio-framework-pack-json -p semio-framework-schema-state -p semio-framework-tool-run-entities -p semio-framework-ui-contract-text -p semio-framework-ui-contract-text-edit -p semio-framework-ui-contract-typegen -p semio-framework-ui-locale -p semio-framework-ui-raster-owner -p semio-framework-value-retirement-owner -p semio-framework-value-testing-presence-retirement -p semio-framework-value-tests -p semio-framework-value-tests-2 -p semio-framework-2d-path-flatten -p semio-framework-2d-path-stroke -p semio-framework-2d-text-font -p semio-framework-2d-trace -p semio-framework-3d-brep-curve -p semio-framework-3d-collision -p semio-framework-async-tests -p semio-framework-async-wasm-pool-tests -p semio-framework-diagnostic -p semio-framework-io-sqlite-snapshot -p semio-framework-manifest-media --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-pack-json -p semio-framework-value-tests -p semio-framework-async-tests -p semio-framework-async-wasm-pool-tests -p semio-framework-io-sqlite-snapshot --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-mesh-component-references -p semio-framework-pack-json-tests -p semio-framework-replication-history-index -p semio-framework-replication-ids -p semio-framework-replication-interaction -p semio-framework-schema-derive -p semio-framework-ui-contract-identity -p semio-framework-value-snapshot-read -p semio-framework-2d-booleans -p semio-framework-2d-text-font-catalog -p semio-framework-2d-text-font-shape-unicode -p semio-framework-3d-brep-surface -p semio-framework-dsl-token -p semio-framework-io-ports -p semio-framework-kernel-history -p semio-framework-pack-error -p semio-framework-replication-command-ingress -p semio-framework-replication-local-interaction -p semio-framework-ui-contract-values -p semio-framework-ui-scene-mesh -p semio-framework-2d-regions -p semio-framework-2d-text-font-shape -p semio-framework-dsl-lexical -p semio-framework-pack-source -p semio-framework-replication-error --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-pack-json-tests -p semio-framework-replication-interaction -p semio-framework-pack-error --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-contract-actions -p semio-framework-ui-scene-pass -p semio-framework-2d -p semio-framework-2d-text-font-run -p semio-framework-dsl-syntax -p semio-framework-replication-codec -p semio-framework-replication-crypto -p semio-framework-ui-contract-presentation -p semio-framework-dsl -p semio-framework-pack-async -p semio-framework-pack-format -p semio-framework-pixels -p semio-framework-replication-wire-core -p semio-framework-ui-contract-components -p semio-framework-compiler -p semio-framework-dsl-notation -p semio-framework-dsl-record-model -p semio-framework-pack-format-retained -p semio-framework-pack-http -p semio-framework-pack-io -p semio-framework-pixels-tests -p semio-framework-replication-format -p semio-framework-replication-mutation -p semio-framework-replication-presence -p semio-framework-surface-terrain --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-2d -p semio-framework-dsl -p semio-framework-pixels -p semio-framework-pixels-tests --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-contract-builder -p semio-framework-ui-contract-operations -p semio-framework-canvas-assets -p semio-framework-dsl-record-emission -p semio-framework-pack-format-catalog -p semio-framework-replication-edit -p semio-framework-ui-contract -p semio-framework-canvas-icons -p semio-framework-canvas-scene -p semio-framework-dsl-record -p semio-framework-kernel-ui -p semio-framework-replication-causal -p semio-framework-ui-contract-validation -p semio-framework-ui-layout -p semio-framework-ui-render-interaction -p semio-framework-ui-runtime-exchange -p semio-framework-ui-theme -p semio-framework-artifact-reference -p semio-framework-canvas -p semio-framework-dsl-record-ownership-oracle -p semio-framework-job -p semio-framework-pack-schema-index -p semio-framework-replication-conflict -p semio-framework-replication-frames -p semio-framework-replication-history --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-canvas-assets -p semio-framework-pack-format-catalog -p semio-framework-artifact-reference -p semio-framework-dsl-record-ownership-oracle -p semio-framework-job -p semio-framework-pack-schema-index --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-action -p semio-framework-ui-render -p semio-framework-ui-select-behavior -p semio-framework-ui-text -p semio-framework-ui-viewport -p semio-framework-ui-wgpu-device -p semio-framework-value-dictionary -p semio-framework-actor-retained-turn -p semio-framework-io-schema -p semio-framework-job-fault-publication -p semio-framework-job-native-decode -p semio-framework-job-progress -p semio-framework-job-publication -p semio-framework-job-registry -p semio-framework-job-scope -p semio-framework-job-worker-authority -p semio-framework-manifest-contribution -p semio-framework-pack-json-retained-schema -p semio-framework-pack-record-schema -p semio-framework-replication-mutation-messages -p semio-framework-surface-paint -p semio-framework-surface-tiled-map-model -p semio-framework-ui-backend-d3d12 -p semio-framework-ui-backend-metal -p semio-framework-ui-backend-vulkan --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-text -p semio-framework-ui-viewport -p semio-framework-ui-wgpu-device -p semio-framework-job-native-decode -p semio-framework-pack-record-schema -p semio-framework-replication-mutation-messages --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-backend-webgpu -p semio-framework-ui-prepared-staging -p semio-framework-ui-scene-camera -p semio-framework-actor-retained-context -p semio-framework-actor-vocabulary -p semio-framework-io-codec -p semio-framework-io-sqlite-snapshot-tests -p semio-framework-job-conformance -p semio-framework-job-worker -p semio-framework-pack-record-encode -p semio-framework-pack-record-preflight -p semio-framework-pack-record-retained -p semio-framework-replication -p semio-framework-surface-tiled-map -p semio-framework-ui-draw-contract -p semio-framework-ui-host -p semio-framework-ui-scene-picking -p semio-framework-actor-job-publication -p semio-framework-graph-manifest -p semio-framework-job-preview -p semio-framework-pack-record -p semio-framework-test-mutation-inventory -p semio-framework-time-travel -p semio-framework-tool-machine -p semio-framework-ui-host-tests --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-backend-webgpu -p semio-framework-io-sqlite-snapshot-tests -p semio-framework-pack-record-retained -p semio-framework-replication -p semio-framework-ui-host -p semio-framework-pack-record -p semio-framework-tool-machine -p semio-framework-ui-host-tests --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui-prepared -p semio-framework-ui-scene-doc -p semio-framework-ui-scene-modelling -p semio-framework-actor -p semio-framework-graph-dsl -p semio-framework-graph-engine -p semio-framework-pack-intrinsic -p semio-framework-tool-machine-tests -p semio-framework-ui-scene-node-graph -p semio-framework-ui-scene-plane -p semio-framework-ui-scene-surface -p semio-framework-ui-scene-views -p semio-framework-ui-scene-world3d -p semio-framework-ui-wgpu-mesh -p semio-framework-ui-wgpu-raster -p semio-framework-actor-component -p semio-framework-graph-drawing -p semio-framework-kernel-identity -p semio-framework-pack -p semio-framework-ui-scene -p semio-framework-ui-wgpu-pipelines -p semio-framework-editor -p semio-framework-kernel-media-jobs -p semio-framework-manifest-arg-schema -p semio-framework-mesh-engine --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-actor -p semio-framework-graph-dsl -p semio-framework-graph-engine -p semio-framework-tool-machine-tests -p semio-framework-ui-wgpu-mesh -p semio-framework-ui-wgpu-raster -p semio-framework-pack -p semio-framework-ui-wgpu-pipelines -p semio-framework-mesh-engine --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-schema-validator -p semio-framework-surface-node-graph -p semio-framework-ui-declaration -p semio-framework-ui-runtime-presentation -p semio-framework-ui-wgpu-draw-execution -p semio-framework-3d-brep-representation -p semio-framework-3d-mesh-clipping -p semio-framework-3d-mesh-document -p semio-framework-editor-component -p semio-framework-manifest-mutation-inputs -p semio-framework-schema -p semio-framework-surface -p semio-framework-ui-retained-tree-tree -p semio-framework-ui-runtime-reconcile-primitives -p semio-framework-ui-wgpu-render -p semio-framework-3d-brep-intersect -p semio-framework-3d-brep-kernel-contract -p semio-framework-3d-brep-tessellation -p semio-framework-3d-mesh-reconstruction -p semio-framework-surface-component -p semio-framework-ui-retained-tree -p semio-framework-ui-runtime-reconcile-cursor -p semio-framework-3d -p semio-framework-3d-brep-queries -p semio-framework-3d-brep-shape-value --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-schema-validator -p semio-framework-ui-wgpu-draw-execution -p semio-framework-3d-brep-representation -p semio-framework-ui-runtime-reconcile-primitives -p semio-framework-3d-brep-tessellation -p semio-framework-3d --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui -p semio-framework-ui-runtime-reconcile -p semio-framework-3d-brep-validation -p semio-framework-interaction -p semio-framework-manifest-address -p semio-framework-tool-run -p semio-framework-ui-runtime -p semio-framework-3d-brep-analysis -p semio-framework-3d-brep-construction -p semio-framework-graph-layout-run -p semio-framework-manifest-actions -p semio-framework-manifest-view-model -p semio-framework-tool-run-presentation -p semio-framework-tool-run-tests -p semio-framework-3d-brep-boolean -p semio-framework-3d-brep-mesh-io -p semio-framework-3d-brep-offset -p semio-framework-3d-brep-sweep -p semio-framework-kernel -p semio-framework-onboarding -p semio-framework-3d-brep-engine -p semio-framework-manifest-app-surface -p semio-framework-action-bus -p semio-framework-manifest-plugin -p semio-framework-time-travel-actions --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-ui -p semio-framework-ui-runtime-reconcile -p semio-framework-3d-brep-validation -p semio-framework-tool-run -p semio-framework-3d-brep-construction -p semio-framework-graph-layout-run -p semio-framework-tool-run-tests -p semio-framework-3d-brep-boolean -p semio-framework-3d-brep-offset -p semio-framework-3d-brep-sweep -p semio-framework-3d-brep-engine --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework-tool-run-actions -p semio-framework -p semio-framework-plugin-declared-verdict -p semio-framework-plugin-registration -p semio-framework-io-sqlite-snapshot-testing-backing -p semio-framework-test-original-snapshot-owners -p semio-framework-test-typed-input-law -p semio-framework-plugin-surface-fixture -p semio-framework-test-transient-preparation-law --lib --keep-going --message-format=short
pl check --manifest-path 🧰️framework/Cargo.toml -p semio-framework -p semio-framework-plugin-surface-fixture -p semio-framework-test-transient-preparation-law --tests --keep-going --message-format=short
```

### 3.2 root workspace (`Cargo.toml`): 265 crates, 265 with `--tests`, 0 lib-only

```sh
pl check --manifest-path Cargo.toml -p semio-framework-artifact-flow-flow-mutation-laws -p semio-framework-os-flow-contract -p semio-framework-os-playbook-contract -p semio-framework-os-renderer-wgpu-asset-authority -p semio-framework-os-scale-fixture -p semio-framework-plugin-host-guest-laws -p semio-framework-plugin-host-shard-laws -p semio-framework-plugin-test-support -p semio-framework-plugin-tests-07-2 -p semio-framework-plugin-tests-09-2 -p semio-framework-plugin-tests-12-2 -p semio-framework-plugin-tests-13 -p semio-framework-plugin-tests-14-2 -p semio-framework-plugin-tests-16 -p semio-framework-repo-events -p semio-framework-repo-mcp -p semio-framework-repo-model -p semio-framework-os-plugin-registry-schema -p semio-framework-repo-languages -p semio-framework-repo-metrics -p semio-framework-repo-providers -p semio-framework-repo-test-runner -p semio-repo-test-host -p semio-framework-pets-contract -p semio-framework-plugin-tests-08 --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-artifact-flow-flow-mutation-laws -p semio-framework-os-flow-contract -p semio-framework-os-playbook-contract -p semio-framework-os-renderer-wgpu-asset-authority -p semio-framework-os-scale-fixture -p semio-framework-plugin-host-guest-laws -p semio-framework-plugin-host-shard-laws -p semio-framework-plugin-test-support -p semio-framework-plugin-tests-07-2 -p semio-framework-plugin-tests-09-2 -p semio-framework-plugin-tests-12-2 -p semio-framework-plugin-tests-13 -p semio-framework-plugin-tests-14-2 -p semio-framework-plugin-tests-16 -p semio-framework-repo-events -p semio-framework-repo-mcp -p semio-framework-repo-model -p semio-framework-os-plugin-registry-schema -p semio-framework-repo-languages -p semio-framework-repo-metrics -p semio-framework-repo-providers -p semio-framework-repo-test-runner -p semio-repo-test-host -p semio-framework-pets-contract -p semio-framework-plugin-tests-08 --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-plugin-tests-11-2 -p semio-framework-plugin-tests-15 -p semio-framework-quiz-schema -p semio-framework-repo-codebase -p semio-framework-repo-move -p semio-framework-repo-statutes -p semio-framework-repo-tickets -p semio-framework-pets-clearance -p semio-framework-quiz -p semio-framework-repo-graphql-fs -p semio-framework-repo-hooks -p semio-framework-repo-tree -p semio-framework-quiz-primitives-tests -p semio-framework-quiz-tests -p semio-framework-repo-cli-event-stream -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-cli-tree-source-and-search -p semio-framework-repo-dashboard-inventory -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli -p semio-framework-os-history-testing-retirement -p semio-framework-os-list-delta -p semio-framework-os-semio -p semio-framework-os-directory --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-plugin-tests-11-2 -p semio-framework-plugin-tests-15 -p semio-framework-quiz-schema -p semio-framework-repo-codebase -p semio-framework-repo-move -p semio-framework-repo-statutes -p semio-framework-repo-tickets -p semio-framework-pets-clearance -p semio-framework-quiz -p semio-framework-repo-graphql-fs -p semio-framework-repo-hooks -p semio-framework-repo-tree -p semio-framework-quiz-primitives-tests -p semio-framework-quiz-tests -p semio-framework-repo-cli-event-stream -p semio-framework-repo-dashboard-repo-domain -p semio-framework-repo-cli-mcp-verb -p semio-framework-repo-cli-tree-source-and-search -p semio-framework-repo-dashboard-inventory -p semio-framework-repo-cli-verbs -p semio-framework-repo-cli -p semio-framework-os-history-testing-retirement -p semio-framework-os-list-delta -p semio-framework-os-semio -p semio-framework-os-directory --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-extension -p semio-framework-os-kernel-dsl-derive-expand -p semio-framework-os-renderer-wgpu-shell-vocab -p semio-framework-os-directory-tests-1 -p semio-framework-os-directory-tests-2 -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-kernel-dsl-derive-tests -p semio-framework-os-space-sqlite-cells -p semio-framework-os-renderer-camera -p semio-framework-plugin-identity -p semio-framework-os-channel -p semio-framework-os-kernel-neural-engine-model -p semio-framework-server -p semio-framework-os-channel-tests -p semio-framework-os-kernel-neural-engine-registry -p semio-framework-server-tests -p semio-framework-os-kernel-neural-engine-budgeted -p semio-framework-os-services -p semio-framework-plugin-interpreter -p semio-framework-os-history-record-decode -p semio-framework-os-kernel-neural-engine -p semio-framework-os-pack -p semio-framework-os-services-tests -p semio-framework-os-vcs -p semio-framework-os-command --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-extension -p semio-framework-os-kernel-dsl-derive-expand -p semio-framework-os-renderer-wgpu-shell-vocab -p semio-framework-os-directory-tests-1 -p semio-framework-os-directory-tests-2 -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-kernel-dsl-derive-tests -p semio-framework-os-space-sqlite-cells -p semio-framework-os-renderer-camera -p semio-framework-plugin-identity -p semio-framework-os-channel -p semio-framework-os-kernel-neural-engine-model -p semio-framework-server -p semio-framework-os-channel-tests -p semio-framework-os-kernel-neural-engine-registry -p semio-framework-server-tests -p semio-framework-os-kernel-neural-engine-budgeted -p semio-framework-os-services -p semio-framework-plugin-interpreter -p semio-framework-os-history-record-decode -p semio-framework-os-kernel-neural-engine -p semio-framework-os-pack -p semio-framework-os-services-tests -p semio-framework-os-vcs -p semio-framework-os-command --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-directory-client -p semio-framework-os-flow-drawing -p semio-framework-os-history -p semio-framework-os-history-record-decode-tests -p semio-framework-os-pack-tools -p semio-framework-os-store-command -p semio-framework-os-directory-client-tests -p semio-framework-os-dsl -p semio-framework-os-spr -p semio-framework-os-dsl-fixture-sweep-tests -p semio-framework-os-dsl-module-tests -p semio-framework-os-dsl-tests -p semio-framework-os-spr-cli -p semio-framework-os-spr-laws-tests -p semio-framework-os-store-member-open -p semio-framework-os-store-space-schema -p semio-framework-os-renderer-wgpu-asset-decode -p semio-framework-os-renderer-wgpu-media-slots -p semio-framework-os-store-space-history -p semio-framework-os-store-text-io -p semio-framework-os-store-codec-registry -p semio-framework-os-store-durable-group-wire -p semio-framework-os-store-engine-owners -p semio-framework-os-io -p semio-framework-os-store-initialization-runtime --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-directory-client -p semio-framework-os-flow-drawing -p semio-framework-os-history -p semio-framework-os-history-record-decode-tests -p semio-framework-os-pack-tools -p semio-framework-os-store-command -p semio-framework-os-directory-client-tests -p semio-framework-os-dsl -p semio-framework-os-spr -p semio-framework-os-dsl-fixture-sweep-tests -p semio-framework-os-dsl-module-tests -p semio-framework-os-dsl-tests -p semio-framework-os-spr-cli -p semio-framework-os-spr-laws-tests -p semio-framework-os-store-member-open -p semio-framework-os-store-space-schema -p semio-framework-os-renderer-wgpu-asset-decode -p semio-framework-os-renderer-wgpu-media-slots -p semio-framework-os-store-space-history -p semio-framework-os-store-text-io -p semio-framework-os-store-codec-registry -p semio-framework-os-store-durable-group-wire -p semio-framework-os-store-engine-owners -p semio-framework-os-io -p semio-framework-os-store-initialization-runtime --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-store-engine-core -p semio-framework-os-renderer-wgpu-dock -p semio-framework-os-renderer-wgpu-scene-canvas2d -p semio-framework-os-store-engine-live -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-wake -p semio-framework-plugin-reactor-tasks -p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-store-engine-stage -p semio-framework-os-store-engine -p semio-framework-os-store-codec-factory -p semio-framework-os-store-durable-group -p semio-framework-os-store-initialization-replay -p semio-framework-os-store-space-member -p semio-framework-os-store-sync -p semio-framework-os-store-composition -p semio-framework-os-store-document -p semio-framework-os-store-sync-native -p semio-framework-os-store-config -p semio-framework-os-store-member-open-store -p semio-framework-os-store-space -p semio-framework-os-store-test-support -p semio-framework-os-kernel -p semio-framework-os-store-testing-fixtures -p semio-framework-artifact-flow-flow-model --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-store-engine-core -p semio-framework-os-renderer-wgpu-dock -p semio-framework-os-renderer-wgpu-scene-canvas2d -p semio-framework-os-store-engine-live -p semio-framework-plugin-host-effects -p semio-framework-plugin-host-wake -p semio-framework-plugin-reactor-tasks -p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-store-engine-stage -p semio-framework-os-store-engine -p semio-framework-os-store-codec-factory -p semio-framework-os-store-durable-group -p semio-framework-os-store-initialization-replay -p semio-framework-os-store-space-member -p semio-framework-os-store-sync -p semio-framework-os-store-composition -p semio-framework-os-store-document -p semio-framework-os-store-sync-native -p semio-framework-os-store-config -p semio-framework-os-store-member-open-store -p semio-framework-os-store-space -p semio-framework-os-store-test-support -p semio-framework-os-kernel -p semio-framework-os-store-testing-fixtures -p semio-framework-artifact-flow-flow-model --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-artifact-space-collection -p semio-framework-artifact-workflow-run -p semio-framework-os-command-tests -p semio-framework-os-config -p semio-framework-os-flow-mesh -p semio-framework-os-flow-vcs -p semio-framework-os-mcp-foundation -p semio-framework-os-mcp-hub-pair -p semio-framework-os-renderer-wgpu-agent-elements -p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-store-testing-doubles -p semio-framework-plugin-guest-contract -p semio-framework-plugin-host-test-component-refusal -p semio-framework-plugin-window-config -p semio-framework-artifact-workflow-run-tests -p semio-framework-os-config-tests-1 -p semio-framework-os-config-tests-2 -p semio-framework-os-flow-vcs-laws -p semio-framework-os-mcp-bridge-codec -p semio-framework-os-mcp-hub-remote -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-store-durable-group-tests -p semio-framework-os-store-member-open-tests -p semio-framework-os-store-sync-tests-1 -p semio-framework-os-store-sync-tests-2 --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-artifact-space-collection -p semio-framework-artifact-workflow-run -p semio-framework-os-command-tests -p semio-framework-os-config -p semio-framework-os-flow-mesh -p semio-framework-os-flow-vcs -p semio-framework-os-mcp-foundation -p semio-framework-os-mcp-hub-pair -p semio-framework-os-renderer-wgpu-agent-elements -p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-store-testing-doubles -p semio-framework-plugin-guest-contract -p semio-framework-plugin-host-test-component-refusal -p semio-framework-plugin-window-config -p semio-framework-artifact-workflow-run-tests -p semio-framework-os-config-tests-1 -p semio-framework-os-config-tests-2 -p semio-framework-os-flow-vcs-laws -p semio-framework-os-mcp-bridge-codec -p semio-framework-os-mcp-hub-remote -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-store-durable-group-tests -p semio-framework-os-store-member-open-tests -p semio-framework-os-store-sync-tests-1 -p semio-framework-os-store-sync-tests-2 --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-store-tests-01 -p semio-framework-os-store-tests-02 -p semio-framework-os-store-tests-03 -p semio-framework-os-store-tests-04 -p semio-framework-os-store-tests-05 -p semio-framework-os-store-tests-06 -p semio-framework-os-store-tests-07 -p semio-framework-os-store-tests-08 -p semio-framework-os-store-tests-09 -p semio-framework-os-store-tests-10 -p semio-framework-plugin-decl-views -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-wit -p semio-framework-plugin-interpreter-runtime -p semio-framework-os-mcp-bridge -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-plugin-child -p semio-framework-plugin-decl-surface -p semio-framework-plugin-describe -p semio-framework-plugin-host-operation -p semio-framework-plugin-host-process-transport -p semio-framework-plugin-job -p semio-framework-plugin-peer -p semio-framework-os-mcp-consent -p semio-framework-os-mcp-shell-channel --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-store-tests-01 -p semio-framework-os-store-tests-02 -p semio-framework-os-store-tests-03 -p semio-framework-os-store-tests-04 -p semio-framework-os-store-tests-05 -p semio-framework-os-store-tests-06 -p semio-framework-os-store-tests-07 -p semio-framework-os-store-tests-08 -p semio-framework-os-store-tests-09 -p semio-framework-os-store-tests-10 -p semio-framework-plugin-decl-views -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-wit -p semio-framework-plugin-interpreter-runtime -p semio-framework-os-mcp-bridge -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-plugin-child -p semio-framework-plugin-decl-surface -p semio-framework-plugin-describe -p semio-framework-plugin-host-operation -p semio-framework-plugin-host-process-transport -p semio-framework-plugin-job -p semio-framework-plugin-peer -p semio-framework-os-mcp-consent -p semio-framework-os-mcp-shell-channel --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-renderer-wgpu-ui-engine-dump -p semio-framework-plugin-command -p semio-framework-plugin-composition-emission -p semio-framework-plugin-host-guest-runtime -p semio-framework-plugin-host-imports -p semio-framework-plugin-tool-run -p semio-framework-plugin-composition-publication -p semio-framework-plugin-document-media -p semio-framework-plugin-host-shard -p semio-framework-plugin-time-travel-panel -p semio-framework-plugin-host-post-turn-relay -p semio-framework-plugin-host-shard-executor -p semio-framework-plugin-publication-mounted -p semio-framework-plugin-reactor-app -p semio-framework-plugin-retained-command -p semio-framework-plugin-time-travel -p semio-framework-plugin-host-routers -p semio-framework-plugin-instance-stores -p semio-framework-plugin-reactor-lifecycle -p semio-framework-plugin-host -p semio-framework-plugin-instance-dispatch -p semio-framework-plugin-reactor-patches -p semio-framework-plugin-runtime-instances --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-renderer-wgpu-ui-engine-dump -p semio-framework-plugin-command -p semio-framework-plugin-composition-emission -p semio-framework-plugin-host-guest-runtime -p semio-framework-plugin-host-imports -p semio-framework-plugin-tool-run -p semio-framework-plugin-composition-publication -p semio-framework-plugin-document-media -p semio-framework-plugin-host-shard -p semio-framework-plugin-time-travel-panel -p semio-framework-plugin-host-post-turn-relay -p semio-framework-plugin-host-shard-executor -p semio-framework-plugin-publication-mounted -p semio-framework-plugin-reactor-app -p semio-framework-plugin-retained-command -p semio-framework-plugin-time-travel -p semio-framework-plugin-host-routers -p semio-framework-plugin-instance-stores -p semio-framework-plugin-reactor-lifecycle -p semio-framework-plugin-host -p semio-framework-plugin-instance-dispatch -p semio-framework-plugin-reactor-patches -p semio-framework-plugin-runtime-instances --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-plugin-tool-run-turns -p semio-framework-os-renderer-wgpu-kernel-pool -p semio-framework-os-renderer-wgpu-shell-async -p semio-framework-plugin-extension -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-runtime-ops -p semio-framework-plugin-time-travel-turns -p semio-framework-os-renderer-wgpu-shell-tests-chrome-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-3 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-6 -p semio-framework-plugin-reactor-jobs -p semio-framework-plugin-reactor-surfaces -p semio-framework-plugin-runtime-exchange -p semio-framework-plugin-instance-app -p semio-framework-plugin-reactor-core -p semio-framework-plugin-builder -p semio-framework-plugin-reactor-turn -p semio-framework-plugin-reactor-wit-bridge -p semio-framework-plugin-testing-laws -p semio-framework-plugin-reactor-context -p semio-framework-plugin-tests-01 -p semio-framework-plugin-tests-02 -p semio-framework-plugin-tests-05 -p semio-framework-plugin -p semio-framework-os-flow-extensions --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-plugin-tool-run-turns -p semio-framework-os-renderer-wgpu-kernel-pool -p semio-framework-os-renderer-wgpu-shell-async -p semio-framework-plugin-extension -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-runtime-ops -p semio-framework-plugin-time-travel-turns -p semio-framework-os-renderer-wgpu-shell-tests-chrome-2 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-3 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-6 -p semio-framework-plugin-reactor-jobs -p semio-framework-plugin-reactor-surfaces -p semio-framework-plugin-runtime-exchange -p semio-framework-plugin-instance-app -p semio-framework-plugin-reactor-core -p semio-framework-plugin-builder -p semio-framework-plugin-reactor-turn -p semio-framework-plugin-reactor-wit-bridge -p semio-framework-plugin-testing-laws -p semio-framework-plugin-reactor-context -p semio-framework-plugin-tests-01 -p semio-framework-plugin-tests-02 -p semio-framework-plugin-tests-05 -p semio-framework-plugin -p semio-framework-os-flow-extensions --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-mcp-plugin-runtime -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-run -p semio-framework-plugin-host-test-component -p semio-framework-print -p semio-framework-os-flow-host-core -p semio-framework-os-mcp-routing -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-scene-render -p semio-framework-print-tests -p semio-framework-os-flow-host-session -p semio-framework-os-flow-wasm-actions-a -p semio-framework-os-flow-wasm-actions-b -p semio-framework-os-mcp-workspace -p semio-framework-os-renderer-wgpu-frame-job -p semio-framework-os-renderer-wgpu-ui-engine -p semio-framework-os-flow-host -p semio-framework-os-flow-wasm -p semio-framework-os-mcp-server -p semio-framework-os-mcp-tools-gateway -p semio-framework-os-renderer-wgpu-frame-presentation -p semio-framework-os-renderer-wgpu-scene-ink-tests -p semio-framework-os-mcp-http-transport -p semio-framework-os-renderer-wgpu-host-winit -p semio-framework-os-mcp --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-mcp-plugin-runtime -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-run -p semio-framework-plugin-host-test-component -p semio-framework-print -p semio-framework-os-flow-host-core -p semio-framework-os-mcp-routing -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-scene-render -p semio-framework-print-tests -p semio-framework-os-flow-host-session -p semio-framework-os-flow-wasm-actions-a -p semio-framework-os-flow-wasm-actions-b -p semio-framework-os-mcp-workspace -p semio-framework-os-renderer-wgpu-frame-job -p semio-framework-os-renderer-wgpu-ui-engine -p semio-framework-os-flow-host -p semio-framework-os-flow-wasm -p semio-framework-os-mcp-server -p semio-framework-os-mcp-tools-gateway -p semio-framework-os-renderer-wgpu-frame-presentation -p semio-framework-os-renderer-wgpu-scene-ink-tests -p semio-framework-os-mcp-http-transport -p semio-framework-os-renderer-wgpu-host-winit -p semio-framework-os-mcp --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path Cargo.toml -p semio-framework-os-renderer-wgpu-browser-worker -p semio-framework-os-renderer-wgpu-ui-engine-tests -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-engine-attach-tests -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests -p semio-framework-os-renderer-wgpu-shell-tests-chrome-1 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-4 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-5 -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-shell-tests-docs-3 -p semio-framework-os-renderer-wgpu-shell-tests-input-1 -p semio-framework-os-renderer-wgpu-shell-tests-input-2 -p semio-framework-os-renderer-wgpu-shell-tests-input-3 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 -p semio-framework-os-renderer-wgpu-shell-tests-windows-2 --lib --keep-going --message-format=short
pl check --manifest-path Cargo.toml -p semio-framework-os-renderer-wgpu-browser-worker -p semio-framework-os-renderer-wgpu-ui-engine-tests -p semio-framework-os-renderer-wgpu -p semio-framework-os-renderer-wgpu-engine-attach-tests -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests -p semio-framework-os-renderer-wgpu-shell-tests-chrome-1 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-4 -p semio-framework-os-renderer-wgpu-shell-tests-chrome-5 -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-shell-tests-docs-3 -p semio-framework-os-renderer-wgpu-shell-tests-input-1 -p semio-framework-os-renderer-wgpu-shell-tests-input-2 -p semio-framework-os-renderer-wgpu-shell-tests-input-3 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 -p semio-framework-os-renderer-wgpu-shell-tests-windows-2 --tests --keep-going --message-format=short
```

### 3.3 ✏️s/🔌️plugins/🗄️stdio/Cargo.toml (`✏️s/🔌️plugins/🗄️stdio/Cargo.toml`): 12 crates, 12 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml -p semio-s-artifact-stdio-contract-part21 -p semio-s-artifact-stdio-contract-list-delta -p semio-s-artifact-stdio-contract-native-print -p semio-s-artifact-stdio-contract-assembly -p semio-s-artifact-stdio-contract-media-export -p semio-s-artifact-stdio-contract-testing -p semio-s-artifact-stdio-contract-wire -p semio-s-artifact-stdio-contract-window-kit -p semio-s-artifact-stdio-contract-editing -p semio-s-artifact-stdio-contract-editing-details -p semio-s-artifact-stdio-contract-editing-patch -p semio-s-artifact-stdio-contract --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🗄️stdio/Cargo.toml -p semio-s-artifact-stdio-contract-part21 -p semio-s-artifact-stdio-contract-list-delta -p semio-s-artifact-stdio-contract-native-print -p semio-s-artifact-stdio-contract-assembly -p semio-s-artifact-stdio-contract-media-export -p semio-s-artifact-stdio-contract-testing -p semio-s-artifact-stdio-contract-wire -p semio-s-artifact-stdio-contract-window-kit -p semio-s-artifact-stdio-contract-editing -p semio-s-artifact-stdio-contract-editing-details -p semio-s-artifact-stdio-contract-editing-patch -p semio-s-artifact-stdio-contract --tests --keep-going --message-format=short
```

### 3.4 ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml (`✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml -p semio-s-space-core --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🪐️space/🫀️core/Cargo.toml -p semio-s-space-core --tests --keep-going --message-format=short
```

### 3.5 ✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml (`✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml`): 4 crates, 4 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml -p semio-s-procedural-core-generation-sqlite-test-support -p semio-s-procedural-core-generation-sqlite -p semio-s-procedural-core-generation-sqlite-native -p semio-s-procedural-core-semantic-ui --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml -p semio-s-procedural-core-generation-sqlite-test-support -p semio-s-procedural-core-generation-sqlite -p semio-s-procedural-core-generation-sqlite-native -p semio-s-procedural-core-semantic-ui --tests --keep-going --message-format=short
```

### 3.6 ✏️s/🔌️plugins/🔋️energy/Cargo.toml (`✏️s/🔌️plugins/🔋️energy/Cargo.toml`): 3 crates, 3 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml -p semio-s-plugin-energy-engine-sim -p semio-s-plugin-energy-engine-bestest -p semio-s-plugin-energy-engine-sim-restore --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🔋️energy/Cargo.toml -p semio-s-plugin-energy-engine-sim -p semio-s-plugin-energy-engine-bestest -p semio-s-plugin-energy-engine-sim-restore --tests --keep-going --message-format=short
```

### 3.7 ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml (`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml -p semio-s-artifact-writer-writer --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml -p semio-s-artifact-writer-writer --tests --keep-going --message-format=short
```

### 3.8 ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml (`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml -p semio-s-artifact-mathematical-equation --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.toml -p semio-s-artifact-mathematical-equation --tests --keep-going --message-format=short
```

### 3.9 ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml -p semio-s-artifact-flow-flow --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml -p semio-s-artifact-flow-flow --tests --keep-going --message-format=short
```

### 3.10 ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml (`✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml -p semio-sequence-mutation-bridge --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.toml -p semio-sequence-mutation-bridge --tests --keep-going --message-format=short
```

### 3.11 ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml (`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml`): 6 crates, 6 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml -p semio-s-artifact-sequence-sequence-schema -p semio-s-artifact-sequence-sequence-inferences -p semio-s-artifact-sequence-sequence-tests -p semio-s-artifact-sequence-sequence-editor -p semio-s-artifact-sequence-sequence-v1 -p semio-s-artifact-sequence-sequence --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.toml -p semio-s-artifact-sequence-sequence-schema -p semio-s-artifact-sequence-sequence-inferences -p semio-s-artifact-sequence-sequence-tests -p semio-s-artifact-sequence-sequence-editor -p semio-s-artifact-sequence-sequence-v1 -p semio-s-artifact-sequence-sequence --tests --keep-going --message-format=short
```

### 3.12 ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml`): 4 crates, 4 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml -p semio-s-artifact-fem-3d-editor-visual-job -p semio-s-artifact-fem-3d-editor-model-window -p semio-s-artifact-fem-3d-editor-session -p semio-s-artifact-fem-3d-viewer --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml -p semio-s-artifact-fem-3d-editor-visual-job -p semio-s-artifact-fem-3d-editor-model-window -p semio-s-artifact-fem-3d-editor-session -p semio-s-artifact-fem-3d-viewer --tests --keep-going --message-format=short
```

### 3.13 ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml (`✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --tests --keep-going --message-format=short
```

### 3.14 ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml (`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml`): 5 crates, 5 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml -p semio-s-artifact-process-process3d-wasm -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-modes -p semio-s-artifact-process-process3d-panels -p semio-s-artifact-process-process3d-editor --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml -p semio-s-artifact-process-process3d-wasm -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-modes -p semio-s-artifact-process-process3d-panels -p semio-s-artifact-process-process3d-editor --tests --keep-going --message-format=short
```

### 3.15 ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml`): 9 crates, 9 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml -p semio-s-artifact-lowpoly-lowpoly-mut-geometry -p semio-s-artifact-lowpoly-lowpoly-mut-laws -p semio-s-artifact-lowpoly-lowpoly-mut-objects -p semio-s-artifact-lowpoly-lowpoly-mut-paint -p semio-s-artifact-lowpoly-lowpoly-commands -p semio-s-artifact-lowpoly-lowpoly-options -p semio-s-artifact-lowpoly-lowpoly-panels -p semio-s-artifact-lowpoly-lowpoly-modes -p semio-s-artifact-lowpoly-lowpoly-editor --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml -p semio-s-artifact-lowpoly-lowpoly-mut-geometry -p semio-s-artifact-lowpoly-lowpoly-mut-laws -p semio-s-artifact-lowpoly-lowpoly-mut-objects -p semio-s-artifact-lowpoly-lowpoly-mut-paint -p semio-s-artifact-lowpoly-lowpoly-commands -p semio-s-artifact-lowpoly-lowpoly-options -p semio-s-artifact-lowpoly-lowpoly-panels -p semio-s-artifact-lowpoly-lowpoly-modes -p semio-s-artifact-lowpoly-lowpoly-editor --tests --keep-going --message-format=short
```

### 3.16 ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml (`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml -p semio-s-artifact-reasoning-wires --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml -p semio-s-artifact-reasoning-wires --tests --keep-going --message-format=short
```

### 3.17 ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml (`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml -p semio-s-artifact-forms-forms --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml -p semio-s-artifact-forms-forms --tests --keep-going --message-format=short
```

### 3.18 ✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml (`✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml -p semio-layout-mutation-bridge --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.toml -p semio-layout-mutation-bridge --tests --keep-going --message-format=short
```

### 3.19 ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml`): 26 crates, 26 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout-editor-commands-tests -p semio-s-artifact-layout-layout-mutations-content-tests-1 -p semio-s-artifact-layout-layout-mutations-content-tests-2 -p semio-s-artifact-layout-layout-mutations-frames-tests-1 -p semio-s-artifact-layout-layout-mutations-frames-tests-2 -p semio-s-artifact-layout-layout-descriptor-diff -p semio-s-artifact-layout-layout-descriptor -p semio-s-artifact-layout-layout-examples -p semio-s-artifact-layout-layout-model -p semio-s-artifact-layout-layout-diff -p semio-s-artifact-layout-layout-engine -p semio-s-artifact-layout-layout-engine-json -p semio-s-artifact-layout-layout-engine-export -p semio-s-artifact-layout-layout-mutations-content-2 -p semio-s-artifact-layout-layout-mutations-frames-2 -p semio-s-artifact-layout-layout-mutations-content -p semio-s-artifact-layout-layout-mutations-frames -p semio-s-artifact-layout-layout-mutations -p semio-s-artifact-layout-layout-editor-commands-2 -p semio-s-artifact-layout-layout-host -p semio-s-artifact-layout-layout-io -p semio-s-artifact-layout-layout-viewer -p semio-s-artifact-layout-layout-editor-commands -p semio-s-artifact-layout-layout-editor-panels -p semio-s-artifact-layout-layout-editor --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout-editor-commands-tests -p semio-s-artifact-layout-layout-mutations-content-tests-1 -p semio-s-artifact-layout-layout-mutations-content-tests-2 -p semio-s-artifact-layout-layout-mutations-frames-tests-1 -p semio-s-artifact-layout-layout-mutations-frames-tests-2 -p semio-s-artifact-layout-layout-descriptor-diff -p semio-s-artifact-layout-layout-descriptor -p semio-s-artifact-layout-layout-examples -p semio-s-artifact-layout-layout-model -p semio-s-artifact-layout-layout-diff -p semio-s-artifact-layout-layout-engine -p semio-s-artifact-layout-layout-engine-json -p semio-s-artifact-layout-layout-engine-export -p semio-s-artifact-layout-layout-mutations-content-2 -p semio-s-artifact-layout-layout-mutations-frames-2 -p semio-s-artifact-layout-layout-mutations-content -p semio-s-artifact-layout-layout-mutations-frames -p semio-s-artifact-layout-layout-mutations -p semio-s-artifact-layout-layout-editor-commands-2 -p semio-s-artifact-layout-layout-host -p semio-s-artifact-layout-layout-io -p semio-s-artifact-layout-layout-viewer -p semio-s-artifact-layout-layout-editor-commands -p semio-s-artifact-layout-layout-editor-panels -p semio-s-artifact-layout-layout-editor --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout --tests --keep-going --message-format=short
```

### 3.20 ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml`): 3 crates, 3 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-cad-cad-panels -p semio-s-artifact-cad-cad-transform --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-cad-cad-panels -p semio-s-artifact-cad-cad-transform --tests --keep-going --message-format=short
```

### 3.21 ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml`): 4 crates, 4 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml -p semio-s-artifact-norm-vdi3805-inferences -p semio-s-artifact-norm-vdi3805-editor -p semio-s-artifact-norm-vdi3805-viewer -p semio-s-artifact-norm-vdi3805 --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml -p semio-s-artifact-norm-vdi3805-inferences -p semio-s-artifact-norm-vdi3805-editor -p semio-s-artifact-norm-vdi3805-viewer -p semio-s-artifact-norm-vdi3805 --tests --keep-going --message-format=short
```

### 3.22 ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml`): 5 crates, 5 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-inferences -p semio-s-artifact-norm-en1993-editor -p semio-s-artifact-norm-en1993-viewer -p semio-s-artifact-norm-en1993 --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.toml -p semio-s-artifact-norm-en1993-checks -p semio-s-artifact-norm-en1993-inferences -p semio-s-artifact-norm-en1993-editor -p semio-s-artifact-norm-en1993-viewer -p semio-s-artifact-norm-en1993 --tests --keep-going --message-format=short
```

### 3.23 ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml`): 5 crates, 5 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml -p semio-s-artifact-norm-en1999-checks -p semio-s-artifact-norm-en1999-inferences -p semio-s-artifact-norm-en1999-editor -p semio-s-artifact-norm-en1999-viewer -p semio-s-artifact-norm-en1999 --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.toml -p semio-s-artifact-norm-en1999-checks -p semio-s-artifact-norm-en1999-inferences -p semio-s-artifact-norm-en1999-editor -p semio-s-artifact-norm-en1999-viewer -p semio-s-artifact-norm-en1999 --tests --keep-going --message-format=short
```

### 3.24 ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml (`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml`): 5 crates, 5 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml -p semio-s-artifact-remodel-remodeling-mut-calib -p semio-s-artifact-remodel-remodeling-mut-params -p semio-s-artifact-remodel-remodeling-mut-results -p semio-s-artifact-remodel-remodeling-mut-vectors -p semio-s-artifact-remodel-remodeling-mut-media --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml -p semio-s-artifact-remodel-remodeling-mut-calib -p semio-s-artifact-remodel-remodeling-mut-params -p semio-s-artifact-remodel-remodeling-mut-results -p semio-s-artifact-remodel-remodeling-mut-vectors -p semio-s-artifact-remodel-remodeling-mut-media --tests --keep-going --message-format=short
```

### 3.25 ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml (`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml`): 4 crates, 4 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml -p semio-s-artifact-energy-model-session -p semio-s-artifact-energy-model-mutations -p semio-s-artifact-energy-model-editor-edit -p semio-s-artifact-energy-model-viewer --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml -p semio-s-artifact-energy-model-session -p semio-s-artifact-energy-model-mutations -p semio-s-artifact-energy-model-editor-edit -p semio-s-artifact-energy-model-viewer --tests --keep-going --message-format=short
```

### 3.26 ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml -p semio-s-artifact-trinity-jack-inferences --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.toml -p semio-s-artifact-trinity-jack-inferences --tests --keep-going --message-format=short
```

### 3.27 ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml (`✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml -p semio-dag-mutation-bridge --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.toml -p semio-dag-mutation-bridge --tests --keep-going --message-format=short
```

### 3.28 ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml`): 9 crates, 9 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml -p semio-s-artifact-dag-dag-inferences -p semio-s-artifact-dag-dag-diff -p semio-s-artifact-dag-dag-config -p semio-s-artifact-dag-dag-tests -p semio-s-artifact-dag-dag-schema -p semio-s-artifact-dag-dag-document-behavior-tests -p semio-s-artifact-dag-dag-dag -p semio-s-artifact-dag-dag-v1 -p semio-s-artifact-dag-dag --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.toml -p semio-s-artifact-dag-dag-inferences -p semio-s-artifact-dag-dag-diff -p semio-s-artifact-dag-dag-config -p semio-s-artifact-dag-dag-tests -p semio-s-artifact-dag-dag-schema -p semio-s-artifact-dag-dag-document-behavior-tests -p semio-s-artifact-dag-dag-dag -p semio-s-artifact-dag-dag-v1 -p semio-s-artifact-dag-dag --tests --keep-going --message-format=short
```

### 3.29 ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml (`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml`): 4 crates, 4 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml -p semio-s-artifact-space-space-schema -p semio-s-artifact-space-space-diff -p semio-s-artifact-space-space-mutations -p semio-s-artifact-space-space-editor --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.toml -p semio-s-artifact-space-space-schema -p semio-s-artifact-space-space-diff -p semio-s-artifact-space-space-mutations -p semio-s-artifact-space-space-editor --tests --keep-going --message-format=short
```

### 3.30 ✏️s/Cargo.toml (`✏️s/Cargo.toml`): 2 crates, 2 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/Cargo.toml -p semio-s-fem-analyses-model -p semio-s-spatial-kernel-semio-session --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/Cargo.toml -p semio-s-fem-analyses-model -p semio-s-spatial-kernel-semio-session --tests --keep-going --message-format=short
```

### 3.31 ✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml (`✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml -p semio-s-dev-trinity-jack --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.toml -p semio-s-dev-trinity-jack --tests --keep-going --message-format=short
```

### 3.32 ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml (`✏️s/🧑‍💻dev/🧩️composition/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml -p semio-s-composition-laws --lib --keep-going --message-format=short
pl check --manifest-path ✏️s/🧑‍💻dev/🧩️composition/Cargo.toml -p semio-s-composition-laws --tests --keep-going --message-format=short
```

### 3.33 🎓️teaching/Cargo.toml (`🎓️teaching/Cargo.toml`): 1 crates, 1 with `--tests`, 0 lib-only

```sh
pl check --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --lib --keep-going --message-format=short
pl check --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --tests --keep-going --message-format=short
```

### 3.34 🌎️hub/Cargo.toml (`🌎️hub/Cargo.toml`): 36 crates, 36 with `--tests`, 0 lib-only

```sh
pl check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-boot-laws -p semio-hub-directory-backend-laws -p semio-hub-directory-laws -p semio-hub-directory-route-laws -p semio-hub-hostile-input-laws -p semio-hub-open-plan-laws -p semio-hub-presence-route-laws -p semio-hub-session-route-laws -p semio-hub-trusted-catalog-laws -p semio-hub-testing -p semio-hub-directory-contract -p semio-hub-directory -p semio-hub-directory-neo4j-rows -p semio-hub-directory-neo4j -p semio-hub-directory-postgres -p semio-hub-directory-sqlite -p semio-hub-directory-dispatch -p semio-hub-directory-service -p semio-hub-lag-rebootstrap -p semio-hub-local-bootstrap -p semio-hub-stores -p semio-hub-artifact-creation-service -p semio-hub-ledgers -p semio-hub-route-models -p semio-hub-gis --lib --keep-going --message-format=short
pl check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-boot-laws -p semio-hub-directory-backend-laws -p semio-hub-directory-laws -p semio-hub-directory-route-laws -p semio-hub-hostile-input-laws -p semio-hub-open-plan-laws -p semio-hub-presence-route-laws -p semio-hub-session-route-laws -p semio-hub-trusted-catalog-laws -p semio-hub-testing -p semio-hub-directory-contract -p semio-hub-directory -p semio-hub-directory-neo4j-rows -p semio-hub-directory-neo4j -p semio-hub-directory-postgres -p semio-hub-directory-sqlite -p semio-hub-directory-dispatch -p semio-hub-directory-service -p semio-hub-lag-rebootstrap -p semio-hub-local-bootstrap -p semio-hub-stores -p semio-hub-artifact-creation-service -p semio-hub-ledgers -p semio-hub-route-models -p semio-hub-gis --tests --keep-going --message-format=short
```

```sh
pl check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-inference-catalog -p semio-hub-note -p semio-hub-vcs -p semio-hub-bim -p semio-hub-inference-runtime -p semio-hub-state -p semio-hub -p semio-hub-session-routes -p semio-hub-document-routes -p semio-hub-route-testkit -p semio-hub-stdio --lib --keep-going --message-format=short
pl check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-inference-catalog -p semio-hub-note -p semio-hub-vcs -p semio-hub-bim -p semio-hub-inference-runtime -p semio-hub-state -p semio-hub -p semio-hub-session-routes -p semio-hub-document-routes -p semio-hub-route-testkit -p semio-hub-stdio --tests --keep-going --message-format=short
```

## 4. Feature and target lanes (plain batches above do not carry these)

Run each line as its own `pl` call. Each `--target` lane is one call per target. The source lines name the RESULT that requested the lane.

```sh
# framework wasm (runbook 1e, RESULT-FRAMEWORK-RUST-FILE-SPLITS line 109, RESULT-SCHEMA-VALUE-MATH-SPLITS line 131, RESULT-TOOL-TEST-COMPANIONS line 110, RESULT-PACK-QUIZ-FILE-SPLITS line 141)
pl check -p semio-framework-number -p semio-framework-async --lib --target wasm32-wasip2 --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-number -p semio-framework-async --lib --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-value-resident -p semio-framework-math-sampling-processors -p semio-framework-math-sampling-constraints --lib --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-tool-run -p semio-framework-tool-machine --lib --target wasm32-wasip2 --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-tool-run -p semio-framework-tool-machine --lib --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-pack-record -p semio-framework-pack-format-catalog --lib --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-pack-record -p semio-framework-pack-format-catalog --lib --target wasm32-wasip2 --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-ui-wgpu-device -p semio-framework-ui-wgpu-draw-execution -p semio-framework-ui-wgpu-mesh -p semio-framework-ui-wgpu-pipelines -p semio-framework-ui-wgpu-raster --lib --target wasm32-unknown-unknown --manifest-path 🧰️framework/Cargo.toml --keep-going --message-format=short
pl check -p semio-framework-canvas-assets --lib --target wasm32-wasip2 --manifest-path 🧰️framework/Cargo.toml   # runbook 1b, dead_code expectation
# feature lanes
pl check -p semio-framework-replication --lib --features deflate --manifest-path 🧰️framework/Cargo.toml          # runbook 1b
pl check -p semio-framework-graph-layout-run --features testing --lib --manifest-path 🧰️framework/Cargo.toml       # RESULT-TOOL-GRAPH-MACHINE-SPLITS line 156
pl check -p semio-framework-quiz-schema --lib --features testkit --keep-going --message-format=short              # runbook 8d (root)
pl check -p semio-framework-os-directory-client-tests --lib --tests --features ureq,sync,testkit                  # RESULT-CENSUS5-C9 line 72 (root)
pl check -p semio-framework-os-directory-client --lib --target wasm32-unknown-unknown                             # RESULT-CENSUS5-C9 line 72, browser arm (root)
pl check -p semio-hub-directory-postgres -p semio-hub-directory-neo4j --lib --features semio-hub-directory-postgres/testkit,semio-hub-directory-neo4j/testkit --manifest-path 🌎️hub/Cargo.toml   # runbook 9b
# plugin lanes
pl check -p semio-s-artifact-process-process3d-host -p semio-s-artifact-process-process3d-wasm --lib --target wasm32-wasip2 --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml   # runbook 7h
pl check -p semio-s-spatial-kernel-semio-session --lib --target wasm32-unknown-unknown --manifest-path ✏️s/Cargo.toml   # runbook 7q
pl check -p semio-framework-os-renderer-wgpu-asset-decode -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-frame-presentation -p semio-framework-os-renderer-wgpu-frame-job -p semio-framework-os-renderer-wgpu-host-winit --lib --target wasm32-unknown-unknown   # RESULT-WGPU section 4 (starred crates), root
pl check -p semio-framework-os-renderer-wgpu-browser-worker --lib --target wasm32-unknown-unknown                   # RESULT-WGPU section 4, wasm only, root
pl check -p semio-framework-os-renderer-wgpu-ui-engine-dump --lib --target wasm32-unknown-unknown                   # runbook 6c, root
```

Plugin-host and kernel wasm/testkit lanes named in the RESULTs (`semio-framework-plugin-host-effects`, `-guest-runtime`, `-shard` with `--features testkit`; `semio-framework-plugin-retained-command`, `semio-framework-plugin-child` with `artifact-app-testing`; `semio-framework-os-kernel-dsl-derive-expand` with `testkit`; the 12 WP0 new crates on `wasm32-unknown-unknown` and `wasm32-wasip2`; `semio-framework-os-mcp-hub-remote` and `semio-framework-os-mcp-bridge-codec` on wasm32; `semio-framework-os-pack` on wasm32 and wasip2) are in the same `pl` form with the feature or target added. Their exact lines are in runbook sections 3, 4 and 5 and in RESULT-PLUGIN-HOST-CARVE, RESULT-PLUGIN-INTEGRATION-1 section 8 item 2, RESULT-CENSUS5-G line 88.

Test runs (`pl test -p … --lib`, `--test <name>`, the `test -p semio-framework-value-derive` corpus run with `SVM_CORPUS_OUT`) are unchanged from runbook sections 1f, 3b, 3c, 3d, 6, 7e, 8d and 9e. They are not regenerated here.

## 5. Names not in the plan (unknown, nonexistent, or not resolvable from the sources)

- `semio-tech`: npm scope in prose, not a Cargo package.
- `semio-mut-family-2`: appears only as a rename note in COMPILE-DEFECT-LEDGER ("now `-semio-mut-family-2`"). No package by that name exists.
- Unresolved, not expanded (the named source does not contain the list):
  - RESULT-OS-KERNEL-SQLITE-HOST.md, "seven crates": the file names the dependency checks but not the seven package names. Only the names in the runbook are in the plan (`semio-framework-os-store-sync-native`, `semio-framework-os-services`, `semio-framework-os-channel`).
  - RESULT-RENDERER-INTEGRATION-2.md line 90 (31-crate shell batch): the list is passed to `ri2-tools/cargo_batch.sh` by its caller and is not in the RESULT. The plan has the renderer shell crates from the runbook's 6a and 6b only.
  - RESULT-DUPLICATE-MOUNTS-2.md line 147 and RESULT-FRAMEWORK-TEST-COMPANIONS line 72 were expanded by name, and all their names exist.
  - led2 "unmounted draft": the `🏗️construction/🧪️tests/🦀️.rs` test with the edited `include_str!` path is not attributed to a package in result-led2.md. Compile it with its owner once named.
  - `🗑️generated/cp2-worklist-fw.txt` (the runbook's 240-member framework list) is missing from the ticket folder. Replaced by the 359 framework members of `🧰️framework/Cargo.toml`: 101 with `--tests`, 258 lib-only (runbook 1c).
  - RESULT-OS-KERNEL-WP8: the 14 test crates are taken as the 10 `os-store-tests-01..10`, `os-store-sync-tests-1/2`, `os-store-member-open-tests`, `os-store-durable-group-tests` (the composition of RESULT-OS-KERNEL-WP8 line 22). Confirm against that RESULT.
  - WP0 "12 new crates" (okwp0-tools/touched.txt): 11 names derived from the package directories of the touched files, not from a list. Confirm against RESULT-OS-KERNEL-WP0.
  - Layout, dag, sequence and bridge crates (checklist step 8): derived from the manifests' members, not from a RESULT list.

## 6. Corrections to the runbook found by this audit

1. `✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml` does not exist. The `semio-s-procedural-core-*` crates (generation-sqlite, -native, -test-support, semantic-ui) are members of `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.toml`. Runbook 7d uses the wrong manifest. The plan uses foundation.
2. `semio-s-artifact-stdio-contract-testing` is a member of `✏️s/🔌️plugins/🗄️stdio/Cargo.toml`, not of `✏️s/Cargo.toml` (runbook 7a). The plan uses the stdio manifest.
3. `teaching-proctor` belongs to `🎓️teaching/Cargo.toml` and not to root (runbook 8c, coordinator resolution 1). The plan puts it only in the teaching batch.
4. Runbook lines that default to ROOT but name framework crates (1e wasm lines, 3b wasm line, 4d mixed batch): `semio-framework-ui-wgpu-device`, `-draw-execution`, `-mesh`, `-pipelines`, `-raster`, `semio-framework-ui-runtime-reconcile`, `-primitives`, `semio-framework-pack-record`, `semio-framework-pack-format-catalog` are framework members. `semio-framework-ui-runtime-reconcile` is not listed by the root workspace. `ui-wgpu-mesh` is listed by both root and framework; the framework one is the nearest and is the one runbook 1e uses. The 4d batch mixes root and framework crates under one manifest and must be split. The plan does so.
5. Runbook 2.4 gives the root lock refresh without `--manifest-path`. The plan passes `--manifest-path Cargo.toml` explicitly.

## 7. Ordering and method

- Workspace order is the runbook phase order: framework (leaves), root (OS kernel, store, pack, plugin host, renderer), plugin workspaces (stdio, space core, procedural foundation, energy, then the rest), `✏️s` root and `🧑‍💻dev` items, teaching, hub. The workspace graph is not acyclic: root and plugin workspaces depend on each other through normal and dev dependencies and bridge crates. A strict workspace topological order does not exist, so the runbook order is used.
- Inside each workspace, crates are sorted by the normal and build dependency level among the 784 crates, computed from the `Cargo.toml` dependency keys (leaves first). Cargo resolves dependencies inside each `-p` call regardless.
- Ownership: each crate is assigned to the deepest workspace whose `members` glob matches its directory and whose `exclude` does not. A crate with its own `[workspace]` is its own owner.
- Verified by machine: 784 names exist as `[package] name` in non-ticket manifests (ticket copies excluded). 119 check calls, 34 lock refreshes, every batch at most 25 `-p` names.
