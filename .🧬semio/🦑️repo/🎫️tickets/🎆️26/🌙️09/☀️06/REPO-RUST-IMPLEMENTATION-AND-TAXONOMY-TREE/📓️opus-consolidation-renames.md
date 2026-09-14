# 📓️ Opus executor — consolidation renames (divergence item (f))

Host: Windows, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`.
Scope: Go only. No Rust crate, `🥒️.feature`, `🧫️fixtures` or `🧬️schema` file was touched, and neither
`📊️metrics` nor `📜️statutes` was opened.

## 1. Outcome

Every one of the five families now carries the **plain** name the Rust crate carries, on the
**port-based** signature. The repository-global twins are gone — deleted outright, with no alias, no
wrapper and no shim. The follow-up recorded in §4.5 of
`📓️opus-go-align-tickets-goals-todos-contributors.md` is closed.

| Rust (contract) | Go now | global twin that was deleted |
| --- | --- | --- |
| `list_contributors(store)` | `contributors.ListContributors(store ContributorStore) []model.Contributor` | `ListContributors() ([]model.Contributor, error)` |
| `is_root_goal(goal_id)` / `is_first_gen_goal` / `is_deeper_goal` | `goals.IsRootGoal(goalID string) bool` / `IsFirstGenGoal` / `IsDeeperGoal` | `IsRootGoal(*model.Goal)` / `IsFirstGenGoal(*model.Goal)` / `IsDeeperGoal(*model.Goal)` |
| `scan_todos(tree)` | `todos.ScanTodos(tree TodoTree) []*model.Todo` | `ScanTodos(rootDir string) ([]*model.Todo, error)` |
| `list_drafts(store)` / `create_draft` / `delete_draft` | `todos.ListDrafts(store DraftStore)` / `CreateDraft(store, title, files []TreeFile)` / `DeleteDraft(store, id)` | `ListDrafts()` / `CreateDraft(title, files []string)` / `DeleteDraft(id)` |
| `filter_ticket_workspace_files(folder_path, files)` | `tickets.FilterTicketWorkspaceFiles(folderPath string, files []string) []string` | `FilterTicketWorkspaceFiles(*model.Ticket, []string)` |
| `resolve_plan_source(store, roots, kind, id)` / `apply_ticket_plan_from_ids(...)` | `tickets.ResolvePlanSource(store, roots, kind, id) (PlanSource, error)` / `ApplyTicketPlanFromIDs(store, roots, ticket, kind, planID, specID)` | `ResolvePlanSource(kind, id) (string, bool, error)` / `ApplyTicketPlanFromIDs(ticket, kind, planID, specID)`, plus the private `planClientTag` it used |

Nothing turned out to be impossible to delete. Where a global twin did real work the port does not do
(host path resolution, host file reading, host roots), that work moved into a *separately named,
single-purpose* helper rather than into a wrapper around the port:

- `todos.LoadTreeFiles(paths []string) ([]TreeFile, error)` — reads host files into `TreeFile` values
  under their base name. Replaces the `move.CopyFile` loop the old `CreateDraft` did; the `move`
  import left `📝️todos` with it.
- `todos.LocateTodosOnDisk(root string, found []*model.Todo) []*model.Todo` — rewrites the tree
  relative `ParentID` / `Location.FilePath` of scanned todos into host paths. This is exactly the
  half of the old `ScanTodos(rootDir)` that was not the scan; `🔗️graphql` and `🏃️test-runner` need it
  because they read and rewrite the real files afterwards.
- `tickets.HostPlanRoots() PlanRoots` — `{RepoRoot: workspace.RootDir, HomeDir: os.UserHomeDir()}`,
  the roots the split-era `OpenTicket` / `ReopenTicket` pass to the port.
- `contributors.hostContributors()` (unexported) —
  `ListContributors(NewFsContributorStore(workspace.GetRootDir()))`, used by the five in-package
  split-era callers and by the `model.LookupContributors` port wiring.

## 2. Call sites repointed

`⌨️cli/📦️packages/🐹️go/🐹️.go`

- new package helper `draftStore() todos.DraftStore { return todos.NewFsDraftStore(workspace.GetDraftsPath()) }`
- `draftCommand` create → `todos.LoadTreeFiles(paths)` + `todos.CreateDraft(draftStore(), title, files)`
- `draftCommand` delete, `ToolDraftDelete` → `todos.DeleteDraft(draftStore(), slug)`
- `ToolDraftCreate` → `LoadTreeFiles` + `CreateDraft(draftStore(), …)` (its parameter is now `paths`)
- `ToolDraftList` → `todos.ListDrafts(draftStore())` (no error to unwrap any more)
- `ToolContributorList` → `contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(workspace.GetRootDir()))`
- `🔬️_test.go` `TestParityDraftList` → `todos.ListDrafts(draftStore())` in both subtests

`🔗️graphql/📦️packages/🐹️go/🐹️.go`

- two new unexported `RepoContext` methods: `scanTodos()`
  (`LocateTodosOnDisk(c.rootDir, ScanTodos(NewFsTodoTree(c.rootDir)))`) and `draftStore()`
- `GetTodos`, `TodoChange`, `TodoDelete`, `TodoToTicket` → `c.scanTodos()`
- `GetDrafts` → `todospkg.ListDrafts(c.draftStore()), nil`; `DraftCreate` → `LoadTreeFiles` +
  `CreateDraft(c.draftStore(), …)`; `DraftDelete` → `DeleteDraft(c.draftStore(), id)`
- `GetContributors` → `contributorspkg.ListContributors(contributorspkg.NewFsContributorStore(c.rootDir))`
- 16 `goalspkg.IsRootGoal/IsFirstGenGoal/IsDeeperGoal(&goal|goal)` → `(goal.ID)`
- `🔭️exhaustive_test.go` (build tag `exhaustive`): `TestExhaustiveFilterTicketWorkspaceFiles` rewritten
  against the port contract (repo-relative folder and files); its now unused `path/filepath`, `model`
  and `workspace` imports dropped. Verified with `go vet -tags exhaustive ./...`.

`🏃️test-runner/📦️packages/🐹️go/🐹️.go`

- `GenerateTechnologyTodos` → `todospkg.LocateTodosOnDisk(bundleRoot, todospkg.ScanTodos(todospkg.NewFsTodoTree(bundleRoot)))`

`🌳️tree/📦️packages/🐹️go/🐹️.go` (an extra caller not listed in the brief)

- `todos.ListDrafts(todos.NewFsDraftStore(workspace.GetDraftsPath()))`

`🎫️tickets/📦️packages/🐹️go/🐹️.go` (in-package)

- `OpenTicket` and `ReopenTicket` → `ApplyTicketPlanFromIDs(NewFileTicketStore(), HostPlanRoots(), ticket, mcpKind, planID, specID)`
- `ComputeTicketFiles` → `FilterTicketWorkspaceFiles(workspace.NormalizeRepoPath(ticket.FolderPath), files)`
  under a `ticket != nil && ticket.FolderPath != ""` guard. The files reaching that line already went
  through `normalizeTicketFileInputs`, which calls `workspace.NormalizeRepoPath` on each, so the
  absolute-path branch the deleted global carried was already dead on this path.
- `🔬️_test.go`: `TestResolvePlanSourceCursorPlanID`, `TestResolvePlanSourceKiroSpecID` and
  `TestApplyTicketPlanFromIDsCursor` moved to the port signature (`PlanSource.Path` / `.IsDirectory`).

Test adapters updated: `🎯️goals/🧪️tests/🪪️goal-id-scheme/🐹️.go`,
`🧑️contributors/🧪️tests/🪪️contributor-identity-parse/🐹️.go`,
`📝️todos/🧪️tests/✏️draft-lifecycle/🐹️.go`, `📝️todos/🧪️tests/🔍️todo-scanning/🐹️.go`.

## 3. Behaviour notes

- `ListContributors` no longer returns an `error`; the fs store simply yields nothing when the devs
  directory is unreadable, and the empty-store case still yields the single `unknown` contributor —
  the same fallback the deleted global had, now keyed on "no documents" instead of "no directory".
- `CreateDraft` writes file *content* through the store rather than copying bytes with `move.CopyFile`.
  A missing or unreadable source is still an error, raised by `LoadTreeFiles` before anything is created.
- `ResolvePlanSource` yields forward-slash store paths where the global yielded `filepath.Clean`
  output. The ticket tests compare through `filepath.Clean`, so both spellings agree on Windows.

## 4. Verification

### `gofmt -l . && go build ./... && go vet ./...`

Clean (no output) for all of `🧑️contributors`, `🎯️goals`, `📝️todos`, `🎫️tickets`, `⌨️cli`, `🔗️graphql`,
`🏃️test-runner`, `🌳️tree`:

```
===== 🧑️contributors
===== 🎯️goals
===== 📝️todos
===== 🎫️tickets
===== ⌨️cli
===== 🔗️graphql
===== 🏃️test-runner
===== 🌳️tree
```

Every module listed in `go.work` also builds:

```
$ grep -oE '\./[^ ]+' go.work | while read d; do (cd "$d" && go build ./...) ; done
DONE
```

### `go test -count=1 ./...`

```
===== 🧑️contributors
ok  	github.com/usalu/semio/repo/contributors	0.547s
===== 🎯️goals
ok  	github.com/usalu/semio/repo/goals	0.541s
===== 📝️todos
ok  	github.com/usalu/semio/repo/todos	0.712s
===== 🎫️tickets
ok  	github.com/usalu/semio/repo/tickets	1.136s
===== ⌨️cli
ok  	github.com/usalu/semio/repo/cli	6.567s
===== 🏃️test-runner
ok  	github.com/usalu/semio/repo/testrunner	0.481s
===== 🌳️tree
ok  	github.com/usalu/semio/repo/tree	22.701s
```

`⌨️cli` is fully green — the ~16 pre-existing failures the brief warned about did not appear.

`🔗️graphql` has five failures, all pre-existing and none naming a symbol touched here (they are the
removed `fix` mutation, a missing `asset/fixture` path and an empty `bundles` query):

```
--- FAIL: TestFixApplyAutofixes (0.00s)
    🔬️_test.go:148: failed to read fixture: open C:\git\semio\repo\asset\fixture\some\folder\🧪️file-fixable\🟦️.tsx: The system cannot find the path specified.
