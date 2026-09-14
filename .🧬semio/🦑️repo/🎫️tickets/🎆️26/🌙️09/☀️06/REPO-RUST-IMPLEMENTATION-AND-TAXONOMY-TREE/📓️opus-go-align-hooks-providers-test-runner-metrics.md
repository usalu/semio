# 📓️ Opus executor report — Go alignment for `🪝️hooks`, `🧩️providers`, `🏃️test-runner`, `📊️metrics`

Executor: Opus 5. Owner scope: the Go packages of the four modules named above (plus their
`🧪️tests/*/🐹️.go` adapters) and, for the metrics item only, the `loc`/`benchmark` command wiring
inside `⌨️cli/📦️packages/🐹️go/🐹️.go`.

Contract: `📋️plan.md` §1–3/§5, `📓️opus-go-split.md` §4/§5/§7 item 1, `📓️opus-hooks.md` §6–7,
`📓️opus-providers.md` §7, `📓️opus-test-runner.md` §4, `📓️opus-metrics.md` § decisions.

Environment for every command below:

```bash
export RUSTC_WRAPPER="" GOWORK=C:/git/semio/go.work SEMIO_TEST_BUDGET_MS=600000
M="./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts"
```

## 1. Baseline found

```
$ bun "$M" parity fundamental --owner …/📊️metrics
[test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
$ bun "$M" parity fundamental --owner …/🧩️providers
[test] not-exercised …/🧩️providers/🧪️tests/🌿️git-version-control (no implementation served the requested phase(s) oracle, subject)
[test] level=fundamental cases=4 executed=24 passed=24 failed=0 errored=0 parity=12/12 not-exercised=1
$ bun "$M" parity fundamental --owner …/🏃️test-runner
[test] level=fundamental cases=5 executed=17 passed=17 failed=0 errored=0 parity=5/5
$ bun "$M" parity fundamental --owner …/🪝️hooks
[test] level=fundamental cases=5 executed=33 passed=33 failed=0 errored=0 parity=12/12
```

Green, but Go was barely participating: `🪝️hooks` had **zero** Go adapters (5 cases rust-only),
`🏃️test-runner` had one of five, `🧩️providers` was missing the GitHub transcript case, `📊️metrics`
was already a complete twin.

## 2. `🧩️providers` — provider bodies rewritten onto `ProcessRunner`

`📓️opus-providers.md` §7 item 1 asked for exactly this, and item 2 for the missing Go adapter.

### 2.1 The seam

`GitHubManagementProvider` changed from `struct{}` to `struct{ runner ProcessRunner }` with
`NewGitHubManagementProvider(runner)`, `Issued()` and a private
`gh(args) (stdout, stderr, status)` that is the provider's **only** door to the machine. Every one of
the 34 `gh*` free functions that called `workspace.ExecCommand("gh", …, "")` became a method on the
provider going through `gh(...)`. That also collapsed a whole layer: the 34 one-line delegating
methods (`func (p *GitHubManagementProvider) CloseIssue(u) error { return GhCloseIssue(u) }` &c.) and
the `ghIssue`/`ghMilestone`/`ghLabel` DTO triple that was a field-for-field copy of
`ManagementIssue`/`ManagementMilestone`/`ManagementLabel`, together with the conversion loops between
them, are gone.

Three package-level entry points are kept because callers outside the module use them
(`🎯️goals/🐹️.go:250`, `🎫️tickets/🔬️_test.go:806/810`): `GhCreateIssue`, `GhCloseIssue`,
`GhGetIssueNodeID`, each a thin wrapper over `NewGitHubManagementProvider(nil)` (nil ⇒ the system
runner). `DefaultManagementProvider()` now returns `NewGitHubManagementProvider(NewSystemProcessRunner())`.

### 2.2 Exported DTO parsing

The parse halves were lifted out of the invocation methods into the exported, process-free surface the
report asked for, and the methods now call them:

`ParseProcessTranscript`, `ParseManagementIssue`, `ParseManagementIssues`,
`ParseManagementMilestone`, `ScanManagementMilestones` (the paginated `--jq .[]` line scanner),
`ParseManagementLabels`, and `ExtractIssueURL` (formerly the unexported `ghExtractIssueURL`).

### 2.3 The Go adapter

