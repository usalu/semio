# Audit Gates 1019: Crate Source Gate And Hand-Authored Census Re-Run

Auditor `aud-gates`, 2026-10-19 (18:0x CEST). Read-only on source. No builds, no `bun repo`, no git mutations.

## 0. Scope, method, deviations

- Working tree measured (uncommitted splits included). HEAD is `bd9e4e02bd0` (12:21); the working tree differs in 1,760 `.rs`/`.ts`/`.toml` files (`git status` 3,415 entries).
- Crate gate: `crate-gate8-disk.ts`, unchanged logic, run from a copy in `🗑️generated/aud-gates-tools/` (the original writes `crates8.disk.N.json` into `aud-census8-tools/`, which would overwrite census-8 outputs). Gate = `storage.sourceBoundary` of `⚡️caching/🔣️policy.json` at HEAD: 32 files, 262,144 bytes, 4,000 active lines. Reader `readRustSourceClosureV1`, prod cfg `crateTreeHostCfg()`, test cfg = prod + `test`. 8 shards run with `bun 1.3.14` (`SHARD=0..7 NSHARD=8`). Ratio = max(files/32, bytes/262144, lines/4000); dim = arg max.
- Hand-authored census: a copy of `census8.py` run with `python3 -I`, copied to `🗑️generated/aud-gates-tools/census8.py` with the census-7 path made absolute. Run as-is it reads `git ls-tree HEAD` and returns the census-8 numbers unchanged (183,879 blobs, 17 over 1,000). That result is HEAD, not the working tree, so I added `census8-wt.py` (same logic, file list and bytes from disk, `git ls-files --cached --others --exclude-standard`). Both outputs are in `🗑️generated/aud-gates-tools/`.
- Targets: 3453 crate targets (lib and bin) over 3398 package manifests, 0 skipped. Gate outputs: `gates-merged.json`, `census8-wt.json`, `new-crate-crosscheck.json`.

## 1. Headline

| Measure | Prod cfg | Test cfg | Either |
|---|---:|---:|---:|
| Over 1.00 (refused by gate) | 0 | 8 | 8 |
| 0.90 to 1.00 | 13 | 151 | 151 |
| Refused by reader (no ratio) | 0 | 12 | 12 |

All 8 over-gate targets are test-cfg only. No production-cfg crate exceeds the gate. Nothing is refused in prod cfg.

## 2. Crates over 1.00 (refused)

Owner = `📋️build-owners.json` project name (`owners[].name`) covering the crate directory.

| # | Crate | Kind | Owner (executor) | Prod ratio (dim) | Test ratio (dim) | Manifest |
|---:|---|---|---|---|---|---|
| 1 | `teaching-proctor-base` | lib | teaching-proctor-base | 0.493 (lines) | 2.876 (bytes) | `🎓️teaching/🛂️proctor/🔨️modules/🧱️base/📦️packages/🦀️rust/Cargo.toml` |
| 2 | `semio-s-plugin-energy-engine-precompute` | lib | semio-s-plugin-energy-engine-precompute | 0.272 (lines) | 1.376 (bytes) | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/📦️packages/🦀️rust/Cargo.toml` |
| 3 | `semio-s-plugin-energy-engine-sim-unit-tests` | lib | semio-s-plugin-energy-engine-sim-unit-tests | 0.220 (bytes) | 1.311 (bytes) | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 4 | `semio-s-plugin-energy-engine-sim-restore` | lib | semio-s-plugin-energy-engine-sim-restore | 0.148 (lines) | 1.284 (bytes) | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️restore/📦️packages/🦀️rust/Cargo.toml` |
| 5 | `semio-s-plugin-energy-engine-site` | lib | semio-s-plugin-energy-engine-site | 0.087 (lines) | 1.177 (bytes) | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/📦️packages/🦀️rust/Cargo.toml` |
| 6 | `semio-framework-repo-dashboard-terminal` | lib | @semio-tech/repo-dashboard-terminal-rs | 0.596 (lines) | 1.156 (bytes) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust/Cargo.toml` |
| 7 | `semio-framework-replication-history` | lib | @semio-tech/replication-history-rs | 0.669 (bytes) | 1.131 (bytes) | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📦️packages/🦀️rust/Cargo.toml` |
| 8 | `semio-framework-plugin-window-config` | lib | @semio-tech/plugin-window-config-rs | 0.878 (bytes) | 1.025 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📦️packages/🦀️rust/Cargo.toml` |

Distinct owners of the over-gate crates: @semio-tech/plugin-window-config-rs, @semio-tech/replication-history-rs, @semio-tech/repo-dashboard-terminal-rs, semio-s-plugin-energy-engine-precompute, semio-s-plugin-energy-engine-sim-restore, semio-s-plugin-energy-engine-sim-unit-tests, semio-s-plugin-energy-engine-site, teaching-proctor-base.

## 3. Crates 0.90 to 1.00 (band)

151 targets have max ratio in [0.90, 1.00] (either cfg). Sorted by max ratio.

