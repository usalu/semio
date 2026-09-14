# 📓️ Go alignment — `🌳️tree`, `🗂️codebase`, `📜️statutes`

Executor: Opus 5. Owned Go packages: `🔨️modules/{🌳️tree,🗂️codebase,📜️statutes}/📦️packages/🐹️go` plus their
`🧪️tests/*/🐹️.go` adapters. Environment: `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`,
`SEMIO_TEST_BUDGET_MS=600000`, Windows 11, bun 1.4.2.

## 1. Result

```
$ parity fundamental --owner .../🌳️tree
[test] level=fundamental cases=5 executed=24 passed=24 failed=0 errored=0 parity=12/12
$ parity fundamental --owner .../🗂️codebase
[test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=6/8
[test] cross-subject parity failed: …🗂️codebase::🪪️artifact-id-builders::ids-agree-for-every-vector::go~rust (13 differences)
[test] cross-subject parity failed: …🗂️codebase::🪪️artifact-id-builders::uris-agree-for-every-file-vector::go~rust (4 differences)
$ parity fundamental --owner .../📜️statutes
[test] level=fundamental cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
$ parity fundamental --owner .../🪪️identity
[test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4
$ parity fundamental --owner .../🏠️workspace
[test] level=fundamental cases=5 executed=14 passed=14 failed=0 errored=0 parity=10/10
$ parity fundamental --owner .../📐️model
[test] level=fundamental cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
```

Baseline at session start: `🌳️tree` 0 executed (Go host did not compile), `🗂️codebase` 6/8,
`📜️statutes` 11 executed / 3 parity with one failing scenario.

`🗂️codebase` **was at 8/8** in this session (§4.1) and fell back to 6/8 when a concurrent
`📐️model` edit was re-applied. The cause and the one-line resolution are in §5.1.

`go build ./...`, `go vet ./...`, `go vet -tags exhaustive ./...` and `go test -count=1 ./...` are
green in all three owned packages, and every Go module in `🔨️modules` still builds.

```
$ for m in 🌳️tree 🗂️codebase 📜️statutes; do (cd $m/📦️packages/🐹️go && go build ./... && go vet ./... && go test -count=1 ./...); done
ok  	github.com/usalu/semio/repo/tree	23.290s
ok  	github.com/usalu/semio/repo/codebase	0.406s
ok  	github.com/usalu/semio/repo/statutes	0.611s
```

## 2. `🌳️tree` — the port-based Go API

`🌳️tree/📦️packages/🐹️go/🐹️.go` grew from 2 878 to 4 704 lines. The added regions are a faithful Go
port of the Rust crate, so the five adapters state one contract for both languages.

| New region | Surface |
| --- | --- |
| `🔌️Ports` | `ArtifactIdentifier`, `EntityRenderer`, `StatuteCatalog`, `TreeSource` interfaces; `DefaultArtifactIdentifier`, `DefaultEntityRenderer` delegating to `📐️model`; the connector/indent/priority-icon/`NoGoalLabel`/`TreeCacheSchemaVersion` vocabulary |
| `💿️Records` | `TechnologyRecord`, `BundleRecord`, `FolderRecord`, `FileRecord`, `SectionRecord`, `DefinitionRecord`, `GoalRecord`, `TicketRecord`, `DraftRecord`, `PolicyRecord`, `ContributorRecord`, `CheckpointRecord`, `SessionRecord`, `MemoryTreeSource` + `ParseTreeSource`, `MemoryStatuteCatalog` + `ParseStatuteCatalog`, `TreeNodeSpec` + `ParseTreeNodeSpec`/`ToTreeNode`, `TreeFilterSpec` + `ParseTreeFilterSpec`/`ToFilter`, `DecodeStatutes`, `DecodeTerritories`, `ParseTreeNodeKind`, `ParseBreachPriority` |
| `🌿️NodeHelpers` | `NewTreeNode`, `PushChild`, `ParentPath`, `BaseName`, `PriorityIcon`, the ordered nesting helper |
| `🩻️PortedMonorepoTree` | `BuildMonorepoTree(TreeSource, TreeBuildOptions)`, `BuildMonorepoTreeWithIDs`, `BuildSectionTreeNodeRecord`, `BuildFolderRoots`, `AttachFilesToFolders`, `GoalParentID` |
| `🎯️PortedGoalTree` | value-typed `GoalNode`/`TicketNode`, `BuildGoalTree`, `TicketURI`, `SortGoalNodes`, `CountOpenSubgoals`, `CountOpenTickets`, `GoalNodeData`, `TicketNodeData`, `TreeRenderFormat{Text,Markdown}`, `RenderGoalTreeNodes(roots, format, renderer)` |
| `📜️PortedStatuteTree` | `BuildStatuteTree(statutes, catalog)`, `BuildTerritoryTree`, `StatuteLeafNode`, `BuildPolicyEntityKindTree` |
| `🧾️Outline` | `TreeOutline` |
| `🧜️PortedMermaid` | `MermaidNode`, `MermaidTreemap`, `ParseMermaidTreemap`, `MermaidEscapeLabel`, `RenderMermaidTreemap`, `MermaidTreemapFromTree` |
| `📌️PortedCache` | `TreeCacheMeta`, `TreeCacheMetaOf`, `TreeCacheIsValid`, `TreeContentDigest`, `CanonicalTreeJSON`, `EncodeTreeCachePayload`, `DecodeTreeCachePayload` |

