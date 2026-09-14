# 📓️ Opus `events` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events`

Executor: Opus 5. Scope: wave 1, the `📡️events` module of the target tree (plan §2, §3).
All module-relative paths below are under `🧰️framework/🛍️products/🦑️repo/🔨️modules/`.

## 1. What exists now

```
📡️events/
├── 🔣️oracle.json                         ajv (JSON Schema 2020-12) + node:crypto (OpenSSL), one noOracleDecision
├── 🧬️schema/
│   ├── 🔣️.json                           envelope, 24 payload types, StoreEvent, ExportResult (draft 2020-12)
│   └── 🔣️event-kinds.json                the 70 kind strings — the single source of truth
├── 🧫️fixtures/
│   ├── ✉️payload-vectors.json            30 golden encodings, one or two per payload type
│   ├── 🗄️store-vectors.json              MOVED out of 💻️client/⌨️cli/🧫️fixtures/1️⃣g1-contract.json `eventStore`
│   └── 📤️export-vectors.json             6 entities, the frozen snapshot identity and namespaced input ids
├── 🧪️tests/
│   ├── 📋️event-kind-catalog/{🥒️.feature,🐹️.go,🦀️.rs,🟦️.ts}
│   ├── ✉️payload-encoding/{🥒️.feature,🐹️.go,🦀️.rs,🟦️.ts}
│   ├── 🗄️store-append-sequence/{🥒️.feature,🐹️.go,🦀️.rs,🟦️.ts}
│   └── 📤️export-content-hash/{🥒️.feature,🐹️.go,🦀️.rs,🟦️.ts}
└── 📦️packages/
    ├── 🐹️go/{go.mod,🐹️.go,🧪️_test.go,📋️project.json,📜️script.ts}
    └── 🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}
```

Go module `github.com/usalu/semio/repo/events`, package `events`, `go 1.25`, stdlib only.
Rust crate `semio-framework-repo-events`, `[lib] path = "🦀️.rs"`, `[lints] workspace = true`,
`[package.metadata.semio] role = "library"`, dependencies `serde`/`serde_json` (workspace) only.
Nx projects `@semio-tech/repo-events-go` and `@semio-tech/repo-events-rs`.

Go regions: `📋️EventKind`, `✉️Envelope`, `🌨️Payloads`, `📤️Emit`, `🗄️Store`, `📦️Export`, `🗣️LanguagesPending`.
Rust regions mirror them plus `🔏️Sha256`, `🧫️Fixtures`, `🧪️Tests`.

### Schema-first kind loading

`🧬️schema/🔣️event-kinds.json` is the only place a kind string is written down.

* Rust: `include_str!("../../🧬️schema/🔣️event-kinds.json")` — a compile-time load, parsed once
  through a `OnceLock` in `kind_catalog()`.
* Go: `KindCatalogPath()` resolves the file from `runtime.Caller(0)` (module-relative), and
  `LoadKindCatalog()` reads it once via `sync.Once`. `//go:embed` cannot reach a parent directory,
  so a runtime read is the only schema-first option that keeps the file out of the Go package
  directory. The typed constants stay compile-time; `TestKindConstantsMatchSchemaCatalog` (Go) and
  `kind_constants_match_schema_catalog` (Rust) assert constants ≡ file, in order, no duplicates.

### Ports

* `Emitter` / `HttpEmitter` (Go interface, Rust trait + struct). The Rust `HttpEmitter` writes a
  hand-rolled HTTP/1.1 `POST` over `std::net::TcpStream` — no external crate. Behaviour matches Go
  `Emit`: a no-operation unless `COMPOSE_SERVER_ADDR` is set, `Authorization: Bearer` from
  `COMPOSE_SERVER_TOKEN`, every delivery failure swallowed. `emit_url` / `EmitURL` are shared and
  tested. **Divergence to record:** Go's `net/http` speaks TLS, the hand-rolled Rust emitter does
  not, so an `https://` address is a no-operation in Rust rather than a real POST. The coordinator
  is bound to `127.0.0.1:8787` by default, so this is not currently reachable; a later ticket must
  either forbid `https://` for `COMPOSE_SERVER_ADDR` or give the Rust emitter TLS.
