# WP4b — `🦑️repo` product, `💻️client` modules

Worker partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/**` only (CLI, MCP, VS Code, SQLite).
`📚️library`, `🧪️test`, `🖥️server` are other workers'.

Assigned rows of `📋️cross-partition-requests.md`: **18** (the two unreferenced SQLite schemas) and
**19** (exhaustive Go test baseline for `⌨️cli`).

Implemented against `📋️execution-contract.md` (§A identity, §B layout, §E rules). Repo MCP was down for
the whole session; no MCP tool was called, no ticket was opened/closed/reopened, `🗑️generated/` was not
deleted. No `cargo` was run (machine saturated by peer checks). No git-modifying command was used.

---

## 1. Row 18 — which SQLite schema is live

**Decision: `🗄️.sql` (7 tables) is the live contract. `📐️schema.sql` (24 tables) is deleted.**

Neither file was read or executed by any code — that part of `📓️wp4-repo-product.md` §8.1 is confirmed:
`grep -rn "schema.sql|🗄️.sql|📐️schema"` over `*.go|*.ts|*.rs|*.json|*.md` returns no hit inside
`💻️client`, and the only `sqlite` token in the client's Go sources is a file-extension literal
(`⌨️cli/🧩️component.go:10764`, in a binary-extension list). So "which one does the client *execute*"
has no answer; the question that does have an answer is **which one is the client's own entity model**,
and four independent lines of evidence agree.

### 1.1 Evidence

1. **`📐️schema.sql` is not valid SQL.** Its first table is unparseable (missing comma after `alias`,
   trailing comma after the last column):

   ```
   $ sqlite3 /tmp/b.db < 📐️schema.sql
   Parse error near line 28: near "github": syntax error
     E CHECK (length (trim(alias)) > 0) -- nn_text     github TEXT NOT NULL UNIQUE
                                         error here ---^
   ```

   `contributor` is therefore never created, and `release_contributors`, `version`, `checkpoint`,
   `session` and `event` all declare `FOREIGN KEY … REFERENCES contributor (id)` against a table that
   cannot exist. `🗄️.sql` loads clean (sqlite3 3.51.0, macOS system binary):

   ```
   $ sqlite3 /tmp/a.db < 🧬️schema/🗄️.sql && sqlite3 /tmp/a.db ".tables"
   bundle      file        repo        technology
   definition  folder      section
   ```

2. **`🗄️.sql`'s seven tables are exactly the client's exported entity set.** `⌨️cli/📤️event_export.go`
   materializes `ExportResult{Technologies, Bundles, Folders, Files, Sections, Definitions}` from
   `RepoContext.GetTechnologies/GetBundles/GetFolders/GetFiles/GetSections/GetDefinitions` — the six
   `technology, bundle, folder, file, section, definition` tables one for one, plus the `repo` root the
   scan is taken from (`Repo{ID, Name, Path, Technologies, Bundles}`, `GetRootDir()`).
   `📐️schema.sql` adds `release`, `release_contributors`, `version`, `checkpoint`, `entity`,
   `mechanism`, `system`, `system_entities`, `agent`, `session`, `event`, `event_kind` and six `*_kind`
   lookup tables; **none** of those has any counterpart in the client's Go model
   (`⌨️cli/🧩️component.go`: `Repo`, `Technology`, `Bundle`, `Folder`, `File`, `Section`, `Definition`).

3. **Column shape matches.** `🗄️.sql` stores the kind as an inline `kind INTEGER NOT NULL DEFAULT 0`,
   which is how the Go structs carry it (`Folder.Kind FolderKind`, `File.Kind string`,
   `Definition.Kind DefinitionKind`). `📐️schema.sql` normalizes kinds into `folder_kind`/`file_kind`/
   `technology_kind`/`bundle_kind`/`definition_kind`/`client_kind` lookup tables and adds a mandatory
   `checkpoint_id` FK on `folder`/`file` — neither exists anywhere in the client.

4. **Provenance.** `🗄️.sql` is the taxonomy-canonical rename of `🛢️checkpoint.sql`, done in the
   2026-09-02 taxonomy normalization (`git log --follow`: `e5465a2c1c 2026-09-02 13:31:56`, a pure
   rename `🛢️checkpoint.sql => 🗄️.sql`). `📐️schema.sql` was **not** renamed by that sweep and has had no
   commit of its own since `3550b3dc09 2026-08-13`; it is what the sweep left behind.

Nothing in the tree references the 24-table design: `grep -rn` for `release_contributors|system_entities|
folder_kind|definition_kind|client_kind|technology_kind|bundle_kind|file_kind` over `*.go|*.ts|*.rs|
*.json|*.sql` (excluding `📐️schema.sql` itself) returns only unrelated taxonomy-`fileKinds` code in
`💻️os`. So there is **no live consumer**, the "both become exports of the same module" branch of the
brief does not apply, and `📐️schema.sql` is deleted outright (no alias, no redirect).

### 1.2 Persisted local-only vs shared

Every row export is annotated `x-semio-persistence: "local-only"`. Evidence: the seven entities are
produced by a scan of the local working tree (`RepoContext`), the export writes to a local path
(`ExportToEventLogContext` defaults to `<root>/repo.events.jsonl`), and no client→coordinator call
carries them — the only client traffic is `syncTicketToServer` (`/api/v1/ticket`, ticket contract) and
`repopkg.Emit` (`FilePayload{Path}`, path only). Shared repo state lives in
`🖥️server/🧬️schema/🐘️postgres/🗄️.sql`. No row in this module is `shared`.

---

## 2. New scope

| Scope id | Module path | `$id` | Formats |
|---|---|---|---|
| `repo.client.sqlite` | `🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧬️schema/` | `https://semio.tech/schema/repo/client/sqlite/schema.json` | `🔣️.json` (draft-07), native `🗄️.sql` |

Exports (7), one `<Table>Row` per native table, each `x-semio-persistence: "local-only"`:
`RepoRow`, `FolderRow`, `TechnologyRow`, `BundleRow`, `FileRow`, `SectionRow`, `DefinitionRow`.

`dependsOn`: nothing. Ready for WP2's `📚️library/🔣️schema-catalog.json`.

Column types are projected from the DDL only (`INTEGER` → `integer`, `TEXT` → `string`, nullable where
the column is neither `NOT NULL` nor `PRIMARY KEY`). `🗄️.sql` declares no `CHECK (… IN (…))`, so no
`enum` was invented; the kind vocabularies documented in the DDL comments and in
`⌨️cli/🐹️entity_kinds.g.go` are recorded as `$comment` on each `kind` property.

---

## 3. Files created / moved / deleted

### Created

- `💻️client/🪶️sqlite/🧬️schema/🔣️.json` — the draft-07 contract (7 row exports).
- `💻️client/🪶️sqlite/📦️packages/🟦️typescript/📋️project.json` — nx targets `test`, `test-quick`,
  `test-long`, `test-exhaustive` (the package was in the root `package.json` workspace list but was not
  an nx project and had no test host).
- `💻️client/🪶️sqlite/📦️packages/🟦️typescript/📜️script.ts` — router, `test` only.
- `💻️client/🪶️sqlite/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` — vitest config, `passWithNoTests: false`,
  `include: [resolve(root, "../../🧪️tests/*/🟦️.ts")]` (same shape as
  `💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`).
