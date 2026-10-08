# R2 Audit: Dashboard Daemon Against Fleet-Plan §3 And Runtime-Audit Defects

Read-only audit, 2026-10-08. Audited HEAD (`2604f70`) plus the working tree; no source file was modified.
Sources: `r2-fleet-brief.md`, `fleet-plan.md` §3, `dashboard-runtime-audit.md` (P0-1..P2-7).

## 0. Path Aliases

| Alias | Path |
| --- | --- |
| `DASH` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard` |
| `DAEMON` | `DASH/🌀️daemon/🦀️.rs` (1133 lines, the only daemon file compiled: `lib.rs` `#[path = "../../🌀️daemon/🦀️.rs"] pub mod daemon`) |
| `ORPH-IPC` | `DASH/🌀️daemon/✉️ipc/🦀️.rs` (459 lines, NOT compiled) |
| `ORPH-TR` | `DASH/🌀️daemon/🚚️transport/🦀️.rs` (596 lines, NOT compiled) |
| `ORPH-RP` | `DASH/🌀️daemon/📼️replay/🦀️.rs` (673 lines, NOT compiled) |
| `CONN` | `DASH/📎️connection/🦀️.rs` (105 lines) |
| `VIEW` | `DASH/🖥️terminal/🦀️.rs` |
| `SCHEMA` | `DASH/🧬️schema/🌀️daemon/🔣️.json` |
| `UI` | `🧰️framework/🔨️modules/🖱️ui/⌨️tui` |
| `PTY` | `UI/🚇️pty/🦀️.rs` (930 lines) |
| `BACK` | `UI/🔌️backend/🦀️.rs` |
| `WIDGET` | `UI/🪀️widget/🦀️.rs` |
| `ANSI` | `UI/🔡️ansi/🦀️.rs` |
| `ENGINE` | `UI/⚙️engine/🦀️.rs` |
| `REG` | `DASH/🎮️registry/🦀️.rs` |
| `TREE` | `DASH/🌳️command-tree/🦀️.rs` |
| `INV` | `DASH/📚️inventory/🦀️.rs` |
| `PREFS` | `DASH/⚙️preferences/🦀️.rs` |
| `WF` | `DASH/🌊️workflow/🦀️.rs` |

Evidence tags: `[code]` read in source, `[build]` observed from cargo output, `[audit]` taken from `dashboard-runtime-audit.md` (not re-run here).

## 1. Summary

- **Test run: 0 tests executed.** `cargo test -p semio-framework-repo-dashboard --lib` fails to compile on this Windows host in the UI crate (`semio-framework-ui`, `PTY:728`, E0277). Reproduced twice. The daemon's 15 unit tests (4 ignored) never ran. Details in §2.
- **§3 daemon contract:** 0 of 12 bullets IMPLEMENTED end to end. 5 PARTIAL (types or fields exist but nothing produces or consumes them), 7 MISSING.
- **Runtime-audit defects (24 ids, P0-1..P2-7):** 0 fully IMPLEMENTED. 2 PARTIAL (P1-4, P1-5). 20 STILL-BROKEN (P2-4 title half included). 2 UNVERIFIED on the sibling-audit axis (P2-4 colour half, P2-5).
- **Structural problem:** the most complete implementation of §3 is not compiled. `ORPH-IPC`, `ORPH-TR` and `ORPH-RP` (ring plus log files, ready detection, poll-based transport, FNV endpoint names, `build_id`) are tracked in git but no `mod` or `#[path]` references them. `DAEMON` carries an older inline duplicate of `ipc` (`DAEMON:10-441`) that diverges from `ORPH-IPC`.
- **Windows:** the PTY backend does not compile (the blocker above); named-pipe server is unverified; no `TERM`/`COLORTERM`; no client environment; Ctrl+Break/terminate stage absent.

## 2. Build And Test Result

Command (per brief, package name verified in `DASH/📦️packages/🦀️rust/Cargo.toml:3`):

`CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-audit-a2 cargo test -p semio-framework-repo-dashboard --lib`