* `ExportSource` (Go interface, Rust trait) + `BuildExportSnapshot` / `build_export_snapshot`.
  The pure half of `📤️event_export.go` is now here; `💻️client/⌨️cli/📤️event_export.go` stays where it
  is and only adapts `RepoContext` to the port (`repoExportSource`), exactly as instructed.
* `Interrupt` trait (Rust) is the shape of Go's `context.Context` + progress callback: `cancelled()`
  and `report(Progress)`. The four progress steps (`encoded`, `staged`, `appended`, `synced`,
  then `committed`) and their cancellation points are identical in both implementations.

### `🗣️LanguagesPending` — what belongs to `🗣️languages`

The whole `#region 🗣️LanguagesPending` of `📡️events/📦️packages/🐹️go/🐹️.go` is source-parsing, not
events, and must move wholesale into `🗣️languages` (the module already exists and its `🗣️Pending`
region already lists exactly these symbols as incoming, so no duplicate was created today —
delete this region when that agent lands them):

`ParsedSection`, `ParsedDefinition`, `IsEmojiRune`, `ExtractEntityEmoji`, `ParseRegionMarker`,
`ParseMarkdownHeading`, `DefinitionPatterns`, `ParseSectionsFromLines`, `ParseDefinitionsFromLines`,
`BuildScopeID`, `ScopeEntry`, `BuildScopesForFile`.

`IsEmojiRune` and `ExtractEntityEmoji` are further destined for `🪪️identity` per that module's own
note. Nothing in `📡️events` calls any of them, and the Rust crate deliberately has no twin of them.

### SHA-256 consolidation note

`📡️events/📦️packages/🦀️rust/🦀️.rs` carries its own hand-rolled `Sha256` (`//#region 🔏️Sha256`).
`🔌️mcp` carries a second copy. Per the brief nothing is shared yet; a later ticket should move one
implementation into `🪪️identity` and have both crates depend on it. The doc-comment on `Sha256`
records this.

## 2. Moves, deletions and repointing

Deleted:

* `📚️library/🐹️.go`, `📚️library/go.mod`, `📚️library/📦️packages/🐹️go/` (the nx wrapper `repo-go-lib`).
  `📚️library` is now TypeScript-only, as the plan requires.
* `💻️client/⌨️cli/internal/eventstore/` (both files).

`go.work`: `📚️library` removed, `📡️events/📦️packages/🐹️go` added. (The mcp and graphql agents added
their own entries concurrently; the file now lists cli, mcp, events, graphql, coordinator.)

Root `Cargo.toml`: `…/📡️events/📦️packages/🦀️rust` added to `[workspace] members`.

Import repointing — **import lines only**, no identifier edits, because the aliases were kept:

| File | Change |
| --- | --- |
| `💻️client/⌨️cli/go.mod` | require/replace `repo/go`→`repo/events` at `../../📡️events/📦️packages/🐹️go` |
| `💻️client/⌨️cli/🧩️component.go` | `repopkg "…/repo/events"` (alias unchanged, 0 identifier edits) |
| `💻️client/⌨️cli/🔬️component_test.go` | `eventstore "…/repo/events"` (alias unchanged, 0 identifier edits) |
| `💻️client/⌨️cli/🤝️g1_contract_test.go` | `eventstore "…/repo/events"`; see below |
| `💻️client/⌨️cli/📤️event_export.go` | rewritten onto the port (this file was in scope) |
| `🖥️server/🎛️coordinator/go.mod` | require/replace repointed |
| `🖥️server/🎛️coordinator/🧩️component.go` | `repopkg "…/repo/events"` |
| `🖥️server/🎛️coordinator/🧪️g3_event_store_test.go` | `repopkg "…/repo/events"` |
| `🔌️mcp/📦️packages/🐹️go/go.mod` | indirect require/replace repointed |

`🧩️component.go` and `🔬️component_test.go` were touched on their import line and nowhere else, as
instructed — keeping the `repopkg` / `eventstore` aliases made every call site valid unchanged.

`🤝️g1_contract_test.go`: the `eventStore` fixture section and the six `TestG1EventStore*` /
`TestG1DuplicateInterruptedAndCorruptEvent` tests moved into `📡️events/📦️packages/🐹️go/🧪️_test.go`
(5,784 characters removed). The two export tests stay in the CLI because they need `RepoContext`.