- `💻️client/🪶️sqlite/🧪️tests/🔬️schema/🟦️.ts` — the parity test (§5). Written as
  `📦️packages/🟦️typescript/🔬️schema.test.ts`; a concurrent repo-wide taxonomy sweep relocated it to the
  owner module's `🧪️tests/<case>/🟦️.ts` slot mid-session (commit `9869c6e99b`, 17:55). That is the
  taxonomy-correct home and matches contract §B's `🧪️tests` (`testsDirName`) rule, so it was kept there
  and the vitest config was pointed at it rather than moved back. Its `moduleRoot` (`../../🧬️schema`)
  resolves identically from both locations.

### Moved

| From | To |
|---|---|
| `💻️client/🪶️sqlite/🗄️.sql` | `💻️client/🪶️sqlite/🧬️schema/🗄️.sql` |
| `💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go` | `💻️client/⌨️cli/🐹️entity_kinds.g.go` (§4) |

### Deleted

- `💻️client/🪶️sqlite/📐️schema.sql` — the 24-table draft of §1.
- `💻️client/⌨️cli/⚡️implementations/🐹️go/` and `💻️client/⌨️cli/⚡️implementations/` — left empty by the move.

### Edited

- `💻️client/🪶️sqlite/🧬️schema/🗄️.sql` — header now names the contract it implements (it carried a stale
  `sql/sqlite/repo/📐️schema.sql` self-reference); `🎯️Requirements` records the local-only class.
