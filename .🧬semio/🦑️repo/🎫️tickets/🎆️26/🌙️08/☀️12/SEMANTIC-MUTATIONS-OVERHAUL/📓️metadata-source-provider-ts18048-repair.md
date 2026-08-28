# Metadata Source-Provider TS18048 Repair

## Narrow Change

`mutationMetadataResolvedRoutes` now excludes both `null` and `undefined` in the outer type predicate and in the duplicate-provider comparison before either route's provider/facade fields are read. The preceding `some` rejection is runtime evidence, but TypeScript does not use it to narrow callback parameters. This removes the four TS18048 reads without weakening the rejection condition.

## Regression

The neutral fixture adds `facade-mixed-macro-and-trait-namespaces`: one façade publicly reexports both canonical `lower::MutationLeaf` and canonical `derive::MutationLeaf`. Its manifest declares both local dependencies. The proof must accept because type and macro namespaces resolve independently; the known wrong namespace route is excluded rather than dereferenced or treated as an ambiguous provider.

Executed:

`SEMIO_TEST_ARTIFACT_DIR='<ticket>/🧪️metadata-source-provider/🧪️ts18048-regression' bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test -t 'proves independent canonical derive'`

Result: one focused group passed, 28 assertions, 303 filtered, zero failures.

The existing strict library TypeScript target was also executed through the required workspace wrapper. It is red on six pre-existing/unrelated root-directory and `ImportMeta` diagnostics in actor/plugin/styling sources; its output contains no TS18048 diagnostic. No lint success is claimed.

Coherent discovery source SHA-256 after the repair:

`e3bb834cf51dfb97b8ca3118e3bab763d2826974f6052906e34e3584389257a1`

Scoped `git diff --check` passed.
