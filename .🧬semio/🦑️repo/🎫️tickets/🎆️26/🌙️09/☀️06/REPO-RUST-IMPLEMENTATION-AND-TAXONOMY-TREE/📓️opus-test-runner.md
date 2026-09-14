# 📓️ Opus executor report — `🔨️modules/🏃️test-runner`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/` (new, L3 of the plan's DAG).
Source of truth: the frozen Go snapshot `🗑️generated/go-snapshot/client/🧩️component.go`, regions
`🕸️Test Command` (1513–2125) and `🖲️Missing Test Functions` (43679–44008) plus the three
`💾️Missing Utility Functions` entries only test-file resolution calls.

## 1. What was built

```
🏃️test-runner/
├── 🧬️schema/🔣️.json                      JSON Schema 2020-12 — snapshot, plan, transcripts, TestOutcome, 5 fixture envelopes
├── 🔮️oracle/🔣️.json                      owner oracle contribution (1 oracle, 2 no-oracle decisions)
├── 🧫️fixtures/                            5 shared fixtures (detection, planning, transcripts, selectors, cancellation)
├── 🧪️tests/
│   ├── 🧭️runner-detection/               🥒️.feature + 🦀️.rs
│   ├── 🗺️invocation-planning/            🥒️.feature + 🦀️.rs
│   ├── 📊️result-parsing/                 🥒️.feature + 🦀️.rs (subject) + 🟦️.ts (oracle: second parser)
│   ├── 🛑️cancellation/                   🥒️.feature + 🦀️.rs
│   └── 🧬️scope-identifier-normalisation/ 🥒️.feature + 🦀️.rs + 🐹️.go
└── 📦️packages/
    ├── 🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}
    └── 🐹️go/{go.mod, 🐹️.go, 📋️project.json, 📜️script.ts}
```

Registered: root `Cargo.toml` member, root `go.work` use, `.vscode/🧩️launch.seed.jsonc` (3 entries,
regenerated into `.vscode/launch.json`).

## 2. Rust crate — `semio-framework-repo-test-runner`

`[lib] path = "🦀️.rs"`, `serde`/`serde_json` workspace, path deps on **both** sibling crates that
already existed at start: `semio-framework-repo-model` (`Bundle` → `From<&Bundle> for SnapshotBundle`)
and `semio-framework-repo-workspace` (`glob_match` for the `*.csproj` / `*.sln` probes). Zero clippy
warnings under the workspace lint set.

### Regions and public API

**`🪪️IdentityPending`** — `EntityIdentity` and `LanguageTable` traits + `PendingIdentity`,
`flat`, `path_from_uri_path`, `language_of_extension`.
*Recorded gap*: `id_to_uri` needs `DetectEntityKindFromId` and the entity-emoji table, which
`🪪️identity`'s Rust crate does not export yet, so `PendingIdentity::id_to_uri` answers `""` — Go's
own answer for an id whose kind it cannot detect. Callers passing `repo://…` URIs (CLI and MCP both
do) are unaffected. When `🪪️identity` exports it, this region collapses to a re-export.

**`🧮️Paths`** — `normalize_separators`, `is_absolute_path` (POSIX *and* Windows drive roots, so one
fixture runs on every host), `join_path`, `clean_path`, `base_name`, `dir_name`, `relative_path`.

**`🌲️Snapshot`** — `EntryKind`, `SnapshotEntry`, `SnapshotBundle`, `FilesystemSnapshot`
(`absolute`, `file_exists`, `directory_exists`, `read_text`, `glob_children`, `walk_files`,
`bundle_by_name`, `bundle_by_path`), `detect_bundle_language`.

**`🔭️Scope`** — `ScopeKind`, `TestScope`, `resolve_test_scopes`, `resolve_test_scope`,
`resolve_test_scope_from_bundle_sub_path`, `resolve_test_scope_from_file_sub_path`.

**`🗺️Planning` (pure)** — `Runner` (`go`, `cargo`, `cargo-nextest`, `dotnet`, `npx`, `npm`, `uv`,
`pytest`, `rspec`), `RunnerInvocation { runner, argv, cwd, env, filter }`, `InvocationPlan`,
`plan_scopes`, `plan_scope`, `plan_all`, `plan_technology`, `plan_bundle`, `plan_file`,
`plan_section`, `plan_definition`, `detect_js_test_runner`, `collect_go_tests_in_section`,
`resolve_test_function_name`, `unflatten_test_name`.
Given a snapshot and a scope the plan is fully determined — no process, no clock, no ambient
filesystem — which is what makes an argv list a golden.