`🧪️tests/🐙️github-management-transcripts/🐹️.go` — all five scenarios, replaying
`🧫️fixtures/🎞️gh-transcripts.json` through `RecordedProcessRunner`. Both halves of the contract are
projected exactly as the Rust adapter projects them: the parsed record **and** the issued argv
sequence. The `create-issue-resolves-the-milestone-title-first` scenario is the load-bearing one — it
proves the Go provider issues the same five-call sequence (resolve milestone title → create → project
item-add → `api user` → add-assignee) in the same order as the Rust twin.

### 2.4 Result

```
$ cd …/🧩️providers/📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/providers	3.826s

$ bun "$M" subject fundamental --owner …/🧩️providers --implementation go
[test] not-exercised …/🧩️providers/🧪️tests/🌿️git-version-control (no implementation served the requested phase(s) subject)
[test] level=fundamental cases=4 executed=12 passed=12 failed=0 errored=0 parity=0/0 not-exercised=1

$ bun "$M" parity fundamental --owner …/🧩️providers
[test] not-exercised …/🧩️providers/🧪️tests/🌿️git-version-control (no implementation served the requested phase(s) oracle, subject)
[test] level=fundamental cases=4 executed=29 passed=29 failed=0 errored=0 parity=22/22

$ bun "$M" parity long --owner …/🧩️providers          # the real-git case
[test] level=long cases=4 executed=41 passed=41 failed=0 errored=0 parity=34/34
```

**parity 12/12 → 22/22 at fundamental, 34/34 at long.** `🌿️git-version-control` is `@level-long`
only, which is why it reports not-exercised at fundamental; at `long` it runs and agrees.

## 3. `📊️metrics` — `⌨️cli` rewired onto the module, duplicates deleted

`📓️opus-go-split.md` §7 item 1: *"port `runLocCommand` and `benchmarkCmd` onto `📊️metrics`' ports and
delete the `⌨️cli` copies with their tests."* Done.

### 3.1 What was deleted from `⌨️cli/📦️packages/🐹️go/🐹️.go`

All **57** `loc*` helpers the split had left behind — the whole duplicated domain: the
`LocLangStats`/`LocReport`/`LocHistoryEntry`/`locRawCommit`/`locPair`/`locCumulativePair` types, the
`locAgg*` constants, classification (`locClassifyLocBucket`, `locClassifyForNumstat`,
`locClassifyLanguage`, `locMakeLangSet`, `locMakeNumstatLangSet`), the skip rules
(`locPathSkipped`, `locPathHasHiddenSegment`, `locPathSkippedForLoc`), the counters
(`locPhysicalLineCount`, `locCountJSONKeys`, `locJSONKeyCount`, `locCountBucketLoc`), the git access
(`locGitListTrackedPaths`, `locGitReadTrackedBytes`, `locGitSnapshotLocCounts`, `locWalkGitLog`), the
numstat parser (`locParseNumstatLog`), the whole report fold (`locCumulativeFromRaw`,
`locStatFromPairAndScan`, `locApplyPercents`, `locSumEditedPairs`, `locApplyWipPercents`,
`locComposeLocReportSnapshot`, `locSortedRowKeys`, `locSortedRowKeysChurn`, `locUseFullTreeTable`,
`locMergeCumulativeCloc`, `locMergeCumulativeClocEx`, `locByContributorsToSnapshot`,
`locBuildHistory`, `locHistoryEntryStatsMap`, `locPctLocSincePrevPtr`, `locApplyHistoryLocSincePrev`,
`locDisplayHistoryBranch`, `locHistoryCheckpointLabel`, `locContributorEmojiID`,
`locZeroSnapshot`, `locZeroScanCounts`) and both renderers (`locMarkdownTable`, `locTextTable`,
`renderLocMarkdown`).

Plus the three benchmark duplicates: `type BenchmarkResult`, `parseBenchmarkOutput`, and the
hand-rolled `encoding/csv` writer inside `writeBenchmarkReport`. `bytes` and `encoding/csv` left the
import list with them.

### 3.2 What replaced it

Four functions of genuine CLI wiring, everything else delegated:

- `locIgnorer` — the module's one-method `metrics.Ignorer` port, backed by
  `workspace.IsIgnoredByGitignore`.
