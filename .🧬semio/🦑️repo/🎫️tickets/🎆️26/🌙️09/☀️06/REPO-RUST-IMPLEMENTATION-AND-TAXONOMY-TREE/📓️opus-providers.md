# 📓️ Opus executor report — `🔨️modules/🧩️providers`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/` (net new).
Source of truth: the frozen snapshot `$TICKET/🗑️generated/go-snapshot/client/{🧩️component.go, 🔬️component_test.go}`.
Binding conventions: `📋️plan.md` §2/§3; fixes 5 and 7 of `📓️go-region-dependency-graph.md` §8; case shape from `📓️harness-verification.md`.

## 1. What exists now

```
🔨️modules/🧩️providers/
├── 🧬️schema/🔣️.json                                  JSON Schema (draft 2020-12) for the module's wire shapes
├── 🔮️oracle/🔣️.json                                  owner contribution manifest (2 oracles, 3 no-oracle decisions)
├── 🧫️fixtures/                                       (empty — every fixture in these four cases is case-local)
├── 🧪️tests/
│   ├── 🐙️github-management-transcripts/{🥒️.feature, 🦀️.rs, 🟦️.ts, 🧫️fixtures/🎞️gh-transcripts.json}
│   ├── 🌿️git-version-control/{🥒️.feature, 🦀️.rs, 🐹️.go, 🟦️.ts}
│   ├── 🪝️editor-hook-output-format/{🥒️.feature, 🦀️.rs, 🐹️.go, 🧫️fixtures/🪝️hook-outputs.json}
│   └── 🪪️mcp-client-kind-parse/{🥒️.feature, 🦀️.rs, 🐹️.go, 🧫️fixtures/🪪️client-kinds.json}
└── 📦️packages/
    ├── 🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}     semio-framework-repo-providers
    └── 🐹️go/{go.mod, 🐹️.go, 🧪️_test.go, 📋️project.json, 📜️script.ts} github.com/usalu/semio/repo/providers
```

Registered in: root `Cargo.toml` `[workspace].members`, root `go.work` `use (…)`,
`.vscode/🧩️launch.seed.jsonc` (six `4_gate` entries, orders 425.3–425.8), and `.vscode/launch.json`
via `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`.

## 2. Public Rust API — `semio-framework-repo-providers`

Crate root carries `#![allow(async_fn_in_trait)]` (the per-crate obligation of every `#[dyn_enum]`
declaring crate) and depends only on `serde`, `serde_json` and the path crate `dispatch_macros`
(`semio-framework-dispatch-macros`).

### `🔖️ProviderError`
`ProviderError { message: String }` (`new`, `Display`, `std::error::Error`), `type ProviderResult<T> = Result<T, ProviderError>`.

### `📐️ModelPending` — shapes that belong in `📐️model`
`ToolKind` (`Generic|Plan|CodeSearch|CodeEdit|Test|Build|Terminal`, `as_str`, `parse`),
`HookEvent` (26 variants, `as_str`, `parse`), `ALL_HOOK_EVENTS: [HookEvent; 26]`,
`HookResult { allowed, message, extra: Map<String, Value> }` (`new`, `with`, `payload`).

**Decision, to be undone by whoever grows the model crate.** At the time this module was written the
model crate existed (`🔨️modules/📐️model/📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs}`, crate
`semio-framework-repo-model`) but exported **no** `Kind`, `HookEvent`, `HookResult` or `ToolKind` —
its 2784 lines carry the slug vocabularies, the enums (`DefinitionKind`, `TicketStatus`, `BundleKind`,
…) and the node/ticket/goal/contributor structs, and nothing hook-shaped. Depending on it would
therefore have bought nothing and coupled this crate to a manifest another agent is still editing. So
the three shapes live in the `📐️ModelPending` region here, they are re-exported from the crate root,
and the move is a mechanical one: delete the region, add
`semio-framework-repo-model = { path = "../../../📐️model/📦️packages/🦀️rust" }`, `pub use`. The Go
twin records the same decision in its `🧩️Pending` region.
`Kind` (the entity-kind enum, fix 1 of the graph report) is **not** modelled here at all — the Go
providers only ever call `Kind() string`, so the Rust traits return `&'static str` and nothing in this
module needs the enum. When `Kind` lands in `🪪️identity`, `ManagementProvider::kind` &c. can be
retyped without touching any provider body.

