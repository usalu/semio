# Go CLI Mutation Resolvers — Investigation & Fix

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go` (package `client`).

## Key finding: the premise was mostly stale

The ticket description assumed all 25 named `mutationResolver` methods (lines
~33753–34026) were dead stubs returning `fmt.Errorf("not implemented")`. On
inspection, **each of these methods already has the shape**:

```go
func (r *mutationResolver) X(ctx context.Context, ...) (..., error) {
	if r.Ctx != nil {
		return r.Ctx.X(...)
	}
	return nil/false, fmt.Errorf("not implemented")
}
```

The `fmt.Errorf("not implemented")` only fires when `r.Ctx` (the
`RepoContext` interface) is `nil`. In production `r.Ctx` is always set —
`NewResolver(rootDir)` sets `Ctx: NewRepoContext(rootDir)` and
`NewResolverWithContext` takes an explicit non-nil context — so that branch
is an unreachable defensive guard, not a stub. The real implementation lives
one level down, on `*repoContext` (the `RepoContext` interface
implementation returned by `NewRepoContext`), which is also what every
`ticketCommand`/`goalCommand`/`todoCommand`/... cobra subcommand ultimately
calls through `runGraphQL` → the same GraphQL executor → the same resolver →
the same `repoContext` method. There is no separate "imperative engine" to
delegate to — the resolver *is* the shared implementation, and the cobra
commands already go through it via GraphQL. So there was nothing to
de-duplicate.

Checking all 25 `*repoContext` methods individually, **24 of them were
already fully, correctly implemented** (real file I/O, git/GitHub calls,
event emission, etc. — not placeholders). Only **one was a genuine stub**:

- `(*repoContext).TodoChange` — `return nil, fmt.Errorf("not implemented")`
  (was at line 40524).

This is the only resolver in the list that needed real work. (The unrelated,
separately-flagged `defaultContext` implementations, e.g.
`(*defaultContext).TodoChange` at line 30203, are a deliberate null-object
context used elsewhere and were left untouched — that's their intended
no-op behavior, not part of this ticket's scope.)

## What was implemented

`(*repoContext).TodoChange(input TodoChangeInput) (*Todo, error)` in
`🐹️component.go` (currently ~line 40522), following the exact pattern of the
sibling `TodoCreate`/`TodoDelete` methods (same file, ~line 40475 and
~40571):

1. Locates the todo by `input.ID` via the existing `ScanTodos(c.rootDir)`
   (shared with `GetTodos`/`TodoDelete`).
2. Returns `"todo not found"` if no match, `"todo has no location"` if the
   scanned todo lacks a `Location`.
3. Merges `input.Name`/`input.Description` (pointers, optional) onto the
   existing values.
4. Rewrites the source in place:
   - If the todo lives in a `.todos.md` file (`- TODO Name: Description`
     line format), rewrites that one line via a new helper
     `replaceLineInMarkdown(path, oldName, newName, newDescription)`.
   - If the todo is an inline code comment (`// TODO Name: Description`,
     `#`/`--` variants), rewrites that one line via a new helper
     `replaceLineInFile(path, lineNum, newName, newDescription)`, using the
     same regex shape as the existing `ParseTodoComments` parser so the
     comment marker/indentation is preserved.