- Result: exit 101, build failed in 8 s, no test binary built, **0 passed / 0 failed / 0 ignored**.
- Blocking error `[build]`:
  `error[E0277]: *mut c_void cannot be sent between threads safely` at `UI/🚇️pty/🦀️.rs:728` (ConPTY output thread, `std::thread::Builder::…spawn(move || …)`).
  Cause `[code]`: `Shared` at `PTY:531-533` has `unsafe impl Send`, but the closure uses `output_read.0`, so Rust 2021 disjoint capture moves the inner `OwnedHandle` (`UI/🪟️windows/🦀️.rs:218`, no `Send` impl) and bypasses the wrapper.
- Same single error appears in the other `r2-audit-*cargo-check*` logs in `🗑️generated/` (written by peer agents), so this is not specific to my run.
- `git status` for `UI` and `DASH` is clean at HEAD, so this is a committed defect, not an uncommitted peer edit. I did not fix it (read-only brief, owned by the A-2 slice per fleet-plan §5).
- Logs: `T/🗑️generated/r2-audit-daemon-cargo-test.log`, `…-retry.log`, `r2-audit-daemon-cargo-time.txt`.
- The daemon unit tests were therefore not run. Anything in §3–§5 that depends on them is marked UNVERIFIED.
- Within the 15-minute budget (failed at 8 s).

Daemon unit tests present (for later run) `[code]` in `DASH/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs`: 15 `#[test]` (4 `#[ignore]`: lines 29, 36, 85, 167). Coverage: protocol vectors (line 10), supervisor across views (370), duplicate spawn/restart/shutdown (421), cancellation bound (448), port release (473), single instance (500), three Windows named-pipe tests (545, 579, 603). **No test covers** hello/handshake, group spawn, replay of >8 KiB, >20 sessions, stalled client, input frames, ready detection, child env, `TERM`, `128+signal`, or flow control.

## 3. Structural Findings

### 3.1 Compiled daemon versus unwired modules

| File | Lines | Referenced by | Contents (verified by reading headers and symbols) |
| --- | --- | --- | --- |
| `DAEMON:10-441` (inline `pub mod ipc`) | 432 | compiled | Protocol 2, frames, `TaskLabel`, `Ready`, `SessionCommand` with `command_id/label/group/ready`, `SessionInfo` with `started_ms/ended_ms/ready_url/title/group`; `socket_path` via `DefaultHasher` + `std::env::temp_dir()` (`DAEMON:239-245`); Windows pipe name via `DefaultHasher` (`DAEMON:359-367`) |
| `ORPH-IPC` | 459 | **none** | Same types with `build_id()` (`ORPH-IPC:320-330`), FNV-1a names (`:277-284`), per-user `/tmp` runtime dir (`:289-300`), `log_dir` (`:272`), `endpoint_path` |
| `ORPH-TR` | 596 | **none** | Poll-based Unix `Stream`/`Listener`/`Poller`/`Waker` over `readiness::wait` (`:45-243`); Windows overlapped named pipe with 1 MiB inbound / 256 KiB outbound queues (`:270-271`), `PIPE_REJECT_REMOTE_CLIENTS` (`:261`) |
| `ORPH-RP` | 673 | **none** | `RING_BYTES` 2 MiB (`:19`), `SEGMENT_BYTES` 8 MiB rotating log files (`:21`), mark-based replay start, mode/title tracker, `READY_HOSTS` and `await_ready` (`:29`, `:200`) |

Lib wiring `[code]`: `DASH/📦️packages/🦀️rust/🦀️.rs:19-20` mounts only `🌀️daemon/🦀️.rs`; `pub use daemon::ipc` at `:50`. A repo-wide `git grep` for the three file names finds no `mod` or `#[path]` reference.

Consequence: §3 features that exist in `ORPH-*` do not run. Wiring them requires deleting the inline `ipc` copy (`DAEMON:10-441`), which is the rule the brief asks for (no duplicate implementations).

### 3.2 Protocol revision and schema drift

Schema `SCHEMA` (protocol 2) against compiled `DAEMON`:

