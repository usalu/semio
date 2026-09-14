# 📓️ Opus executor — `🎯️goals` + `🧑️contributors` + `📝️todos`

Wave 2 domain executor. This report resumes and completes the job a previous session started (it
was killed by a rate limit at the contributors "session-kind and checkpoint" cases). Everything
below was executed on this host; every quoted line is real command output.

## 1. State found on disk

| Module | Rust crate | Cases found | Fixtures | Schema | Oracle manifest |
| --- | --- | --- | --- | --- | --- |
| `🎯️goals` | `semio-framework-repo-goals`, 1195 lines, complete | 4 (`🪪️goal-id-scheme`, `📄️goal-document-codec`, `🌳️goal-tree-rendering`, `🔓️goal-lifecycle`) | 4 | ✓ | ✓ (`ajv-goal-document` + 3 no-oracle decisions) |
| `🧑️contributors` | `semio-framework-repo-contributors`, 756 lines, complete | 3 (`🪪️contributor-identity-parse`, `📡️session-kind-derivation`, `🏁️checkpoint-listing`) | 3 | ✓ (defective, see §2) | ✓ (`ajv-contributor-document` + 2 no-oracle decisions) |
| `📝️todos` | `semio-framework-repo-todos`, 768 lines, complete | **0** | **0** | **missing** | **missing** |

So the Rust half of all three crates was already written and compiling; the goals and contributors
test surfaces were complete except for one schema defect; the whole todos test surface was missing.

## 2. Defect found and fixed — the contributors schema rejected this repository's own documents

`🧑️contributors/🧬️schema/🔣️.json` declared `ContributorDocument.required` as
`["alias", "github", "name", "email"]`, transcribed from the Go struct tags (`email` carries no
`omitempty`). But `🧫️fixtures/🧑️‍💻️contributor-documents.json` holds the five *real* committed
documents of `.🧬semio/🦑️repo/🧑️‍💻️devs`, and three of them (`🦅️adrian`, `🦉️jonathan`,
`🦊️christian`) carry no `email` member at all. The `ajv` oracle therefore errored rather than
disagreeing — the failure was invisible in the summary line except as `errored=1`:

```
[test] level=fundamental cases=3 executed=1 passed=0 failed=0 errored=1 parity=0/0 not-exercised=2
```

with the diagnostic only in `📤️results.jsonl`:

```
🦅️adrian does not satisfy ContributorDocument: data must have required property 'email'
```

The schema's own `$comment` claims "a real document taken out of `.🧬semio/🦑️repo/🧑️‍💻️devs`
validates against it unchanged", so the schema was wrong, not the fixture. Fixed by requiring only
the members every stored document actually carries (`alias`, `github`, `name`) and stating in the
`$comment` that the encoder always writes `email` while hand-written documents omit it. After the
fix:

```
[test] level=fundamental cases=3 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=2
```

This is exactly the drift the ajv oracle exists to catch, and it caught it.

## 3. What was built for `📝️todos`

The Rust crate was already port-based (`TodoTree`, `TicketOpener`, `DraftStore`, `Emitter`, each
with an in-memory and a filesystem implementation). Added around it:

- `🧬️schema/🔣️.json` — `Todo`, `Location`, `Draft`, `TreeFile`, and the four fixture shapes
  (`ScanTreeVectors`, `LineVectors`, `TodoScript`, `DraftScript`).
- `🧫️fixtures/🔍️scan-tree.json` — a whole tree stated as its own files: a directory `.todos.md`,
  an item with no description, a description carrying a colon, a nested item, comments under each
  of the three openers, an unsupported extension (`.sql`), a plural keyword, a comment with no
  colon, a markdown file that is not a `.todos.md`, and the three directory kinds a filesystem
  scan refuses (`node_modules`, `dist`, dotted).
- `🧫️fixtures/📝️line-vectors.json` — the two documents a todo can live in plus every rewrite,
  removal and refusal (a name no line carries, a line number past the end, the one-based zero).
- `🧫️fixtures/🔓️lifecycle-script.json` — one ordered history: create under a directory, create
  under a file, create under a non-existent parent, change by name only, change by description
  only, change an unknown id, promote to a ticket, promote the same todo again, delete, delete
  again.
