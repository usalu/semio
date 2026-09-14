# 📓️ `⌨️cli` verbs — the eleven refusing verbs wired, and the four behavioural gaps closed

Executor: Opus 5 (`cli-verbs`), wave 3 follow-up to `📓️opus-cli.md` §4.
Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`, crate `semio-framework-repo-cli`.
Reference: the Go CLI package `📦️packages/🐹️go/{🐹️.go, 🖨️render.go, 🔌️mcp.go}` and the Go domain packages,
rebuilt from source at the start of verification (`go build -o semio-repo.exe ./🚀️bin`) — the committed
`semio-repo.exe` was stale and answered with wording no longer in the Go tree.

## 1. What landed

`📓️opus-cli.md` §4 listed eleven verbs that refused with a named missing port, plus four narrower
behavioural gaps. All fifteen are now wired and diffed against the Go binary.

| Verb | Now backed by |
| --- | --- |
| `test` | `🏃️test-runner`: `FilesystemSnapshot::for_bundles` + `resolve_test_scopes` + `plan_scopes`, each invocation announced with `running_line` and spawned with the parent's streams |
| `hook` | `🪝️hooks`: `resolve_hook_event` → `dispatch_hook` over `SystemHookEnvironment` / `WorkspaceTestFileResolver` → `render_hook_output` |
| `loc` | `📊️metrics`: `SystemGit` + `build_loc_report`, rendered as JSON / `render_markdown` / a port of `renderLocText` |
| `mermaid` | `🌳️tree`: the three LOC treemaps, over a `StreamFiles`-shaped repository walk |
| `export` | `📡️events`: `build_export_snapshot` + `Store::append` over the production `RepoContext` |
| `configure` | `🧩️providers`: `remove_git_hooks` + `install_micro_commit_hooks` |
| `micro-commit` | the reference's bun resolution, delegating to `./📜️script.ts micro-commit` |
| `benchmark` | the five ecosystem recipes + `📊️metrics`' `benchmark_csv` |
| `update` | the dependabot manifest reader and the per-ecosystem recipes |
| `auth whoami` | `📡️events`: `server_whoami` over the hand-rolled HTTP/1.1 client |
| `ticket purge-artifacts` | `🎫️tickets`: `ticket_folder_roots` + `purge_oversized_artifacts` |
| `contributor add` / `remove` | `🧑️contributors` writes, through the new `VerbContext` |
| `ticket delete` | the ticket folder removal and issue deletion, through `VerbContext` |
| `sync management` | `🧩️providers`' `GitHubManagementProvider` over the real `gh` runner, through `VerbContext` |

### 1.1 New regions in the domain crates

Everything the verbs needed from the machine went into the crate that owns the port, as a new region:

| Crate | Region | Content |
| --- | --- | --- |
| `🏠️workspace` | `🔎️ProgramLookup` | `look_path`, `program_exists`, `spawn_command` — the `exec.LookPath` twin (§1.4) |
| `📡️events` | `🔐️ServerClient` | `server_addr`, `server_token`, `server_whoami` |
| `🎫️tickets` | `🧹️RealPurge` | `ticket_folder_roots`, `purge_oversized_artifacts` over a real folder |
| `🧩️providers` | `🪝️GitHookFiles` | `REPO_MANAGED_GIT_HOOKS`, `MICRO_COMMIT_GIT_HOOKS`, `remove_git_hooks`, `install_micro_commit_hooks` |
| `🏃️test-runner` | `💽️RealWorld` | `FilesystemSnapshot::{from_root, for_bundles, hydrate, with_bundles}`, `SystemProcessRunner`, `run_invocation_inherited`, `running_line` |
| `🪝️hooks` | `💽️SystemPorts` | `SystemHookEnvironment`, `WorkspaceTestFileResolver` |
| `🌳️tree` | `🔢️MermaidLoc` | `MermaidLocFile`, `mermaid_loc_by_technologies_bundles_folders_files`, `mermaid_loc_flat` and the two titles |
| `🧑️contributors` | `💽️ContributorWrites` | `load/create/save/remove_contributor`, `contributor_dir`, `git_author_alias` |

### 1.2 `VerbContext`

`FsContext` left `ticketDelete`, `contributorAdd`, `contributorRemove` and `syncManagement` unwired
because the mutations they need reach `🧩️providers` and the contributor registry. `FsContext` is
owned by the concurrent `consolidation` executor, so rather than edit it, `🔖️VerbPorts` adds
`VerbContext`: a `RepoContext` that delegates all forty-three already-answered surfaces to
`FsContext` and implements the four missing ones itself. `Session::graphql` builds a `VerbContext`,
so every GraphQL-backed verb now runs against the complete context.

`sync_management` is the faithful port of the reference's whole reconciliation: the label catalogue,
then every goal (root goals get a milestone, child goals get an issue and a sub-issue link, a stale
milestone is migrated away), then every ticket carrying an issue, then a sweep over the remaining
issues. Every remote failure is a `Warning: …` line on standard error, exactly as
`workspace.WriteWarningf` writes it.

### 1.3 The snapshot the `test` verb builds

`FilesystemSnapshot::from_root` walks a whole repository, and that is the right shape for the hook
resolver, which really does need every `.rs` file in the tree. It is the wrong shape for the `test`
verb: the reference never walks for `test`, it stats the manifests of each bundle root. A full walk
of this repository takes 81s in Rust (and ~29s in Go for the hook case, which does need it), so the
verb builds `FilesystemSnapshot::for_bundles` instead — the bundle table plus one directory listing
per bundle root, which is everything `detect_bundle_language`, `detect_js_test_runner` and the
scope resolver read. `semio test` now answers in 0.2s, as the reference does in 0.06s.

`WorkspaceTestFileResolver` defers its walk to first use with a `OnceCell`, so the hook events that
never ask which test files a command selects — which is most of them — never pay for it.

### 1.4 `exec.LookPath`, and why `npx` did not run

`std::process::Command::new("npx")` hands the name to `CreateProcess`, which appends only `.exe`.
On Windows `npx`, `npm`, `bun` and `uv` ship as `.cmd` shims, and a Git Bash `PATH` additionally
carries an extensionless `npx` shell script that `CreateProcess` rejects with error 193. Go's
`exec.LookPath` resolves the name against `PATHEXT` and never accepts an extensionless file.

`🏠️workspace`'s `🔎️ProgramLookup` is that twin, and `spawn_command` additionally routes a resolved
`.cmd`/`.bat` through `%COMSPEC% /c`, which this runtime cannot spawn directly. `🏃️test-runner`,
`🧩️providers` and the `benchmark`/`update`/`micro-commit` verbs all spawn through it. Without this
`semio test` on a TypeScript bundle refused where `semio-repo test` ran vitest.

### 1.5 Divergences found and fixed outside the verb regions

Each of these was a real disagreement with the reference, found by diffing and fixed in the crate
that owns it:

1. **Hook result field order.** `🧩️providers`' `HookResult` carried its event-specific fields in a
   `serde_json::Map`, which sorts; Go's `model.HookResult*` records marshal in declaration order.
   `HookResult.extra` is now an ordered `Vec<(String, Value)>` with a Go-faithful `Serialize`.
2. **Hook `message` shadowing.** Go's `HookResultAgentBase` declares
   `MessageID string \`json:"message,omitempty"\``, which shadows `HookResultBase.Message`: a blocked
   command's reason is readable through `GetMessage()` and never reaches the marshalled record.
   `HookResult.message_shadowed` reproduces that, and `agent_base` sets it.
