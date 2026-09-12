# Current Composition API Declaration Closures

The coordinator used the installed TypeScript checker to bind references among top-level declarations in the current OS dev and Hub command sources. This is a read-only extraction-planning artifact. It is not a typecheck, an import-resolution audit or runtime execution. Import declarations terminate local dependency traversal. References are followed transitively by checker symbol identity, including type-only dependencies. Class bodies outside these explicit public API seeds remain additional work; these closures are not a plan to move one entire file into one helper.

## os-dev

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts. Current 5690 newline segments; 379 top-level statements and 998 internal statement edges. Native parser diagnostics: 0.

| API Concern | Local Declarations | Declaration Lines | Imported Statements | Missing Seeds |
| --- | ---: | ---: | ---: | ---: |
| browserStaging | 15 | 165 | 15 | 0 |
| pluginPublication | 42 | 368 | 16 | 0 |
| engineActivation | 7 | 62 | 5 | 0 |
| distributionPublication | 7 | 135 | 5 | 0 |

## hub

Source: 🌎️hub/📦️packages/🦀️rust/📜️script.ts. Current 17239 newline segments; 435 top-level statements and 1526 internal statement edges. Native parser diagnostics: 0.

| API Concern | Local Declarations | Declaration Lines | Imported Statements | Missing Seeds |
| --- | ---: | ---: | ---: | ---: |
| fixtureExpectation | 2 | 5 | 0 | 0 |
| orderedDirectoryPublication | 2 | 72 | 2 | 0 |
| credentialDelivery | 10 | 88 | 2 | 0 |

All expected API seeds were found. The seven bounded closures provide initial ownership cuts for 📓️large-composition-script-extraction-packet-2026-09-12.md. Shared declarations appear in multiple closures and require actual semantic ownership; the reported counts must not be summed as unique changes. DistributionBundlePlan/materializeDistributionBundle only cover that explicit exported projection, leaving further distribution compiler/publication class bodies to inspect. The same caveat applies to Hub credential/fixture roots versus its numerous native composition proof classes.

Exact seed names, bound local declarations, native source line coordinates, imported statements and source SHA provenance are in generated/coordinator/composition-declaration-closures-current.json. Re-read current files before mutation; no source SHA belongs in a permanent ownership test. No product change, process execution, live output or lifecycle mutation occurred.
