# 📓️ Opus executor report — `🔨️modules/🗂️codebase`

Agent: Opus 5 executor `codebase` (wave 2). Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/`.
Source of record: the ticket-start Go snapshot `🗑️generated/go-snapshot/client/🧩️component.go`, region `🏩️Codebase`
(21615–22700) plus the `build*ID` helpers at 36522–36631 (fix 2 of `📓️go-region-dependency-graph.md`) and the
supporting symbols those two reach: `LoadTechnologies` / `loadTechnologiesInternal` / `LoadBundles` /
`GetTechnologies` (24229–24503), `ScopeToFiles` / `isRepoExcludedPath` / `filterConsideredFiles` (36196–36290),
`globByExtension` / `LoadGitignore` / `matchesIgnorePattern` (17278–17430), `isGitIgnored` (17016),
`DeriveBundleKind` / `Bundle.GetID` / `normalizeBundleLabel` / `DeriveFolderKind` / `IsGeneratedFolder` /
`DeriveFileKind` (10318–10812), `GetArtifactID` and the five `*KindEmoji` helpers (41318–42200),
`buildFileUriFromPath` (42372), `ParseContributorIdentity` / `findSectionForDefinition` (37000–37025).

## 1. What was built

### 1.1 Rust crate `semio-framework-repo-codebase`

`🗂️codebase/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}`, lib path `🦀️.rs`,
`[lints] workspace = true`, `[package.metadata.semio] role = "library"`, dependencies limited to
`serde`/`serde_json` (`.workspace = true`) plus the four sibling path crates the DAG allows —
`semio-framework-repo-{workspace,identity,model,languages}` — and `semio-framework-repo-yaml` (L0, used for the
`README.md` / `AGENTS.md` front matter the technology walk reads; the Go twin uses the same module for exactly
that). Registered as a root `Cargo.toml` member directly after `📐️model`.

Public surface, grouped by region:

| Region | Surface |
| --- | --- |
| `🛤️Paths` | `dir_of`, `base_of`, `ext_of` — Go `filepath.Dir/Base/Ext` semantics over normalised slash paths, including Go's `Clean` pass |
| `🙈️Ignore` | `is_generated_folder` |
| `🏷️Kinds` | `derive_file_kind`, `normalize_bundle_label` |
| `🗂️Codebase` | `Codebase::{new, root_dir, root_uri, invalidate_technology_cache, technologies, bundles, bundle_by_path, bundle_id, technology_id, derive_bundle_kind, derive_folder_kind, gitignore_patterns, normalize_repo_path, is_repo_excluded_path, is_gitignored, filter_considered_files, glob_by_extension, scope_to_files, build_folder_id, build_file_id, file_uri, folder_uri}` |
| `🪪️Section Ids` | `build_section_id`, `build_definition_id`, `is_test_function_name` |
| `🧭️Scope` | `Scope::{Repo, Technology, Folder, File}` |
| `🔧️Helpers` | `matches_ignore_pattern`, `count_sections`, `extract_file_path`, `find_section_for_definition`, `parse_contributor_identity` |
| `📚️Context` | `CodebaseContext::{new, load_bundles, load_files, with_breachs, bundle_for_file, bundle_info, build_bundles, build_folders, build_files, build_sections, build_definitions, build_breachs, build_tree, build}` |

Two deliberate departures from a literal transcription, both required by the plan rather than optional:

- **No process-global root.** The Go original keeps `rootDir`, `technologyCache`, `folderKindCache` and
  `cachedGitignore` in package-level globals. The Rust crate binds all four to a `Codebase` handle, so a walk can
  be pointed at any root without a process-wide mutation. This is what makes the fixture-tree cases possible at
  all, and it is also why the Go adapters have to call `workspace.SetRootDir` + `codebase.InvalidateTechnologyCache`
  where the Rust ones simply construct a second `Codebase`.
- **Contributors, tickets and policies are not projected here.** `LoadBreachs`/`LoadTickets`/`LoadPolicies` in the
  Go original call `AnalyzeFile` (statutes, L2 sibling), `ListTickets` (tickets, L3) and `GetPolicies` (statutes),
  and `BuildCodebaseContributors` calls `ListContributors` (contributors, L3). Every one of those is an upward edge
  the plan's DAG forbids from L2. Breaches therefore enter as already-decoded `model::Breach` records through
  `CodebaseContext::with_breachs` (the whole breach→bundle/folder/file fan-out — `build_breachs`, the per-file
  `FileBreachRef` list and the per-bundle/folder breach counts — is implemented here and driven off those records),
  and the ticket, policy and contributor aggregates stay `None` in `CodebaseContext::build`, to be filled by their
  owning L3 modules. `model::Breach` already carries `priority`/`autofixable`/`reason`/`solution` inline, which is
  what `Statute.Info()` supplies in Go, so no port was needed for that either.

Behaviour that is a literal transcription and was checked line by line against the snapshot: the lexical
`os.ReadDir` ordering of both the technology walk and `glob_by_extension` (Rust `read_dir` is unordered, so entry
names are sorted explicitly), the `SkipDir` on any directory whose name starts with `.`, the two-stage ignore
(`matches_ignore_pattern` during the descent, then `filter_considered_files` afterwards), the longest-root bundle
match, the `repo/repo` fallback label and its unconditional seeding of the bundle aggregate, the `README.md` /
`AGENTS.md` exclusion from bundle metrics, the `sites` one-level expansion, the `bundleKind` manifest override,
the `sourceRoot`/`tags` manifest read, the folder-kind indicator list plus the `.csproj`/`.sln` scan, the whole
`DeriveFileKind` decision table in its original order, the lab-file test reclassification inside
`build_definition_id`, and the `path < path` sort applied to the folder, file, section and definition aggregates.

### 1.2 Language-agnostic cases

Four cases under `🗂️codebase/🧪️tests/`, each `{🥒️.feature, 🦀️.rs, 🐹️.go}`, plus three shared fixtures under
`🗂️codebase/🧫️fixtures/` and the module schema `🗂️codebase/🧬️schema/🔣️.json` describing both the fixture shapes
and the four projection shapes.

| Case | Scenarios | What is compared |
| --- | --- | --- |
| `🚶️workspace-walk` | `walk-reports-the-same-considered-files`, `walk-projects-the-same-bundle-and-folder-aggregates` | the considered-file list and its order; the bundle and folder aggregates (`id\|folder\|folders\|files\|lines`, `path\|id\|files\|lines`) |
| `🪪️artifact-id-builders` | `ids-agree-for-every-vector`, `uris-agree-for-every-file-vector` | 21 folder/file/section/definition id vectors and the 7 file uris |
| `📦️technology-bundle-detection` | `technologies-and-bundles-agree`, `bundle-ids-and-labels-agree` | detected technology and bundle rows (name, root, kind, emoji, source root, tags) and each bundle's emoji id and normalised label |
| `🙈️ignore-integration` | `every-vector-gets-the-same-three-verdicts`, `filtering-a-walk-drops-the-same-paths` | 20 paths × the three independent verdicts (structural exclusion, gitignore, generated folder), and the kept/dropped split of a raw glob |

All eight scenarios carry `@id-`, `@level-fundamental`, `@mode-differential`; every feature carries
`@comparison-ordered-json-v1`. The shared fixture `📡️repo-tree.json` is one synthetic repository (two
technologies, four bundles including one site bundle, a gitignored folder, a generated folder, a vendored tree, a
hidden directory, a non-source file) that every case materialises into its own `work_dir/🌳️tree` before walking
it — fixtures stay immutable, mutation happens on the copy, per the harness rule.

**Oracle decision.** The task asked whether the TypeScript library's `🔍️discovery/🟦️.ts` exposes equivalent id
functions. It does not: I read its export list and grepped it for `buildFileId`/`buildFolderId`/`emojiText`/
`*KindEmoji`; its `artifactId` is a *directory name* inside the taxonomy contracts, carries no emoji chain, and
there is no id builder of any kind in that module. So `🗂️codebase/🔮️oracle/🔣️.json` records **four
`noOracleDecisions` and no oracles** — `repo-codebase-walk`, `repo-codebase-artifact-ids`,
`repo-codebase-technology-detection`, `repo-codebase-ignore-integration` — each with
`substitutes: ["independent-implementations", "specification-vectors"]` (the ignore one uses
`metamorphic-laws` as its second substitute, since the gitignore semantics themselves are already judged against a
real engine by `🏠️workspace`'s own oracle-backed cases and what this case adds is how the three verdicts combine).
Because every scenario is `@mode-differential`, those decisions are *not* self-discharging: the harness requires
two implementations to actually run and agree (see §3).

### 1.3 Wiring

- `📋️project.json` → `@semio-tech/repo-codebase-rs` with `build`, `test`, `test-quick`, `test-long`,
  `test-exhaustive`, each `bun ./📜️script.ts <cmd>`, mirroring `@semio-tech/repo-model-rs` exactly.
- `📜️script.ts` extends `BundleScript`/`ScriptRouter` from the library, `build` → `cargo build -p`, `test` →
  `runCargoTestBudgeted`.
- Root `Cargo.toml`: member added.
- `.vscode/🧩️launch.seed.jsonc`: `🧪️test🧰️repo🗂️codebase🦀️rust`, `…🐹️go`, `…🥒️parity` inserted in the existing
  group before the `🗣️languages` block; `.vscode/launch.json` regenerated with the plugin registry script.

## 2. Verification — real command output

Environment: `RUSTC_WRAPPER=""`, `GOWORK=C:/git/semio/go.work`, `SEMIO_TEST_BUDGET_MS=600000`, Windows 11, bun 1.4.2.
Everything below is verbatim output of a run made **after** the two fixes of §2.5 and §2.6 landed.

### 2.1 Crate builds, tests and is clippy-clean

```
$ cargo test -p semio-framework-repo-codebase
   Compiling semio-framework-repo-codebase v0.1.0 (…\🗂️codebase\📦️packages\🦀️rust)
    Finished `test` profile [unoptimized] target(s) in 2.78s
     Running unittests 🦀️.rs (target\debug\deps\semio_framework_repo_codebase-aca83866229bf7c9.exe)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests semio_framework_repo_codebase
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Zero in-crate unit tests is deliberate, not an omission: this module's behaviour is pinned by the four
language-agnostic cases under `🧪️tests/`, which is what the repo's multi-implementation rule asks for. A `#[test]`
here could only assert against the same Rust implementation it is testing.