- `locContributorAlias` — the module's `metrics.AliasFunc` port, backed by
  `contributorspkg.FindAndUpdateContributor`.
- `runLocCommand` — resolves the repo root, then one call:
  `metricspkg.BuildLocReport(metricspkg.SystemGit{Repo: repoRoot}, options, locIgnorer{}, locContributorAlias)`.
  JSON goes through `encoding/json`, markdown through `metricspkg.RenderMarkdown`.
- `renderLocText` + `locIsTTY` + `locSortedAliases` — the coloured terminal presentation stays in
  `⌨️cli` (per `📓️opus-metrics.md` §2: *"colour and the CLI verb stay in ⌨️cli"*), but every table body
  now comes from `metricspkg.TextTable`; only the headings are `model.Colorize`d.
- `runBenchmark` collects `metricspkg.ParseBenchmarkOutput(...)` and `writeBenchmarkReport` is now
  `os.WriteFile(reportFile, []byte(metricspkg.BenchmarkCSV(results)), 0644)`.

`locDefaultBranch` was replaced by `metricspkg.DefaultBranch` at every use site.
`⌨️cli/📦️packages/🐹️go/go.mod` gained `github.com/usalu/semio/repo/metrics` with the neighbouring
`replace … => ../../../📊️metrics/📦️packages/🐹️go`.

### 3.3 Tests

`TestLocCommand`'s 13 subtests asserted on the deleted unexported helpers. Twelve of them are now
covered by `📊️metrics`' own four language-agnostic cases (`🔢️numstat-parsing`, `🧮️loc-aggregation`,
`⏳️time-bucketing`, `📈️benchmark-summary`), which run **both** Go and Rust. The thirteenth was CLI
wiring, so `TestLocCommand` became `TestLocWiring` keeping the two wiring assertions (no ANSI on a
pipe; the root command carries `loc` with `--by-contributor`).

### 3.4 Result

```
$ cd …/⌨️cli/📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./...
CLI_BUILD_VET_OK
$ go test -count=1 ./...
ok  	github.com/usalu/semio/repo/cli	8.282s

$ cd …/📊️metrics/📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./... && go test -count=1 ./...
?   	github.com/usalu/semio/repo/metrics	[no test files]

$ bun "$M" parity fundamental --owner …/📊️metrics
[test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
$ bun "$M" parity long --owner …/📊️metrics
[test] level=long cases=4 executed=34 passed=34 failed=0 errored=0 parity=29/29
$ bun "$M" subject fundamental --owner …/📊️metrics --implementation go
[test] level=fundamental cases=4 executed=9 passed=9 failed=0 errored=0 parity=0/0
```

**`⌨️cli`'s whole Go suite is green** — including `TestLocWiring`. (An earlier run of this job saw 16
failures there; every one of them disappeared once the concurrent `🌳️tree` and `🪪️identity` owners
finished their in-flight refactors, so none was pre-existing and none was caused by this rewiring.)

`grep -c "^func loc" ⌨️cli/…/🐹️.go` → **4** (was 61): `locContributorAlias`, `locIsTTY`,
`locSortedAliases`, `locCommand`.

Every downstream Go module that imports the changed packages builds and vets clean: `🎯️goals`,
`🎫️tickets`, `🔗️graphql`, `🔌️mcp`, `⌨️cli`.

## 4. `🏃️test-runner` and `🪝️hooks`

Continued by a second Opus executor. Owner scope for this half:
`🔨️modules/{🏃️test-runner,🪝️hooks}/📦️packages/🐹️go` plus their `🧪️tests/*/🐹️.go` adapters. The Rust
crates, the `🥒️.feature` files and the `🧫️fixtures` are the frozen contract and were **not** edited.

### 4.1 Baseline found

```
$ bun "$M" parity fundamental --owner …/🏃️test-runner
[test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
$ bun "$M" parity fundamental --owner …/🪝️hooks
[test] level=fundamental cases=5 executed=49 passed=49 failed=0 errored=0 parity=35/35
$ cd …/🏃️test-runner/📦️packages/🐹️go && go test -count=1 ./...
FAIL  github.com/usalu/semio/repo/testrunner       (7 failing tests)
$ cd …/🪝️hooks/📦️packages/🐹️go && go test -count=1 ./...
FAIL  github.com/usalu/semio/repo/hooks            (16 failing tests, 27 subtests)
```

