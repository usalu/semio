# Fast Opinionated Native Dashboard

The new objective supersedes the preceding explicit language-prompt requirement. The default must start immediately without questions; English, dark appearance and native terminology are reasonable defaults, with explicit and persistent customization available. The earlier completion record describes a working control plane, not proof of the new startup and usability requirements.

Repository MCP resource repo://goals was read again before ticket_reopen. The existing Dashboard Tui Workforce ticket was reopened under Running Framework / Running Products / Running Repo; management integration remains disabled. The raw resource and lifecycle diagnostics are ticket-generated outputs. No repository goals were changed.

## Completion Requirements

1. A native Rust executable owns dashboard startup, UI, configuration and process control. Installed startup performs no dependency provisioning, Cargo build, Nx graph computation or source-tree walk before the first usable frame.
2. A measured warm native startup target is below 250 ms to the first usable frame; default developer entry must avoid the current multi-minute graph/build path. First installation/build is an explicit separately observable operation with cancellation.
3. No language or terminology question on ordinary startup or every new task. Defaults work immediately; persistent preferences, environment and explicit flags have documented precedence and strict validation.
4. A simple task overview and searchable command launcher expose dev/build/test and renderer/example choices. Advanced choices remain configurable. Discovery, refresh and connection recovery must not freeze interaction.
5. Full lifecycle control retains start, duplicate focus, output inspection/replay, restart, graceful cancel, kill descendants, detach/reattach and shutdown. Clear task states, failure diagnostics and responsive progress/cancellation remain required.
6. Optional configuration/customization is durable and schema-first: language, appearance, terminology and presentation/launch preferences. User-local and workspace-shared policy must be distinguished; changes use commands/events and bounded atomic publication.
7. Tests use neutral fixtures/scenarios and independent existing implementations. Actual binary first-frame timing and runtime interaction must verify defaults, changes, recovery and lifecycle behavior; configuration or tiny unit tests alone cannot prove the full goal.
8. Existing Bun/Nx authoring, build and test infrastructure remains authoritative. Starting an installed dashboard is a native action; it must not rediscover/build the tool on each launch. Canonical editor launch entries and documented direct invocation must match the resulting launch model.

## Current Evidence

package.json dashboard enters Bun, Nx bootstrap, full project graph generation and CLI build prerequisites. The run target depends on build. The TypeScript launcher hashes and copies the complete executable on every acquisition. The Rust terminal then connects/starts the daemon and performs synchronous recursive command discovery before opening the terminal or presenting its first frame. Language selection is deliberately mandatory whenever SEMIO_LOCALE is absent, and changes are not persisted.

The command tree expands every playground renderer/example across language and terminology axes. This creates repeated paths and asks users questions that should be defaults/settings. The current controls already implement useful process ownership and lifecycle behaviors, so those must be retained while replacing the launch and interaction model.

The previous goal turn completed the earlier runtime task. This goal turn makes progress by reopening the authoritative ticket, inspecting current source and deriving the full new acceptance matrix. New completion remains unproven.

## Implementation and Red Checks

The startup regression was observed through the canonical Bun/Nx test target: the previous terminal implementation failed the new assertion that ordinary startup must not ask for language. An earlier attempt passed the test harness flag through Nx incorrectly and failed before assertions; that command is not counted as a red test. Explicit --args forwarding produced the intended assertion failure.

Preferences now have a language-neutral schema and fixtures, an owned Rust projection and durable command-event journals. Workspace-shared events and local-only events use separate paths. Defaults are English/dark/native/React/tabs; shared events, local events, environment and explicit arguments apply in that order. The existing Ajv independently checks the change vectors and JavaScript independently projects the fixture layers. Operating-system file locks serialize concurrent preference writers without stale lease files.

The terminal now enters before daemon acquisition and recursive command discovery. It immediately presents task controls, starts background discovery/connection, accepts optional settings, and uses a searchable direct launcher. Discovery is cancellable and publishes a compact cache atomically. Language and terminology no longer multiply command paths; effective settings bind the declared launch environment when a command starts. Repo-domain queries run in owned daemon sessions so expensive queries can be cancelled or killed.

Installation owns executable hashing and immutable publication. Ordinary launches read one small installation record, then hand the terminal to the native executable. Package/editor commands keep their existing Bun/Nx spelling, while the bootstrap routes installed-tool execution before provisioning, graph construction and source generation. Repository builds/tests/installation still use Nx; this is a native installed-tool launch, not an alternative build scheduler. Run/daemon/workflow targets have no build prerequisite. First installation is explicit through dashboard:install.

The first complete dashboard test run after the native UI/preferences/inventory change passed 48 Rust tests with 2 binary-dependent tests still ignored, plus independent Bun oracle tests. Later changes to layout/progress/search and installation have not yet been verified by that result. A focused search red check, binary build, actual startup timing, actual interaction, final regression checks and documentation remain required.


## Final Acceptance Review

The preceding turn made concrete progress: it changed startup ownership, preferences, command selection and native input handling, then produced actual runtime evidence. Completion was withheld while the real-terminal checks exposed missing control decoding, Windows code-page ownership, a synchronous seed-catalog cost, a task-selection connection race and harness assumptions. The final implementation moves seed discovery, cache loading, full discovery and command projection to the background. The UI no longer retains a duplicate command tree. Selections made before connection remain in a bounded, cancellable pending-start queue, and output panes obtain their layout before choosing a terminal size.

