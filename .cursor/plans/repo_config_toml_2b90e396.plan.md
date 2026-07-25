---
name: Repo Config Toml
overview: Introduce a `.repo/config.toml` read by the repo CLI to make hook logging configurable (what is logged + detail level), defaulting session-file logging to off.
todos:
 - id: config-loader
   content: "Add ⚙️RepoConfig region: RepoConfig/LoggingConfig structs, DefaultRepoConfig, LoadRepoConfig (hand-parsed TOML from .repo/config.toml), detail helpers."
   status: completed
 - id: gate-artifacts
   content: Gate writeHookArtifacts on cfg.Logging.Session (default off); thread config into writers and operations call.
   status: completed
 - id: writers-detail
   content: Parameterize writeSessionHookLog and logRepoOperationHook with LoggingConfig; apply detail level, operations, and plan gates.
   status: completed
 - id: config-file
   content: Create documented default .repo/config.toml (session=false) and add Configuration section to repo/README.md.
   status: completed
 - id: tests
   content: "Extend main_test.go: config helper, new TestRepoConfig, update session-asserting tests, run focused go tests and confirm pass."
   status: completed
 - id: ticket
   content: Read repo://goals, open repo-cli ticket before editing, close with summary + files after.
   status: completed
isProject: false
---

## Repo Config Toml

### Goal

Add a `.repo/config.toml` that the repo CLI reads to control hook logging. Make configurable: whether session-file logging happens, whether derived repo-operation events and plan tracking are written, and the detail level of each event. Disable session-file logging by default (off when no config file or no override present).

### Where logging happens today

All session-file logging is in the repo CLI (`repo/client/cli/main.go`), driven by hook events:

- [repo/client/cli/main.go](repo/client/cli/main.go) `writeHookArtifacts` (~line 43660): the single entry that builds the `.repo/⚡/🤖/YY/MM/DD/<session>/` dir and calls the writers.
- `writeSessionHookLog` (~line 43694): writes `session.json` events, attaches raw `native.event` input and `response` block, and tracks plan steps.
- `logRepoOperationHook` (~line 44212): appends derived `agent.<operation>.<phase>` events into `session.json`.

There is no TOML decode library in [repo/client/cli/go.mod](repo/client/cli/go.mod); the codebase already hand-parses TOML (`dependencyBoundaryMergeCargoToml`, ~line 20489), so the config will be parsed the same way (no new dependency, consistent with `AGENTS.md`).

### Config format (`.repo/config.toml`)

```toml
[logging]
# Write per-session session.json hook logs under .repo/⚡/🤖/...
session = false
# Include derived agent.<operation>.<phase> events (only applies when session = true)
operations = true
# Track agent plan steps in session.json (only applies when session = true)
plan = true
# Detail per event: "minimal" (event only) | "standard" (+ response) | "full" (+ raw native input)
detail = "standard"
```

### Implementation

1. Add a `// #region ⚙️RepoConfig` to [repo/client/cli/main.go](repo/client/cli/main.go) with:
   - `type LoggingConfig struct { Session bool; Operations bool; Plan bool; Detail string }`
   - `type RepoConfig struct { Logging LoggingConfig }`
   - `func DefaultRepoConfig() RepoConfig` -> `Session:false, Operations:true, Plan:true, Detail:"standard"`.
   - `func LoadRepoConfig(repoRoot string) RepoConfig`: reads `filepath.Join(repoRoot, ".repo", "config.toml")` via `ReadTextFile`; returns defaults if missing; hand-parses `[table]` headers + `key = value` (bool/string), mirroring `dependencyBoundaryMergeCargoToml`. Unknown keys ignored.
   - Detail helpers: `includeResponse()` (standard/full), `includeNative()` (full only).

2. Gate the production path in `writeHookArtifacts`:
   - After the existing `HookKindVersion` early-return, load `cfg := LoadRepoConfig(repoRoot)`.
   - If `!cfg.Logging.Session`: return before creating the log dir (nothing written) — this is the new default-off behavior.
   - Pass `cfg.Logging` to `writeSessionHookLog`; only call `logRepoOperationHook` when `cfg.Logging.Operations`.

3. Thread config into the writers:
   - `writeSessionHookLog(ctx, result, logDir, sessionID, lg LoggingConfig)`: attach `entry.Native` only when `lg.includeNative()`; attach `entry.Response` only when `lg.includeResponse()`; gate plan-step tracking behind `lg.Plan`.
   - `logRepoOperationHook(...)`: add a `lg LoggingConfig` param; skip when `!lg.Operations`.

4. Create the documented default file `.repo/config.toml` with the format above (`session = false`), so the option is discoverable. Behavior is identical whether the file is absent or present with `session = false`.

5. Document the config in [repo/README.md](repo/README.md) under a short "Configuration" section.

### Tests (extend [repo/client/cli/main_test.go](repo/client/cli/main_test.go), no new files)

- Add helper `writeRepoLoggingConfig(t, root string, lg LoggingConfig)` that writes `<root>/.repo/config.toml`.
- New `TestRepoConfig` covering: defaults when file absent; parse of all keys; default-off => `RunHook` writes no `.repo/⚡` files; `detail` levels control presence of `native`/`response`; `operations=false` suppresses derived events; `plan=false` suppresses plan tracking.
- Update existing session-asserting tests (e.g. `TestSessionJsonTracksPlan`, the agent-event/session and `logRepoOperationHook` tests around lines 16560-17560) to enable logging via the helper, and pass `LoggingConfig` to the now-parameterized `writeSessionHookLog`/`logRepoOperationHook` direct calls. Tests asserting raw `native` use `detail = "full"`.
- Run `nx`/`go test ./repo/cli` focused hook + config tests and confirm pass (do not claim pass without running).

### Workflow

Per repo rules: read `repo://goals`, open a ticket (e.g. `MAKE-REPO-LOGGING-CONFIGURABLE`) associated with the repo-cli goal before editing, and close it with a summary + touched files when done.

### Out of scope

Coordinator/server (`repo/server/...`) logging — config struct is structured to extend later, but only CLI hook logging is wired now.
