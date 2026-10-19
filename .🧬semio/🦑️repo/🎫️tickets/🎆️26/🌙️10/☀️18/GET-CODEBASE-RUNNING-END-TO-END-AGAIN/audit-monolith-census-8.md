## 1. Totals at HEAD (bd9e4e02bd0)

- Tracked blobs at HEAD: 183879. In-scope files (census-7 extensions and exclusions): 51899, 5346383 newline-counted lines.
- Excluded by reason: dts 97, ext-host-build 3, semio 16660, storybook 8, vendored-dir 76.
- Over 1,000 lines at HEAD: 20. Header `@generated`-type match (excluded as generated): 3. Hand-authored over 1,000: **17**, of which `validateAgainst` / `(value: unknown)` signature: 2.
- Census 7 over-1,000 list: 44 paths. Still over 1,000 at HEAD (hand-authored): 0. Still over 1,000 (generated): 3. Dropped from the list (now below 1,000 or gone): 41. New hand-authored over-1,000 since census 7: **17**.
- Hand-authored over-1,000 files touched by the bulk commits: 8f91299152c 5, bd9e4e02bd0 17.
- Census 7 measured the working tree at 10-18 18:50. This census measures HEAD. The `pre-bulk lines` column reads the same paths at 4dab415ed90 (10-18 12:04), the last commit before the first bulk commit, as a proxy for the census-7 state.

## 2. Hand-authored files over 1,000 lines at HEAD

| # | Lines | Lang | Owner / area | In census 7 list | Pre-bulk lines (4dab) | Bulk commit | Signature | Path |
| ---: | ---: | --- | --- | --- | ---: | --- | --- | --- |
| 1 | 4,446 | rs | (no owner) ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | no | 128 | bd9e4e0 | no | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` |
| 2 | 2,167 | tsx | (no owner) 🧰️framework/🛍️products/📚️course/🧪️tests | no | absent | bd9e4e0 | no | `🧰️framework/🛍️products/📚️course/🧪️tests/🚶️learner-journey/🟦️.tsx` |
| 3 | 2,116 | rs | (no owner) ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | no | 211 | bd9e4e0 | no | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` |
| 4 | 1,495 | rs | (no owner) 🧰️framework/🛍️products/🦑️repo/🔨️modules | no | 910 | 8f91299, bd9e4e0 | no | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs` |
| 5 | 1,446 | rs | (no owner) 🧰️framework/🛍️products/🐾️pets/🔨️modules | no | 332 | bd9e4e0 | no | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🚶️locomotion/🦀️.rs` |
| 6 | 1,413 | rs | (no owner) ✏️s/🔨️modules/🏗️fem/⚙️engine | no | 503 | bd9e4e0 | no | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🔬️unit/🦀️.rs` |
| 7 | 1,275 | ts | @semio-tech/fem-2d-rs | no | 4 | 8f91299, bd9e4e0 | no | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts` |
| 8 | 1,151 | ts | (no owner) 🎓️teaching/🏛️architecture/📚️course/🚀️deploy | no | absent | bd9e4e0 | no | `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts` |
| 9 | 1,077 | rs | (no owner) 🧰️framework/🛍️products/💻️os/🔨️modules | no | 988 | 8f91299, bd9e4e0 | no | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs` |
| 10 | 1,065 | rs | (no owner) ✏️s/🔌️plugins/🏗️fem/🗿️artifacts | no | 988 | 8f91299, bd9e4e0 | no | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🎞️visual-job/🪜️solve-step/🦀️.rs` |
| 11 | 1,051 | tsx | (no owner) 🧰️framework/🛍️products/📚️course/🧪️tests | no | absent | bd9e4e0 | no | `🧰️framework/🛍️products/📚️course/🧪️tests/📡️presence-client/🟦️.tsx` |
| 12 | 1,045 | rs | (no owner) ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | no | 991 | bd9e4e0 | no | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs` |
| 13 | 1,026 | rs | (no owner) ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | no | 945 | bd9e4e0 | no | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs` |
| 14 | 1,016 | ts | (no owner) 🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn | no | 994 | bd9e4e0 | no | `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` |
| 15 | 1,010 | ts | @semio-tech/framework-renderer-react | no | 993 | 8f91299, bd9e4e0 | yes | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` |
| 16 | 1,006 | rs | (no owner) 🧰️framework/🛍️products/💻️os/🔨️modules | no | 893 | bd9e4e0 | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🦀️.rs` |
| 17 | 1,002 | ts | @semio-tech/s-dev-composition | no | 966 | bd9e4e0 | yes | `✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🟦️.ts` |

## 3. Generated files over 1,000 lines (excluded)