```
$ cargo clippy -p semio-framework-repo-codebase --all-targets 2>&1 | grep -E "^(warning|error)"
warning: this `if` can be collapsed into the outer `match`
warning: `semio-framework-repo-workspace` (lib) generated 1 warning …
warning: called `map(<f>).unwrap_or(<a>)` on an `Option` value
warning: called `map(<f>).unwrap_or(<a>)` on an `Option` value
warning: `semio-framework-repo-identity` (lib) generated 2 warnings …
```

**Zero warnings attributable to this crate.** The three that remain belong to `🏠️workspace` (1) and `🪪️identity`
(2) and are pre-existing; they are unchanged by the `identity::flat` fix of §2.6.

All twenty repo crates still build together after that fix:

```
$ cargo build -p semio-framework-repo-{workspace,identity,yaml,search,model,languages,events,providers,
    codebase,statutes,metrics,tickets,goals,contributors,todos,tree,move,test-runner,hooks,graphql}
   … EXIT=0
```

(`cargo build --workspace` still fails, but in `semio-framework-graph` under `🧰️framework/🔨️modules/🕸️graph` —
a build-script failure on its generated `🦀️registry.rs`, a different product area, unrelated to this ticket and
pre-existing.)

### 2.2 Harness — subject, both implementations

```
$ … parity fundamental --owner 🗂️codebase
[test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=6/8
```