- `🧫️fixtures/✏️draft-script.json` — create, duplicate slug, title carrying no slug, a deep source
  path collapsing to a base name, delete, delete again.
- `🔮️oracle/🔣️.json` — one oracle (`ajv-todo-line-vectors`) and three recorded no-oracle
  decisions (`repo-todos-scanning`, `repo-todos-lifecycle`, `repo-todos-drafts`), each stating
  honestly that this checkout carries no `.todos.md` and only two genuine todo comments (both
  inside test literals of `🔗️graphql`/`💻️client`), so the vectors are specification vectors.
- Four cases with `🥒️.feature`, `🦀️.rs` and `🐹️.go`, and a `🟦️.ts` ajv oracle for the round-trip
  case:
  `🔍️todo-scanning` (2 scenarios: in-memory scan+search, and a real filesystem scan that must
  refuse the three directory kinds), `📝️todo-markdown-roundtrip`, `🎫️todo-to-ticket`,
  `✏️draft-lifecycle`.

The TypeScript oracle restates the line grammar with JavaScript regular expressions (a different
engine from either hand-written scanner), re-derives every rewrite and removal from the committed
documents, and additionally validates the fixture against `LineVectors` with `ajv`. It agreed with
the Rust subject byte for byte on the first run — `parity=1/1` below.

## 4. Clippy

`cargo clippy` reported five warnings in crates I own; all five fixed:

- `🎯️goals`: `map(…).unwrap_or_else(…)` → `map_or_else`; `nest_tickets(Vec<TicketNode>)` →
  `&[TicketNode]` (with both call sites); `sort_goal_nodes(&mut Vec<GoalNode>)` → `&mut [GoalNode]`.
- `📝️todos`: two redundant clones and one redundant field name.

Remaining warnings in the workspace come from `🏠️workspace` and `🪪️identity`, which belong to the
`foundation` executor — not touched.

```
$ RUSTC_WRAPPER="" cargo clippy -p semio-framework-repo-goals -p semio-framework-repo-contributors -p semio-framework-repo-todos --all-targets
warning: `semio-framework-repo-workspace` (lib) generated 1 warning
warning: `semio-framework-repo-identity` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s)
```

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-goals -p semio-framework-repo-contributors -p semio-framework-repo-todos
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (×6: lib + doc, three crates)
```

The crates carry no `#[cfg(test)]` unit tests by design — the behaviour is proven by the
language-agnostic harness cases, not by per-language duplicates.

## 5. Harness — real output

`SEMIO_TEST_BUDGET_MS=600000`, `RUSTC_WRAPPER=""`, run from the repository root.

### `subject … --implementation rust`

```
=== subject 🎯️goals
[test] level=fundamental cases=4 executed=4 passed=4 failed=0 errored=0 parity=0/0
=== subject 🧑️contributors
[test] level=fundamental cases=3 executed=3 passed=3 failed=0 errored=0 parity=0/0
=== subject 📝️todos
[test] level=fundamental cases=4 executed=5 passed=5 failed=0 errored=0 parity=0/0
```

### `oracle …`

```
=== oracle 🎯️goals
[test] not-exercised …/🪪️goal-id-scheme (recorded no-oracle decision repo-goals-id-scheme — its evidence is discharged by the subject phase)
[test] not-exercised …/🔓️goal-lifecycle (recorded no-oracle decision repo-goals-lifecycle — …)
[test] not-exercised …/🌳️goal-tree-rendering (recorded no-oracle decision repo-goals-tree — …)
[test] level=fundamental cases=4 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=3
=== oracle 🧑️contributors
[test] not-exercised …/🏁️checkpoint-listing (recorded no-oracle decision repo-contributors-checkpoints — …)
[test] not-exercised …/📡️session-kind-derivation (recorded no-oracle decision repo-contributors-sessions — …)
[test] level=fundamental cases=3 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=2
=== oracle 📝️todos
[test] not-exercised …/✏️draft-lifecycle (recorded no-oracle decision repo-todos-drafts — …)
[test] not-exercised …/🔍️todo-scanning (recorded no-oracle decision repo-todos-scanning — …)
[test] not-exercised …/🎫️todo-to-ticket (recorded no-oracle decision repo-todos-lifecycle — …)
[test] level=fundamental cases=4 executed=1 passed=1 failed=0 errored=0 parity=0/0 not-exercised=3
```

