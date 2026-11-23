# C: Drive Audit

Read-only audit of what fills C: (785 GB total, about 15 GB free at start). Nothing was deleted, moved or modified. Raw measurements: `C:\Users\Ueli\AppData\Local\Temp\claude\c1f0d791bc008866\63d92c99-4bf1-4b98-aac5-244dadcd1b5d\scratchpad\sizes.tsv` (825 targets, byte sums and newest file mtime, scanned 2026-11-22 about 17:30 to 17:40).

## Headline

- Used space is roughly 770 GB. Accounted for: Users 217 GB, git 134 GB, semio 123 GB, t 91 GB, Program Files 51 GB, Windows 44 GB, semio-merge leftover 32 GB, ProgramData 21 GB, Program Files (x86) 14 GB, pagefile 14 GB, hiberfil 27 GB.
- The owner's belief is partly right. Repo-tree caches and build output are the largest reclaimable block: `C:\t` (91 GB), `C:\semio\.cache` (56 GB), `C:\git\semio\.🧬semio\🎓️teaching\⚡️cache` (63 GB) and the ticket tree under `C:\semio` (52 GB, mostly generated output).
- Temp files are not the main culprit. `AppData\Local\Temp` is 21 GB, of which 19 GB is this Claude session's own scratch (`Temp\claude`, written today).
- Largest single stale candidate: `C:\semio-merge-20261113-2107` (32 GB, last write 2026-11-15).

## Top 40 items (largest first)

Last write = newest file mtime found in the tree. Some mtimes are in the future (2036-11-16 on `C:\semio\.cache`); the clock or file dates are unreliable there, so treat those as unknown.