The store record type had to be renamed `Event` → `StoreEvent` in Go, because the envelope already
owns the name `Event` in the merged package. No caller outside the moved tests named that type.

**Naming deviation:** the plan writes the Go test file as `🧪️.go`. The go tool only compiles tests
from `*_test.go`, so the file is `🧪️_test.go`. With `🧪️.go` the package reported `[no test files]`.

## 3. Verification — real command output

Environment: `GOWORK=C:/git/semio/go.work`, `RUSTC_WRAPPER=""`.

### Go subject

`go test ./...` in `📡️events/📦️packages/🐹️go`:

```
ok  	github.com/usalu/semio/repo/events	1.465s
```

`go test -v ./...` — 49 passing assertions (12 top-level, 37 subtests):

```
--- PASS: TestKindConstantsMatchSchemaCatalog (0.00s)
--- PASS: TestPayloadGoldenEncoding (0.00s)        [30 subtests, TicketPayload-0 … ExtractPayload-29]
--- PASS: TestEnvelopeEncoding (0.00s)
--- PASS: TestEmitURLAndNoOperation (0.00s)
--- PASS: TestStoreFixtureDeterministicReplay (0.01s)
--- PASS: TestStoreDuplicateInterruptedAndCorruptEvent (0.01s)
--- PASS: TestStoreCancellationAndMaximum (0.03s)
--- PASS: TestStoreInterruptionsPreserveCommittedLog (0.04s)   [encoded, staged, appended, synced]
--- PASS: TestStoreReplayCancellationPreservesLog (0.01s)
--- PASS: TestStagedAppendRecovery (0.02s)         [stage only, partial batch, complete batch]
--- PASS: TestExportSnapshotContentHash (0.00s)
--- PASS: TestDigestAndChecksumAreStable
```

Via the nx wrapper, `bun ./📜️script.ts test` in `📡️events/📦️packages/🐹️go`:

```
ok  	github.com/usalu/semio/repo/events	1.236s
```

### Rust subject

`cargo test -p semio-framework-repo-events`:

```
running 12 tests
test tests::digest_and_checksum_are_stable ... ok
test tests::emit_url_and_target ... ok
test tests::store_duplicate_interrupted_and_corrupt_event ... ok
test tests::envelope_encoding ... ok
test tests::payload_golden_encoding ... ok
test tests::kind_constants_match_schema_catalog ... ok
test tests::export_snapshot_content_hash ... ok
test tests::store_replay_cancellation_preserves_log ... ok
test tests::store_fixture_deterministic_replay ... ok
test tests::staged_append_recovery ... ok
test tests::store_interruptions_preserve_committed_log ... ok
test tests::store_cancellation_and_maximum ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s
```

`cargo clippy -p semio-framework-repo-events --all-targets` — clean (three findings were fixed:
two `redundant_clone`, one `question_mark`):

```
    Checking semio-framework-repo-events v0.1.0 (…\📡️events\📦️packages\🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2.09s
```

### Downstream Go stays green

```
$ (cd 💻️client/⌨️cli && go build ./...)          # no output
$ (cd 🖥️server/🎛️coordinator && go build ./...)   # no output
$ (cd 💻️client/⌨️cli && go vet ./...)             # no output — test files compile too
$ (cd 🖥️server/🎛️coordinator && go vet ./...)     # no output
```

The two export tests that stayed behind still pass against the port:

```
--- PASS: TestG1ExportRetainsHistoryAndRejectsDuplicateSnapshot (0.02s)
--- PASS: TestG1FailedAndInterruptedExportPreserveExistingLog (0.04s)
ok  	github.com/usalu/semio/repo/client	0.211s
```

### Three-way parity: Go ↔ Rust ↔ third-party oracle

The Protocol v2 harness could not be executed (see §5), so parity was proven with a ticket-local
runner that uses the same fixtures, the same oracle libraries and the same comparison the harness
case files declare. Sources (kept, they are inputs):
`🔬️events-parity/{🐹️go/{main.go,go.mod},🦀️rust/{Cargo.toml,src/main.rs},🟦️oracle.mjs}`.

