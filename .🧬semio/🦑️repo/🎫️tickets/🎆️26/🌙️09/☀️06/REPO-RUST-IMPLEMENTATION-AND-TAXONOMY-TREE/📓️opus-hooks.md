# 📓️ Opus executor report — `🪝️hooks`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks`
Crate: `semio-framework-repo-hooks` (`📦️packages/🦀️rust/🦀️.rs`, 2 092 lines)
Go package: `github.com/usalu/semio/repo/hooks` (`📦️packages/🐹️go/🐹️.go`, 2 041 lines + `🔬️_test.go`, 4 008 lines) — **owned by the concurrent `go-split` executor, not edited here**.

This report resumes the job that a rate limit interrupted at "launch seed entries and regeneration". Everything below was executed on this host; no result is inferred.

## 1. What the module contains

Rust crate regions (`grep -n '^//#region' 🦀️.rs`):

```
1    //#region 🧲️Header
32   //#region 🔁️Reexports
48   //#region 🔖️HookError
74   //#region 🏷️HookKind
117  //#region 🎯️HookContext
187  //#region 🔌️Ports
229  //#region 🧭️PayloadReading
702  //#region 🏛️ToolClassification
928  //#region 🛡️BlockingPolicy
1243 //#region 🗺️PlanSteps
1357 //#region 🔶️Dispatch
1703 //#region 🖨️Formatting
1755 //#region 📓️SessionLogging
1977 //#region 🔖️MicroCommit
2020 //#region 🛤️Paths
2045 //#region 🧪️Tests
```

Ports (no process, clock or filesystem reaches the domain logic): `HookEnvironment` / `InertEnvironment`, `TestFileResolver` / `InertTestFileResolver`, `SessionStore` / `MemorySessionStore` / `DirectorySessionStore`. Micro-commit reset is delegation only — the argv is produced by the crate and handed to the environment port, so a test can replay it through a recorded runner without starting a process.

The `Cargo.toml` dependency set is `serde`, `serde_json` and the sibling path crates `semio-framework-repo-{model,workspace,providers}` — no external runtime crate, per §3 of the plan.

Test cases (5 case dirs, 22 scenarios, all with emoji-led directory names per the 2026-09-06 06:10 coordinator decision):