3. **The same two, in the Go twin.** Go's own pure `hooks.HookResult` is map-based and therefore
   disagreed with the Go *binary* it is the twin of. It gained `Order` and `MessageShadowed` and an
   ordered `MarshalJSON`, so `DispatchHook` and `RunHook` now marshal alike — and the
   `🖨️hook-result-formatting` parity case, which broke on the Rust change, is green again on both.
4. **VS Code envelope ordering.** The one place the reference *does* sort is
   `FormatVSCodeHookOutput`, which round-trips the record through a `map[string]interface{}`. The
   Rust twin now sorts there and only there.
5. **`SystemProcessRunner` spawn failures.** `🧩️providers` reported the runtime's own words
   ("program not found"); Go reports `exec.Error`, i.e. `exec: "gh": executable file not found in
   %PATH%`. Ported verbatim.
6. **`SystemGit` failures.** `📊️metrics`' `SystemGit::run` returned bare stderr; Go returns
   `git <args>: <stderr>`.
7. **Go float encoding.** `encoding/json` writes a `float64` with no fractional part as `0`, not
   `0.0`. `LocLangStats`' three float fields now serialize through `go_float`, so `loc --json`
   matches byte for byte.
8. **`--branch` default.** The `loc` flag defaulted to `"dev"`; both `DefaultBranch` constants are
   `⛳️wip`. The flag now takes `metrics::DEFAULT_BRANCH`.