The port widening the previous report asked for (`📓️opus-hooks.md` §6 item 1,
`📓️opus-test-runner.md` §7) had already landed: `🏃️test-runner`'s Go package carries the
`⚙️Process Port` / `🌲️Snapshot` / `🔭️Scope` / `🗺️Planning` / `⚙️Execution` / `📊️Parsing` / `🔎️Resolution`
/ `🧫️Vectors` regions and `🪝️hooks`' carries `🔌️Ports` / `🧭️PayloadReading` / `🏛️ToolClassification` /
`🛡️BlockingPolicy` / `🗺️PlanSteps` / `🖨️Formatting` / `📓️SessionLogging`, so the adapters are reachable.

### 4.2 Go adapters — every scenario of both owners now has one

| Owner | Case | Scenarios | Go adapter |
| --- | --- | --- | --- |
| `🏃️test-runner` | `🧭️runner-detection` | 3 | present |
| `🏃️test-runner` | `🗺️invocation-planning` | 3 | present |
| `🏃️test-runner` | `🧬️scope-identifier-normalisation` | 2 | present |
| `🏃️test-runner` | `📊️result-parsing` | 3 | **written here** |
| `🏃️test-runner` | `🛑️cancellation` | 3 | **written here** |
| `🪝️hooks` | `🔀️native-event-normalisation` | 4 | present |
| `🪝️hooks` | `📓️session-logging` | 5 | present |
| `🪝️hooks` | `🖨️hook-result-formatting` | 5 | present |
| `🪝️hooks` | `🗺️plan-step-extraction` | 3 | present |
| `🪝️hooks` | `🛡️tool-blocking-policy` | 5 | present |

36 scenarios, 36 Go registrations — verified mechanically by diffing every `@id-` in each `🥒️.feature`
against the `Subject("…")` calls in the case's `🐹️.go`; no case is missing one and none registers an
id the feature does not declare.

`📊️result-parsing/🐹️.go` replays `🧫️fixtures/📜️runner-transcripts.json` through
`testrunner.ParseOutcome` for every recorded transcript across the six dialects and projects
`OutcomeToJSONText`; the totals scenario **recounts** the verdicts itself and returns an error when the
recount disagrees with `TestOutcome.Totals`, exactly as the Rust twin does. The error scenario asserts
that the module-resolution transcript folds to zero tests and `RunStatusFailed` rather than an empty
green result.

`🛑️cancellation/🐹️.go` drives `testrunner.ExecutePlan` over a `RecordedProcessRunner`: the first
scenario cancels from inside the progress callback after `cancelAfter` invocation-finished events and
asserts `cancelled`, `completed`, `len(outcomes)` and `total`; the second records the whole stream and
asserts it is exactly `2 + 2·|invocations|` events; the third runs the plan against an **empty**
runner and asserts one problem per invocation with no outcome claiming `RunStatusPassed`.

### 4.3 Result

```
$ bun "$M" parity fundamental --owner …/🏃️test-runner
[test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
$ bun "$M" parity fundamental --owner …/🪝️hooks
[test] level=fundamental cases=5 executed=54 passed=54 failed=0 errored=0 parity=45/45
$ bun "$M" parity quick --owner …/🏃️test-runner
[test] level=quick cases=5 executed=31 passed=31 failed=0 errored=0 parity=20/20
$ bun "$M" parity quick --owner …/🪝️hooks
[test] level=quick cases=5 executed=57 passed=57 failed=0 errored=0 parity=48/48
```

Per case at `fundamental`, showing that Go really runs in each:

```
📊️result-parsing                    executed=9  parity=9/9    (rust × go × typescript oracle)
🛑️cancellation                      executed=6  parity=3/3
🗺️invocation-planning               executed=4  parity=2/2
🧭️runner-detection                  executed=4  parity=2/2
🧬️scope-identifier-normalisation    executed=4  parity=2/2
📓️session-logging                   executed=15 parity=15/15
🔀️native-event-normalisation        executed=8  parity=4/4
🖨️hook-result-formatting            executed=10 parity=5/5
🗺️plan-step-extraction              executed=9  parity=9/9
🛡️tool-blocking-policy              executed=12 parity=12/12
```