| Case | Scenarios | Evidence |
| --- | --- | --- |
| `🔀️native-event-normalisation` | 4 | no-oracle decision `repo-hooks-native-normalisation` (specification vectors) |
| `🛡️tool-blocking-policy` | 5 | oracle `hooks-blocking-policy-typescript` (`🟦️.ts`, regex engine vs. the subject's hand-rolled scanner) |
| `🗺️plan-step-extraction` | 3 | oracle `hooks-plan-merge-typescript` |
| `🖨️hook-result-formatting` | 5 | no-oracle decision `repo-hooks-result-formatting` (specification vectors) |
| `📓️session-logging` | 5 | oracle `hooks-session-log-typescript` |

`🔮️oracle/🔣️.json` declares `oracleHostPackages: []` — all three oracles are `cross-semio-implementation` second implementations written in the case's own `🟦️.ts`, so nothing enters `🔒️dependencies.json` and no third-party package is pulled in.

## 2. Clippy fixes applied this session

`cargo clippy -p semio-framework-repo-hooks --all-targets` reported three warnings in the crate. All three were fixed in `📦️packages/🦀️rust/🦀️.rs`:

1. `clippy::question_mark` at line 1199 — `let Some(first) = tokens.first_mut() else { return None };` → `let first = tokens.first_mut()?;`
2. `clippy::map_unwrap_or` at line 1309 — `.map(|found| (*found).clone()).unwrap_or_else(|| …)` → `.map_or_else(|| …, |found| (*found).clone())`
3. `clippy::collapsible_match` at line 1331 — the inner `if` of the `"completed" | "done"` arm folded into a match guard (semantics unchanged: the only other reachable arm is `_ => {}`)

After the fix, re-running clippy with the crate touched so it is genuinely recompiled:

```
$ touch .../🪝️hooks/📦️packages/🦀️rust/🦀️.rs
$ cargo clippy -p semio-framework-repo-hooks --all-targets
warning: this `if` can be collapsed into the outer `match`
   --> 🧰️framework\🛍️products\🦑️repo\🔨️modules\🏠️workspace\📦️packages\🦀️rust\🦀️.rs:558:17
warning: `semio-framework-repo-workspace` (lib) generated 1 warning
warning: called `map(<f>).unwrap_or(<a>)` on a `Result` value
    --> 🧰️framework\🛍️products\🦑️repo\🔨️modules\🧩️providers\📦️packages\🦀️rust\🦀️.rs:1276:19
warning: `semio-framework-repo-providers` (lib) generated 1 warning
```

**The `🪝️hooks` crate is clippy-clean.** The two remaining warnings belong to `🏠️workspace` and `🧩️providers` and were left to their owners.

## 3. Verification — real output

### 3.1 `cargo test -p semio-framework-repo-hooks`

```
running 6 tests
test tests::a_kill_by_lsof_port_is_blocked ... ok
test tests::a_neutral_slug_resolves_without_a_client ... ok
test tests::a_session_log_is_silent_when_logging_is_off ... ok
test tests::inline_python_hiding_git_is_blocked ... ok
test tests::a_composite_command_is_blocked_by_its_worst_segment ... ok
test tests::a_dropped_plan_step_is_abandoned ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests semio_framework_repo_hooks
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 3.2 Harness `discover`

```
test-framework-products-repo-modules-hooks-59e89a-hook-result-formatting       …/🪝️hooks/🧪️tests/🖨️hook-result-formatting        [rust]
test-framework-products-repo-modules-hooks-59e89a-native-event-normalisation   …/🪝️hooks/🧪️tests/🔀️native-event-normalisation    [rust]
test-framework-products-repo-modules-hooks-59e89a-plan-step-extraction         …/🪝️hooks/🧪️tests/🗺️plan-step-extraction          [rust,typescript]
test-framework-products-repo-modules-hooks-59e89a-session-logging              …/🪝️hooks/🧪️tests/📓️session-logging               [rust,typescript]
test-framework-products-repo-modules-hooks-59e89a-tool-blocking-policy         …/🪝️hooks/🧪️tests/🛡️tool-blocking-policy          [rust,typescript]
```

### 3.3 `subject fundamental --owner 🪝️hooks --implementation rust`

```
[test] level=fundamental cases=5 executed=21 passed=21 failed=0 errored=0 parity=0/0
```

### 3.4 `oracle fundamental --owner 🪝️hooks`

```
[test] not-exercised …/🪝️hooks/🧪️tests/🖨️hook-result-formatting (recorded no-oracle decision repo-hooks-result-formatting — its evidence is discharged by the subject phase)
[test] not-exercised …/🪝️hooks/🧪️tests/🔀️native-event-normalisation (recorded no-oracle decision repo-hooks-native-normalisation — its evidence is discharged by the subject phase)
[test] level=fundamental cases=5 executed=12 passed=12 failed=0 errored=0 parity=0/0 not-exercised=2
```

### 3.5 `parity fundamental --owner 🪝️hooks`

```
[test] level=fundamental cases=5 executed=33 passed=33 failed=0 errored=0 parity=12/12
```

All twelve subject↔oracle pairs agree under `ordered-json-v1`; the two no-oracle cases are discharged by their recorded decisions plus the frozen specification vectors in their fixtures.

Command form used for all of the above (from the repo root):

```bash
export RUSTC_WRAPPER=""
export GOWORK=C:/git/semio/go.work
export SEMIO_TEST_BUDGET_MS=600000
M="./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts"
bun "$M" subject fundamental --owner 🪝️hooks --implementation rust
bun "$M" oracle  fundamental --owner 🪝️hooks
bun "$M" parity  fundamental --owner 🪝️hooks
```

### 3.6 `contract`

`bun "$M" contract --owner 🪝️hooks` validates the whole tree (the `--owner` selector does not narrow this phase). It exits non-zero, but **no breach in `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` is attributable to `🪝️hooks`**: `grep -c "🪝️hooks"` over the breach file returns `0`, and the single literal `hooks` match is `temp/brepjs/apps/playground/src/hooks/useCameraPresets.ts`. The reported breaches are the repo-wide `testing/discovery` baselines (`temp`, `🧰️framework`, `.storybook`, `✏️s`, `♻️mit-bestand`) and note-plugin fixture-vector gaps owned elsewhere.

## 4. Launch entries

Added to `.vscode/🧩️launch.seed.jsonc` (immediately after the `🧪️test🧰️repo📜️statutes🥒️parity` entry, following the existing grouping and naming):

- `🧪️test🧰️repo🪝️hooks🦀️rust` → `bun nx run @semio-tech/repo-hooks-rs:test`
- `🧪️test🧰️repo🪝️hooks🐹️go` → `bun nx run framework-products-repo-modules-hooks-go:test`
- `🧪️test🧰️repo🪝️hooks🥒️parity` → `bun ./…/🧪️test/📜️script.ts parity fundamental --owner …/🪝️hooks`

Regenerated:

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/📇️registry/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
$ grep -c "🧪️test🧰️repo🪝️hooks" .vscode/launch.json
3
```

## 5. Go side — blocked, honestly reported

**No `🐹️.go` test adapters were added, because they cannot be written against the Go package as it stands, and the Go package belongs to the concurrent `go-split` executor.**

The Go package builds (`GOWORK=C:/git/semio/go.work go build ./...` → exit 0), but its exported surface is the pre-split coarse one:

```
var BlockedToolPatterns
func ExtractSessionIDFromInput / ExtractTranscriptFromInput
func HookEventKind
func InstallMicroCommitHooks / RemoveGitHooks / RunMicroCommitScript
func IsToolBlocked
func ResolveEventSessionID / ResolveEventSecondID
func ResolveHookEvent
func RunHook / RunHookExecutionCtx
```

The Rust adapters project over the crate's fine-grained, port-isolated API — `classify_tool`, `classify_command_kind`, `extract_command_and_cwd`, `extract_llm`, `extract_effort`, `extract_message_id`, `extract_hook_event_name`, `resolve_parent_session_id`, the plan-step merge, the `HookOutput` formatter, the `SessionStore` port. In the Go package the corresponding functions are either unexported (`extractCommandCwdFromInput`, `extractLLMFromInput`, `extractEffortFromInput`, `extractHookEventNameFromStdin`, `resolveParentSessionID`, …), absent entirely (there is no tool/command classification and no plan-step merge), or reachable only through `RunHook`, which writes the filesystem directly (`writeHookArtifacts`, `writeSessionHookLog`) instead of going through a store port.

This matters because of the harness's own rule (`🧪️test/📜️script.ts:362-394`, `ownerShipsImplementation`): the owner ships a `📦️packages/🐹️go`, so the moment a `🐹️.go` adapter file appears, **go becomes a claimed subject for that case and every scenario it does not register errors with `adapter has no subject registration` and drags the case's parity ratio to zero**. A partial Go adapter is therefore strictly worse than none. Current state, as executed:

```
$ bun "$M" subject fundamental --owner 🪝️hooks --implementation go
[test] not-exercised …/🖨️hook-result-formatting (recorded no-oracle decision repo-hooks-result-formatting — …)
[test] not-exercised …/🔀️native-event-normalisation (recorded no-oracle decision repo-hooks-native-normalisation — …)
[test] not-exercised …/🗺️plan-step-extraction (no implementation served the requested phase(s) subject)
[test] not-exercised …/📓️session-logging (no implementation served the requested phase(s) subject)
[test] not-exercised …/🛡️tool-blocking-policy (no implementation served the requested phase(s) subject)
[test] level=fundamental cases=5 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=5
```

### 5.1 The Go package's own tests currently fail (not caused by this ticket's work)

The `🧪️test🧰️repo🪝️hooks🐹️go` launch entry was added for symmetry with every neighbour, but it does not pass today:

```
$ cd .../🪝️hooks/📦️packages/🐹️go && GOWORK=C:/git/semio/go.work go test ./...
--- FAIL: TestTrackHookInOpenTicketUsesStableSessionIDs (0.18s)
panic: runtime error: invalid memory address or nil pointer dereference [recovered, repanicked]
[signal 0xc0000005 code=0x0 addr=0x108]
github.com/usalu/semio/repo/tickets.OpenGoal(...)
        .../🎫️tickets/📦️packages/🐹️go/🐹️.go:824 +0x1ae
github.com/usalu/semio/repo/hooks.TestTrackHookInOpenTicketUsesStableSessionIDs(...)
        .../🪝️hooks/📦️packages/🐹️go/🔬️_test.go:125 +0x465
FAIL    github.com/usalu/semio/repo/hooks       0.889s
```

The panic is a nil `RepoContext` reaching `ctx.GoalCreate(input)` at `🎫️tickets/📦️packages/🐹️go/🐹️.go:825`, i.e. inside a *different* module's split package, from a test file (`🔬️_test.go`, 83 `TestXxx` functions) that `go-split` moved. Both files are `go-split` territory and I did not touch either. The whole package panics, so no partial pass count is available.

## 6. What is left

1. **`go-split` must widen `github.com/usalu/semio/repo/hooks`** to the port-isolated API the Rust crate exposes (classification, payload readers, plan merge, `HookOutput` formatter, a session-store interface) before Go adapters can exist. Once that lands, the five `🐹️.go` adapters are mechanical: mirror each `🦀️.rs` scenario's projection key-for-key, since parity is `ordered-json-v1` cross-subject.
2. **`go-split` (or the `🎫️tickets` owner) must fix the nil-`RepoContext` panic** at `🎫️tickets/📦️packages/🐹️go/🐹️.go:825` so `TestTrackHookInOpenTicketUsesStableSessionIDs` and the rest of `🔬️_test.go` can run; until then the `🧪️test🧰️repo🪝️hooks🐹️go` launch entry fails.
3. Repo-wide `contract` breaches (discovery baselines, note-plugin fixture vectors) belong to the audit wave — none is a `🪝️hooks` breach.

## 7. Files touched by this executor

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust/🦀️.rs` (3 clippy fixes)
- `.vscode/🧩️launch.seed.jsonc` (3 new entries)
- `.vscode/launch.json` (regenerated)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated` (regenerated by the same command)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-hooks.md` (this report)

No `🗑️generated/` output was produced; all command output was read from the terminal and is quoted above.
