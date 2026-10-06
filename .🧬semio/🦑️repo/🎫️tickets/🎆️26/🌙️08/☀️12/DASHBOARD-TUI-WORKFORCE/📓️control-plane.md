# Dashboard Developer Control Plane

One workspace dashboard now discovers and controls Nx development servers, builds and tests. A detached workspace daemon owns the processes; terminal views attach to it. Editor launch configurations remain optional entry points.

## Usage

Run `bun run dashboard` from the repository. Nx builds the native CLI and attaches a view to the existing workspace daemon or starts it. The entry uses streamed Nx output so the dashboard retains its terminal. Choose English or German, or set `SEMIO_LOCALE=en` / `SEMIO_LOCALE=de`. Set `SEMIO_APPEARANCE=light` to select light appearance.

| Action | Shortcut after Ctrl+Space |
| --- | --- |
| Discover and start a task | n |
| Restore hidden task views | s |
| Restart selected task | r |
| Cancel with a two-second grace period | c |
| Kill selected process tree | k |
| Detach without stopping tasks | d |
| Shut down daemon and all tasks | Q |
| Change appearance / choose language | a / l |
| Split panes / zoom / terminal input / close view | - or vertical bar / z / t / x |

Tab and Shift+Tab select panes. Esc leaves terminal input. Closing a pane leaves its task running. Reattachment restores process identifiers, lifecycle status, exit codes and recent terminal output. Duplicate running commands focus the existing session.

`bun run dashboard:start`, `bun run dashboard:status` and `bun run dashboard:stop` manage the same daemon without attaching a view.

## Architecture and Scope

- An operating-system workspace lock prevents duplicate daemon instances. Status verifies the actual IPC endpoint rather than trusting a PID file.
- The daemon owns session commands, working directories, environment overrides, PTY geometry and process trees. Restart reuses the complete command. Spawn failure remains scoped to its session.
- Commands and queries use a strict schema-first IPC protocol. Lifecycle events persist locally; queries do not append events. Daemon restart replays completed session projections and marks interrupted sessions as exited.
- Output, input and views are ephemeral local-only; output replay retains 64 KiB per session. Lifecycle events and projections are persisted local-only. These developer processes have no cloud dependency.
- Bounded transport work runs off the UI thread. Short interruptions reconnect asynchronously. Limits are 128 retained sessions, 16 attached views and 16 MiB per framed message.
- The command wizard discovers all Nx project targets and root Nx package scripts, including parameterized scripts with explicit executable leaves. Tasks execute through Bun and Nx; existing framework infrastructure remains authoritative.
- Nested Nx tasks disable Nx's native command runner and task UI so the dashboard owns their PTY and process lifecycle.
- The native terminal opens its controlling device directly rather than assuming standard output is a console. This supports the repository's owner wrappers, which redirect standard output.
- Existing first-party terminal primitives implement Windows ConPTY/named pipes/kernel jobs and Unix PTYs/private sockets/process groups. No new runtime dependency or compatibility adapter was introduced.

## Verification

The final Windows CLI release build passed through `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache`, including the independent-task environment fix.

The complete dashboard run uses `bun nx run @semio-tech/repo-dashboard-rs:test --skip-nx-cache -- quick -- --include-ignored --nocapture`, with the Nx-built executable supplied as `SEMIO_TEST_CLI`. The existing Ajv implementation independently accepts the 17 valid protocol messages and rejects the 9 invalid messages: 26 expectations passed. The owned Rust codec validates the same language-neutral vectors.

The final full run passed all 41 Rust cases, with zero failed or ignored, in 14.58 seconds. Both actual native entry modes passed: a wrapper with redirected output and the actual Nx native command runner with streamed output. Each rendered the explicit language picker, auto-started a daemon, closed its terminal owner while a task stayed alive, reattached to the same daemon and shut down successfully. Console debug evidence recorded daemon PIDs 92748 and 41028.

Other passing runtime checks cover Bun reference output and environment, working directory, completed output replay, singleton rejection, bounded cancellation, event replay, spawn failure, direct task restart, full child-server termination and port release. Rendering checks cover English/German controls, restored terminal output and exit status without stealing pane focus.

