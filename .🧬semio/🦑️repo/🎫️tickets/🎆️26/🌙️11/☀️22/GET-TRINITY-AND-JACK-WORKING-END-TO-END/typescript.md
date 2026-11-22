# Trinity and Jack TypeScript side on Windows

## Result

Natively on Windows (bun 1.4.2, vitest 4.1.12) every TypeScript suite of the four projects passes, and both published-consumer typechecks (`verify snapshot-sqlite-source` equivalent) are clean.
`repo run <project>:<target>` could not be used: the graph service queued for more than 30 minutes, then failed with `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond is required` (host log host-0d7c94a3618e.log; the key exists in `policy.json` now) and later with `ENOENT lstat` on files replaced under it by the fleet. These are repo-product host failures, not trinity failures. The `📜️script.ts` entries need the repo-issued process envelope, so they cannot be run with bare bun either.

| Project | Natively run | Result |
|---|---|---|
| `@semio-tech/trinity-jack` test | 10 bun suite files (SQLite, wire-types, query inference, binary receiving, mutate-jack-1, manifest-id-only, document-contract, executor transport and resumable query, graph window config) | 56 pass, 0 fail |
| `@semio-tech/trinity-jack` verify-snapshot-sqlite-source | tsc over 44 non-test published files, strict | status 0 |
| `@semio-tech/trinity-rewriting` test | 7 suite files (SQLite snapshot, contract, retirement, map ownership, document contract, binary64 transport, window config) | 62 pass, 0 fail |
| `@semio-tech/trinity-rewriting` verify-snapshot-sqlite-source | tsc over 62 files, strict | status 0 |
| `@semio-tech/trinity-js` test | the 4 demo and demo-session example tests of jack and rewriting | 4 pass, 0 fail |
| `@semio-tech/trinity-jack-lsp-worker` test | vitest, 4 files | 31 pass, 0 fail |
| `✏️s/🧪️tests/🏘️plugin-composition/🔱️trinity-graph-child` | 1 bun file | 6 pass, 0 fail |
| `🎮️commands`, `🏭️bridge` | commands holds only `📌️.empty.md`; the bridge script only forwards to the Rust binary | not runnable without the Rust build (Rust agent) |

`build` and `check` of jack and rewriting were not runnable natively (they run only through the repo envelope); the closest equivalents (typecheck of all published sources, all suites) are the rows above.

## Root causes and fixes

1. Windows URL bug: `new URL(absolutePath, import.meta.url)` on `C:\...` parses as scheme `c:` and throws "URL must be a non-empty file: path". The Jack sqlite test fed `frameworkSourcePath(...)` into such a reader; the Rewriting sqlite test did the same for GraphQL and proto roots. Fixed with the existing `readFrameworkSourceJson` and `frameworkSourceUrl` helpers.
2. Windows path separators: recursive `readdirSync` returns backslash paths, so `endsWith("/📸️snapshot/...")` filters matched nothing (roster expected 9, got 0). Normalized to `/` in the Rewriting sqlite test (7 sites) and in the trinity-graph-child composition test.
3. Stale fixture reference: `trinity-graph-child` read `🪆️child/🏢️initial/🪆️content/🔣️.json`, a file that is not in git and no longer exists; the child is inline in `initial.child`, which the test already asserts in detail. Dropped that comparison.
4. Timeouts under load: the wire-types declaration-compiler test (TS program over the owner, 5 to 17 s) now has an explicit 120 s timeout; the LSP worker `beforeEach` (cold dynamic import took more than 10 s once) has a 60 s hook timeout.
5. Concurrent schema change by another agent (Jack artifact now holds only `manifestId`, no inline `manifest`; Jack SQLite is 3 tables; manifest interned by id, `🛂️manifest.json` is the nakagin source). The Rewriting side embedded the old shape and failed 44 of 50 tests. Followed through at the root:
   - removed the inline `manifest` from all 27 Jack parents in 25 Rewriting JSON fixtures and from the two `edit-working-graph` rows of `🥒️.feature`; added `manifestId` ("nakagin", `null` in diffs) where a parent had none;
   - Rewriting `🗄️.sql` is now its 17 own tables plus the Jack `🗄️.sql`; `REWRITING_SQLITE_SCHEMA` is its own tables plus the imported `JACK_SQLITE_SCHEMA` (no second copy of Jack tables); table count 30 to 20;
   - fixtures: ten removed jack tables dropped from `tableWidths` and `tableRowCounts`; unused `manifestOracle` removed; retirement byte budgets 59, 113, 61 raised by 7 (the `manifestId` string) to 66, 120, 68, to be matched by the Rust twin, which reads the same fixture;
   - tests: inline-manifest value-type test replaced by a manifest-id-only test; GraphQL body test uses the id-only parent; row frontier 89 to 55; contract test inserts `manifest_id` and no manifest rows; retirement oracle asserts no inline manifest; Python oracle of `mutate-rewrite-1` requires `manifestId`, no `manifest`;
   - `s.` prefix drift: the child schema envelope is `stdio.semio.graph` while the artifact kind is `s.stdio.semio`; the composed-slot assertion now compares `"s."+child.schema`.