- `💻️client/🪶️sqlite/README.md` — documented the module instead of the deleted file; the ER diagram was
  a third, non-existent shape (it drew `commit`, `repo.github`, `file.extension`, `section.start_line`,
  `definition.file_id`, none of which is in either SQL file) and now mirrors `🗄️.sql` exactly.
- `💻️client/⌨️cli/🔬️component_test.go:4183` — `TestPostgresSchemaIncludesKitVersionControlTables` read
  `repo/postgres/🛢️schema.sql`, a path that has not existed since the taxonomy sweep, so the test could
  only ever `t.Fatalf`. Repointed at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🐘️postgres/🗄️.sql`;
  all 20 asserted snippets are present there (extracted from the test body and substring-checked against
  the file: `checked 20 missing 0`).

`AGENTS.md` files were not touched. After this change the only schema-format file left outside a
`🧬️schema/` module in this partition is `🔌️mcp/🔗️graphql/🔗️.graphql` (§7.3).

---

## 4. `go test ./...` was structurally impossible before this change

`⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go` declares `package client` but sits in a subdirectory,
so Go treats it as its own package with the import path
`github.com/usalu/semio/repo/client/⚡️implementations/🐹️go`. Go rejects that path outright:

```
$ GOWORK=…/go.work go list ./...
malformed import path "github.com/usalu/semio/repo/client/⚡️implementations/🐹️go": invalid char '⚡'
```

Package loading aborts, so `go list`, `go vet` and `go test ./...` all fail immediately with that single
line — the file itself was never compiled by anything. Its own docstring says it is "Named distinctly
from the pre-existing `EntityKinds` … elsewhere **in this package**", i.e. it was always meant to be part
of the root `client` package. Moved to `⌨️cli/🐹️entity_kinds.g.go`; `go build ./...` is then clean (no
symbol collision), which is what made the row-19 baseline runnable at all.

The file is generator output and `.gitignore:109` (`**/🐹️entity_kinds.g.go`) ignores it in both the old
and the new location, so this move is not a tracked change — `git ls-files` never knew the old path. That
also means the breakage only appears on a machine where the generator has run; a fresh clone has no such
directory and `go test ./...` loads fine. The generator and the taxonomy entry that name the old path are
outside this partition — cross-partition requests **A** and **F** in §7.

---

## 5. Verification

### 5.1 `repo.client.sqlite` parity test — passing

`bun ./📜️script.ts test` in `💻️client/🪶️sqlite/📦️packages/🟦️typescript`:

```
 RUN  v4.1.10 …/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript

 Test Files  1 passed (1)
      Tests  12 passed (12)
   Start at  17:58:23
   Duration  1.60s (transform 388ms, setup 0ms, import 421ms, tests 603ms, environment 0ms)