### `🪪️McpClientKind` — fix 7
`McpClientKind` (`Generic|Cursor|Kiro|Copilot|Claude|Codex`, `as_str`),
`parse_mcp_client_kind(&str) -> ProviderResult<McpClientKind>`,
`hook_client_for_mcp_kind`, `mcp_kind_from_resolved_client`, `mcp_server_name`.

**Coordination note.** `🔨️modules/🪪️identity` was checked before this was written: neither
`📦️packages/🦀️rust/🦀️.rs` (Id, seeded entropy, humanize, entity emojis, semantic ids, artifact refs)
nor `📦️packages/🐹️go/🐹️.go` mentions `McpClientKind`, `EditorKind` or any IDE identity, and identity's
`🧬️schema/🔣️entity-emojis.json` is about entity kinds, not clients. So it lives here, next to the eight
editor providers that already enumerate the same IDE set — which is the second option fix 7 names. If
the identity executor later decides identity should own it, the move is one region plus a re-export;
until then `🎫️tickets` (15 edges) and `🪝️hooks` (14 edges) reach it downward at L1 instead of reaching
up into `🔌️mcp` at L5, which is what fix 7 asks for either way.

### `🏃️ProcessRunner` — the seam that makes every provider testable
```rust
pub struct ProcessRequest { pub argv: Vec<String>, pub cwd: Option<String>, pub stdin: Option<String> }
pub struct ProcessOutcome { pub stdout: String, pub stderr: String, pub status: i32 }
pub struct ProcessExchange { pub argv: Vec<String>, pub stdin: Option<String>, pub stdout: String, pub stderr: String, pub status: i32 }
pub struct ProcessTranscript { pub exchanges: Vec<ProcessExchange> }

#[dyn_enum]
pub trait ProcessRunner {
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome;
    fn issued(&self) -> Vec<Vec<String>>;
}
pub struct SystemProcessRunner;      // std::process, logs every argv
pub struct RecordedProcessRunner;    // ::new(transcript) / ::from_json(&str)
dyn_enum_close! { pub enum ProcessRunners: ProcessRunner { System(..), Recorded(..) } }
```
`RecordedProcessRunner` serves the **first not-yet-consumed** exchange whose argv matches exactly, so
a provider's optional calls may be omitted from a fixture without reordering it, an exchange is
replayed at most once, and anything unrecorded comes back as exit 127 with a naming stderr — an argv
the provider changed can never be mistaken for a parse that still works.

### `🔭️ProviderInterfaces` — four closed-set traits
All four use `#[dyn_enum]` + `dyn_enum_close!` exactly as `🎮️commands/🌊️workflow/🦀️.rs` does, so there
is no `Box<dyn Trait>` anywhere in the module.

| Trait | Methods | Closed enum |
| --- | --- | --- |
| `ManagementProvider` | 33, mirroring the Go interface one-for-one | `ManagementProviders { GitHub, Null }` |
| `VersionControlProvider` | `kind, repo_url, checkpoint, current_checkpoint, checkin, checkout, current_branch, staged_files, stage_all` | `VersionControlProviders { Git }` |
| `SandboxProvider` | `kind` | `SandboxProviders { Devcontainer }` |
| `EditorProvider` | `kind, resolve_native_event, format_hook_output, native_event_from_hook_event` | `EditorProviders { Copilot, Cursor, Windsurf, ClaudeCode, Droid, Codex, Antigravity, Kiro }` |

DTOs: `ManagementIssue`, `ManagementIssueMilestone`, `ManagementMilestone` (`due_on` keeps its REST
spelling), `ManagementLabel` — the Go `ghIssue`/`ghMilestone`/`ghLabel` structs, moved to the module
that parses them.