| Schema item | Compiled state |
| --- | --- |
| `hello` with `build_id`, `env` (`SCHEMA:99-111`) | variant exists (`DAEMON:40`), validated (`DAEMON:195-198`), **no handler**: falls to `_ => Err("unsupported control message")` (`DAEMON:753`); the view never sends it (no `Hello` in `VIEW`) |
| `attached` with `protocol`, `build_id` (`SCHEMA:246-257`) | `protocol: 2` sent (`DAEMON:668`); `build_id: String::new()` always (`DAEMON:668`) |
| `watch`, `subscribe`, `unsubscribe`, `forget`, `spawn_group`, `stop_group`, `kill_group` | variants exist; all fall to the unsupported arm (`DAEMON:753`) |
| Input frames, kind 3 (`SCHEMA:8`) | `KIND_INPUT` defined (`DAEMON:21`) but `ClientReader::turn` only handles `KIND_CONTROL` and drops every other kind silently (`DAEMON:491-495`, `:540-544`) |
| `session_removed`, `replay_start` (`SCHEMA:264-287`) | never emitted (no `ServerMsg::SessionRemoved` or `ReplayStart` constructed in `DAEMON`) |
| `sessions.more` paging (`SCHEMA:271-279`) | always `more: false` (`DAEMON:690,701,745`) |
| `error.code` enum (`SCHEMA:299-303`) | always empty (`DAEMON:706`) |
| `status` includes `interrupted` (`SCHEMA:75,80`) | never produced; restart marks running sessions `Exited` with code -1 (`DAEMON:622-627`) |
| `status` includes `pending` (`SCHEMA:75,80`) | never produced (no group code) |
| Undecodable control messages | silently dropped (`DAEMON:492-494`, `:541-543`) |

## 4. Fleet-Plan §3 Verdicts

Legend: IMPLEMENTED (end to end, verified), PARTIAL (types/fields/pieces exist, not connected), MISSING (absent from the compiled path), STILL-BROKEN (defect present). Evidence is `[code]` unless noted.