| Lines | Path | Reason |
| ---: | --- | --- |
| 1,096 | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💠️values/🦀️.rs` | generated header (`@generated`/`DO NOT EDIT` in first 25 lines) |
| 10,981 | `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/⚡️out-jspi-explicit/jcoprobe.js` | generated header (`jco` transpile output; census 7 had it as unconfirmed) |
| 10,981 | `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.js` | generated header (`jco` transpile output; census 7 had it as unconfirmed) |

## 4. Census 7 over-1,000 list at HEAD

| Census 7 path | HEAD lines | Status at HEAD |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💠️values/🦀️.rs` | 1,096 | below 1,000 |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🩹️patches/📚️library/🦀️.rs` | 25 | below 1,000 |
| `🌎️hub/🧪️tests/📺️renderer/🧊️wgpu/🤝️collaboration/🟦️.ts` | 159 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` | 64 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/🦀️.rs` | 59 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` | 90 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/📤️staging/🦀️.rs` | 72 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs` | 80 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` | 60 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs` | 55 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` | 99 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs` | 27 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🦀️.rs` | 56 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs` | 46 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🎬️execution/🦀️.rs` | 46 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🦀️.rs` | 33 | below 1,000 |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🖼️raster/🦀️.rs` | 70 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` | 76 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` | 120 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs` | 50 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | 868 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` | 22 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` | 165 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs` | 19 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs` | 42 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` | 59 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs` | 57 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🕸️node-graph/🦀️.rs` | 73 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs` | 63 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs` | 63 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | 110 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs` | 63 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs` | 32 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs` | 42 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs` | 18 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs` | 11 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs` | 15 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs` | 18 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs` | 53 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs` | 112 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs` | 92 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` | 73 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/⚡️out-jspi-explicit/jcoprobe.js` | 10,981 | below 1,000 |
| `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-bundles/📞️out-callback/jcoprobe.js` | 10,981 | below 1,000 |

## 5. Rust crate source gate (canonical reader, working tree)

Gate from `storage.sourceBoundary` in `⚡️caching/🔣️policy.json` at HEAD: 32 files, 262144 bytes, 4000 active lines. Reader: `readRustSourceClosureV1` from `🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`, host cfg from `crateTreeHostCfg()`, same as the canonical `measureRustCrate`. Ratio = max over files, bytes, lines. Production = host cfg. Test = host cfg plus `test`. Primary run reads the working tree (what the gate sees). Targets: `[lib]` and `[[bin]]` roots of every `Cargo.toml` with `[package]` (not `.🧬semio`, not vendored, not generated dirs). Integration tests and benches are not measured.

- Crate targets measured: 3413. Refused by reader, production cfg: 6. Refused, test cfg: 21.
- At or over 0.90 of a dimension, production cfg: **38** (over 1.00: 13). Test cfg: 189 at or over 0.90, 30 over 1.00.
- Crates whose `.rs` changed in the two bulk commits: 449 manifests. Of these, at or over 0.90 (either cfg) or refused: 35.

| Crate | Manifest | Kind | Prod files / bytes / lines | Prod ratio (dim) | Test ratio (dim) | Touched by bulk | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `semio-framework-3d-mesh-document` | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/📄️document/📦️packages/🦀️rust/Cargo.toml` | lib | Missing or ambiguous Rust module: journal | REFUSED | REFUSED | no | refused |
| `semio-framework-plugin-decl-app` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml` | lib | Missing or ambiguous Rust module: original_command | REFUSED | REFUSED | no | refused |
| `semio-framework-repo-tickets` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/Cargo.toml` | lib | Unbalanced Rust source delimiter | REFUSED | REFUSED | yes | refused |
| `semio-framework-os-vcs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/📦️packages/🦀️rust/Cargo.toml` | lib | Missing or ambiguous Rust module: history_read | REFUSED | REFUSED | no | refused |
| `semio-framework-server` | `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml` | lib | Missing or ambiguous Rust module: tests | REFUSED | REFUSED | no | refused |
| `semio-framework-repo-mcp` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/Cargo.toml` | lib | Unbalanced Rust source delimiter | REFUSED | REFUSED | yes | refused |
| `semio-s-artifact-fem-3d-mesh` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 34 / 82092 / 1286 | 1.062 (files) | 3.531 (files) | yes | over gate |
| `semio-s-artifact-fem-2d-mesh` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 34 / 78343 / 1229 | 1.062 (files) | 3.531 (files) | yes | over gate |
| `semio-framework-value` | `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml` | lib | 55 / 470523 / 4993 | 1.795 (bytes) | 3.470 (bytes) | yes | over gate |
| `semio-framework-replication-mutation-messages` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 285463 / 2817 | 1.089 (bytes) | 3.399 (bytes) | yes | over gate |
| `teaching-proctor-base` | `🎓️teaching/🛂️proctor/🔨️modules/🧱️base/📦️packages/🦀️rust/Cargo.toml` | lib | 6 / 109815 / 1972 | 0.493 (lines) | 2.876 (bytes) | no | over gate |
| `semio-s-artifact-fem-2d-load` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 49355 / 786 | 0.781 (files) | 2.594 (files) | yes | over gate |
| `semio-s-artifact-fem-3d-load` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 47940 / 776 | 0.781 (files) | 2.594 (files) | yes | over gate |
| `semio-s-artifact-fem-2d-mutations` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 54567 / 722 | 0.250 (files) | 1.625 (files) | no | over gate |
| `semio-s-artifact-fem-3d-mutations` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 40597 / 534 | 0.250 (files) | 1.625 (files) | no | over gate |
| `semio-framework-value-native-decoding` | `🧰️framework/🔨️modules/🌱️value/🛬️decode/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 159510 / 1213 | 0.750 (files) | 1.594 (files) | no | over gate |
| `semio-framework-pets-staging` | `🧰️framework/🛍️products/🐾️pets/🔨️modules/🎬️staging/📦️packages/🦀️rust/Cargo.toml` | lib | 12 / 265991 / 4673 | 1.168 (lines) | 1.438 (lines) | no | over gate |
| `semio-s-artifact-puzzle-5d-puzzle5d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 290725 / 5094 | 1.274 (lines) | 1.393 (lines) | yes | over gate |
| `semio-s-plugin-energy-engine-precompute` | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 60311 / 1089 | 0.272 (lines) | 1.376 (bytes) | no | over gate |
| `semio-s-artifact-energy-model-mut-window-layer` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥂️window-layer/📦️packages/🦀️rust/Cargo.toml` | lib | 43 / 76460 / 1314 | 1.344 (files) | 1.344 (files) | no | over gate |
| `semio-s-plugin-energy-engine-sim-unit-tests` | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 57680 / 0 | 0.220 (bytes) | 1.311 (bytes) | no | over gate |
| `semio-framework-course-primitives-tests` | `🧰️framework/🛍️products/📚️course/🧪️tests/🍰️companion-primitives/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 359 / 0 | 0.031 (files) | 1.284 (bytes) | no | over gate |
| `semio-s-plugin-energy-engine-sim-restore` | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️restore/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 30980 / 595 | 0.149 (lines) | 1.283 (bytes) | no | over gate |
| `semio-framework-tool-run` | `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 306573 / 3726 | 1.169 (bytes) | 1.262 (bytes) | yes | over gate |
| `semio-framework-job` | `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml` | lib | 34 / 285149 / 2900 | 1.088 (bytes) | 1.250 (files) | no | over gate |
| `semio-s-artifact-fem-3d` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 275124 / 259 | 1.050 (bytes) | 1.186 (bytes) | no | over gate |
| `semio-s-plugin-energy-engine-site` | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 18088 / 349 | 0.087 (lines) | 1.177 (bytes) | no | over gate |
| `semio-s-artifact-energy-model-mut-pv-system` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☀️pv-system/📦️packages/🦀️rust/Cargo.toml` | lib | 37 / 65149 / 1326 | 1.156 (files) | 1.156 (files) | no | over gate |
| `semio-framework-repo-dashboard-terminal` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust/Cargo.toml` | lib | 6 / 150411 / 2382 | 0.596 (lines) | 1.149 (bytes) | no | over gate |
| `semio-framework-value-derive` | `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 289415 / 3956 | 1.104 (bytes) | 1.140 (bytes) | no | over gate |
| `semio-framework-replication-history` | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 175455 / 2103 | 0.669 (bytes) | 1.131 (bytes) | no | over gate |
| `semio-s-artifact-fem-2d` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 258509 / 261 | 0.986 (bytes) | 1.118 (bytes) | no | over gate |
| `semio-framework-pack-json` | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml` | lib | 19 / 252966 / 3659 | 0.965 (bytes) | 1.025 (bytes) | no | over gate |
| `semio-framework-plugin-window-config` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 230098 / 3120 | 0.878 (bytes) | 1.025 (bytes) | no | over gate |
| `semio-framework-value-retained-clone` | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 149826 / 2137 | 0.572 (bytes) | 1.010 (bytes) | no | over gate |
| `semio-framework-course` | `🧰️framework/🛍️products/📚️course/📦️packages/🦀️rust/Cargo.toml` | lib | 12 / 263090 / 2890 | 1.004 (bytes) | 1.004 (bytes) | yes | over gate |
| `semio-s-artifact-fem-3d-boundary` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 23588 / 418 | 0.406 (files) | 1.000 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-fem-3d-material` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 23283 / 436 | 0.406 (files) | 1.000 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-fem-2d-boundary` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🛡️boundary/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 23960 / 428 | 0.406 (files) | 1.000 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-fem-2d-material` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 23502 / 438 | 0.406 (files) | 1.000 (files) | yes | 0.90 to 1.00 |
| `semio-framework-ui-retained-tree-tree` | `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 171781 / 3065 | 0.766 (lines) | 0.993 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-mutations` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/📦️packages/🦀️rust/Cargo.toml` | lib | 26 / 198210 / 2571 | 0.812 (files) | 0.985 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-plugin-decl-views` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 225213 / 3592 | 0.898 (lines) | 0.976 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-interaction` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 210173 / 3590 | 0.897 (lines) | 0.969 (lines) | yes | 0.90 to 1.00 |
| `semio-s-artifact-writer-writer-tests-2` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 154050 / 2548 | 0.750 (files) | 0.969 (files) | yes | 0.90 to 1.00 |
| `semio-framework-plugin-testing-fixtures` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧰️fixtures/📦️packages/🦀️rust/Cargo.toml` | lib | 19 / 66098 / 1033 | 0.594 (files) | 0.969 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-fem-2d-model` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 160641 / 2459 | 0.656 (files) | 0.969 (files) | no | 0.90 to 1.00 |
| `semio-framework-os-store-replay` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 204988 / 2573 | 0.782 (bytes) | 0.969 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-ui-tui-core` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧱️cell-model/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 138061 / 2678 | 0.669 (lines) | 0.959 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-energy-model-snapshot` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 220293 / 2250 | 0.840 (bytes) | 0.958 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-reasoning-wires-wires` | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 111149 / 1869 | 0.467 (lines) | 0.956 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-actor` | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` | lib | 12 / 137204 / 2078 | 0.523 (bytes) | 0.954 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-plugin-instance-app` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 248846 / 3155 | 0.949 (bytes) | 0.949 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-os-kernel-db-io-driver` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/📦️packages/🦀️rust/Cargo.toml` | lib | 7 / 96682 / 1847 | 0.462 (lines) | 0.949 (lines) | no | 0.90 to 1.00 |
| `semio-framework-os-services` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 218973 / 3794 | 0.949 (lines) | 0.949 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-mut-any-10` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️selection/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 118507 / 2149 | 0.537 (lines) | 0.944 (lines) | no | 0.90 to 1.00 |
| `semio-framework-value-intrinsic` | `🧰️framework/🔨️modules/🌱️value/🌳️intrinsic/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 160785 / 2665 | 0.666 (lines) | 0.943 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-bitmap-bitmap` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 205158 / 3550 | 0.887 (lines) | 0.942 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-dispatch` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎬️dispatch/📦️packages/🦀️rust/Cargo.toml` | lib | 26 / 235836 / 3764 | 0.941 (lines) | 0.941 (lines) | yes | 0.90 to 1.00 |
| `semio-framework-os-mcp-hub-remote` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌍️hub-remote/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 135734 / 2324 | 0.581 (lines) | 0.939 (lines) | no | 0.90 to 1.00 |
| `semio-framework-ui-contract` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 137132 / 2975 | 0.744 (lines) | 0.939 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 175819 / 3130 | 0.782 (lines) | 0.938 (lines) | yes | 0.90 to 1.00 |
| `semio-s-artifact-mathematical-equation-snapshot` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 179996 / 3307 | 0.827 (lines) | 0.938 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-docx-base-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 30 / 224915 / 3364 | 0.938 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-mathematical-equation-editor` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 150658 / 2469 | 0.844 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gif-editor` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 84471 / 1351 | 0.562 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-mut-any` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 30 / 59788 / 1253 | 0.938 (files) | 0.938 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-fem-3d-model` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 180615 / 2555 | 0.689 (bytes) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-semio-text-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 55101 / 998 | 0.844 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-xml-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 23 / 62470 / 1184 | 0.719 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-xlsx-transitional` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 84335 / 1487 | 0.688 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-retained` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 193078 / 3296 | 0.906 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-binary-snapshot` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 71990 / 904 | 0.844 (files) | 0.938 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-dwg-sqlite-objects` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/📦️objects/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 244909 / 926 | 0.934 (bytes) | 0.934 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-3d-brep-queries` | `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 145245 / 2984 | 0.746 (lines) | 0.934 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-process-process3d-model` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 235252 / 3337 | 0.897 (bytes) | 0.934 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-value-tests` | `🧰️framework/🔨️modules/🌱️value/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 2072 / 0 | 0.031 (files) | 0.932 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-os-renderer-wgpu-runtime-mailbox-tests` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪤️runtime-mailbox-tests/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 4434 / 0 | 0.031 (files) | 0.931 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-3d-brep-offset` | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/↔️offset/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 183866 / 3272 | 0.818 (lines) | 0.931 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-gis-gismap-demo-asset` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 243819 / 5 | 0.930 (bytes) | 0.930 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-cad-cad-play` | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 243202 / 191 | 0.928 (bytes) | 0.928 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-dwg-read-objects` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/📦️objects/📦️packages/🦀️rust/Cargo.toml` | lib | 6 / 242787 / 3187 | 0.926 (bytes) | 0.926 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-os-renderer-wgpu-host-plumbing` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔧️host-plumbing/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 134185 / 1717 | 0.512 (bytes) | 0.925 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-fem-2d-editor-session` | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 184138 / 3179 | 0.795 (lines) | 0.924 (lines) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-dxf-diff` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 208121 / 3693 | 0.923 (lines) | 0.923 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-layout-layout-editor` | `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 147033 / 2271 | 0.568 (lines) | 0.922 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-draw-drawing-editor` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 241017 / 2687 | 0.919 (bytes) | 0.922 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-dsl-record` | `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 115631 / 1496 | 0.441 (bytes) | 0.920 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-plugin-reactor-jobs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 106837 / 1346 | 0.408 (bytes) | 0.919 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-plugin-tests-01` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍎️tests-01/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 240545 / 2041 | 0.918 (bytes) | 0.918 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-os-renderer-wgpu-shell-tests-input-1` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🍎️shell-tests-input-1/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 4783 / 0 | 0.031 (files) | 0.915 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-norm-en1993-checks` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 147116 / 2969 | 0.742 (lines) | 0.915 (lines) | no | 0.90 to 1.00 |
| `semio-framework-pack-record-encode` | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 187207 / 1430 | 0.714 (bytes) | 0.914 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-3d-wfc3d` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 153219 / 2785 | 0.696 (lines) | 0.913 (lines) | no | 0.90 to 1.00 |
| `semio-framework-os-infinite-world-model` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/📦️packages/🦀️rust/Cargo.toml` | lib | 7 / 151314 / 3305 | 0.826 (lines) | 0.913 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-unit-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 6770 / 123 | 0.031 (files) | 0.912 (lines) | no | 0.90 to 1.00 |
| `semio-framework-os-kernel-db-io-core` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/📦️packages/🦀️rust/Cargo.toml` | lib | 12 / 159016 / 3546 | 0.886 (lines) | 0.912 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-energy-model-diff` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 231057 / 863 | 0.881 (bytes) | 0.910 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-grid3d-editor` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 167836 / 2993 | 0.748 (lines) | 0.910 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-gis-gismap-tests` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 108948 / 1899 | 0.500 (files) | 0.908 (lines) | no | 0.90 to 1.00 |
| `semio-framework-plugin-reactor-turn` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 238145 / 3016 | 0.908 (bytes) | 0.908 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-pets-contract` | `🧰️framework/🛍️products/🐾️pets/🔨️modules/📜️contract/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 114717 / 2747 | 0.687 (lines) | 0.908 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-energy-model-editor` | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 103974 / 1617 | 0.404 (lines) | 0.908 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-xml-snapshot` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 215096 / 3587 | 0.897 (lines) | 0.907 (lines) | no | 0.90 to 1.00 |
| `semio-framework-plugin-tests-15` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🥥️tests-15/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 810 / 11 | 0.031 (files) | 0.907 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-mathematical-equation-mutations` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 85231 / 1411 | 0.500 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-2d-mutations` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 54888 / 933 | 0.406 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-grid2d-create-tile` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-tile/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 43352 / 895 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-15` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 46388 / 802 | 0.250 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-inferences` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 152574 / 2958 | 0.750 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-xml-schema-2` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 23 / 90223 / 1583 | 0.719 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-mutations-mut-any-8` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🧪️tests/🚫️removes-the-8d7de9/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 13183 / 149 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-semio-value-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 65041 / 1085 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-framework-plugin-tests-02` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍐️tests-02/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 37843 / 321 | 0.281 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-2d-pin-slot` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 49193 / 1029 | 0.656 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-gis-gismap-mutations` | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 70676 / 1093 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-shooting-shooting-mutations` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-asset/🧪️tests/➕️appends/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 53288 / 730 | 0.312 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-schema-4` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 51865 / 827 | 0.281 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-12` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗂️set-catalog-entry/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 73233 / 1155 | 0.289 (lines) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-writer-writer-editor` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 64622 / 1223 | 0.875 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-forms-forms-tests-2` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📨️response/🧪️tests/🔬️events/📦️packages/🦀️rust/Cargo.toml` | lib | 19 / 88421 / 1575 | 0.594 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-norm-din4108-io` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 83490 / 961 | 0.781 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-trinity-rewriting-operations` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 78976 / 1315 | 0.688 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-html-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 91888 / 1564 | 0.656 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-tests` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 23 / 92515 / 1262 | 0.719 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-x-2` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 17382 / 201 | 0.066 (bytes) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-11` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🪄️replace-page/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 38154 / 605 | 0.312 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-note-note-delete-blocks` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧹️delete-blocks/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 62200 / 1334 | 0.875 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-mut-any-7` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟢️node/⚓change-node-anchor/🧪️tests/⚓️derives/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 14101 / 169 | 0.156 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-mut-any-5` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/🌱️appends-part-c/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 130225 / 1961 | 0.750 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-framework-plugin-interpreter-runtime` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 140720 / 1560 | 0.537 (bytes) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-writer-writer-operations` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 40970 / 840 | 0.500 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-mathematical-equation-update-graph-algorithm` | `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🧬️schema/🧬️mutations/🧮️update-graph/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 59951 / 1323 | 0.781 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-3d-mutations` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 13 / 52029 / 854 | 0.406 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-grid3d-mutations` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 49898 / 858 | 0.500 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-shooting-shooting-set-active-asset` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️set-active-asset/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 46031 / 909 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-norm-din18599-editor` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 61639 / 1083 | 0.562 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-svg-handcrafted-diff-codec-tests` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 29708 / 643 | 0.844 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-10` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 36238 / 559 | 0.156 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-8` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 105620 / 1787 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-framework-artifact-workflow-run` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 100042 / 1509 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-forms-forms-diff` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 80237 / 1491 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-zip-schema` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 94623 / 1988 | 0.781 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-svg-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 26 / 61998 / 1255 | 0.812 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-snapshot` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 35850 / 666 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-9` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 47889 / 791 | 0.250 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-jpg-schema-2` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 66229 / 1292 | 0.531 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-mutations-mut-any-5` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/✅️required/➕️add/🧪️tests/🔬️t041/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 14180 / 157 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-mutations-mut-any-7` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧬️morph/🌱️create/🧪️tests/🔬️t065/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 11830 / 129 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-semio-kit-mutations` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/💾️binary/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 61871 / 1102 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-note-note-diff` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 82966 / 1634 | 0.750 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-interaction` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🕹️interaction/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 69552 / 1228 | 0.875 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-sourcing-curation-operations` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/📦️packages/🦀️rust/Cargo.toml` | lib | 23 / 59293 / 1115 | 0.719 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-framework-plugin-tests-04-2` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧶️tests-04-2/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 770 / 11 | 0.031 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-writer-writer-inferences` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 61089 / 1021 | 0.781 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-sequence-sequence-inferences` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | lib | 25 / 48321 / 778 | 0.781 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-svg-tests` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 16 / 33767 / 647 | 0.500 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-diff` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 75958 / 1455 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 76292 / 1329 | 0.332 (lines) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-md-diff` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 120084 / 2159 | 0.750 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-tiff-snapshot` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 83463 / 734 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-inferences-clearance` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/↔️clearance/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 24920 / 362 | 0.312 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-note-note-mutations` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 144571 / 1716 | 0.625 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-store-preparation` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧰️kit/💾️store-preparation/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 59636 / 1064 | 0.906 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-sourcing-curation-inferences` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 87609 / 1498 | 0.844 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-sequence-sequence-tests` | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 21 / 105644 / 1959 | 0.656 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-forms-forms-forms` | `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 107370 / 1673 | 0.418 (lines) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-13` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 3 / 34920 / 569 | 0.142 (lines) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-base-4` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 50042 / 775 | 0.875 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-mutations-mut-any-9` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🚚️move/🧪️tests/⬇️demotes-the-first-f41a10/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 14360 / 161 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-animate-presentation-mutations` | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 80686 / 1376 | 0.438 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-shooting-shooting-tests` | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 58170 / 1174 | 0.688 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-norm-en1996-editor` | `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml` | lib | 18 / 56742 / 1029 | 0.562 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-trinity-rewriting-mut-any` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | lib | 27 / 44151 / 887 | 0.844 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-family-14` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 3 / 39100 / 612 | 0.153 (lines) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-testkit` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧪️testkit/📦️packages/🦀️rust/Cargo.toml` | lib | 20 / 105229 / 1929 | 0.625 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-gltf-mutations-mut-any-6` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🧪️tests/🔬️t053/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 15327 / 174 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-note-note-tests-hides-the-grid` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/🙈️hides/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 63771 / 1140 | 0.688 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-framework-os-config` | `🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 106757 / 2325 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-writer-writer-rename-writer` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-writer/📦️packages/🦀️rust/Cargo.toml` | lib | 19 / 41068 / 780 | 0.594 (files) | 0.906 (files) | yes | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-base-8` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕️append-page-content/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 14058 / 179 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-sourcing-curation-domain` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️domain/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 144748 / 2293 | 0.875 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-2d-diff` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 28 / 48659 / 1065 | 0.875 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-wfc-grid2d-diff` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 26 / 58389 / 1260 | 0.812 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-conformance-support` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🏅️conformance-support/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 142566 / 2438 | 0.610 (lines) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-mut-base-9` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🎚️remove-ext-g-state/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 2 / 14076 / 176 | 0.062 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-docx-base-model` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 29 / 166402 / 2716 | 0.906 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-diff` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml` | lib | 23 / 138120 / 2559 | 0.719 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-5d-mut-any-7` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/🤝️adds/📦️packages/🦀️rust/Cargo.toml` | lib | 4 / 10394 / 138 | 0.125 (files) | 0.906 (files) | no | 0.90 to 1.00 |
| `semio-framework-os-run` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 175115 / 2551 | 0.668 (bytes) | 0.904 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-bim-model-inferences` | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` | lib | 10 / 209134 / 3062 | 0.798 (bytes) | 0.903 (bytes) | yes | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-3d-precompute-fill` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/📦️packages/🦀️rust/Cargo.toml` | lib | 9 / 170346 / 3518 | 0.879 (lines) | 0.901 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-tests-2` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml` | lib | 22 / 205068 / 3510 | 0.877 (lines) | 0.901 (lines) | no | 0.90 to 1.00 |
| `semio-s-artifact-stdio-pdf-schema-5` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 226665 / 3367 | 0.865 (bytes) | 0.900 (bytes) | no | 0.90 to 1.00 |
| `semio-framework-os-kernel-db-artifact` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📦️packages/🦀️rust/Cargo.toml` | lib | 6 / 96900 / 1762 | 0.441 (lines) | 0.900 (bytes) | no | 0.90 to 1.00 |
| `semio-s-artifact-puzzle-2d-tests` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/🧪️tests/📦️packages/🦀️rust/Cargo.toml` | lib | 17 / 169609 / 3205 | 0.801 (lines) | REFUSED | no | refused |
| `semio-framework-os-store-testing-fixtures` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/📦️packages/🦀️rust/Cargo.toml` | lib | 24 / 96026 / 1730 | 0.750 (files) | REFUSED | no | refused |
| `semio-framework-plugin-decl-surface` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/📦️packages/🦀️rust/Cargo.toml` | lib | 8 / 157775 / 2466 | 0.617 (lines) | REFUSED | no | refused |
| `semio-framework-replication-mutation` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️packages/🦀️rust/Cargo.toml` | lib | 12 / 139331 / 2195 | 0.549 (lines) | REFUSED | no | refused |
| `semio-framework-job-worker` | `🧰️framework/🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml` | lib | 14 / 128306 / 1691 | 0.489 (bytes) | REFUSED | no | refused |
| `semio-s-artifact-puzzle-2d-text-inverse` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/↩️inverse/📦️packages/🦀️rust/Cargo.toml` | lib | 11 / 118899 / 1752 | 0.454 (bytes) | REFUSED | no | refused |
| `semio-s-artifact-puzzle-2d-connect-handles-preparation` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️handle/🪢️connect-handles/🎮️prepare/📦️packages/🦀️rust/Cargo.toml` | lib | 5 / 41239 / 509 | 0.157 (bytes) | REFUSED | no | refused |
| `semio-s-fem-app-surface` | `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/📦️packages/🦀️rust/Cargo.toml` | lib | 3 / 13172 / 306 | 0.094 (files) | REFUSED | no | refused |
| `semio-framework-value-shared-issuance` | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 15960 / 137 | 0.061 (bytes) | REFUSED | no | refused |
| `semio-framework-os-store-durable-group-tests` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🥧️companion/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 2424 / 0 | 0.031 (files) | REFUSED | no | refused |
| `semio-framework-os-kernel-db-sync-laws` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️laws/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 552 / 0 | 0.031 (files) | REFUSED | no | refused |
| `semio-framework-plugin-tests-14-2` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🧵️tests-14-2/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 692 / 10 | 0.031 (files) | REFUSED | no | refused |
| `semio-framework-os-store-tests-05` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 2181 / 0 | 0.031 (files) | REFUSED | no | refused |
| `semio-framework-os-store-tests-04` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌌️bin-04/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 2299 / 0 | 0.031 (files) | REFUSED | no | refused |
| `semio-framework-os-spr-laws-tests` | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/🥣️laws-companion/📦️packages/🦀️rust/Cargo.toml` | lib | 1 / 446 / 0 | 0.031 (files) | REFUSED | no | refused |

