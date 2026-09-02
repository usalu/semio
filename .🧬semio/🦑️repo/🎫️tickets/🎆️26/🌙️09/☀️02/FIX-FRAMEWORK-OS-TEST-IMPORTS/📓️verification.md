# Framework-OS Test Import Verification

## Repaired References

- Four local-interaction tests now read `🧫️fixtures/🏠️local-interaction/🧪️query/🔣️.json`.
- Two linked-session-engine tests now read `🧪️fixtures/🔣️.schema.json`.
- `🟦️backbone-worker.ts` already imports `./🟦️`, rather than the removed `./🟦️component`, in the concurrently staged taxonomy migration.

## Checks

- The fixture files exist, and no repaired source retains either removed path.
- `bun --eval` imported the actual `linkedSessionEngines` implementation, validated the local-interaction and linked-session fixtures with Ajv, checked every valid vector's engine output, and checked that invalid vectors fail both schema validation and implementation validation.
- `bun 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts test` reached 215 passing tests, including the repaired local-interaction tests. It still exits non-zero because the concurrent taxonomy migration moved five workflow fixtures below `🧪️*/🗣️.dsl` while a separate test still scans only the fixture root for `*.dsl`.
- `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts test` reaches nine passing tests, then fails before the in-source `📜️script.ts` suite starts: Vite injects an import before its shebang and Rollup rejects the resulting source. The linked-session fixture contract was therefore verified by the direct Ajv-and-implementation check above.