**16 of 16 subject executions pass — 8 Rust and 8 Go.** The Go side now compiles, runs and produces results for
every scenario, which is the thing that was outstanding when this report was first written.

### 2.3 Harness — oracle

```
$ … oracle fundamental --owner 🗂️codebase
[test] not-exercised …/🪪️artifact-id-builders (recorded no-oracle decision repo-codebase-artifact-ids — its evidence is discharged by the subject phase)
[test] not-exercised …/🙈️ignore-integration (recorded no-oracle decision repo-codebase-ignore-integration — …)
[test] not-exercised …/📦️technology-bundle-detection (recorded no-oracle decision repo-codebase-technology-detection — …)
[test] not-exercised …/🚶️workspace-walk (recorded no-oracle decision repo-codebase-walk — …)
[test] level=fundamental cases=4 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=4
```

Green and expected: no oracle role is claimed by any of these four features.

### 2.4 Harness — parity: 6 of 8

```
$ … parity fundamental --owner 🗂️codebase
[test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=6/8
[test] cross-subject parity failed: …::📦️technology-bundle-detection::technologies-and-bundles-agree::go~rust (4 differences)
[test] cross-subject parity failed: …::🚶️workspace-walk::walk-projects-the-same-bundle-and-folder-aggregates::go~rust (4 differences)
```