6. Odd file `🛂️manifest.jsonnakagin.manifest.json`: nothing referenced the exact name (searched `✏️s`, framework, teaching, tests); content was the legacy shape (`id`/`ports`/`presentation`), no other copy existed. I deleted it, and the same accidental name in rewriting (`🛂️manifest.jsonrewrite-lhs.manifest.json`). Meanwhile the generator or another agent produced the correct `🛂️manifest.json` in both artifacts (28807 bytes, same content as the stray file), which now feeds `🤖️generated/🏢️nakagin`.

## Files changed by me

Jack
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🛂️manifest.jsonnakagin.manifest.json` (deleted)
- `…/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `…/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🧪️tests/📦️wire-types/🟦️.ts`
- `…/🔌️jack/🔨️modules/🧠️lsp/🧪️tests/📡️worker/🟦️.ts`

Rewriting (`…/🗿️artifacts/♻️rewriting`)
- `🛂️manifest.jsonrewrite-lhs.manifest.json` (deleted)
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts`, `🗄️.sql`
- `…/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`, `📋️contract/🟦️.ts`, `🧫️fixtures/🔣️.json`, `🧫️fixtures/🌳️typed/🔣️.json`
- `…/🧬️schema/♻️retirement/🧪️tests/🔬️document-retirement/🟦️.ts`
- `…/🧪️tests/♻️mutate-rewrite-1/🥒️.feature`, `🐍️.py`
- `…/🧪️tests/🧫️fixtures/♻️retirement/🔣️.json`, `♻️mutate-rewrite-1/🔣️.snapshot.json`, `♻️mutate-rewrite-1/🏢️nakagin-ground-floor/🔣️.snapshot.json`
- `…/🧪️tests/🧫️fixtures/🧬️mutations/**` snapshot, diff and mutation JSON of edit-lhs, edit-rhs, set-rule-layout-points, change-rule-layout-point, change-parameter-binding, edit-working-graph, remove-rule-layout-point, remove-parameter-binding, drag-rule-nodes (22 files)

Composition
- `✏️s/🧪️tests/🏘️plugin-composition/🔱️trinity-graph-child/🟦️.ts`

Ticket helper scripts (kept): `typescript-verify.ts`, `sweep-manifest.ts`, `add-manifest-id.ts`, `compose-rewriting-schema.ts`, `drop-jack-tables.ts`, `replace-inline-manifest-test.ts`, `simplify-graphql-parent.ts`, `strip-inline-manifest-text.ts`, `firstdiff.ts`. Their logs were deleted.

## Still open

- `repo run` of any target is blocked by the repo host (see Result); rerun `repo run @semio-tech/trinity-jack:build`, `:check`, `:test`, `:verify-snapshot-sqlite-source` and the same for rewriting, trinity-js and the lsp worker once the host is healthy.
- Rust twins must match: Rewriting `🗄️.sql` (20 tables), retirement byte budgets 66, 120, 68, id-only fixtures; Rewriting Rust sources still mention manifest in 40 places (Rust agent).
- C: was at 98 percent; the repo gate freed caches repeatedly. Low disk can fail builds.
