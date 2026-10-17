# Result: Hub Schema Catalog (slice HUB-SCHEMA-CATALOG, prefix `hsc`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Tools: `hsc-tools/`. Logs: `🗑️generated/hsc-*` (nothing deleted). No `[DEBUG]` log added.

STATUS: COMPLETE. Every row below was run.

## 0. Found state after the session restart

Checked on restart: every move and edit was fully applied; no half-applied move.
- Moved and present: `📦️cargo/🧬️schema`, `🗄️stdio/📥️execution`, `🧪️testing/🎛️run`, `🧪️testing/💾️backup-restore`. Old locations absent (`cargo/🧫️fixture/🧬️schema`, `📦️packages/🦀️rust/📥️execution`, `🧪️tests/🎛️run`, `🧪️tests/💾️backup-restore`, `🎟️carried-phase/🧬️schema`).
- All six kept schemas carry exactly one `$id`; every importer path edit is present (grep verified).
- No background process of mine was left running. The restart killed one `bun test` sequence (its first run died on `ENFILE: file table overflow`, a machine condition) and the generate/docs/check loop; all re-run below. A second restart (00:18) interrupted a `schema generate` loop; the catalog JSON still parsed and every edit was intact, so nothing needed rolling back. `ENFILE` and machine load made foreground runs long; the final generate, docs and `--check` ran in separate calls.
- tsc run (`hsc-tools/tsconfig.json`, 15 touched files) had completed before the restart: one error, `🤝️integration-harness/🟦️.ts(311)` (`Headers` iterable), a file I did not touch.

H = `🌎️hub`, L = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 1. Verdict

- The seven uncatalogued hub schema modules are now all catalogued or removed; the hub-tests `📌️document-check-in` file loads (2 tests, 0 failed).
- hub-tests route: **12 of 12 files load** (before 11 of 12). Failing tests **9 to 5**: 0 new failures, 4 fixed (all in `🤝️integration`, which previously died on the same stale-catalog guard). The 5 remaining failures are untouched by this slice (section 5).
- `schema generate --check`: passes ("is current (3566 scopes)") when run right after `generate` and `docs`. Scopes 3560 to 3566 (+6).

## 2. Why the generator skipped them (inventory diagnostics, `hsc-tools` run of `slf-tools/inv.mjs`)

The catalog only admits a module that declares a valid `$id`, sits at an eligible owner level, and is not a test-corpus contract. `hubSchemaExport` (`🤝️integration-harness`) refuses to run unless every `🧬️schema/🔣️.json` under H is catalogued.

| # | Module (under H) | Cause (diagnostic) | Fix |
|---|---|---|---|
| 1 | `🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/🧬️schema` | no `$id`; `schema-fixture-defines-schema` (wire marker `semio.hub.check-in-process-fixture/v1` names a fixture) | moved to the owned `📦️cargo/🧬️schema`, `$id` `hub/bootstrap/provenance/cargo/schema.json`, root export `Fixture` renamed `Seed`, wire marker renamed `semio.hub.check-in-process-seed/v1` (the data is the seed copied into the Host destination for purposes `runtime` and `publication`, as the cargo test names it) |
| 2 | `🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧬️schema` | no `$id`; dialect 2020-12 | draft-07 plus `$id` `hub/compositions/stdio/component/compiler/capture/schema.json`; its test switched from `ajv/dist/2020` to draft-07 `Ajv` |
| 3 | `🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📥️execution/🧬️schema` | `schema-owner-ineligible`: `**/📦️packages/**` never owns a schema | whole `📥️execution` module moved to `🗄️stdio/📥️execution`, `$id` `hub/compositions/stdio/execution/schema.json`; 7 relative imports re-based, script import and the test's script path updated, the package's Nx `namedInputs.default` gained the new folder |
| 4 | `🚀️local-bootstrap/🎛️host/🧬️schema` | no `$id` | `$id` `hub/local-bootstrap/host/schema.json` |
| 5 | `🧪️tests/💾️backup-restore/🧬️schema` | test collection owns a contract (`schema-fixture-defines-schema`) | the drill module (it is operational code run by the hub package script, not an example) moved with its schema to `🧪️testing/💾️backup-restore` (`testing-support` owner), `$id` `hub/testing/backup-restore/schema.json` |
| 6 | `🧪️tests/🎛️run/🧬️schema` | same | runner module moved to `🧪️testing/🎛️run`, `$id` `hub/testing/run/schema.json` |
| 7 | `🧪️tests/🎟️carried-phase/🧬️schema` | same; it only validated the parameter lists of its own corpus `🔣️.json` | no domain contract exists, so the schema was deleted (pre-image `🗑️generated/hsc-carried-phase-schema.json.pre`); the bun test keeps its AST oracle and replaced the schema check by `toHaveLength(4)` and `toEqual(["invocation","phaseControl"])` |

## 3. Readers updated

