# Handoff: Procedural 3d End To End (2026-11-22 17:00)

Supersedes [📓️handoff-2026-11-22.md](./📓️handoff-2026-11-22.md) (read it for launch commands and background). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END` is open; `ticket_close` needs `📌️important/📝️.md`, which is still missing.

## Running at handoff (do not restart blindly)
- React launch `repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype`, started 14:34. Its log was lost when an agent deleted `🗑️generated`; follow it in the service-host log `$TMPDIR/semio-repo/501/host-0003b7003034.log` (look for `[graph-closure]`, `[graph-seal]`, `request-failed`). Not serving 6018 yet.
- WGPU launch `…:dev-generation3d-wgpu-prototype`, started ~15:12, log `🗑️generated/wgpu-v13.txt`. At 17:00 it was still in the graph-slice read (round 3, peers edit paths under the read). Not serving 6118 yet.
- Both reads restart whenever a peer adds a native crate; that is now handled (registry follow, see below), but it costs time. Check with `curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:6018/` (and 6118).
- Background agents that may still be finishing: engine-attach-tests-2 port plus the node-graph fit trace emitter, retained-commands test rerun, and the frame-loop fix. Their notes are listed below; if a note is missing or partial, restart that lane from its note.

## First blocker for any wgpu compile
Every `repo cargo` call that builds `wgpu 29.0.4` fails with `couldn't read …/src/util/blit.wgsl`, although the file exists in the extracted registry. Two agents hit it (lanes 3 and 6). Diagnose it inside the gate before resuming any wgpu lane: the sealed or snapshotted source view of registry crates (`include_str!` of a non-`.rs` file), offload to a remote worker, or source capture. Compare with [📓️cargo-registry-extraction-race.md](./📓️cargo-registry-extraction-race.md) and [📓️procedural-cargo-missing-files-audit.md](./📓️procedural-cargo-missing-files-audit.md). Also re-check that `semio-renderer-wgpu-host-winit` stays under its 262144-byte source boundary.

## Done this session
- Launch blocker (window-resource owner): no registration change needed; the registry was transiently missing rows while peers regenerated it. Oracle and analysis: [📓️window-resource-owner-blocker-2026-11-22.md](./📓️window-resource-owner-blocker-2026-11-22.md), [native-owner-closure-check.py](./native-owner-closure-check.py) (4835 crates, 0 unowned roots).
- Root fix: the registry follow now judges the current registry by content, so a transient registry triggers a re-read instead of a stable owner failure (`🧭️registry-follow/🟦️.ts`, test `🧪️tests/🕸️graph/🪤️registry-transient`). Live hosts now log `[graph-repair] registry followed`.
- Cargo gate blocker "builtin-read import coverage refused": a peer's `[DEBUG] require('node:fs')` probe in `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts` was removed: [📓️builtin-read-coverage-refusal-2026-11-22.md](./📓️builtin-read-coverage-refusal-2026-11-22.md), scanner [builtin-read-coverage-scan.ts](./builtin-read-coverage-scan.ts). If cargo fails with that message again, rerun the scanner.
- Compile-slot busy-wait (one cargo run held about 100 spinning wrappers and drove load to 145): waiters sleep on fs events now: [📓️cargo-wrapper-busy-wait.md](./📓️cargo-wrapper-busy-wait.md). Takes effect after the sealed tool copy is refreshed. An end-to-end wrapper count is unconfirmed.
- Rust image reclaim thresholds are relative to partition capacity, the tsc errors are fixed, and the tests are green: [📓️rust-image-auto-reclaim.md](./📓️rust-image-auto-reclaim.md).
- Battery oracle fixture-path bug fixed with the shared resolver `🐍️example-fixtures.mjs` (8 oracles for 8 examples offline), run commands in [📓️battery-readiness-audit.md](./📓️battery-readiness-audit.md). The batteries need the repo bun on PATH.