```
$ GOWORK=off go run . <fixtures> <out>          # in 🔬️events-parity/🐹️go
go report written
$ cargo run --quiet -- <fixtures> <out>         # in 🔬️events-parity/🦀️rust
rust report written
$ node 🔬️events-parity/🟦️oracle.mjs C:/git/semio <out>
PASS  ajv accepts every golden payload encoding
PASS  ajv accepts a declared-kind envelope
PASS  ajv rejects an undeclared-kind envelope
PASS  node:crypto reproduces the frozen export snapshot
PASS  node:crypto reproduces the frozen namespaced input ids
PASS  go and rust write byte-identical event logs
PASS  the event log equals the bytes node:crypto predicts
PASS  go.kinds matches the oracle
PASS  rust.kinds matches the oracle
PASS  go.encodings matches the oracle
PASS  rust.encodings matches the oracle
PASS  go.envelope matches the oracle
PASS  rust.envelope matches the oracle
PASS  go.snapshot matches the oracle
PASS  rust.snapshot matches the oracle
PASS  go.inputIds matches the oracle
PASS  rust.inputIds matches the oracle
PASS  go.storeDigest matches the oracle
PASS  rust.storeDigest matches the oracle

19/19 checks passed
```

The two written logs were byte-identical, and equal to what `node:crypto` predicts:

```
{"schema":"semio.event/1","sequence":1,"id":"folder:a","kind":"folder.recorded","data":{"path":"a"},"checksum":"2bfd18cf9b20df4357e6659d54ed4f492b6786f8da24817f749deee073d42550"}
{"schema":"semio.event/1","sequence":2,"id":"file:a/x.go","kind":"file.recorded","data":{"path":"a/x.go"},"checksum":"bcc8ee13f2fee9fe7c791607ccafc48b73cf3bd17e0bb74e8e8a0fcad72f357e"}
```

Export snapshot identity agreed by three independently written implementations (Go, Rust,
Node/OpenSSL) and frozen in the fixture:
`c8fc1d7a580387eefecc074d1bfa9f4c54af00da83a1dbf301171110330b7612`.

The golden payload vectors were themselves derived independently (a Python transcription of the
omit-empty rule table), so a Go tag and a serde attribute cannot agree on a shared mistake: three
encoders and one JSON-Schema judge all had to converge.

### launch.json

`.vscode/🧩️launch.seed.jsonc` gained `🧪️test🧰️repo📡️events🦀️rust` and `🧪️test🧰️repo📡️events🐹️go`,
placed directly after the existing `🧪️test🧰️repo🔌️mcp*` pair.

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …\🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
$ grep -n "repo-events" .vscode/launch.json
7884:      "command": "bun nx run @semio-tech/repo-events-rs:test",
7891:      "command": "bun nx run @semio-tech/repo-events-go:test",
```

## 4. Language-agnostic test cases

Four cases, each with `🥒️.feature` + Go, Rust and TypeScript adapters. The TypeScript adapters are
**oracles**, not subjects: ajv judges the encodings against `🧬️schema/🔣️.json`, and `node:crypto`
recomputes the SHA-256s. `🔣️oracle.json` registers both, with one `noOracleDecision`
(`repo-event-store-recovery`) covering the staged-append recovery contract, which nothing third
party implements and which is therefore carried by Go↔Rust byte equivalence.

| Case | Scenarios | Oracle |
| --- | --- | --- |
| `📋️event-kind-catalog` | catalog-is-the-schema-file, envelope-accepts-only-declared-kinds | ajv |
| `✉️payload-encoding` | golden-encoding-per-payload, omit-empty-and-explicit-null, envelope-round-trip | ajv |
| `🗄️store-append-sequence` | append-then-replay-sequences, duplicate-and-corrupt-are-refused, interruption-preserves-committed-log, record-checksum-is-sha256 | node:crypto + noOracleDecision |
| `📤️export-content-hash` | snapshot-identity, input-ids-are-namespaced, unchanged-export-is-refused | node:crypto |

Every scenario carries `@id-`, exactly one `@level-` and exactly one `@mode-`; every feature carries
`@capability-`, an `@oracle-`/`@no-oracle-` pair and `@comparison-ordered-json-v1`, per the
`🖥️host-protocol-parity` example.

## 5. Blockers found (not caused by this change, not fixed here)

1. **The Protocol v2 harness cannot run at all right now.**
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` is an unmerged file
   (`git status` reports `UU`) containing four unresolved conflict blocks at lines 5883/13565/20683/22454
   (`<<<<<<< Updated upstream` … `>>>>>>> Stashed changes`), left by an interrupted
   `Auto stash before merge of "🐙ueli/⛳wip" and "origin/🐙ueli/⛳wip"`. Every harness phase loads it:

   ```
   $ bun ./📜️script.ts discover        # in 🔨️modules/🧪️test
   SyntaxError: JSON Parse error: Unrecognized token '<'
         at testTaxonomy (…/🧪️test/📦️packages/🟦️typescript/🟦️.ts:118:23)
         at discoverTestCases (…:540:20)
   ```

   Resolving it needs git operations this agent is forbidden from performing. Until a human resolves
   the merge, `discover`, `contract`, `oracle`, `subject` and `parity` are all unavailable, and the
   four case adapters above are therefore **authored but not executed**. `📓️harness-verification.md`
   was not present at any point during this work.

