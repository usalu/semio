# End to End Os Hub Collaboration and Mcp — Coordination Plan

## Objective

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, …).
3. Working collaboration between users over the hub.
4. Working ai integration for users over the semio mcp (`.mcp.json` → `semio`).

## Fleet

- Coordination: main chat (Opus 5.5 High).
- Read-only exploration and audits: Composer 2.5 agents.
- Execution: Grok 4.7 High agents, one disjoint work package each.

## Canonical Ticket

This is session 9 of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` (reopened there; ASCII entry `.tmp-ticket-0918`). Its `📓️status.md`, `📋️g17-acceptance-ledger.md` and `📓️worker-preamble.md` are the authoritative history. Session 8 ended 2026-09-22 ~23:30 with C9, S13, TC4, M10, TS3, PX1, GJ1 in flight. Session-9 worker rules: `📓️session-9-preamble.md`; build locks: `📜️fleet-mutex.sh wasm|hub`.

## Fleet Rules Learned

- Grok agents crash (`[unavailable] Error`) after long shell commands in which emoji paths get corrupted. Prompts must require short commands, emoji paths resolved via globs into variables, and file tools with absolute paths.
- Resumed Grok agents re-crash; restart fresh with the no-emoji-typing protocol: ASCII wildcard globs (`*ShellHost`) and ASCII symlinks under `wp-<id>/links/` edited via file tools.
- Temporary scripts only inside this ticket folder (a `wp-<id>/` subfolder), never `/tmp`. The only repo-root exception is the ASCII entry symlink `.tmp-wp-<id>` → `wp-<id>/`, removed at ticket close.

## Phase 1 — Audits (Composer 2.5, parallel)

| Area | Report |
| --- | --- |
| Os frontend shell | `📓️audit-os-frontend.md` |
| Plugins ✒️writer … 📋️forms | `📓️audit-plugins-a.md` |
| Plugins 📏️layout … 🪵️sourcing | `📓️audit-plugins-b.md` |
| Hub backend | `📓️audit-hub-backend.md` |
| Collaboration path | `📓️audit-collaboration.md` |
| Semio mcp | `📓️audit-semio-mcp.md` |

## Phase 2 — Execution (Grok 4.7 High, parallel)

Work packages are derived from the audit blocker lists and tracked in `📓️work-packages.md`.

## Phase 3 — Verification (Composer 2.5 audits + runtime evidence)

Every requirement is re-audited against current state: builds, tests, running os + hub, two-client collaboration e2e, mcp probe transcript.