```

(Re-run at 17:58 after the peer sweep moved the spec — same 12 tests, same result as the 17:05 run.)

The test mirrors `🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts`:
draft-07 identity, `$id`, one `<Table>Row` per `CREATE TABLE`, and per-table column-name + nullability
parity between `🗄️.sql` and `🔣️.json`. Additions over the template: the native entity set is asserted
literally (so a table appearing on either side is caught), and `x-semio-persistence` must be declared for
every row. Third-party oracle: `ajv` 8.20.0 compiles the document and accepts/rejects three `FolderRow`
samples (valid, `name: null`, extra property).

One deviation from the template, and it is a real bug in it: the postgres test skips a clause when
`clause.toUpperCase().startsWith("CHECK")`, which also swallows any column whose name begins with those
letters. `repo.checkpoint` is exactly such a column, and the first run dropped it (observed:
`expected [ 'checkpoint', 'id', 'name', …(1) ] to deeply equal [ 'id', 'name', 'summary' ]`). Here the
constraint keywords are matched with a word boundary (`/^(?:PRIMARY KEY|UNIQUE|FOREIGN KEY|CHECK|CONSTRAINT)\b/i`).
The SQLite DDL also carries trailing `-- …` comments on column lines, which the template's
"drop lines starting with `--`" filter leaves in place, so comments are stripped to end-of-line instead.
See cross-partition request **D**.

### 5.2 Ledger row 12 — this partition is clear

`bun ./📜️script.ts schema check --report …` over the whole tree after the change:

```
[schema check] modules=3165 scopes=1854 findings=8784
[schema check] catalog-stale=1
[schema check] document-dialect-unexpected=961
[schema check] document-id-duplicate=11
[schema check] document-id-missing=32
[schema check] document-id-unaddressable=78
[schema check] document-not-object=1
[schema check] export-id-invalid=441
[schema check] module-level-ineligible=455
[schema check] module-scope-id-inconsistent=305
[schema check] module-scope-id-missing=73
[schema check] mutation-aggregate-kinds-redundant=59
[schema check] mutation-leaf-id-grammar=2067
[schema check] placement-retired-location=45
[schema check] ref-not-catalog-addressable=2771
[schema check] ref-not-export-addressed=537
[schema check] ref-unresolved=74
[schema check] scope-id-duplicate=873
```

`grep -c "💻️client" report.jsonl` → **0**. Not one finding of any code names a path in this partition,
so ledger row 12 is clear here. (`catalog-stale=1` is the tree-wide catalog, not mine — see request E.)

### 5.3 Scope discovery

`bun ./📜️script.ts schema audit --out …` sees the new module without any taxonomy edit:

```json
{
  "modulePath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧬️schema",
  "ownerPath":  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite",
  "level": "product-module",
  "facetKindId": "🧬️data",
  "scopeId": "repo.client.sqlite"
}
```

with `exports` = the seven `<Table>Row` ids and `dependsOn: []`.

### 5.4 Row 19 — exhaustive Go test baseline

See §6.

---

## 6. Row 19 — `go test ./...` in `⌨️cli`

One attempt, foreground, no `-short`, full output in `🗑️generated/wp4b-cli-go-test.txt`:

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli
$ GOWORK=<repo>/go.work go test ./... -count=1 -timeout 40m
```

started 17:08:29, ended 17:50 (compile ≈ 7 min, then the test binary ran 40 min to its own timeout).

**Baseline: not green, and it is not a load problem.** Three distinct failures, none of them caused by
this ticket's changes:

```
--- FAIL: TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap (6.51s)
    --- FAIL: …/creates_workspace_from_root_and_submodules (3.78s)
        🔬️component_test.go:369: bash [.devcontainer/post-attach.sh] failed: exit status 127
            .devcontainer/post-attach.sh: line 320: mapfile: command not found
    --- FAIL: …/updates_workspace_only_for_missing_repos (2.74s)
        🔬️component_test.go:488: bash [.devcontainer/post-attach.sh] failed: exit status 127
            .devcontainer/post-attach.sh: line 320: mapfile: command not found
--- FAIL: TestExhaustiveBundlesNonEmpty (0.00s)
    🔬️component_test.go:932: bundles collection should not be empty
panic: test timed out after 40m0s
	running tests:
		TestExhaustiveFoldersNonEmpty (39m43s)
…
FAIL	github.com/usalu/semio/repo/client	2400.536s
?   	github.com/usalu/semio/repo/client/cmd/repo	[no test files]
ok  	github.com/usalu/semio/repo/client/internal/command	0.910s
ok  	github.com/usalu/semio/repo/client/internal/eventstore	1.742s
?   	…/internal/{glob,graphql,humanize,id,ignore,mcp,mcpserver,search,templatefunc,yaml}	[no test files]
FAIL
```

1. **`TestExhaustiveFoldersNonEmpty` does not terminate.** It ran 39m43s of the 40m budget and was still
   running when the alarm fired. The goroutine dump puts it in the real-monorepo scan, not in I/O wait:
   `ExecuteJSON("{ folders { path } }")` → `graphql.Do` → `queryResolver.Folders` →
   `repoContext.GetFolders` → `CodebaseContext.LoadFiles` → `ScopeToFiles`
   (`🧩️component.go:36202`, `:21648`, `:17394`), with a runnable goroutine inside `unicode.SimpleFold`
   — i.e. it is burning CPU in glob/scope matching over the whole tree. Peer `cargo` load slows the box
   but does not explain a 2400× overrun of a query the `-short` suite skips. This is the real content of
   ledger row 19: the exhaustive suite is **not** merely slow, one case is effectively non-terminating,
   and an idle machine will not change that. The other two failures below prove the binary was making
   progress and reached them long before the alarm.
2. **`TestExhaustiveBundlesNonEmpty` fails in 0.00s** — the same executor answers
   `{ bundles { name } }` with an empty collection. It is instant, so it is a resolver/discovery defect
   against the current taxonomy layout, not a timeout.