## Lanes and their state
1. Retained-commands: checks clean (1 error fixed; a separate testkit crate does not exist). Tests not yet verified; a rerun was in flight. Note: [📓️retained-commands-port-2026-11-22.md](./📓️retained-commands-port-2026-11-22.md).
2. Procedural cargo "couldn't find file": stale (offload fix of 11:20Z; registry healthy): [📓️procedural-cargo-missing-files-audit.md](./📓️procedural-cargo-missing-files-audit.md). Still to do: run the io-text, preview-eval and generation3d tests ONE package per call, and make the `🔖️mutation-source-authority` EPERM errors name the path and error kind (with a test). Paused because of swap.
3. wgpu test drift: engine-attach-tests-2 source edits are applied but never compiled. Cargo failed before reaching the crate (a peer's wire crate, then `wgpu-29.0.4: couldn't read util/blit.wgsl`, a registry or snapshot race). Next: compile, fix the marquee `advance_paint2d_pixel_selection_into` call (missing `window_id`), then run `fit_graph_action_persists_…`. The `shell node-graph fit` emitter already exists (`framework_fit_node_graph`), but it logs only when the viewport changes. Notes: [📓️engine-attach-tests-2-port.md](./📓️engine-attach-tests-2-port.md), [📓️wgpu-test-drift-audit.md](./📓️wgpu-test-drift-audit.md). shell-tests-input-3 and docs-1/-2 need 0 test edits; verifying them by compile was paused.
4. Job-context: 5/7 green; contracts 19/24 and receiving-context 2/6 are red. Decisions made (binding):
   - (a) The birth service admits the decoder's first receiving scope as reserved work; the caller is never charged.
   - (b) Control rows are paid at claim time inside the reserved claim, and `finish_scope` posts only the body. Only the second half is implemented (`🛬️receiving/🎟️state/🦀️.rs`), so control rows are currently paid by no one. Restore exact conservation first.
   - (c) The law_1 fixture must supply a body that fits one call.
   - Still undiagnosed: law_4 (posting-refusal count 0 vs 1), both source_law_2 tests, and the 4 receiving-context tests.
   - Full log: sections 11+ of [📓️rust-law-verification-2026-11-22.md](./📓️rust-law-verification-2026-11-22.md). Paused.
5. world_host_unit_3 GPU-section test: compile symbols resolve; the failure is at runtime (probably the job fixture loop or the prepared gate). The first panic has not been read yet: [📓️world-host-unit-3-gpu-section.md](./📓️world-host-unit-3-gpu-section.md). Paused.
6. Batteries: [📓️battery-readiness-audit.md](./📓️battery-readiness-audit.md) lists the likely reds. The frame-loop row needs the trace emitters, idle assertions, the idle-spin fix and non-fatal frame-credit refusals: [📓️wgpu-frame-loop-quarantine-audit.md](./📓️wgpu-frame-loop-quarantine-audit.md) §6, result note [📓️wgpu-frame-loop-fix.md](./📓️wgpu-frame-loop-fix.md). Steps A–D and the probe and battery rows are edited but neither compiled nor run (blocked by `blit.wgsl`). Also open: the earlier `frame_maintenance_terminal_fault(.., deadline_exceeded)` lane still exists. Retire it together with the three law sources that pin its text, then correct audit R12 and item 7.
- Also open: the replica stale-overwrite leftovers (regenerate the remodel `🛂️.descriptor.semio`, hub `Cargo.lock` metadata, compile-check the merged wgpu renderer files): [📓️replica-stale-overwrite.md](./📓️replica-stale-overwrite.md). Paused.

## Host discipline (learned the hard way)
- macOS paused the session for swap at about 16:00, with 8+ agents compiling and two graph reads holding about 9 GB of bun. Keep at most 2 compiling agents while both reads are running.
- Run cargo one package per call at parallelism 1–2. Before each call, wait while free memory is below 40 % (`memory_pressure`) or the 1-minute load is above 20.
- Agents must never delete `🗑️generated`. An agent did, and the React launch log was lost.