| # | Size | Last write | Path | Verdict | Owner / note |
|---|------|-----------|------|---------|--------------|
| 1 | 217.2 GB | 2026-11-22 | C:\Users | live | All user profiles (sub-items below) |
| 2 | 161.6 GB | 2026-11-22 | C:\Users\Ueli\AppData | live | Local, Roaming, Temp (sub-items below) |
| 3 | 134.1 GB | 2026-11-22 | C:\git | live | Clones and repo trees (sub-items below) |
| 4 | 123.2 GB | 2036-11-16 (bogus) | C:\semio | live (mixed) | Live fleet tree; `.cache` and ticket tree dominate |
| 5 | 91.5 GB | 2026-11-22 | C:\t | live, build output | FEM agents' build dirs (see below) |
| 6 | 66.1 GB | 2026-11-15 | C:\git\semio\.🧬semio\🎓️teaching | likely stale | Teaching cache and outputs, last write 7 days ago |
| 7 | 62.8 GB | 2026-11-15 | C:\git\semio\.🧬semio\🎓️teaching\⚡️cache | likely stale | Teaching render cache; safe to regenerate |
| 8 | 56.6 GB | 2026-11-22 | C:\t\2d-ab | likely live (today) | FEM 2D build dir of an active agent |
| 9 | 56.5 GB | 2036-11-16 (bogus) | C:\semio\.cache | mixed | Shared cache, see items 14 to 16 |
| 10 | 56.0 GB | 2036-11-16 (bogus) | C:\semio\.cache\b | mixed | Cache lane `b`, see items 14 to 16 |
| 11 | 52.5 GB | 2026-11-22 | C:\semio\.🧬semio\🦑️repo | live | Repo state |
| 12 | 52.5 GB | 2026-11-22 | C:\semio\.🧬semio | live | Repo state root |
| 13 | 51.8 GB | 2026-11-22 | C:\semio\.🧬semio\🦑️repo\🎫️tickets | live | Ticket tree, mostly generated output in ticket folders; largest candidate for pruning of `🗑️generated` |
| 14 | 30.5 GB | 2026-11-22 | C:\semio\.cache\b\d | live (today) | Cache lane `d`, written today by a running job |
| 15 | 13.5 GB | 2026-11-22 | C:\semio\.cache\b\r | live (today) | Cache lane `r`, written today |
| 16 | 6.0 GB | 2026-11-22 | C:\semio\.cache\b\g | live (today) | Cache lane `g`, written today |
| 17 | 36.5 GB | 2026-11-22 | C:\t\3db | likely live (today) | FEM 3D build dir |
| 18 | 32.0 GB | 2026-11-15 | C:\semio-merge-20261113-2107 | likely stale | Leftover from 2026-11-13 merge, 7+ days old; owner Ueli (merge work) |
| 19 | 31.5 GB | 2026-11-22 | C:\git\semio\.git | live | Git object store of the clone; not to touch |
| 20 | 27.5 GB | system | C:\hiberfil.sys | system | Hibernation file; only removable with hibernation off (a system setting, not changed here) |
| 21 | 25.2 GB | 2026-11-21 | C:\Users\Ueli\AppData\Local\Docker | likely live | Docker Desktop data (VHDX); check with `docker system df` |
| 22 | 21.2 GB | 2026-11-22 | C:\ProgramData | mixed | Packages and caches of installed software |
| 23 | 21.2 GB | 2026-11-22 | C:\Users\Ueli\AppData\Local\Temp | live | Temp root, mostly the session scratch below |
| 24 | 19.0 GB | 2026-11-22 | C:\Users\Ueli\AppData\Local\Temp\claude | live | Claude Code session scratch and tool results |
| 25 | 15.9 GB | 2025-03-05 | C:\Users\Ueli\miniconda3 | likely stale (ask) | Old conda install from 2025 |
| 26 | 14.0 GB | 2026-11-22 | C:\Program Files (x86) | system | Installed programs |
| 28 | 13.4 GB | 2026-11-22 | C:\Users\Ueli\AppData\Local\Packages | live | Windows store app data |
| 29 | 12.1 GB | 2026-11-22 | C:\Users\Ueli\AppData\Local\Microsoft | live | Microsoft app caches (Edge, Teams, OneDrive, etc.) |
| 30 | 12.1 GB | 2026-11-22 | C:\git\semio\.🧬semio\🦑️repo | live | Repo state in the clone |
| 31 | 11.8 GB | 2026-11-22 | C:\Users\Ueli\AppData\Roaming\Claude | live | Claude desktop app data |
| 32 | 10.5 GB | 2026-11-22 | C:\semio\.git | live | Git object store of C:\semio |
| 33 | 8.2 GB | 2026-11-22 | C:\Users\Ueli\AppData\Local\Programs | live | Per-user installed apps |
| 34 | 7.4 GB | 2026-11-20 | C:\Users\Ueli\.rustup | live | Rust toolchains |
| 35 | 6.0 GB | 2026-11-22 | C:\Users\Ueli\AppData\Roaming\Code | live | VS Code data (not an IDE target for this repo) |
| 36 | 5.3 GB | 2026-11-22 | C:\git\semio\.p9d5220c6 | likely stale | Unknown leftover dir (2026-11-05) in the clone |
| 37 | 5.3 GB | 2036-11-16 (bogus) | C:\semio\.cache\b\t | mixed | Cache lane `t` |
| 38 | 4.8 GB | 2026-11-20 | C:\Users\Ueli\AppData\Roaming\Thunderbird | live | Mail client data |
| 39 | 4.7 GB | 2026-11-22 | C:\Users\Ueli\.codex | live | Codex CLI state |
| 40 | 4.2 GB | 2021-10-16 | C:\$GetCurrent | stale | Old Windows upgrade leftover (2021); removable via Windows cleanup, not done here |

Also large but outside the top 40: `C:\Users\Ueli\.claude` 4.1 GB (live), `C:\Users\Ueli\miniconda3` above, `C:\msys64` 1.7 GB (2021, stale), `C:\cargo-dl` 1.7 GB (2026-11-22, live cargo tool install), `C:\pagefile.sys` 13.0 GB (system).

## Breakdown of the repo trees