| # | §3 bullet | Verdict | Evidence |
| --- | --- | --- | --- |
| 1 | `TaskLabel` and `Ready` are wire types in `daemon::ipc`; registry produces them | **PARTIAL** | Wire types: `DAEMON:79-94` (`TaskLabel`, `Ready`). The registry defines its own copies (`REG:21`, `REG:34`), so there is no single home. Registry produces them (`REG:545-557` `LaunchProcess.label/ready`), but the view's conversion drops both: `TREE:62` builds `CommandSpec { cmd, args, cwd, env }` only. Third copy in `ORPH-IPC:76,88`. |
| 2 | `SessionCommand` gains `command_id`, `label`, `group`, `ready` | **PARTIAL** | Fields present and serialized (`DAEMON:116-126`; `SCHEMA:55-72`). Producer ignores them: `VIEW:236` `SessionCommand { …, ..Default::default() }`. Daemon never reads `command_id`, `label`, `group` or `ready` outside the struct; `SessionCommand::validate` checks only cmd, cols, rows, cwd, args and env (`DAEMON:170-177`). |
| 3 | `SessionInfo` gains `started_ms`, `ended_ms`, `ready_url`, `title`, `group` | **PARTIAL** | Fields exist (`DAEMON:147-159`). Daemon never sets them: `spawn_session` uses `..Default::default()` (`DAEMON:776`); no code writes `ended_ms`, `ready_url`, `title` (OSC 0/2 parsing absent from `DAEMON`), `group`. `started_ms` stays 0. |
| 4 | Ready detection runs in the daemon; views and `--wait-ready` consume `SessionChanged` | **MISSING** | No URL scan in `DAEMON`, `VIEW` or `PTY`. Only `ORPH-RP:29,195-215` has it (unwired). `--wait-ready` does not exist (no match in `DASH` or `⌨️cli`). |
| 5 | Compounds and `requires` orchestrated by daemon (`SpawnGroup`), incl. stop-together | **MISSING** | `SpawnGroup` validated (`DAEMON:189-194`) but unhandled (`DAEMON:753`). `StopGroup`/`KillGroup` likewise. `Launch.requires/group/stop` (`REG:559-566`) are discarded by `TREE:62` and `VIEW:429` spawns each compound member as an independent session into one window. |
| 6 | Bounded ring per session + per-session log file for scrollback and `semio logs`; replay restores full screen and scrollback | **PARTIAL** | Ring: 64 KiB `VecDeque` (`DAEMON:643-644`). No log file is ever written (`log_dir` only in `ORPH-IPC:272`; `ORPH-RP` unwired). Replay is raw tail chunks of 4 KiB sent on attach with no `ReplayStart` (`DAEMON:689-699`), so no screen or mode restoration. `semio logs` absent (see §6). |
| 7 | Flow control instead of disconnect on backlog | **MISSING (STILL-BROKEN)** | Disconnects: `DAEMON:652` (`try_send` failure removes client via `retain`), `DAEMON:663` (backlog error), `DAEMON:674` (writer exits on write error), `DAEMON:696` (replay `send` error propagates to `handle_for`), and `serve` detaches a client on any `handle_for` error (`DAEMON:981`). Client side: `CONN:74` backlog error → `Disconnected`, `CONN:88` `try_send` on 64 slots. |
| 8 | PTY writes are queued and retried | **MISSING** | `input` calls `pty.write_all` synchronously on the single daemon thread (`DAEMON:791-792`), which waits up to 10 s for the terminal to accept (`PTY:290-307`). No queue on Unix. Windows has a 256 KiB ConPTY input backlog (`PTY:814-823`) but the same blocking `write_all` (`PTY:826-843`, 5 ms sleep loop). `stop` also writes ^C synchronously (`DAEMON:819`). |
| 9 | Child gets `TERM=xterm-256color`, `COLORTERM=truecolor` | **MISSING** | No occurrence of `TERM=` / `COLORTERM` in `DASH` or `UI` source (only `SIGTERM` identifiers). |
| 10 | Child environment is the requesting client's, not the daemon's | **MISSING** | Client never sends `hello` (`VIEW` contains only `Attach`, `Spawn`, `Input`, `Resize`, `Stop`, `Kill`, `Restart`, `Shutdown`, `Detach`). Daemon has no `hello` handler (`DAEMON:753`). Spawn: `Pty::spawn` inherits the daemon environment and removes only `NX_INVOCATION_ROOT_PID` (`DAEMON:774`, `PTY:156-162`). `Pty::spawn_exact` exists (`PTY:166`; Windows `PTY:622`) but is unused. |
| 11 | Exit reports signal deaths as `128 + signal` | **PARTIAL** | Unix natural death: `PTY:408-410` maps signal to `128+sig` and the tick reports it (`DAEMON:859-873`). Windows: console-Ctrl exit mapped to 130 (`PTY:861`). Daemon-initiated kill and stop report `-1` (`DAEMON:837`, `DAEMON:867`), not 137/143/130. |
| 12 | Views and daemon exchange build id and protocol version on attach; mismatch shown, never silent | **MISSING** | Daemon sends `protocol: 2` and empty `build_id` (`DAEMON:668`). Client ignores both (`CONN:35` `let ServerMsg::Attached { daemon_pid, .. }`); no mismatch UI. `build_id()` exists only in `ORPH-IPC:320`. Installed binary is pinned by hash-named directory (`ORPH-IPC:318-330`) but nothing compares it. |

