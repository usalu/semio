# October 19 Source Invocation Family Audit

## Scope

This read-only follow-up examines the tracked HEAD reference in memory, not a restored editor artifact or runtime catalog. It contains 12768 rows, whereas the complete pre-retirement inspection examined 13,043. The 275-row difference is not reconstructed or guessed. Exact family counts below apply only to this explicitly identified reference. Current compiler controls are grounded in live owner code, not that reference. string-argv, an existing third-party development dependency, parsed quoted argument vectors.

## Checked Families

2899 reference rows carry a TypeScript source token. 426 distinct present source files were read; 49 rows have a missing or dynamic source token. 42 source rows explicitly clear an environment value.

| Runner family | Rows | Required current shape |
| --- | ---: | --- |
| direct:bun-script | 1052 | run-script: Bun precedes the selected authored script; Arguments follow script |
| exec:bun-script | 1631 | run-script: Bun precedes the selected authored script; Arguments follow script |
| direct:owned-target-source-argument | 58 | Owning project target interprets source arguments; do not rewrite as bare Bun execution |
| exec:bun-test | 158 | bun-test target/tool: bun test precedes source; trailing Arguments are additional test flags/files |

Of the present sources, 38 contain explicit current invocation/issued-resource API references and 32 contain compiler invocation or current typecheck-owner references. These are source indicators, not execution proofs: passing file existence does not prove that an old flag profile or fresh-environment route satisfies the current Original.

## Current Compiler Owner

The existing repository typecheckProject function owns writable compiler state, dependency/product allocation, cancellation, deadlines and progress. It runs the pinned TypeScript compiler as Bun + compiler + --noEmit + -p + authored project + admitted diagnostic switches + managed incremental state. Its public argument contract accepts eight diagnostic switches; it does not accept arbitrary root-file, output, project or state replacements. A bare-script tool cannot emulate compiler-before-source ordering by appending words. The typed typecheck-project target added in this phase uses that owner rather than adding bun x tsc to a generic shell.

Subsequent parent verification repaired the owner's two unresolved schema-validator imports through the canonical public package entry. The actual compiler owner and authored standard-library contract now initialize under the isolated linker. The focused compiler form test passed one test and 50 assertions, including real module initialization and the existing independent selection/argument oracles. This supersedes the earlier module-load limitation only; actual admitted compiler execution remains unverified behind the native storage blocker.

## Representative Exact Reference Invocations

### direct:bun-script

- `bun "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts" test prototype`
- `bun "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts" test release`
- `bun "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts" types prototype`

### exec:bun-script

- `bun repo exec --projects=@semio-tech/framework-os-dev --exclude-dependencies -- bun "${workspaceFolder}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts" compile`
- `bun repo exec --projects=@semio-tech/framework-os-dev --exclude-dependencies -- bun "${workspaceFolder}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts" compile`
- `bun repo exec --projects=ticket-fixed-slots --exclude-dependencies -- bun "${workspaceFolder}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/pa11/📜️script.ts" receive pa11-red32 359e224bdd40be538c46adc9e239fb2dcf2920ef7be2b9cca19c71d898c8d3ce 944aa2bcf38cf6afd05bf02f8eae85c8d86f67a422cd30a40c6875e6c38f9a74 944aa2bcf38cf6afd05bf02f8eae85c8d86f67a422cd30a40c6875e6c38f9a74`

### direct:owned-target-source-argument

- `bun repo run @semio-tech/plugin-registry:test --configuration=prototype --skip-cache --exclude-dependencies -- "🧪️tests/🎚️component-modes/🟦️.ts"`
- `bun "repo" "run" "@semio-tech/framework-renderer-react:test" "--exclude-dependencies" "--skip-cache" "--" "long" "--run" "${workspaceFolder}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🧩️component/🟦️.tsx" "--reporter=verbose"`
- `bun "repo" "run" "@semio-tech/framework-renderer-react:test" "--exclude-dependencies" "--skip-cache" "--" "long" "--run" "${workspaceFolder}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/⏪️time-travel/🧪️tests/🕰️history-body/🟦️.tsx" "--reporter=verbose"`

### exec:bun-test

- `bun "repo" "exec" "--projects=workspace" "--exclude-dependencies" "--skip-cache" "--" "bun" "test" "${workspaceFolder}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🧬️mvd/🧪️tests/🎛️history-inputs/🟦️.ts"`
- `bun "repo" "exec" "--projects=workspace" "--exclude-dependencies" "--skip-cache" "--" "bun" "test" "${workspaceFolder}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🧪️tests/🔎️ready-candidate/🟦️.ts"`
- `bun "repo" "exec" "--projects=workspace" "--exclude-dependencies" "--skip-cache" "--" "bun" "test" "${workspaceFolder}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧪️tests/🎛️history-inputs/🟦️.ts"`

## Remaining Acceptance

Native admitted execution is blocked in the parent session. No claim is made that all historical script profiles run unchanged, that arbitrary raw compiler flags are admitted, or that source-file existence is friendly discovery. Current searchable source suggestions and compiler forms need native dry-run, actual finite task and accessibility journey verification after a valid installed dashboard is available.

## Issuer-Sensitive Source Indicators

21 source files contain an envelope-creation call and 4 contain an explicit fresh-entry environment marker. These indicate contracts needing review, not a blanket judgment that a fixture issuer is invalid.

| Source | Envelope marker | Entry environment marker |
| --- | --- | --- |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/issuer-bootstrap/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/native-cleanup/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/build-mode/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/sdk-derivative/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/graph-headroom/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/headroom/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/ui-wgpu-render/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/rust-headroom/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/actor-current/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/cargo-validation/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/cpu/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/verification/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/asset-publication/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | Fresh-entry environment contract |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/file-publication/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | Fresh-entry environment contract |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/parent-factory/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/tooling-diagnostic/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | Fresh-entry environment contract |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/native-publication/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/graph-purpose/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | Fresh-entry environment contract |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/const-fold/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/typescript/📜️script.ts` | Creates an envelope; actual Original acceptance must be checked | — |

The dominant typescript-artifact helper explicitly creates a synthetic envelope naming old repo-private-typescript-artifact Nx selections; its types profiles run raw compiler flags over selected individual files. Its current acceptance cannot be inferred from generic script invocation. tooling-diagnostic explicitly rejects incoming FreshEnvironment keys in fresh-entry routes. Neither the generic run-script control nor the new authored-project compiler target proves those old profiles accepted under the current actual Original. They need current permanent-owner acceptance tests and, where required, refactoring.