--- FAIL: TestFixViaRepoContext (0.00s)
    🔬️_test.go:864: Fix failed: fix was removed; handle autofix inside script.ts policy export
--- FAIL: TestGraphQLBundlesQuery (0.00s)
    🔬️_test.go:1414: Expected result to contain 'compose/js', got: {
          "repo": {
            "bundles": []
          }
        }
--- FAIL: TestGraphQLFixMutation (0.00s)
    🔬️_test.go:1441: ExecuteGraphQL fix mutation returned error: graphql errors: [fix: fix was removed; handle autofix inside script.ts policy export]
    🔬️_test.go:1444: Expected result to contain 'fixed', got:
--- FAIL: TestFixHeaderWithShebang (0.03s)
    🔬️_test.go:1703: Fix failed: fix was removed; handle autofix inside script.ts policy export
FAIL	github.com/usalu/semio/repo/graphql	2.158s
```

### Language-agnostic parity

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner <owner>

=== parity fundamental 🧑️contributors
[test] level=fundamental cases=3 executed=7 passed=7 failed=0 errored=0 parity=5/5
=== parity fundamental 🎯️goals
[test] level=fundamental cases=4 executed=9 passed=9 failed=0 errored=0 parity=6/6
=== parity fundamental 📝️todos
[test] level=fundamental cases=4 executed=11 passed=11 failed=0 errored=0 parity=7/7
=== parity fundamental 🎫️tickets
[test] level=fundamental cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
=== parity fundamental 🏃️test-runner
[test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
```