### `parity …`

```
=== parity 📝️todos
[test] level=fundamental cases=4 executed=6 passed=6 failed=0 errored=0 parity=1/1
[test] …/✏️draft-lifecycle: no-oracle decision repo-todos-drafts claims the independent-implementations substitute but only one implementation ran
[test] …/🔍️todo-scanning: no-oracle decision repo-todos-scanning claims the independent-implementations substitute but only one implementation ran
[test] …/🎫️todo-to-ticket: no-oracle decision repo-todos-lifecycle claims the independent-implementations substitute but only one implementation ran
```

The one parity pair that could run — Rust subject against the `ajv` TypeScript oracle for
`📝️todo-markdown-roundtrip` — is equal. The three no-oracle cases are *not yet* discharged,
because their `independent-implementations` substitute needs the Go subject, which cannot compile
yet (§6). The same holds for `🎯️goals` and `🧑️contributors`.

### `contract`

Run repo-wide and filtered for these three owners:

```
$ bun ./…/🧪️test/📜️script.ts contract | grep -i "goals\|contributors\|todos"
(no output)
```

No contract breach is attributable to any of the three owners. The breaches the phase does report
(`temp`, `🧰️framework`, `.storybook`, `✏️s`, `♻️mit-bestand` discovery counts, and a `🗒️note`
plugin mutation vector) predate this work and belong elsewhere.

## 6. Go subjects — honest status: all three still fail to compile

Per the brief the Go packages under `📦️packages/🐹️go` belong to the concurrent `go-split`
executor and were **not edited**. The Go adapters under `🧪️tests/<case>/🐹️.go` (mine) are written
against the port-based Go twin the plan calls for — the same shape as the Rust crates — and the Go
packages are currently still the raw AST split of `component.go`, so every Go subject fails at
compile time. Run last, with `GOWORK=C:/git/semio/go.work`:

```
=== go subject 🎯️goals
[test] level=fundamental cases=4 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=4
.\adapter.go:46:22: undefined: goals.DecodeGoal
.\adapter.go:54:25: undefined: goals.EncodeGoal
.\adapter.go:51:21: cannot use path (variable of type string) as *model.Goal value in argument to goals.IsRootGoal
.\adapter.go:55:10: undefined: goals.RootGoalID
.\adapter.go:57:21: undefined: goals.GoalPathToComposeID
.\adapter.go:62:86: undefined: goals.ComposeGoalID
.\adapter.go:67:27: undefined: goals.ParseMilestoneNumber
.\adapter.go:58:17: undefined: goals.NewMemoryGoalStore
.\adapter.go:61:21: undefined: goals.NewGoals
.\adapter.go:73:40: undefined: goals.GoalCreateInput
.\adapter.go:79:47: undefined: goals.ErrorClass

=== go subject 🧑️contributors
[test] level=fundamental cases=3 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=3
.\adapter.go:48:25: undefined: contributors.NewMemoryCheckpointSource
.\adapter.go:58:75: undefined: contributors.ParseCheckpointLog
.\adapter.go:57:24: undefined: contributors.NewMemoryContributorStore
.\adapter.go:63:35: undefined: contributors.ParseContributorIdentity
.\adapter.go:83:26: undefined: contributors.ParseGitAuthor
.\adapter.go:28:58: undefined: contributors.MemorySessionSource

=== go subject 📝️todos
[test] level=fundamental cases=4 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=4
.\adapter.go:81:16: undefined: todos.NewMemoryTodoTree
.\adapter.go:55:28: undefined: todos.TodoTree
.\adapter.go:39:42: undefined: todos.TreeFile
.\adapter.go:89:26: undefined: todos.ReplaceInMarkdown
.\adapter.go:112:49: undefined: todos.RemoveFromFile
.\adapter.go:118:42: undefined: todos.SplitTodoCommentParts
.\adapter.go:128:62: undefined: todos.TodoCommentOpener
.\adapter.go:49:17: undefined: todos.NewMemoryDraftStore
.\adapter.go:70:62: undefined: todos.DraftID
```