3. **`TestExhaustiveDevcontainerPostAttachGitKrakenWorkspaceBootstrap` is not portable to macOS.**
   `.devcontainer/post-attach.sh:320` uses `mapfile`, a bash ≥ 4 builtin; `/bin/bash` on macOS is 3.2, so
   the script exits 127 there. It would pass in the devcontainer and fail on every native macOS machine.
   This one contradicts CLAUDE.md's "zero-touch and cross-platform for devcontainer, native windows,
   native macos and native linux".

Everything outside the root package is green: `internal/command` and `internal/eventstore` pass, the
other eleven packages have no tests. **Note that `go test ./...` could not even load before §4** — the
figures above are the first exhaustive baseline this module has had.

All three failures are pre-existing and orthogonal to schema ownership: 1 and 2 are repo-scan/resolver
defects in `🧩️component.go`, 3 is a devcontainer script portability defect. Fixing them is a separate
ticket; I did not touch them, since row 19 asked for the baseline, not the repair. All three are
`testing.Short()`-gated, so the nx `test` target skips them by design, which is why they stayed
invisible.

### 6.1 The default `-short` target is red too (67 tests)

Because it costs ~70s I also ran what the nx target actually runs; full output in
`🗑️generated/wp4b-cli-go-test-short.txt`:

```
$ GOWORK=<repo>/go.work go test . -short -count=1 -timeout 9m
…
FAIL	github.com/usalu/semio/repo/client	69.513s
$ grep -c '^--- FAIL' → 67
```

The failures cluster into the same few root causes and none is schema-ownership work: the CLI's
technology/bundle discovery no longer finds the current layout (`technology "repo" not found`,
`technology "compose" not found`, `TestBundleListCommand`, `TestFolderTreeCommand`,
`TestGraphQLBundlesQuery`, `TestToolTechnologyList`, … — same root cause as the exhaustive
`TestExhaustiveBundlesNonEmpty`), the section/definition policy family
(`TestSectionNewlineAfterRegion`, `TestDefinitionNativeDocstring`, …), the hook/track family, and
`TestMcpStdioInitializeHandshake` (30s, `read initialize response: EOF`).

One test in that suite **is** mine and now passes — `TestPostgresSchemaIncludesKitVersionControlTables`
is not `-short`-gated, so its dead path (§3) had been failing the default target on every run:

```
$ go test . -short -run TestPostgresSchemaIncludesKitVersionControlTables -v
=== RUN   TestPostgresSchemaIncludesKitVersionControlTables
--- PASS: TestPostgresSchemaIncludesKitVersionControlTables (0.00s)
ok  	github.com/usalu/semio/repo/client	0.399s
```

The 67 measured failures are with that fix already applied; before it the same run would also have
reported this test (`os.ReadFile` of a path that exists nowhere in the tree → `t.Fatalf`). I did not
measure the pre-fix count, so treat 67 as the post-fix baseline, not as a delta. The 67 predate this
ticket and are untouched.

---

## 7. Cross-partition requests

**A — `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:115` — needed now, one line**
(framework schema worker). The entity-catalog emitter writes the Go artifact to an emoji directory that
Go can never compile (§4). In `generatedTargets()`, change

```ts
{ path: join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go"), content: emitGo(kinds) },
```

to

```ts
{ path: join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go"), content: emitGo(kinds) },
```

The `🟦️entity-kinds.ts` and `🤖️generated.rs` targets are unaffected. **This is time-sensitive**: the
same `generatedTargets()` feeds `GenerateScript`, `PreviewGeneratedScript` and `CheckScript`, so until
it lands that package's `check` target byte-compares against a path that no longer exists and a
`generate` run recreates the uncompilable package. The console line at `:143` is only cosmetic.

**B — root `.vscode/launch.json`** (tooling worker). `📋️project.json` for `@semio-tech/repo-sqlite` is
new, so CLAUDE.md's "register all executable commands in launch.json" needs one entry. Insert it after
the `🦑️mcp test` entry (currently `.vscode/launch.json:491–500`, `group: "3_dev"`), shifting the
following order values by one:

```json
    {
      "name": "🪶️sqlite schema test",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/repo-sqlite:test",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "3_dev",
        "order": -2
      }
    },
```

I did not edit the file myself: it is outside this partition and was already dirty in `git status` at
session start.

