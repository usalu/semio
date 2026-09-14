# 📓️ `🔗️graphql` — aligning the Go executor with the frozen Rust contract

Executor: Opus 5 (`go-align-graphql`).
Owned: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go`, the four new
`🧪️tests/*/🐹️.go` adapters, and the `model.RepoContext` port declaration.

## 1. What the gap actually was

`📓️opus-graphql-executor.md` §5 recorded it correctly but understated it. The Go package did not
merely bypass the injected `RepoContext` in three resolvers: it was a **different executor over a
different source model**. Rust resolves a selection set against pre-derived JSON *source objects*
(`🖼️Sources`) with a `Field.resolver` key table; Go resolved against Go structs by reflection, with
2 760 lines of closure-carrying `buildSchema`, package-level streamers (`ticketspkg.StreamTickets`,
`StreamBundles`, `contributorspkg.StreamContributors`, `statutespkg.StreamPolicies`,
`ticketspkg.ListInteractions`), and identity builders (`model.BuildFolderID`, `BuildFileID`) that
recomputed ids the record already carried. Retrofitting `r.Ctx` into three call sites would have
moved the measured difference count from 24 071 to something still in the thousands, because every
derived field (`Ticket.dates`, `Ticket.id`, `Bundle.id`, `Statute.id`, `Breach.kind` overlay,
`Contributor.links`, `Section.children` ordering, …) is computed differently on the two sides.

So the Go half was **ported to the Rust design**, region for region, rather than patched.

## 2. What was replaced in `📦️packages/🐹️go/🐹️.go`

| region | before | after |
| --- | --- | --- |
| `📜️Schema` | `Object`/`Fields map[string]*Field` with resolver closures, `Interfaces` dropped | `TypeRef`/`Field{Name,Type,Args,Resolver}`/`ObjectType{Interfaces,Fields []Field}`/`Schema{Types,Query,Mutation}` — ordered, cycle-safe by name, mirroring Rust |
| `⚡️Execution` | map-driven, no argument coercion, non-deterministic enum reverse lookup | `execution` struct bound to schema + `model.RepoContext`, `coerceInput`, `resolveDefault`, `project`, `concrete` via `__typename`, deterministic enum order |
| `🐹️Schema Builder` (2 760 lines) | one god function | 22 per-aggregate `🏗️Schema*` fragments + `BuildSchema()` |
| `🗂️/💻️/🧪️ resolvers` + `🗼️Resolver Interfaces` + `📜️Resolver Methods` + `⚙️Types` | `queryResolver`/`mutationResolver`/`repoResolver` reading the filesystem | one `dispatch(key, source, args)` over 60 resolver keys, all reads through `run.repo` |
| — (new) | — | `🖼️Sources`: `technologySource`, `bundleSource`, `folderSource`, `fileSource`, `sectionSource`, `definitionSource`, `statuteSource`, `breachSource`, `territorySource`, `policySource`, `interactionSource`, `interactionResourceSource`, `ticketSource`, `goalSource`, `draftSource`, `todoSource`, `checkpointSource`, `contributorSource`, `analyzeResultSource`, `fixResultSource` |
| — (new) | — | `📜️Sdl`: `reachable`, `RenderSDL`, `SchemaInventory` — the Rust `render_sdl` / `schema_inventory` byte-for-byte |
| — (new) | — | `🩻️RecordingContext`: `NewRecordingContext([]byte)`, `NewRecordingContextFromText`, `Events()`, `Snapshot()`, all 49 port methods |
| `🧱️GraphQL Executor` | no schema accessor, `%v` error envelope | `Executor.Schema() Schema`, `graphql errors: [<message>]` envelope |

Dead code removed with the old layer: `parseFileListInput`, the whole `⚙️Types` region
(`ResolveContributorContributions`), and a duplicated `#region 🩻️Default Context` header.
`Gql`, `PrintGQL`, `NewExecutor`, `NewExecutorWithContext`, `NewRepoContext`, `RepoResolverInstance`,
`UpdateGoalTitle`, `Do(Params{…})` and `buildSchema(resolver)` kept their signatures, so
`⌨️cli` and the package's own 2 089-line test suite compile unchanged.

## 3. Port change in `📐️model`

`model.RepoContext` gained one method — `GetInteractions() ([]InteractionResource, error)` — the
48-method Go port was missing the 49th Rust method (`interactions`). Implemented by
`graphql.RepoContext` (via `ticketspkg.ListInteractions`), `graphql.defaultContext` (empty) and
`graphql.RecordingContext`. One test double outside my ownership had to follow:
`⌨️cli/📦️packages/🐹️go/🔬️_test.go` `testExportContext` (four lines).

## 4. Three real defects the parity run exposed, and where they live

1. **`model.Interaction` has a pointer receiver `UnmarshalJSON`, and `model.InteractionResource`
   embeds `Interaction` by value.** Go promotes the method, so decoding an `InteractionResource`
   silently drops `sourceKind`, `sourceId`, `goalId` and `ticketId` — six differences on
   `▶️query-execution::interactions`. Rust has no such promotion. Worked around **inside my own
   `RecordingContext`** by decoding the resource fields from a second, embedding-free struct. The
   underlying hazard is still in `📐️model` and will bite any Go caller that unmarshals an
   `InteractionResource`; it is not mine to fix and is recorded here for the `📐️model` owner.
2. **`model.Ticket.MarshalJSON` rewrites `goal` into a compose id** (`repo-rust` → `🎯reporust`) and
   **`model.Goal.MarshalJSON` does the same for `parent`.** Rust's model is plain serde. Three
   differences in `✏️mutation-execution::records`.
3. **`model.Ticket.UnmarshalJSON` back-fills `summary` from the last `ticket.close` interaction.**
   Rust's model does not. One further difference (`SPLIT-GO.summary` = `"done"`).

Defects 2 and 3 are model-layer divergences between Go and Rust, both in a package I do not own.
They were neutralised where they belong — in the fixture-backed double, which must reproduce the
frozen record set **verbatim**, exactly as Rust's plain serde does: `plainTicket`/`plainGoal`
(defined types over `model.Ticket`/`model.Goal`, which in Go do **not** inherit the source type's
methods) are used for the recording context's decode and for `ticketSource`/`goalSource`/`Snapshot`.
No `model` behaviour was changed. **Rust was not edited and was not found wrong anywhere**: on every
one of the 34 executed scenarios the Rust subject is the one the `graphql-js` oracle agrees with.

## 5. Filesystem independence — verified, not assumed

Grepping the three new regions (`⚡️Execution` 670–1002, `📜️Sdl`+`🖼️Sources`+resolvers 4637–6442,
`🩻️RecordingContext` 6662–7389) for `os.`, `filepath.`, `exec.`, `codebase.`, `ticketspkg.`,
`statutespkg.`, `contributorspkg.`, `goalspkg.`, `todospkg.`, `move.`, `events.`, `languages.`,
`providers.` returns **nothing**. The only non-`model` symbol reached is `workspace.Flat`, a pure
string function (it is what `model`'s own id builders use, and it matches Rust's `identity::flat`
including the `> 0x7F` rule). Entity emojis come from `model.Emoji*` constants through
`model.EmojiText`, not from `identity.Entity`, which would have read
`🪪️identity/🧬️schema/🔣️entity-emojis.json` off disk.

Runtime behaviour with the **real** filesystem context was confirmed with a temporary
`[DEBUG]`-prefixed smoke test (since removed):

```
[DEBUG] { repo { id name } } -> {"repo":{"id":"repo:compose","name":"compose"}}
[DEBUG] { tickets(year: 26, month: 9, day: 6) { id slug title status dates { started } } } ->
        {"tickets":[{"dates":{"started":"0026-09-06T00:00:00Z"},"id":"🎫energypluginendtoend",
         "slug":"ENERGY-PLUGIN-END-TO-END","status":"OPEN","title":"Energy Plugin End To End"}, …]}
[DEBUG] { contributors { github name } } -> 5 real contributors
[DEBUG] { statutes { id priority } } -> the real statute catalog
[DEBUG] ctx tickets=3365 err=<nil>
```

`technologies` and `bundles` answer `[]` through the filesystem context because
`codebase.LoadBundles()` and `codebase.LoadTechnologies()` themselves return 0 rows
(`[DEBUG] LoadBundles=0 LoadTechnologies=0`). That is a pre-existing gap in `🗂️codebase`, not a
regression: the streamer the old resolver used returned 0 rows for the same query before this
change (recorded in `📓️opus-graphql-executor.md` §5). Recorded for the `🗂️codebase` owner.

## 6. Verification — real command output

Environment: `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`.

```
$ go build ./...      (🔗️graphql/🐹️go, 📐️model/🐹️go, ⌨️cli/🐹️go)     — silent
$ go vet ./...        (same three)                                     — silent
$ gofmt -l .          (same three, plus the four test adapters)        — silent
$ go build all        (whole go.work)                                  — silent
```

```
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner 🔗️graphql
[test] level=fundamental cases=8 executed=21 passed=21 failed=0 errored=0 parity=18/18

$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner 🔗️graphql
[test] level=quick cases=8 executed=34 passed=34 failed=0 errored=0 parity=29/29
```

The intermediate measurements on the way there, so the progression is on record:
`24 071` differences (parked adapter, old resolvers) → `6` (ported executor, interaction-resource
decode bug) → `6` (query-execution fixed, model marshal/unmarshal divergences) → `0`.

Every scenario of every case now carries a Go subject (`📤️results.jsonl`, all `passed`):

| case | scenario | implementations |
| --- | --- | --- |
| `📃️document-parsing` | corpus-projects-identically / operation-kind-is-recovered | typescript oracle, rust, **go** |
| `🚫️syntax-errors` | malformed-inputs-are-rejected | typescript oracle, rust, **go** |
| `🔀️variable-coercion` | arguments-resolve-against-variables / defaults-fill-only-absent-arguments | typescript oracle, rust, **go** |
| `🙅️unsupported-syntax` | subset-boundary-is-identical | rust, **go** |
| `▶️query-execution` | corpus-executes-identically / arguments-coerce-before-resolution | typescript oracle, rust, **go** |
| `📜️sdl-dump` | served-schema-matches-the-committed-sdl | typescript oracle, rust, **go** |
| `✏️mutation-execution` | script-changes-records-and-emits-events / a-mutation-that-cannot-apply-is-refused | rust, **go** |
| `❌️execution-errors` | every-refusal-carries-its-verbatim-message / a-refusal-writes-no-event-and-changes-no-record | rust, **go** |

`RenderSDL(BuildSchema())` was diffed against the committed
`🧬️schema/🔣️schema.graphql` — **byte-identical, zero diff lines**. That is the strongest single
statement available: the Go schema built in code *is* the committed document, type for type, field
for field, argument for argument, in declaration order.

`go test ./...` in `📦️packages/🐹️go` reports 9 failing tests. All 9 are **pre-existing** and
unrelated: measured on the pre-change file (with only the two `GetInteractions` stubs added so it
would compile) the failure set is character-for-character the same —
`TestFixApplyAutofixes`, `TestFixViaRepoContext`, `TestDefinitionNativeDocstringAutofix`,
`TestPythonTripleQuoteDocstringAutofix`, `TestPythonTripleQuoteDocstringMerge`,
`TestGraphQLBundlesQuery`, `TestGraphQLFixMutation`, `TestFixHeaderWithShebang`,
`TestSectionNewlineAfterRegion` (6 sub-tests). Their causes are `RepoContext.Fix` returning
`fix was removed; handle autofix inside script.ts policy export` unconditionally, a missing
`repo/asset/fixture/**` tree, `codebase.LoadBundles()` returning 0, and breach detection —
none of them in the executor. `TestGraphQLEffortMutationsAndQueries`, the one test that drives
`buildSchema` + `Do(Params{…})` directly, passes on the rewritten package.

## 7. New files

- `🧪️tests/▶️query-execution/🐹️.go` — the parked adapter finished against the exported
  `graphql.NewRecordingContext`; the 700-line hand-rolled fixture double it carried is gone, because
  the package now exports the real one.
- `🧪️tests/✏️mutation-execution/🐹️.go`
- `🧪️tests/❌️execution-errors/🐹️.go`
- `🧪️tests/📜️sdl-dump/🐹️.go`

`🔗️graphql-go-query-execution-adapter.go.txt` was deleted from the ticket root: it is superseded by
the real adapter, and its `recordingContext` copy is exactly the duplication the exported
`RecordingContext` exists to prevent.

## 8. launch.json

No entry was added: this change introduces **no new executable command**. The five
`⚖️gate🔗️graphql*` entries already cover all four executor cases and now exercise both subjects.
One pre-existing registration gap is noted for whoever owns it: the nx target
`@semio-tech/repo-graphql-go:test` has no gate entry (its Rust twin
`⚖️gate🔗️graphql🦀️rust🧪️test` does). It was deliberately **not** added here, because it would
land red on the 9 pre-existing failures of §6.

## 9. Left for other owners

- `📐️model` (Go): the `InteractionResource` embedding hazard (§4.1), and the `MarshalJSON` /
  `UnmarshalJSON` inference on `Ticket` and `Goal` that Rust's model does not perform (§4.2, §4.3).
  Either Go drops the inference or Rust gains it; the two models are not currently the same model.
- `🗂️codebase` (Go): `LoadBundles()` / `LoadTechnologies()` return 0 rows on this repository (§5).
- `📦️packages/🐹️go` test suite: the 9 pre-existing failures of §6, and the missing gate of §8.
