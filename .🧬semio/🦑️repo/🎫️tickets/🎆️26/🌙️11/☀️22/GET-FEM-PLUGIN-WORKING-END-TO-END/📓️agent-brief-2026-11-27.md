# 📓 Agent Brief (FEM plugin end to end, 2026-11-27)

Coordinating ticket: `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END` (`$T22` = `C:\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️22\GET-FEM-PLUGIN-WORKING-END-TO-END`). Sibling 3D ticket with the most recent work: `26/11/25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE` (`$M25`).

Read first: `$M25/📓️agent-brief.md` (paths, cargo env, detached launch rule). All its rules apply, with these changes:
- Your scripts go in `$T22`, outputs in `$T22/🗑️generated/<you>/`, final report `$T22/<topic>.md`.
- Failing features: `$T22/feature-matrix-2d-v2.md` (2D) and `$T22/feature-matrix-3d-v2.md` (3D, F1-F14); 2D fixes already done: `$T22/fem2d-plugin-fixes.md`; 3D status audit: `$M25/f-status-audit.md`; 3D fixes since: the `.md` reports in `$M25` (viewport-drag, playback, history-replay-custody, modal-buckling, first-click-pick, gumball-rotate, clipboard, …).
- The coordinator alone builds and publishes the wasm component (`$T22/fem-wasm-pipeline.sh`, target `C:\t\wd`/`C:\t\wdb`). Never touch `C:\t\wd*`, never stop the vite servers on 6086/6087, never kill processes you did not start.
- Use `-j 3`. Before a long build check `df -h /c`; if under 25 GB free, delete only your own finished `C:\t\<you>*` dirs.
- Every fix: root cause, a native law (Rust `#[test]`, plus a language-agnostic vector where the area has fixtures), 2D and 3D twins kept in step.
- Do not stop to ask. If a peer's half-written edit breaks your build, retry after a few minutes; fix it yourself if it stays broken over 20 min and is trivially in scope.