**`⚙️Execution`** — `ProcessRequest`, `ProcessOutput`, `ProcessRunner` (trait),
`RecordedProcessRunner`, `CancellationToken`, `ProgressEvent`, `ExecutionReport`, `execute_plan`.
*Recorded for consolidation*: `ProcessRequest` / `ProcessOutput` / `ProcessRunner` are declared here
because `🧩️providers` does not exist yet; they are the exact shape that module should own for every
shelled-out command (git, gh, devcontainer, runners). Consolidating means deleting the block and
importing the identical trait — no call site changes. The Go package declares the same three shapes.

**`📊️Parsing`** — `TestStatus`, `RunStatus`, `TestCaseOutcome`, `TestTotals`, `TestOutcome`,
`parse_outcome`, plus `parse_go_test_json`, `parse_vitest_json`, `parse_pytest`, `parse_cargo_test`,
`parse_cargo_nextest`, `parse_dotnet_test`, `parse_rspec_json`. Totals are always recounted from the
parsed tests, never read from the report's own summary line.

**`🔎️Resolution`** — `PYTHON_TEST_RUNNER_BINS`, `JS_TEST_RUNNER_BINS`, `CommandKind`,
`split_command_segments`, `classify_command_kind`, `extract_test_segment_from_command`,
`is_test_command_segment`, `trim_pipeline_tail`, `resolve_test_files_from_command`,
`resolve_{go,cargo,dotnet,python,pytest,rspec}_test_files`, `find_js_test_files`.
*Recorded duplication*: `split_command_segments` and `classify_command_kind` belong to `🪝️hooks`
(plan.md §2) but are the direct dependency of `extract_test_segment_from_command`; they are ported
here in full and must be de-duplicated when `🪝️hooks` lands.

**`🧫️Vectors`** — `DetectionVector(s)`, `PlanningVector(s)`, `SelectorVectors`,
`TranscriptVector(s)`, `CancellationVectors`, `parse_*_vectors`, and the concrete renderers
`plan_to_json_text`, `scope_to_json_text`, `outcome_to_json_text`, `report_to_json_text`,
`progress_to_json_text`. Renderers are concrete on purpose so no serialization type from outside this
codebase crosses the public API.

### Behavioural fidelity to Go

Every planning decision reproduces the snapshot, including two behaviours that look like defects and
were deliberately preserved and pinned as goldens:

- `resolveTestScope`'s `f/<path>` route splits on `/` with `SplitN(rest, "/", 2)`, so a file URI whose
  path contains a separator resolves the FIRST segment as the file. `PathToUriPath` does not encode
  `/`, so this is reachable in practice. Pinned in `🧫️fixtures/🧬️scope-selectors.json`.
- `collectGoTestsInSection` strips exactly ONE leading non-ASCII rune from a region name, so a region
  written `🔖️Alpha` (emoji + VS-16) never matches section `Alpha` — the VS-16 survives the strip and
  `Flat` keeps it. The fixture's Go file therefore uses `Alpha` and `🚀Gamma` and both goldens are
  pinned.

Execution and parsing are **net new**: Go streams runner output straight to the terminal
(`runExternalCommand`) and never parses it, has no cancellation and no progress. `cargo nextest` and
`rspec` have no Go planning counterpart either — they exist in the outcome model because
`classifyCommandKind` and `resolveTestFilesFromCommand` already know both, and the plan's §2 charter
for this module names rspec.

## 3. Go package — `github.com/usalu/semio/repo/testrunner`

`go 1.25`, no dependencies. Contains:

- `🏃️Pending` region: `PendingSymbols()` — the 34-entry move list (name, snapshot line, region) the
  `go-split` agent brings here — and `PendingElsewhere()` — the 6 symbols inside the same regions
  that belong to `⌨️cli`, `🪝️hooks`, `🪪️identity` and `🗂️codebase` instead, so the split agent does not
  over-move.
- `⚙️Process Port` region: `ProcessRequest`, `ProcessOutput`, `ProcessRunner`, `RecordedTranscript`,
  `RecordedProcessRunner` — the same shapes as the Rust twin, declared here so a moved symbol finds
  its port already in place.

`gofmt -l` clean, `go vet ./...` clean, `go build ./...` clean.

## 4. Language-agnostic tests