Green: both `🪪️artifact-id-builders` scenarios, both `🙈️ignore-integration` scenarios,
`📦️technology-bundle-detection::bundle-ids-and-labels-agree`, and
`🚶️workspace-walk::walk-reports-the-same-considered-files`.

The two remaining failures share **one** root cause, and it is not in this crate — see §3.1. Every other value in
those two projections agrees exactly, ids included.

### 2.5 Fix 1 — the Go subject hung forever; the fixture was not self-rooting

The previous run of this module reported the Go phase as "blocked on `go-split`" after a
`go run … exceeded 600000ms — killed`. That diagnosis was wrong, and the 600 s budget was hiding the real fault.
Measured separately, the generated Go host **builds in 2 s** and then **hangs indefinitely at runtime** (killed at
120 s and at 540 s, no output, no results stream).

Cause: the adapters call `workspace.SetRootDir(root)`, but that function does not store `root` —

```go
func SetRootDir(dir string) {
	RootDir = FindRepoRoot(dir)      // ← ascends, does not keep `dir`
	…
}
```

and `FindRepoRoot` ascends looking for `repo/cli/main.go`, then `.git`, then `go.mod`. The materialised fixture
tree lives at `<workDir>/🌳️tree`, and the work directory is itself **inside the monorepo**
(`.🧬semio/🦑️repo/⚡️cache/tests/work/…`). The tree carries none of those three markers, so the ascent walked out
of the fixture and landed on `C:\git\semio` — and the Go walk then scanned the entire monorepo, `target/`,
`node_modules/` and `.git/` included. That is the hang.

Fix, in the shared fixture rather than in four adapters — one entry, both languages, no adapter code:

```json
{ "path": ".git/.keep", "content": "" },
```

It makes the tree self-rooting, so `FindRepoRoot` stops at the tree. It is invisible to every projection because
**both** walks skip dot-directories (Rust `🦀️.rs:631` `is_dir && name.starts_with('.')`, Go `🐹️.go:1432`), the
same way the tree's pre-existing `.hidden/` folder is skipped. Confirmed immediately after:

```
$ … subject fundamental --case 🚶️workspace-walk --implementation go
[test] level=fundamental cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0
```

Seconds, not 600 s. **The Go compile error quoted in the first version of this report is gone**: `go-split` has
landed a compiling `📜️statutes`, and `go build ./...` in `🗂️codebase/📦️packages/🐹️go` exits 0.

### 2.6 Fix 2 — `identity::flat` dropped every emoji (Rust was wrong, Go was right)

With the Go phase finally running, parity came in at **3/8**. All five failures were one bug. Go and Rust
disagreed on every id built from a name that contains an emoji:

```
go   : bundle-root-folder=🎼compose🖥🖥️app
rust : bundle-root-folder=🎼compose🖥app
```

Ground truth is the snapshot's `Flat` (`🧩️component.go:17504`), which keeps `A-Za-z0-9` **and every rune above
`0x7F`**, then lower cases:

```go
if (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') || r > 0x7F {
	buf.WriteRune(r)
}
```

`Flat("🖥️app")` is therefore `🖥️app`, and `GetArtifactID`'s folder case is `folderKindEmoji + Flat(name)` =
`🖥` + `🖥️app`. **Go was correct; the Rust `identity::flat` was wrong** — it lower cased first and then filtered
to `a-z0-9`, discarding all non-ASCII, so an artifact id lost the emoji its name already carried. Rewritten in
`🪪️identity/📦️packages/🦀️rust/🦀️.rs` to mirror Go exactly (filter first on the original runes, then a simple
non-special-casing fold, matching the existing `go_to_upper` helper).

