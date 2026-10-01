# Snapshot SQLite Test Exploration

Read-only exploration performed on 2026-09-30. Root AGENTS.md, ✏️s/AGENTS.md and ✏️s/🔨️modules/AGENTS.md were read. No code changes or modifying Git commands were made. No tests were run, and this report makes no passing-test or working-runtime claim.

## Existing Owners and Fixtures

Universal artifact IO is owned by `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, with language mirrors under `🧬️schema` and Rust unit, fidelity and IO-mechanism-law tests under `🧪️tests`. Existing IO contexts expose cancellation, resource limits, budgets, payload source/sink boundaries and generic artifact codecs. Expensive import/export should consume those owned contracts.

Language-neutral before/after artifact snapshot JSON fixtures are extensively available under `✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/🧫️fixtures/🧬️mutations/*/*/📸️snapshot/{⬅️before,➡️after}/🔣️.json`. Observed domains include energy, puzzle, shooting, animation, writer, playbook, demonstrator and imperative. Representative fixtures alone cannot establish every-kind coverage; enumerate declared artifact contributions or document schemas to make missing support fail the audit.

Store owns a schema-keyed erased snapshot codec in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`. Its existing `document_codec_of_round_trips_dsl_and_pack_and_edit_text` test is in the adjacent unit test file around line 5878. It covers native pack/DSL mirrors and edit text and supplies an established generic codec integration pattern. Other nearby tests cover linked versus mounted component ownership and unowned schemas.

## Independent SQLite Oracles

`bun:sqlite` is already used in hub task scripts for independent SQLite assertions. Bun is installed at `/Users/ueli/.bun/bin/bun`. `/usr/bin/sqlite3` is available locally, but portable permanent verification should use Bun rather than assume a platform CLI.

Existing Rust bundled SQLite dependency is `rusqlite = 0.38.0`, used by hub and OS DB packages, with `bundled` enabled and `blob` enabled for OS DB. Framework itself currently has only async macros and base64 under dev-dependencies. Adding any oracle should remain test-only; repository-owned public interfaces must not expose external types.

OS DB SQLite storage in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs` persists snapshot generations, WAL and payloads in a live storage database. That storage schema alone is not evidence that standalone snapshot files import/export over IO.

## Task and Launch Infrastructure

`@semio-tech/framework-rs` is configured in `🧰️framework/📦️packages/🦀️rust/📋️project.json` and routes to its existing `📜️script.ts`. Generic framework tests use the `test` target. `test-core-modules` runs only hash, pixels, intrinsic-size and mesh-engine crates, so it does not include universal IO tests.

Dedicated new verification should be registered in the existing script router and Nx project targets, then added to `.vscode/launch.json` with adjacent framework Rust tests around lines 19863–19973. Use Bun and Nx as required. The script already provides budgeted test helpers and exact native-law execution patterns, including cancellation and progress reporting.

## Recommended Contract Coverage

Create a schema-first language-neutral fixture describing file identity/version, dialect or schema identity, exact owned snapshot bytes and expected rejection categories. Preserve bytes rather than reducing arbitrary snapshots to JSON; binary payloads, opaque extensions and future artifact kinds then remain representable.

1. Export through the public IO boundary, open exported bytes with Bun SQLite, query metadata and payload independently, and compare exact fixture output.
2. Produce a file independently with Bun SQLite, import it through the public IO boundary, compare the owned snapshot and identity, and re-export it for SQLite inspection.
3. Enumerate artifact kinds or document schemas and assert universal support without requiring individual plugins to register SQLite entries.
4. Cover empty payload, nested rich JSON, Unicode, null values, binary bytes and a large payload spanning multiple SQLite pages.
5. Reject corrupt/truncated headers or pages, unsupported version or identity, missing and duplicate required rows, foreign schema/dialect identity, wrong SQLite field types and absent payloads.
6. Confirm limits reject oversized input and pre-cancellation stops processing. If incremental IO supports cancellation/progress, exercise cancellation between pages and observable progress during a large round trip.
7. Keep live storage database export distinct from standalone artifact snapshot files; prove IO dispatch invokes the generic container route.

Both export and import interoperability are necessary: same-implementation round trips alone can conceal a shared file-format mistake. Byte equality and independent SQLite integrity/query checks provide meaningful verification beyond mirroring implementation.