2. **Every Rust nx test target is broken on native Windows.**

   ```
   $ bun ./📜️script.ts test            # in 📡️events/📦️packages/🦀️rust
   error: Exact owner catalog mode drift: …/📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json
         at semanticExactOwnedFileCatalog (…/📚️library/🔍️discovery/🟦️.ts:4867:42)
         at loadTaxonomy … getCargoWorkspaceIndex … resolveCargoPackageName … runCargoTestBudgeted
   ```

   The file is `100644` both on disk and in git; `semanticExactOwnedFileCatalog` requires
   `snapshot.mode === 0o644`, which Node cannot report on Windows. This hits `runCargoTestBudgeted`,
   so it affects every `📦️packages/🦀️rust` nx `test` target, not just this module. `cargo test -p …`
   works, and the Go nx target works because it does not call `loadTaxonomy`.

3. **The coordinator Go test suite fails on Windows, before and after this change.**
   Four `TestG3*` cases fail with `store metadata durability unsupported: …: Access is denied` from
   `🛡️durability.go`. The cause is `syncStoreParent` in `🪟️durability_windows.go` calling
   `syscall.Open(dir, O_RDONLY, 0)`, which cannot open a directory handle on Windows — it needs
   `CreateFileW` with `FILE_FLAG_BACKUP_SEMANTICS`. No events symbol is on that path;
   `go build ./...` and `go vet ./...` are green. Worth its own ticket.

4. **A `📚️library` fixture is now stale.** `📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`
   still names `📚️library/📦️packages/🐹️go/📋️project.json` and `…/🐹️go/README.md`, which this ticket
   deleted. That fixture belongs to `📚️library`; it must be regenerated by whoever owns it (or by the
   wave-4 wiring agent) or its statute case will fail.

## 6. Left to do