All five owners: 0 failed, 0 errored.

### Spelling sweep

```
$ grep -rn 'ListContributorsFrom\|ScanTodosIn\|ListDraftsIn\|CreateDraftIn\|DeleteDraftIn\|FilterWorkspaceFilesUnder\|IsRootGoalID\|IsFirstGenGoalID\|IsDeeperGoalID\|ResolvePlanSourceIn\|ApplyPlanFromIDsIn' --include=*.go 🔨️modules
(no output, exit 1)

$ grep -rn 'func ListContributors\|func IsRootGoal\|…' --include=*.go 🔨️modules
🎫️tickets/📦️packages/🐹️go/🐹️.go:4716:func ResolvePlanSource(store TicketStore, roots PlanRoots, kind providers.McpClientKind, id string) (PlanSource, error) {
🎫️tickets/📦️packages/🐹️go/🐹️.go:4779:func ApplyTicketPlanFromIDs(store TicketStore, roots PlanRoots, ticket *model.Ticket, kind providers.McpClientKind, planID, specID string) error {
🎫️tickets/📦️packages/🐹️go/🐹️.go:4964:func FilterTicketWorkspaceFiles(folderPath string, files []string) []string {
🎯️goals/📦️packages/🐹️go/🐹️.go:389:func IsRootGoal(goalID string) bool { return GoalDepth(goalID) == 0 }
🎯️goals/📦️packages/🐹️go/🐹️.go:392:func IsFirstGenGoal(goalID string) bool { return GoalDepth(goalID) == 1 }
🎯️goals/📦️packages/🐹️go/🐹️.go:395:func IsDeeperGoal(goalID string) bool { return GoalDepth(goalID) >= 2 }
📝️todos/📦️packages/🐹️go/🐹️.go:1119:func ScanTodos(tree TodoTree) []*model.Todo {
📝️todos/📦️packages/🐹️go/🐹️.go:1554:func ListDrafts(store DraftStore) []*model.Draft {
📝️todos/📦️packages/🐹️go/🐹️.go:1566:func CreateDraft(store DraftStore, title string, files []TreeFile) (*model.Draft, error) {
📝️todos/📦️packages/🐹️go/🐹️.go:1589:func DeleteDraft(store DraftStore, id string) error { return store.Delete(id) }
🧑️contributors/📦️packages/🐹️go/🐹️.go:984:func ListContributors(store ContributorStore) []model.Contributor {
```

## 5. Files touched

- `🔨️modules/🧑️contributors/📦️packages/🐹️go/🐹️.go`
- `🔨️modules/🧑️contributors/🧪️tests/🪪️contributor-identity-parse/🐹️.go`
- `🔨️modules/🎯️goals/📦️packages/🐹️go/🐹️.go`
- `🔨️modules/🎯️goals/🧪️tests/🪪️goal-id-scheme/🐹️.go`
- `🔨️modules/📝️todos/📦️packages/🐹️go/🐹️.go`
- `🔨️modules/📝️todos/🧪️tests/{✏️draft-lifecycle,🔍️todo-scanning}/🐹️.go`
- `🔨️modules/🎫️tickets/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}`
- `🔨️modules/⌨️cli/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}`
- `🔨️modules/🔗️graphql/📦️packages/🐹️go/{🐹️.go, 🔭️exhaustive_test.go}`
- `🔨️modules/🏃️test-runner/📦️packages/🐹️go/🐹️.go`
- `🔨️modules/🌳️tree/📦️packages/🐹️go/🐹️.go`
- this report

No `🗑️generated` output was produced, and no git-modifying command was run.
