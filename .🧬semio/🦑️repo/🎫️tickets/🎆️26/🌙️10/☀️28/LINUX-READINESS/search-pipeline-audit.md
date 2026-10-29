# Bounded Native Search Pipeline

The observed Linux monorepo failure came from materializing every searchable tree node into one 250000-document index before executing any query. Raising that global cap would preserve unbounded intermediate collections. The new owner streams events into 64 SHA-256 partitions, each retaining the original document, event and byte limits, and replays one partition at a time. A finite ranked query retains only its requested top K hits; total counts cover all eligible matches. Existing bounded single-index Size -1 behavior remains available.

The schema declares partitioning, resident-partition, manifest, document, attribute, query, traversal-depth and expanded-result budgets. Neutral fixtures describe exact ranking, a 250001-document late match, pagination and cold/warm behavior. SQLite FTS5 independently verifies the neutral exact ranking. Cursors bind the repository snapshot and manifest generation, normalized query terms and fuzziness, and structural eligibility. Scoped projections have separate cache directories and metadata identities. Structural filtering precedes top K so a hidden high-ranking document cannot displace a visible match.

Tree cache staging now walks with one frame per ancestor and rejects cycles or depth overflow. Failure closes staging descriptors before retirement. Publication verifies every shard and manifest before swapping directories. An owned generation-lease interface uses Unix flock and Windows LockFileEx without external runtime libraries. Warm readers share a persistent ordinary lock inode; publication owns it exclusively, and all leases remain held until reader descriptors close. The kernel releases ownership when a process dies. Age-based stealing and unlinking lease files are unnecessary. Paths and file identities are checked before leasing and again after acquisition. Acquisition follows the caller context with bounded polling; it does not impose a shorter timeout on a healthy cold writer. New tests cover shared readers, canceled exclusive acquisition, and killed-process recovery. Existing cancellation and preservation tests now compare every manifest/shard byte, with metadata checked independently.

Stream filters use the same bounded document matcher and propagate query, document-size and cancellation errors through every stream producer. The unused batch-index filter and old unbounded fuzzy walkers are removed. Arbitrary tree fixtures use a fresh sharded projection with context, progress, explicit limit and cleanup; repository commands share the cached generation. Canonical CLI search and query use the cache, and ranked search defaults to 100 retained hits with an explicit limit up to 10000. Expanded matched ancestors retain descendant semantics within explicit node and byte budgets; an oversized expansion reports an error asking for a narrower query.

Validation in progress: the initial Go sharded suite passed in 4.492 seconds before the final lease and byte-limit refinements. The current independent SQLite FTS5 suite passes 2 tests with 12 assertions in 0.506 seconds, including all 250001 documents, continuation, eligibility and scope isolation. All 24 Go files passed formatting; a later two-file list wiring correction has its own formatter request. Correct-workspace core, tree and CLI Go checks remain queued under normal admission. Fresh native rebuild, real-monorepo cold/warm search and authenticated installer proof remain required. These bounds cover index and result construction; the existing caller supplies an already materialized tree snapshot.

Initial integration attempts exposed a harness configuration mistake: disabling the canonical Go workspace prevents transitive local module resolution. Retries use the repository go.work. Standalone duplicated-client compilation additionally reports pre-existing missing EntityKindCatalog and EntityKindByEmoji declarations; this is reported separately from the active canonical CLI. The initial owned-interface embedding collision was fixed before retries.

## Owned Product Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/🏛️core/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/🧵️shard/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/🔌️ports/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/🐚️unix-host/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🐹️go/🪟️windows-host/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧵️shards/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🐹️go/🎊️query-cache/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🐹️go/🔍️search/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/🧪️tests/🔦️search/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🔎️search/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🌳️tree/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🎊️query-cache/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🔍️tree-search/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🌊️stream-filters/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🔎️query-commands/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/❄️goal-management/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🌬️stream-artifacts/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏭️technology-bundle-loading/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/👤️contributor-store/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📖️ticket-store/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🕐️sessions/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧊️policy-registry/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🤝️g1-contract/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️tree-filter-and-search/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧵️shards/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧪️tests/🧫️fixtures/🧵️shards/🔣️.json`

The four exploratory compiler overlays are generated tool outputs, not authored fixtures. After confirming no live process referenced their old overlay arguments, they were moved under the ticket generated directory. Current Go checks use the combined canonical workspace overlay there. No source file was copied into the ticket.

The Go regression is automatically adopted by the existing @semio-tech/repo-search-go:test target through canonicalGoPlan owner-test discovery. The independent Bun suite is verified through registered repo tools exec; root coordination has been notified to wire it into the appropriate authored test target if it is not selected by the owner runner. No shared TypeScript budget/module file was changed by this search repair.

## Completed Native Validation

All Go gates passed: 25 search tests, 39 complete tree tests, 472 client root-package tests, 114 complete canonical CLI tests, and two native opacity tests covering 28 neutral vectors plus both schemas. Independent SQLite ranking passed two tests with twelve assertions. Client Ajv/projection ownership and native emitter freshness passed. Search cross-compiled for Windows amd64 and macOS arm64.

The Go candidate is frozen. Final native rebuild, real monorepo cold/warm search, authenticated installer route, and still-live authored target integration proofs are recorded separately in [native-final-validation.md](native-final-validation.md).