| Case | Adapters | Scenarios | Oracle |
| --- | --- | --- | --- |
| `🧭️runner-detection` | rust | 2 fundamental (conformance) + 1 quick (property) | `@no-oracle-repo-test-runner-planning` |
| `🗺️invocation-planning` | rust | 2 fundamental (conformance, error) + 1 quick (property) | same |
| `📊️result-parsing` | rust (subject) + typescript (oracle) | 3 fundamental (differential, conformance, error) | `@oracle-repo-test-runner-second-parser` |
| `🛑️cancellation` | rust | 3 fundamental (2 conformance, 1 error) | `@no-oracle-repo-test-runner-planning` |
| `🧬️scope-identifier-normalisation` | rust + go | 2 fundamental (differential) | `@no-oracle-repo-test-runner-scope-identifiers` |

Every adapter **asserts** its frozen vectors (returns `Err` on any mismatch) rather than only
projecting them — otherwise a single-implementation conformance case would pass while disagreeing
with its own golden. That assertion immediately caught a real defect: `FilesystemSnapshot.uv_available`
had no `#[serde(rename = "uvAvailable")]`, so the schema-declared fixture key was silently ignored and
every Python bundle planned `pytest` instead of `uv run pytest`. Fixed.

### Oracle decisions

- `repo-test-runner-second-parser` (kind `cross-semio-implementation`, ecosystem javascript): the
  `🟦️.ts` adapter is an independently written second parser for all six dialects, run in the **oracle**
  role. `vitest` IS a root dependency and was evaluated as a real third-party oracle: it can *produce*
  a reporter document but cannot consume one, so it is evidence for the fixture's shape, not for the
  parse. No other npm/crate library parses all six dialects into one model, and four of the six have
  no parser library at all — recorded in the entry's `rationale`.
- `repo-test-runner-planning` (`specification-vectors` + `metamorphic-laws`): scope→argv is semio's
  own vocabulary; the vectors carry the exact expected language and argv transcribed from Go, and the
  features assert three metamorphic laws (narrowing never widens; a filter only appends; planning is
  idempotent).
- `repo-test-runner-scope-identifiers` (`independent-implementations` + `specification-vectors`):
  Go client × Rust crate, compared pairwise by the harness.

### What the Go adapter covers, and precisely what it does not

`🧬️scope-identifier-normalisation/🐹️.go` imports the CURRENT `github.com/usalu/semio/repo/client`,
marked `// 🚚️ repoint to github.com/usalu/semio/repo/testrunner after split`, and covers
`client.Flat` and `client.PathFromUriPath` — the only two rules of this domain that are **exported**
today. Everything else this module ports is unexported in package `client` and therefore unreachable
from any external Go adapter: `testScopeKind`, `testScope`, `resolveTestScope(s)`,
`resolveTestScopeFrom{Bundle,File}SubPath`, `findBundleByName`, `detectBundleLanguage`,
`detectJSTestRunner`, `uvExists`, `runTestScope`, `runAllTests`, `runTechnologyTests`,
`runBundleTests`, `runFileTests`, `runSectionTests`, `runDefinitionTest`,
`collectGoTestsInSection`, `resolveTestFunctionName`, `unflattenTestName`, `runExternalCommand`, and
the whole `🖲️Missing Test Functions` file-resolution family. Those behaviours are covered today by the
Rust adapters against the frozen vector tables, and become Go-reachable the moment `go-split` moves
the symbols listed in `🏃️Pending`. At that point the Go adapters of `🧭️runner-detection`,
`🗺️invocation-planning` and `📊️result-parsing` can be added and their `@mode-conformance` scenarios
upgraded to `@mode-differential`.

The Go host gap flagged in `📓️harness-verification.md` §2 (no domain SUT wiring in
`materializeGoHost`) was **already fixed by another agent** before I needed it —
`goWorkspaceModules` now `require`s + `replace`s every `go.work` member — so the Go adapter compiles
and runs.

## 5. Verification — real command output