This is a foundation-module edit made from a consumer module, so it was checked for blast radius rather than
assumed safe. Most `flat` callers pass ASCII slugs (`ticket.slug`, `draft.id`, `policy.id`, `todo.id`) and are
unaffected by construction; the change only moves ids built from names that contain non-ASCII, which is this
module's domain. Verified:

```
$ … parity fundamental --owner 🪪️identity
[test] level=fundamental cases=2 executed=5 passed=5 failed=0 errored=0 parity=4/4
```

```
$ … parity fundamental --owner 🏃️test-runner
[test] level=fundamental cases=5 executed=15 passed=15 failed=0 errored=0 parity=3/3
[test] …/🧬️scope-identifier-normalisation: go subject host exited 1 without emitting results
[test] # semio.test/host
.\adapter.go:57:40: undefined: client.Flat
.\adapter.go:69:36: undefined: client.PathFromUriPath
```

`🏃️test-runner` is the one other module whose cases exercise `flat` directly. Its Rust side passes and all three
parity comparisons that ran agree; the single failing case is **pre-existing and unrelated** — that case's Go
adapter still imports the old `client` package `go-split` is dismantling, so it cannot compile. It is owed by
`🏃️test-runner`'s owner, not by this fix.

Parity went 3/8 → 6/8 on this change alone.

## 3. Findings for other owners

### 3.1 `🗂️codebase` Go — `Bundle.Root` carries OS-native separators (the only parity blocker left)

The two failing scenarios differ in exactly one field, the bundle root:

```
go   : compose/🌐️web|compose\sites\🌐️web|site||compose/sites/🌐️web/app|
rust : compose/🌐️web|compose/sites/🌐️web|site||compose/sites/🌐️web/app|
```

Everything else — ids, kinds, emojis, source roots, tags, and every `Folders|Files|Lines` metric — agrees. Go
builds the value with `filepath.Join`, which yields backslashes on Windows:

- `🗂️codebase/📦️packages/🐹️go/🐹️.go:1144` `bundlePath := filepath.Join(name, bunName, siteName)` → `:1147` `Root: bundlePath`
- `🗂️codebase/📦️packages/🐹️go/🐹️.go:1191` `bundlePath := filepath.Join(name, bunName)` → `:1196` `Root: bundlePath`

The port is **faithful** — the snapshot does the same at `24351`/`24398` — so this is a defect inherited from the
original Go, not a porting mistake. It is nonetheless a real one: the same test yields different domain values on
Windows and on Linux, and Go is internally inconsistent about it, since `Folder.Path` in the very same projection
is already slash-normalised (`compose/sites/🌐️web`). Rust normalises via `normalize_repo_path`, which is the
repo-wide contract for a repo-relative path and the platform-stable choice.

Fix, owed by whoever owns `🗂️codebase/📦️packages/🐹️go` (`go-split` today — this agent was instructed not to edit
the Go packages): wrap both assignments in `filepath.ToSlash(...)`. That single change should take parity to 8/8.
It was deliberately **not** papered over in the adapter: normalising the separator in the projection would have
hidden a genuine cross-platform divergence behind a green test.

### 3.2 `🏠️workspace` — a directory-only gitignore rule is honoured by **neither** implementation

Previously recorded as a Rust-only defect that Go parity would expose. It is worse than that: with the Go side now
running, both implementations return the same verdicts, so **parity is green while both are wrong**.

```
go   : gitignored-folder-member=false,false,false
rust : gitignored-folder-member=false,false,false
       gitignored-extension=false,true,false     (both — the file-glob rule works in both)
```

With a root `.gitignore` containing `ignored/`, the path `compose/🖥️app/ignored/💀️dead.ts` is reported as **not**
gitignored by both. Git itself treats a trailing-slash pattern with no leading slash as matching that directory at
any depth, so the middle verdict should be `true` and the file should be dropped by `filter_considered_files`.

This is exactly the failure mode a differential test cannot catch, and it is the concrete argument for the
`🙈️ignore-integration` case eventually gaining a real oracle instead of resting on
`independent-implementations` + `metamorphic-laws`. The vector is deliberately left in place as the discriminating
input. Owner: `🏠️workspace`.