* Delete `#region 🗣️LanguagesPending` from `📡️events/📦️packages/🐹️go/🐹️.go` once `🗣️languages`
  lands those twelve symbols (that module's `🗣️Pending` region already claims them).
* Move one hand-rolled SHA-256 into `🪪️identity` and have `📡️events` and `🔌️mcp` depend on it.
* Give the Rust `HttpEmitter` TLS, or forbid `https://` in `COMPOSE_SERVER_ADDR`.
* Run the four harness cases once blocker 1 is resolved; the Go adapters additionally need the
  generated host's `go.mod` to `replace` `github.com/usalu/semio/repo/events`, and the Rust
  adapters need the generated host to depend on `semio-framework-repo-events` by path — neither
  could be confirmed because `discover` never ran.
* Regenerate the stale `📚️library` fixture (blocker 4).

## 7. Reproducing the parity run

```bash
export GOWORK=C:/git/semio/go.work RUSTC_WRAPPER=""
FIX="C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧫️fixtures"
OUT="$TICKET/🗑️generated/events"
(cd "$TICKET/🔬️events-parity/🐹️go"   && GOWORK=off go run .          "$FIX" "$OUT")
(cd "$TICKET/🔬️events-parity/🦀️rust" && cargo run --quiet --         "$FIX" "$OUT")
node "$TICKET/🔬️events-parity/🟦️oracle.mjs" C:/git/semio "$OUT"
```

`🗑️generated/events/` was emptied after the numbers above were recorded, per the ticket rules; the
command above rebuilds it from the committed runner sources.

## 8. Harness execution (follow-up)

The `🔣️taxonomy.json` conflict (§5 blocker 1) is resolved, so the four cases of §4 were executed
through the Protocol v2 harness. They now run and pass for Go, Rust and the TypeScript oracle. Six
things had to be fixed; none of them was a defect in the framework's generated-host logic, which
already did everything these cases needed.

### 8.1 The oracle manifest was at a path nothing discovers

`📡️events/🔣️oracle.json` was never read. `discoverTestContributions` looks for
`<owner>/<testContributionDirName>/<json leaf>`, and the taxonomy declares
`testContributionDirName = "🔮️oracle"` and `testContributionFileKindId = "json"` — i.e.
`📡️events/🔮️oracle/🔣️.json`, which is where all ~200 discovered manifests live. The symptom was
`unknown oracle id node-crypto` on two cases while `@oracle-ajv` resolved anyway, because `ajv` is
registered repository-wide by `🛍️products/📓️print`. The manifest was moved and rewritten in the
discovered shape (`capabilities`, `comparisonProfiles`, `engine`, `source`, `license`, `testOnly`,
`platforms`, `rationale`), replacing the `covers` key, which nothing reads.

### 8.2 Both oracle ids are owner-scoped

`@oracle-ajv` raised `oracle-capability-mismatch`: an oracle is trusted only for the capability set
it was surveyed against, `registry.oracles.find` resolves an id to the FIRST entry carrying it, and
print's `ajv` was surveyed for print's catalogue. `node:crypto` is likewise already registered by
`🔌️mcp` as `node-crypto-sha256`, for mcp capabilities only. Reusing either id means editing another
owner's survey, so this owner registers its own: **`ajv-repo-events`** and
**`node-crypto-repo-events`**, both naming the same distributions, each surveyed for this module's
four capabilities. The feature tags name them. This adds no dependency breach: the
`oracle-in-production` scan reports one hit per `path::package`, and both packages were already
registered elsewhere (the repository-wide count is unchanged at 65 for `node:crypto`, 1 for `ajv`).

### 8.3 The four case directories were renamed to kebab-case

`✉️payload-encoding` → `payload-encoding`, `📋️event-kind-catalog` → `event-kind-catalog`,
`📤️export-content-hash` → `export-content-hash`, `🗄️store-append-sequence` → `store-append-sequence`.
`testCaseSlugPattern` is `^[a-z0-9]+(?:-[a-z0-9]+)*$` and these four were the ONLY `case-slug`
breaches in the repository — every other discovered case is already kebab-case, matching the
`🔗️graphql` cases. `--case store-append-sequence` addresses them now.

### 8.4 The Rust adapters declared dependencies they cannot have

Three adapters used `#[derive(serde::Deserialize)]` and bare `serde_json::…`. A generated Rust host
depends on the test-host crate, the owner's crate and the owner's declared oracle crates — nothing
else — so this could not compile (`E0433: unresolved module or unlinked crate serde`). The fix is the
rule CLAUDE.md already states and `🔗️graphql` already follows: the crate re-exports what its public
API forces on a client. `semio-framework-repo-events` now carries `pub use serde_json;` (its
`Event::payload`, `Input::data` and `ExportEntity::value` are all `serde_json::Value`, so no client
could use the API without naming the type), and the adapters read their fixtures through
`events::serde_json` and a `Value`-walking decoder instead of derives. `serde` itself is NOT
re-exported: nothing in the public API requires a client to derive.

### 8.5 Four adapters were not hermetic — the real test failure

With the compile fixed, `store-append-sequence` failed all three Go fundamental scenarios with
`duplicate event: folder:a` / `corrupt event log at sequence 1`. The harness reuses ONE work
directory per `(case, role, implementation)` (`planExecution`: `work/<projectName>-<role>-<impl>`)
and never empties it — it only clears the `🏁️done` marker. An append-only store is exactly the thing
that notices: the log written by the previous RUN was still there, so the first append of the next
run was a duplicate. Rust passed only because its work directory happened to be fresh that run, so
this was a latent failure in both languages and in `export-content-hash` too. Every scenario that
writes a log now owns a subdirectory of `WorkDir` named after the scenario, removed and recreated
before use, in all four adapters (`freshScenarioDir` in Go, `fresh_scenario_dir` in Rust).

### 8.6 Cold `go run` exceeds the fundamental budget on Windows

`testLevelBudgetMs("fundamental")` is 15 s, and the first `go run .` in a freshly generated host
directory compiles the events module and the test-host module from cold, which takes longer on this
machine. It is a warm-cache effect, not a case defect: `SEMIO_TEST_BUDGET_MS=600000` on the first run
after any change to the module or to a case slug, and every subsequent run fits the default budget.
All the output below is from runs with the DEFAULT budget.

### 8.7 Real output

```
$ export RUSTC_WRAPPER="" GOWORK=C:/git/semio/go.work
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts discover | grep modules/📡️events
test-…-events-ed0df0-event-kind-catalog     …/📡️events/🧪️tests/event-kind-catalog     [rust,typescript,go]
test-…-events-ed0df0-export-content-hash    …/📡️events/🧪️tests/export-content-hash    [rust,typescript,go]
test-…-events-ed0df0-payload-encoding       …/📡️events/🧪️tests/payload-encoding       [rust,typescript,go]
test-…-events-ed0df0-store-append-sequence  …/📡️events/🧪️tests/store-append-sequence  [rust,typescript,go]

$ bun …/🧪️test/📜️script.ts subject fundamental --owner 📡️events
[test] level=fundamental cases=4 executed=18 passed=18 failed=0 errored=0 parity=0/0

$ bun …/🧪️test/📜️script.ts oracle fundamental --owner 📡️events
[test] level=fundamental cases=4 executed=9 passed=9 failed=0 errored=0 parity=0/0

$ bun …/🧪️test/📜️script.ts parity fundamental --owner 📡️events
[test] level=fundamental cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27

$ bun …/🧪️test/📜️script.ts parity long --owner 📡️events
[test] level=long cases=4 executed=36 passed=36 failed=0 errored=0 parity=36/36

$ bun …/🧪️test/📜️script.ts parity fundamental --case store-append-sequence
[test] level=fundamental cases=1 executed=9 passed=9 failed=0 errored=0 parity=9/9
```

`long` adds the `@level-long` scenario `interruption-preserves-committed-log` and the three
`@level-quick` ones, so all 12 declared scenarios are exercised: 12 Go + 12 Rust subjects and 12
TypeScript oracle results, every subject paired and equal under `ordered-json-v1`.

Contract, unit tests and the neighbouring owner:

```
$ bun …/🧪️test/📜️script.ts contract --owner 📡️events
   → breaches naming 📡️events: 1 (was 7)
     testing/dependency  …/📡️events/📦️packages/🦀️rust/🦀️.rs
       Production source imports the registered oracle serde-json-equation-carrier-reader
   (the repository-wide false positive of §6; 143 identical hits, every crate using serde_json)
   Repository-wide breach count 1368 → 1362; the phase still exits 1 on the 1362 pre-existing ones.

$ cargo test -p semio-framework-repo-events     → 12 passed; 0 failed
$ cargo clippy -p semio-framework-repo-events --all-targets   → clean
$ (cd 📡️events/📦️packages/🐹️go && go test ./...)   → ok  github.com/usalu/semio/repo/events
$ bun …/🧪️test/📜️script.ts parity long --owner 🔗️graphql
[test] level=long cases=4 executed=17 passed=17 failed=0 errored=0 parity=16/16   (unchanged)
```

### 8.8 What §6 can now be struck

* "Run the four harness cases once blocker 1 is resolved" — done, all four green.
* "the Go adapters additionally need the generated host's `go.mod` to `replace`
  `github.com/usalu/semio/repo/events`, and the Rust adapters need the generated host to depend on
  `semio-framework-repo-events` by path" — **neither needed a change**. `goSutModule` +
  `materializeGoHost` (added by the graphql agent) and `rustSutCrate` + `materializeRustHost` both
  already walk up from the owner and link the owner's package; `materializeRustHost` additionally
  gates the subject crate behind the `sut` feature, which the subject role turns on. The only thing
  missing was on this side of the boundary: the crate was not re-exporting what its API demands.

Still open from §6: the `🗣️LanguagesPending` region, the SHA-256 consolidation into `🪪️identity`, TLS
for the Rust `HttpEmitter`, and the stale `📚️library` fixture.
