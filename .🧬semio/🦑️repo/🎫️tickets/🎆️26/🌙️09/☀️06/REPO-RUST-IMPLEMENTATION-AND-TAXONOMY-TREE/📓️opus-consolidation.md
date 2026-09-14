# 📓️ Opus executor — consolidation and the cross-language divergences

Host: Windows 11, `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`,
bun 1.4.2, go 1.25. Scope: the `⌨️cli` crate's `🔖️Ansi` / `🔖️Entity` / `FsContext` / `FsTreeSource` /
clock-checkpoint-tracker regions, the dead `🔌️mcp` entry points, the domain crates' existing
regions, and every Go package for the recorded divergences. Sub-reports written by the four
executors this session dispatched:

- `📓️opus-consolidation-braces.md` — item 2(c)
- `📓️opus-consolidation-vs16.md` — item 2(b)
- `📓️opus-consolidation-metrics-statutes.md` — item 2(g)
- `📓️opus-consolidation-renames.md` — item 2(f)
- `📓️opus-consolidation-graphql-tests.md` — the five stale `🔗️graphql` Go tests

## 1. Result

Every gate is green.

```
$ parity fundamental --owner <owner>
=== 🗂️codebase      [test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=8/8
=== 🪪️identity      [test] level=fundamental cases=2 executed=5  passed=5  failed=0 errored=0 parity=4/4
=== 🏠️workspace     [test] level=fundamental cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
=== 🏃️test-runner   [test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
=== 📐️model         [test] level=fundamental cases=3 executed=13 passed=13 failed=0 errored=0 parity=8/8
=== 🎫️tickets       [test] level=fundamental cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
=== 🧑️contributors  [test] level=fundamental cases=3 executed=7  passed=7  failed=0 errored=0 parity=5/5
=== 📜️statutes      [test] level=fundamental cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
=== 📊️metrics       [test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
=== 🌳️tree          [test] level=fundamental cases=5 executed=24 passed=24 failed=0 errored=0 parity=12/12
```

`🗂️codebase` was at **6/8** at session start (`📓️opus-go-align-tree-codebase-statutes.md` §5.1) and is
now **8/8**; `🏠️workspace` moved **10/10 → 13/13** and `🔗️graphql` **?/? → 18/18** with its Go half
finally running.

`--owner` matches owner path *segments*: `--owner 🎫️tickets` works, `--owner ./…/🎫️tickets` selects
zero cases and is not a result. Three sub-agents hit that independently; recorded here so the next
executor does not.

### 1.1 Rust

24 `semio-framework-repo-*` crates: `cargo build`, `cargo test` and `cargo clippy --all-targets` all
green, and **clippy reports zero findings against any `🦑️repo` crate source file**.

```
$ cargo test -p …(24 crates)…
51 × "test result: ok"   0 failed
$ cargo build -p semio-framework-repo-cli --release
    Finished `release` profile [optimized] target(s) in 1m 07s
```

The clippy gate is checked by `🗑️generated/consolidation/clippy_report.sh` (kept out of the tree; the
one-liner is `cargo clippy -p <crate> --all-targets --message-format short | grep '🦑️repo.*warning'`),
which reports `0` for all 24. Warnings still exist in *other* framework crates the repo crates depend
on (`🖱️ui`, `📡️replication`, `🌱️value`); those belong to their owners.