Totals: IMPLEMENTED 0, PARTIAL 5 (#1, #2, #3, #6, #11), MISSING 7 (#4, #5, #7, #8, #9, #10, #12). Compiled daemon emits 7 of 9 `ServerMsg` variants in the schema's sense (no `session_removed`, `replay_start`); see §3.2.

## 5. Runtime-Audit Defects (P0, P1, P2)

Each row: current verdict and evidence. Audit line numbers are stale (the daemon was split after the audit); evidence is re-cited.

### P0 (blocking)

| ID | Verdict | Evidence |
| --- | --- | --- |
| P0-1 transport: non-blocking writer kills client on first `WouldBlock`; sessions list / replay lost; second view flaps | **STILL-BROKEN** | `DAEMON:959` `set_nonblocking(true)` then `DAEMON:960` `try_clone()`: duplicated fds share `O_NONBLOCK`; writer thread `DAEMON:672-676` breaks on `write_all` error. Client removed at next broadcast (`DAEMON:651-653`). Attach replay pushes up to 16 frames per session through `try_send` into a 4096-slot channel (`DAEMON:671`, `:689-699`). Same mechanism as audit D-1. Fix exists only in unwired `ORPH-TR`. |
| P0-2 resize: no SIGWINCH / size poll; PTY sizes frozen | **STILL-BROKEN** | Native wait is `select` on the tty fd only (`BACK:422-451`); no SIGWINCH handler, no `Event::Resize` produced. `VIEW:635-637` resize arm is dead on native. `resize_terminals` runs only on `ReplayComplete` (`VIEW:513`) and that dead arm (`VIEW:528-539`). Daemon `resize` (`DAEMON:801-816`) works but is never driven by a real size change. `Pty::refresh` (`PTY:366-371`) has no caller. |
| P0-3 interactive tasks: `/`, Tab, Esc, paste, PageUp/Down, bursts | **STILL-BROKEN** | Tab consumed before terminal input (`VIEW:836`). Esc drops terminal focus (`VIEW:854-856`). `WIDGET:363-383` consumes PageUp, PageDown, End, Ctrl+Home, `/` and Ctrl+P. Keys become one `ClientMsg::Input` JSON control message each (`VIEW:867`) and `CONN:38,88` send through a 64-slot `try_send`. Paste: the `Event::Paste` arm was not re-read in this pass (UNVERIFIED). |
| P0-4 task output ~75 KB/s | **STILL-BROKEN** | Daemon reads at most 16 chunks of 4 KiB per session per tick (`DAEMON:856-862`), then `sleep(10 ms)` (`DAEMON:984`, `:1044`). No readiness wait in `DAEMON`. `readiness::wait` exists (`PTY:91-126`) and `ORPH-TR` uses it, but unwired. |
| P0-5 task identity: tabs are raw `cmd args`; no label/verb/source in protocol | **STILL-BROKEN** | Tab title `format!("{} {}", cmd, args)` (`VIEW:251`), status-prefixed titles (`VIEW:335`), Overview rows `cmd args` (`VIEW:101`). Protocol fields exist (see §4 #1–#3) but are not populated; registry label dropped at `TREE:62`. |
| P0-6 compounds: one window, leaked terminal, member 1 detached | **STILL-BROKEN** | `VIEW:429` loops `spawn_output` over members into the same `win_id`; each call creates a new terminal node (`open_output`, `VIEW:208-226`) and rebinds the window. Daemon groups unsupported (`DAEMON:753`). |

### P1

| ID | Verdict | Evidence |
| --- | --- | --- |
| P1-1 mouse: hover, wheel, drag, selection, focus-follows-click, open on double-click | **STILL-BROKEN** | `ENGINE:155` forwards only `MouseKind::Down(MouseButton::Left)` to widgets; `ANSI:63` enables `?1002` (drag) but not `?1003` (hover) or `?1004`; `VIEW:640-641` dispatches all mouse events but the engine drops everything except left-down. Click-to-run path via `WIZARD` not re-traced (UNVERIFIED runtime). |
| P1-2 terminal not restored on SIGTERM / SIGHUP / SIGINT | **STILL-BROKEN** | Restore only in `Drop` (`BACK:487`, `:727`). `ISIG` cleared (`BACK:377`). `interrupts()` installs SIGINT/SIGTERM/SIGHUP counters (`PTY:66-85`) but has no caller anywhere in `DASH` or `UI`, so it does nothing. |
| P1-3 task env = daemon env (stale PATH, NX_*) | **STILL-BROKEN** | See §4 #9–#10: `DAEMON:774`, `PTY:156-162`; `spawn_exact` (`PTY:166`) unused. |
| P1-4 signal deaths shown as -1; no SIGTERM stage; kill forks `ps` on daemon thread | **PARTIAL** | Fixed: signal ladder with `Interrupt`, `Terminate`, `Kill` (`PTY:331-363`) and `128+sig` (`PTY:408-410`); process table read in-process via `/proc` or `proc_listallpids` (`PTY:412-444`), no `ps` fork. Still open: daemon never uses `Terminate` (`DAEMON:833` `terminate()` = SIGKILL; `DAEMON:875` SIGKILL after 2 s stop; `DAEMON:837` code -1); the process-table scan runs synchronously on the daemon thread (`PTY:386-405`, D-5). |
| P1-5 startup uses cached catalog; fast clean writes; no tmp residue; graph reuse | **PARTIAL** | Fixed: snapshot gate raised to 64 MiB (`INV:16`, `INV:57`); writer buffered at 1 MiB (`INV:69`); stale `commands.*.tmp` swept (`INV:78-84`). Not re-verified at runtime: graph reuse remains mtime-based (`REG` `graph_is_current`, `INV:450`). |
| P1-6 Overview stability, focus after close, `q` quits mid-typing | **STILL-BROKEN** | Close focuses `order[0]` (`VIEW:398`). `q` quits outside terminal input and outside launcher (`VIEW:823-824`), which includes Overview and Settings with typing filters. `Ctrl+B h` path (V-2) not re-traced. |
| P1-7 damage-only repaint | **STILL-BROKEN** | Every paint is `term.present(&tui.render_full(), None)` (`VIEW:907`); extra `render_full` call on every terminal open (`VIEW:214`, inside `open_output`). |
| P1-8 page/home/end, caret, line editing, paste in filter, match count | **STILL-BROKEN** | `WIZ` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧙️Wizard/🎯️targets/⌨️tui/🦀️.rs:33-64`) handles only Up, Down, Enter, Esc, Backspace and printable characters. |
| P1-9 launcher: ticket leaves mutate without confirmation; 31 % ticket leaves | **STILL-BROKEN** | `mutating` flag is computed (`REG:535-536`, `REG:951-953`) but `VIEW` never checks it (no `mutating` reference in `VIEW`). Close/reopen actions are still generated (`TREE:171-174`) and listed for open tickets, so one Enter still closes or reopens. Share of leaves not re-measured. |
| P1-10 build id, handshake, undecodable messages dropped silently | **STILL-BROKEN** | See §4 #12; silent drops at `DAEMON:492-494`, `:541-543`; `Error.code` always empty (`DAEMON:706`). |
| P1-11 full i18n; English literals; hints/status truncated at 80 columns | **STILL-BROKEN** | Two-language helper `VIEW:74`; hard-coded English `connected`, `reconnecting`, `daemon stopped` at `VIEW:476`, `:486`, `:521`. Hint truncation not re-measured. |

### P2

| ID | Verdict | Evidence |
| --- | --- | --- |
| P2-1 preferences journal can refuse startup (torn line, lock contention, 1 MiB cap) | **STILL-BROKEN** | 1 MiB cap (`PREFS:8`, `:80`, `:112`); no compaction; torn line (no trailing newline) is an error (`PREFS:83`); `load` error exits 2 (`VIEW:606`). |
| P2-2 `workflow run` deadlock, ignores concurrency; registry check validates wrong file; `dev` vs dashboard lock defaults | **STILL-BROKEN** | `WF:45-61`: serial batch loop with `child.wait()` (`WF:50`) despite `concurrency`; agents spawned with `Stdio::piped()` never read (`WF:254,268,282`). `PREG:31` validates `🎠️playgrounds.json`; loader reads `🚀️playgrounds.json` (`REG:873`). `PSESS:44-48` defaults `Lock::All`; `TREE:55` presentation values are fixed. |
| P2-3 stale dirs/sockets in `$TMPDIR`; non-portable hasher; events journal never compacted, fsync per event; eviction by key order | **STILL-BROKEN** | `DAEMON:239-245` `temp_dir()` + `DefaultHasher`; Windows `DAEMON:359-367` `DefaultHasher`; fsync per event `DAEMON:567`, no compaction (`DAEMON:551-568`); eviction by first key in `BTreeMap` order `DAEMON:578-581` and `DAEMON:764-767`; `remove_file(socket)` only, directory never removed (`DAEMON:986-987`, `:1049-1050`). Fixed only in `ORPH-IPC:277-300` (FNV, per-user runtime dir), unwired. |
| P2-4 colour fidelity (task background `#000000`); OSC 0/2 titles ignored | **UNVERIFIED (colour); STILL-BROKEN (title)** | Colour is sibling VT/blit audit (not re-run). Title: no OSC capture in `DAEMON`, so `SessionInfo.title` is never set. `PTY`/`VT` parse OSC (`📟️vt/🦀️.rs:282`) but nothing forwards it. |
| P2-5 zoom leaves overlapped tab text | **UNVERIFIED** | Sibling chrome audit; not re-run. |
| P2-6 Esc then Ctrl+B parsed as one chord; no literal Ctrl+B; leader has no timeout | **STILL-BROKEN** | `LeaderMode::Armed` has no timestamp (`VIEW:34-36`); no timeout branch in the loop. Esc/Ctrl+B timing not re-measured. |
| P2-7 pending-start limit after window creation; orphan windows; reconnect thread per 500 ms | **STILL-BROKEN** | `open_output` runs first (`VIEW:251`), the `pending_starts >= 128` check follows (`VIEW:259`). Reconnect spawns a thread every 500 ms (`VIEW:453-457`). |

### Cross-references to audit D- and M- ids

| Audit id | Verdict |
| --- | --- |
| D-1 (non-blocking writer) | STILL-BROKEN, see P0-1 |
| D-2 (75 KB/s) | STILL-BROKEN, see P0-4 |
| D-3, D-4 (reattach empty, flapping) | STILL-BROKEN, see P0-1 |
| D-5 (kill on daemon thread) | STILL-BROKEN: `PTY:386-405` synchronous table scan inside `signal` |
| D-6 (restart error injected into focused terminal) | UNVERIFIED (not re-traced) |
| D-7 (no backpressure policy) | STILL-BROKEN, see P0-1 |
| M-1 (no task identity) | STILL-BROKEN, see P0-5 |
| M-2 (no start/end times, exit notification, log file, group lifecycle) | STILL-BROKEN, see §4 #3, #5, #6 |
| M-3 (env not inspectable) | STILL-BROKEN, see §4 #10 |

## 6. Missing Surface (Fleet-Plan §2.4 And Round-2 Scope)

- CLI verbs `semio run`, `tasks`, `logs`, `stop`, `restart`, `kill`, `open`, `--wait-ready`, `--detach`: absent. `⌨️cli` dispatch (`DASH/📦️packages/🦀️rust/🦀️.rs:55-70`) has only `dashboard`, `preferences`, `repo-view`, `daemon`, `workflow`, `dev`, `catalog`, `command-tree`, `commands`, `plugin registry`. Unknown verbs go to root delegation (`DASH/📜️root-delegation`), so `semio run …` would be delegated to `bun ./📜️script.ts`, bypassing the daemon.
- `daemon` subcommands present: `start`, `serve`, `stop`, `status`, `attach` only (`DAEMON:1096-1126`).

## 7. Windows-Specific Gaps

1. **Build blocker (compile):** `PTY:728` ConPTY output thread (see §2). Until fixed, nothing in the dashboard daemon can be compiled or tested on Windows.
2. **Named-pipe server, compiled path** (`DAEMON:416-440`): synchronous (non-overlapped) pipe; `CreateNamedPipeW` called with null security attributes (`DAEMON:428`), so the default DACL applies and no per-user restriction is set; no `PIPE_REJECT_REMOTE_CLIENTS` (the flag exists only in `ORPH-TR:261`); no `FILE_FLAG_FIRST_PIPE_INSTANCE`. Remote-client rejection and ACL: UNVERIFIED on host (no run).
3. **Reader/writer on one synchronous pipe** `[code]`: the client-writer thread (`DAEMON:672-676`) blocks in `WriteFile` on a duplicated handle while the main loop uses `PeekNamedPipe` and `ReadFile` on the same file object (`DAEMON:393-404`, `:518-546`). The comment at `DAEMON:393-397` describes the serialization hazard. `ORPH-TR:246-300` used overlapped I/O to avoid it. Behaviour under a stalled client: UNVERIFIED (not run).
4. **Pipe name stability:** `pipe_name` uses `DefaultHasher` (`DAEMON:359-367`), so a view and a daemon built by different toolchains can disagree. The recorded endpoint file written by `listen` (`DAEMON:406-414`) is never read by `connect` (`DAEMON:370-372` recomputes the name).
5. **Job objects (positive):** created with `KILL_ON_JOB_CLOSE` (0x2000) and assigned before `ResumeThread` (`PTY:678-713`). Note the same limit word also sets `0x0800`, which in the Win32 `JOB_OBJECT_LIMIT_*` table is `BREAKAWAY_OK` (child may break out of the job). Check against the SDK header on the host (UNVERIFIED here).
6. **ConPTY (positive):** `CreatePseudoConsole` with pipes and three worker threads, output bounded at 256 KiB (`PTY:722-780`, `:741`). Gaps: `Terminate` stage reaches nothing on Windows (`PTY:869-870` comment), so stop is ^C then job kill. `write_all` blocks the daemon thread up to 10 s (`PTY:826-843`).
7. **Environment:** Windows env block merges, upper-cases keys and dedups `PATH` (`PTY:586-612`), but no `TERM`/`COLORTERM` and no client environment (see §4 #9–#10).
8. **Signals:** Windows console-control handler is a counter only (`PTY:499-510`), no caller. No SIGTERM-equivalent. Exit code 130 mapping only (`PTY:861`).

## 8. Items Not Verified (UNVERIFIED)

- All daemon unit tests (build blocker, §2).
- Runtime behaviour of every P0/P1/P2 item: no PTY or daemon run was performed. Verdicts above are source-based; `[audit]` evidence is quoted from the earlier runtime capture where cited.
- P1-1 click activation path through `WIZ` (engine path read only as far as `ENGINE:155`).
- P1-9 ticket-leaf share and close/reopen behaviour at runtime.
- P2-4 colour and P2-5 zoom (sibling audits).
- Whether `ORPH-IPC`, `ORPH-TR` and `ORPH-RP` compile. They are not in the module tree, so cargo never checks them.
- Windows named-pipe behaviour and ConPTY behaviour beyond the compile error.

## 9. Recommended Next Steps (For Coordinator)

1. Fix the blocker at `PTY:728` (move the whole `Shared` wrapper into the closure, e.g. `let output_read = output_read;` before `spawn`, or take the handle through a method) so `semio-framework-ui` compiles on Windows, then re-run the dashboard tests.
2. Decide the transport: wire `ORPH-IPC`, `ORPH-TR` and `ORPH-RP` into `DAEMON` (delete the inline `ipc` copy at `DAEMON:10-441`), or delete them. Keeping both is a duplicate implementation.
3. Make the view send `hello` with `build_id` and client env, and make the daemon answer with real `build_id` and `protocol` (§4 #10, #12).
4. Route `SpawnGroup`/`StopGroup`/`KillGroup` and `Launch.requires/group/stop` through the daemon (§4 #5), and stop dropping `label`/`ready`/`group` at `TREE:62` and `VIEW:236`.
5. Add the tests fleet-plan §3 implies but that do not exist (§2 list), starting with: stalled client plus 200 retained sessions, replay of more than 8 KiB, hello/env, ready detection, group start/stop, `128+signal` exit.

Report path: `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️09\☀️23\DASHBOARD-LAUNCH-COCKPIT\r2-audit-daemon.md`