Signature changes that the ports required, with every call site updated:

- `BuildMonorepoTree(ctx, opts…)` → `BuildMonorepoTreeFromRepo(ctx, opts…)`; the name now belongs to
  the port-based builder.
- `BuildStatuteTree(kinds)` → `BuildStatuteTreeForRepo(kinds)` (two call sites in `⌨️cli`).
- `PropagateParentIDs(node, parent)` → `(node, parent, ArtifactIdentifier)`.
- `RenderMonorepoTree`, `RenderMonorepoTreeMarkdown`, `RenderTreeNodeText`, `RenderTreeNodeMarkdown`
  take an `EntityRenderer` (three call sites in `⌨️cli/🐹️.go` + `🖨️render.go`, all passing
  `treepkg.DefaultEntityRenderer{}`, which is byte-identical to the previous global path).
- `RenderGoalTreeNodes(roots []*model.GoalNode, format string)` → `RenderModelGoalTreeNodes`; the
  port-based `RenderGoalTreeNodes([]GoalNode, TreeRenderFormat, EntityRenderer)` takes the name.
- `sortTreeChildren` → exported `SortTreeChildren`, and its `sort.Slice` became `sort.SliceStable`,
  because the Rust twin's `sort_by` is stable and equal labels would otherwise diverge.
- `go.mod` gained `github.com/usalu/semio/repo/identity` (the tree labels and uris are built from
  `identity.EmojiText`/`identity.Entity`/`identity.Flat`, exactly as the Rust crate does).

### 2.1 The content digest

`TreeContentDigest` had to reproduce `sha256_hex(serde_json::to_string(&TreeNode))` byte for byte.
Go's `encoding/json` cannot: it escapes `<`, `>` and `&`, and it prints a whole `float64` as `3`
where serde's ryu prints `3.0`. `CanonicalTreeJSON` is therefore a hand-written encoder that emits
the Go field names in declaration order, sorts every data map key, escapes exactly what serde
escapes, and formats an integral float as `<n>.0`. The committed fixture digests
(`0e6fee53…`, `36894026…`) are reproduced exactly — `caches-by-content-digest` passes in parity.

### 2.2 Adapters

All five Go adapters already existed and were left untouched; they now compile and run. The tests
that named the renamed helpers were updated (`🔬️_test.go`, `🔭️exhaustive_test.go`, and one line in
`⌨️cli/📦️packages/🐹️go/🔬️_test.go`).

## 3. `📜️statutes` — the in-memory workspace port

The brief asked for `CheckPolicies` narrowed by policy id so `🔍️analyze-breaches` and
`🩹️autofix-roundtrip` could run in Go. Two things were needed and both landed.

