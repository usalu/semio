# OS Staging Root Source Selection Repair

The focused staging/freshness integration is repaired. Its portable fixture now reads 63 current source files: six retained boundaries, all 49 OS development implementation owners extracted from the former command body, and eight current WGPU compiler/native/browser authorities. The three absent WGPU package-local command, HTML and Trunk coordinates are removed from current source selection. Trunk's temporary configuration is represented by the actual WASM compiler source; browser HTML/configuration/server and native runner/module publication use their present owners.

Fixture consumer labels now distinguish the WGPU Vite mount and native descriptor staging input. They do not imply that a native published runtime uses the browser's staging directory. The existing portable path/freshness vectors and independent whatwg-url/picomatch oracles remain in use.

The test now decodes file URLs through fileURLToPath for platform-correct native paths. A sandbox may legitimately be beneath the repository because all temporary output must stay inside this ticket; its isolation control now compares its projected directory against the global staging directory rather than incorrectly forbidding every repository-descendant sandbox. No production staging behavior was changed.

## Test-Driven Evidence

- Before repair, the selected ordinary route ran 20 passing, two failing and 133 skipped cases in 9.75 seconds. It reproduced the absent WGPU command ENOENT and exposed the sandbox-prefix assumption under the required ticket-local TMPDIR.
- Before the cache metadata edit, the added input-closure control failed with the missing stagingRootSources named input: one failure and 155 skipped cases in 9.81 seconds.
- Final ordinary package route: `bun ./📜️script.ts test quick --testNamePattern='one plugin staging root|staged module freshness'` from the OS dev TypeScript package. Exit zero, 37 passing and 119 skipped cases in 5.47 seconds, one passing and two skipped files.
- Actual registered route: `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache --testNamePattern='one plugin staging root|staged module freshness'`. Exit zero, 37 passing and 119 skipped cases in 4.59 seconds, Nx run 6.1 seconds, critical path 5.6 seconds, cache explicitly skipped.
- All test/Nx/temp/artifact outputs were isolated below this repair's ticket-generated child. No live browser, compiler, service or publication was started.

## Native Input And Editor Closure

At 2026-09-13T02:25:13.180Z, installed Nx 23.2.0 getTargetInputs independently expanded all five existing test routes. Every route contained all 65 exact staging-source inputs; each had 74 self patterns and nine dependency patterns. The explicit target inputs preserve Nx's native default and dependency-default behavior and add stagingRootSources. Installed TypeScript 5.9.3 parsed the changed test with zero diagnostics.

The 65-entry source map contains all 63 source files plus the portable fixture and executing test. The test rejects duplicate source identities, checks every selected source for retired roots/symbols, and compares the full named input array against the exact fixture/test/source set. The existing project default glob covers its own manifest. This is exact additional source-data coverage, not a claim that these 65 files exhaust every transitive runtime import.

| Existing target | Native self patterns | Native dependency patterns | Missing declared sources |
| --- | ---: | ---: | ---: |
| `browser-host-staging-check` | 74 | 9 | 0 |
| `test` | 74 | 9 | 0 |
| `test-quick` | 74 | 9 | 0 |
| `test-long` | 74 | 9 | 0 |
| `test-exhaustive` | 74 | 9 | 0 |

The seed now declares `⚖️gate🔌️plugin📦️staging-root`, group `4_gate`, order 400.21, selecting the same existing filtered test-quick route. At 2026-09-13T02:34:26.363Z, the actual current registry renderCatalogFiles and generateLaunchJson authorities regenerated launch.json: 3219 prior and 3220 resulting named entries, exactly this one addition, zero removed identities, 61 playgrounds and 59 component launchers. No other generated catalog outputs were published.

## Exact Changed Product Paths

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

Only these five product paths are attributed to this repair, with seed-derived launch projection rather than hand editing the output. Shared unrelated hunks remain owned by their other authors. Root also created this report and updated the queue/index/coordinator ledger.

## Exact Current Source Selection

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔄️refresh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🏗️staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔐️lease/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔍️freshness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️layering-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️export-path-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔬️catalog-smoke/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/🏗️structure/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/🖼️pixels/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/🔬️probe/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/🌐️server-pool/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/📊️report/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/⚖️parity/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📥️source/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🔍️freshness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🖥️host/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🌐️browser/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🧪️stub/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🛂️validation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🦀️native/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🌐️.html`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`

## Limits And Cleanup

The earlier broad quick failure is now repaired at its selected staging/freshness boundary. The unfiltered quick suite was not rerun, and the remaining OS browser-host private receipt/no-follow, hostile distribution publication and activation lease/cancellation controls remain separate tasks. There is no full OS lifecycle, native Windows, Rust/Wasm or browser result. Source as data for mandatory command files remains necessary where those commands still contain separately queued implementation; this repair does not accept those bodies as anonymous leaves.

Owned disposable output is `🗑️generated/coordinator/os-staging-root-repair`; exact evidence is retained above and the source inventory remains in the portable fixture. Cleanup follows independent audit and does not touch other lanes. The scoped three-product-file whitespace check passed before the seed insertion; final five-path whitespace/seed parity validation is recorded below when complete.


Final five-path `git diff --check` passed with exit zero. Native Bun JSONC parsing found exactly one matching launch entry in both seed and generated output, equal in name/type/request/command/cwd/group/order to the intended current selection. These are static/editor-projection results; no VS Code UI action ran.


At 2026-09-13T02:40:11.199Z, root removed only its two completed generated children after native lsof +D returned exit1 with no open-handle rows and all root sessions had ended. The complete accepted evidence, current authored controls, reports and product fixtures remain. Native no-follow inventory and final absence checks:

| Owned generated child | Files | Directories | Symlinks | Bytes | Final state |
| --- | ---: | ---: | ---: | ---: | --- |
| `🗑️generated/coordinator/postcss-loader` | 18 | 8 | 0 | 1008530 | absent |
| `🗑️generated/coordinator/os-staging-root-repair` | 990 | 7 | 0 | 262544390 | absent |
