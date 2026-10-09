# Dashboard Control Plane: Final Verification

Ticket: `26/09/23/DASHBOARD-LAUNCH-COCKPIT`. Verified on native Windows, 2026-10-08.

Start from the workspace with `bun run dashboard`. One workspace daemon owns task identity, process supervision and retained output. TUI views and headless commands attach to that same control plane. Developers can select declared commands, parameters and configurations; start, stop, kill and restart tasks; inspect logs; and reconnect views without duplicating processes. Default prefix is Ctrl+B; `h` shows tasks and `s` restores them.

## Delivered Behavior

The root package has four bootstrap entries: `nx`, `setup`, `dashboard` and `dashboard:install`. Owning Nx manifests and typed dashboard declarations replace polluted editor launch catalogs. Both active VS Code launch files were removed. The complete-source authority scan finds no active VS Code or Claude launch catalog, generator or reader. The Claude launch file was already absent at the beginning of this phase. Authored historical ticket scripts remain preserved inputs; the declared workspace script tool executes them through Bun and Nx with explicit arguments, directory and environment.

The shared TUI repairs cover hover, selection, mouse activation, task-derived titles, cursor-aware text editing, wide characters and combining marks, Windows committed text and Ctrl+Space, window borders and hidden layout. Restore focuses the newest task after asynchronous and paginated snapshots. Prefix feedback remains visible in English and German at 80 columns. HTTP readiness requires a successful response from the announced endpoint; bounded asynchronous probes support cancellation without blocking daemon control.

## Executed Acceptance

| Check | Observed result |
| --- | --- |
| Whole-source bootstrap and launch authority | 6 passed, 0 failed |
| Unsampled independent registry coverage | 9 passed, 0 failed; all 16,384 target IDs matched, including 2,698 nontrivial and 13,686 plain targets |
| Shared UI TUI suite | 367 passed, 0 failed; 1 unrelated test filtered |
| Full native dashboard suite | 213 passed, 0 failed |
| Canonical combined route | 6 authority, 11 control-plane, 8 registry/oracle, 2 CLI-handle and 213 native tests passed; 0 failures |
| Canonical journeys against installed release | 13 CLI/traceability and 16 independent PTY tests passed; 0 failures |
| Canonical load against installed release | 4 CLI and 3 PTY passed; 0 failures; 1 exploratory scenario ignored |
| Real monorepo finite Nx test | 1 smoke passed; underlying execution ran 4 tests successfully |
| Real monorepo HTTP server | 1 smoke passed, 10 assertions; actual quiz service on 6061 returned HTTP 200 at ready, then stopped and released its port |
| Real monorepo native UI build | 1 smoke passed, 10 assertions; cache bypass verified, two generation prerequisites executed, 62 deliverables staged, task exited 0 |
| Stronger absolute caret replay | 1 passed, 0 failed; independent visible-row oracle agreed with hardware cursor, end (5,5), left (5,4), pasted (5,4); task received `!Z QA界` |
| Renamed focused HTTP readiness regression | 1 passed, 0 failed; 212 filtered |
| Scoped whitespace check | Exit 0, no errors; CRLF normalization warnings only |

Independent test oracles include Ajv, Taffy, Unicode segmentation/width libraries and `portable-pty` with `vt100`. They are test dependencies. Production HTTP probing uses the standard library. Neutral feature scenarios and shared vectors describe the implemented contracts.

Load coverage included 128 concurrent sessions, 16 views, a 10.2 MiB output burst and 50,028 searchable commands. Measured TUI key latency was 48/287/38/18 ms against a 1,500 ms budget; list requests during the burst were 195/618/519/230/273 ms. Real server task was `group-cdd0-18dca3b0390e5538-0.0`; real native build task was `group-bc20-18dca529f1dc9064-0.0`. Status and retained logs remained accessible during the real build.

The optimized installed executable has source digest `788db2346cb97131180c5a5ef9fce7c5aef327497e143e872ff84533672c6109`. The first-party freshness assertion reported no stale reason before or after checking. The subsequent edits affected tests only. Harness selection now checks the exact Cargo test target and requires actual nonzero test execution. Captured child output was inspected where Nx omitted inherited stdout; Nx success alone was not used as proof.

## Scope And Practical Limits

Every declared target was resolved against an independent oracle. Representative real server, build and test lifecycles were executed. This does not mean every potentially destructive monorepo command was run. Native Windows and its terminal backend were exercised; native Linux and macOS execution were not available. WSL Ubuntu 24.04 lacked a native Cargo/Node toolchain. Cross-platform paths and declarations remain in the implementation, but no native Linux/macOS runtime pass is claimed.

Cold or queued Nx/native preparation remains expensive: the real UI build smoke took 19m45s, including a 17m51s Nx graph. The daemon stayed controllable during that work. The chronological audit retains earlier failures and their corrections; this table is the final current evidence, not a reinterpretation of those failed runs.

Read-only final checks found all six owned smoke/script instances stopped, including `oct8-real-native-build`. The shared workspace daemon and other developers' processes were left untouched. No Git mutation, worktree, AGENTS edit or repository-goal mutation was performed. Ticket-generated compiler outputs, captures and caches are removed at closure; authored scripts, configurations and Markdown evidence are preserved.