Note the import resolution itself works — the harness's generated Go host does reach
`github.com/usalu/semio/repo/<suffix>` through `go.work`, so the gap flagged in
`📓️harness-verification.md` §2 has been closed by someone. Only the symbols are missing.

### What `go-split` must export for these adapters to compile

- `🎯️goals`: `DecodeGoal`, `EncodeGoal`, `GoalDepth`, `IsRootGoal(string)`, `IsFirstGenGoal(string)`,
  `IsDeeperGoal(string)` (**currently `*model.Goal`-taking — a real signature divergence, not just a
  missing symbol**), `GoalIDForFilesystem`, `RootGoalID`, `ParentGoalID`, `GoalPathToComposeID`,
  `ComposeIDToGoalPath`, `ComposeGoalID`, `ParseMilestoneNumber`, `ParseIssueNumber`,
  `NewMemoryGoalStore`, `NewRecordingManagement`, `NewMemoryEmitter`, `NewGoals`, `ErrorClass`,
  `GoalCreateInput`/`GoalChangeInput`/`GoalCloseInput`/`GoalReopenInput`/`GoalDeleteInput`,
  `GoalSeed`, `TicketSeed`, `BuildGoalTree`, `CountOpenSubgoals`, `CountOpenTickets`,
  `RenderGoalTree`, `TreeFormatText`/`TreeFormatMarkdown`, `PlainTreeLines`.
- `🧑️contributors`: `ParseGitAuthor`, `ParseContributorIdentity`, `StoredContributor`,
  `NewMemoryContributorStore`, `ResolveAuthorToAlias`, `MemorySessionSource`, `MemorySessionEntry`,
  `MemorySession`, `SessionKey`, `NewMemorySessionSource`, `ListSessions`, `Checkpoint`,
  `NewMemoryCheckpointSource`, `ParseCheckpointLog`, `ListCheckpoints`, `SearchCheckpoints`,
  `CheckpointID`.
- `📝️todos`: `TreeFile`, `TreeEntry`, `TodoTree`, `NewMemoryTodoTree`, `NewFsTodoTree`,
  `ScanTodos(TodoTree)` (**the current `ScanTodos(rootDir string)` must move behind the port**),
  `SearchTodos`, `ParseTodoMarkdown`, `ParseTodoComments`, `SplitTodoCommentParts`,
  `TodoCommentOpener`, `ReplaceInMarkdown`, `RemoveFromMarkdown`, `ReplaceInFile`,
  `RemoveFromFile` (**the current `ReplaceLineIn*`/`RemoveLineFrom*` are path-taking and mutate
  files; the twin must be document-taking and pure**), `ErrorClass`, `NewMemoryEmitter`,
  `NewRecordingTicketOpener`, `NewTodos`, `TodoCreateInput`, `TodoChangeInput`,
  `NewMemoryDraftStore`, `ListDrafts`, `CreateDraft(store, title, files)`,
  `DeleteDraft(store, id)`, `DraftID`, `DraftURI`.

## 7. launch seed

Nine entries added to `.vscode/🧩️launch.seed.jsonc`, immediately after the `🚚️move` block,
following the existing grouping and naming:

`🧪️test🧰️repo{🎯️goals,🧑️contributors,📝️todos}{🦀️rust,🐹️go,🥒️parity}` — the Rust and Go ones
call `bun nx run <project>:test`, the parity ones call the harness with `--owner`. Regenerated:

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
$ grep -c "…goals\|…contributors\|…todos" .vscode/launch.json
9
```

## 8. What is left

1. **`go-split` must land the port-based Go twins** listed in §6. Until then the three `🥒️parity`
   launch entries and the `independent-implementations` substitute of six no-oracle decisions
   (three for goals, two for contributors, three for todos) stay undischarged. Nothing else blocks
   them: the fixtures, features and Go adapters are committed and the harness reaches the Go
   modules.
2. `🎯️goals`/`🧑️contributors`/`📝️todos` test case directories already carry the leading emoji the
   coordinator's 2026-09-06 06:10 decision requires, so the audit wave's rename sweep does not have
   to touch them.
3. `📝️todos`' fixtures are specification vectors, stated as such in the no-oracle rationales. If a
   `.todos.md` ever lands in this repository, `🔍️scan-tree.json` should gain a recorded copy of it
   next to the stated tree.
