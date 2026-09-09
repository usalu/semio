# CI Baseline Canonical Case

The concurrently added flat CI baseline test and its language-neutral fixture moved under their existing baseline owner to `🧪️tests/🧭️baseline-selection`. Only the schema and production import paths changed within the test body; the cache-contract dispatcher now imports the named case. All assertions and the lodash, Ajv, and read-only Git ancestry oracles are preserved.

Public Bun/Nx ran the actual relocated case successfully: 11 baseline selection vectors, full fallback, input validation, cancellation, and native read-only Git ancestry passed. The case uses lodash as its selection oracle and Ajv to validate the language-neutral fixtures.

```json
[
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚦️ci/🧭️baseline/🧪️tests/🧭️baseline-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻ci-baseline-runtime/📜️script.ts",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-late-ci-baseline-2026-09-09.md"
]
```
