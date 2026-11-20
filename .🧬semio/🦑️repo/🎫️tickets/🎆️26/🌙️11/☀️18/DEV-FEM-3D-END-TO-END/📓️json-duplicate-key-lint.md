# JSON duplicate key lint

A JSON object that repeats a key is now a high-priority breach in `repo verify gate`.

## Where lints live
Real rules are not `defineLint` scripts (only the router `📜️script.ts` files use that, and they resolve graph entities). Whole-repo rules are "laws" in `⚖️laws/<law>/`: a module exporting `policyXBreaches(repoRoot): BreachRecord[]`, a `🧬️schema/`, a `🧪️tests/` with fixtures and a Python oracle, wired into `🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts` with bilingual texts in `📖️texts/🔣️.json`. The new rule follows that shape (modelled on `🧫️test-store-isolation` and `🚷️raw-process-spawn`).

## Files (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`)
- `⚖️laws/🔑️json-duplicate-keys/🟦️.ts`: `findJsonDuplicateKeysV1` (regex-skipping streaming scanner, no value built, reports `$`-path, 1-based line and decoded key; tolerates comments, BOM, syntax errors), `listJsonFilesV1` (tracked `*.json` via git, walk fallback), `jsonDuplicateKeyReportV1`, `policyJsonDuplicateKeyBreaches` (kind `repo/json-duplicate-key`, priority high).
- `⚖️laws/🔑️json-duplicate-keys/🧬️schema/🔣️.json`: schema of the fixtures.
- `⚖️laws/🔑️json-duplicate-keys/🧪️tests/🧫️fixtures/🔣️.json`: 15 language-agnostic cases (root, nested with line, siblings fine, array elements fine, keys inside strings, `"a"` vs `"a"`, surrogate-pair escape, arrays, triple repeat, bracketed path, nested vs outer, comments, BOM).
- `⚖️laws/🔑️json-duplicate-keys/🧪️tests/🐍️oracle/🐍️.py`: oracle on Python's `json` `object_pairs_hook`; shares only JSON data with the TypeScript code.
- `⚖️laws/🔑️json-duplicate-keys/🧪️tests/🟦️.ts`: 5 tests (schema, fixtures, Python agrees on fixtures, no duplicate in every 8th tracked JSON file with Python agreeing, breach record content).
- `🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts`: imports the law and runs it as gate step `gates.json-duplicate-keys`.
- `🎛️commands/✅️verify/📖️texts/🔣️.json`: `gates.json-duplicate-keys` and `gates.json-duplicate-key-breach` (en, de).

## Results
- `bun test` on the law test: 5 pass, 0 fail (31 s).
- `bun test` on the verify texts test: 3 pass.
- Full sweep of all 87,146 tracked JSON files: 0 duplicates (list 5 s, scan about 134 s on this loaded machine, I/O bound). The repo-wide unit test therefore samples every 8th file; the verify gate runs the full sweep.

## Notes
- Tests were run directly with `bun test <file>`; no refusal, so `native-run.ts` was not needed.
- The semio MCP was down, so no ticket tools were used.
- The Python oracle does not cover comments or BOM (fixture flag `oracle: false`).