### `🔌️Ports` — fix 5
```rust
pub trait HookFormatter { fn format_hook_output(..) -> String; fn native_event_from_hook_event(..) -> String; }
impl HookFormatter for EditorProviders { … }

pub trait IssueTracker { create_issue, close_issue, reopen_issue, get_issue_details, add_comment,
                         add_labels, remove_labels, find_milestone_by_title, list_repo_labels }
impl IssueTracker for ManagementProviders { … }
```
Both ports are declared **here**, at L1, and implemented by the closed enums. `🪝️hooks` (L3) takes a
`HookFormatter` instead of `providers` importing `hooks.HookResult`/`HookEvent` (36 edges), and
`🎫️tickets` (L3) takes an `IssueTracker` instead of `providers` importing the `gh*` DTOs from tickets
(23 edges). The impls are on the enums rather than blanket `impl<T: EditorProvider>` on purpose: a
blanket impl collides with the inherent methods `dyn_enum_close!` generates (`E0034`, reproduced and
recorded here so nobody re-tries it).

### `🐙️GitHub`, `🌿️Git`, `⛑️Devcontainer`, `🎆️Editors`, `🎖️Registry`
`extract_issue_url`, `GitHubManagementProvider::{new, issued}`, `NullManagementProvider`;
`GitVersionControlProvider::{new, issued}`, `archive_branch_name`, `utc_year_month_day`
(civil-from-days, no time crate — the Go `time.Now().UTC()` branch-name format is reproduced exactly);
`DevcontainerSandboxProvider`;
`resolve_pre_tool_use`, `resolve_post_tool_use`, `resolve_shell_pre_tool_use`,
`resolve_shell_post_tool_use`, `resolve_copilot_event`, `resolve_cursor_event`,
`resolve_windsurf_event`, `resolve_claude_compatible_event`, `resolve_kiro_event`,
`format_vscode_hook_output`, `vscode_event_from_hook_event`, `format_plain_hook_output`, the eight
provider structs;
`all_editor_providers`, `get_editor_provider`, `default_management_provider`,
`default_version_control_provider`, `default_sandbox_provider`, `system_management_provider`,
`system_version_control_provider`.

Every `gh` and `git` argv is byte-identical to the snapshot, including
`repos/{owner}/{repo}/milestones/%d` for `get_milestone_title` versus `repos/:owner/:repo/milestones/%d`
for the rest (the Go code really does use both spellings), the `--paginate --jq .[]` NDJSON scan, the
two embedded GraphQL mutations with their tab indentation, and the `--color 1d76db --description
"Compose technology or bundle"` label creation.

## 3. Go package — deliberately almost empty

`📦️packages/🐹️go/🐹️.go` (`package providers`, `module github.com/usalu/semio/repo/providers`,
`go 1.25`, no external deps) contains exactly two things, per the brief:

1. **`🏃️ProcessRunner` region (new)** — `ProcessRequest`, `ProcessOutcome`, `ProcessExchange`,
   `ProcessTranscript`, the `ProcessRunner` interface, `SystemProcessRunner` and
   `RecordedProcessRunner`, with the same replay semantics as the Rust twin so Go cases can be
   fixture-driven the moment the split lands.
2. **`🧩️Pending` region** — the exact list of godfile symbols the split agent moves here, by name and
   snapshot line range: the whole `🪵️Providers` region (12348–13190), the 37 `gh*` symbols and three
   DTOs that today sit under Tickets/Goals (23230–23861, 40272–40538), the 11 editor hook functions
   from the Missing-Hook-Functions regions (43390–43674), and the `🪪️McpClientKind` block
   (45916–46006). It also states which symbols must NOT come here (`ToolKind`, `HookEvent`, the
   `HookResult` family — they are model shapes).

`🧪️_test.go` covers the runner both ways; the subprocess-spawning test is named
`TestQuickSystemProcessRunnerReportsStatusAndLogsArgv` so `goLevelTestArgs` skips it below the
`quick` level, which is the repo's own leveling convention.

## 4. Schema — `🧬️schema/🔣️.json`