| # | Crate | Owner | Prod ratio (dim) | Test ratio (dim) | Manifest |
|---:|---|---|---|---|---|
| 1 | `semio-framework-ui-retained-tree-tree` | @semio-tech/ui-retained-tree-tree-rs | 0.766 (lines) | 0.993 (lines) | `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/📦️packages/🦀️rust/Cargo.toml` |
| 2 | `semio-s-artifact-puzzle-5d-interaction` | semio-s-artifact-puzzle-5d-interaction | 0.915 (lines) | 0.987 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` |
| 3 | `semio-framework-value` | @semio-tech/value-rs | 0.835 (bytes) | 0.985 (bytes) | `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml` |
| 4 | `semio-s-artifact-puzzle-2d-mutations` | semio-s-artifact-puzzle-2d-mutations | 0.812 (files) | 0.985 (bytes) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/📦️packages/🦀️rust/Cargo.toml` |
| 5 | `semio-framework-plugin-decl-views` | @semio-tech/plugin-decl-views-rs | 0.911 (lines) | 0.984 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📦️packages/🦀️rust/Cargo.toml` |
| 6 | `semio-framework-job` | @semio-tech/framework-job-rs | 0.882 (bytes) | 0.981 (bytes) | `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml` |
| 7 | `semio-framework-os-store-replay` | @semio-tech/os-store-replay-rs | 0.790 (bytes) | 0.977 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📦️packages/🦀️rust/Cargo.toml` |
| 8 | `semio-framework-job-worker` | @semio-tech/framework-job-worker-rs | 0.489 (bytes) | 0.977 (bytes) | `🧰️framework/🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml` |
| 9 | `semio-s-artifact-fem-2d-model` | semio-s-artifact-fem-2d-model | 0.656 (files) | 0.969 (files) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 10 | `semio-s-artifact-writer-writer-tests-2` | semio-s-artifact-writer-writer-tests-2 | 0.750 (files) | 0.969 (files) | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 11 | `semio-framework-plugin-testing-fixtures` | @semio-tech/plugin-testing-fixtures-rs | 0.594 (files) | 0.969 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧰️fixtures/📦️packages/🦀️rust/Cargo.toml` |
| 12 | `semio-framework-value-retained-clone` | @semio-tech/value-retained-clone-rs | 0.570 (bytes) | 0.968 (bytes) | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml` |
| 13 | `semio-framework-ui-tui-core` | @semio-tech/ui-tui-core-rs | 0.669 (lines) | 0.959 (lines) | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧱️cell-model/📦️packages/🦀️rust/Cargo.toml` |
| 14 | `semio-s-artifact-energy-model-snapshot` | semio-s-artifact-energy-model-snapshot | 0.840 (bytes) | 0.958 (bytes) | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` |
| 15 | `semio-s-artifact-reasoning-wires-wires` | semio-s-artifact-reasoning-wires-wires | 0.467 (lines) | 0.956 (bytes) | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 16 | `semio-framework-actor` | @semio-tech/framework-actor-rs | 0.523 (bytes) | 0.954 (bytes) | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` |
| 17 | `semio-framework-plugin-instance-app` | @semio-tech/plugin-instance-app-rs | 0.949 (bytes) | 0.949 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust/Cargo.toml` |
| 18 | `semio-framework-os-kernel-db-io-driver` | @semio-tech/os-db-io-driver-rs | 0.462 (lines) | 0.949 (lines) | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/📦️packages/🦀️rust/Cargo.toml` |
| 19 | `semio-s-artifact-puzzle-5d-tests` | semio-s-artifact-puzzle-5d-tests | 0.789 (lines) | 0.944 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 20 | `semio-s-artifact-puzzle-2d-mut-any-10` | semio-s-artifact-puzzle-2d-mut-any-10 | 0.537 (lines) | 0.944 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/📦️packages/🦀️rust/Cargo.toml` |
| 21 | `semio-framework-value-intrinsic` | @semio-tech/value-intrinsic-rs | 0.666 (lines) | 0.943 (lines) | `🧰️framework/🔨️modules/🌱️value/🌳️intrinsic/📦️packages/🦀️rust/Cargo.toml` |
| 22 | `semio-s-artifact-wfc-bitmap-bitmap` | semio-s-artifact-wfc-bitmap-bitmap | 0.887 (lines) | 0.942 (lines) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 23 | `semio-s-artifact-puzzle-5d-dispatch` | semio-s-artifact-puzzle-5d-dispatch | 0.941 (lines) | 0.941 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎬️dispatch/📦️packages/🦀️rust/Cargo.toml` |
| 24 | `semio-framework-replication-mutation-retained` | @semio-tech/replication-mutation-retained-rs | 0.531 (bytes) | 0.940 (bytes) | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust/Cargo.toml` |
| 25 | `semio-framework-os-mcp-hub-remote` | @semio-tech/framework-os-mcp-hub-remote-rs | 0.581 (lines) | 0.939 (lines) | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌍️hub-remote/📦️packages/🦀️rust/Cargo.toml` |
| 26 | `semio-framework-ui-contract` | @semio-tech/ui-contract-rs | 0.744 (lines) | 0.939 (lines) | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` |
| 27 | `semio-s-artifact-mathematical-equation-snapshot` | semio-s-artifact-mathematical-equation-snapshot | 0.827 (lines) | 0.938 (lines) | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` |
| 28 | `semio-s-artifact-stdio-gif-editor` | semio-s-artifact-stdio-gif-editor | 0.562 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 29 | `semio-s-artifact-stdio-docx-base-mutations` | semio-s-artifact-stdio-docx-base-mutations | 0.938 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 30 | `semio-s-artifact-puzzle-2d-retained` | semio-s-artifact-puzzle-2d-retained | 0.906 (files) | 0.938 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📦️packages/🦀️rust/Cargo.toml` |
| 31 | `semio-s-artifact-stdio-xml-mutations` | semio-s-artifact-stdio-xml-mutations | 0.719 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 32 | `semio-framework-os-vcs` | @semio-tech/os-vcs-rs | 0.688 (files) | 0.938 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/📦️packages/🦀️rust/Cargo.toml` |
| 33 | `semio-s-artifact-stdio-xlsx-transitional` | semio-s-artifact-stdio-xlsx-transitional | 0.688 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust/Cargo.toml` |
| 34 | `semio-s-artifact-mathematical-equation-editor` | semio-s-artifact-mathematical-equation-editor | 0.844 (files) | 0.938 (files) | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 35 | `semio-s-artifact-stdio-binary-snapshot` | semio-s-artifact-stdio-binary-snapshot | 0.844 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` |
| 36 | `semio-s-artifact-fem-3d-model` | semio-s-artifact-fem-3d-model | 0.689 (bytes) | 0.938 (files) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 37 | `semio-s-artifact-stdio-semio-text-mutations` | semio-s-artifact-stdio-semio-text-mutations | 0.844 (files) | 0.938 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 38 | `semio-framework-pack-json` | @semio-tech/framework-pack-json-rs | 0.874 (bytes) | 0.935 (bytes) | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml` |
| 39 | `semio-framework-3d-brep-queries` | semio-framework-3d-brep-queries | 0.746 (lines) | 0.934 (lines) | `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/📦️packages/🦀️rust/Cargo.toml` |
| 40 | `semio-s-artifact-process-process3d-model` | semio-s-artifact-process-process3d-model | 0.897 (bytes) | 0.934 (bytes) | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 41 | `semio-framework-value-tests` | @semio-tech/value-tests-rs | 0.031 (files) | 0.932 (bytes) | `🧰️framework/🔨️modules/🌱️value/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml` |
| 42 | `semio-framework-os-renderer-wgpu-runtime-mailbox-tests` | @semio-tech/os-renderer-wgpu-runtime-mailbox-tests-rs | 0.031 (files) | 0.932 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪤️runtime-mailbox-tests/📦️packages/🦀️rust/Cargo.toml` |
| 43 | `semio-framework-3d-brep-offset` | semio-framework-3d-brep-offset | 0.818 (lines) | 0.931 (lines) | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/↔️offset/📦️packages/🦀️rust/Cargo.toml` |
| 44 | `semio-s-artifact-gis-gismap-demo-asset` | semio-s-artifact-gis-gismap-demo-asset | 0.930 (bytes) | 0.930 (bytes) | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/📦️packages/🦀️rust/Cargo.toml` |
| 45 | `semio-framework-os-renderer-wgpu-host-plumbing` | @semio-tech/os-renderer-wgpu-host-plumbing-rs | 0.512 (bytes) | 0.925 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔧️host-plumbing/📦️packages/🦀️rust/Cargo.toml` |
| 46 | `semio-s-artifact-fem-2d-editor-session` | semio-s-artifact-fem-2d-editor-session | 0.795 (lines) | 0.924 (lines) | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml` |
| 47 | `semio-s-artifact-stdio-dxf-diff` | semio-s-artifact-stdio-dxf-diff | 0.923 (lines) | 0.923 (lines) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 48 | `semio-s-artifact-layout-layout-editor` | semio-s-artifact-layout-layout-editor | 0.568 (lines) | 0.922 (bytes) | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 49 | `semio-s-artifact-draw-drawing-editor` | semio-s-artifact-draw-drawing-editor | 0.919 (bytes) | 0.922 (bytes) | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 50 | `semio-framework-dsl-record` | @semio-tech/framework-dsl-record-rs | 0.441 (bytes) | 0.920 (bytes) | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 51 | `semio-framework-plugin-reactor-jobs` | @semio-tech/plugin-reactor-jobs-rs | 0.408 (bytes) | 0.920 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/📦️packages/🦀️rust/Cargo.toml` |
| 52 | `semio-framework-os-renderer-wgpu-shell-tests-input-1` | @semio-tech/os-renderer-wgpu-shell-tests-input-1-rs | 0.031 (files) | 0.915 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🍎️shell-tests-input-1/📦️packages/🦀️rust/Cargo.toml` |
| 53 | `semio-s-artifact-norm-en1993-checks` | semio-s-artifact-norm-en1993-checks | 0.742 (lines) | 0.915 (lines) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust/Cargo.toml` |
| 54 | `semio-framework-pack-record-encode` | @semio-tech/pack-record-encode-rs | 0.714 (bytes) | 0.914 (bytes) | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/📦️packages/🦀️rust/Cargo.toml` |
| 55 | `semio-s-artifact-wfc-3d-wfc3d` | semio-s-artifact-wfc-3d-wfc3d | 0.696 (lines) | 0.913 (lines) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 56 | `semio-framework-os-infinite-world-model` | @semio-tech/os-infinite-world-model-rs | 0.826 (lines) | 0.913 (lines) | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/📦️packages/🦀️rust/Cargo.toml` |
| 57 | `semio-s-artifact-puzzle-2d-unit-tests` | semio-s-artifact-puzzle-2d-unit-tests | 0.031 (files) | 0.912 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 58 | `semio-framework-os-kernel-db-io-core` | @semio-tech/os-db-io-core-rs | 0.886 (lines) | 0.912 (lines) | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/📦️packages/🦀️rust/Cargo.toml` |
| 59 | `semio-s-artifact-energy-model-diff` | semio-s-artifact-energy-model-diff | 0.881 (bytes) | 0.910 (bytes) | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 60 | `semio-s-artifact-wfc-grid3d-editor` | semio-s-artifact-wfc-grid3d-editor | 0.748 (lines) | 0.910 (lines) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 61 | `semio-s-artifact-gis-gismap-tests` | semio-s-artifact-gis-gismap-tests | 0.500 (files) | 0.908 (lines) | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 62 | `semio-framework-pets-contract` | @semio-tech/pets-contract-rs | 0.687 (lines) | 0.908 (lines) | `🧰️framework/🛍️products/🐾️pets/🔨️modules/📜️contract/📦️packages/🦀️rust/Cargo.toml` |
| 63 | `semio-s-artifact-energy-model-editor` | semio-s-artifact-energy-model-editor | 0.405 (lines) | 0.908 (bytes) | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 64 | `semio-s-artifact-stdio-xml-snapshot` | semio-s-artifact-stdio-xml-snapshot | 0.897 (lines) | 0.907 (lines) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` |
| 65 | `semio-framework-plugin-tests-15` | @semio-tech/plugin-tests-15-rs | 0.031 (files) | 0.907 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🥥️tests-15/📦️packages/🦀️rust/Cargo.toml` |
| 66 | `semio-s-artifact-writer-writer-rename-writer` | semio-s-artifact-writer-writer-rename-writer | 0.594 (files) | 0.906 (files) | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-writer/📦️packages/🦀️rust/Cargo.toml` |
| 67 | `semio-s-artifact-wfc-2d-mutations` | semio-s-artifact-wfc-2d-mutations | 0.406 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds/📦️packages/🦀️rust/Cargo.toml` |
| 68 | `semio-s-artifact-forms-forms-tests-2` | semio-s-artifact-forms-forms-tests-2 | 0.594 (files) | 0.906 (files) | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🧪️tests/🔬️events/📦️packages/🦀️rust/Cargo.toml` |
| 69 | `semio-s-artifact-stdio-zip-schema` | semio-s-artifact-stdio-zip-schema | 0.781 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 70 | `semio-s-artifact-stdio-svg-mutations` | semio-s-artifact-stdio-svg-mutations | 0.812 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 71 | `semio-s-artifact-stdio-pdf-mut-family-9` | semio-s-artifact-stdio-pdf-mut-family-9 | 0.250 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 72 | `semio-s-artifact-stdio-gltf-mutations-mut-any-5` | semio-s-artifact-stdio-gltf-mutations-mut-any-5 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required/➕️add/🧪️tests/🔬️t041/📦️packages/🦀️rust/Cargo.toml` |
| 73 | `semio-s-artifact-note-note-diff` | semio-s-artifact-note-note-diff | 0.750 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 74 | `semio-s-artifact-note-note-delete-blocks` | semio-s-artifact-note-note-delete-blocks | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/📦️packages/🦀️rust/Cargo.toml` |
| 75 | `semio-framework-plugin-tests-02` | @semio-tech/plugin-tests-02-rs | 0.281 (files) | 0.906 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍐️tests-02/📦️packages/🦀️rust/Cargo.toml` |
| 76 | `semio-s-artifact-wfc-2d-pin-slot` | semio-s-artifact-wfc-2d-pin-slot | 0.656 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/📦️packages/🦀️rust/Cargo.toml` |
| 77 | `semio-s-artifact-wfc-grid2d-diff` | semio-s-artifact-wfc-grid2d-diff | 0.812 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 78 | `semio-s-artifact-wfc-3d-mutations` | semio-s-artifact-wfc-3d-mutations | 0.406 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 79 | `semio-s-artifact-shooting-shooting-tests` | semio-s-artifact-shooting-shooting-tests | 0.688 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 80 | `semio-s-artifact-stdio-svg-tests` | semio-s-artifact-stdio-svg-tests | 0.500 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 81 | `semio-s-artifact-stdio-pdf-mut-family-14` | semio-s-artifact-stdio-pdf-mut-family-14 | 0.153 (lines) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 82 | `semio-s-artifact-stdio-pdf-conformance-support` | semio-s-artifact-stdio-pdf-conformance-support | 0.610 (lines) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🏅️conformance-support/📦️packages/🦀️rust/Cargo.toml` |
| 83 | `semio-s-artifact-stdio-pdf-testkit` | semio-s-artifact-stdio-pdf-testkit | 0.625 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧪️testkit/📦️packages/🦀️rust/Cargo.toml` |
| 84 | `semio-s-artifact-note-note-mutations` | semio-s-artifact-note-note-mutations | 0.625 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 85 | `semio-s-artifact-puzzle-5d-mut-any-7` | semio-s-artifact-puzzle-5d-mut-any-7 | 0.125 (files) | 0.906 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds/📦️packages/🦀️rust/Cargo.toml` |
| 86 | `semio-framework-os-config` | @semio-tech/framework-os-config | 0.906 (files) | 0.906 (files) | `🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/Cargo.toml` |
| 87 | `semio-s-artifact-writer-writer-editor` | semio-s-artifact-writer-writer-editor | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/📦️packages/🦀️rust/Cargo.toml` |
| 88 | `semio-s-artifact-wfc-grid2d-create-tile` | semio-s-artifact-wfc-grid2d-create-tile | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-tile/📦️packages/🦀️rust/Cargo.toml` |
| 89 | `semio-s-artifact-forms-forms-forms` | semio-s-artifact-forms-forms-forms | 0.418 (lines) | 0.906 (files) | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 90 | `semio-s-artifact-trinity-rewriting-operations` | semio-s-artifact-trinity-rewriting-operations | 0.688 (files) | 0.906 (files) | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` |
| 91 | `semio-s-artifact-stdio-pdf-tests` | semio-s-artifact-stdio-pdf-tests | 0.719 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 92 | `semio-s-artifact-stdio-pdf-mut-family-13` | semio-s-artifact-stdio-pdf-mut-family-13 | 0.142 (lines) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 93 | `semio-s-artifact-stdio-pdf-inferences` | semio-s-artifact-stdio-pdf-inferences | 0.750 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` |
| 94 | `semio-s-artifact-stdio-pdf-mut-base-8` | semio-s-artifact-stdio-pdf-mut-base-8 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕️append-page-content/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 95 | `semio-s-artifact-stdio-xml-schema-2` | semio-s-artifact-stdio-xml-schema-2 | 0.719 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 96 | `semio-s-artifact-stdio-jpg-schema-2` | semio-s-artifact-stdio-jpg-schema-2 | 0.531 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 97 | `semio-s-artifact-stdio-gltf-mutations-mut-any-9` | semio-s-artifact-stdio-gltf-mutations-mut-any-9 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🚚️move/🧪️tests/⬇️demotes-the-first-f41a10/📦️packages/🦀️rust/Cargo.toml` |
| 98 | `semio-s-artifact-stdio-gltf-mutations-mut-any-7` | semio-s-artifact-stdio-gltf-mutations-mut-any-7 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph/🌱️create/🧪️tests/🔬️t065/📦️packages/🦀️rust/Cargo.toml` |
| 99 | `semio-s-artifact-stdio-semio-value-mutations` | semio-s-artifact-stdio-semio-value-mutations | 0.906 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 100 | `semio-s-artifact-stdio-semio-kit-mutations` | semio-s-artifact-stdio-semio-kit-mutations | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 101 | `semio-s-artifact-puzzle-5d-mut-any-5` | semio-s-artifact-puzzle-5d-mut-any-5 | 0.750 (files) | 0.906 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/🌱️appends-part-c/📦️packages/🦀️rust/Cargo.toml` |
| 102 | `semio-s-artifact-sourcing-curation-domain` | semio-s-artifact-sourcing-curation-domain | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️domain/📦️packages/🦀️rust/Cargo.toml` |
| 103 | `semio-framework-plugin-interpreter-runtime` | @semio-tech/framework-plugin-interpreter-runtime | 0.537 (bytes) | 0.906 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust/Cargo.toml` |
| 104 | `semio-s-artifact-writer-writer-operations` | semio-s-artifact-writer-writer-operations | 0.500 (files) | 0.906 (files) | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` |
| 105 | `semio-s-artifact-wfc-grid3d-mutations` | semio-s-artifact-wfc-grid3d-mutations | 0.500 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 106 | `semio-s-artifact-sequence-sequence-inferences` | semio-s-artifact-sequence-sequence-inferences | 0.781 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` |
| 107 | `semio-s-artifact-norm-en1996-editor` | semio-s-artifact-norm-en1996-editor | 0.562 (files) | 0.906 (files) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 108 | `semio-s-artifact-stdio-pdf-mut-family-8` | semio-s-artifact-stdio-pdf-mut-family-8 | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 109 | `semio-s-artifact-stdio-pdf-schema-4` | semio-s-artifact-stdio-pdf-schema-4 | 0.281 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 110 | `semio-s-artifact-stdio-pdf-mut-base-9` | semio-s-artifact-stdio-pdf-mut-base-9 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🎚️remove-ext-g-state/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 111 | `semio-s-artifact-stdio-gltf-inferences-clearance` | semio-s-artifact-stdio-gltf-inferences-clearance | 0.312 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↔️clearance/📦️packages/🦀️rust/Cargo.toml` |
| 112 | `semio-s-artifact-stdio-gltf-mutations-mut-any-6` | semio-s-artifact-stdio-gltf-mutations-mut-any-6 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🧪️tests/🔬️t053/📦️packages/🦀️rust/Cargo.toml` |
| 113 | `semio-s-artifact-note-note-tests-hides-the-grid` | semio-s-artifact-note-note-tests-hides-the-grid | 0.688 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/🙈️hides/📦️packages/🦀️rust/Cargo.toml` |
| 114 | `semio-s-artifact-mathematical-equation-mutations` | semio-s-artifact-mathematical-equation-mutations | 0.500 (files) | 0.906 (files) | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 115 | `semio-s-artifact-sequence-sequence-tests` | semio-s-artifact-sequence-sequence-tests | 0.656 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 116 | `semio-s-artifact-norm-din4108-io` | semio-s-artifact-norm-din4108-io | 0.781 (files) | 0.906 (files) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml` |
| 117 | `semio-s-artifact-stdio-html-mutations` | semio-s-artifact-stdio-html-mutations | 0.656 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 118 | `semio-s-artifact-stdio-pdf-mut-base-4` | semio-s-artifact-stdio-pdf-mut-base-4 | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/📦️packages/🦀️rust/Cargo.toml` |
| 119 | `semio-s-artifact-puzzle-2d-interaction` | semio-s-artifact-puzzle-2d-interaction | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` |
| 120 | `semio-s-artifact-writer-writer-inferences` | semio-s-artifact-writer-writer-inferences | 0.781 (files) | 0.906 (files) | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` |
| 121 | `semio-s-artifact-gis-gismap-mutations` | semio-s-artifact-gis-gismap-mutations | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 122 | `semio-s-artifact-shooting-shooting-mutations` | semio-s-artifact-shooting-shooting-mutations | 0.312 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-asset/🧪️tests/➕️appends/📦️packages/🦀️rust/Cargo.toml` |
| 123 | `semio-s-artifact-norm-din18599-editor` | semio-s-artifact-norm-din18599-editor | 0.562 (files) | 0.906 (files) | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` |
| 124 | `semio-s-artifact-stdio-pdf-mut-family-12` | semio-s-artifact-stdio-pdf-mut-family-12 | 0.289 (lines) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗂️set-catalog-entry/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 125 | `semio-s-artifact-stdio-md-diff` | semio-s-artifact-stdio-md-diff | 0.750 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 126 | `semio-framework-artifact-workflow-run` | @semio-tech/framework-workflow-run-rs | 0.906 (files) | 0.906 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust/Cargo.toml` |
| 127 | `semio-s-artifact-stdio-pdf-mut-x-2` | semio-s-artifact-stdio-pdf-mut-x-2 | 0.066 (bytes) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 128 | `semio-s-artifact-stdio-pdf-mut-family-15` | semio-s-artifact-stdio-pdf-mut-family-15 | 0.250 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 129 | `semio-s-artifact-stdio-pdf-mut-family-11` | semio-s-artifact-stdio-pdf-mut-family-11 | 0.312 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🪄️replace-page/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 130 | `semio-s-artifact-stdio-gltf-mutations-mut-any-8` | semio-s-artifact-stdio-gltf-mutations-mut-any-8 | 0.062 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🧪️tests/🚫️removes-the-8d7de9/📦️packages/🦀️rust/Cargo.toml` |
| 131 | `semio-s-artifact-puzzle-2d-mut-any-7` | semio-s-artifact-puzzle-2d-mut-any-7 | 0.156 (files) | 0.906 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️derives/📦️packages/🦀️rust/Cargo.toml` |
| 132 | `semio-s-artifact-sourcing-curation-operations` | semio-s-artifact-sourcing-curation-operations | 0.719 (files) | 0.906 (files) | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` |
| 133 | `semio-framework-plugin-tests-04-2` | @semio-tech/plugin-tests-04-2-rs | 0.031 (files) | 0.906 (files) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧶️tests-04-2/📦️packages/🦀️rust/Cargo.toml` |
| 134 | `semio-s-artifact-mathematical-equation-update-graph-algorithm` | semio-s-artifact-mathematical-equation-update-graph-algorithm | 0.781 (files) | 0.906 (files) | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph/📦️packages/🦀️rust/Cargo.toml` |
| 135 | `semio-s-artifact-wfc-2d-diff` | semio-s-artifact-wfc-2d-diff | 0.875 (files) | 0.906 (files) | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 136 | `semio-s-artifact-animate-presentation-mutations` | semio-s-artifact-animate-presentation-mutations | 0.438 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 137 | `semio-s-artifact-shooting-shooting-set-active-asset` | semio-s-artifact-shooting-shooting-set-active-asset | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/📦️packages/🦀️rust/Cargo.toml` |
| 138 | `semio-s-artifact-trinity-rewriting-mut-any` | semio-s-artifact-trinity-rewriting-mut-any | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` |
| 139 | `semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests` | semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/📦️packages/🦀️rust/Cargo.toml` |
| 140 | `semio-s-artifact-stdio-pdf-mut-family-10` | semio-s-artifact-stdio-pdf-mut-family-10 | 0.156 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 141 | `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests` | semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests | 0.332 (lines) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value/📦️packages/🦀️rust/Cargo.toml` |
| 142 | `semio-s-artifact-stdio-docx-base-model` | semio-s-artifact-stdio-docx-base-model | 0.906 (files) | 0.906 (files) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 143 | `semio-s-artifact-puzzle-5d-diff` | semio-s-artifact-puzzle-5d-diff | 0.719 (files) | 0.906 (files) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` |
| 144 | `semio-s-artifact-sourcing-curation-inferences` | semio-s-artifact-sourcing-curation-inferences | 0.844 (files) | 0.906 (files) | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` |
| 145 | `semio-framework-os-run` | semio-framework-os-run | 0.668 (bytes) | 0.904 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml` |
| 146 | `semio-s-artifact-bim-model-inferences` | semio-s-artifact-bim-model-inferences | 0.798 (bytes) | 0.903 (bytes) | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` |
| 147 | `semio-s-artifact-puzzle-3d-precompute-fill` | semio-s-artifact-puzzle-3d-precompute-fill | 0.879 (lines) | 0.901 (lines) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/📦️packages/🦀️rust/Cargo.toml` |
| 148 | `semio-s-artifact-stdio-pdf-tests-2` | semio-s-artifact-stdio-pdf-tests-2 | 0.877 (lines) | 0.901 (lines) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` |
| 149 | `semio-framework-os-kernel-dsl-derive-expand` | @semio-tech/dsl-derive-expand-rs | 0.468 (bytes) | 0.900 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧩️expand/📦️packages/🦀️rust/Cargo.toml` |
| 150 | `semio-s-artifact-stdio-pdf-schema-5` | semio-s-artifact-stdio-pdf-schema-5 | 0.865 (bytes) | 0.900 (bytes) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/📦️packages/🦀️rust/Cargo.toml` |
| 151 | `semio-framework-os-kernel-db-artifact` | @semio-tech/os-db-artifact-rs | 0.441 (lines) | 0.900 (bytes) | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📦️packages/🦀️rust/Cargo.toml` |

