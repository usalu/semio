# Root Taxonomy Workflow Extraction Acceptance Audit

**Status: accepted.** The live extraction has 11 taxonomy-owned implementation leaves, an acyclic dependency closure, direct consumers bound to the leaves, and a green actual Nx target. No root facade was restored.

## Independent execution evidence

All commands below used the audit-specific artifact, temporary, Bun-cache, and Nx-cache roots under `🗑️generated/terra-root-taxonomy-workflow-audit` while that scratch existed.

| Invocation | Result | What it establishes |
| --- | --- | --- |
| `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test root-taxonomy-workflow-source` | 4/4, 1,101 assertions | Package-command dispatch of the portable owner, exact context, root-declaration removal, and route-registration contract. |
| `bun nx run @semio-tech/repo-lib:test-root-taxonomy-workflow-source` | 4/4, 1,105 assertions; Nx target succeeded, cache 0/1 | Actual Nx target execution, distinct from package-command dispatch. It waited for another agent's graph construction, then ran normally in 5.2 s. |
| Package command `test inventory-artifact-shards` | 8/8, 156 assertions | Direct consumers use the serialization, shard, and publication owners; canonical content, reconstruction, corruption rejection, and registered metadata remain sound. |
| Package command `test mutation-source-index-capture` | 1/1, 19 assertions | Injected admission is used without a second collector; source roster/digest changes, rejected admission, and cancellation-before-progress are checked. |
| Package command `test mutation-ticket-role-routing` | 2/2, 200 assertions | Dynamic source-as-data imports reach the extracted workflow and index owners. The child observed all five expected source-admission calls. |

Sol's separate final verification also passed private Nx `run-many` for `test-mutation-ticket-role-routing`, `test-mutation-source-index-capture`, `test-mutation-source-file-facts`, and `test-mutation-source-roster-roles`, plus the Nx root target above. Its focused workflow core passed 9/9 with 407 assertions, including empty structural directories, no-follow scope handling, stable inventory, reachability, wrapped origin, and terminal application. Those results corroborate this audit but are not represented as independent execution.

An early exploratory invocation, `test source-index-capture`, was a wrong package-command route. It selected an unrelated broad path, exposed stale test consumers, and hit its 15-second default budget. It is not evidence of a registered-route budget defect. The exact registered route is `test mutation-source-index-capture`, which passed above.

## Taxonomy and dependency closure

The current fixture and live source declare these 11 owners:

| Parent context | Owner kind | Leaf |
| --- | --- | --- |
| `repo-normalization` | `repo-normalization-command-contract` | `🎮️command-contract/🟦️.ts` |
| `repo-normalization-inventory` | `repo-inventory-serialization` | `📇️inventory/🧾️serialization/🟦️.ts` |
| `repo-normalization-inventory` | `repo-inventory-shards` | `📇️inventory/🧩️shards/🟦️.ts` |
| `repo-normalization-inventory` | `repo-inventory-publication` | `📇️inventory/📦️publication/🟦️.ts` |
| `repo-normalization-mutation` | `repo-mutation-identity` | `🧬️mutation/🪪️identity/🟦️.ts` |
| `repo-normalization-mutation` | `repo-mutation-captured-source` | `🧬️mutation/📸️captured-source/🟦️.ts` |
| `repo-normalization-mutation` | `repo-mutation-source-index` | `🧬️mutation/📇️index/🟦️.ts` |
| `repo-normalization-mutation` | `evidence` | `🧬️mutation/🧾️evidence/🟦️.ts` |
| `repo-normalization-mutation` | `repo-mutation-workflow` | `🧬️mutation/🔁️workflow/🟦️.ts` |
| `repo-normalization-mutation` | `repo-mutation-structural-reachability` | `🧬️mutation/📐️structural-reachability/🟦️.ts` |
| `schema` | `repo-schema-subset-validation` | `🧬️schema/✅️validation/🟦️.ts` |

The owner AST import graph was refreshed after the final repair. It has **18 runtime edges and one type-only edge** (`workflow → captured-source`), for **19 internal edges**, with zero runtime cycles and zero owner-to-root-router edges. The root router imports six public owners at [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts:196): command contract, inventory publication, mutation identity, captured source, structural reachability, and workflow. The remaining owners are reached through those owners or direct test consumers. No direct import from `📜️script.ts` binds any moved workflow symbol.

## Behavioral review

`mutationTaxonomySourceIndex` accepts an injected admission and otherwise obtains one canonical admission for a snapshot. It captures admitted bytes into its index and hashes the sorted source roster at [📇️index/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️index/🟦️.ts:58). `inventoryMutationTaxonomy` deliberately captures a before/after pair to reject a changed source epoch; it projects all facts from each captured view rather than conducting an independent post-capture traversal at [🧾️evidence/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/🧾️evidence/🟦️.ts:192).

Cancellation is checked before each admitted-file capture and before each consumer-graph iteration. A pre-existing cancel file produces no progress events in the independently run index test. Explicit paths are fail-closed: every ancestor is `lstat`-checked for symlinks and non-`ENOENT` read failures propagate at [📸️captured-source/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📸️captured-source/🟦️.ts:74). An admitted source that becomes unreadable or absent cannot be silently omitted: index capture fails when no owned snapshot is available at [📇️index/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📇️index/🟦️.ts:78).

Structural reachability reads only the captured structural view. It requires exactly one unconditional canonical direct-leaf mount, derives a single wrapped declaration origin (`sourcePath`, `declarationName`, `modulePath`), and rejects ambiguity at [📐️structural-reachability/🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts:323). This preserves mount and origin evidence without another source graph.

## Resolved audit findings

The first broad exploratory run found three stale source-as-data tests still targeting the root router. They were corrected without re-exporting moved definitions:

| Former root consumer | Current owner |
| --- | --- |
| `🧪️tests/🎭️source-roster-roles/🟦️.ts` | `🧬️mutation/📸️captured-source/🟦️.ts` for `MutationTaxonomySourceRecord` |
| `🧪️tests/🎫️ticket-role-routing/🟦️.ts` | `🧬️mutation/🔁️workflow/🟦️.ts` and `🧬️mutation/📇️index/🟦️.ts` |
| `🧪️tests/🧾️source-file-facts/🟦️.ts` | `🧬️mutation/📸️captured-source/🟦️.ts` |

The dynamic source-index capture test already imports the indexed owner by content-hashed file URL. The current ticket-routing test likewise imports its workflow and index subjects by exact file URL. The repaired direct and private-Nx checks above confirm these bindings.

## Limits

I did not rerun Sol's 88-second workflow-core suite concurrently. Its final result is recorded separately above. This audit performed no implementation, Git, lifecycle, or product-output changes. Temporary command logs, AST tooling, caches, and fixtures were confined to the ticket scratch and removed after this report was written.