5. Emits `repopkg.EventTodoChangeEnded` with `repopkg.TodoChangePayload`
   (`TodoPayload` + `Name`/`Description` pointers) — the same event/payload
   types already used by `TicketChange`/`GoalChange`, and already defined in
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🐹️component.go` (no new
   types added).
6. Returns the updated `*Todo`.

New helper functions `replaceLineInMarkdown` and `replaceLineInFile` were
added right next to the existing `removeLineFromMarkdown`/
`removeLineFromFile` helpers (same file, ~line 40598+), mirroring their
style (best-effort, no external deps).

No GraphQL schema, resolver signature, or `RepoContext` interface change was
needed — `TodoChangeInput`, `Todo`, and the interface method already existed
and were already wired through `mutationResolver.TodoChange` →
`r.Ctx.TodoChange`.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️component.go`
  - Implemented `(*repoContext).TodoChange` (was a stub at line 40524).
  - Added `replaceLineInMarkdown` and `replaceLineInFile` helpers.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`
  - Added permanent test `TestRepoContextTodoChange` (after
    `TestToolDraftLifecycle`), covering the markdown-entry rewrite, the
    inline-comment rewrite, and the not-found error path — see §3 above.

No other files were changed.

## Full resolver → implementation map (all 25, verified real)

| Resolver | `mutationResolver` (delegates to `r.Ctx.X`) | `repoContext` implementation | Status |
|---|---|---|---|
| SyncManagement | :33753 | :29755 | already real (GitHub label/milestone/issue sync, ~250 lines) |
| DraftCreate | :33775 | :28048 (`CreateDraft`) | already real |
| DraftDelete | :33784 | :28054 (`DeleteDraft`) | already real |
| TicketOpen | :33793 | :29386 (`OpenTicket`) | already real |
| TicketClose | :33801 | :29397 (`FinishTicket` incl. bulk-close) | already real |
| TicketReopen | :33810 | :29455 (`ReopenTicket`) | already real |
| TicketChange | :33819 | :27897 | already real |
| TicketDelete | :33901 | :28012 | already real |
| GoalCreate | :33828 | :27505 | already real |
| GoalClose | :33847 | :27779 | already real |
| GoalReopen | :33883 | :27826 | already real |
| GoalDelete | :33892 | :27975 | already real |
| TodoCreate | :33856 | :40475 | already real |
| **TodoChange** | :33865 | :40522 | **implemented in this ticket (was a stub)** |
| TodoDelete | :33874 | :40571 | already real |
| ContributorAdd | :33910 | :29629 | already real |
| ContributorRemove | :33919 | :29683 | already real |
| FolderMove | :33938 | :29489 (`ToolFolderMove`) | already real |
| FolderDelete | :33947 | :29500 (`ToolFolderDelete`) | already real |
| FileMove | :33966 | :29521 (`ToolFileMove`) | already real |
| FileDelete | :33975 | :29532 (`ToolFileDelete`) | already real |
| SectionMove | :33994 | :29558 (`ToolSectionMove`) | already real |
| SectionDelete | :34003 | :29570 (`ToolSectionDelete`) | already real |
| Integrate | :34013 | :29580 (`ToolIntegrate`) | already real |
| Extract | :34022 | :29606 (`ToolExtract`) | already real |

Line numbers are from the file as of this ticket's final state (after the
~50-line `TodoChange` implementation was inserted around line 40522; the
`mutationResolver` lines above shifted down slightly from the ticket's
original 33757–34026 range because two smaller unrelated additions were
made in between — the delta is a few lines, not structural).

## Verification (commands actually run, with real output)

### 1. `go build`

```
cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli" && go build ./...
```
Output: none (clean, exit 0).

### 2. `go vet`

```
cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli" && go vet ./...
```
Output: none (clean, exit 0).

### 3. Package tests

> **Pre-existing failures / timeouts — read this before re-investigating.**
> The following were observed while verifying this ticket and **pre-date
> this change**; do not attribute them to `TodoChange` or this ticket:
> - `TestExhaustiveFoldersNonEmpty` — does not finish inside Go's own
>   10-minute (`600s`) default test timeout, because it walks
>   `GetFolders` → `ScopeToFiles` → `globByExtension` → `filepath.WalkDir`
>   over the entire ~3.4M-uloc monorepo with no caching/short-circuit. This
>   is a plain, unmodified query-resolver test; nothing in this ticket
>   touches that code path. The same full-repo-walk cost independently
>   shows up in `ScanTodos` (used by `TodoChange`/`TodoDelete`/`GetTodos`):
>   a live `todos` GraphQL query against the real repo root did not return
>   within 180s either (see §4 below).
> - `TestGraphQLFixMutation` — fails because the `fix` mutation was
>   intentionally removed elsewhere ("fix was removed; handle autofix
>   inside script.ts policy export"); the test is stale, unrelated to any
>   resolver in this ticket.
> - `TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement` — fails on
>   a ticket-open response URI double-prefix bug (produces
>   `file:///Users/ueli/Documents/semio/Users/ueli/Documents/semio/...`),
>   unrelated to `TodoChange`/mutation-resolver delegation.
> - `TestExhaustiveSectionCommands/Go` — fails on Go-language section
>   extract/rename ("known issue with mod-based section format" is also
>   separately noted for Rust in the same test), unrelated to todos.
>
> None of these four exercise `TodoChange`, `replaceLineInMarkdown`, or
> `replaceLineInFile`. They are candidates for their own follow-up tickets,
> not something this ticket fixed or should be blamed for.