### 3.3 Ten Go packages use a non-conventional nx project name — the launch entries do not resolve

`🗂️codebase/📦️packages/🐹️go/📋️project.json` is named `framework-products-repo-modules-codebase-go`, but the
established convention — used by the Rust twin (`@semio-tech/repo-codebase-rs`) and by twelve other Go packages
(`@semio-tech/repo-{workspace,identity,yaml,search,model,languages,events,providers,graphql,mcp,metrics,test-runner}-go`)
— is `@semio-tech/repo-<module>-go`. Confirmed to be a live breakage, not a cosmetic one:

```
$ bun nx show project @semio-tech/repo-codebase-go
Could not find project @semio-tech/repo-codebase-go
```

Ten packages share the deviation: `codebase`, `cli`, `contributors`, `goals`, `hooks`, `move`, `statutes`,
`tickets`, `todos`, `tree`. The launch entry `🧪️test🧰️repo🗂️codebase🐹️go` was left pointing at the **conventional**
name rather than bent to the current one: per `CLAUDE.md` this is a greenfield tree that must converge on the
clean name, and renaming ten `project.json` files is a single coordinated edit owed by `go-split`, not something
to work around here. Until that lands, that one launch entry fails; the `🦀️rust` and `🥒️parity` entries both work.

### 3.4 Taxonomy mismatch with the Go split (unchanged, still open)

Plan §2 and fix 2 of `📓️go-region-dependency-graph.md` put the artifact id builders in `codebase`. `go-split`
instead placed `BuildFileID`, `BuildFolderID`, `BuildSectionID`, `BuildDefinitionID`, `DeriveFileKind`,
`DeriveFolderKind`, `IsGeneratedFolder`, `NormalizeBundleLabel`, `ScopeToFiles`, `FilterConsideredFiles` in
`📐️model`, and `GlobByExtension`, `IsGitIgnored`, `IsRepoExcludedPath` in `🏠️workspace`. The Rust crate owns all
of them in `codebase` as the plan prescribes. All four Go adapters import
`github.com/usalu/semio/repo/codebase` as required, plus `model` and `workspace` for the symbols that live there
today — visible in their import blocks, and the concrete evidence of the divergence. Behaviour is identical either
way; only ownership differs, and a coordinator decision is still needed on which side moves.

### 3.5 `derive_folder_kind` / `is_generated_folder` are consulted by the id builders

Unchanged: anyone moving them must keep them below `codebase`, or the id builders move with them.

### 3.6 Every nx Rust `test` target in the repo is blocked by a missing generated file

The `🧪️test🧰️repo🗂️codebase🦀️rust` launch entry runs `bun nx run @semio-tech/repo-codebase-rs:test`. The project
name resolves and the target is wired correctly, but the run dies before reaching cargo:

```
$ bun nx run @semio-tech/repo-codebase-rs:test
error: Invalid taxonomy schema:
- generatorContracts["wgpu-frame-worker"] tracked output
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js" is missing.
    at loadTaxonomy (…/📚️library/🔍️discovery/🟦️.ts:1288:38)
    at getCargoWorkspaceIndex … at resolveCargoPackageName … at runCargoTestBudgeted
```

This is **repo-wide and pre-existing**, not specific to this module — `@semio-tech/repo-model-rs:test` fails with
the identical error. `runCargoTestBudgeted` calls `loadTaxonomy()` to resolve a crate name, and `loadTaxonomy`
validates *every* generator contract in the workspace, so one missing generated artefact in the wgpu renderer
blocks every Rust test target in the monorepo. Direct `cargo test -p semio-framework-repo-codebase` is unaffected
and green (§2.1). Owner: whoever owns `📺️renderer/🧊️wgpu` (regenerate `🎞️frame-worker/🤖️generated/🟨️.js`), with a
question for `📚️library` about whether resolving one crate name should require validating the whole workspace.

### 3.7 A harness note — the 600 s budget masked a hang for a whole session

