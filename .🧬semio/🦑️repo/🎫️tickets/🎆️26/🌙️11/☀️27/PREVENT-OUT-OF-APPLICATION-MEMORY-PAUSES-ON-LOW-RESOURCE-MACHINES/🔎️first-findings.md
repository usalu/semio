# 🔎️ First Findings: Out of Application Memory on 2026-11-27 (Coordinator, 18:05)

Machine: 16 GB macOS, 10 cores. macOS paused Claude (pid 11702). The kernel logged `memorystatus: resuming Claude [11702]` at 18:01:21, and the pause window started around 17:35–17:52. Swap at 18:01: 1.5 of 3 GB. Free memory then was 57 %.

## Observed

1. **Claude desktop app diff pane.** Claude Helper (pid 74276) repeatedly spawns this command:
   `git … diff --ignore-submodules=dirty --submodule=short --no-textconv --numstat -z -M --raw --no-abbrev 277881dca4ea…`
   - One run reached 2.06 GB RSS within 30 s.
   - The base `277881dca` is `refs/heads/old-repo/1/⛳wip` (2026-06-04), 762 commits back.
   - The diff spans 315,851 files and +101,956,842 lines. Git skips rename detection; `renameLimit` would need 304,427.
   - The repo has no `main` ref, so the app falls back to an old branch as base.
   - This is outside the repo tools, and the capacity queue does not reserve for it.
2. **The warden restarts often.** Reasons logged: "stopped: its own source changed", because other agents edit the repo tool, and "stopped: no subject for 10 minutes". Each restart leaves a gap with no guard.
3. **The warden acts only on its own subjects (grants).** Processes outside grants are invisible to enforcement: the desktop app's git, the Claude renderer (800 MB), Firefox, and nohup shells.
4. **Footprint reads fail on darwin.** `footprint -f bytes --noCategories -p <pid>` fails, and the guard falls back to RSS.
5. **The queue admits against "available" but reserves less than tests use.** Example: CL-T1 measured 11.3 GB on a 3.6 GB grant earlier today, and CL-P ran 2718 MB on a 2500 MB grant.
6. **Load at the time.** Final model-catalog shards (3600 MB each), the full multiview matrix (2700 MB requests, 2 lanes), and lanes CX-JSON and CL-T4 (2500–3600 MB) were all running. The queue showed 4 running and 6–9 waiting.

## Open

- Memory timeline 17:00–18:01 from `~/Library/Caches/semio/host-capacity/metrics/t*.ring`.
- Peak per process tree.
- Why the compressor and swap could not absorb the load.