`go test ./...` with no `-run` filter hit the `TestExhaustiveFoldersNonEmpty`
timeout described above.

Given that, tests were run targeted (`-run`) against the mutation-resolver
and CRUD test surface most relevant to this change and its neighbours,
`-timeout 300s`:

```
go test -run 'TestFolderCreateMoveDelete|TestFileCreateMoveDelete|TestGoalCreateAndCleanup|TestGraphQLFixMutation|TestCliWrongArgs_TodoOperations|TestToolFolderCRUD|TestToolFileCRUD|TestToolDraftLifecycle|TestSyncCommandRunsGitHubSynchronization|TestGraphQLEffortMutationsAndQueries|TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement|TestExhaustiveCliE2E_GoalLifecycle_Syntaxes_NoManagement|TestExhaustiveSectionCommands|TestCliWrongArgs_FolderOperations|TestCliWrongArgs_FileOperations|TestCliWrongArgs_SectionOperations|TestCliWrongArgs_ContributorOperations' -timeout 300s -v ./...
```

Result: 13 PASS, 3 FAIL. The 3 failures are exactly the pre-existing
`TestGraphQLFixMutation`, `TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement`,
and `TestExhaustiveSectionCommands/Go` failures called out in the box above
— none of them exercise `TodoChange`.

There was no pre-existing dedicated `TodoChange`/todo-lifecycle test in the
suite (`grep -n "func Test.*Todo"` only turned up ID-formatting and
wrong-args tests). A permanent test,
`TestRepoContextTodoChange`, was added to
`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️component_test.go`
(inserted directly after `TestToolDraftLifecycle`, following that file's
`t.TempDir()`-isolated CRUD-test convention, e.g. `TestToolFolderCRUD`/
`TestToolFileCRUD`). It constructs a `&repoContext{rootDir: t.TempDir()}`
directly — bypassing the package-global `rootDir`/`setupToolTest` and, more
importantly, bypassing any walk of the real monorepo — and covers all three
paths proved during initial verification:
- Renaming a `.todos.md`-backed todo (name + description), asserting the
  file line was rewritten and the old name is gone.
- Renaming an inline `// TODO Name: Desc` code-comment todo, asserting the
  comment marker/indentation was preserved and only the name/description
  changed.
- A not-found ID returning a non-nil error.

Because it is `t.TempDir()`-isolated it does not hit `ScanTodos`'s full-repo
walk cost, so it needs no `-short`/slow-test guard — it runs in well under a
second.

```
go test -run TestRepoContextTodoChange -v ./... 2>&1 | tail -20
```
Output:
```
=== RUN   TestRepoContextTodoChange
--- PASS: TestRepoContextTodoChange (0.40s)
PASS
ok  	github.com/usalu/semio/repo/client	0.894s
testing: warning: no tests to run
PASS
ok  	github.com/usalu/semio/repo/client/internal/command	0.293s [no tests to run]
testing: warning: no tests to run
PASS
ok  	github.com/usalu/semio/repo/client/internal/eventstore	0.760s [no tests to run]
?   	github.com/usalu/semio/repo/client/internal/glob	[no test files]
?   	github.com/usalu/semio/repo/client/internal/graphql	[no test files]
?   	github.com/usalu/semio/repo/client/internal/humanize	[no test files]
?   	github.com/usalu/semio/repo/client/internal/id	[no test files]
?   	github.com/usalu/semio/repo/client/internal/ignore	[no test files]
?   	github.com/usalu/semio/repo/client/internal/mcp	[no test files]
?   	github.com/usalu/semio/repo/client/internal/mcpserver	[no test files]
?   	github.com/usalu/semio/repo/client/internal/search	[no test files]
?   	github.com/usalu/semio/repo/client/internal/templatefunc	[no test files]
?   	github.com/usalu/semio/repo/client/internal/yaml	[no test files]
```
`go build ./...` and `go vet ./...` were re-run after adding this test and
are both clean.