## 4. Crates refused by the reader (no ratio)

12 targets. All refusals are test cfg (`test` added to the host cfg); prod cfg has 0 refusals. Refused targets are not scored, so no 0.000 is recorded for them.

Reason classes: Missing source input (non-.rs fixture) (5); Missing or ambiguous Rust module (4); Unsupported Rust compile expression at line 108 (1); Unsupported Rust compile expression at line 122 (1); Unsupported Rust compile expression at line 94 (1).

| # | Crate | Owner | Reason (test cfg) | Manifest |
|---:|---|---|---|---|
| 1 | `semio-s-fem-app-surface` | semio-s-fem-app-surface | Missing Rust source input: 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️color-input/🔣️.json | `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/📦️packages/🦀️rust/Cargo.toml` |
| 2 | `semio-framework-plugin-decl-surface` | @semio-tech/plugin-decl-surface-rs | Missing or ambiguous Rust module: snapshot_builder_ownership_tests | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/📦️packages/🦀️rust/Cargo.toml` |
| 3 | `semio-framework-os-store-testing-fixtures` | @semio-tech/os-store-testing-fixtures-rs | Missing or ambiguous Rust module: tests | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/📦️packages/🦀️rust/Cargo.toml` |
| 4 | `semio-s-artifact-puzzle-2d-connect-handles-preparation` | semio-s-artifact-puzzle-2d-connect-handles-preparation | Unsupported Rust compile expression at line 108: include_str | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/📦️packages/🦀️rust/Cargo.toml` |
| 5 | `semio-framework-os-store-tests-05` | @semio-tech/os-store-tests-05-rs | Missing or ambiguous Rust module: shared_semantic_source_tests | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/📦️packages/🦀️rust/Cargo.toml` |
| 6 | `semio-s-artifact-puzzle-2d-tests` | semio-s-artifact-puzzle-2d-tests | Unsupported Rust compile expression at line 122: include_str | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🧪️tests/📦️packages/🦀️rust/Cargo.toml` |
| 7 | `semio-framework-os-store-durable-group-tests` | @semio-tech/os-store-durable-group-tests-rs | Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📦️prepared/🔗️binding/🧫️fixtures/🔣️.json | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🥧️companion/📦️packages/🦀️rust/Cargo.toml` |
| 8 | `semio-s-artifact-puzzle-2d-text-inverse` | semio-s-artifact-puzzle-2d-text-inverse | Unsupported Rust compile expression at line 94: include_str | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml` |
| 9 | `semio-framework-os-store-tests-04` | @semio-tech/os-store-tests-04-rs | Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🎮️one-item/🛤️lane/🔣️.json | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/📦️packages/🦀️rust/Cargo.toml` |
| 10 | `semio-framework-os-spr-laws-tests` | @semio-tech/os-spr-laws-tests-rs | Missing or ambiguous Rust module: tests | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/🥣️laws-companion/📦️packages/🦀️rust/Cargo.toml` |
| 11 | `semio-framework-plugin-tests-14-2` | @semio-tech/plugin-tests-14-2-rs | Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧵️tests-14-2/🧫️fixtures/📥️remote-prefix/🔣️.json | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧵️tests-14-2/📦️packages/🦀️rust/Cargo.toml` |
| 12 | `semio-framework-os-kernel-db-sync-laws` | @semio-tech/os-kernel-db-sync-laws-rs | Missing Rust source input: 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🧫️fixtures/📝️actor-retirement/🔣️.json | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️laws/📦️packages/🦀️rust/Cargo.toml` |

## 5. Hand-authored files over 1,000 lines (working tree)

- Hand-authored over 1,000: **0** (census 8 at HEAD: 17).
- Over 1,000 but generated (header match, excluded): **3**:
  - `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💠️values/🦀️.rs` (1096 lines)
  - `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/⚡️out-jspi-explicit/jcoprobe.js` (10981 lines)
  - `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.js` (10981 lines)

## 6. New crates today: workspace members and build-owners

92 `Cargo.toml` manifests are new against HEAD or committed today under `.🧬semio` excluded (49 already committed today, 43 uncommitted). Nearest-ancestor workspace status: {'not-listed': 17, 'member': 75}.

- **Not in the nearest workspace members (14 crates):** all under `🧰️framework/`. `🧰️framework/Cargo.toml` (the nested workspace) does not list them, but the root `Cargo.toml` `members` does (14 of 14 verified). Cargo resolves the nearest `[workspace]` first, so `cargo` may reject these as "believes it's in a workspace when it's not". **Not verified: no cargo run (builds forbidden).** Fix candidate: add the 14 paths to `🧰️framework/Cargo.toml` `members`, or confirm root-only membership is intended.
  - `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/Cargo.toml` owner ``
  - `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/Cargo.toml` owner ``
  - `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/Cargo.toml` owner ``
  - `🧰️framework/🛍️products/🐾️pets/🔨️modules/📝️draft/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/pets-staging-draft-rs`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧱️flow-host-model/📦️packages/🦀️rust/Cargo.toml` owner `semio-framework-os-flow-host-core`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧭️prelude/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/os-store-testing-prelude-rs`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧪️tests/📦️packages/🦀️rust/Cargo.toml` owner `semio-framework-plugin-store-stage-laws`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧯️retirement-drivers-and-turn-limits/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/plugin-reactor-turn-limits-rs`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/os-services-native-io-rs`
  - `🧰️framework/🛍️products/📚️course/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-rs`
  - `🧰️framework/🛍️products/📚️course/🔨️modules/👥️presence/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-presence-rs`
  - `🧰️framework/🛍️products/📚️course/🧪️tests/🍰️companion-primitives/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-primitives-tests-rs`
  - `🧰️framework/🛍️products/📚️course/🧪️tests/🎞️companion-runs/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-tests-runs-rs`
  - `🧰️framework/🛍️products/📚️course/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-tests-rs`
  - `🧰️framework/🛍️products/📚️course/🧪️tests/🧷️companion-support/📦️packages/🦀️rust/Cargo.toml` owner `semio-framework-course-tests-support`
  - `🧰️framework/🛍️products/📚️course/🧬️schema/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/course-schema-rs`
  - `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/👥️presence/📦️packages/🦀️rust/Cargo.toml` owner `@semio-tech/framework-server-presence-rs`