### 1.6 Divergences found and fixed inside the verb regions

- **The top-level error contract.** `repo_cli::run` printed `Error: <message>` and returned the
  engine's exit code. The reference's `main` prints `err.Error()` bare on standard error and exits
  1 — and for a failure that carries an exit code it prints `exit status N`, because
  `workspace.ExitError` renders that way. Reproduced exactly.
- **`auth status`** did not normalise `COMPOSE_SERVER_ADDR`; it now prints what `GetServerAddr`
  returns.
- **`contributor add`'s emails.** The reference reads operands 2… as emails when `--email` is
  absent; the Rust verb only read the flag.
- **`ticket purge-artifacts <path>`** anchored the ticket layout at the tickets directory instead of
  the repository meta directory, so it printed `🎫️tickets/🎫️tickets/…`, and printed forward slashes
  where `filepath.Join` prints backslashes.
- **Go's JSON HTML escaping.** `json.Marshal` and `json.Encoder` escape `<`, `>` and `&`; every
  `--json` document this crate writes by hand now goes through `escape_like_go`.
- **Export identities and field order.** Entities are keyed by the identity `GetID()` computes from
  the path, not by the record's stored id, and `ExportResult` is written in declaration order.

### 1.7 Two deliberate divergences

- **`mermaid` tie ordering.** The reference sorts its flat treemaps with `sort.Slice` — unstable —
  over a Go map, so two languages with equal LOC come out in a different order run to run. Verified:
  eight consecutive runs of `semio-repo mermaid loc-by-language` on the same tree produced two
  different orders. The Rust twin breaks ties by label, which is deterministic. Reproducing a
  coin flip is not parity.
- **The `semio` root.** `semio` with no argv is the orchestrator (TUI, `daemon`, `workflow`, …)
  while `semio-repo` with no argv is the repo command tree. That split is `📓️opus-cli.md` §1.9 and
  is unchanged.

## 2. Language-agnostic tests

Three new cases under `⌨️cli/🧪️tests/`, each with a Rust adapter, a Go adapter and a
`noOracleDecision` in `⌨️cli/🔮️oracle/🔣️.json`. The Go adapters call three new harness entry points
in the Go package's `🧪️Projection` region — `TestVerbLines`, `ExportRecords`, `HookVerbDispatch` —
mirroring `repo_cli::{test_verb_lines_json, export_records_json, hook_verb_dispatch_json}`.

| Case | Scenarios | Fixture |
| --- | --- | --- |
| `🧪️test-verb-planning` | operands plan their stated invocations · an unplannable scope refuses and announces nothing · planning is deterministic | one frozen snapshot (four bundles, four languages) and seven operand vectors stating the `Running: …` lines in order |
| `📤️export-verb-records` | the batch carries one count per kind and a sha-256 digest · every input id reads `snapshot:<digest>:<kind>:<entity id>` and the ids are sorted · the batch is deterministic | `🔗️graphql`'s frozen recording context |
| `🪝️hook-verb-dispatch` | every invocation writes its bytes · an unknown event slug is refused before the domain sees it · dispatch is deterministic | ten native invocations across `claude-code`, `copilot-chat` and a refusal |

The export case deliberately does **not** compare the digest across implementations: the digest
hashes the marshalled entity records, whose field encoding belongs to `📐️model` and is held by that
owner's cases (§4.1). What it holds is the batch's own shape — which entities, under which
identities, in which order — plus the laws the digest obeys inside one implementation.

## 3. Verification — real command output

### 3.1 Parity, every owner this ticket touched

