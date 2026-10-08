# Round 2 Summary: The Dashboard Is The Only Control Plane

Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`, 2026-10-08. Plan: `r2-plan.md`. Audits: `r2-audit-*.md`. Slice
reports (newest section on top): `r2-slice-*.md`.

## Outcome

- **Launch files dissolved.** `.vscode/launch.json` (4,667 rows), `.vscode/🧩️launch.seed.jsonc` and
  `.claude/launch.json` (79 entries) are deleted; no code, test, gate, config or doc reads them. Every capability
  they offered maps to a dashboard command (`r2-slice-m2.md`, `r2-m2-mapping.json`); the root interactivity gate,
  the devcontainer port law and all tests were re-expressed against the registry (`r2-slice-l1.md`).
- **One entry point.** `bun run dashboard` (TUI) and `bun run dashboard <verb>` ≡ `semio <verb>`:
  `commands`, `run [--param k[=v]] [--env K=V] [--detach --wait-ready] [--dry-run] [-- args]`, `tasks`, `logs`,
  `stop|restart|kill`, `open`, `daemon start|status|stop`, `preferences`, `catalog`. One stable task handle;
  commands and groups are addressable by id. `semio` forwards nothing to `📜️script.ts` any more; `semio dev`,
  `semio workflow`, the root `start` verb, the TypeScript playground resolvers and the wgpu renderer aliases are
  dissolved. Root `package.json` keeps only `nx`, `setup`, `dashboard`, `dashboard:install`; sub-package `dev`
  aliases are gone. Startup rebuilds a stale installed binary automatically (17–30 ms check).
- **Declarative and canonical.** Commands come only from Nx targets, the playground catalog,
  `metadata.semio.dashboard` declarations and open tickets' `🎮️commands.json` (schema-first,
  `🧬️schema/🎮️registry`). MCP client files (`.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`,
  `.codex/config.toml`) are written by `bun ./📜️script.ts agents write` from the root tools declaration and
  drift-checked; devcontainer editor settings derive from `.vscode/` (`generate config --check`); dependabot,
  nextest copies, hooks and stale allow-lists were canonicalised; `🚚️migration.json` removed.
- **Daemon (§3 complete).** Wired ipc/transport/replay, hello handshake with client environment, build id and
  protocol, daemon-side readiness confirmed by a listening port, compounds/requires with stop-together, ring +
  log files with mode-restoring replay, flow control, Windows pipe security, job breakaway fallback, `.exe`
  over `.cmd` resolution, `cmd /s /c` quoting, isolated instances (`SEMIO_DASHBOARD_INSTANCE`), view limits.
- **TUI (§4).** Native resize, waker, cursor, synchronized output, capability detection, UTF-8 paste, focus and
  motion reporting, click counting, keypad keys, escape timeout; correct incremental rendering (property tested),
  hover/capture/drag/wheel/context menus, single focus source, overlays, chrome per the parity audit; full VT
  encoders, scrollback with reflow, selection; grapheme widths in scalar and cluster modes, 50k-row lists, Tree,
  Progress, Toggle; data-driven keymap, en/de catalogue, TaskLabel tabs with status glyphs.
- **Nx graph integrity.** Fixture manifests are excluded (`.nxignore`); the registry check reports graph plugin
  errors, missing playground targets and port conflicts across owners. Real workspace: 26,645 commands,
  0 problems.

## Evidence

- `cargo test -p semio-framework-repo-dashboard --lib`: 206 passed, 0 failed (twice in a row, `r2-slice-a2.md`).
- `cargo test -p semio-framework-ui --features tui-terminal --lib tui::`: 363 passed, 0 failed (`r2-slice-q1a.md`).
- Battle tests (`r2-slice-v1.md`): coverage of the whole monorepo, CLI and PTY journeys with vt100/pyte screen
  oracles, load (128 sessions, 10 MiB burst, 50k launcher), real-workspace smoke on an isolated instance.
- Coordinator cutover on the real workspace: `bun run dashboard commands --check` rebuilt the stale release
  binary (2 m 42 s) and reported 26,564 commands, 0 problems; `bun run dashboard run
  @teaching/architecture-quiz:dev --param steady --detach --wait-ready` printed `http://127.0.0.1:6061`, the page
  rendered without console errors, `daemon stop` ended the task tree and freed the port.

## Owner Actions

1. `AGENTS.md` lines 50–51 still say developers use `launch.json` and never the CLI; agents may not edit it.
   Suggested: "The dashboard (`bun run dashboard`, `semio …`) is the only developer entry point; runnable
   commands are Nx targets, playground catalog entries and `metadata.semio.dashboard` declarations."
2. The three launch-file deletions are unstaged (agents run no git-modifying commands).
3. Kept as declared, please decide: tools `gemini --yolo`, `kiro-cli chat --trust-all-tools`; `Monorepo.sln`.
4. The reviewed-README snapshot fixture still names `🚚️migration.json` (hash-pinned historical snapshot).
5. Out of scope, offered as separate tasks: extension descriptor role contradictions (`verify interactivity
   apps`), zero-touch staging of the OS MCP gateway binary.

## Not Verified

- Unix runtime paths (compiled for `x86_64-unknown-linux-gnu`, not run); live SIGWINCH on a real console.
- Devcontainer image build with the `semio` PATH shim.
