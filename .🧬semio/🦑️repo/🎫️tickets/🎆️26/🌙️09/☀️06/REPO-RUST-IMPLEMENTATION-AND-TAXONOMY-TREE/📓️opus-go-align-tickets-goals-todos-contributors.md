# 📓️ Opus executor — Go alignment of `🎫️tickets`, `🎯️goals`, `📝️todos`, `🧑️contributors`

Host: Windows, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`.
Owned: the four Go packages `🔨️modules/{🎫️tickets,🎯️goals,📝️todos,🧑️contributors}/📦️packages/🐹️go`
and their `🧪️tests/*/🐹️.go` adapters. Nothing else was edited — no Rust crate, no other Go package,
no fixture, no feature file, no schema, no oracle manifest, no `launch.seed.jsonc`.

The `repo` MCP server refused to initialise for this session (`-32602: invalid initialize params`),
so no `ticket_reopen`/`ticket_close` call was possible from here; the work is recorded in this file
instead.

## 1. Outcome

All four owners now register **every** scenario of **every** case in Go and agree with Rust and with
the third-party oracles, at `fundamental` and at `quick`:

```
$ SEMIO_TEST_BUDGET_MS=600000 GOWORK=C:/git/semio/go.work RUSTC_WRAPPER="" \
  bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner <owner>

=== parity fundamental 🎫️tickets
[test] level=fundamental cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
=== parity fundamental 🎯️goals
[test] level=fundamental cases=4 executed=9  passed=9  failed=0 errored=0 parity=6/6
=== parity fundamental 📝️todos
[test] level=fundamental cases=4 executed=11 passed=11 failed=0 errored=0 parity=7/7
=== parity fundamental 🧑️contributors
[test] level=fundamental cases=3 executed=7  passed=7  failed=0 errored=0 parity=5/5
```

```
=== parity quick 🎫️tickets
[test] level=quick cases=5 executed=59 passed=59 failed=0 errored=0 parity=43/43
=== parity quick 🎯️goals
[test] level=quick cases=4 executed=13 passed=13 failed=0 errored=0 parity=8/8
=== parity quick 📝️todos
[test] level=quick cases=4 executed=11 passed=11 failed=0 errored=0 parity=7/7
=== parity quick 🧑️contributors
[test] level=quick cases=3 executed=11 passed=11 failed=0 errored=0 parity=7/7
```

Before this session those four owners produced `executed=0 … not-exercised=N` for Go
(`📓️opus-goals-contributors-todos.md` §6, `📓️opus-tickets.md` §3.1). The 16 cases and 60 parity
comparisons above are new differential evidence: every `no-oracle` decision that names
"independent implementations" as its substitute is now discharged for these four owners.

## 2. What was built

### 2.1 `🧑️contributors` — `🚪️Ports` region (≈ 700 lines)

- `GitAuthor` + `ParseGitAuthor`, `ParseContributorIdentity` (the four-digit-run grammar).
- `ContributorStore` port with `StoredContributor`, `NewMemoryContributorStore`,
  `NewFsContributorStore`; `ListContributorsFrom`, `SearchContributorsFrom`,
  `ResolveAuthorToAlias`, `FindContributor`.
- `SessionSource` port with `SessionKey`, `MemorySession`, `MemorySessionEntry`,
  `NewMemorySessionSource`, `NewFsSessionSource`; `DeriveSessionKindFrom`,
  `ExtractSessionClientFrom`, `ExtractSessionSecondFrom`, `ExtractSessionCheckpointFrom`,
  `ListSessions`, `SearchSessions`, `RunningWindowSeconds`, `AgentEndedKind`,
  `Session.ID()`/`Session.URI()`.
- `CheckpointSource` port with `Checkpoint`, `NewMemoryCheckpointSource`,
  `NewFsCheckpointSource`, `ParseCheckpointLog`, `ListCheckpoints`, `SearchCheckpoints`,
  `CheckpointID`, `CheckpointLogFormat`.
- `InteractionOrigin`, `InteractionsAuthor`, `InteractionsCheckpoint`, `AttributeInteractions`.
- `Clock`/`FixedClock`/`SystemClock`, `MemoryEmitter`.
- `SessionKindEmoji` now reads the shared `🧬️schema/🔣️entity-emojis.json` through
  `identity.Entity`, which is what the Rust twin does (see §4.1).

### 2.2 `🎯️goals` — `🚪️Ports` region (≈ 1 200 lines)

- `GoalError` + `ErrorClass` with the nine refusal classes the Rust `GoalError::class` names.
- **`GoalStatus` as a closed enum** (`GoalStatusOpen`, `GoalStatusClosed`, `AllGoalStatuses`,
  `ParseGoalStatus`); `DecodeGoal` refuses a document whose `status` is outside it, and refuses a
  document missing any of `title`, `description`, `prompt`, `status`, `client`, `llm` — the
  members serde has no default for.
- `DecodeGoal`/`EncodeGoal`: two-space indentation, declaration order, trailing newline, and the
  `parent` member rewritten into its compose form, byte-for-byte with the committed documents.
- `GoalStore` port with `MemoryGoalStore` (+`Snapshot`) and `FsGoalStore`.
- `ManagementPort` with `NullManagement` and `RecordingManagement` (deterministic numbers, ordered
  call log); `MemoryEmitter`.
- `Goals` aggregate — `NewGoals(store, management, emitter, author)` with `List`, `Read`, `Search`,
  `Create`, `Change`, `Close`, `Reopen`, `Delete`. It touches no global: `workspace.RootDir` is
  never read on this path.
- Id scheme: `IsRootGoalID`/`IsFirstGenGoalID`/`IsDeeperGoalID`, `GoalPathToComposeID`,
  `ComposeIDToGoalPath`, `RootGoalID`, `ParentGoalID`, `ComposeGoalID`, `ParseMilestoneNumber`,
  `ParseIssueNumber`.
- Tree: `GoalSeed`, `TicketSeed`, `TreeFormat`, `TreeLines`, `PlainTreeLines`, `BuildGoalTree`,
  `CountOpenSubgoals`, `CountOpenTickets`, `RenderGoalTree`.
- Reexports so an adapter needs one import: `Goal`, `GoalDates`, `GoalManagementData`, the five
  input types, `GoalNode`, `TicketNode`.

### 2.3 `📝️todos` — `🚪️Ports` region (≈ 900 lines)

- `TodoError` + `ErrorClass`.
- `TodoTree` port with `TreeEntry`, `TreeFile`, `NewMemoryTodoTree` (+`Snapshot`) and
  `NewFsTodoTree` (which applies the scan's own skip rules: dependency, build and dotted
  directories other than the repository meta root).
- **`ScanTodosIn(tree)` takes the filesystem port**; `SearchTodos(tree, term)`.
- **Non-mutating rewrites**: `ReplaceInMarkdown`, `RemoveFromMarkdown`, `ReplaceInFile`,
  `RemoveFromFile` are document-in/document-out and touch nothing. The path-taking
  `ReplaceLineInMarkdown`, `ReplaceLineInFile`, `RemoveLineFromMarkdown`, `RemoveLineFromFile` the
  CLI and GraphQL call are now **thin read-apply-write wrappers over them**, so the mutation lives
  in exactly one place and the grammar in another.
- `SplitTodoCommentParts` (prefix, name, description), `TodoCommentOpener`, `ParseTodoMarkdown`,
  `ParseTodoComments`, and the four vocabulary tables (`TodoMarkdownName`, `TodoMarkdownPrefix`,
  `TodoScanExtensions`, `TodoCommentOpeners`, `TodoSkippedDirectories`).
- `TicketOpener` port with `RecordingTicketOpener`; `Todos` aggregate
  (`NewTodos(tree, emitter, author)`, `List`, `Find`, `Create`, `Change`, `Delete`, `ToTicket`);
  `MemoryEmitter`.
- `DraftStore` port with `MemoryDraftStore` and `FsDraftStore`; `ListDraftsIn`, `CreateDraftIn`,
  `DeleteDraftIn`, `DraftID`, `DraftURI`.
- The legacy `ScanTodos(rootDir)` is now a wrapper over `ScanTodosIn(NewFsTodoTree(rootDir))` that
  re-absolutises the reported locations, so the CLI surface is unchanged.

### 2.4 `🎫️tickets` — `🚪️Ports` region (≈ 1 900 lines) and five brand-new Go adapters

The package exported **no types at all** before this session (`📓️opus-tickets.md` §3.1). It now
carries the whole Rust contract:

- `TicketError` (`invalid`/`not-found`/`store`/`conflict`) + `TicketErrorClass`.
- `TicketID` (both spellings, `ID`/`RelPath`/`URI`/`ParentSlug`, `ParseTicketID`), `TicketLayout`,
  `PadNumber`, `FormatYearDir`/`FormatMonthDir`/`FormatDayDir`, `ParseDatedDirSegment`,
  `NormalizeSeparators`, `JoinPath`, `BaseName`, `DirName`, `TicketSlugFromTitle`,
  `ValidateTicketEmojiTitle`, and the eight path constants.
- `GoJSONString` + a `MarshalIndent`-shaped renderer, `DecodeTicketDocument`,
  `EncodeTicketDocument`, `AppendSessionID`, `IsTicketInteractionKind`. The codec is written
  independently of `model.Ticket`'s own `MarshalJSON`/`UnmarshalJSON` on purpose — see §4.2.
- `TicketStore` port (`Metadata`, `Read`, `Write`, `CreateExclusive`, `CreateDirectory`,
  `RemoveFile`, `RemoveDirectory`, `RemoveTree`, `Rename`, `Entries`) with **`MemoryTicketStore`**
  (failure injection, sized files, seeded symlinks and directories, `Paths`, `Snapshot`) and
  **`FileTicketStore`**.
- The important-document transaction: `TransactionJournal`, `JournalStep`, `ImportantPreimage`,
  `ImportantCreation`, `InspectImportantDocument`, `RestoreImportantDocument`,
  `RemoveImportantDocument`, `EnsureImportantDocument`, `RollbackImportantCreation`.
- Plan resolution over the store: `PlanSource`, `PlanRoots`, `PlanClientTag`,
  `ResolvePlanSourceIn`, `ApplyPlanFromIDsIn`, `MovePlanIntoFolderIn`, `StripPlanFrontmatter`,
  `FormatPlanFileSection`.
- **`VersionControl` port** with `RecordedVersionControl`, `ParseDiffLines`, `IsRepoExcludedPath`,
  `NormalizeTicketFileInput(s)`, `FilterWorkspaceFilesUnder`, `TicketFileScope`,
  `ComputeTicketFileScope`, `CanCloseTicket`.
- **`IssueTracker` port** (declared here, narrow, and satisfied by
  `*providers.NullManagementProvider`) with `NullIssueTracker`, `IssueTrackerScript`,
  `RecordedIssueTracker` (+`RecordedIssueTrackerFromJSON`, `Calls`), `EnsureTicketIssue`,
  `CloseTicketIssue`, `MilestoneNumberForTitle`, `SyncOpenIssue`, `SyncCloseIssue`,
  `IssueSyncOutcome`, `FormatPromptHeading`, `FormatSummaryHeading`.
- `TicketClock`/`FixedTicketClock`/`SystemTicketClock`, `EventSink`/`RecordingEventSink`/
  `CoordinatorEventSink`, `TicketEventSource`.
- `TicketService` — `NewTicketService(layout, store, tracker, clock, sink)` with `Read`, `Save`,
  `List`, `FindBySlug`, `Latest`, `Search`, `Open`, `Close`, `Reopen`, `Change`, `PurgeArtifacts`,
  plus the four request types, the four `Parse*Request` readers, `TicketOutcome`, `PurgeReport`,
  `OversizedFileBytes`, `OversizedFolderBytes` and `TicketQuery`.
- Five new adapters: `🪪️ticket-id-scheme`, `📄️ticket-document-codec`, `🐙️issue-sync-transcripts`,
  `🔓️open-close-reopen-lifecycle`, `💾️important-document-transaction` — 25 scenarios, all
  registered, none skipped.

## 3. Verification (real output)

```
$ GOWORK=C:/git/semio/go.work  (in each package directory)
===== 🎫️tickets
build ok
vet ok
ok  	github.com/usalu/semio/repo/tickets	0.939s
===== 🎯️goals
build ok
vet ok
ok  	github.com/usalu/semio/repo/goals	0.408s
===== 📝️todos
build ok
vet ok
ok  	github.com/usalu/semio/repo/todos	0.794s
===== 🧑️contributors
build ok
vet ok
ok  	github.com/usalu/semio/repo/contributors	0.444s

$ (in 🎫️tickets) go vet -tags exhaustive ./...
tickets exhaustive vet ok

$ gofmt -l .    (all four packages, all adapters)
(no output)
```

Every dependent Go package still compiles against the new surfaces:

```
⌨️cli: build ok
🔗️graphql: build ok
🪝️hooks: build ok
🌳️tree: build ok
🏃️test-runner: build ok
🔌️mcp: build ok
```

Their own test suites still show the wave-1 divergences the split report attributes elsewhere
(`TestSectionNewlineAfterRegion`, `TestFix*`, `TestRenderMonorepoTree`, `TestMicroCommit…`); `⌨️cli`
and `🏃️test-runner` are fully green. None of those failures touches a symbol this session changed,
and `🌳️tree` and `🪝️hooks` have their own concurrent owners (`🗑️generated/go-align-tree`,
`🗑️generated/go-align-hooks`).

### 3.1 Pre-existing failures inside the owned packages — fixed

The split report listed `🎯️goals`, `📝️todos` and `🧑️contributors` as `fail (pre-existing)`. All
three were single tests asserting a godfile-era vocabulary the schema-first tables no longer carry:

| test | asserted | the frozen contract says |
| --- | --- | --- |
| `🧑️contributors::TestSessionKindEmoji` | `model.EmojiSessionRunning` = `🟡️` (with VS16) | `identity.Entity("session-running")` = `🟡`, because `🟡` is not in `textDefaultEmojis` |
| `🎯️goals::TestGoalIDForFilesystem` | a compose id resolves by scanning the goals directory back to `AI-OPTIMIZED-REPO/REPO-CLIENT` | the compose codec's own flattened segments, which is what `🪪️goal-id-scheme` pins for both languages |
| `📝️todos::TestEventIDsUseComposeRepoFormat` | a range ref carries `📌️` (with VS16) | `model.EmojiText(model.EmojiLine)` = `📌`, from the same shared table |

Each test now states the contract through the shared table rather than through a hard-coded
literal, so it follows the table if the table changes. `🎯️goals`' now-unused `findTestRepoRoot`
helper and its `os`/`filepath`/`runtime`/`workspace` imports were removed with it.

## 4. Divergences found, and how each was resolved

### 4.1 `identity.Flat` (Go) drops every non-ASCII character; `identity::flat` (Rust) keeps it

`🪪️identity`'s Rust `flat` keeps every code point above `0x7F`; the Go twin keeps only `a-z0-9`.
`🏠️workspace.Flat` (Go) is the one that matches Rust. The divergence is invisible for slugs but
decides `goal_path_to_compose_id` when the input **already** carries goal segments:

```
go   (identity.Flat):  🎯️aioptimizedrepo → 🎯aioptimizedrepo
rust (identity::flat): 🎯️aioptimizedrepo → 🎯🎯️aioptimizedrepo
```

This is what the first `🎯️goals` parity run failed on (`parity=5/6`, 3 differences). Rust is the
frozen contract, so `goals.GoalPathToComposeID` was written against `workspace.Flat` rather than
delegating to `identity.GoalPathToComposeID`, and `🧑️contributors` uses `workspace.Flat` for the
session and contributor segments for the same reason.

**Owner action (`foundation`): Go `identity.Flat` should keep code points above `0x7F` and lower-case
them, exactly as `workspace.Flat` and Rust `identity::flat` do; `identity.GoalPathToComposeID` and
`identity.ContributorToComposeID` are wrong for emoji input until it does.** I did not edit
`🪪️identity`.

### 4.2 `model.Ticket`'s codec is not the ticket document codec

Go's `model.Ticket.UnmarshalJSON` rewrites `goal` from its compose form into a filesystem path by
**scanning the live goals directory**, and rewrites contributor compose ids inside interactions;
`MarshalJSON` rewrites `goal` back and normalises session ids. Rust's `decode_ticket_document`/
`encode_ticket_document` do none of that — they read and write the member verbatim. The committed
fixtures store `goal` in compose form, so routing the codec through `model` would have made the
result depend on the developer's own checkout.

`tickets.DecodeTicketDocument`/`EncodeTicketDocument` are therefore written directly against the
document, produce a `*model.Ticket`, and reproduce Go's `MarshalIndent` bytes (HTML escaping
included) through an explicit renderer. `📄️ticket-document-codec` is `parity=12/12` against the
`ajv` oracle and the Rust subject over the six real committed documents.

### 4.3 `model.Checkpoint.Date` is `time.Time` in Go and `String` in Rust

Rust's `model::Checkpoint.date` is a `String` holding the `--date=iso-strict` text; Go's is a
`time.Time`, which cannot carry that text back out unchanged. `🧑️contributors` therefore declares
its own `Checkpoint` (`id`, `sha`, `title`, `authorId`, `date string`) — the shape Rust re-exports
— instead of aliasing `model.Checkpoint`.

**Owner action (`model`): `model.Checkpoint.Date` should be a string, as the Rust twin has it; until
then the Go checkpoint reader cannot round-trip a checkpoint log.** I did not edit `📐️model`.

### 4.4 Emitted payloads must be key-sorted, not declaration-ordered

Rust emits event payloads as `serde_json::Value::to_string()`. `serde_json`'s map is a `BTreeMap`
(the workspace does not enable `preserve_order`), so a payload is **alphabetically ordered and
compact, with no HTML escaping**. Go's `json.Marshal` of a struct is declaration-ordered and
HTML-escaping. Every emitter in the three lifecycle packages therefore renders through a
`canonicalPayload` helper: marshal → re-read as a generic value → re-encode with
`SetEscapeHTML(false)`, which sorts map keys the way Go always does. This is what makes the
`envelopes`/`events` projections agree.

### 4.5 Names that could not be taken, because a package I do not own calls them

`⌨️cli`, `🔗️graphql` and `🏃️test-runner` call the repository-global spellings of five names with a
different signature from the port-based twin. Renaming their call sites would mean editing packages
another agent owns concurrently, so the port keeps a distinct name and the global one is untouched:

| port-based twin (this session) | global twin (kept for the CLI) | external callers of the global |
| --- | --- | --- |
| `contributors.ListContributorsFrom(store)` | `contributors.ListContributors()` | `⌨️cli`, `🔗️graphql` |
| `goals.IsRootGoalID(id)` / `IsFirstGenGoalID` / `IsDeeperGoalID` | `goals.IsRootGoal(*model.Goal)` … | `🔗️graphql` |
| `todos.ScanTodosIn(tree)` | `todos.ScanTodos(rootDir)` | `🔗️graphql`, `🏃️test-runner` |
| `todos.ListDraftsIn(store)` / `CreateDraftIn` / `DeleteDraftIn` | `todos.ListDrafts()` / `CreateDraft` / `DeleteDraft` | `🔗️graphql` |
| `tickets.FilterWorkspaceFilesUnder(folder, files)` | `tickets.FilterTicketWorkspaceFiles(*model.Ticket, files)` | — (same package) |
| `tickets.ResolvePlanSourceIn` / `ApplyPlanFromIDsIn` | `tickets.ResolvePlanSource` / `ApplyTicketPlanFromIDs` | — (same package) |

**Follow-up for the `wiring`/`cli` wave**: once `⌨️cli` and `🔗️graphql` are repointed at the ports,
the global twins should be renamed (`ListRepoContributors`, `ScanRepoTodos`, …) and the port-based
twins should take the plain names, which is what the Rust crates carry. Nothing else about the
shape has to change.

## 5. What is left

1. The three follow-ups above: `identity.Flat` (§4.1), `model.Checkpoint.Date` (§4.3), and the
   rename sweep (§4.5). None of them blocks the cases; all three are cross-owner.
2. `🎫️tickets` has no `🐹️.go` oracle beyond the two it already had; the three `no-oracle` decisions
   `repo-ticket-id-scheme`, `repo-ticket-lifecycle` and `repo-ticket-important-document` now have
   their differential half and their rationales are accurate as written — no manifest edit needed.
3. The Go package files are large (`🎫️tickets/🐹️.go` is now ≈ 5 500 lines). Splitting the ports
   region out of the split-era region is natural once the CLI is repointed and the global twins go.
4. The `repo` MCP server needs fixing before a ticket can be closed from a session like this one.

## 6. Files touched

- `🔨️modules/🧑️contributors/📦️packages/🐹️go/{🐹️.go, go.mod, 🔬️_test.go}`
- `🔨️modules/🧑️contributors/🧪️tests/🪪️contributor-identity-parse/🐹️.go`
- `🔨️modules/🎯️goals/📦️packages/🐹️go/{🐹️.go, go.mod, 🔬️_test.go}`
- `🔨️modules/🎯️goals/🧪️tests/🪪️goal-id-scheme/🐹️.go`
- `🔨️modules/📝️todos/📦️packages/🐹️go/{🐹️.go, go.mod, 🔬️_test.go}`
- `🔨️modules/📝️todos/🧪️tests/{✏️draft-lifecycle,🔍️todo-scanning}/🐹️.go`
- `🔨️modules/🎫️tickets/📦️packages/🐹️go/{🐹️.go, go.mod}`
- `🔨️modules/🎫️tickets/🧪️tests/{🪪️ticket-id-scheme,📄️ticket-document-codec,🐙️issue-sync-transcripts,🔓️open-close-reopen-lifecycle,💾️important-document-transaction}/🐹️.go` — all five created
- this report

No git-modifying command was run.