`cargo build --workspace` does **not** pass, for a reason outside this product: another fleet's
in-flight edits to `🧰️framework/🔨️modules/🧬️schema/…/📜️script.ts` and
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` leave
`semio-framework-os-kernel` failing with 45 `MutationLeaf provenance failed: owner escapes workspace
root` errors and `semio-framework-graph`'s build script demanding a `generate` run. Both were green
at session start; neither touches a `🦑️repo` crate. Recorded, not fixed.

### 1.2 Go

`gofmt -l`, `go build ./...`, `go vet ./...`, `go test -count=1 ./...` in every
`🔨️modules/*/📦️packages/🐹️go` plus `🖥️server/🎛️coordinator/📦️packages/🐹️go`:

```
⌨️cli 8.2s · 🌳️tree 22.1s · 🎫️tickets 1.1s · 🎯️goals 0.4s · 🏃️test-runner 0.4s · 🏠️workspace 1.4s
📊️metrics [no test files] · 📐️model 0.4s · 📜️statutes 0.5s · 📝️todos 0.6s · 📡️events 0.7s
🔌️mcp 0.5s · 🔎️search 0.3s · 🔗️graphql 1.8s · 🗂️codebase 0.4s · 🗣️languages 0.3s · 🚚️move 0.7s
🧑️contributors 0.4s · 🧩️providers 3.4s · 🧾️yaml 0.3s · 🪝️hooks 14.0s · 🪪️identity 0.4s
🖥️server/🎛️coordinator 2.8s
```

All `ok`. Two notes:

- `🧪️test/📦️packages/🐹️go` is deliberately **not** in `go.work` (the harness generates its own
  workspace per host), so `go build ./...` there refuses under `GOWORK`. With `GOWORK=off` it builds
  and vets clean. `📋️plan.md` §3 says the root `go.work` lists every module; this one is the
  documented exception and belongs to the harness owner's design.
- `🖥️server/🎛️coordinator`'s three Go files are not `gofmt`-clean (a single trailing blank line at
  `🐹️.go:3452`). Pre-existing, outside this executor's scope, left alone rather than rewritten under
  a concurrent owner.

## 2. Consolidation

### 2.1 Entity rendering left `⌨️cli` — for `📐️model`, not `🪪️identity`

The brief suggested ids → `🪪️identity`, props/templates/ANSI → `📐️model`. **The ids could not go to
`🪪️identity`,** and the reason is structural: `artifact_id`'s `file` branch needs `derive_file_kind`,
which needs `model::FileKind`, and `📐️model` already imports `🪪️identity` (`flat`, `emoji_text`,
`entity`). Putting the ids in `🪪️identity` inverts that edge into a cycle.

The decision is also the one the Go twin already made: `📐️model/📦️packages/🐹️go/🐹️.go` owns
`GetArtifactID`, `GetArtifactURI`, `BuildFileID`, `BuildSectionID`, `DeriveFileKind`,
`CollectEntityProps`, `RenderEntityHuman/Markdown/MarkdownLink`, `InferEntityKind`, `Colorize`,
`TruncateANSI` and `GetTerminalWidth` — all of it in one place. Rust now matches Go exactly, which is
the point of the ticket.

What moved:

| From | To | Surface |
| --- | --- | --- |
| `⌨️cli` `🔖️Ansi` (123 lines) | `📐️model` `🎨️Ansi` | `ansi::{RESET…BOLD, code, colorize, prop_color, terminal_width, truncate}` |
| `⌨️cli` `🔖️Entity` (633 lines) | `📐️model` `🪪️Entity` | `entity::{artifact_id, artifact_uri, goal_artifact_id, build_file_id, build_section_id, collect_props, render_human, render_markdown, render_markdown_link, infer_kind, sanitize_prop, sanitize_single_line, normalize_path, parse_flexible_epoch, path_to_uri_path}` |
| `🗂️codebase` `🏷️Kinds` / `🔣️Emojis` / `🪪️Section Ids` | `📐️model` `🏷️File Kinds` / `🔣️Kind Emojis` / `🪪️Section Ids` | `derive_file_kind`, the five typed kind-emoji helpers, `build_section_id`, `build_definition_id`, `is_test_function_name` |
| `🗂️codebase` `🛤️Paths` | `🪪️identity` `🛤️Paths` | `clean_path`, `dir_of`, `base_of`, `ext_of` — the `filepath` twins, next to `normalize_path` which was already there |

`⌨️cli` now carries two `pub use` lines where 756 lines of ported domain used to sit, so **every call
site in the verb regions is unchanged** (`ansi::colorize`, `entity::render_human`, … resolve through
the re-export). `🗂️codebase` re-exports what it lost, so its own callers are unchanged too.

`🌳️tree` gained `DefaultArtifactIdentifier` and `DefaultEntityRenderer` in its `🔌️Ports` region,
delegating to `📐️model::entity` — the twins of the Go `tree.DefaultArtifactIdentifier` /
`DefaultEntityRenderer`. The two ports the crate declared are now satisfiable from the domain without
reaching up into the CLI, which is what §4 item 1 of `📓️opus-cli.md` asked for.

### 2.2 `SystemClock`, `GitCheckpoints`, `OfflineTracker` moved behind their existing traits

- `🎫️tickets` `⏰️Ports` gained `SystemClock` (`impl Clock`) with the two civil-date helpers, and
  `NullIssueTracker` (`impl IssueTracker`) — the name the Go twin uses.
- `🧑️contributors` `🏁️Checkpoints` gained `GitCheckpoints` (`impl CheckpointSource`) and
  `DEFAULT_CHECKPOINT_LIMIT`.
- `⌨️cli::context` re-exports all three (`OfflineTracker` is an alias of `tickets::NullIssueTracker`),
  so the `FsContext` wiring reads the same.

**A real bug was fixed by the move.** The CLI's `GitCheckpoints` asked git for
`--pretty=format:%H%x1f%an%x1f%ae%x1f%aI%x1f%s` — five unit-separated fields — while
`contributors::parse_checkpoint_log` splits on `|` and needs four. The production checkpoint source
produced a log its own parser could not read, so `checkpoint list` was always empty. The moved
version uses the trait's own `CHECKPOINT_LOG_FORMAT` (`%H|%aN|%ad|%s`) with `--date=iso-strict`.

### 2.3 `mcp::run` / `UnwiredRepository` deleted

Confirmed dead by grep across every `.rs`, `.toml`, `.ts` and `.json` under `🧰️framework`: the only
references were their own definitions. Both removed; `run_with` (which `📦️mcp-main.rs` calls) stays.
The `🔌️mcp` crate builds, tests and lints clean.

### 2.4 `FsContext::folders`/`files` now carry `parentId` / `folderId` / `bundleId`

`model::CodebaseFolder` gained `bundleId`, `model::CodebaseFile` gained `folderId` and `bundleId`, in
both languages, in the same field order, with `🧬️schema/🔣️.json` updated to match. `🗂️codebase` fills
them during the walk (`build_folder_id` of the parent directory, `bundle_id` of the deepest owning
bundle) and `FsContext` reads them straight through instead of writing `None`.

Two divergences closed on the way: Go's `BuildCodebaseFolders` set neither `Name` nor `ParentID` while
Rust set both, and Rust's `parentId` held the parent *path* rather than the parent's *artifact id*,
which contradicted its own name and `model::Folder.parentId`. Both languages now emit the artifact
id. New Go helpers: `CodebaseContext.ParentFolderID` and `CodebaseContext.OwningBundleID`.

### 2.5 The `🌳️tree` cache is written from `search` / `list` / `query`

`⌨️cli::tree_source` gained `build_monorepo_tree_cached(context, include_sections)` — the twin of Go's
`tree.BuildMonorepoTreeCached` — with `cache_dir` (`.🧬semio/🦑️repo/⚡️cache/<sha256(abs root)>`, the
Go path), `tree_fingerprint` (super head + tracked dirtiness + submodule pointers + submodule working
state + structural repo metadata + schema version, the same six parts Go folds), `load_cached_tree`,
`save_cached_tree` and a `LockGuard` directory lock that steals a lock older than a minute. The three
call sites (`verbs::monorepo_tree`, `repo_cli::query`, `repo_cli::kind_listing`) each changed by one
line; nothing else in the verb regions was touched.

**Runtime confirmation**, shipped release binary against this repository:

```
$ semio list --text          # cold
real    12m55.724s
$ ls .🧬semio/🦑️repo/⚡️cache/02e23b55…/
tree-meta.json   tree.json.gz (36,808,629 bytes)
$ cat tree-meta.json
{ "SchemaVersion": 3,
  "Fingerprint": "bde2347c2a9274179a1843c7e672aceb7e2bef77eabfa3d70795941e462e4dad",
  "IncludeSections": false,
  "ContentDigest": "1a623c3f0875403b304a9afd184b8a4e0936fc5bbed142eb7c1128a75a10438b" }
$ semio list --text          # warm
real    1m8.746s
```

**12m55s → 1m09s.** The remaining minute is the fingerprint's `git status` over this tree plus
decoding a 36 MB payload.

## 3. The seven divergences — one contract each

| # | Divergence | Contract chosen | Pinned by |
| --- | --- | --- | --- |
| a | `identity.Flat` / `stripLeadingEntityEmoji` / `build_file_id` non-ASCII | **Keep every rune above `0x7F`; strip nothing.** The ticket-start snapshot (`🧩️component.go:42121-42136`) and the Rust twin both do; only the `📐️model` Go rewrite stripped. `stripLeadingEntityEmoji` deleted. | `🗂️codebase/🧫️fixtures/📡️artifact-id-vectors.json` + two new named vectors, `emoji-named-file-keeps-its-leading-emoji` and `emoji-named-folder-keeps-its-leading-emoji`, and the rule stated in the feature's own prose |
| b | `collect_go_tests_in_section` orphans U+FE0F in Rust | **Strip one grapheme, not one rune** — Go's `StripLeadingGrapheme` ported to Rust | new `🗺️planning-vectors.json` vector `section-name-drops-a-leading-emoji-with-a-variation-selector`, content `// #region 🚀️Gamma` (U+1F680 U+FE0F) |
| c | `workspace.Match` has no brace alternation | **Any-expansion-matches, nesting binds a comma to its innermost group, an unpaired brace stays literal, empty alternatives match the empty string, cap 1024** — implemented identically in Go and Rust and checked against `micromatch` first | 21 new `braceVectors` + scenario `brace-alternation-expands-the-same-way` in `🏠️workspace/🧪️tests/🃏️glob-matching`, registered in `🐹️.go`, `🦀️.rs` and the micromatch oracle |
| d | `model.Checkpoint.Date` `time.Time` vs `String` | **String** — the raw `--date=iso-strict` log text, which is what `🧬️schema/🔣️.json` already declared. Go changed; `contributors.Checkpoint` collapsed from a duplicate struct to `= model.Checkpoint` | second `Checkpoint` golden in `📐️model`'s `🔣️goldens.json` carrying `"date":"2026-09-06 06:00:00"` — a value no timestamp type can round-trip |
| e | Go `model.Ticket` codec rewrites `goal` by scanning the live goals directory and back-fills `summary` | **Read and write every member verbatim**, as the schema and `decode_ticket_document` do. The goal rewrite, the summary back-fill, the contributor compose-id rewrite and the session-id normalisation are all gone from `UnmarshalJSON`/`MarshalJSON` | `Ticket` added to `wire_type_names` in both languages plus a `Ticket` golden whose `goal` is in compose form; three Go unit tests renamed and rewritten to state the identity contract |
| f | Five Go port names had to differ from their Rust twins | **The port takes the plain name and the repository-global twin is deleted** — no alias, no shim | the forbidden-spelling grep returns nothing; `🧑️contributors`, `🎯️goals`, `📝️todos`, `🎫️tickets`, `🏃️test-runner` parity all green with Go serving every scenario |
| g | `📊️metrics` rename/octal quoting; `requiresDefinitionRequirements` `TrimLeft` cutset | metrics: **already converged** — Go's `UnquoteGitPath`/`ResolveNumstatPath` match Rust line for line and the recorded transcript already carries a row that is simultaneously a cross-directory rename and two octal-quoted emoji paths. statutes: **strip whole space-separated prefix words**, in both languages; the frozen golden moved 69 → 71, exactly the two predicted `🧪️file/🟦️.tsx` entries (`TestComponent`, `TestClass`) | `🔢️numstat-parsing`'s `resolves-renames-and-quoted-paths`; the regenerated `🔣️breaches.json` |

Item (a) deserves a note. The `📐️model` executor's `stripLeadingEntityEmoji` had a real argument
behind it ("the artifact id names the entity once … rather than twice"). It was rejected because the
frozen Rust contract, the ticket-start Go snapshot and `Codebase::build_file_id` all agree on the
other rule, and because `flat` keeping non-ASCII is what `🪪️identity`'s own case pins. Adopting the
strip would have meant changing four implementations and a fixture set to match one; adopting the
snapshot meant deleting nine lines. The two vectors now name the rule so it cannot drift back
silently.

## 4. Fixed in passing

- **`🎛️dashboard` `discover_carries_the_repo_domain_branches` was flaky under parallel test
  execution.** `go_implementation_projects_process_leaves_at_the_go_binary` mutated the process-global
  `SEMIO_REPO_IMPLEMENTATION` while its sibling read it. `inject_repo_domain` now takes
  `RepoImplementation` as an argument and no test touches the environment. 28/28, order-independent.
- **13 clippy findings in `🎛️dashboard`** and 7 in `⌨️cli` (both crates' own sources), plus the
  `no_effect_replace`, two `needless_pass_by_value`, `type_complexity` (`JobQueue` alias) and
  `needless_borrow` in `🔌️mcp` and `should_implement_trait` on `🔗️graphql`'s `Lexer::next` (renamed
  `next_token`).
- **Five stale `🔗️graphql` Go tests** (a fixture path in the deleted legacy tree, three asserting the
  removed `fix` surface, one walking the developer's own checkout for a bundle named `compose/js`) —
  see `📓️opus-consolidation-graphql-tests.md`. That module's Go suite is green for the first time
  since the split.
- `🏠️workspace/📦️packages/🐹️go/🐹️.go` was not `gofmt`-clean; formatted.

## 5. Left open

1. **`semio list` / `search` / `query` render nothing.** Confirmed by A/B against the pre-change
   release binary: it behaved identically before any edit in this session, so it is not a regression
   from the cache or the entity move — the tree itself is 36 MB, so the projection is not empty and
   the fault is in the verb's rendering path. Belongs to the `cli-verbs` owner.
2. **`cargo build --workspace`** fails in `semio-framework-os-kernel` and `semio-framework-graph`,
   both from another fleet's concurrent edits outside `🦑️repo` (§1.1).
3. **`🧪️test/📦️packages/🐹️go` is outside `go.work`** (§1.2) — a documented deviation from `📋️plan.md`
   §3 that belongs to the harness owner.
4. **The `fix` mutation is still in the GraphQL schema** while `RepoContext.Fix` can only error, so
   introspection advertises a field that always fails. Schema edits were out of the graphql-test
   executor's scope; it belongs to the schema owner.
5. **`🖥️server/🎛️coordinator`'s Go files are not `gofmt`-clean** (one trailing blank line).

## 6. Files touched

Rust:
- `🪪️identity/📦️packages/🦀️rust/🦀️.rs` — new `🛤️Paths` region
- `📐️model/📦️packages/🦀️rust/{🦀️.rs, Cargo.toml}` — `🏷️File Kinds`, `🔣️Kind Emojis`, `🪪️Section Ids`, `🎨️Ansi`, `🪪️Entity`; `Ticket` in `wire_types!`; `CodebaseFolder.bundle_id`, `CodebaseFile.folder_id`/`bundle_id`; `identity` dependency
- `🗂️codebase/📦️packages/🦀️rust/🦀️.rs` — moved code replaced by re-exports; folder/file ids filled during the walk
- `🌳️tree/📦️packages/🦀️rust/🦀️.rs` — `DefaultArtifactIdentifier`, `DefaultEntityRenderer`
- `🎫️tickets/📦️packages/🦀️rust/🦀️.rs` — `SystemClock`, `NullIssueTracker`
- `🧑️contributors/📦️packages/🦀️rust/🦀️.rs` — `GitCheckpoints`, `DEFAULT_CHECKPOINT_LIMIT`
- `🔌️mcp/📦️packages/🦀️rust/🦀️.rs` — dead entry points deleted; four clippy fixes; `JobQueue`
- `🔗️graphql/📦️packages/🦀️rust/🦀️.rs` — `Lexer::next` → `next_token`
- `⌨️cli/📦️packages/🦀️rust/{🦀️.rs, 📦️main.rs}` — two re-export regions, three port re-exports, `build_monorepo_tree_cached` and its cache/fingerprint/lock helpers, `run(&[String])`
- `🎛️dashboard/{🌀️daemon,🌳️command-tree,🖥️terminal,📇️playground-catalog}/🦀️.rs` — clippy and the test-isolation fix

Go:
- `📐️model/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}` — `stripLeadingEntityEmoji` deleted, `Checkpoint.Date` string, `Ticket` codec verbatim, `CodebaseFolder`/`CodebaseFile` fields, three tests rewritten
- `🗂️codebase/📦️packages/🐹️go/{🐹️.go, 🔬️_test.go}` — `ParentFolderID`, `OwningBundleID`, folder/file records filled, one test expectation
- `🧑️contributors/📦️packages/🐹️go/🐹️.go` — `Checkpoint` aliased to `model.Checkpoint`, raw date
- `🏠️workspace/📦️packages/🐹️go/🐹️.go` — `gofmt`

Fixtures and schema:
- `📐️model/🧬️schema/🔣️.json` — `bundleId`, `folderId`
- `📐️model/🧪️tests/🔣️json-encoding-conformance/{🧫️fixtures/🔣️goldens.json, 🐹️.go}` — `Ticket` and raw-date `Checkpoint` goldens
- `🗂️codebase/🧫️fixtures/📡️artifact-id-vectors.json`, `🗂️codebase/🧪️tests/🪪️artifact-id-builders/🥒️.feature`

Everything else is listed in the four sub-reports. No git-modifying command was run.
