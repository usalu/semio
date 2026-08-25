# Phase 9 G1 Go CLI Zero-External Contract

Date: 2026-08-25  
Owner: G1 Go CLI packet  
Status: independent-audit P0 remediation complete; scoped gates green; repository-wide module suite remains baseline-red

## Scope

The packet named a nested `📦️packages/🐹️go` path, but the live Go module is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli`

Only that module's `go.mod`, naturally-unused `go.sum`, Go source, tests, canonical fixture, and this Phase 9 report were changed. `go.work`, root/shared scripts, other modules, ticket metadata, Rust, JavaScript, Cargo, Nx, Wasm, and browser surfaces were not changed.

## Exact Census

| Measure | Before (`HEAD`) | After |
| --- | ---: | ---: |
| Third-party `require` rows | 58 | 0 |
| Direct third-party rows | 11 | 0 |
| Indirect third-party rows | 47 | 0 |
| First-party workspace rows | 1 | 1 |
| `go.sum` lines | 178 | absent, naturally unused |
| Distinct third-party source import paths in the main source | 15 | 0 |
| Third-party module-backed packages including tests | not zero | 0 |

The retained requirement is the genuine local first-party module `github.com/usalu/semio/repo/go v0.0.0`, replaced by `../../📚️library`.

Removed direct roots:

- `github.com/Masterminds/sprig/v3`
- `github.com/blevesearch/bleve/v2`
- `github.com/bmatcuk/doublestar/v4`
- `github.com/dustin/go-humanize`
- `github.com/google/uuid`
- `github.com/graphql-go/graphql`
- `github.com/mark3labs/mcp-go`
- `github.com/sabhiram/go-gitignore`
- `github.com/spf13/cobra`
- `gopkg.in/yaml.v3`
- `modernc.org/sqlite`

All 47 transitive rows left with those roots. No external package type is present in an owned public signature.

## Owned Replacements

| Contract | Owned standard-library implementation |
| --- | --- |
| command parse/help/dispatch | `internal/command` schema, bounded flag parser, help renderer, dispatch, positional validation, context and streams |
| templates | `internal/templatefunc` deterministic `text/template` function vocabulary; invalid-template errors remain explicit |
| recursive glob and ignore rules | `internal/glob` and `internal/ignore`; cross-platform slash normalization, recursive `**`, deterministic traversal, bounded pattern cache, cancellation/progress traversal |
| search/index | `internal/search`; bounded append/replay index events, deterministic ranked results, corruption detection, context/progress throughout reindex and query, staged verified cache swaps |
| identifiers | `internal/id`; `crypto/rand` UUID-v4-shaped identifiers without external types |
| YAML/config | `internal/yaml`; bounded YAML mappings, sequences, scalars and deterministic encoding |
| GraphQL | `internal/graphql`; owned schema/parser/executor and validation/operation inspection behind the existing executor boundary |
| MCP | `internal/mcp` and `internal/mcpserver`; owned wire schemas, registry, bounded stdio JSON-RPC loop |
| relative time | `internal/humanize` |
| persisted export | `internal/eventstore` plus `🐹️event_export.go`; append-only checksummed JSONL events and deterministic replay replace SQLite CRUD |

The event store uses schema `semio.event/1`, monotonically increasing sequences, duplicate-ID rejection, SHA-256 checksums, 1 MiB event/64 MiB batch/100,000-event bounds, context cancellation, progress callbacks, fsync, and a durable `.stage` record around `O_APPEND`. The stage records the last-valid byte boundary and checksums for both the committed prefix and pending batch. Recovery retains a fully synced batch or rolls a partial batch back to the recorded boundary. The prior log is never removed or truncated before a new append succeeds. Cancellation at encoding, staging, append, sync, or replay leaves the last-valid log byte-identical.

Exports assign every entity a deterministic snapshot-scoped event ID. Re-exporting an identical snapshot produces explicit `ErrDuplicate` idempotence without writing; a changed snapshot appends at the next sequence and retains all prior history.

Search persistence uses `semio.search.event/1` records with sequence and corruption checks. Reindexing performs cancellable fingerprint subprocesses and lock waiting, iterative bounded traversal, bounded document mutation, cancellable persistence, staged verification, and a recoverable directory swap. Bounds are explicit for traversal nodes (500,000), documents (250,000), document bytes (1 MiB), pending events (500,000), index bytes (256 MiB), query terms (32), and query bytes (4 KiB). Cancellation, maximum-plus-one, or an index error removes only the staged candidate and preserves the last-valid index. Matching-fingerprint corruption and query/index errors propagate to `search` and `list`; they are not silently replaced by an in-memory fallback. Neither implementation uses CRUD or CRDT behavior.

Source is organized with regions/subregions. The canonical language-agnostic fixture is `🧫️fixtures/g1-contract.json`.

## Executable Contract Coverage

`🧪️g1_contract_test.go` executes the canonical fixture and covers:

- invalid flags;
- recursive glob;
- bad templates;
- search results, corrupt index, and cancellation;
- reindex progress and cancellation during collection, mutation, persistence, and lock waiting;
- maximum-plus-one reindex preservation and corrupt-current-index error propagation;
- deterministic event bytes/replay;
- retained export history, distinct changed snapshots, and explicit duplicate-snapshot rejection;
- existing-log preservation on failed and interrupted export;
- event interruption at encoded, staged, appended, synced, and replay phases;
- stage-only, partial-append, and completed-append crash recovery;
- maximum payload and maximum-plus-one rejection with byte-identical last-valid state;
- YAML/config decoding.

Existing preservation tests were also exercised for command dispatch/flags, ignored-directory globbing, repo-root templates, MCP kinds and initialization handshake, GraphQL query/mutation paths, and event export.

## Verification Evidence

Run from the live CLI module unless noted otherwise.

### Green

```text
go test -run '^$' ./...
PASS: root plus 12 owned internal packages compile
```

```text
go test -count=1 -run '^(TestG1.*|TestStagedAppendRecovery|TestGlobByExtensionSkipsIgnoredDirectoryRoot|TestSyncCommandRunsGitHubSynchronization|TestTreeCommandFlags|TestRenderPromptTemplateUsesRepoMetaRoot|TestMcpCommandKinds|TestMicroCommitCommandExists|TestMcpStdioInitializeHandshake|TestGraphQLRepoQuery|TestGraphQLPoliciesQuery|TestGraphQLContributorsQuery|TestGraphQLEffortMutationsAndQueries|TestExportToEventLogSchema|TestExportToEventLogEmpty|TestSearchMonorepoTreeWithCache)$' ./...
PASS: client 4.862s; internal event store 0.312s
```

```text
go vet ./...
PASS