- Seed rename: `📦️cargo/🧫️fixture/🟦️.ts` (schema import path, type literal), `📦️cargo/🧪️tests/🟦️.ts` (2 imports), `📦️cargo/🧪️tests/🔣️.json` (corpus marker), `🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts` (type literal), `🧪️tests/🔬️bin-unit/🚀️boot/🔬️unit/🦀️.rs` line 582 (string literal only; **not compiled**, cargo gate). TS names `CheckInProcessFixtureV1` etc. and the produced directory `check-in-process-fixture` keep their names (a rename of the API is a separate sweep).
- Moves: `📦️packages/🟦️typescript/📜️script.ts` (4 specifiers), `🚀️local-bootstrap/🎛️host/🧪️tests/🟦️.ts` (5 specifiers/paths), `🗄️stdio/📦️packages/🦀️rust/📜️script.ts`, `📋️project.json`, execution test.
- Not touched: `🔣️taxonomy.json` (`members-of-tests` still lists `💾️backup-restore`; harmless registry name).

## 4. Verification (commands run and results)

| Command | Result |
|---|---|
| inventory diagnostics before/after (`inv.mjs`, `hsc-inv-all.log`, `hsc-inv-after.log`) | the 7 modules had 12 diagnostics; after: 0 for all six remaining modules; hub diagnostics 96 to 83 (rest: 65 `export-id-invalid`, 17 `export-incomplete`, and one `owner-ineligible` for the pre-existing empty directory `🗄️stdio/📦️packages/🦀️rust/🧬️schema`, no `🔣️.json`, untouched) |
| `bun ./📜️script.ts schema generate` / `schema docs` / `schema generate --check` | 3566 scopes; `--check` exit 0 ("is current") |
| the six scope ids in the catalog | `hub.bootstrap.provenance.cargo`, `hub.compositions.stdio.component.compiler.capture`, `hub.compositions.stdio.execution`, `hub.local-bootstrap.host`, `hub.testing.run`, `hub.testing.backup-restore`: each once |
| hub-tests vitest (`hsc-tools/vt.sh`; `hsc-sum-{base,after,final}.json`) | base 11/12 load, 72 tests, 9 failed; final 12/12, 74 tests, 5 failed, 4 skipped; 0 new failures |
| `tsc -p hsc-tools/tsconfig.json` (15 touched TS files) | 1 error, `🤝️integration-harness/🟦️.ts(311)` `Headers` iterable, a file I did not touch |
| `bun test` cargo tests | 16 pass, 1 fail (`🧾️provenance/🧪️tests` `import.meta.dir`/paths[0], not seed-related) with `SEMIO_TEST_ARTIFACT_DIR` set; the seed contract/use tests pass |
| `bun test` host tests | 24 pass, 0 fail (runner and drill schemas at their new `$id`/location) |
| `bun test` carried-phase | 3 pass, 0 fail |
| `bun test` execution | 1 pass |
| `bun test` capture | 2 fail, 1 pass: the schema validation line passes; the failures assert the content of `📜️script.ts` (`copyCatalogArtifact(` no longer in `EditorComponentCheckScript`), a source drift unrelated to the schema |

`schema generate --check` is racy under the fleet: peers edit plugin schemas continuously, so a check separated from the generate by a minute went stale (3 foreign scopes differed, `hsc-tools/catalogdiff.mjs`). Run it right after `generate`.

## 5. Foreign findings (not fixed)

1. Remaining hub-tests failures: `⛓️linked-codec-ownership` (ENOENT `🗄️stdio/🔣️.json`), `📊️observability` x3 (unresolved `$ref` to `os/plugin/describe/component-receipt/component.json`; README route parity `/healthz`), `🤝️integration` x1 (`fixture-base` vs `fixture.base`).
2. capture test source assertions drifted from `📜️script.ts`.
3. `🧾️provenance/🧪️tests` fails on `import.meta.dir`/`SEMIO_TEST_ARTIFACT_DIR`.
4. TS API still says "Fixture" for the seed (`CheckInProcessFixtureV1`, `🧫️fixture` folder); rename sweep needs the Rust test and 10 hub files.
5. Empty directory `🗄️stdio/📦️packages/🦀️rust/🧬️schema` yields an `owner-ineligible` diagnostic.

## 6. Files changed

- Moved: `H/🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/🧬️schema` to `📦️cargo/🧬️schema`; `H/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📥️execution` to `H/🧩️compositions/🗄️stdio/📥️execution`; `H/🧪️tests/🎛️run` and `H/🧪️tests/💾️backup-restore` to `H/🧪️testing/`.
- Deleted: `H/🧪️tests/🎟️carried-phase/🧬️schema/🔣️.json`.
- Edited: files listed in section 3, the five `$id` additions, `H/🧪️tests/🎟️carried-phase/🟦️.ts`, `L/🔣️schema-catalog.json`, `L/📓️schema-catalog.md` (regenerated by their own generator; pre-images `🗑️generated/hsc-schema-catalog.pre.{json,md}`).
- Created in F: `hsc-tools/` (`env.sh`, `vt.sh`, `sum.mjs`, `catalogdiff.mjs`, `tsconfig.json`) and this file.
