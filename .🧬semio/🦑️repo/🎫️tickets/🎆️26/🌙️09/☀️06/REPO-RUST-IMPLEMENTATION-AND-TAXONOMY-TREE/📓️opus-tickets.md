# 📓️ Opus executor report — `🎫️tickets`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets`.
Session: resumed after a rate limit killed the previous executor at the "oracle and parity phases" step.
Host: Windows, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`.

## 1. What is on disk

```
🎫️tickets
├── 🧬️schema/🔣️.json                                    270 lines
├── 🔮️oracle/🔣️.json                                     70 lines   (2 oracles, 4 no-oracle decisions)
├── 📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}   🦀️.rs = 2 710 lines
├── 📦️packages/🐹️go/{go.mod, 🐹️.go, 🔬️_test.go, 🔭️exhaustive_test.go, …}  🐹️.go = 3 546 lines (owned by `go-split`)
└── 🧪️tests/
    ├── 🪪️ticket-id-scheme/            {🥒️.feature, 🦀️.rs, 🧫️fixtures/🪪️id-vectors.json}
    ├── 📄️ticket-document-codec/       {🥒️.feature, 🦀️.rs, 🟦️.ts (ajv oracle), 🧫️fixtures/📄️documents.json}
    ├── 🔓️open-close-reopen-lifecycle/ {🥒️.feature, 🦀️.rs, 🧫️fixtures/🔓️lifecycle.json}
    ├── 💾️important-document-transaction/ {🥒️.feature, 🦀️.rs, 🧫️fixtures/💾️cases.json}
    └── 🐙️issue-sync-transcripts/      {🥒️.feature, 🦀️.rs, 🟦️.ts (second implementation), 🧫️fixtures/🐙️transcripts.json}
```

Crate `semio-framework-repo-tickets` (`[lints] workspace = true`, `role = "library"`, deps only
`serde`/`serde_json` + the sibling path crates `model`, `identity`, `providers`, `events`) covers the
whole scope of the assignment:

- id/path scheme — `TicketId` (both spellings + `repo://` URI + parent slug), `TicketLayout`,
  `ticket_slug_from_title`, `validate_ticket_emoji_title`, `parse_dated_dir`, path helpers;
- document codec — `decode_ticket_document` / `encode_ticket_document` reproducing Go's
  `json.MarshalIndent` bytes including its HTML escaping (`go_json_string`);
- lifecycle — `TicketService::{open, close, reopen, change, search, purge, read}` over the
  `TicketStore` trait, with `MemoryTicketStore` and `FileTicketStore` implementations, `TicketQuery`
  filters, `PurgeReport` (`OVERSIZED_FILE_BYTES`, `OVERSIZED_FOLDER_BYTES`);
- important-document transaction — `inspect/ensure/restore/remove_important_document`,
  `rollback_important_creation`, `TransactionJournal`/`JournalStep`;
- GitHub sync through the providers `IssueTracker` port — `ensure_ticket_issue`,
  `close_ticket_issue`, `milestone_number_for_title`, `sync_open_issue`, `sync_close_issue`, plus
  `IssueTrackerScript`/`RecordedIssueTracker` for scripted transcripts;
- plan/spec resolution — `PlanRoots`, `resolve_plan_source`, `apply_ticket_plan_from_ids`,
  `move_ticket_plan_into_folder`, `strip_plan_frontmatter`, `format_plan_comment`;
- file scope through a `VersionControl` port — `compute_ticket_file_scope`, `parse_diff_lines`,
  `filter_ticket_workspace_files`, `is_repo_excluded_path`, `can_close_ticket`;
- events — `EventSink` trait with `RecordingEventSink` and `CoordinatorEventSink`, `Clock`/`FixedClock`.

Everything filesystem-shaped goes through `TicketStore`; nothing in the crate touches `std::fs`
outside `FileTicketStore`.

## 2. Verification (real output)

### 2.1 Crate unit tests

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-tickets
running 9 tests
test tests::the_encoder_html_escapes_the_way_go_does ... ok
test tests::a_document_without_a_status_is_refused ... ok
test tests::a_title_becomes_an_upper_kebab_slug ... ok
test tests::unknown_members_are_dropped_on_re_encoding ... ok
test tests::ticket_id_round_trips_through_both_spellings ... ok
test tests::a_hunk_stream_becomes_line_numbers ... ok
test tests::an_open_close_reopen_round_trip_emits_three_events ... ok
test tests::a_failed_save_rolls_the_important_document_back ... ok
test tests::an_oversized_artifact_is_purged_but_the_document_is_not ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 2.2 Clippy