| Requirement | Current authoritative evidence | Result |
| --- | --- | --- |
| Clean native startup | Installed manifest selects an immutable Rust executable. The bootstrap dispatches installed dashboard commands before provisioning or Nx graph construction. The first-frame path loads only bounded preferences and builds the interactive shell; catalog work starts afterward. Release installation and native front-door tests passed. | Met |
| Fast first usable frame | Actual PTY timings below cover three direct starts, a cold daemon start through the package wrapper, and the real workspace. Every measured native frame is below 250 ms; every final wrapper-to-frame result is below one second. The original package launch took 2m26s. | Met |
| Opinionated defaults | English/dark/native/React/tabs start without language or terminology questions. Shared vectors and actual first-frame assertions verify the behavior. Language/terminology no longer multiply launcher paths. | Met |
| Optional durable customization | Owned schemas, Ajv and independent projection vectors cover preferences and validation. Concurrent event writers serialize revisions with OS locks. The actual settings UI committed German, and subsequent direct/wrapper starts replayed it. Local and workspace-shared journals remain distinct. | Met |
| Direct searchable commands | Neutral multiword/Unicode fixtures verify filtering, selection and mouse viewport mapping. Actual native views searched `build workspace`, ran the selected `dashboard-fixture:build` through Nx, received exit 0 and rendered its real console output. Both the ordinary wrapper and Nx native command runner were exercised. | Met |
| Complete task lifecycle | The full Rust dashboard suite exercised concurrent Nx server/build/test commands, duplicate selection, failure/exit state, output replay, restart, cancellation, descendant termination, singleton ownership and reattachment. Native views detached with a running task and retained the same daemon. Cold package-wrapper startup and real-workspace detach also retained their daemon identity. Earlier real React/WebGPU/native app audits remain in this ticket. | Met |
| Responsive discovery and presentation | Known commands, cached inventory and full discovery run in a cancellable worker. Command projection also runs there. The real workspace accepted settings while discovery was active. Manual discovery refresh/cancellation, layouts, appearance, renderer preference and terminal lifecycle controls remain available. Windows acquires/restores UTF-8 console pages; actual captures show correct box drawing and German labels. | Met |
| Authoritative tooling and registration | Builds/tests/installation use Bun, Nx and the existing bundle scripts. The small ticket-local Nx workspace forwarded to those same owners while the shared graph queue was busy. Installation/preferences entries were added to both launch.json and its comment-preserving seed. No modifying Git command, worktree, AGENTS.md edit or repository goal-management action was used. | Met |

### Validation Scope

Final checks: 53 Rust dashboard tests passed, with the three executable-dependent tests explicitly exercised separately; all 3 native runtime tests passed. Six independent Bun oracle tests passed with 80 expectations. Four Windows terminal cleanup/ownership tests passed. Two installation/argument/publication tests passed with 15 expectations. The final release build installed successfully. Scoped Git whitespace validation passed.

The native runtime suite includes actual command execution and console markers, not only configuration inspection. Its timing reader reconstructs the terminal screen because ConPTY emits incremental ANSI patches. It keeps draining output while waiting for preference commits, avoiding artificial PTY backpressure. The launcher test uses the actual domain label `workspace`, rather than assuming the declared Nx project identifier is the display label.

The final default control key is **Ctrl+B**. ConPTY dropped raw Ctrl+Space in this environment, including a Win32-input experiment. Ctrl+B opens the controls, `p` opens settings, `n` opens commands, `e` cancels discovery and `d` detaches. The generic input parser still decodes NUL correctly as Ctrl+Space; Node readline independently validates the shared key/Unicode projection after normalizing that equivalent control byte.

Startup preference precedence is defaults → shared events → local events → environment → explicit arguments. Interactive changes take effect in the current view; subsequent launches resolve stored events with their own environment/arguments again. Preference journals are bounded to 1 MiB each; the inventory cache is bounded to 16 MiB. Session lifecycle is durable local state, while output and view/input state remain bounded ephemeral local state.

Runtime measurements were made on native Windows. Shared Rust logic and the existing Unix socket/process-group backend remain portable, but this ticket does not claim new native macOS, Linux or devcontainer runtime execution. First installation/build and full workspace discovery remain explicitly expensive operations with progress/cancellation. Ordinary installed startup performs neither operation synchronously.

Authored reproduction inputs and Markdown reports are retained. Tool/compiler/Nx/protocol outputs and dependency junctions are removed from the ticket's generated folder after recording the results. Repository MCP ticket closure follows the final inventory review.


### Final Actual First-Frame Measurements

| Trial | Entry | Workspace | Native frame | Host to frame |
| --- | --- | --- | --- | --- |
| 0 | Native executable | 1,000-project fixture | 2.464 ms | 54 ms |
| 1 | Native executable | 1,000-project fixture | 2.749 ms | 50 ms |
| 2 | Native executable | 1,000-project fixture | 2.217 ms | 43 ms |
| 3 | Bun package wrapper | 1,000-project fixture | 2.434 ms | 227 ms |
| 4 | Bun package wrapper | Actual repository | 18.808 ms | 854 ms |

Installed immutable executable: `.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/bcc69ff5b88dc535b3f47cd2c1fca5855d5215cb4453c9130aa19ab14e2f6b22/semio.exe`.

## Final Cleanup

Generated build, test, and protocol outputs were removed. Four verified ticket-owned node_modules junctions were removed without following them; authored test inputs and the workspace Nx installation were retained. The active workspace dashboard daemon was left available for developers.

Repository MCP ticket_close returned closed for Dashboard Tui Workforce. Its closing scope records 247 authored/source paths; protocol diagnostics were removed after verifying closure.
