# 📓️ Final fixes — the eight open items of `📓️opus-cli-verbs.md` §4 and `📓️opus-consolidation.md` §5

Executor: Opus 5 (`final-fixes`).
Host: Windows 11, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`, bun 1.4.2,
go 1.25, 16 cores. Every command below was run; every number is real output.

The `repo` MCP server refused `initialize` for this session (`-32602 invalid initialize params`), so no
ticket lifecycle call was made; the work landed in the existing ticket folder.

## 1. Result

| # | Item | Outcome |
| --- | --- | --- |
| 1 | `definitions()` aggregate, `File.extension`/`kind`, `Folder.name`/`kind` | fixed on both sides; the three aggregates are now byte-identical |
| 2 | Export digest | identical digest and identical event log for the same repository |
| 3 | `list` / `search` / `query` | root cause found (it is not "renders nothing"); two defects fixed, one structural divergence recorded |
| 4 | `loc` performance | Rust 9m40s → **1m17s**, Go >30min → **2m00s**; output identical except one recorded 0.013% residue |
| 5 | `🔤️rename-casings` | contract decided in Go's favour, Rust and the vectors changed, Go adapter added, parity 12/12 → 16/16 |
| 6 | `analyze` short text | the Go root now registers the breach analyzer; the projection substitution is gone |
| 7 | `🧩️vscode` codegen | regenerated from the SDL; all ten operations resolve; the extension builds |
| 8 | nx duplicate project | reproduced and recorded, not edited |

## 2. `definitions()`, `File` and `Folder` — one contract, both implementations

### 2.1 The contract, and where it is written down

The frozen recording context `🔗️graphql/🧫️fixtures/🔣️repo-records.json` (and its two copies under
`⌨️cli/🧪️tests/`) is the statement of the wire shape, and it says:

```json
{ "id": "file:repo/go/main.go", "path": "repo/go/main.go", "name": "main.go", "extension": ".go", "kind": "code", … }
{ "id": "folder:repo",          "path": "repo",           "name": "repo", "kind": "organization", … }
{ "id": "definition:main", "name": "main", "kind": "implementation", "filePath": "repo/go/main.go", "sectionPath": "Header", "emoji": "🛠️", … }
```

So: **the extension carries its leading dot**, **the kind is derived**, and **a definition is fully
addressed** — file, enclosing section, kind, name, emoji. The SDL agrees (`File.kind: String!`,
`Folder.kind: String!`). Measured before the fix, on a four-file fixture repository:

```
GO: {"extension":"ts","id":"🗃️src💻a","kind":"","name":"a.ts","path":"src/a.ts"}
RS: {"extension":".ts","id":"🗃️src💻a","kind":"code","name":"a.ts","path":"src/a.ts"}
GO folders: {"id":"🗃️src","kind":"","name":"src","path":"src"}
RS folders: {"id":"🗃️src","kind":"required","name":"src","path":"src"}
GO definitions: []
RS definitions: [{"id":"🛠️alpha",…},{"id":"🪨beta",…},{"id":"🛠️gamma",…}]
```

Neither side was right about folders (Go answered `""`, Rust hard-coded `FolderKind::Required`), Rust
was right about files, and the empty Go definition list had a single cause: `move.ToolDefinitionList`
was a **stub** — `"(definition parsing not implemented in Go - use TypeScript API)"` — even though
`languages.ParseDefinitions` has existed all along and `codebase.BuildCodebaseDefinitions` uses it.

### 2.2 What changed

A definition of a file is a property of the whole file — its name and range come from the language
plugin, but the section that encloses it does not — so the composition now lives once per language in
the module that owns both halves, `🗂️codebase`:

- `codebase.FileDefinitions(content, filePath) []model.Definition` / `codebase::file_definitions(…)`
  fill `FilePath`, `SectionPath` (through `FindSectionForDefinition`) and `Emoji`.
- `codebase.DefinitionID(fileID, definition)` / `codebase::definition_id(…)` build the identity from
  the file, the normalised section path, the kind and the name.

Then:

- `🚚️move` Go: `ToolDefinitionList` is implemented over `codebase.FileDefinitions` and prints the
  definitions it found (or `(no definitions found)`), the way `ToolSectionList` already did.
- `🔗️graphql` Go `RepoContext`: `GetFiles` keeps the dot (`filepath.Ext`) and derives
  `model.DeriveFileKind(name)`; `GetFolders` derives `model.DeriveFolderKind` and
  `model.IsGeneratedFolder`; both now read `FolderID`/`BundleID`/`ParentID` off the walk's own record
  instead of recomputing them (`GetBundleForFile` answered the synthetic `"repo/repo"` for a
  repository with no bundles, which is how a `bundleId` appeared on records that belong to no bundle);
  `GetDefinitions` and `GetSections` fill the identity through the new shared helpers.
- `⌨️cli` Rust `FsContext`: `folders()` derives the kind through `Codebase::derive_folder_kind` and
  `is_generated_folder`; `files()` derives the kind from the *name*, not the path; `definitions()` and
  `sections()` walk the same considered file set and fill `id`, `filePath`, `sectionPath` and `emoji`.
- `📐️model` Go gained the public `DefinitionKindEmoji(kind)` the private map-keyed twin now delegates to.

### 2.3 Verified

```
$ semio-repo graphql '{ repo { files { id path name extension kind } } }' --json
$ semio      graphql '{ repo { files { id path name extension kind } } }' --json
… both: {"repo":{"files":[{"extension":".ts","id":"🗃️src💻a","kind":"code","name":"a.ts","path":"src/a.ts"},
                          {"extension":".go","id":"🗃️src💻b","kind":"code","name":"b.go","path":"src/b.go"}]}}