HEAD-tree comparison (`git` blobs at bd9e4e02bd0, reader unchanged): 3409 targets, refused production 27, at or over 0.90 production 37. Refusals at HEAD are generated sources that exist only in the working tree (`🤖️generated` or ignored module files), so HEAD is not a valid gate state for them.

## 6. Paths over 256 characters among tracked files at HEAD

- Tracked paths at HEAD: 183882 (including `.🧬semio` tickets). Over 256 code points: 0. Over 256 UTF-16 units (Windows MAX_PATH unit): 0. Over 256 UTF-8 bytes: 0 (macOS and Linux count bytes, all under PATH_MAX).
- Longest repo-relative UTF-16 length: 226. Paths above 216 UTF-16 units (i.e. over 256 once a 40-unit checkout prefix such as `C:\Users\<name>\src\semio\` is added): 21. Paths above 200 UTF-16 units: 684.

| UTF-16 units | Path |
| ---: | --- |
| 226 | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json` |
| 219 | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🧫️fixtures/👁️set-brush-preview/✅️set-brush/📸️snapshot/⬅️before/🔣️.json` |
| 219 | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️set-camera/📸️snapshot/⬅️before/🔣️.json` |
| 219 | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🧫️fixtures/👁️set-layer-visibility/✅️set-layer/📸️snapshot/⬅️before/🔣️.json` |
| 219 | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🧫️fixtures/📷️set-camera/✅️set-camera-applied/📸️snapshot/⬅️before/🔣️.json` |
| 219 | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🧫️fixtures/⚙️set-editor-settings/✅️set-editor/📸️snapshot/⬅️before/🔣️.json` |
| 218 | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🧫️fixtures/👁️set-brush-preview/✅️set-brush/📸️snapshot/➡️after/🔣️.json` |
| 218 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🧫️fixtures/📊️replace-query/✅️replac/📸️snapshot/⬅️before/🔣️.json` |
| 218 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🧫️fixtures/🔍️set-lod-mode/✅️set-lod-mode/📸️snapshot/⬅️before/🔣️.json` |
| 218 | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🎚️config/🧪️tests/🧫️fixtures/🎥️set-camera/🟰️set-camera/📸️snapshot/➡️after/🔣️.json` |

## 7. Packages by owner (over-1,000 hand-authored files, disjoint)

| Owner / area | Files | Lines | New since census 7 |
| --- | ---: | ---: | ---: |
| (no owner) ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts | 4 | 8,633 | 4 |
| (no owner) 🧰️framework/🛍️products/📚️course/🧪️tests | 2 | 3,218 | 2 |
| (no owner) 🧰️framework/🛍️products/💻️os/🔨️modules | 2 | 2,083 | 2 |
| (no owner) 🧰️framework/🛍️products/🦑️repo/🔨️modules | 1 | 1,495 | 1 |
| (no owner) 🧰️framework/🛍️products/🐾️pets/🔨️modules | 1 | 1,446 | 1 |
| (no owner) ✏️s/🔨️modules/🏗️fem/⚙️engine | 1 | 1,413 | 1 |
| @semio-tech/fem-2d-rs | 1 | 1,275 | 1 |
| (no owner) 🎓️teaching/🏛️architecture/📚️course/🚀️deploy | 1 | 1,151 | 1 |
| (no owner) ✏️s/🔌️plugins/🏗️fem/🗿️artifacts | 1 | 1,065 | 1 |
| (no owner) 🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn | 1 | 1,016 | 1 |
| @semio-tech/framework-renderer-react | 1 | 1,010 | 1 |
| @semio-tech/s-dev-composition | 1 | 1,002 | 1 |

## 8. Crate split packages (production cfg at or over 0.90, or refused; disjoint by crate)

| Area (first 3 segments of crate dir) | Crates | Max prod ratio | Refused | Crate names |
| --- | ---: | ---: | ---: | --- |
| `✏️s/🔌️plugins/🗄️stdio` | 9 | 0.938 | 0 | `semio-s-artifact-stdio-docx-base-mutations`, `semio-s-artifact-stdio-semio-value-mutations`, `semio-s-artifact-stdio-dwg-sqlite-objects`, `semio-s-artifact-stdio-pdf-snapshot`, `semio-s-artifact-stdio-dxf-diff`, `semio-s-artifact-stdio-pdf-diff`, `semio-s-artifact-stdio-tiff-snapshot`, `semio-s-artifact-stdio-dwg-read-objects`, `semio-s-artifact-stdio-docx-base-model` |
| `🧰️framework/🛍️products/💻️os` | 8 | 0.949 | 2 | `semio-framework-plugin-instance-app`, `semio-framework-artifact-workflow-run`, `semio-framework-plugin-decl-app`, `semio-framework-plugin-reactor-turn`, `semio-framework-os-config`, `semio-framework-os-vcs`, `semio-framework-plugin-tests-01`, `semio-framework-os-services` |
| `✏️s/🔌️plugins/🧩️puzzle` | 5 | 1.274 | 0 | `semio-s-artifact-puzzle-5d-puzzle5d`, `semio-s-artifact-puzzle-5d-mut-any`, `semio-s-artifact-puzzle-5d-store-preparation`, `semio-s-artifact-puzzle-2d-retained`, `semio-s-artifact-puzzle-5d-dispatch` |
| `✏️s/🔌️plugins/🏗️fem` | 4 | 1.062 | 0 | `semio-s-artifact-fem-3d-mesh`, `semio-s-artifact-fem-3d`, `semio-s-artifact-fem-2d-mesh`, `semio-s-artifact-fem-2d` |
| `🧰️framework/🔨️modules/🌱️value` | 2 | 1.795 | 0 | `semio-framework-value`, `semio-framework-value-derive` |
| `🧰️framework/🛍️products/🦑️repo` | 2 | 0.000 | 2 | `semio-framework-repo-tickets`, `semio-framework-repo-mcp` |
| `✏️s/🔌️plugins/🔋️energy` | 2 | 1.344 | 0 | `semio-s-artifact-energy-model-mut-pv-system`, `semio-s-artifact-energy-model-mut-window-layer` |
| `🧰️framework/🔨️modules/📡️replication` | 1 | 1.089 | 0 | `semio-framework-replication-mutation-messages` |
| `🧰️framework/🛍️products/🐾️pets` | 1 | 1.168 | 0 | `semio-framework-pets-staging` |
| `🧰️framework/🔨️modules/🧊️3d` | 1 | 0.000 | 1 | `semio-framework-3d-mesh-document` |
| `🧰️framework/🔨️modules/🧵️job` | 1 | 1.088 | 0 | `semio-framework-job` |
| `🧰️framework/🛍️products/📚️course` | 1 | 1.004 | 0 | `semio-framework-course` |
| `✏️s/🔌️plugins/📐️cad` | 1 | 0.928 | 0 | `semio-s-artifact-cad-cad-play` |
| `🧰️framework/🔨️modules/🎒️pack` | 1 | 0.965 | 0 | `semio-framework-pack-json` |
| `✏️s/🔌️plugins/📋️forms` | 1 | 0.906 | 0 | `semio-s-artifact-forms-forms-diff` |
| `✏️s/🔌️plugins/🌍️gis` | 1 | 0.930 | 0 | `semio-s-artifact-gis-gismap-demo-asset` |
| `🧰️framework/🛍️products/🖥️server` | 1 | 0.000 | 1 | `semio-framework-server` |
| `✏️s/🔌️plugins/🖍️draw` | 1 | 0.919 | 0 | `semio-s-artifact-draw-drawing-editor` |
| `🧰️framework/🔨️modules/⏯️tool-run` | 1 | 1.169 | 0 | `semio-framework-tool-run` |