`materializeGoHost` writes a `go.mod` requiring **every** workspace Go module, so a cold Go host compile is
genuinely slow and a timeout there looks exactly like a compile that is merely slow. It cost this module a full
session of misdiagnosis (§2.5). Worth considering for `🧪️test`: report build and run as separate phases with
separate budgets, so "cannot compile", "compiles slowly" and "compiles and then hangs" stop being one symptom.
The earlier `📓️harness-verification.md` §2 gap — `materializeGoHost` having no equivalent of `rustSutCrate` —
remains **not reproduced**: the generated host resolved `github.com/usalu/semio/repo/codebase` and its whole
`replace` chain without help.

## 4. What is left

1. **`filepath.ToSlash` on `Bundle.Root`** at `🗂️codebase/📦️packages/🐹️go/🐹️.go:1144` and `:1191` (§3.1). One
   line each, owed by `go-split`. This is the **only** thing between this module and 8/8 parity; re-run
   `… parity fundamental --owner 🗂️codebase` after it lands.
2. **Rename the ten non-conventional Go nx projects** to `@semio-tech/repo-<module>-go` (§3.3), which repairs the
   `🧪️test🧰️repo🗂️codebase🐹️go` launch entry and nine siblings. Owed by `go-split`.
3. **The directory-only gitignore rule** (§3.2) — a shared defect in `🏠️workspace` that parity cannot see. Owed by
   the foundation executor. When it is fixed, `🙈️ignore-integration`'s `gitignored-folder-member` vector flips to
   `false,true,false` and one file leaves the considered set, so both ignore scenarios and
   `walk-reports-the-same-considered-files` will need their projections re-confirmed.
4. **`🏃️test-runner`'s `🧬️scope-identifier-normalisation` Go adapter** still imports the dismantled `client`
   package (`client.Flat`, `client.PathFromUriPath`) and cannot compile (§2.6). Pre-existing, owed by that module.
5. **Regenerate `📺️renderer/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`** (§3.6) — until it exists, *no* nx Rust
   `test` target in the repo runs, this module's launch entry included. Pre-existing and repo-wide; `cargo test`
   directly is unaffected.
6. **The contributor, ticket and policy aggregates** of the Go `BuildCodebase` remain intentionally unimplemented
   here (§1.1); `CodebaseContext::build` leaves those three fields `None` for `🧑️contributors`, `🎫️tickets` and
   `📜️statutes` to fill.
7. **A coordinator decision on §3.4**, the id-builder ownership split between `codebase` and `model`/`workspace`.
8. **`repo contract`** over the four cases: a repo-wide scan of many minutes, reported nothing attributable to
   `🗂️codebase` before it was stopped. Should be run to completion by the wave-5 audit.

## 5. Files touched

Created (first pass):
- `🔨️modules/🗂️codebase/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}`
- `🔨️modules/🗂️codebase/🧬️schema/🔣️.json`
- `🔨️modules/🗂️codebase/🔮️oracle/🔣️.json`
- `🔨️modules/🗂️codebase/🧫️fixtures/{📡️repo-tree.json, 📡️artifact-id-vectors.json, 📡️ignore-vectors.json}`
- `🔨️modules/🗂️codebase/🧪️tests/🚶️workspace-walk/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `🔨️modules/🗂️codebase/🧪️tests/🪪️artifact-id-builders/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `🔨️modules/🗂️codebase/🧪️tests/📦️technology-bundle-detection/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- `🔨️modules/🗂️codebase/🧪️tests/🙈️ignore-integration/{🥒️.feature, 🦀️.rs, 🐹️.go}`
- this report

Updated (first pass):
- root `Cargo.toml` (one member line)
- `.vscode/🧩️launch.seed.jsonc` (three entries) and the regenerated `.vscode/launch.json`

Updated (this pass):
- `🔨️modules/🗂️codebase/🧫️fixtures/📡️repo-tree.json` — the `.git/.keep` self-rooting marker and its comment (§2.5)
- `🔨️modules/🪪️identity/📦️packages/🦀️rust/🦀️.rs` — `flat` rewritten to Go's rule (§2.6)
- `.vscode/launch.json` — regenerated with
  `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`
  (`plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages)`,
  `.vscode/launch.json regenerated`); all three `🗂️codebase` entries present in seed and generated file
- this report

No temporary output was left under `🗑️generated/`: this pass's command output went to the session scratchpad
outside the repository, and the four `🗑️generated/codebase/*.txt` files the first pass mentioned are gone.