Clippy was **not** clean when this session resumed: 3 lib warnings + 3 test warnings on the tickets
crate. Fixed in this session:

- `&previous.title.clone()` → `&previous.title` in the reopen path (`redundant_clone`);
- `dated_children` returned `TicketResult<Vec<i64>>` while never failing → returns `Vec<i64>`; its
  three call sites in the search walk lost their `?` (`unnecessary_wraps`);
- `!self.slugs.iter().any(|slug| *slug == ticket.slug)` → `!self.slugs.contains(&ticket.slug)`;
- 3 redundant `.clone()` calls on `opened.id`/`closed.id` in the crate's own tests (`cargo clippy --fix`).

```
$ RUSTC_WRAPPER="" cargo clippy -p semio-framework-repo-tickets --all-targets
    Checking semio-framework-repo-tickets v0.1.0 (…\🎫️tickets\📦️packages\🦀️rust)
    Finished `dev` profile [unoptimized] target(s)
```

No warning of any kind is emitted for `semio-framework-repo-tickets` any more (the run still prints 3
warnings belonging to `semio-framework-repo-identity` and `semio-framework-repo-providers`, which are
other owners' crates and were left untouched). `cargo test` re-run after the fixes: 9 passed.

### 2.3 Harness — discover

```
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts discover | grep tickets
test-…-tickets-da100c-important-document-transaction   …/🧪️tests/💾️important-document-transaction   [rust]
test-…-tickets-da100c-issue-sync-transcripts           …/🧪️tests/🐙️issue-sync-transcripts           [rust,typescript]
test-…-tickets-da100c-open-close-reopen-lifecycle      …/🧪️tests/🔓️open-close-reopen-lifecycle      [rust]
test-…-tickets-da100c-ticket-document-codec            …/🧪️tests/📄️ticket-document-codec            [rust,typescript]
test-…-tickets-da100c-ticket-id-scheme                 …/🧪️tests/🪪️ticket-id-scheme                 [rust]
```

### 2.4 Harness — subject (rust), oracle, parity

```
$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts subject fundamental --owner 🎫️tickets --implementation rust
[test] level=fundamental cases=5 executed=24 passed=24 failed=0 errored=0 parity=0/0

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts oracle fundamental --owner 🎫️tickets
[test] not-exercised …/💾️important-document-transaction (recorded no-oracle decision repo-ticket-important-document — its evidence is discharged by the subject phase)
[test] not-exercised …/🔓️open-close-reopen-lifecycle (recorded no-oracle decision repo-ticket-lifecycle — …)
[test] not-exercised …/🪪️ticket-id-scheme (recorded no-oracle decision repo-ticket-id-scheme — …)
[test] level=fundamental cases=5 executed=9 passed=9 failed=0 errored=0 parity=0/0 not-exercised=3

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity fundamental --owner 🎫️tickets
[test] level=fundamental cases=5 executed=33 passed=33 failed=0 errored=0 parity=9/9   (exit 0)
```

The 9 parity comparisons are the two cases that have a second reader: `📄️ticket-document-codec`
against the `ajv` draft-2020-12 oracle over the six **real committed** `🎫️ticket.json` documents, and
`🐙️issue-sync-transcripts` against the TypeScript second implementation of the `IssueTracker`
synchronisation contract.

### 2.5 Launch entries

Six entries already exist in `.vscode/🧩️launch.seed.jsonc` (group `4_gate`, orders 425.9–426.5) and
were regenerated in this session:

```
⚖️gate🎫️tickets🦀️rust🧪️test       bun x nx run @semio-tech/repo-tickets-rs:test
⚖️gate🎫️tickets🪪️id-scheme        …:test-parity -- fundamental --case 🪪️ticket-id-scheme
⚖️gate🎫️tickets📄️document-codec   …:test-parity -- fundamental --case 📄️ticket-document-codec
⚖️gate🎫️tickets🔓️lifecycle        …:test-parity -- quick       --case 🔓️open-close-reopen-lifecycle
⚖️gate🎫️tickets💾️important-document …:test-parity -- fundamental --case 💾️important-document-transaction
⚖️gate🎫️tickets🐙️issue-sync       …:test-parity -- fundamental --case 🐙️issue-sync-transcripts

$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

One of the generated gates was executed end to end to prove the entry is real:

```
$ bun x nx run @semio-tech/repo-test-domain:test-parity -- fundamental --case 🪪️ticket-id-scheme
[test] level=fundamental cases=1 executed=5 passed=5 failed=0 errored=0 parity=0/0
NX   Successfully ran target test-parity for project @semio-tech/repo-test-domain
```

## 3. Open items and honest gaps

### 3.1 No Go adapters — blocked on the Go package's shape, not on this module

The Go package builds (`GOWORK=…/go.work go build ./…/🎫️tickets/📦️packages/🐹️go/` succeeds, exit 0),
but the split produced the **legacy, filesystem-global** shape, not a port-shaped twin:

```
$ go doc -all …/🎫️tickets/📦️packages/🐹️go | grep -E '^type '
(no output — the package exports no types at all)
```

Its exported surface is `OpenTicket`, `ReadTicket(year, month, day, slug)`, `SaveTicket(*model.Ticket)`,
`FinishTicket`, `ReopenTicket`, `GetTicketPath`, `PurgeOversizedTicketArtifacts`, … — all of which
resolve the live repository root themselves and read/write the developer's own `.🧬semio`. There is
no `DecodeTicket`/`EncodeTicket`, no `MemoryTicketStore`, no `TicketID`, no service value, so **no Go
adapter can call the same contract the five cases pin** (compare `🎯️goals`, whose Go package does
expose `DecodeGoal`/`EncodeGoal`/`NewMemoryGoalStore` and therefore has Go adapters). Producing them
requires editing `🎫️tickets/📦️packages/🐹️go/🐹️.go`, which is owned by the concurrent `go-split`
agent (whose brief regenerates that file from the AST splitter, so any edit here would be both
out of bounds and overwritten). I did not write a Go adapter that re-implements the codec inside
the adapter itself, because that would be a subject testing its own source rather than the Go
implementation.

Run for the record:

```
$ bun …/🧪️test/📜️script.ts subject fundamental --owner 🎫️tickets --implementation go
[test] level=fundamental cases=5 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=5
```
(`🐙️issue-sync-transcripts` and `📄️ticket-document-codec`: "no implementation served the requested
phase(s) subject"; the other three: their recorded no-oracle decision.)

**Hand-off**: once `github.com/usalu/semio/repo/tickets` exposes `DecodeTicket`/`EncodeTicket`, a
`TicketStore`-equivalent interface with an in-memory implementation, an id/layout type and a service
value, the five `🐹️.go` adapters are mechanical — mirror the Rust adapters scenario for scenario
(they project plain ordered JSON only) and re-run
`parity fundamental --owner 🎫️tickets`. The two no-oracle decisions `repo-ticket-id-scheme` and
`repo-ticket-lifecycle` already name that Go twin as the differential half that is still missing;
those rationales stay accurate as written and need no edit when the twin lands, only the adapters.

### 3.2 `nx run @semio-tech/repo-tickets-rs:test` fails for a repo-wide reason

```
$ bun x nx run @semio-tech/repo-tickets-rs:test
error: Invalid taxonomy schema:
- generatorContracts["wgpu-frame-worker"] tracked output "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js" is missing.
  at loadTaxonomy (…/📚️library/🔍️discovery/🟦️.ts:1288:38)
  at getCargoWorkspaceIndex → resolveCargoPackageName → runCargoTestBudgeted
```

Not a tickets defect: `loadTaxonomy()` validates every generator contract in the repository, and the
`🧊️wgpu/🎞️frame-worker/🤖️generated/` directory does not exist in this working tree. The identical
failure reproduces on a sibling owner (`bun x nx run @semio-tech/repo-model-rs:test`), so every
Rust nx test target in the monorepo is down until the renderer owner regenerates that file. The
underlying `cargo test -p semio-framework-repo-tickets` passes (§2.1), and the harness phases, which
do not go through `runCargoTestBudgeted`, all pass (§2.4). The `⚖️gate🎫️tickets🦀️rust🧪️test` launch
entry will start working again with no change on this side once that generated file is back.

### 3.3 Case-directory naming

All five case directories already carry the leading emoji identity required by the 2026-09-06 06:10
coordinator decision, so this owner needs no rename sweep in the audit wave.

## 4. Files touched in this session

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust/🦀️.rs` — clippy fixes (§2.2).
- `.vscode/launch.json` — regenerated from the seed (the seed itself already carried the six gates).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-tickets.md` — this report.

No Go package, no other owner's crate and no `🔣️taxonomy.json` was modified. No git-modifying command
was run.