```
$ cargo build -p semio-framework-repo-test-runner
   Compiling semio-framework-repo-test-runner v0.1.0 (…\🏃️test-runner\📦️packages\🦀️rust)
    Finished `dev` profile [unoptimized] target(s)

$ cargo clippy -p semio-framework-repo-test-runner        # (workspace lint set)
    Finished `dev` profile — 0 warnings attributed to 🏃️test-runner\📦️packages\🦀️rust\🦀️.rs
    (the 5 remaining warnings belong to 📐️model and 🏠️workspace, other agents' files)

$ cargo test -p semio-framework-repo-test-runner
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ gofmt -l .                                              # (📦️packages/🐹️go)  → no output
$ GOWORK=…/go.work go vet ./... && go build ./...          → clean

$ bun …/🧪️test/📜️script.ts discover | grep test-runner
test-…-testrunner-9ce079-📊️result-parsing                 …/🧪️tests/📊️result-parsing   [rust,typescript]
test-…-testrunner-9ce079-🗺️invocation-planning            …/🧪️tests/🗺️invocation-planning [rust]
test-…-testrunner-9ce079-🧬️scope-identifier-normalisation …/🧪️tests/🧬️scope-…           [rust,go]
test-…-testrunner-9ce079-🧭️runner-detection               …/🧪️tests/🧭️runner-detection  [rust]
test-…-testrunner-9ce079-🛑️cancellation                   …/🧪️tests/🛑️cancellation      [rust]

$ bun …/🧪️test/📜️script.ts parity quick --case <each>
[test] level=quick cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0   # 🧭️runner-detection
[test] level=quick cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0   # 🗺️invocation-planning
[test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3   # 📊️result-parsing
[test] level=quick cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0   # 🛑️cancellation
[test] level=quick cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2   # 🧬️scope-identifier-…

$ bun …/🧪️test/📜️script.ts parity quick --owner …/🔨️modules/🏃️test-runner   # the launch entry
[test] level=quick cases=5 executed=19 passed=19 failed=0 errored=0 parity=5/5

$ node node_modules/nx/bin/nx.js run @semio-tech/repo-test-runner-go:test
?   github.com/usalu/semio/repo/testrunner  [no test files]
 NX   Successfully ran target test for project @semio-tech/repo-test-runner-go

$ bun …/🔌️plugin/📇️registry/📜️script.ts generate
.vscode/launch.json regenerated  → 3 test-runner entries at lines 8014 / 8021 / 8028
```

`parity=3/3` on `📊️result-parsing` is the substantive result: the Rust subject and the independently
written TypeScript reference agree, scenario for scenario, over all nine recorded transcripts covering
all six dialects — including durations, failure messages and per-test suites. `parity=2/2` on
`🧬️scope-identifier-normalisation` is Rust × the committed Go client.

## 6. Pre-existing conditions found (NOT introduced here, not fixed here)

1. **`case-slug` contract breach on every emoji-prefixed test case.**
   `🔣️taxonomy.json`'s `testCaseSlugPattern` is `^[a-z0-9]+(?:-[a-z0-9]+)*$`, which no emoji-prefixed
   directory can match — including the framework's own reference case. Proven:
   `contract --case 🖥️host-protocol-parity` reports
   `case-slug 🧰️framework/…/🧪️test/🧪️tests/🖥️host-protocol-parity`. The `📡️events` module's four cases
   breach identically. Either the pattern must admit a leading emoji or every case directory in the
   repository must be renamed; that is a `📚️library` / coordinator decision, not a per-module one. My
   five case names are the ones the task assigned.
2. **`oracle-in-production` breach for `serde_json`.** 316 such breaches exist repo-wide; the sibling
   repo crates `🧾️yaml`, `🧩️providers` and `🔗️graphql` carry the identical
   `serde-json-equation-carrier-reader` breach. plan.md §3 mandates `serde`/`serde_json` for every
   crate, so this is a registry-vs-plan conflict owned by whoever registered that oracle.
3. **`nx run @semio-tech/repo-test-runner-rs:test` fails before reaching cargo**, with
   `Invalid taxonomy schema: generatorContracts["wgpu-frame-worker"] tracked output …🤖️generated/🟨️.js
   is missing`. `@semio-tech/repo-model-rs:test` fails with the same message, so this is a repo-wide
   blocker in `loadTaxonomy`, unrelated to this module. `cargo test -p …` and the Go nx target both
   work; the launch entry will start working the moment that generated file is restored.

## 7. What is left

- `go-split` moves the 34 `🏃️Pending` symbols into `📦️packages/🐹️go`; then repoint
  `🧬️scope-identifier-normalisation/🐹️.go` (marker in place) and add Go adapters to the other four
  cases, upgrading their conformance scenarios to differential.
- `🪪️identity` exports `id_to_uri` / `DetectEntityKindFromId` → delete `🪪️IdentityPending`'s default
  and take the real implementation; `🗣️languages` exports its plugin table → delete
  `language_of_extension`.
- `🧩️providers` lands → delete the `⚙️Execution` port declarations in both languages and import them.
- `🪝️hooks` lands → de-duplicate `split_command_segments` / `classify_command_kind`.
- `⌨️cli` wires `plan_scopes` + `execute_plan` behind the `semio test` verb, rendering `ProgressEvent`
  through the existing renderers and binding cancellation to the CLI's interrupt.
- No real-process `ProcessRunner` is shipped here on purpose: spawning belongs to `🧩️providers`.