`$defs`: `Argv`, `ProcessExchange`, `ProcessTranscript`, `ProcessRequest`, `ProcessOutcome`,
`ToolKind`, `HookEvent` (all 26 slugs), `EditorKind` (the 8 client slugs in registration order),
`McpClientKind`, `HookResult`, `HookOutputRecord`, `ManagementLabel`, `ManagementIssueMilestone`,
`ManagementIssue`, `ManagementMilestone`, `ManagementTranscriptCase`. Root properties `transcript`,
`hookOutputs`, `managementCases` describe the three fixture families the cases actually commit.

## 5. Language-agnostic cases and their evidence

| Case | Subjects | Reference | Modes | Levels |
| --- | --- | --- | --- | --- |
| `🐙️github-management-transcripts` | rust | `@oracle-gh-transcript-reader-typescript` (`🟦️.ts`) | 4 × differential, 1 × error | fundamental |
| `🌿️git-version-control` | rust, go | `@oracle-git-cli` — the real `git`, driven from `🟦️.ts` | 3 × differential, 1 × error | long |
| `🪝️editor-hook-output-format` | rust, go | `@no-oracle-repo-editor-hook-output` | 2 × differential, 1 × error | fundamental |
| `🪪️mcp-client-kind-parse` | rust, go | `@no-oracle-repo-mcp-client-identity` | 3 × differential, 1 × error | fundamental |

All four carry `@comparison-ordered-json-v1`. Three `noOracleDecisions` are recorded in
`🔮️oracle/🔣️.json` with full rationales: `repo-gh-cli-transcript`, `repo-editor-hook-output`,
`repo-mcp-client-identity`.

**Why the transcripts case has an oracle rather than only a decision.** The brief asked for a
`noOracleDecisions` entry *and* a `🟦️.ts` cross-implementation adapter. A TypeScript adapter is only
ever **dispatched** by the harness in the oracle role unless the owner ships `📦️packages/🟦️typescript`
(`ownerShipsImplementation`, `📜️script.ts:377`), and this module has no TypeScript package and needs
no devDependency, so a `@no-oracle-` tag would have left the second reader on disk and never executed
— evidence that does not run is not evidence. The decision is therefore recorded in the manifest
(`repo-gh-cli-transcript`, with the argument that no third-party library consumes the GitHub CLI's own
`--json` projections and that going to the REST API would put the network inside a test), and the
second reader is registered as `kind: "cross-semio-implementation"` — the schema's own name for "a
second implementation written inside this repository, explicitly NOT independent evidence" — so it
runs on every parity pass and is labelled for exactly what it is worth.

**`git-cli` oracle registration.** `kind: "third-party-cli"`, `ecosystem: "javascript"`,
`package: "git"`, no `oracleHostPackages`. `ecosystem` names the **host** the oracle runs in, not the
reference: `oracleDecision` maps ecosystem → adapter implementation, and `"native"` maps to no adapter
at all, so a CLI oracle has to be hosted somewhere. Nothing is installed and nothing enters
`🔒️dependencies.json`, because no package is linked — the oracle adapter shells out to whatever `git`
the machine has and builds its own throwaway repository.

**Throwaway repositories are created under the OS temp directory, not `ctx.workDir`.** The work
directory lives inside this checkout's own cache, so `git rev-parse` there answers from the
surrounding repository and the not-a-repository scenario could never fail. The directory name is fixed
per implementation and scenario (`semio-providers-<impl>-<scenario>`) and removed at the start of each
run, so re-runs reuse one directory instead of accumulating. Object names are never projected — they
are a function of the clock and the author identity; what is projected is the branch, whether HEAD is
a 40-hex object name, whether two reads agree, the staged path list, and whether a checkpoint advanced
HEAD.

**Hook output is projected parsed, not as a string.** Go marshals a struct in declaration order and
Rust's `serde_json::Map` is sorted, so comparing the raw formatted string would compare two JSON
writers rather than two implementations. Both adapters parse the formatted output and project the
record; `ordered-json-v1` treats key order as insignificant and array order as significant, which is
exactly the right contract here. The fixture uses only base-field results (`allowed` + `message`)
because Go's `HookResultBase` is the one concrete result both implementations can build identically
from a fixture; the event-specific results are `HookResult::extra` in Rust and are covered by the
crate's own unit tests.

### What the Go adapters cover, and what they cannot

