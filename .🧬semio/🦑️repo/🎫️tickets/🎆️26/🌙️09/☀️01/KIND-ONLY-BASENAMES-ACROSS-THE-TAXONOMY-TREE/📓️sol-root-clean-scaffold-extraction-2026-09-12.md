# Root Cleanup and Scaffold Ownership Extraction

Date: 2026-09-12

## Outcome

The mandatory root command no longer defines the workspace cleanup, taxonomy CLI workflow, scaffold artifact/mutation authoring, concrete `new` command helpers, or direct mutation-directory index. Eighty-three declarations now live in eleven anonymous TypeScript leaves beneath their semantic domains. The root imports only the three command/index entry points it uses and retains command registration.

The portable contract enumerates eleven owners, fifteen exact taxonomy contexts, eight direct or source-as-data consumers, ten internal owner edges, and zero owner cycles or imports back to the root command. No root compatibility declaration or implementation re-export was added.

No live workspace cleanup, taxonomy apply, or scaffold publication was executed. All filesystem mutation evidence used private directories beneath this ticket.

## Schema-first red phase

The language-agnostic fixture and JSON Schema were written before the extraction. Its first Bun run produced one pass and four failures: `repo-workspace-cleanup` was not registered, the owner files did not exist, eighty moved declarations were still found in the root, and the Bun/Nx/launch route was absent. The final fixture grew to eighty-three declarations when the direct mutation index and test-output environment owner were included.

The final portable control validates its fixture with independent JSON and installed TypeScript parsers. It then resolves every declared context through the real taxonomy, typechecks all eleven owners with the installed TypeScript compiler, walks direct and source-text consumers, computes owner imports and cycles, executes the test-output owner through Bun and transpiled TypeScript, and checks the Bun/Nx/package/seed-derived launch route.

## Semantic owner tree

| Domain | Anonymous owner | Declarations |
| --- | --- | ---: |
| workspace cleanup protection | `🧼️workspace-cleanup/🛡️protection/🟦️.ts` | 34 |
| workspace cleanup discovery | `🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts` | 14 |
| workspace cleanup removal | `🧼️workspace-cleanup/🗑️removal/🟦️.ts` | 2 |
| workspace cleanup command composition | `🧼️workspace-cleanup/🎮️command/🟦️.ts` | 1 |
| taxonomy CLI orchestration | `🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts` | 1 |
| scaffold authoring contract/templates | `🏗️authoring/🧱️contract/🟦️.ts` | 5 |
| artifact tree authoring | `🏗️authoring/🗿️artifact-tree/🟦️.ts` | 5 |
| mutation tree authoring | `🏗️authoring/🧬️mutation-tree/🟦️.ts` | 15 |
| scaffold command composition | `🏗️authoring/🎮️command/🟦️.ts` | 2 |
| direct mutation-owner indexing | `🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts` | 3 |
| test-output environment | `🏃️process/🌿️environment/🧪️test-output/🟦️.ts` | 1 |

The full prefix of every table entry is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. The fifteen registered contexts include all owner directories and the new `🏃️process`, `🌿️environment`, and `🧪️test-output` ancestor chain. `loadCatalogTaxonomy()` reports no validation problems, and every fixture context resolves to its expected kind.

The internal graph has ten edges: candidate discovery uses protection; removal uses discovery and protection; cleanup command uses removal and the taxonomy workflow; artifact tree uses the authoring contract; mutation tree uses the contract and direct-owner index; scaffold command uses both authoring trees. The test-output environment owner has no edge to the router or root.

## Behavior retained

Workspace cleanup keeps protected prefixes, canonical ticket/repository boundaries, generated-output interpretation, manifest state, no-follow path handling, Windows-illegal and build-output discovery, shallow/deep de-duplication, dry-run projection, removal, and cache-prune delegation. The focused controls exercise strict protection projection, active Cargo lease protection, and removal of only generated ticket probes.

The taxonomy workflow retains inventory, plan, apply and verify orchestration around its already extracted option, capture, mutation and decision APIs. Cancellation is checked before planning and forwarded to each planning implementation. The real CLI fixture now hand-materializes deterministic SHA-1 empty-tree and commit objects plus refs with ordinary private-fixture filesystem writes. It uses read-only `git rev-parse` and `git cat-file` as oracles, so verification runs no `git init`, `git add`, or `git commit` command.

Artifact and mutation authoring retain no-overwrite behavior, explicit scaffold placeholders, dry runs, no-follow traversal, cancellation, aggregate replacement, changed-source rejection, and rollback restricted to owned inodes/content. The portable artifact fixture vocabulary was corrected from stale `🎚️options` to the registered `☑️options`; its request fixture and schema were updated together.

## Consumer and command closure

The mandatory root directly imports `CleanScript`, `CleanMechanismNewScript`, and `policyListMutationDirs` from their semantic owners. Its moved declaration set is empty.

The exact-Cargo and clean-ticket controls import the cleanup command owner. The taxonomy-cancellation control parses the workflow and cleanup command owners. The workspace contract imports the mutation-tree owner, dynamically imports protection, and points its subprocess inventory at the cleanup command owner. The artifact authoring control resolves the artifact-tree and test-output owners. The path statutes control reads the direct-owner index. The package router imports the test-output owner. None reads a moved declaration from the mandatory root.

The focused route is registered as `test-root-clean-scaffold-source` in the package router, Nx project, package script, launch seed, and derived launch output. The Nx target declares the schema, fixture, ownership control, taxonomy, owners, consumers, and root command as inputs.

## Zero-touch private output

