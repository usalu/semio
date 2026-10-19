# Audit: Layering Violation Inventory (dependency-cruiser)

Read-only audit by Haiku auditor. Repo `/Users/ueli/Documents/semio`, snapshot taken 2026-10-18 ~19:17 (peers keep editing, counts drift).

## 0. Verdict

- Current dependency-cruiser run: **11,025 error violations**, 0 warnings, over 19,808 cruised modules (`summary.totalCruised`). The recorded value in `RESULT-REPO-GATES-7.md` (9,237) is stale: it predates later edits, and the per-rule counts there do not match (for example `no-cross-package-relative` was 5,215, now 6,957).
- 63% of all violations are one family: `no-cross-package-relative` (deep relative imports into another package's internals). These are fixable mechanically by going through public package entries, not by dependency inversion.
- Real direction violations are concentrated in plugins, hub, and the OS product. `framework/modules` and `framework/packages` depend on no product (0 violations for that direction).
- About 387 violations are config false positives (node builtins `util`/`zlib`, and react imported through the `.cache` store path). Fix those in the cruiser policy, not in code. About 866 `no-core-path` hits come from the taxonomy stem `base` (`🧱️base` artifact subsets), which is a taxonomy decision.

## 1. Method and exact command

- Config: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`. It is a 5-line wrapper over `loadDependencyDirectionPolicy(...)` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs`. The wrapper file is untracked in git.
- Invocation, copied from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🕸️dependency-lint/🟦️.ts` (the `verify` gate uses the same roots with `--output-type err`):
  `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config <config above> --output-type json --output-to <report>`
- dependency-cruiser 16.10.4 (installed). Exit 0, runtime about 30 min under load.
- Raw JSON (63 MB) was written to `🗑️generated/depcruise.json` in this ticket and removed after aggregation. The aggregation scripts are in the session scratchpad, not in the ticket.

## 2. Inventory by rule family

| rule family | count | what it means |
|---|---:|---|
| no-cross-package-relative | 6957 | deep relative import into another package's internals (the `-<package>` suffix names the target package) |
| no-core-path | 1179 | a path segment equals a banned stem (`base` 866, `core` 14, `shared` 7, 177 unmatched by exact segment (compound names such as `graph-core`, file names), Node builtins `util`/`zlib` 115 = false positives) |
| plugins-framework-sdk-only | 735 | plugin source imports framework/product API that is not the neutral SDK |
| no-circular | 688 | import cycle (os 346, framework/modules 182, plugins 92, repo 35, ui 22, other 11) |
| no-cross-technology | 620 | hub, scripts, or dev catalog import framework across a technology boundary |
| renderer-hosts-only-ui | 433 | OS react renderer imports outside its allow-list (266 are `.cache` react false positives, 6 are `bun:test`) |
| ui-no-framework-packages | 285 | `framework/modules/ui` imports framework modules or packages |
| no-state-outside-os | 112 | non-OS code imports OS store/session internals |
| plugin-no-extension-or-artifact | 15 | plugin imports artifact or extension internals |
| framework-no-implementation | 1 | `framework/products/repo` imports `hub` |
| **total** | **11025** | |

`framework-no-products` no longer appears in this run at all (recorded 190 at 22:40). `framework-no-plugin-packages` and `s-modules-no-plugins` have 0 hits.

## 3. Violations by source area (top 12)

| source area | count |
|---|---:|
| framework/products/os | 3754 |
| framework/products/repo | 2393 |
| s/plugins/stdio | 1269 |
| framework/modules/ui | 858 |
| s/dev/catalog | 158 |
| hub/compositions/stdio | 122 |
| hub/bootstrap/provenance | 114 |
| s/plugins/cad | 100 |
| s/plugins/norm | 89 |
| s/plugins/flow | 83 |
| framework/modules/actor | 80 |
| framework/modules/process | 68 |

Target areas with the most incoming violations: framework/products/repo 3137, framework/products/os 1890, framework/modules/process 1610, framework/modules/ui 1017, s/plugins/stdio 917, framework/modules/actor 439, framework/modules/schema 355, framework/modules/compiler 294.

## 4. Top 30 from-area to to-area edges

Areas are the first three path segments with leading emoji stripped in the grouping; `(root)` is a path with no folder. `.cache` targets are the bun store, `cache/b/d` in the table.

| # | from → to (area) | count | rule families | example from → to |
|---|---|---:|---|---|
| 1 | `framework/products/os → framework/products/os` | 1247 | no-cross-package-relative 729, no-circular 346, no-core-path 118, renderer-hosts-only-ui 54 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/♻️artifact-lifecycle/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/⚙️constants/🟦️.ts` |
| 2 | `framework/products/repo → framework/modules/process` | 1199 | no-cross-package-relative 1199 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🏗️builder/🏃️execution/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` |
| 3 | `s/plugins/stdio → s/plugins/stdio` | 905 | no-core-path 856, no-circular 27, no-cross-package-relative 12, plugin-no-extension-or-artifact 10 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️generator/⚙️mechanical/📜️script.ts` → `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️generator/📜️script.ts` |
| 4 | `framework/products/os → framework/products/repo` | 792 | no-cross-package-relative 768, no-core-path 17, renderer-hosts-only-ui 7 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🧪️tests/🔬️interactivity-mcp-http-transport/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🗄️database/🌱️database-lifecycle/🟦️.ts` |
| 5 | `framework/modules/ui → framework/modules/ui` | 564 | no-cross-package-relative 534, no-circular 20, no-core-path 10 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🗺️mesh-collection/🟦️.ts` → `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` |
| 6 | `framework/products/repo → framework/products/repo` | 553 | no-cross-package-relative 493, no-circular 35, no-core-path 25 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/📤️publication/📦️transaction/🟦️.ts` |
| 7 | `framework/products/os → framework/modules/ui` | 384 | no-cross-package-relative 384 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts` → `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` |
| 8 | `framework/products/os → framework/modules/actor` | 377 | no-cross-package-relative 314, renderer-hosts-only-ui 63 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts` → `🧰️framework/🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts` |
| 9 | `s/plugins/stdio → framework/products/repo` | 338 | plugins-framework-sdk-only 178, no-cross-package-relative 147, no-cross-technology 13 | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/📦️packages/🟦️typescript/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts` |
| 10 | `framework/products/repo → framework/modules/schema` | 324 | no-cross-package-relative 324 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🟦️.ts` → `🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts` |
| 11 | `framework/products/os → cache/b/d` | 266 | renderer-hosts-only-ui 266 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/👥️session-presence-and-caret/🟦️.ts` → `.cache/b/d/m/j/4151d2262cb4a4e7a25a/node_modules/.bun/react@19.3.1/node_modules/react/index.js` |
| 12 | `framework/products/os → framework/modules/compiler` | 152 | no-cross-package-relative 150, renderer-hosts-only-ui 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧬️shell-component-decomposition/🟦️.ts` → `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts` |
| 13 | `framework/modules/ui → framework/modules/process` | 119 | ui-no-framework-packages 118, no-cross-package-relative 1 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🐍️python/🔌️provider/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts` |
| 14 | `s/dev/catalog → framework/products/os` | 91 | no-cross-technology 46, no-cross-package-relative 45 | `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎬️host-activation/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧩️catalog/🟦️.ts` |
| 15 | `s/plugins/norm → framework/products/repo` | 87 | plugins-framework-sdk-only 44, no-cross-package-relative 40, no-cross-technology 3 | `✏️s/🔌️plugins/📕️norm/🏭️bridge/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏭️inventory/🟦️.ts` |
| 16 | `hub/compositions/stdio → framework/products/repo` | 79 | no-cross-package-relative 47, no-cross-technology 32 | `🌎️hub/🧩️compositions/🗄️stdio/🏭️bridge/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts` |
| 17 | `framework/products/os → framework/modules/manifest` | 71 | no-cross-package-relative 62, renderer-hosts-only-ui 9 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️native-accessibility/🟦️.ts` → `🧰️framework/🔨️modules/🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts` |
| 18 | `framework/products/os → framework/modules/process` | 66 | no-cross-package-relative 63, renderer-hosts-only-ui 3 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` → `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` |
| 19 | `framework/products/repo → framework/modules/filesystem` | 62 | no-cross-package-relative 62 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/👓️reading/📥️inspection/🧪️tests/🟦️.ts` → `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` |
| 20 | `framework/products/repo → framework/modules/compiler` | 60 | no-cross-package-relative 60 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔗️schema-reads/🟦️.ts` → `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🟨️ecma/🟦️.ts` |
| 21 | `s/plugins/wfc → framework/products/repo` | 56 | no-cross-package-relative 28, plugins-framework-sdk-only 28 | `✏️s/🔌️plugins/🀄️wfc/⚙️engine/📦️packages/🦀️rust/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts` |
| 22 | `s/dev/catalog → framework/products/repo` | 56 | no-cross-package-relative 28, no-cross-technology 28 | `✏️s/🧑‍💻dev/🧩️catalog/🌐️browser-host/🏗️staging/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🌐️service/🟦️.ts` |
| 23 | `framework/products/repo → framework/products/os` | 53 | no-cross-package-relative 48, no-state-outside-os 5 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🏭️member/📥️source/🟦️.ts` |
| 24 | `hub/bootstrap/provenance → framework/products/os` | 48 | no-cross-package-relative 24, no-cross-technology 24 | `🌎️hub/🏗️bootstrap/🧾️provenance/🌍️gis-browser-proof/🟦️.ts` → `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` |
| 25 | `hub/bootstrap/provenance → framework/products/repo` | 48 | no-cross-package-relative 24, no-cross-technology 24 | `🌎️hub/🏗️bootstrap/🧾️provenance/📦️cargo/🎯️selection/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🔎️verification/🎭️actor/🟦️.ts` |
| 26 | `framework/modules/actor → framework/modules/actor` | 47 | no-circular 47 | `🧰️framework/🔨️modules/🎭️actor/📃️page/🧪️tests/🧪️actorbytepage-matches-shared-vectors-and-node-buffer-for-every-fixed-wor/🟦️.ts` → `🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts` |
| 27 | `framework/products/repo → framework/modules/identity` | 46 | no-cross-package-relative 46 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts` → `🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts` |
| 28 | `framework/products/os → framework/modules/kernel` | 42 | no-cross-package-relative 36, renderer-hosts-only-ui 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts` → `🧰️framework/🔨️modules/🎠️kernel/📦️package-assets/🟦️.ts` |
| 29 | `framework/modules/ui → framework/packages/typescript` | 41 | ui-no-framework-packages 41 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🌈️color/🟦️.ts` → `🧰️framework/📦️packages/🟦️typescript/🫧️transient/🟦️.ts` |
| 30 | `framework/products/repo → (root)` | 40 | no-core-path 40 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🎼️utilities/🟦️.ts` → `util` |

| rule family | count |
|---|---:|
| no-cross-package-relative | 6957 |
| no-core-path | 1179 |
| plugins-framework-sdk-only | 735 |
| no-circular | 688 |
| no-cross-technology | 620 |
| renderer-hosts-only-ui | 433 |
| ui-no-framework-packages | 285 |
| no-state-outside-os | 112 |
| plugin-no-extension-or-artifact | 15 |
| framework-no-implementation | 1 |

## 5. Config-level false positives (fix in the policy, not in code)

- `no-core-path` treats Node builtins as banned stems: `util` 89 and `zlib` 26 (115 total, all `to` without a slash). Exempt `node:`/builtin specifiers.
- `renderer-hosts-only-ui`: 266 hits resolve to `.cache/b/d/m/j/4151d2262cb4a4e7a25a/node_modules/.bun/react@19.3.1/.../react/index.js` because `node_modules` is a symlink into `.cache`. The allow-list matches the resolved path only. Allow the store path or match on the package specifier. 6 more are `bun:test`.
- `no-core-path` stem `base` (taxonomy `bannedNameStems` includes `base`): 866 hits from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/🪆️subsets/🧱️base/**`. Either rename the subset folder family or add a taxonomy exception. This is one decision that clears about 8% of the total.

## 6. Work packages (≤ 12)

Each violation is assigned to exactly one work package by its source area and target area (the classification script is in the scratchpad, totals sum to 11,025). A single source file can still appear in two packages (for example a plugin file with imports to both repo and os), so coordinate per file, not per package.

| WP | source → target | violations | source files | fix direction |
|---|---|---:|---:|---|
| WP1 | plugins → framework (repo 1102, modules 33, ui 8, other 14) | 1157 | 367 | plugins import only the neutral SDK; expose the repo process/cargo/inventory APIs through a framework public entry (`framework/modules/process` package root) and remove deep imports. Largest single pair: `s/plugins/*` → `framework/products/repo` |
| WP2 | plugins → framework/products/os | 333 | 137 | plugins talk to the OS only through its public host/plugin bridge contract; `no-state-outside-os` 99, `plugins-framework-sdk-only` 147 |
| WP3 | plugins ↔ plugins (and s-dev → plugins) | 989 | 365 | rename or exempt the `base` subset folder (866 no-core-path); break 89 plugin cycles through a shared plugin contract package |
| WP4 | hub → framework | 713 | 141 | hub bootstrap and compositions use framework public packages instead of `framework/products/repo` internals (454), `framework/modules` (150) and `os` (109) |
| WP5 | s-dev, s-tests, mit-bestand → framework/os | 393 | 96 | catalog/dev tooling consumes framework through public packages; `no-cross-technology` dominates |
| WP6 | os (non-renderer) → os | 475 | 156 | split internal OS deep imports into the OS store/flow public index; breaks some of the 346 OS cycles |
| WP7 | os (non-renderer) → repo | 712 | 216 | OS MCP/transport tests and laws import repo `laws/` and `caching/` internals; move shared test fixtures to a neutral test-kit package |
| WP8 | os (non-renderer) → framework modules, ui, other | 453 | 145 | replace deep relative imports into `framework/modules/{schema,manifest,kernel,process,actor,compiler}` with package entries |
| WP9 | os/renderer → all | 2114 | 506 | renderer: 772 internal cycles/deep imports, 624 to framework modules, 361 to ui, 266 false-positive react path (section 5). Real renderer-host violations are about 161 (`renderer-hosts-only-ui` less FP) |
| WP10 | framework/products/repo → framework modules, repo internal, os, other | 2393 | 733 | 1725 deep imports into framework modules (1199 to `framework/modules/process` alone); move to package entries; 553 repo internal (35 cycles) |
| WP11 | other products (print, quiz, pets, presentation, server) → repo/others | 112 | 72 | small, mostly `product-other → repo` (60) and internal deep imports |
| WP12 | framework modules, packages and ui → framework modules/ui/other | 1178 | 354 | `ui` must not depend on framework packages (285): inject through a ui port; `framework/modules/ui` internal deep imports (534 ncpr) and 182 framework-module cycles (actor 47) |

Notes for the owner:
- No framework module depends on a product: `framework/modules|packages|ui` → `framework/products/*` has 0 violations. The framework-no-products direction is already clean; the remaining work is public-entry hygiene and cycles.
- Recommended order: config fixes (section 5) first, since they clear about 387 hits without code change; then WP2 and WP1 (direction violations), then WP4 (hub); the ncpr-only packages (WP8, WP10) are mechanical.