GOWORK=off go mod tidy -diff
PASS: empty diff

GOWORK=off go mod verify
all modules verified
```

```text
GOWORK=off go list -m all
github.com/usalu/semio/repo/client
github.com/usalu/semio/repo/go v0.0.0 => ../../📚️library
```

```text
GOWORK=off go list -deps -test -f '{{if .Module}}{{.Module.Path}}{{end}}' ./... | sed '/^$/d' | sort -u
github.com/usalu/semio/repo/client
github.com/usalu/semio/repo/go
```

The strict source-import scan and forbidden-root declaration scan both emitted no rows.

```text
GOWORK=off GOOS=windows GOARCH=amd64 go test -run '^$' -exec=true ./...
PASS

GOWORK=off GOOS=linux GOARCH=amd64 go test -run '^$' -exec=true ./...
PASS
```

All changed Go files were formatted with `gofmt`; the scoped tracked diff passed `git diff --check`.

### Full-Suite Baseline Failure

The pre-change `go test ./...` was already red: the devcontainer test invokes Bash `mapfile`, unavailable in the host Bash, and MCP bootstrap asset assertions expect stale root script arguments. The initial run was stopped after 132 seconds while later exhaustive repository scans were still executing.

The final bounded command was:

```text
go test -count=1 -timeout 180s ./...
FAIL after 180.302s
```

It reproduced the two devcontainer `mapfile: command not found` failures and five stale MCP bootstrap assertions, then timed out in the pre-existing repository-wide `TestExhaustiveFoldersNonEmpty` scan. The timeout stack was traversing the shared repository from `ScopeToFiles`; the dedicated recursive-glob and ignore-root tests pass. These failures require changes to root devcontainer/MCP assets, stale broad fixture expectations, or the scope of the exhaustive repository test, all outside G1 ownership.

The post-remediation bounded short suite was:

```text
GOWORK=off go test -short -count=1 -timeout 90s ./...
FAIL: client 6.053s; internal/eventstore PASS 0.516s
```

It completed with the same stale MCP assertions plus broad pre-existing repository taxonomy, missing legacy paths, removed-fix-command, emoji-variation, and hook-config expectation drift. No G1 contract, staged-recovery, export, or reindex test failed.

## Audit Conclusion

The isolated CLI module has zero third-party manifest rows, zero third-party module-backed packages (including tests), zero third-party source imports, no former-provider terminology, and no residual declaration of the removed roots. Its required runtime behaviors are supplied by owned standard-library packages and executable canonical hostile fixtures. The destructive export and unbounded/error-swallowing reindex findings from `📓️codex-p9-g1-go-cli-independent-acceptance-audit-2026-08-25.md` are resolved with byte-preservation tests. The only retained module edge is the genuine first-party local repo library.