- **Workspace-only manifests, not crates (3):** `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/Cargo.toml`, `…/🧱️material/Cargo.toml`, `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/Cargo.toml`. Each is a `[workspace]` root with `members = ["📦️packages/🦀️rust"]` and no `[package]`; not in build-owners by root (no crate is lost; their `📦️packages/🦀️rust` crates are owned and members of their own workspace).
- build-owners: 89 of 92 manifests have an owner entry; the 3 without are the workspace-only stubs above. Every crate with a `[package]` has an owner.

## 7. Delta vs census 8 (`C/audit-monolith-census-8.md`, HEAD bd9e4e02bd0)

| Measure | Census 8 (HEAD, 10-19 14:27) | Now (working tree, 18:0x) | Delta |
|---|---:|---:|---:|
| Crate targets | 3413 | 3453 | +40 |
| Prod over 1.00 | 13 | 0 | -13 |
| Test over 1.00 | 30 | 8 | -22 |
| Prod refused | 6 | 0 | -6 |
| Test refused | 21 | 12 | -9 |
| Prod 0.90 to 1.00 | 25 | 13 | -12 |
| Test 0.90 to 1.00 | 159 | 151 | -8 |
| Hand-authored over 1,000 lines | 17 (HEAD) | 0 (working tree) | -17 |
| Generated over 1,000 lines | 3 | 3 | 0 |
| Long paths over 256 chars | 0 | 0 | 0 |

Census-8 over-1,000 list (17, HEAD): 15 now at or under 1,000 lines, 2 deleted in the working tree (`🧰️framework/🛍️products/📚️course/🧪️tests/🚶️learner-journey/🟦️.tsx`, `…/🧪️tests/📡️presence-client/🟦️.tsx`), 0 still over. Census-8 counts above are the prior shard files `aud-census8-tools/crates8.disk.*.json` and the census-8 document; the wt census list is `census8-wt.json`.

## 8. Open items

1. Fix or validate the 8 test-cfg over-gate targets (owners above). Prod cfg is clean.
2. 12 test-cfg reader refusals need reader support or fixture staging (`include_str!`, missing `.json` fixtures, ambiguous `mod tests`).
3. Validate the 14 framework crates' nested-workspace membership with `cargo metadata` (not run here; builds forbidden for this audit).
4. `census8.py` as committed measures HEAD only; use `census8-wt.py` (in generated) or make it read the working tree before the next census.
5. The 3 generated over-1,000 files are excluded by the header rule; jcoprobe bundles are 10,981 lines each.