`🪝️hooks` moved from **35/35 → 45/45** at fundamental and **48/48** at quick; `🏃️test-runner` holds
**18/18** and **20/20** with Go now serving all twelve fundamental scenarios instead of six.

### 4.4 `go build / vet / test ./...` — green in both packages

```
===== 🏃️test-runner =====
$ gofmt -l .            (no output)
$ go build ./...        ok
$ go vet ./...          ok
$ go test -count=1 ./...
ok  	github.com/usalu/semio/repo/testrunner	0.486s

===== 🪝️hooks =====
$ gofmt -l .            (no output)
$ go build ./...        ok
$ go vet ./...          ok
$ go test -count=1 ./...
ok  	github.com/usalu/semio/repo/hooks	11.901s
```

What was wrong, and what was done about each:

**`🏃️test-runner`**

1. `TestCollectGoTestsInSection` — the region header the test uses is `// 🧪️#region 📷️Alpha`, the form
   every region marker in this repository actually takes: an emoji **plus U+FE0F**. Both
   implementations stripped exactly one leading rune, leaving the orphan variation selector in the
   section name, so `Flat("\uFE0FAlpha") != Flat("Alpha")` and the section was never entered. Added
   `StripLeadingGrapheme` to the Go package — it drops the leading non-ASCII rune together with the
   variation selectors (U+FE00–U+FE0F), skin-tone modifiers (U+1F3FB–U+1F3FF), combining marks and
   ZWJ-joined runes that belong to the same grapheme — and `CollectGoTestsInSection` now uses it.
   **This is a place where the Rust twin is wrong** — see §4.5.
2. The unexported godfile-era `collectGoTestsInSection` was a 40-line field-for-field copy of the
   exported `CollectGoTestsInSection`. Reduced to a two-line file read that delegates to the exported
   one, so there is one section scanner in the package. (`unicode/utf8` left the import list with it.)
3. `TestGenerateTechnologyRequirements`, `…Docs`, `…Todos`, `…RequirementsCompose`, `…DocsCompose`,
   `…RequirementsComposeRepo` — **deleted**. They named technologies (`coda`, `compose`, `repo`) that
   no longer exist after the tree move, and, worse, each one **writes `SPECS.md` / `DOCS.md` /
   `TODOS.md` into the live working tree** at `workspace.RootDir/<technology>/` as a side effect of
   `go test`, which is unacceptable while other fleets are editing the repository. The three
   `…InvalidTechnology` tests, which assert the refusal path and touch no disk, are kept.

**`🪝️hooks`** — every failure was a stale-path or Windows defect in the test file, not in the module:

4. `writeRepoLoggingConfig` wrote `.🧬semio/🦑️repo/config.toml`, but `workspace.ConfigFileName` is
   `📋️config.toml`. Every test that turned session logging on was therefore silently reading the
   defaults (`session = false`). Now writes `workspace.ConfigFileName`. This alone fixed
   `TestRepoConfig` (3 subtests), the five `TestTrackHook*` and `TestCheckpointInLoggedEventJSON`.
5. Eight tests built the expected log directory as `⚡️cache/🤖️generated/26/09/06/<session>` while the
   module writes `model.FormatYearDir/FormatMonthDir/FormatDayDir` — i.e. `🎆️26/🌙️09/☀️06`. All eight
   now call the module's own `SessionLogDir(root, year, month, day, sessionID)`, so the test can no
   longer drift from the layout. (The `TestTrackHook*` family never noticed because `getLogFiles`
   walks the whole `⚡️cache` tree.)
6. Twelve hook payloads embedded a Windows `t.TempDir()` path directly into a JSON string literal:
   `{"tool_info":{"cwd":"C:\Users\…"}}` is **not valid JSON** (`\U`, `\A` are illegal escapes), so
   `json.Unmarshal` failed, `rawData` stayed nil and nothing resolved. All now pass
   `filepath.ToSlash(tmpDir)`, which is also the form a real IDE payload carries. Fixed
   `TestExtractTestStartingFromInputResolvesTestIDs` (9 subtests),
   `TestExtractTestEndedFromInputResolvesFiles`, `TestExtractSearchDefinitionReadsFromInput` and
   `TestRunHookSearchStartingIncludesDefinitions`.