**C — `🧰️framework/🛍️products/🦑️repo/README.md:134`** (repo product worker). The line
`- [sqlite](sqlite/README.md) – SQLite schema and helpers` points at a path that does not exist; the
module is at `🔨️modules/💻️client/🪶️sqlite/README.md`.

**D — `🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts`** (server worker).
`TABLE_CONSTRAINTS.some(c => clause.toUpperCase().startsWith(c))` silently drops any column whose name
starts with a constraint keyword (`check…`, `unique…`, `constraint…`, `primary…`). The postgres DDL has
no such column today, so the test is green by luck; the fix is the word-boundary regex used in
`🪶️sqlite/🧪️tests/🔬️schema/🟦️.ts`.

**E — WP2 catalog owner.** No taxonomy change is needed (verified in §5.3): the scope is already
discovered at level `product-module`. It only has to be picked up by the next
`bun ./📜️script.ts schema generate` into `📚️library/🔣️schema-catalog.json`, which is one of the
tooling worker's files. The entry the generator renders is:

```json
"repo.client.sqlite": {
  "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧬️schema",
  "level": "product-module",
  "facetKind": "🧬️data",
  "formats": { "🔣️jsonschema": "🔣️.json" },
  "exports": ["BundleRow", "DefinitionRow", "FileRow", "FolderRow", "RepoRow", "SectionRow", "TechnologyRow"],
  "dependsOn": [],
  "hashes": { "🔣️.json": "4c4af8e0f10c723fa43d1d0ffc9a33e6b571f8c40ffb6497cd8dfba4f0aed956" }
}
```

**F — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:24661`** (library worker).
The `schema-entity-catalog` generated-path registry declares the same stale `outputRoots` entry as
request A. Change

```json
{ "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go", "inclusion": "ignored" }
```

to

```json
{ "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go", "inclusion": "ignored" }
```

`.gitignore:109` is `**/🐹️entity_kinds.g.go`, so `inclusion: "ignored"` still holds at the new location
and the ignore rule itself needs no change. Land A and F together.

### 7.3 Not acted on: `🔌️mcp/🔗️graphql/🔗️.graphql`

The one remaining schema-format file outside a `🧬️schema/` module in this partition (12.9 KB, tagged
`schema-format-file` in `📊️wp0-ledger.json` `/listings/schema-format-file[425]`). It is a *mirror*: the
authority is the Go source `⌨️cli/internal/graphql/🔗️graphql.go` ("provides the owned schema and
deterministic query executor for the CLI"), and the consumer is the VS Code extension's typed documents
(`🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` §🧬️CodegenGraphql). Making it `🔌️mcp/🧬️schema/🔗️.graphql`
(or, more correctly, an export of a CLI-owned scope, since it is not an MCP contract at all) requires
deleting the `🔗️graphql` bundle directory — the **only** directory with that name in the repo — whose
name is a declared `memberNames` entry in `📚️library/🔣️taxonomy.json:8560`, plus a Go↔GraphQL parity
story for the mirror. Both are outside this partition, so doing half of it would leave the member
registry inconsistent. Flagged for the coordinator as a follow-up row rather than silently restructured.

Also noted in passing (not in this partition, not acted on): `⌨️cli/🧩️component.go:18639,18645` declare
the `repo` policy's scopes as `"go/repo/main.go"`, `"js/vscode/package.json"`,
`"graphql/repo/🔗️.graphql"` — three pre-taxonomy paths that no longer exist, so that policy's parity
checks match nothing. It is inside this partition but is a policy-content decision, not a schema
ownership one; left for whoever owns the repo policy sweep.

---

## 8. Open questions

1. `🗄️.sql` has no code that executes it. It is now a declared contract with a tested JSON Schema
   projection, but the client still exports to `repo.events.jsonl` (an event log), not to SQLite. Whether
   the SQLite projection should be built (an `ExportToSQLite` exists in `🦑️repo/AGENTS.md` §23 as a
   specified operation but not in the Go sources) is a product decision outside this ticket.
2. `🔣️.json` has no `🟦️.ts` / `🦀️.rs` sibling, matching `repo.server`. If WP2's catalog requires every
   scope to publish at least one programmatic format, this scope and `repo.server` need the same
   treatment at the same time.
3. `💻️client/mcp` and `💻️client/client` are Mach-O arm64 binaries (12.6 MB and 10.5 MB) sitting at the
   module root — outputs of the `build` targets. They are untracked (`git ls-files` does not know them),
   so this is local build residue, not committed artefacts. Outside row 18/19; not touched.