(An earlier, throwaway version of this same test,
`🧪️todochange_verify_test.go`, was written, run, and deleted during initial
investigation before this permanent test was added — it is no longer in the
tree; `TestRepoContextTodoChange` above is its permanent replacement, per
CLAUDE.md's "at least one test per feature" rule.)

### 4. `client` (MCP) binary build + end-to-end GraphQL smoke test

```
cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp" && go build -o <scratchpad>/client-check .
```
Clean build, no output.

End-to-end `TodoCreate` against the real repo (throwaway target, inside this
ticket's own folder so it's discoverable and safely disposable):

```
<scratchpad>/client-check --repo /Users/ueli/Documents/semio graphql \
  --query 'mutation TodoCreate($input: TodoCreateInput!) { todoCreate(input: $input) { id name description parentId } }' \
  --vars '{"input":{"parentId":".../STUBS-AND-PLACEHOLDERS-COMPLETION","name":"E2ESmokeCheck","description":"original desc"}}'
```
Output (CLI markdown rendering):
```
[.../STUBS-AND-PLACEHOLDERS-COMPLETION📝📝e2esmokecheck](repo://todo/.../STUBS-AND-PLACEHOLDERS-COMPLETION📝📝e2esmokecheck) - `E2ESmokeCheck`
```
and the created `.todos.md` file contained exactly:
```
- TODO E2ESmokeCheck: original desc
```
confirming `TodoCreate` (already-implemented, unrelated to this fix) works
against the live binary.

Follow-up attempts to query/change/delete that same todo through the live
binary (`todos` query, then `todoChange`) against the real
`--repo /Users/ueli/Documents/semio` root **did not complete** — both hit
120s/180s timeouts because `ScanTodos`/`GetFolders`-style full-repo walks
over this ~3.4M-uloc tree are simply very slow in this environment (see the
`TestExhaustiveFoldersNonEmpty` timeout above — a `go test` run of the
*existing, unmodified* `GetFolders` path didn't finish inside Go's own
600s test timeout either). This is a pre-existing performance
characteristic of `ScanTodos`/`GetFolders`/`ScopeToFiles`
(`filepath.WalkDir` over the whole repo, no caching, no ignore-list
short-circuit visible in the hot path) that predates this change and
affects `TodoDelete`/`GetTodos` equally — not something introduced by the
`TodoChange` fix. It is a known limitation worth a follow-up ticket, not
something fixed here (out of scope: this ticket is about implementing the
stub, not about `ScanTodos` performance).

Because the live end-to-end query/change/delete calls could not complete in
reasonable time, the throwaway `.todos.md` created above was cleaned up
directly (`rm` the untracked file — verified via
`git status --porcelain` that it was untracked and created by this session,
alongside three other untracked ticket files from a concurrent session that
were left untouched). The isolated `t.TempDir()`-based Go test above is what
actually proves `TodoChange`'s correctness (both markdown and inline-comment
paths, plus the error path); the live binary smoke test only proves
`TodoCreate` (pre-existing) still works end-to-end.

## What was not finished, and why

- **Live end-to-end `TodoChange`/`TodoDelete` smoke test against the real
  monorepo root** could not be completed within the session's practical time
  budget because `ScanTodos` walks the entire ~3.4M-uloc tree and did not
  return within 180s (and the analogous `GetFolders` walk didn't return
  within Go's own 600s test timeout either — see the pre-existing-failures
  box in §3). Correctness is instead proven by the permanent
  `TestRepoContextTodoChange` unit test (§3), which is
  `t.TempDir()`-isolated by design specifically to avoid this walk. This is
  a pre-existing `ScanTodos`/`GetFolders` performance characteristic, not a
  gap in the `TodoChange` implementation itself, and would be a reasonable
  follow-up ticket on its own (e.g. respecting `.gitignore`/ignore-lists in
  the walk, or caching).
- Four pre-existing, unrelated failures/timeouts were observed
  (`TestExhaustiveFoldersNonEmpty` 10-minute timeout,
  `TestGraphQLFixMutation`, `TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement`,
  `TestExhaustiveSectionCommands/Go`) — see the box at the top of §3 for
  details. Left as-is, out of scope for this ticket, and not touched. Please
  do not mistake these for regressions introduced here.
- No other resolver in the list of 25 needed changes — they were already
  fully implemented, contrary to the ticket's premise.