```
🏠️workspace   [test] level=fundamental cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
🧩️providers   [test] level=fundamental cases=4 executed=29 passed=29 failed=0 errored=0 parity=22/22 not-exercised=1
🏃️test-runner [test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
📊️metrics     [test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19
🪝️hooks       [test] level=fundamental cases=5 executed=54 passed=54 failed=0 errored=0 parity=45/45
📡️events      [test] level=fundamental cases=4 executed=27 passed=27 failed=0 errored=0 parity=27/27
🎫️tickets     [test] level=fundamental cases=5 executed=57 passed=57 failed=0 errored=0 parity=42/42
🌳️tree        [test] level=fundamental cases=5 executed=24 passed=24 failed=0 errored=0 parity=12/12
🧑️contributors [test] level=fundamental cases=3 executed=7 passed=7 failed=0 errored=0 parity=5/5
⌨️cli          [test] level=fundamental cases=8 executed=32 passed=32 failed=0 errored=0 parity=16/16
```

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity quick --owner ⌨️cli
[test] level=quick cases=8 executed=48 passed=48 failed=0 errored=0 parity=24/24
```

### 3.2 Build, tests, clippy

```
$ cargo build --release -p semio-framework-repo-cli --bins
    Finished `release` profile [optimized] target(s) in 41.96s

$ cargo test --release -p semio-framework-repo-cli -p semio-framework-repo-hooks \
    -p semio-framework-repo-providers -p semio-framework-repo-tickets -p semio-framework-repo-events \
    -p semio-framework-repo-tree -p semio-framework-repo-test-runner -p semio-framework-repo-contributors \
    -p semio-framework-repo-workspace -p semio-framework-repo-metrics
     13 test result: ok. 0 passed …
      1 test result: ok. 12 passed; 0 failed …
      1 test result: ok. 12 passed; 0 failed …
      1 test result: ok. 9 passed; 0 failed …
      1 test result: ok. 6 passed; 0 failed …
      1 test result: ok. 4 passed; 0 failed …

$ cargo clippy --release -p semio-framework-repo-cli … | grep -- '-->' | grep -E '⌨️cli|🏠️workspace|🧩️providers|📊️metrics|🏃️test-runner|🪝️hooks|🎫️tickets|📡️events|🌳️tree|🧑️contributors'
--- clippy scan complete            # no warning against any line of any crate this ticket touched
```

### 3.3 Verb-by-verb diff against the Go binary

`$TICKET/🗑️generated/cli-verbs/sweep.txt` is the whole run; the driver is
`$TICKET/🏗️sweep-cli-verbs.sh`. Each entry runs both binaries with the same argv, in the same
working directory, normalises only the hook timestamp, the export digest and the child runners' own
elapsed-time lines, and diffs stdout+stderr and the exit code.

```
== auth ==
PASS  auth whoami (exit 1)
PASS  auth status (exit 0)
== configure / micro-commit ==
PASS  configure (exit 0)
PASS  micro-commit reset (no bun) (exit 1)
== update ==
PASS  update (exit 0)
PASS  update rust (exit 0)
PASS  update --dry-run (exit 0)
PASS  update --apply (no dependabot) (exit 1)
== benchmark ==
PASS  benchmark --dry-run (exit 0)
== ticket purge-artifacts ==
PASS  ticket purge-artifacts --all (exit 0)
PASS  ticket purge-artifacts 26/09/06/… (exit 0)
== test ==
PASS  test (all) (exit 1)
PASS  test 🧰️demo (exit 1)
PASS  test repo://p/u/demo (exit 1)
PASS  test bundle engine (exit 0)
PASS  test bundle kernel (exit 0)
PASS  test bundle core (exit 1)
== mermaid ==
PASS  mermaid loc-by-technologies-bundles-folders-files (exit 0)
FAIL  mermaid loc-by-language (go=0 rs=0)
5d4
<     "typescript": 3
6a6
>     "typescript": 3
PASS  mermaid loc-by-contributors (exit 0)
== loc ==
PASS  loc --json (exit 0)
PASS  loc --md (exit 0)
PASS  loc --text (exit 0)
PASS  loc --json --by-contributors (exit 0)
PASS  loc --md --by-contributors (exit 0)
PASS  loc --json --history (exit 1)
PASS  loc --json --languages Go,Rust (exit 0)
== export ==
FAIL  export
9c9
<   "definitions": 0
---
>   "definitions": 4
== contributor ==
PASS  contributor add (stdout + document)
PASS  contributor remove (exit 0)
== sync management (offline) ==
PASS  sync management (exit 0)
== ticket delete (mutation surface) ==
PASS  graphql ticketDelete (exit 1)