The actual output/environment policy lives in `🏃️process/🌿️environment/🧪️test-output/🟦️.ts`, not in the mandatory package router. `repoTestArtifactEnvironment` preserves an explicit `SEMIO_TEST_ARTIFACT_DIR` and otherwise creates a route-specific directory below:

`<ticket>/🗑️generated/sol-root-clean-scaffold-extraction/repo-lib-test-artifacts`

The package router calls it for `process-budgets`, `exact-cargo-laws`, `taxonomy-cli-cancellation`, `artifact-empty-facet-authoring`, and `workspace-contract`. The portable control proves all five default paths, explicit override preservation, preservation of unrelated environment fields, directory creation through Bun and installed-TypeScript execution, and exact call sites. Ordinary package and registered Nx commands therefore require no developer-supplied environment variable.

## Runtime evidence

| Control | Result | Evidence boundary |
| --- | --- | --- |
| direct `bun ./📜️script.ts test root-clean-scaffold-source` after formatting, environment unset | 7/7, 140 assertions, 11.54 s | complete current owner/context/consumer/type/graph/environment/registration contract |
| isolated registered `@semio-tech/repo-lib:test-root-clean-scaffold-source --skip-nx-cache` after formatting, environment unset | 7/7, 140 assertions; test 23.51 s, Nx target 24.8 s | actual Bun → Nx → package router path; private Nx data/cache; cache skipped |
| direct `bun ./📜️script.ts test root-script-compiler` with this lane's artifact root | 6/6, 86 assertions, 4.46 s | Bun and esbuild compile the actual root and source-as-data declarations from their real owners |
| root module import | exit 0, `[DEBUG] root import succeeded` | current root dependency graph loads at runtime |
| ordinary `bun ./📜️script.ts test taxonomy-cli-cancellation`, environment unset, after final formatting | 4/4, 27 assertions, 7.73 s | zero-touch output, command delegation, cancellation, handcrafted committed fixture, read-only Git oracles |
| isolated registered `@semio-tech/repo-lib:test-taxonomy-cli-cancellation --skip-nx-cache`, environment unset | 4/4, 27 assertions; Nx target 3.9 s | actual registered zero-touch cancellation route |
| ordinary `bun ./📜️script.ts test artifact-empty-facet-authoring`, environment unset, after environment-owner move | 43/43, 602 assertions, 11.65 s | portable/native scaffold output, cancellation, containment and zero-touch route output |
| direct mutation transaction selection | 1/1, 45 assertions | guarded publication, rollback, cancellation, symlink/no-follow and changed-source behavior in private fixture |
| direct mutation idempotence selection | 1/1, 13 assertions | occupied implementation remains unchanged |
| direct active-ticket protection selection | 1/1, 58 assertions | first-party policy plus third-party oracle parity |
| direct clean-ticket fixture | 1/1, 5 assertions | four intended generated nodes removed; retained ticket material preserved |
| direct active exact-Cargo lease selection | 1/1, 2 assertions | active lease protects ticket evidence; disposable generated output can be removed afterward |
| direct mutation-index source selection | 1/1, 8 assertions | direct and captured inventories enumerate registered two-tier operation owners |
| ordinary exact-Cargo route, environment unset | 26/26, 596 assertions | zero-touch route startup and existing exact-Cargo controls before the helper was moved into its semantic owner |
| Prettier check over eleven owners and five focused controls | clean | final formatted owned sources |
| tracked staged and unstaged whitespace checks over root, launch, and repo-library paths | clean | no diff whitespace defects |

All Nx evidence used `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, a lane-private `NX_WORKSPACE_DATA_DIRECTORY`, a lane-private `NX_CACHE_DIRECTORY`, and `--skip-nx-cache`.

## Limits and observed intermediate failures

The minimal clean-ticket and exact-Cargo cleanup fixtures do not contain the cache-prune child script. Cleanup reports the delegated child as unavailable with exit 1, and the controls intentionally catch that result. These runs prove cleanup selection/removal/protection only; they are not cache-prune evidence.

An initial ordinary cancellation route failed before assertions because `SEMIO_TEST_ARTIFACT_DIR` was required but neither the route nor launch supplied it. The final environment owner closes that defect without placing output policy in the router. An earlier cancellation fixture used Git mutation commands; it was replaced by handcrafted committed-object materialization before the final 4/27 runs. One earlier isolated Nx attempt stopped during native plugin socket startup before its target; later isolated runs reached and passed their registered targets.

The artifact authoring compiler exceeded its earlier five-second case budget under concurrent load, and one package wrapper exceeded fifteen seconds. That case and route now have explicit thirty-second budgets; the final ordinary run passed in 11.65 seconds. The separate forty-five-second kind-only route budget was a coordinator-owned change and is not attributed here.

No broad workspace-contract suite, live cleanup, live taxonomy apply, live scaffold publication, shared Git mutation, or ticket lifecycle operation was run.

## Exact file attribution

New semantic owners:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🗑️removal/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🧱️contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🗿️artifact-tree/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🧬️mutation-tree/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/🎮️command/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts`

Portable contract:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-clean-scaffold-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-clean-scaffold-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-clean-scaffold-source/🟦️.ts`

Root and consumer closure:

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean-ticket-runs/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`

Artifact fixture vocabulary:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json`

Shared registration files, limited to this lane's owner contexts, imports, target/package route, and launch entries; concurrent hunks remain attributed to their owners:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The independent Terra audit accepted the eleven-owner semantic split, zero-touch launch repair, private-fixture containment, source/data consumer closure, and registered routing within the limits documented above.