The harness now wires **every** `go.work` module into a generated Go host by `require` + `replace`
(`goSutModule` + `goWorkspaceModules`, `🧪️test/📜️script.ts:491-533` — added by another agent during
this ticket; the gap flagged in `📓️harness-verification.md` §2 is closed). So a Go adapter can import
`github.com/usalu/semio/repo/client` today, and all three Go adapters here do, each carrying the
marker `// 🚚️ repoint to github.com/usalu/semio/repo/providers after split`.

`🐙️github-management-transcripts` has **no Go adapter**, and this is a statement about the Go source
rather than a gap in the work:

- the GitHub provider today calls `gh` through `ExecCommand` with no injectable seam, so a transcript
  cannot be substituted for the machine at all; and
- every parsing half is **unexported** — `ghIssue`, `ghMilestone`, `ghLabel`, `ghGetIssueDetails`,
  `ghFindMilestoneByTitle`, `ghListRepoLabels`, `ghCreateIssue`, `ghExtractIssueURL` are all
  lowercase, so an adapter in another package cannot reach them even to test the pure parsing.

The exported surface (`GetManagementProvider()`, `ManagementProvider`) only offers methods that would
shell out to a real `gh` against a real repository. Precisely, until the split runs and rewrites those
bodies onto `ProcessRunner`, **the Go side of GitHub management is untested**: the argv sequences, the
`gh issue view`/`gh api milestones`/`gh label list` parsing, `ghExtractIssueURL`, and the
milestone-title resolution inside `ghCreateIssue` are all covered for Rust (and cross-checked by the
TypeScript reader) and for nothing else. The moment `🐹️.go`'s `🧩️Pending` list is filled in, adding
`🐹️.go` to that case is a mechanical port of the existing `🦀️.rs` adapter — the fixture, the feature
and the projections need no change.

The other three cases are fully covered on both subjects: `🌿️git-version-control` exercises
`client.GitVersionControlProvider` (exported struct, exported methods) against real git;
`🪝️editor-hook-output-format` exercises `client.GetEditorProvider`, `FormatHookOutput`,
`NativeEventFromHookEvent`, `ResolveNativeEvent`, `client.HookResultBase` and `client.AllHookEvents`;
`🪪️mcp-client-kind-parse` exercises `client.ParseMcpClientKind`, `McpServerName`,
`HookClientForMcpKind` and `McpKindFromResolvedClient`.

## 6. Verification — real command output