TOTAL pass=30 fail=2
```

The `test`, `mermaid` and `export` entries run in a purpose-built fixture repository (one
technology, a TypeScript bundle with a vitest manifest, a Go module, a Rust crate) because the
reference's `mermaid` and `test` read the process-global root from the working directory rather than
from `--repo`; `loc` runs in a small real git repository because a full `loc` over this monorepo does
not finish within thirty minutes in **either** implementation. The `test` entries really do run
`npx vitest run`, `go test ./...` and `cargo test`, and the diff covers the runners' own output and
the exit code the verb ends with.

The additional hook sweep — eleven native events × two output modes, on this repository — is
`identical` for all twenty-two:

```
  agent.started/claude-code --json: identical (exit 0)          agent.started/claude-code : identical (exit 0)
  agent.ended/claude-code --json: identical (exit 0)            agent.ended/claude-code : identical (exit 0)
  agent.prompt.submitting/claude-code --json: identical (0)     agent.prompt.submitting/claude-code : identical (0)
  agent.compacting/claude-code --json: identical (exit 0)       agent.compacting/claude-code : identical (exit 0)
  agent.thinking.ended/claude-code --json: identical (exit 0)   agent.thinking.ended/claude-code : identical (exit 0)
  agent.tool.terminal.starting (git commit) --json: identical (0)   … plain: identical (exit 1)
  agent.tool.terminal.starting (ls -la) --json: identical (0)       … plain: identical (exit 0)
  agent.tool.code.edit.ended/claude-code --json: identical (0)      … plain: identical (exit 0)
  PreToolUse/copilot-chat --json: identical (exit 0)            PreToolUse/copilot-chat : identical (exit 0)
  SessionStart/claude-code --json: identical (exit 0)           SessionStart/claude-code : identical (exit 0)
  preToolUse/cursor-chat --json: identical (exit 0)             preToolUse/cursor-chat : identical (exit 0)