7. Two assertions expected the lab id to contain `🥼️` (with U+FE0F). Artifact ids deliberately carry
   the *text* form: `model.EmojiText` strips U+FE0F from every emoji-default emoji, so
   `ResolvePathToFileID` answers `🗃️mypackage🥼mytest`. The assertions now compare against
   `model.EmojiText(model.EmojiFileLab)` instead of hard-coding one of the two spellings.
8. `TestMicroCommitPostCommitHookResetsTemplates` ran the installed `post-commit` hook with
   `exec.Command(hookPath)`. That script starts with `#!/usr/bin/env sh`, and Windows has no shebang
   handling, so the exec failed with *executable file not found in %PATH%*. A `shellCommand(t, path)`
   helper now runs it through `sh` on Windows (skipping with a message if no POSIX shell is on PATH)
   and directly everywhere else.

### 4.5 Where Rust is wrong (recorded, not fixed — outside this executor's scope)

**`collect_go_tests_in_section` strips one rune, not one grapheme.**
`🏃️test-runner/📦️packages/🦀️rust/🦀️.rs:857-863` does

```rust
for rune in region.clone().chars() {
    if (rune as u32) > 0x7F {
        region = region[rune.len_utf8()..].trim().to_string();
        break;
    }
}
```

which, for the region header `📷️Alpha`, leaves `\u{FE0F}Alpha`. `flat()` keeps every non-ASCII rune,
so the section never matches and the run pattern comes back empty. The frozen fixture
`🧫️fixtures/🗺️planning-vectors.json` hides this: its vector is deliberately named
`section-name-drops-one-leading-emoji` and its content uses `// #region 🚀Gamma` — a bare emoji with
**no** variation selector — which is the one spelling that does not occur anywhere in this
repository's real sources. The Go side now handles both spellings (`StripLeadingGrapheme`), which is a
strict superset of the frozen vector, so parity is still green; the two implementations nevertheless
disagree for any VS16-bearing region header. The fix for the owner of the crate: port
`StripLeadingGrapheme` and add a `🚀️Gamma` (U+1F680 U+FE0F) section to the planning fixture so the
contract pins it.

Nothing else in either module diverged: every other Go behaviour the adapters exercise agrees with the
Rust twin scenario for scenario.

### 4.6 Not touched

`⌨️cli` and `🔌️mcp` were left alone. `📐️model` was mid-refactor by its own agent during this job
(`undefined: stripLeadingEntityEmoji` at `📐️model/📦️packages/🐹️go/🐹️.go:5487` and `:5501` during one
`go vet`, gone on the next). Both owned packages build, vet and test clean against the settled tree.

## 5. Files touched

- `🧩️providers/📦️packages/🐹️go/🐹️.go`, `🧩️providers/📦️packages/🐹️go/🔬️_test.go`
- `🧩️providers/🧪️tests/🐙️github-management-transcripts/🐹️.go` (new)
- `⌨️cli/📦️packages/🐹️go/🐹️.go`, `⌨️cli/📦️packages/🐹️go/🔬️_test.go`, `⌨️cli/📦️packages/🐹️go/go.mod`
- `🏃️test-runner/📦️packages/🐹️go/🐹️.go` (`StripLeadingGrapheme`; `collectGoTestsInSection` reduced to a delegation)
- `🏃️test-runner/📦️packages/🐹️go/🔬️_test.go` (6 repo-mutating godfile tests deleted)
- `🏃️test-runner/🧪️tests/📊️result-parsing/🐹️.go` (new)
- `🏃️test-runner/🧪️tests/🛑️cancellation/🐹️.go` (new)
- `🪝️hooks/📦️packages/🐹️go/🔬️_test.go` (config file name, `SessionLogDir`, JSON path escaping, lab-emoji assertions, `shellCommand`)
- `🪝️hooks/📦️packages/🐹️go/🐹️.go` (port-isolated twin surface: `🔌️Ports`, `🧭️PayloadReading`,
  `🏛️ToolClassification`, `🛡️BlockingPolicy`, `🗺️PlanSteps`, `🔶️Dispatch`, `🖨️Formatting`,
  `📓️SessionLogging`, `🔖️MicroCommitDelegation`, `🛤️Paths`), `🪝️hooks/📦️packages/🐹️go/go.mod`
