# Publication Generator Route Repair — 2026-10-21

The residual self-output path was an authored generator reference. The `plugin-registry` generator contract still named `✏️s/🧑‍💻dev/📜️script.ts` in both `inputPatterns` and `inputDiscovery.implementationEntryPaths`, and still passed `catalog-publication preview`. The publication commands had already moved to their dedicated source entry. This broad old entry could pull generated consumers back into generator discovery.

The neutral command fixture and closed schema expectation were written first, then a separate publication taxonomy regression, then the authored taxonomy was corrected. No test, dry run or bootstrap was executed during this edit; execution remains gated on the combined review and parent’s save boundary. The initial source observation showed the old two references and two preview arguments disagreed with the new fixture expectation.

## Exact Authored Changes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: only `generatorContracts["plugin-registry"]` was changed. Both implementation entry references now name `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts`; `previewArguments` is exactly `["preview"]`. Targets, owner, discovery protocol, resource bounds, other inputs and output roots retain their authored values.
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🧪️tests/🧫️commands.json`: adds language-neutral `generator` expectations for ID, owner, exact entry and preview arguments. Existing target/operation cases remain intact.
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🧬️schema/📥️commands.json`: requires the closed generator expectation object with exact authored constants.
- `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🧪️tests/🟦️.ts`: adds a separate taxonomy regression. It checks the current taxonomy via the native JSON parser and independent `jsonc-parser`, validates the neutral fixture through Ajv, asserts exactly one owner script input and one implementation entry, and checks the preview route against the fixture and actual publication entry’s TypeScript AST router keys.

Parent separately owns the Generate target’s freshness observation role and the graph fixture role assertion. This repair adds no execution-kind field, native skip or fake instrumentation. No unrelated source or private receipt/cache was changed.

## Regression and Execution Gate

The existing `@semio-tech/s-dev-composition:test-catalog-publication` target already registers the updated test entry, so the prepared four-root batch covers this new neutral contract. Parent’s source-demand role fixture is covered by the same batch’s `test-build-source-demand` root. `infra-batch-2026-10-21.md` retains the exact supported single-job/workers-off argv and missing target semantics. No assertion pass is claimed. Low was notified of all four source/schema/fixture/test paths for read-only combined review.
