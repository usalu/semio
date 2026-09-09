# Product and Plugin Fixture Separation

## Scope and Taxonomy

This lane covers `✏️s/**`, including product modules and plugin trees, and excludes `🧰️framework/**`. The root and `✏️s/AGENTS.md` instructions were read before changes. The taxonomy used here is:

- `🧫️fixtures`: language-neutral testing-only example inputs and expected outputs, owned beside `🧪️tests` by the nearest shared semantic owner.
- `🖼️assets`: static data used by production or shipped examples.
- `🧬️schema`: contracts rather than example instances.
- `🧪️tests/<case>/<implementation>`: canonical test implementations and feature description only; fixture bundles do not remain below the case.

The repository test URI contract now resolves `shared://` only from `<owner>/🧫️fixtures`, resolves `asset://` only from `<owner>/🖼️assets`, and rejects `local://`.

## Completed Initial Wave

The initial authored census found 361 tracked files under 63 `✏️s/**/🧪️tests/<case>/🧫️fixtures` directories. All 361 files moved to `<owner>/🧫️fixtures/<case>/...`. Their 361 before/after SHA-256 pairs and byte counts are identical. The case implementations and features remain at `<owner>/🧪️tests/<case>/<implementation>`.

The migration changed 172 case source files from `local://<path>` to `shared://<case>/<path>`. A further three source files contained stale prose or generic resolver matching for `local://`; those references were removed. A scoped source scan after the migration found zero `local://` occurrences.

The Sequence editor WASM protocol compiled three test ledgers into production through `include_str!("🧫️fixtures/...")`. Only `🧪️tests/🔬️protocol-unit/🦀️.rs` consumes these values. The three includes now live in that test implementation, while `📡️protocol.rs` retains only the production schema include. This removes the production dependency while preserving the ledgers as testing-only owner fixtures.

The exact incremental move and source-change report is `🗑️generated/plugins/📊️moves.json`. The retained migration input is `🔌️plugins/📜️script.ts`.

## Remaining Asset URI Census

The current `asset://` census contains 1,586 occurrences in 345 source-like files and 668 distinct URI spellings. None currently resolve under the required `<owner>/🖼️assets` root. The dominant groups are:

- 1,167 occurrences rooted at `🧬️schema`, primarily tests reading example snapshots and mutation payloads embedded in schema test-case folders. These are example data, not contracts; they must move to the nearest shared `🧫️fixtures` owner and become `shared://` references.
- 304 occurrences rooted at `📚️examples/.../🖼️assets`. These require owner-by-owner classification: shipped example data belongs under the resolver's actual `🖼️assets` root, while test-only copies belong in `🧫️fixtures`.
- 28 occurrences spelled `asset://🧫️fixtures/...`, all in Energy BESTEST tests. The physical data is already in the subset owner's fixture directory; these references should become `shared://...` without moving the data.
- 83 occurrences name `plan.png`, `plan-v2.png`, or `elevation.png`, mostly strings inside Puzzle schema example snapshots. They are embedded example document values and require classification with the surrounding moved snapshot rather than superficial URI rewriting.
- Four occurrences traverse through another standard's example assets and two use a generated `{LEAF_DIR}` placeholder.

The flat census with source path, line, URI, inferred test owner, and current/required target existence is `🗑️generated/plugins/📊️asset-census.json`. The coordinator's broader physical snapshot is `🗑️generated/coordinator/fixture-layout-current.json`; its 12,510 case-local data findings overlap active moves and must be filtered against current existence before further migration.

## Verification Status

Byte preservation is proven for all 361 fixture moves by the flat JSON report. Source scans prove no authored `local://` remains in `✏️s`. Focused Nx runtime tests have not yet been run for this wave; verification must cover at least one migrated multi-language case, the Sequence WASM protocol unit case, and the Energy owner-fixture URI wave after its references are changed.
