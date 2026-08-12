# W3 — Daemon IPC + Multiplexed Dashboard Client

## W3a Ipc / Daemon
Added to `📦️glue.rs`:
- `ipc`: length-prefixed frames (`u32 le | u8 kind | payload`), `ClientMsg` / `ServerMsg`, UDS listen/connect (unix), named-pipe path helper (windows), output codec.
- Cache dir: `.🦑️repo/⚡️cache/🎛️dashboard/` (`daemon.sock`, `daemon.pid`, `events.jsonl`).
- `daemon::Supervisor`: attach clients, spawn/resize/kill PTY sessions (unix), tick/read/broadcast, append-only event log.
- CLI: `semio daemon start|serve|stop|status|attach`.

### Tests
- `ipc_frame_roundtrip_and_output_codec`
- `daemon_supervisor_ping_appends_event_log`

## W3b Client
Rewrote `tui_dashboard::run`:
- Leader `Ctrl-Space` then `p` palette / `u` utilities / `-` `|` split / `z` zoom / `d` detach
- `Alt-1..9` and `Alt-h/j/k/l` window focus
- Per-window `TerminalState` panes (piped nx session lines fed as text; local PTY supervisor spawn on launch)
- In-process `LocalBus` supervisor when no external daemon is required for the interactive path

## Verification
```
CARGO_TARGET_DIR=<ticket>/🎯️target-w3 cargo test --lib
```
Result: **13 passed; 0 failed**.