… folders, both: {"repo":{"folders":[{"id":"🗃️src","kind":"organization","name":"src","path":"src"}]}}

… definitions, both: {"repo":{"definitions":[{"id":"🗃️src💻a🛠️alpha","kind":"IMPLEMENTATION","name":"alpha"},
                                             {"id":"🗃️src💻a🪨beta","kind":"CONSTANT","name":"beta"},
                                             {"id":"🗃️src💻b🛠️gamma","kind":"IMPLEMENTATION","name":"Gamma"}]}}
```

Two scenarios were added to `🗂️codebase/🧪️tests/🚶️workspace-walk`, with a Rust and a Go adapter each,
over the committed `📡️repo-tree.json`:

- `walk-projects-the-same-file-and-folder-records` — identity, path, name, extension, kind and
  generated flag of every walked record;
- `walk-projects-the-same-definitions` — identity, name, kind, file, section path, emoji and range of
  every definition of every considered file.

```
$ … parity fundamental --owner 🗂️codebase
[test] level=fundamental cases=4 executed=20 passed=20 failed=0 errored=0 parity=10/10       (was 16 / 8)
```

## 3. The export digest

`📤️export-verb-records` deliberately did not compare the digest across implementations because the
records' encoding belonged to `📐️model`. With §2 in place the counts agreed and only the digest did
not:

```
GO: "snapshot": "66169871fdf26e89430087129e9d694e190ab77b60f8d4e3ef7c82a9649db6b4"
RS: "snapshot": "be0cf0efad31bd4ac50780897aad19e5c20fb3b4b31a40e4618a5f1a982f5fe7"
```

The written log showed why in one line: Go emitted
`{"id":…,"path":…,"uri":…,"name":…,"extension":".ts",…}` and Rust
`{"emoji":"","extension":".ts","folderId":…,"generated":false,"id":…}` — **declaration order versus
sorted order**. The reference marshals a domain struct with `encoding/json`, which writes fields in
declaration order; the Rust export routed every record through `serde_json::to_value`, and
`serde_json::Value::Object` is a sorted map, so the order was lost before the hash saw it.

`serde_json::to_string` on the struct itself does keep declaration order — that is exactly what
`📐️model`'s `round_trip_json` and its 62-golden set already pin, `Folder`, `File`, `Definition`,
`Section`, `Technology` and `Bundle` among them. The fix is to stop passing the record through
`Value`:

- `📡️events` gained `Payload`, the twin of Go's `json.RawMessage`: a JSON document carried as the
  exact text its owner encoded, with `Payload::of(&value)` encoding through the value's own
  `Serialize`. `ExportEntity.value`, `Input.data` and `StoreEvent.data` are `Payload` now, and the
  digest and the log line both read `payload.text()`.
- `Input` keeps the reference's asymmetry exactly: the reference declares `Data interface{}` on an
  offered record and `json.RawMessage` on a committed one, so an offered generic document is
  re-encoded (compact, sorted) while a committed one is carried verbatim. `Input`'s
  `deserialize_with = "normalized_payload"` is that difference; without it a pretty-printed fixture
  payload would have carried its newlines straight into a JSONL line.
- serde_json's `raw_value` feature is enabled in the workspace manifest. It is a feature of a
  dependency this workspace already declares, not a new dependency.

Verified on the same fixture repository, whole files diffed:

```
$ diff exp-go.json exp-rs.json   → EXPORT-IDENTICAL
$ diff ev-go.jsonl ev-rs.jsonl   → EVENTLOG-IDENTICAL
```

`📤️export-verb-records` gained `the-digest-and-the-encoded-records-agree` (`@mode-differential`),
which states the digest itself plus every input's `kind\x01<encoded record>`; both entry points
(`repo_cli::export_records_json`, `cli.ExportRecords`) now report `records` for it.

```
$ … parity fundamental --owner ⌨️cli
[test] level=fundamental cases=8 executed=34 passed=34 failed=0 errored=0 parity=17/17       (was 32 / 16)
$ … parity fundamental --owner 📐️model
[test] level=fundamental cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
$ … parity fundamental --owner 📡️events
[test] level=fundamental cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27
```

## 4. `list` / `search` / `query`

### 4.1 They do not render nothing

`📓️opus-consolidation.md` §5.1 recorded that the three verbs "render nothing". They do not. On this
monorepo they are **slow**, and a probe with a 300-second timeout returns an empty pipe:

```
$ time semio      list yaml        → real 23m9s,  27 lines
$ time semio-repo list yaml        → real 27m26s, 36760 lines
```

On a small repository both answer immediately. What the verbs really disagree on is *content*, and
the cause is structural, below.

### 4.2 Two defects fixed

1. **The `Codebase` category carried no emoji.** Both twins built its label with
   `identity.Entity("codebase")`, but `codebase` is registered in `🔣️entity-emojis.json` under
   `collections`, not `entities`, so the lookup answered `""`. Both now call `Collection("codebase")` /
   `collection("codebase")` and produce `🖥Codebase`, which is what the reference's
   `EmojiText(EmojiCodebase)` produces. `🌳️tree/🧫️fixtures/📤️tree-build-expectations.json` states the
   emoji now (4 places).
2. **File and folder nodes lost their parent when rendered.** `list` and `search` render a node that
   carries `data` through the entity renderer, and `artifact_id` composes a file identity as
   `parentId + kindEmoji + flat(stem)`. The tree's `FileRecord`/`FolderRecord` carried no parent, so
   the renderer printed `💻a` where the reference prints `🗃️src💻a`. Both records now carry
   `ParentID`/`parent_id`, filled from the walk's own `folderId`/`parentId`, and both node builders
   put it into the node data.

```
before  RS: - [🛅src](repo://folder/🛅src) / - [💻a](repo://file/💻a) / - [Codebase](repo://codebase)
after   RS: - [🗃️src](repo://folder/🗃️src) / - [🗃️src💻a](repo://file/🗃️src💻a) / - [🖥Codebase](repo://codebase)
GO        : - [🗃️src](repo://folder/🗃️src) / - [🗃️src💻a](repo://file/🗃️src💻a) / - [🖥Codebase](repo://codebase)
```

### 4.3 What is left, and exactly why — for the audit

`🌳️tree` Go carries **two** monorepo tree builders:

- `BuildMonorepoTreeFromRepo` (`🩻️Monorepo Tree`, line ~790) — the godfile-era builder, and the one
  the `semio-repo` binary still calls through `BuildMonorepoTreeCached`;
- `buildCodebaseNode` / `BuildMonorepoTree(source)` (`🌿️NodeHelpers`, line ~3640) — the port-based
  twin of the Rust builder, which is what the Rust `semio` binary and both harness adapters use.

The two disagree on which files reach the tree (the godfile builder walks every non-ignored file; the
port-based one takes the codebase walk's *considered source files*, so `package.json`, `docs/` and
`README.md` are absent) and on whether a policy carries its statutes as children. That is a
**Go-binary-versus-both-twins** split, not a Go/Rust divergence: `parity fundamental --owner 🌳️tree`
is 12/12 because the two twins agree with each other. Closing it means deleting
`BuildMonorepoTreeFromRepo` and pointing `BuildMonorepoTreeCached` at the port-based builder, which is
a `🌳️tree` module change with its own fixtures, and it is the single remaining reason `list` and
`search` differ between the two binaries.

The second open item on that surface: **`Section.path` is never filled** — neither
`BaseLanguage.ParseSections` nor `parse_marker_sections` sets it, so `BuildSectionID(fileID, [])`
collapses every section of a file onto the file's own id. Both implementations do this identically, so
no parity case sees it, but `🔣️repo-records.json` states `"path": "Executor#Project"`, i.e. the
`#`-joined ancestor names. Owner: `🗣️languages`.

## 5. `loc`

### 5.1 Where the thirty minutes went

Neither implementation finished a `loc` over this monorepo in thirty minutes at session start. Timed
here, they do — badly:

```
$ time semio loc --json                       → real 9m40s
```

and instrumented (temporary `[DEBUG]` lines, since removed):

```
[DEBUG] numstat_log 66.8s bytes=237930416
[DEBUG] parse      125.3s commits=605
[DEBUG] fold         0.3s
[DEBUG] scan        28.6s
```

Three separate costs, and none of them was a per-file git call or a full-tree rescan — the pipeline
was already one `git log --numstat` pass plus one `git ls-files`:

1. **git itself.** `git log --no-merges --numstat --first-parent --reverse` over 605 commits of a
   77 083-file tree takes **6m07s** warm (8m54s cold, 4m04s with `--no-renames`), because `--numstat`
   reads both blobs of every changed file and this history averages 1 165 changed files per commit.
2. **the parser.** 520 168 numstat lines, each asking the 625-rule `.gitignore` whether the path is
   skipped, with no early exit — 325 million glob evaluations.
3. **the tracked-tree scan.** 77 083 paths through the same rule set, then a read of each classified file.

### 5.2 What changed, in both implementations

- **The numstat walk is split across the first-parent chain.** `numstat_log_ranges(chain, chunks)` /
  `NumstatLogRanges` cut the chain, newest commit first, into contiguous `<older>..<newer>` ranges
  covering it exactly once, oldest range first; `numstat_log_range_args` / `NumstatLogRangeArgs` ask
  git the same question about every commit as the whole-history walk; `join_numstat_chunks` /
  `JoinNumstatChunks` glue the outputs back, inserting the one newline `--pretty=format:` puts between
  commits and omits at the end. A history of 32 commits or fewer is still one `git log`. Sixteen
  workers draw from four ranges each. Verified byte-identical against the single pass before any code
  was written:

  ```
  $ cmp joined.txt warm.txt   → IDENTICAL     (520 168 lines, 1m26s vs 6m07s at 16 ranges, 52s at 64)
  ```
- **The bucket decision is memoised per path** in `parse_numstat_log` / `ParseNumstatLog`.
- **The ignore matcher gained an exact prefilter.** Every compiled rule now carries the longest
  literal run it requires (`Pattern::required_literals` / `requiredLiteral`); a path that carries none
  of a rule's literals cannot match it, so the glob is never run. This is sound by construction — a
  path matching an alternative contains that alternative's literal runs — and `**/` is one token, so
  `**/target` requires `target`, not `/target`. It is what took the Go `🏠️workspace` test suite from
  50.3s to 2.1s.
- **The tracked-tree scan is parallel**, one thread per core over a shared index, each folding into
  its own per-bucket map. `GitLogSource` is `Sync` now.

### 5.3 Measured

```
$ time semio      loc --json     → real 1m17.5s        (was 9m40s)
$ time semio-repo loc --json     → real 2m0.1s         (did not finish in 30 min)
```

Two consecutive Go runs are byte-identical to each other; Go against Rust differs in exactly one
number:

```
== go2 vs go3 ==   (identical)
== go2 vs rs3 ==
21c21
<         "loc": 2147861,          "Data"
---
>         "loc": 2147591,
61c61
<         "loc": 5109281,          "Total"
---
>         "loc": 5109011,
```

Every other bucket and **every** churn column — `edited`, `added`, `removed`, `percent`,
`wip_percent`, for Code, C#, Go, Python, Rust, TypeScript, Markup and Total — is identical. Since the
churn columns are computed from the same `path_skipped_for_loc` over half a million historical paths,
their agreement is the evidence that the two ignore prefilters are consistent and that the divergence
is not in path selection.

**What is left, precisely.** A 270-line (0.013%) residue in the `Data` bucket, which only
`count_unified_loc_for_file`'s JSON branch can produce: a `.json`/`.jsonc` body whose key count one
`json` implementation answers and the other refuses, falling back to the physical line count.
`physical_line_count`/`PhysicalLineCount` and `count_json_keys`/`CountJSONKeys` are line-for-line
identical, so the disagreement is between `serde_json::from_str` and `encoding/json` on some real
document. Ruled out by measurement over all 21 850 tracked JSON files: non-UTF-8 bodies (none),
nesting deeper than serde_json's 128-frame limit (none), unreadable or over-long paths (none), and
lone-surrogate escapes (53 files, all under `.🧬semio`, which `path_is_repo_meta` skips). Note also
that a run-to-run drift of a few dozen lines in the same bucket is normal here — other fleets rewrite
generated JSON in the working tree while the scan runs. Owner: `📊️metrics`.

## 6. `🔤️rename-casings`

`📓️opus-move.md` §4 left this open with the Rust behaviour and the frozen vectors on one side and
`cli.ToolRename` on the other. The contract is now **Go's**: naming a scope narrows *where*
occurrences are rewritten, it does not exempt the scope itself, so the scope root is renamed exactly
when its own name carries the token. That is also what `ToolRename` does — it rewrites the *basename*
of each walked entry, so a scope root named `model` is renamed and one named `docs` is not.

- `plan_rename` no longer skips the scope root.
- `🧫️fixtures/🔤️rename-vectors.json`, vector `scoped-to-one-directory`: the tree after the rename is
  `shape/`, `shape/nested/Shape.md`, `shape/shape.ts`, and the counters are
  `{filesChanged: 2, filesRenamed: 2, foldersRenamed: 1}` with the matching output line.
  `only-a-filename-changes` is unchanged — its scope root `docs` does not carry `readme`.
- The round-trip scenario renames back under the **rewritten** scope, because the forward rename moved
  the scope root; both adapters compute it the same way, by applying the casing rewrite to the scope's
  last segment.
- The workspace-level rename moved to where plan §2 puts it: `move.ToolRename` in
  `🚚️move/📦️packages/🐹️go`, with its three tests, and `⌨️cli`'s `renameCommand` calls it.
- `🚚️move/🧪️tests/🔤️rename-casings/🐹️.go` is the new Go adapter — it materialises each recorded
  before-tree under the scenario work directory, runs the real `ToolRename` over it and reads the
  whole tree back, the same shape `🚚️file-folder-move` already uses.

```
$ … parity fundamental --owner 🚚️move
[test] level=fundamental cases=5 executed=32 passed=32 failed=0 errored=0 parity=16/16      (was 28 / 12)
$ go test ./…/🚚️move/📦️packages/🐹️go      → ok  github.com/usalu/semio/repo/move  0.720s
```

## 7. `analyze`

`analyze [scope]` / "Analyze codebase for breachs" is the accurate pair: it is what `autofix` pairs
with, what every report means by the verb, what the Rust root registers, and what
`📖️usage-goldens.json` already states. The entity-summary `analyze <id>` that `NewRootWithConfig`
registered was the odd one out and had no counterpart anywhere.

`NewRootWithConfig` registers `analyzeCmd` now, `analyzeCmd` declares `MaximumNArgs(1)` to match
`Arity::Maximum(1)`, the dead `analyzeCommand` factory is gone, and `ProjectionRoot` no longer needs
its `projectionReplaceChild` substitution — the real root and the projected root are the same tree.

## 8. `🧩️vscode` codegen

### 8.1 What was wrong

The committed codegen was not merely missing its `documents` map; it was **spliced**. Overload
signatures ran into the middle of unrelated query strings, one JSDoc block opened inside another, and
every `DocumentNode` literal said `"kind": "Artifact"` where the GraphQL AST says `"kind": "Document"`
— the footprint of a repository-wide `Document` → `Artifact` rename sweep having run through generated
data. `graphql()` closed over an identifier that was never declared, so all ten
`export const XDocument = graphql(\`…\`)` resolved to `{}`.

### 8.2 What was done

`$TICKET/🏗️build-vscode-codegen.ts` is the generator the package never had. It reads the operations out
of the file's own `⌛️Queries` region, parses and **validates every one of them against
`🔗️graphql/🧬️schema/🔣️schema.graphql`** with the real `graphql` package, and emits, from the parsed
ASTs: the per-operation `XQueryVariables` / `XQuery` result types in the client-preset shape, the
`XDocumentNode` constants (no source positions, no empty `arguments`/`directives`), the `documents`
map keyed by the exact source each `graphql` tag is called with, and one typed overload per operation.
The orphan `Repo` and `Codebase` documents, which no operation declares, are gone with the rest of the
spliced text.

```
$ bun ./…/🏗️build-vscode-codegen.ts
[build-vscode-codegen] 10 operations validated against the SDL
  RepoStructure RepoCheckpoints FolderContent Bundles Tickets Policies Contributors Analyze FileContent Goals

$ (every graphql(`…`) call resolved against the documents map)
resolved: 10 unresolved: []

$ cd …/🧩️vscode/📦️packages/🟦️typescript && bun ./📜️script.ts build
out/extension.js …
✓ built in 11.99s
```

The document constants are module-internal (`XDocumentNode`) and `⌛️Queries` keeps the single exported
`XDocument` per operation, which also removes the duplicate-export defect `📓️opus-mcp-hygiene.md` §4.2
found in the `Fix` pair.

## 9. nx — the duplicate project, unedited

Reproduced verbatim, three times, hours apart:

```
$ bun x nx show projects
 NX   Failed to process project graph.
The following projects are defined in multiple locations:
- test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-third-party-puzzle-2d-1:
  - ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🌐️third-party-puzzle-2d-1
  - ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🕸️third-party-puzzle-2d-1
```

Nothing under `🦑️repo` contributes to it; every nx target in the repository stays blocked until its
owner renames one of the two. Not touched.

## 10. Verification — every gate, real output

```
$ cargo build --release -p semio-framework-repo-cli --bins
    Finished `release` profile [optimized] target(s)

$ cargo test --release -p semio-framework-repo-{workspace,codebase,metrics,events,tree,move,model,cli,dashboard}
    every target: test result: ok. 0 failed        (events 12, tree 30, move 12, workspace 4, model 4, …)

$ cargo clippy --release -p semio-framework-repo-{cli,codebase,metrics,dashboard}
    no warning against any line of any 🦑️repo crate

$ go build ./... && go vet ./...     # 📐️model 🗂️codebase 🔗️graphql 🚚️move ⌨️cli 📊️metrics 🌳️tree 🏠️workspace
    all eight: build+vet OK, gofmt clean

$ go test ./...
    model 0.507s · codebase 0.451s · graphql 2.448s · move 0.735s · tree 38.996s · workspace 2.069s · cli 8.282s   all ok

$ … parity fundamental --owner <owner>
🗂️codebase     cases=4 executed=20 passed=20 failed=0 errored=0 parity=10/10
🚚️move         cases=5 executed=32 passed=32 failed=0 errored=0 parity=16/16
📐️model        cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
📊️metrics      cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
⌨️cli          cases=8 executed=34 passed=34 failed=0 errored=0 parity=17/17
🌳️tree         cases=5 executed=24 passed=24 failed=0 errored=0 parity=12/12
📡️events       cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27
🏠️workspace    cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
🎛️dashboard    cases=1 executed=1  passed=1  failed=0 errored=0 parity=0/0
```

## 11. Left open

1. **`🌳️tree`'s two builders** (§4.3) — the godfile `BuildMonorepoTreeFromRepo` the Go binary uses
   against the port-based twin both harnesses use. The remaining `list`/`search` content difference.
2. **`Section.path` is never filled** (§4.3) — identical in both implementations, contradicted by the
   frozen records. Owner: `🗣️languages`.
3. **The `loc` `Data` residue** (§5.3) — 270 lines out of 2.15 million, a JSON acceptance difference
   between `serde_json` and `encoding/json`. Owner: `📊️metrics`.
4. **The nx project graph** (§9) — another fleet's duplicate project name.
5. **`list`/`search` are slow on this monorepo** (§4.1) — 23 and 27 minutes, because they build the
   whole tree. The same three levers `loc` needed (parallel walk, memoised classification, prefiltered
   ignore matching) apply; only the ignore prefilter of them landed here, and it is shared, so both
   verbs are already faster than the numbers above.

## 12. Files

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧪️tests/🔤️rename-casings/🐹️.go`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🏗️build-vscode-codegen.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-final-fixes.md` (this file)

Updated — Rust:

- `📡️events/📦️packages/🦀️rust/🦀️.rs` — `Payload`, `normalized_payload`, `ExportEntity`/`Input`/`StoreEvent`
- `🏠️workspace/📦️packages/🦀️rust/🦀️.rs` — `Pattern::required_literals`, `Rule.literals`, prefiltered `matches_path`
- `🗂️codebase/📦️packages/🦀️rust/🦀️.rs` — `file_definitions`, `definition_id`
- `📊️metrics/📦️packages/🦀️rust/🦀️.rs` — `numstat_log_range_args`, `numstat_log_ranges`, `join_numstat_chunks`, the chunked `SystemGit::numstat_log`, the memoised parser, the parallel `snapshot_loc_counts`, `GitLogSource: Sync`
- `🚚️move/📦️packages/🦀️rust/🦀️.rs` — `plan_rename` includes the scope root
- `🌳️tree/📦️packages/🦀️rust/🦀️.rs` — `FolderRecord.parent_id`, `FileRecord.parent_id`, both node builders, `collection("codebase")`
- `⌨️cli/📦️packages/🦀️rust/🦀️.rs` — `FsContext::{folders,files,sections,definitions}`, `FsTreeSource::load`, `export_entity`, `export_records_json`
- `🎛️dashboard/🌳️command-tree/🦀️.rs` — the two tree records

Updated — Go:

- `📐️model/📦️packages/🐹️go/🐹️.go` — `DefinitionKindEmoji`
- `🏠️workspace/📦️packages/🐹️go/🐹️.go` — `rule.literal`, `requiredLiteral`, prefiltered `MatchesPath`
- `🗂️codebase/📦️packages/🐹️go/🐹️.go` — `FileDefinitions`, `DefinitionID`
- `📊️metrics/📦️packages/🐹️go/🐹️.go` — `NumstatLogRangeArgs`, `NumstatLogRanges`, `JoinNumstatChunks`, the chunked `NumstatLog`, the memoised parser, the parallel `SnapshotLocCounts`
- `🔗️graphql/📦️packages/🐹️go/🐹️.go` — `GetFiles`, `GetFolders`, `GetDefinitions`, `GetSections`
- `🚚️move/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}` — `ToolDefinitionList` implemented, `ToolRename` moved in with its three tests
- `🌳️tree/📦️packages/🐹️go/🐹️.go` — `FolderRecord.ParentID`, `FileRecord.ParentID`, both node builders, `Collection("codebase")`
- `⌨️cli/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}` — `analyzeCmd` registered, `analyzeCommand` deleted, `ProjectionRoot` simplified, `ToolRename` delegated, `ExportRecords` reports `records`

Updated — fixtures, features and adapters:

- `🗂️codebase/🧪️tests/🚶️workspace-walk/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `⌨️cli/🧪️tests/📤️export-verb-records/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `🚚️move/🧪️tests/🔤️rename-casings/{🥒️.feature, 🦀️.rs}` and `🚚️move/🧫️fixtures/🔤️rename-vectors.json`
- `📡️events/🧪️tests/🔏️export-content-hash/🦀️.rs`
- `🌳️tree/🧫️fixtures/📤️tree-build-expectations.json`
- `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` — the two codegen regions regenerated
- `Cargo.toml` — `serde_json` gains the `raw_value` feature

No git-modifying command was run.
