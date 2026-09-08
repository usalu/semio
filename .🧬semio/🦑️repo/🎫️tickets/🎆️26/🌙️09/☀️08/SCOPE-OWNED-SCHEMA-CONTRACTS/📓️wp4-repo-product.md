# WP4 — `🦑️repo` product, `📓️print` and `♻️mit-bestand`

Worker partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/**` except `📚️library/**` and `🧪️test/**`,
plus `🧰️framework/🛍️products/📓️print/**` and `♻️mit-bestand/**` (submodule `♻️mit-bestand/🔎️recherche`
untouched — verified with `git status --porcelain` returning nothing for it).

Implemented against `📋️execution-contract.md` (§A identity, §B layout, §E rules).
Repo MCP was down for the whole session; no MCP tool was called, no ticket was opened/closed/reopened,
`🗑️generated/` was not deleted. Build/test logs went to the session scratchpad; real output is pasted below.

---

## 1. New scopes and their exports

| Scope id | Module path | `$id` | Formats |
|---|---|---|---|
| `repo.server.coordinator` | `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/` | `https://semio.tech/schema/repo/server/coordinator/schema.json` | `🔣️.json` (draft-07), `🟦️.ts` |
| `repo.server` | `🦑️repo/🔨️modules/🖥️server/🧬️schema/` | `https://semio.tech/schema/repo/server/schema.json` | `🔣️.json` (draft-07), native `🐘️postgres/🗄️.sql` (pre-existing, kept) |
| `print.print-font-catalog` | `📓️print/🔨️modules/🔤print-font-catalog/🧬️schema/` | `https://semio.tech/schema/print/print-font-catalog/schema.json` | `🔣️.json` |
| `print.tectonic-template-compilation.catalog` | `…/🖨️tectonic-template-compilation/📇️catalog/🧬️schema/` | `https://semio.tech/schema/print/tectonic-template-compilation/catalog/schema.json` | `🔣️.json` |
| `print.tectonic-template-compilation.bundle` | `…/🖨️tectonic-template-compilation/📚️bundle/🧬️schema/` | `https://semio.tech/schema/print/tectonic-template-compilation/bundle/schema.json` | `🔣️.json` |
| `print.tectonic-template-compilation.toolchain` | `…/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema/` | `https://semio.tech/schema/print/tectonic-template-compilation/toolchain/schema.json` | `🔣️.json` |
| `mit-bestand.bericht.documents` | `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema/` | `https://semio.tech/schema/mit-bestand/bericht/documents/schema.json` | `🔣️.json` |
| `mit-bestand.demonstrator.runtime` | `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema/` | `https://semio.tech/schema/mit-bestand/demonstrator/runtime/schema.json` | `🔣️.json` |

### 1.1 `repo.server.coordinator` exports (39 `$defs`, one `parse<Export>` each)

Shared shapes: `Timestamp`, `NullableTimestamp`, `NullableString`, `NullableInteger`, `EmailAddress`,
`DeveloperRole`.

| Endpoint | Request exports | Response exports |
|---|---|---|
| `GET /api/v1/auth` | — | `AuthWhoAmIResponse` |
| `POST /api/v1/auth` | `CreateDeveloperRequest`, `CreateKeyRequest`, `RevokeKeyRequest` | `Developer`, `DeveloperApiKey`, `CreateKeyResponse`, `StatusResponse` |
| `POST /api/v1/diff` | `DiffIngestRequest`, `FileSnapshot` | `DiffIngestResponse` |
| `GET/POST /api/v1/event` | `EventPublishRequest` | `EventPublishResponse`, `Event`, `EventListResponse` |
| `POST /api/v1/repo` | `IndexFileRequest`, `ReindexRequest` | `StatusResponse`, `ReindexResponse` |
| `GET/POST /api/v1/ticket` | `TicketOpenRequest`, `TicketCloseRequest`, `TicketReopenRequest` | `Ticket`, `TicketListResponse`, `TicketDetailResponse` |
| `GET /api/v1/scope`, `/warning`, `/breach` | — | `Scope`/`ScopeListResponse`, `Warning`/`WarningListResponse`, `Breach`/`BreachListResponse` |
| every endpoint | — | `ErrorResponse` |
| coordinator event log | `G3EventEnvelope` | `G3EventLogContract` |

`🧬️schema/🟦️.ts` also exports `COORDINATOR_SCHEMA_ID`, `ParseResult<T>`, `parseCommandAction()` (the
`action` discriminator used by the auth/repo/ticket dispatch switches) and `COORDINATOR_PARSERS`
(export id → parser), which the tests assert is key-for-key equal to `$defs`.

### 1.2 `repo.server` exports (66 `$defs`)

`Timestamp`, `Date`, `JsonDocument` plus one `<Table>Row` export for each of the **63** tables in
`🧬️schema/🐘️postgres/🗄️.sql` (`DevelopersRow`, `TicketsRow`, `ScopesRow`, `EventsRow`, `KitsRow`,
`KitSnapshotLayoutPiecesRow`, …). Column type, `CHECK (x IN (…))` enum and NOT NULL/PRIMARY KEY
nullability are all projected; the SQL stays the native implementation and the JSON Schema is the
declared contract.

---

## 2. Files created / moved / deleted

### Created

- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🔣️.json`
- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema/🟦️.ts`
- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧫️fixtures/📨️rest-cases.json` (26 accepted + 22 rejected cases, data only)
- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧬️schema_contract_test.go`
- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️schema.test.ts`
- `🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts`
- `🦑️repo/🔨️modules/🖥️server/🧬️schema/🔣️.json`

Authoring aids kept in this ticket folder (not in the repo tree, nothing depends on them at runtime):
`wp4-repo-postgres-row-schema.py` (derived the 63 `<Table>Row` exports from `🗄️.sql` once) and
`wp4-repo-flat-schema-to-module.py` (the one-off flat → module conversion of §2). Both outputs are now
hand-owned files; the parity is re-derived at test time, not by re-running these.

### Moved (flat `🧬️schema.json` → `🧬️schema/🔣️.json`, 2020-12 → draft-07, `$id` added, root `$ref` + `$defs`)

| From | To | Exports |
|---|---|---|
| `📓️print/🔨️modules/🔤print-font-catalog/🧬️schema.json` | `…/🔤print-font-catalog/🧬️schema/🔣️.json` | `PrintFontCatalog`, `PrintFontDescriptor` |
| `…/🖨️tectonic-template-compilation/📇️catalog/🧬️schema.json` | `…/📇️catalog/🧬️schema/🔣️.json` | `PrintDocumentCatalog`, `PrintDocumentEntry` |
| `…/🖨️tectonic-template-compilation/📚️bundle/🧬️schema.json` | `…/📚️bundle/🧬️schema/🔣️.json` | `PrintBundleManifest`, `PrintBundleFile` |
| `…/🖨️tectonic-template-compilation/🔧️toolchain/🧬️schema.json` | `…/🔧️toolchain/🧬️schema/🔣️.json` | `PrintToolchainManifest`, `PrintToolchainPlatform` |
| `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema.json` | `…/📄️documents/🧬️schema/🔣️.json` | `ReportDocumentCatalog`, `ReportDocumentEntry` |
| `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema.json` | `…/🧩️runtime/🧬️schema/🔣️.json` | `DemonstratorRuntimeCatalog`, `DemonstratorRuntimePane` |
| `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️pipeline.schema.json` | folded into the same module as `$defs.DemonstratorPipelineContract` | `DemonstratorPipelineContract` |

### Deleted (no aliases, redirects or wrappers left)

- `🎛️coordinator/📦️packages/🟦️typescript/✅️validation.ts` — the 90-line zod-shaped `ownedSchema`
  combinator library. Superseded outright by the module's parsers (deletion, not a wrapper, per the brief).
- `🎛️coordinator/📦️packages/🟦️typescript/🔬️validation.test.ts` — its 7 assertions are subsumed by the
  47 fixture cases in `🔬️schema.test.ts`.
- `🎛️coordinator/🧫️fixtures/🧬️g3-event-schema.json` — the fixture-local log contract; its five values are
  now `const`s under `$defs.G3EventLogContract` in the owner module, and the event data
  (`📜️g3-event-log.jsonl`) stayed as fixture data.
- the six flat `🧬️schema.json` files and `🧬️pipeline.schema.json` listed above.

---

## 3. Consumer rewiring

| Consumer | Change |
|---|---|
| `🎛️coordinator/…/app/api/v1/auth/route.ts` | `ownedSchema as z` → `parseCommandAction`, `parseCreateDeveloperRequest`, `parseCreateKeyRequest`, `parseRevokeKeyRequest`; three inline `z.object` schemas removed |
| `…/app/api/v1/diff/route.ts` | → `parseDiffIngestRequest` |
| `…/app/api/v1/event/route.ts` | → `parseEventPublishRequest` |
| `…/app/api/v1/repo/route.ts` | → `parseCommandAction`, `parseIndexFileRequest`, `parseReindexRequest` (the `reindex` branch was previously unvalidated and now parses its body) |
| `…/app/api/v1/ticket/route.ts` | → `parseCommandAction`, `parseTicketOpenRequest`, `parseTicketCloseRequest`, `parseTicketReopenRequest`; header comment corrected from `/api/v1/tickets` to the real `/api/v1/ticket` |
| `🎛️coordinator/🧪️g3_event_store_test.go:103` | reads `🧬️schema/🔣️.json` through the new `loadG3EventLogContract(t)` instead of the deleted fixture schema; also asserts the `scalar` const, which the old test ignored |
| `🎛️coordinator/🧩️component.go` | Go models brought field-for-field to the schema (below) |
| `💻️client/⌨️cli/🧩️component.go` `syncTicketToServer` | posted to the non-existent `/api/v1/tickets` with one payload for every action, including `summary` and `session_id` which no endpoint accepts. Now posts to `/api/v1/ticket` and builds the exact per-action contract body (`open` / `close` / `reopen`), with `parent` as a real nullable |
| `📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts` | three `ajv/dist/2020` call sites → draft-07 `ajv`, all reading `🧬️schema/🔣️.json` through a new `printSchemaModule()` helper that asserts dialect + `$id`; new exported `verifyPrintToolchainManifest()` wired into `verifyPrintPipelineQuick()` |
| `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts` | `ajv/dist/2020` → draft-07 `ajv`, reads `🧬️schema/🔣️.json` |
| `♻️mit-bestand/🧺️demonstrator/📜️script.ts` | new in-source vitest case validating `🔣️.json` and `🧫️pipeline.json` against the owner module (both had no validating consumer inside their own product) |

### 3.1 Go ↔ JSON Schema field parity (`🧩️component.go`)

| Type | Change | Why |
|---|---|---|
| `Ticket` | dropped `Emoji`, added `Goal string \`json:"goal"\`` and `Parent *string \`json:"parent"\`` | `emoji` exists in no `tickets` column, no TS type and no consumer; `goal`/`parent` are real columns the Go server silently dropped |
| `Breach` | `Column *int \`json:"column"\`` → `Col *int \`json:"col"\`` | the persisted column and the TS type are `col` |
| `Event` | `Type string \`json:"type"\`` → `Kind string \`json:"kind"\`` (2 call sites updated) | the `events` table and the TS type use `kind` |
| `TicketOpenRequest` | added `Action`, `Goal`, `Parent` | `action` is on the wire for `/api/v1/ticket`, and Go's `DisallowUnknownFields()` decoder rejected the canonical body outright |
| `TicketCloseRequest` | added `Action` | same |
| `TicketReopenRequest` | added `Action`, `Client` | same; `client` was accepted by the Next route only |
| `IndexFileRequest` | added `Action` | same |
| `handleTicketOpen` / `handleTicketReopen` | now carry `goal`/`parent` and `client` into the ticket | |

`🧬️schema_contract_test.go` enforces this: for 13 Go types it compares the sorted `json` tag set against
the sorted `$defs.<Export>.properties` keys, requires every mapped export to be `additionalProperties:
false` (matching the decoder's strictness), and decodes the shared fixtures with `DisallowUnknownFields()`.
`EventPublishRequest` is checked against `repopkg.Event` from `📚️library` (read-only, no edit).

`ReindexRequest`, `ReindexResponse`, `StatusResponse`, `ErrorResponse` and the auth exports have no Go
counterpart (the Go server is token-authenticated and path-dispatched); they are TS-only and are excluded
from the parity map by construction, not by a skip.

---

## 4. Deliberate behaviour changes

1. **Requests are now strict.** Every request export is `additionalProperties: false` and the parsers
   reject unknown properties. The previous `ownedObject` silently stripped them. The Go decoder was
   already strict (`decoder.DisallowUnknownFields()`), so this makes the two implementations agree
   instead of diverging. No compatibility path was left.
2. **`action` is part of the payload contract**, not a route-local pre-check. `parseCommandAction` reads it
   for dispatch and each command parser then re-validates it as a `const`.
3. **The `reindex` body is validated** where it previously was not.

---

## 5. Duplicate `$id` / unresolved `$ref` (task 4)

`📊️wp0-inline-and-generation.json` `refGraph` filtered to this partition:

- `unresolvedLikelyBroken` (134 entries) — **0** under `🛍️products/🦑️repo/`, `🛍️products/📓️print/` or `♻️mit-bestand/`.
- `unresolvedAmbiguousIdStyle` (80) — **0** in this partition.
- `crossScope` — empty list repo-wide.
- `duplicateIds` (19 groups) — **0** group contains a path in this partition (all 19 are `✏️s/🔌️plugins/…` artifact standard subsets).

So there was nothing to repair here. Every `$id` this WP introduced is new and unique
(`https://semio.tech/schema/repo/server/…`, `…/print/…`, `…/mit-bestand/…`); all `$ref`s in the eight
modules are local `#/$defs/…` pointers and resolve — proven by ajv compiling every one of them in §6.

---

## 6. Verification (real output)

### 6.1 Coordinator Go suite — `go test -count=1 ./...`

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/server	13.337s
```

Named runs of the moved/new contract tests:

```
$ go test -v -run 'TestGoTypesMirrorOwnerSchemaExports|TestCoordinatorDecodesOwnerFixtures|TestG3LanguageNeutral' .
=== RUN   TestG3LanguageNeutralGoldenEnvelope
--- PASS: TestG3LanguageNeutralGoldenEnvelope (0.09s)
=== RUN   TestGoTypesMirrorOwnerSchemaExports
--- PASS: TestGoTypesMirrorOwnerSchemaExports (0.00s)
=== RUN   TestCoordinatorDecodesOwnerFixtures
--- PASS: TestCoordinatorDecodesOwnerFixtures (0.01s)
PASS
ok  	github.com/usalu/semio/repo/server	0.421s
```

`go vet ./...` in the coordinator: no output (clean).

### 6.2 Coordinator vitest (own parsers vs. the ajv draft-07 oracle, and SQL↔JSON parity)

```
$ cd .../🎛️coordinator/📦️packages/🟦️typescript
$ node node_modules/vitest/vitest.mjs run --config 🧪️tests/🟦️.ts
 RUN  v4.1.10 .../🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript

 Test Files  2 passed (2)
      Tests  118 passed (118)
   Start at  16:45:59
   Duration  6.08s (transform 724ms, setup 0ms, import 1.56s, tests 4.49s, environment 0ms)
```

Baseline before the change was `1 passed (1) / 7 passed (7)`. The 118 are:
26 accepted + 22 rejected fixture cases (each asserted against **both** our parser and ajv 8.20.0 with
`useDefaults`, deep-comparing the defaulted result), 2 event-log cases, 2 module-identity cases,
3 persistence-module cases and 63 per-table column-parity cases.

### 6.3 Print pipeline schema checks

```
$ bun <scratchpad>/print_check.ts
[DEBUG] Print toolchain manifest schema and exclusive platform targets PASS
[DEBUG] Print catalog schema, independent gallery identities and 87 exclusive Nx document owners PASS
[print-bundle] Acquiring 486 verified TeX support files
[print-bundle] Acquiring 486 verified TeX support files
[print-bundle] Acquiring 486 verified TeX support files
[print-bundle] Acquiring 486 verified TeX support files
[DEBUG] Print bundle schema, pre-cancel and four corrupt-acquisition vectors PASS
[DEBUG] Print fonts: schema, native loading, byte identity and atomic replacement PASS
PRINT-SCHEMA-CHECKS-OK
```

(the scratchpad entry point only calls the four exported verifiers; the `[DEBUG]` lines are the file's own
pre-existing convention.)

### 6.4 `♻️mit-bestand/📋️bericht`

```
$ cd ♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
[DEBUG] Report command 🔨️modules/📄️documents/📜️script.ts: 13 production source files verified
[DEBUG] Report command 🔨️modules/👥️actor-network/📜️script.ts: 3 production source files verified
[DEBUG] Report catalog, Nx output ownership and generator contract verified
[DEBUG] report zwischenbericht: 20 source documents resolved
[DEBUG] report forschungsbericht: 10 source documents resolved
[DEBUG] report kompaktbericht: 1 source documents resolved
[DEBUG] Akteursnetz valid: 798 nodes, 444 edges, 14 programs
```

### 6.5 `♻️mit-bestand/🧺️demonstrator`

```
$ cd ♻️mit-bestand/🧺️demonstrator && node node_modules/vitest/vitest.mjs run --config ⚡️vitest.config.ts
 RUN  v4.1.10 .../♻️mit-bestand/🧺️demonstrator

 Test Files  2 passed (2)
      Tests  5 passed (5)
   Start at  16:23:39
   Duration  22.62s (transform 10.77s, setup 0ms, import 24.53s, tests 1.11s, environment 0ms)
```

An earlier run of this same suite failed with
`Error: Cannot find module '../📃️pageSchema/🟦️.ts' imported from 🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts`.
That is a concurrent peer's in-flight rename in `🎭️actor` (another partition — `📃️page` exists,
`📃️pageSchema` does not); the run above, minutes later, is green with the same source of mine.

### 6.6 Residual flat schemas in the partition

```
$ git ls-files '🧰️framework/🛍️products/🦑️repo' | grep -E 'schema\.json$'
… 24 paths, every one under 🔨️modules/📚️library/ …
$ find 🧰️framework/🛍️products/📓️print ♻️mit-bestand -name '🧬️schema.json' -not -path '*/node_modules/*'
(no output)
```

All 24 remaining `*schema.json` paths under `🦑️repo` are inside `📚️library/**` (tooling worker) — none
under `🧪️test/**` either. Nothing is left in this partition. A filesystem `find` over the same tree agrees
with the index listing.

### 6.7 Not verifiable in this session

- `npx nx run @semio-tech/repo-coordinator:test` never produced a result:
  `NX   The daemon timed out while processing REQUEST_PROJECT_GRAPH` after >10 min. Every suite above was
  therefore run through the same binary the nx target invokes
  (`node node_modules/vitest/vitest.mjs run --config 🧪️tests/🟦️.ts`, i.e. `runVitest`'s exact argv), not
  through nx. No `project.json`/`package.json`/`launch.json` entry was added or changed — the existing
  `test` targets already cover the new test files.
- `💻️client/⌨️cli`: `go build .` and `go vet .` are clean, and
  `go test -run 'TestTicket|TestSyncTicket|TestWriteTicket|TestG1' -timeout 420s .` → `ok … 18.616s`.
  A full `go test .` **fails by test-binary timeout** (`FAIL … 601.093s`, panic stack inside
  `CodebaseContext.LoadFiles` → `ScopeToFiles` under `TestExhaustiveFoldersNonEmpty` /
  `TestExhaustiveNormalizeTicketFileInput`). Those tests walk the whole 61k-file repo and exceed the
  10-minute go-test budget on this box under concurrent load; they do not touch `syncTicketToServer`
  (which returns immediately when `getServerAddr()` is empty, as in tests). I did not confirm a
  pre-change baseline for them, so treat this as unresolved-but-unrelated rather than proven green.

---

## 7. Cross-partition requests

Filed as rows 14–17 of `📋️cross-partition-requests.md`. Two consumers of files I deleted/moved live in
`📚️library/**` (tooling worker) and are currently broken; they need these exact edits:

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts`,
   `testDemonstratorRuntime()` (~line 87):
   ```
   -  assert.ok(new (require("ajv/dist/2020").default)().validate(JSON.parse(readFileSync(join(directory, "🧬️schema.json"), "utf8")), catalog));
   +  const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
   +  assert.ok(new (require("ajv").default)({ strict: false }).validate(schema, catalog));
   ```
   and anywhere in the same file that reads
   `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️pipeline.schema.json`, replace it with
   `…/🧩️runtime/🧬️schema/🔣️.json` compiled at `$ref: "#/$defs/DemonstratorPipelineContract"`
   (an `$id` must be varied when compiling the same document twice under one ajv instance, e.g.
   `{ ...schema, $id: schema.$id + "#pipeline", $ref: "#/$defs/DemonstratorPipelineContract" }` —
   that is exactly what `♻️mit-bestand/🧺️demonstrator/📜️script.ts` now does).
   Note the dialect change: these documents are **draft-07** now, so `ajv/dist/2020` must become `ajv`.

2. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`
   still carries rows (with content hashes) for two deleted files:
   - line ~5102/5110: `…/🎛️coordinator/📦️packages/🟦️typescript/🔬️validation.test.ts`
   - line ~5196/5204: `…/🎛️coordinator/📦️packages/🟦️typescript/✅️validation.ts`
   Both rows must be removed. If the authority is meant to be exhaustive over the package, add rows for
   `🔬️schema.test.ts` and `🔬️server-persistence.test.ts` (both `implementation`/`authored`, owner
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator`).

3. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🐹️.go:345` posts events to
   `"/api/v1/events"`. The Next route is `app/api/v1/event/route.ts`, i.e. **`/api/v1/event`** (singular).
   This emitter has never reached the coordinator. It should read
   `url = strings.TrimSuffix(url, "/") + "/api/v1/event"`. Its body (`{kind, source, payload}` from
   `repopkg.Event`) already matches `$defs.EventPublishRequest` field-for-field — asserted by
   `TestGoTypesMirrorOwnerSchemaExports`.

4. **Taxonomy `schemaScopeOwnerLevels` must declare `♻️mit-bestand/<product>/🔨️modules/<m>`.**
   The wave-2 checker snapshot `🗑️generated/schema-check.jsonl` (which already sees my conversions) has
   exactly three `module-level-ineligible` rows touching this partition, two of them mine:
   ```
   {"code":"module-level-ineligible","path":"♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🧬️schema",
    "detail":"♻️mit-bestand/📋️bericht/🔨️modules/📄️documents is not a declared schemaScopeOwnerLevels level."}
   {"code":"module-level-ineligible","path":"♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧬️schema",
    "detail":"♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime is not a declared schemaScopeOwnerLevels level."}
   ```
   Contract §A lists `🧰️framework/🛍️products/<p>/🔨️modules/<m>` but `♻️mit-bestand` products sit at the
   repo root with the identical shape. The third row
   (`🦑️repo/🧪️tests/🧪️transaction-process-ownership/🧬️schema`) is a `🧪️*` directory and belongs to the
   harness worker. **No other check code in that file names a path in this partition** — in particular no
   `document-dialect-unexpected`, `document-id-missing`, `module-scope-id-inconsistent` or
   `scope-id-duplicate` row, so ledger request #12 is already clear for me.
5. For WP2's derived catalog
   (`📚️library/🔣️schema-catalog.json`): the eight scopes of §1 with their `$id`s and export lists are
   ready to be indexed. `repo.server.coordinator` `dependsOn` nothing; `repo.server` `dependsOn` nothing.

---

## 8. Open questions / findings not acted on

1. **`🦑️repo/🔨️modules/💻️client/🪶️sqlite` holds two competing SQLite schemas outside any `🧬️schema/`
   module** and neither is referenced by any code: `📐️schema.sql` (244 lines, 24 tables — `agent`,
   `contributor`, `session`, `system`, `technology`, `release`, … ) and `🗄️.sql` (82 lines, 7 tables —
   `bundle`, `definition`, `file`, `folder`, `repo`, `section`, `technology`). They are not two formats of
   one contract; they are two different schemas. Relocating them into
   `💻️client/🪶️sqlite/🧬️schema/` requires first deciding which is authoritative (or that both are dead),
   which is a content decision, not a layout one. I left both files untouched rather than delete content on
   a guess. This is the only remaining alternative-format schema outside a `🧬️schema/` module in my
   partition.
2. **`@/lib` is broken in the coordinator's `tsconfig.json`**: `"@/lib": ["../lib/index.ts"]` resolves to
   `📦️packages/lib/index.ts`, which does not exist. Every route imports `@/lib`, so the Next app has never
   type-checked. The real library is
   `🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts` (→ `["../../../📚️library/📦️packages/🟦️typescript/🟦️.ts"]`).
   Left as-is because it is orthogonal to schema ownership and touching module resolution mid-ticket would
   mix concerns; the new `🧬️schema/🟦️.ts` deliberately imports nothing, so it type-checks standalone
   (`tsc --noEmit --strict` on it reports only pre-existing `@types/mdx` JSX namespace errors).
3. **The `repo.server` persistence tests are hosted in the coordinator's vitest project.**
   `🖥️server/📚️library/📦️packages/🟦️typescript/` has the package layout but no `package.json`,
   `📋️project.json`, `📜️script.ts` or `🧪️tests/🟦️.ts`, so it is not an nx project and has no test runner;
   the coordinator package is the only test host under `🖥️server`. Registering a project for the server
   library (and moving `🔬️server-persistence.test.ts` there) is the clean long-term home, but it adds an nx
   project and would collide with the harness worker's concurrent `🧪️test/**` and taxonomy work.
   Recommend doing it as a follow-up once WP2's taxonomy additions land.
4. **`🎛️coordinator` ships two server implementations** — the Go binary (`🧩️component.go`, path-dispatched
   `/ticket/open`, `/events`, built by the `build` target) and the Next app (`/api/v1/*`, action-dispatched).
   The schema module is now the single contract both implement, and the Go structs were aligned to it, but
   the *route shapes* still differ. Whether one should be retired is a product decision outside this ticket.
5. **`🧫️invocations.json`** in `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/` has no schema and no
   consumer I could find. Left alone; flagged for whoever owns the demonstrator fixture sweep.