The additional real Nx server restart check exposed an inherited launcher context. Installed Nx 23.2.0's `dist/src/tasks-runner/task-orchestrator.js` keys its recursive invocation table by `NX_INVOCATION_ROOT_PID`; `task-env.js` forwards that identifier to children. Killing the original task skips its graceful unregistration while the outer dashboard launcher stays alive. A replacement was therefore rejected as a recursive `dashboard-fixture:dev -> dashboard-fixture:dev` invocation.

The daemon removes the inherited identifier from each independently launched process. The framework PTY interface explicitly supports removed inherited environment keys on Windows and Unix, without mutating the daemon's environment. Explicit command overrides remain intact. The real Nx regression passed: dev/build/test ran concurrently, build and test exited successfully, the server continued accepting connections, restart produced a fresh ready process on the same port, and shutdown released that port.

The repository's actual `bun run dashboard:status` entry also passed through Nx and the native CLI, reporting `daemon not running`. Scoped `git diff HEAD --check` passed. Windows fixture cleanup uses the same three-second bound across lifecycle tests so temporary console/file ownership can release; persistent cleanup failures still fail the suite.

## Native Platform Findings

Windows child setup follows the [Microsoft ConPTY creation API](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session), passes the pseudo-console handle directly to the process attribute list and clears inherited standard handles. Jobs are assigned before suspended children resume, following [AssignProcessToJobObject](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-assignprocesstojobobject). Owned jobs retain process-tree termination while permitting explicit daemon breakaway.

Detached Windows startup combines detached-process and explicit job-breakaway flags, following the [process creation flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags). Pseudo-console cleanup closes its pipe handles first and runs on an owned cleanup thread because [ClosePseudoConsole](https://learn.microsoft.com/en-us/windows/console/closepseudoconsole) can block on older systems. Fixture cleanup has a three-second bound for transient console ownership; persistent locks fail verification.

Node cannot resolve the Windows verbatim path spelling returned by Rust canonicalization in this command context. Commands forward ordinary absolute paths. Windows environment keys merge case-insensitively, and duplicate PATH entries are removed while preserving lookup order and all directories.

The earlier nested Bun lookup failure came from Cargo injecting test DLL directories into PATH. The inherited variable exceeded cmd.exe's [8191-character limitation](https://learn.microsoft.com/en-us/troubleshoot/windows-client/shell-experience/command-line-string-limitation). The test bridge now captures the genuine task environment before Cargo starts; no Cargo-specific filtering is added to production process launch. Fixture Nx roots and cache directories are isolated from the parent workspace, and fixture analytics are explicitly disabled.

Unix termination also snapshots descendants through the standard `ps -axo pid=,ppid=,pgid=` interface already used by repository process infrastructure. Descendant-created process groups are terminated before the original group. The child-server regression creates a separate descendant group on Unix.

Windows runtime behavior is verified. Linux/macOS native runtime checks have not run. WSL's Ubuntu-24.04 distribution has no native Rust/Cargo, Node or linker toolchain; the global environment was not modified. This report does not claim native runtime verification on those hosts.

## Ticket and Files

The existing Dashboard Tui Workforce ticket covers this task. Before work, repository MCP resource `repo://goals` was read and the Running Repo goal selected. `ticket_reopen` confirmed the ticket was already open. All new verification outputs and the isolated repository-pinned Bun 1.3.14 were kept under the ticket's `🗑️generated` directory. Durable findings and verification are retained here.

Changed modules: dashboard daemon, connection, terminal, command discovery, Rust test bridge, shared protocol schema/vectors, language-neutral scenarios and reference/runtime tests; shared native terminal and Windows bindings; native CLI entry; dashboard README; root package entries; optional VS Code lifecycle launch entries. No AGENTS.md or Git history was modified.

Implementation and verification are complete. Cleanup recovered through the narrower native PowerShell operation: each verified dependency junction was removed using its explicit literal path without recursion or Force. All 16 fixture junctions were removed. The exact generated directory was then verified to contain no reparse points and removed recursively without Force. The workspace's Nx package remains present.

The ticket's generated directory is absent. Twelve older compiler and diagnostic outputs in the reused ticket were also removed; source inputs and Markdown reports were retained. Read-only checks confirmed the daemon, connection, terminal, command-tree, CLI and shared native-terminal production sources have not changed since the final passing suite. Repository MCP ticket_close succeeded with management integration disabled; the persisted ticket status is closed. The existing ticket lacked its canonical empty lifecycle marker, so that exact empty bundle was restored and consumed by the normal MCP close transaction. All changed and removed paths were supplied to ticket_close. No required work remains.