```

### 3.4 The one entry that still differs, and why

`export` differs in a single number:

```
== export ==
FAIL  export
9c9
<   "definitions": 0
---
>   "definitions": 4
```

This is not the `export` verb. The same disagreement is visible on the aggregate itself:

```
$ semio-repo graphql '{ repo { definitions { id } } }' --json
{"repo":{"definitions":[]}}
$ semio      graphql '{ repo { definitions { id } } }' --json
{"repo":{"definitions":[{"id":"🪨a"},{"id":"🪨b"},{"id":"🛠️run"},{"id":"🛠️run"}]}}
```

The reference's `RepoContext.GetDefinitions` walks its codebase context and answers nothing for this
tree; the Rust `FsContext::definitions` answers four. `export` faithfully reports whatever the
context gives it, and every other count (technologies, bundles, folders, files, sections), the
identities and the sort order agree. §4.1 records it for the owner.

## 4. What is left

### 4.1 Divergences outside this ticket's regions

1. **`definitions()`** — the `FsContext` / `🗂️codebase` / `🚚️move` definition aggregate disagrees with
   the reference (above). It reaches `graphql`, `list`, `search`, `definition list` and `export`.
   Owner: `🔖️Context` (the `consolidation` executor) plus whichever of `🗂️codebase` / `🚚️move` is
   answering. The same walk also disagrees on `File.extension` (`ts` vs `.ts`) and `File.kind`
   (`""` vs `code`), and on `Folder.name`/`kind`.
2. **The export digest** — it hashes the marshalled entity records, and `📐️model`'s Go and Rust
   structs do not encode identically, so the two implementations compute different snapshot digests
   for the same repository. Nothing downstream compares digests across implementations today, but a
   coordinator that replayed one implementation's log into the other would notice. Owner: `📐️model`.
3. **The root verb list** — the Rust root registers six verbs the reference builds and never
   registers (`autofix`, `statute`, `checkpoint`, `interaction`, `draft`, `definition`) plus
   `ticket list`. That is `📓️opus-cli.md` §1.7/§1.8, unchanged and still deliberate.
4. **`analyze`'s short text** — the Go root says `Analyze an entity by its ID`, the Rust root says
   `Analyze codebase for breachs`. The Go harness's own `ProjectionRoot` already substitutes the
   latter, so the language-agnostic usage case agrees while the two binaries' root help does not.
   One of the two Go strings is stale; the audit wave should pick one.

### 4.2 Not exercised end to end

- **`sync management` against a live GitHub.** The verb was diffed offline (`PATH` without `gh`),
  where both implementations emit the same four lines and the same `{"syncManagement":true}`. It was
  **not** re-run online: the reference mutates the real repository's labels, milestones and issues,
  and one accidental run early in this session deleted GitHub labels that the catalogue no longer
  declares. Anyone verifying the online path should do it against a scratch repository.
- **`benchmark` with real ecosystems** and **`update --apply` on this repository** — both mutate the
  working tree; only the dry paths and the skip/refusal paths were diffed.
- **`micro-commit` with bun present** — only the `bun not found` refusal was diffed, because the
  workflow commits.
- **`loc` over this monorepo** — neither implementation finishes within thirty minutes here; the
  diff was taken on a small real git repository across nine flag combinations instead.

## 5. Files

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🧪️test-verb-planning/{🥒️.feature, 🦀️.rs, 🐹️.go, 🧫️fixtures/🧪️test-verb-vectors.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/📤️export-verb-records/{🥒️.feature, 🦀️.rs, 🐹️.go, 🧫️fixtures/🗄️repo-records.json}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧪️tests/🪝️hook-verb-dispatch/{🥒️.feature, 🦀️.rs, 🐹️.go, 🧫️fixtures/🪝️hook-invocations.json}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🏗️sweep-cli-verbs.sh`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-cli-verbs.md`

Updated:

- `⌨️cli/📦️packages/🦀️rust/🦀️.rs` — new `🔖️VerbPorts` region; `🔖️Dispatch2` (eleven dispatch arms, the
  top-level error contract, `auth status`, `contributor add`, `ticket purge-artifacts`, three
  harness entry points); `🔖️Tree` (the `loc --branch` default); `🔖️Verbs` (`Session::graphql` builds
  a `VerbContext`)
- `⌨️cli/📦️packages/🐹️go/🐹️.go` — three harness entry points in `🧪️Projection`
- `🏠️workspace/📦️packages/🦀️rust/🦀️.rs` — new `🔎️ProgramLookup` region
- `📡️events/📦️packages/🦀️rust/🦀️.rs` — new `🔐️ServerClient` region
- `🎫️tickets/📦️packages/🦀️rust/🦀️.rs` — new `🧹️RealPurge` region
- `🧩️providers/📦️packages/🦀️rust/{🦀️.rs, Cargo.toml}` — new `🪝️GitHookFiles` region; ordered
  `HookResult`; `SystemProcessRunner` spawn wording and program lookup; `🏠️workspace` dependency
- `🧩️providers/📦️packages/🐹️go/🐹️.go` — (none; the Go hook result lives in `🪝️hooks`)
- `🪝️hooks/📦️packages/🦀️rust/{🦀️.rs, Cargo.toml}` — new `💽️SystemPorts` region; `🏃️test-runner` and
  `🗂️codebase` dependencies
- `🪝️hooks/📦️packages/🐹️go/🐹️.go` — ordered `HookResult` (`Order`, `MessageShadowed`, `Fields`,
  `WriteOrderedObject`, `MarshalJSON`)
- `🏃️test-runner/📦️packages/🦀️rust/🦀️.rs` — new `💽️RealWorld` region
- `📊️metrics/📦️packages/🦀️rust/🦀️.rs` — `SystemGit::run` wording, `go_float` serialization
- `🌳️tree/📦️packages/🦀️rust/🦀️.rs` — new `🔢️MermaidLoc` region
- `🧑️contributors/📦️packages/🦀️rust/🦀️.rs` — new `💽️ContributorWrites` region
- `⌨️cli/🔮️oracle/🔣️.json` — three `noOracleDecisions`
- `⌨️cli/📦️packages/🐹️go/semio-repo.exe` — rebuilt from source (it was stale)
