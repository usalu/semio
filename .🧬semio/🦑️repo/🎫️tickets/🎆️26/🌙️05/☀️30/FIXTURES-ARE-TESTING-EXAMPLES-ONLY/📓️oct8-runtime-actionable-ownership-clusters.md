# Runtime Actionable Ownership Clusters

The actual 2355 total is repeated per selected host/target unit, not 2355 filesystem reads. It partitions as follows:

- computed-resource: 4 repeated findings, 4 unique path/detail records, 4 physical paths.
- other: 2 repeated findings, 2 unique path/detail records, 2 physical paths.
- compiler-witness: 217 repeated findings, 119 unique path/detail records, 71 physical paths.
- macro-expansion: 2099 repeated findings, 746 unique path/detail records, 505 physical paths.
- filesystem: 33 repeated findings, 12 unique path/detail records, 7 physical paths.

Minimal concrete filesystem cluster: 12 unique read/alias edges in 7 files:

- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs: Filesystem read_dir requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs: Filesystem read_to_string requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs: Filesystem read requires actual read input ownership
- 🧰️framework/🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/🦀️.rs: Filesystem read_to_string requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs: Filesystem read requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs: Filesystem read_dir requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs: Filesystem read_to_string requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️builder/🦀️.rs: Filesystem read_dir requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs: Filesystem read_dir requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs: Filesystem read_to_string requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs: Filesystem read_dir requires actual read input ownership
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs: Filesystem read_to_string requires actual read input ownership

DSL production schema indexing is parent-owned, with actual native RED/production collection exclusion/GREEN pending. Actual artifact-app-testing features must first exclude plugin history acceptance test-only readers. The remaining owners are native SPR command user-source intake, Semio schema source consumption, Value text-snapshot proc-macro source intake, and Infinite/WGPU original asset generation. Actual build-script-executed cfg/env/out_dir retention is plugin-owned, then generated copies will bind original asset owner to OUT_DIR. Macro expansion dominates the count and needs the actual selected production compiler artifact/.d roster, not blanket source removal. Different feature unit configurations remain refused rather than unioned.

## Current 818-Finding Source-Only Receipt Qualification

The explicit empty compiler observation route, terminal42740, produced 223graphs/1584 identities/818 unresolved findings. It is a bounded source frontier before current compiler/mount evidence, not a runtime pass. The previous 2355 count is historical. Exact current partition: 217 missing compiler witnesses, 562 unknown macro expansions,29 filesystem readers,5 computed JS imports,2 computed generated Rust module resources, and3 other native mount/broad-feature candidates. No fixture-edge record was promoted to an actual shipped violation.

| Filesystem Owner | Repeated Findings | Concrete Closure |
|---|---:|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` | 8 | Actual production feature disables test-only helper; preserve user input/test laws |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot/🦀️.rs` | 4 | True proc-macro read observations including unused schema scans; .d alone insufficient |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` | 12 | True proc-macro read observations including unused schema scans; .d alone insufficient |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️builder/🦀️.rs` | 1 | Actual build-script directory/read/copy observations, captured current source and output hashes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/build.rs` | 2 | Actual build-script directory/read/copy observations, captured current source and output hashes |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` | 2 | Actual production feature disables test-only helper; preserve user input/test laws |

Computed roles:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`: Computed import has no declared resolved owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts`: Computed import has no declared resolved owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`: Computed import has no declared resolved owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`: Computed import has no declared resolved owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs`: Computed module has no declared resolved owner
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs`: Computed module has no declared resolved owner

The Store never-published package probe is now retired with actual149 retained behavior laws passed. Plugin child Blob modules remain per-value verified runtime input ownership; WGPU frame-worker bindings require actual boot/server-native mount bytes. The two Rust module findings are actual OUT_DIR generated IconRenderHost/Infinite canvas includes, bound by the matching build-script environment plus compiler/generated-resource observations. No generic computed reader waiver is allowed.