### C:\semio (123.2 GB)
- `.cache` 56.5 GB: lanes `b` 56.0 (`b\d` 30.5, `b\r` 13.5, `b\g` 6.0, `b\t` 5.3), `f` 0.5, `p` 0.7. Lanes `d`, `r`, `g` written today. Cache content, safe to regenerate in principle, but in use.
- `.🧬semio` 52.5 GB, mostly `🦑️repo\🎫️tickets` 51.8 GB (ticket outputs, generated folders).
- `.git` 10.5 GB, `node_modules` 1.5 GB, `🧰️framework` 2.3 GB, `✏️s` 0.9 GB.
- `.cargo` is empty.

### C:\git\semio (134.1 GB)
- `.🧬semio` 78.2 GB: `🎓️teaching` 66.1 GB (of which `⚡️cache` 62.8 GB, last write 2026-11-15, likely stale), `🦑️repo` 12.1 GB (tickets 12.0), `architecture-quiz-e2e` 2.8 GB.
- `.git` 31.5 GB.
- `.p9d5220c6` 5.2 GB (unknown, 2026-11-05, likely stale).
- `🧰️framework` 3.9 GB, `🎓️teaching` 3.3 GB, `.nx` 1.9 GB, `.venv` 0.6 GB.

### C:\t (91.5 GB), FEM agents' build dirs
- `2d-ab` 56.6 GB (written 2026-11-22 17:33, live)
- `3db` 36.5 GB (written 2026-11-22 17:33, live)
- `tmp`, `2d-a`, `3d` near zero.
- Active right now, so do not clean without checking with the FEM agents.

### Users\Ueli (217.2 GB)
- AppData 161.6 GB: Local 130 (Docker 25.2, Temp 21.2, Packages 13.4, Microsoft 12.1, Programs 8.2, uv 3.9, copilot 3.4, gitkraken 2.9, WebEx 2.7, semio 1.9 + Semio 1.9 (two case-variant dirs holding the same data), ms-playwright 1.7, McNeel 1.4, java-db 1.3, Discord 1.2, JetBrains 1.0), Roaming 11.8 Claude + 5.6 Code + 4.4 Cursor + 4.8 Thunderbird + 1.7 npm, and more.
- Profile dirs: miniconda3 15.9, .rustup 7.4, .codex 4.7, .claude 4.1, .vscode 3.1, .elan 3.1, .bun 2.9, .cache 2.4, .antigravity 1.8, .Neo4jDesktop2 1.7, .kiro 1.4, .windsurf 1.4, .cargo 1.2.

### Temp
- `AppData\Local\Temp` 21.2 GB total, of which `claude` 19.0 GB (session scratch, written today). Other entries are small (vscode 0.27, DiagOutputDir 0.18, many `uc-*` at 0.09 each).

## Verdict on the owner's belief

- Caches and build output in the repo trees are the real bulk: about 250 GB across `C:\semio`, `C:\git\semio` and `C:\t`, of which about 150 GB is generated and reproducible.
- Most of it is not temp files but generated outputs and build dirs, and about 100 GB of it was written today by live agents (`C:\t`, `C:\semio\.cache\b\d|r|g`, ticket tree).
- Stale candidates that are not in use today: `C:\semio-merge-20261113-2107` (32 GB), `C:\git\semio\.🧬semio\🎓️teaching\⚡️cache` (63 GB, 2026-11-15), `C:\git\semio\.p9d5220c6` (5.2 GB), `miniconda3` (16 GB, 2025), `$GetCurrent` (4.2 GB, 2021), `msys64` (1.7 GB, 2021).
- Caveat: the `2036-11-16` mtimes on `C:\semio\.cache` mean timestamps there are unreliable and that part of the verdict is inferred from paths.

## Notes on the scan

- Sizes are sums of file lengths (not allocated disk blocks), so compression or sparse files can differ slightly from the drive's used figure. The total of accounted items is within about 10 GB of the drive's 770 GB used.
- Symlinks and junctions were not followed. Paths longer than the .NET limit may have been skipped (error counter in the TSV).
- Nothing was deleted, moved or changed.
