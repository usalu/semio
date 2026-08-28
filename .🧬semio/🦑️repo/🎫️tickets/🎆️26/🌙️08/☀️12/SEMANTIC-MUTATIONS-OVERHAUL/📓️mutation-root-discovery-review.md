# Mutation Root Discovery Review

Read-only review of the newly added tests at [`library TypeScript index.test.ts:5544`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts:5544) and neutral fixture [`mutation-root-discovery/🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-root-discovery/🔣️.json).

## Corrected findings

1. The extraction assertion is sound: the two top-level constants exist and the root's actual red/green test run passed it. My prior claim that the constants were absent was incorrect.

2. The authored `includedRoots` below packages, tests, fixtures, examples, assets, targets, generated and manifest are intentional requirements. The old helper's local skip list was the defect that the new tests correctly expose, not a neutral-fixture error.

3. The virtual scenarios deliberately cover opaque/cache rejection without constructing real compose paths. Root reports the current independent result as 32/32 virtual and 13/13 physical observations across both transpilers; no physical-compose case is required for this test.

4. Neither test distinguishes a bare directory named `🧬️mutations` from a canonical aggregate root containing `🦀️.rs`. That may be intentional for a discovery helper, but it is not a direct-mutation-root proof. The later structural rule must remain required; alternatively add an explicit test name and assertion documenting that discovery is facet-only.

No files were edited outside this ticket report; no compose path, compiler, or runtime test was accessed.