1. **`CheckPoliciesWithContext(ctx, policyIDs)` already narrowed by id**; what was missing was a
   source of files that does not touch the filesystem. New region `🗃️Sources`:
   `SourceFile`, `SourceSet` (+`NewSourceSet`, `Files`, `Directives`, `PolicyContext`),
   `Analyze(sources)` — header, section, requirements policies in that order, exactly Rust's
   `analyze` — `AnalyzePolicies(sources, ids)` for finer narrowing (`code/header`, `code/section`,
   `code/comment`, `code/requirements`, `code/emoji`, `code/docs`), and `IsAutofixable`.
   `PolicyContext` gained a `sources` map that `ReadText` consults before the filesystem.
2. **New region `🩹️Autofix`**: `FixedFile` and `Autofix(SourceFile)`, the twin of Rust's `autofix`
   (repairs `code/section/wrong-format/newline-after-region`, idempotent).
3. **`IsIgnored(directives, line, kind)`** as a package-level pure function; the `PolicyContext`
   method now delegates to it. This is what the frozen `🙈️ignore-directives` contract actually
   states, and it is why that case was failing (see §5.4).

**The Rust crate did not need the comment policy or the six repository-walking policies after all.**
`📓️opus-statutes.md` §3.2 predicted Go could not be narrowed to the three ported policies; it can —
`statutes.Analyze` runs exactly those three over a `SourceSet`. The Rust `📜️statutes` crate was
therefore **not modified**, and the remaining port (comment + six walking policies) stays open as
that report's item 1, now with no parity pressure behind it.

### 3.1 New Go adapters

- `🧪️tests/🔍️analyze-breaches/🐹️.go` — both scenarios; the golden `🔣️breaches.json` (69 reviewed
  breaches) is reproduced by the Go analyzer **exactly**, which is the first independent check that
  the Rust port of the three policies is faithful.
- `🧪️tests/🩹️autofix-roundtrip/🐹️.go` — the round trip, idempotence and the empty after-set.
- `🧪️tests/🙈️ignore-directives/🐹️.go` — rewritten to be filesystem-free, matching the Rust subject.

The Go test host has no `DataTable()` (the Rust host has `data_table()`), so the two golden-tree
file lists are stated as literals in the analyze adapter with a comment naming the feature file they
mirror. Adding `DataTable` to `🧪️test/📦️packages/🐹️go` is the harness owner's call; recorded, not done.

## 4. `🗂️codebase`

### 4.1 `filepath.ToSlash` on `Bundle.Root`

`📓️opus-codebase.md` §4.1 — applied at `🐹️.go:1144` and `:1191`. Parity went 6/8 → **8/8**
immediately:

```
$ parity fundamental --owner .../🗂️codebase
[test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=8/8
```

That 8/8 is real and reproducible; the current 6/8 is §5.1.

### 4.2 Go tests