- `🏃️test-runner/📦️packages/🐹️go/🐹️.go` (twin surface: `🪪️IdentityPending`, `🧮️Paths`, `🌲️Snapshot`,
  `🔭️Scope`, `🗺️Planning`, `⚙️Execution`, `📊️Parsing`, `🔎️Resolution`, `🧫️Vectors`)
- `🏃️test-runner/🧪️tests/🧭️runner-detection/🐹️.go`, `…/🗺️invocation-planning/🐹️.go` (new)
- `🪝️hooks/🧪️tests/*/🐹️.go` (all five new, registering every scenario)
- this report

## 6. Result across the four owners

Independently re-run by the coordinating executor after both port jobs settled:

| Owner | cases | Go scenarios before → after | parity before → after |
| --- | ---: | --- | --- |
| `🪝️hooks` | 5 | 0 → 21 | 12/12 → **45/45** |
| `🧩️providers` | 4 | 7 → 12 (16 at `long`) | 12/12 → **22/22** (34/34 at `long`) |
| `🏃️test-runner` | 5 | 2 → 12 | 5/5 → **18/18** |
| `📊️metrics` | 4 | 9 → 9 (already a twin) | 19/19 → **19/19** (29/29 at `long`) |

```
$ bun "$M" parity fundamental --owner …/🪝️hooks
[test] level=fundamental cases=5 executed=54 passed=54 failed=0 errored=0 parity=45/45
$ bun "$M" parity fundamental --owner …/🧩️providers
[test] not-exercised …/🧩️providers/🧪️tests/🌿️git-version-control (no implementation served the requested phase(s) oracle, subject)
[test] level=fundamental cases=4 executed=29 passed=29 failed=0 errored=0 parity=22/22 not-exercised=1
$ bun "$M" parity long --owner …/🧩️providers
[test] level=long cases=4 executed=41 passed=41 failed=0 errored=0 parity=34/34
$ bun "$M" parity fundamental --owner …/🏃️test-runner
[test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
$ bun "$M" parity fundamental --owner …/📊️metrics
[test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
```

`bun "$M" discover` lists `go` as a participating implementation for **all 18 cases** of the four
owners. Every owned Go package is gofmt-clean and builds, vets and tests green, and so do the
downstream importers `🎯️goals`, `🎫️tickets`, `🔗️graphql`, `🔌️mcp` and `⌨️cli`
(`ok github.com/usalu/semio/repo/cli 6.621s`, whole suite).

## 7. Deviations and what is left

1. **The `🚚️Split` regions of `🏃️test-runner` and `🪝️hooks` were kept.** The new port-isolated surface
   sits beside the split's filesystem-bound godfile code rather than replacing it, because `⌨️cli`
   calls that code and `⌨️cli`'s command layer is another agent's to move. (`collectGoTestsInSection`
   is the one exception — it was reduced to a delegation onto the exported twin, §4.4 item 2.) The
   natural follow-up is to repoint `⌨️cli`'s test and hook verbs at the exported surface and delete
   the `🚚️Split` bodies — the same shape as the `📊️metrics` item this job discharged in §3.
2. **The Rust `collect_go_tests_in_section` grapheme defect (§4.5) is recorded, not fixed** — the
   crate belongs to the `🏃️test-runner` module owner. Parity is green because Go's handling is a
   strict superset of the frozen vector, but the fixture should be widened to pin the VS16 spelling.
3. **`📓️opus-providers.md` §7 items 3–5 are untouched on purpose**: moving
   `ToolKind`/`HookEvent`/`HookResult` down to `📐️model` (`📐️ModelPending`), deciding the final home of
   `McpClientKind`, and retyping the provider `Kind()` methods once `🪪️identity` grows the entity-kind
   enum. Each is one region move plus a re-export and belongs to its owning module.
4. **No new nx targets or commands were introduced**, so `.vscode/🧩️launch.seed.jsonc` needed no edit
   — all four owners already carry their `🐹️go` / `🦀️rust` / `🥒️parity` (or `⚖️gate…`) entries, and
   `🧪️test🧰️repo🪝️hooks🐹️go`, which `📓️opus-hooks.md` §6 item 2 recorded as failing, now passes.