```
$ cargo test -p semio-framework-repo-providers        # RUSTC_WRAPPER=""
running 12 tests
test tests::mcp_client_kinds_round_trip_through_their_slugs ... ok
test tests::every_editor_resolves_and_names_events ... ok
test tests::missing_milestone_leaves_create_issue_without_the_flag ... ok
test tests::shell_events_fall_back_to_terminal_only_for_generic_tools ... ok
test tests::the_null_provider_never_reaches_a_process ... ok
test tests::archive_branch_is_zero_padded ... ok
test tests::copilot_wraps_the_permission_decision_and_others_do_not ... ok
test tests::ports_are_reachable_from_the_provider_implementations ... ok
test tests::recorded_runner_replays_by_argv_and_logs_every_call ... ok
test tests::extract_issue_url_reads_both_bare_and_embedded_forms ... ok
test tests::git_provider_reads_the_branch_and_the_staged_files ... ok
test tests::github_provider_issues_the_documented_argv_and_parses_the_issue ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

```
$ GOWORK=off go test ./... -v -short          # in 🧩️providers/📦️packages/🐹️go
--- PASS: TestRecordedProcessRunnerReplaysByArgvAndLogsEveryCall (0.00s)
--- PASS: TestRecordedProcessRunnerParsesTheFixtureShape (0.00s)
--- PASS: TestQuickSystemProcessRunnerReportsStatusAndLogsArgv (0.31s)
PASS
ok  	github.com/usalu/semio/repo/providers	0.936s
```

```
$ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts discover | grep providers
test-…-providers-dab64b-🌿️git-version-control            🧪️tests/🌿️git-version-control            [rust,typescript,go]
test-…-providers-dab64b-🐙️github-management-transcripts  🧪️tests/🐙️github-management-transcripts  [rust,typescript]
test-…-providers-dab64b-🪝️editor-hook-output-format      🧪️tests/🪝️editor-hook-output-format      [rust,go]
test-…-providers-dab64b-🪪️mcp-client-kind-parse          🧪️tests/🪪️mcp-client-kind-parse          [rust,go]
```

```
$ … parity fundamental --case 🐙️github-management-transcripts
[test] level=fundamental cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
$ … parity fundamental --case 🪝️editor-hook-output-format
[test] level=fundamental cases=1 executed=6  passed=6  failed=0 errored=0 parity=3/3
$ … parity fundamental --case 🪪️mcp-client-kind-parse
[test] level=fundamental cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
$ … parity long --case 🌿️git-version-control
[test] level=long        cases=1 executed=12 passed=12 failed=0 errored=0 parity=12/12
```

`executed` counts every (scenario × role × implementation) pass; `parity` counts oracle-versus-subject
verdicts plus cross-subject pairs. The git case's 12/12 is 4 scenarios × (git-oracle×rust,
git-oracle×go, rust×go).

`contract --case <each of the four>` reports **no breach scoped to this module**. The breaches it
prints are pre-existing and repo-wide: the `🗒️note` plugin's unfixtured mutations, four
`testing/discovery` baselines (`temp`, `🧰️framework`, `.storybook`, `✏️s`, `♻️mit-bestand`), and one
`testing/dependency` `oracle-in-production` on this crate's `🦀️.rs` — the last one fires because some
owner registered `serde_json` as an oracle (`serde-json-equation-carrier-reader`) and every repo Rust
crate that follows `📋️plan.md` §3's "dependencies only `serde`/`serde_json`" rule trips it. The
identical breach is already recorded against `🧾️yaml` and `🔗️graphql`'s crates; it is not this
module's to resolve.

## 7. What is left

1. **Wave 2 `go-split` must fill `🐹️.go`'s `🧩️Pending` list.** Every symbol is named with its snapshot
   line range. The split must also rewrite the moved bodies onto `ProcessRunner` — that is the only
   substantive edit, and without it the Go GitHub provider stays untestable (§5).
2. **Add `🐹️.go` to `🧪️tests/🐙️github-management-transcripts`** once (1) lands. Port the existing
   `🦀️.rs` adapter; the feature, the fixture and the five projections are already language-neutral.
3. **Move `ToolKind`/`HookEvent`/`HookResult` down to `📐️model`** and delete the `📐️ModelPending`
   region plus its Go counterpart note (§2).
4. **Decide where `McpClientKind` finally lives** (`🧩️providers`, as implemented, or `🪪️identity`) —
   fix 7 accepts either; the identity executor should confirm. Nothing depends on the answer beyond
   one region move plus a re-export.
5. **`Kind` (fix 1) is untouched here on purpose.** When the entity-kind enum lands in `🪪️identity`,
   `ManagementProvider::kind`/`VersionControlProvider::kind`/`SandboxProvider::kind`/
   `EditorProvider::kind` can be retyped from `&'static str` without touching a single provider body.
6. **Host-level slowness, not this module's.** `bun x nx run @semio-tech/repo-providers-go:test`
   exceeds the 15 s fundamental budget on this Windows host — `go test` costs ~19 s wall even fully
   cached, while the tests themselves take 0.9 s. The pre-existing `@semio-tech/repo-mcp-go:test`
   fails the same way, so this is the host's Go tooling, not the package.
   `bun …/🧩️providers/📦️packages/🦀️rust/📜️script.ts test` currently dies in `loadTaxonomy` with
   `generatorContracts["wgpu-frame-worker"] tracked output … is missing` — the pre-existing
   `📐️model` crate's script fails identically, so it is another agent's in-flight generated file, not
   this module. `cargo test -p semio-framework-repo-providers` passes directly (§6).
7. **`Cargo.lock` contention.** Concurrent cargo invocations on this host intermittently fail with
   `The requested operation cannot be performed on a file with a user-mapped section open (os error
   1224)`. Retrying the same command succeeds; nothing in the module causes it.