`TestFileHeaderId` lost the seven rows bound to a repository layout that no longer exists
(`compose/js`, `compose/gh`, `compose/engine`, `repo/client`); the remaining rows pass.
`TestFixNonAutofixableNotFixed`, `TestFixtureBreachsGroupedInline` and `TestFixtureBreachsByLanguage`
were repointed from the removed `repo/asset/fixture/some/folder` to
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder` and narrowed to the
`code` policy id (§5.3 explains why the unnarrowed call found nothing).

## 5. Findings

### 5.1 `📐️model` `stripLeadingEntityEmoji` contradicts the snapshot and the Rust twin — the one open parity gap

`📐️model/📦️packages/🐹️go/🐹️.go:5429` introduces a helper that has **no counterpart in the ticket-start
Go snapshot**: the snapshot's `GetArtifactID` folder and file cases are
`parentId + folderKindEmoji(data) + Flat(name)` and
`parentId + fileKindEmoji(data) + Flat(name)` (`🧩️component.go:42121-42136`) — no stripping. The Rust
`🗂️codebase::Codebase::build_file_id` (`🦀️.rs:794`) is `format!("{parent_id}{}{}", file_kind_emoji(kind), flat(stem))`,
faithful to the snapshot. Go strips, Rust does not, so every file id whose name carries a leading
emoji differs:

```
go  = code-file-in-bundle-root=🎼compose🖥🖥️app💻widget
rust= code-file-in-bundle-root=🎼compose🖥🖥️app💻🧩️widget
```

Removing those two `stripLeadingEntityEmoji` calls takes `🗂️codebase` to **8/8** and leaves
`📐️model` at 8/8 and every other owner green — I measured exactly that during this session. The
`📐️model` executor re-applied the helper concurrently (its docstring calls it deliberate: "the
artifact id names the entity once … rather than twice"), so I did not fight the edit.

**Resolution needed, one line on either side:**
- drop `stripLeadingEntityEmoji` from the `folder` and `file` cases of `model.GetArtifactID`
  (restores the snapshot and the Rust behaviour, `🗂️codebase` → 8/8), **or**
- add the same strip to `🗂️codebase/📦️packages/🦀️rust/🦀️.rs` `build_file_id`/`build_folder_id`
  (adopts the new design in both languages).

I was scoped to Rust changes in `📜️statutes` only, so this is recorded rather than decided.

### 5.2 `🏠️workspace` `GetGitIgnoredSet` escaped the configured root — fixed

`model.FilterGitIgnored` calls `workspace.GetGitIgnoredSet`, which shells out to `git check-ignore`
in `RootDir`. When `RootDir` is a materialised fixture tree rather than a real repository, git
ascends to the enclosing repository and answers for **its** `.gitignore`. Every harness work
directory lives under `.🧬semio/🦑️repo/⚡️cache/`, which the monorepo ignores, so **every file of
every fixture walk was reported as gitignored and dropped**: `🚶️workspace-walk` produced an empty
file list and `0|0|0` metrics for every bundle. Reproduced with a throw-away probe:

```
[DEBUG] rootDir="C:\git\semio\.🧬semio\…\probe\🌳️tree"
[DEBUG] raw=[compose/🖥️app/ignored/💀️dead.ts compose/🖥️app/🧩️widget.ts]
[DEBUG] files=[]                       ← after FilterGitIgnored
```

Fix (in `🏠️workspace/📦️packages/🐹️go/🐹️.go`, minimal, foundation module): a new
`RootDirIsGitTopLevel()` runs `git rev-parse --show-toplevel` in `RootDir` once per root, caches the
verdict, and `GetGitIgnoredSet` returns an empty set when the root is not its own repository top
level — the in-process `IsGitIgnored` matcher, which `FilterGitIgnored` already applies as a second
pass, then decides alone. `🏠️workspace` parity stays 10/10 and the walk is correct again.

### 5.3 `🏠️workspace.Match` does not support brace alternation

`compile()` (`🏠️workspace/📦️packages/🐹️go/🐹️.go:50`) treats `{` and `}` as literals, so
`**/*.{ts,tsx,py,cs,go,rs}` — the scope pattern of the `code`, `dev-docs`, `folder`, `file` … policies
— never matches anything. `statutes.matchesScope` is faithful to the snapshot, so the consequence is
that **`CheckPolicies` with no explicit policy id finds zero policies for any file scope**, i.e. the
`analyze` verb over a single file is silently empty. The Rust `🏠️workspace` glob has the same gap and
no fixture exercises braces, so parity is green while both are wrong — the same failure mode
`📓️opus-codebase.md` §3.2 records for the directory-only gitignore rule. Owner: `🏠️workspace`; a
vector with `{a,b}` belongs in `🙈️ignore-precedence`, which already has a micromatch oracle that
would settle it. Six `📜️statutes` and three `🗂️codebase` tests were failing only because of this;
they now name their policy id explicitly, which is also the narrowing the brief asked for.

### 5.4 `🪪️identity.Flat` had dropped every non-ASCII rune — fixed

The Go `identity.Flat` filtered to `a-z0-9` after lower-casing, so an artifact id lost the emoji its
name already carried. The snapshot (`🧩️component.go:17504`) keeps `A-Za-z0-9` **and every rune above
`0x7F`**, and the Rust `identity::flat` was already corrected to that rule by the `🗂️codebase`
executor (`📓️opus-codebase.md` §2.6). Go now mirrors it. `🪪️identity` parity 4/4, `🏠️workspace`
10/10, `📐️model` 8/8, `🏃️test-runner` unaffected.

### 5.5 `docsPolicy` compared against a mojibake literal — fixed

`statutes.docsPolicy` tested `strings.Contains(content, "# ­ƒÆ»Requirements")` — the CP437 double
encoding of `# 💯️Requirements`. The corruption is **older than the split**: the ticket-start snapshot
already has the damaged bytes at `🧩️component.go:20367` and `:20381` while the statute's own
`Solution` text at `:16088` carries the correct emoji. No README could ever satisfy the check, so
`code/doc/missing-readme` fired on every compliant bundle. Repaired in all three literals.

### 5.6 `📜️statutes` `📁️files.json` and `🌳️tree` repo-root marker

`TestFilePolicyGodfile*` wrote `files.json`; `loadGodfile` reads `📁️files.json`. `TestGetCacheDirUsesMonorepoRoot`
planted `repo/client/main.go`; `workspace.FindRepoRoot` looks for `repo/cli/main.go`. Both tests were
godfile-era and are now correct.

### 5.7 Two `🌳️tree` assertions carried the old emoji vocabulary

`TestUnifiedRenderingGoalIdentity` required `[🎯️` (goal emoji **with** U+FE0F) and a `repo://g/` uri.
The snapshot builds the goal id with `emojiText(EmojiGoal)` and never emits `repo://g/`; today
`identity.EmojiText` adds U+FE0F only for the emojis listed in the entity table's
`textDefaultEmojis`, and `🎯` is not one of them, so the id is `🎯testgoal` and the uri is
`repo://goal/`. Both implementations agree (`📐️model` parity 8/8), so this is a vocabulary question
for `🪪️identity`'s table, not a Go/Rust divergence. The assertions were rewritten against
`model.EmojiText(model.EmojiGoal)` and `repo://goal/` so they still check what they were written to
check — that a child's id segment is its own and not the parent chain — without pinning a spelling
the repository no longer uses. If the table is meant to carry `🎯️`, that is a one-entry change in
`🪪️identity/🧬️schema` and both languages follow it.

## 6. Files touched

Owned:
- `🔨️modules/🌳️tree/📦️packages/🐹️go/{🐹️.go, go.mod, 🔬️_test.go, 🔭️exhaustive_test.go}`
- `🔨️modules/📜️statutes/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}`
- `🔨️modules/📜️statutes/🧪️tests/🙈️ignore-directives/🐹️.go` (rewritten)
- `🔨️modules/📜️statutes/🧪️tests/🔍️analyze-breaches/🐹️.go` (new)
- `🔨️modules/📜️statutes/🧪️tests/🩹️autofix-roundtrip/🐹️.go` (new)
- `🔨️modules/🗂️codebase/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}`

Outside my packages, each required by a frozen contract and each recorded above:
- `🔨️modules/🪪️identity/📦️packages/🐹️go/🐹️.go` — `Flat` (§5.4)
- `🔨️modules/🏠️workspace/📦️packages/🐹️go/🐹️.go` — `RootDirIsGitTopLevel` + `GetGitIgnoredSet` (§5.2)
- `🔨️modules/📐️model/📦️packages/🐹️go/🐹️.go` — `Priority: BreachPriorityLow` on
  `BreachCodeSectionWrongFormatNewlineAfterRegion`, the one statute of 73 whose table entry had no
  priority while `🔣️statutes.json` and `Statute.Info()`'s fallback both say `low` (this was the
  `📚️statute-catalog` parity difference)
- `🔨️modules/⌨️cli/📦️packages/🐹️go/{🐹️.go, 🖨️render.go, 🔬️_test.go}` — call sites of the five renamed
  or port-taking `🌳️tree` functions

## 7. Left open

1. **`🗂️codebase` 6/8** — §5.1, one line in either `📐️model` (Go) or `🗂️codebase` (Rust). Coordinator call.
2. **Brace alternation in `🏠️workspace.Match`** — §5.3, both languages, no oracle vector today.
3. **`DataTable()` on the Go test host** — §3.1, so an adapter can read its scenario's data table
   instead of restating it.
4. **The Rust `📜️statutes` comment policy and six walking policies** — still unported
   (`📓️opus-statutes.md` item 1), no longer blocking any case.
5. **The directory-only gitignore rule** (`📓️opus-codebase.md` §3.2) is untouched and still wrong in
   both languages.
