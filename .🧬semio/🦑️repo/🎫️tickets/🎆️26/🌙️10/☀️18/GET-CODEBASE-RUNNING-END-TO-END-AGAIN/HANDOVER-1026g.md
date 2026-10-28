# Handover 2026-10-26 (wave 1026g) → next session

Ticket: `2026/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` (C = this folder). Do NOT close it.
Weekly usage at 98 % (resets 2026-10-31 23:00). The machine rebooted at 22:42; no lane, chain or build of the old fleet survives.

## Resume prompt (paste into a new session)

> Resume ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN` as coordinator. Read `HANDOVER-1026g.md` first, then the last 60 lines of `coordination.md`, `gate-g-tools/queue.md` ("CHECKPOINT 22:20") and `pregate-1026g.md`. Rules: `brief-wave-1026g.md` with all addenda (library/process/launcher edits only via the landing gate; verify on `mirror-tools/mirror.py` mirrors; every `bun test` path starts with `./`; no `repo ticket purge-artifacts` / `repo clean`; gate priority over measurement runs). Run at most 10 lanes. Order: (1) after the next reboot, confirm the journal no longer writes into the unmounted tools image (fixed 23:35, unproven by a real reboot; the hand-mount workaround is described below), (2) course:test's proctor gap is fixed (verify once more with course e2e), (3) land the correctness fixes one at a time with a product build each, (4) re-land quietfc from quietfc-g's new batch, (5) measure the six targets.

## State of the live tree (23:00)

| Area | State |
|---|---|
| Last product-GREEN build | `g6c-course` 22:59 (coordinator): course:build exit 0, 246 s, peak 8.1 GB, generation 580 (39 projects in 73 s). It covers everything landed so far, including purge-g `purge`, verify2 `journal-fix`, toolsetup `probes`, verify2 `vouch` + rev4-g `vouch-anchor`, oracle2 (each with its docs3 batch). Revert commands if a later product turns red: `gate-g-tools/queue.md` "CHECKPOINT 22:20" |
| Dashboard / print verdicts | `g6-dash` repo-dashboard-rs:build GREEN (186 s, 4.6 GB); `g6-print` print-rs:test GREEN (422 s, 8.0 GB, 1/1). The live tree is product-verified as of 23:13. `g6-ctest` (course:test, red at 21:12 on generation 578) started 23:13; result in `coordination.md` |
| Reverted | quietfc (21:50, upgrade asked 28672 MB); graphlat `plan-once` (22:49 by the coordinator: it WAS live, gate-g's "HELD" was wrong; backup `gate-g-tools/run/backups/gen-before-planonce-revert.mjs`) |
| Restored by the user | the 8,024 monolith-split files (22:4x, git status clean) |
| Detached by the coordinator | 4 stray module-directory cache images (not deleted) |

## Fixed by the coordinator after the reboot (23:20–23:40, landed live as the gate)

- **proctor slice gap (course:test RED → GREEN):** `sliceClosureV1` / `sliceViewV1` in `🕸️graph/🕰️generations/🔌️load/🟨️.mjs` (+ `.d.mts`) now follow named input groups (`group`, `^group`, and the group a `projects` row names on that owner) to the projects their rows name. The Python oracle `🧪️tests/🐍️.py` mirrors the rule. 2 corpus scenarios were red on the old code and are green now; the load suite passes 72/72. Official `course:test` (`g7-ctest`) GREEN: 136 s, 6.6 GB, generation 583. Backups: `proctor-slice-tools/base/`.
- **journal writes into the unmounted tools image (fix for the blocker below):** `ensureWatcherV1` takes `writable()`. The tools binding passes `toolsPartitionWritableV1(root)`: a partition with an `image.json` receipt is writable only while `.cache/b/t/m` lies on another device than `.cache/b/t`. After a reboot the journal now refuses to install instead of blocking every build. Provision suite 8/8 (2 new laws). Not yet proven by an actual reboot. Backups: `watcher-image-tools/base/`. Verification build `g7-course` GREEN (181 s, 8.2 GB).

## BLOCKER found 22:50: every build fails after a reboot (fixed 23:35, see above)

`course:build` fails in 5 s: `Unknown entry exceeds exact Native creator inventory` (cleanup `inspectPersistentReceiver`: an unmounted image root must be empty).
Cause: the change-journal service (`📚️library/🏗️build/🏡️resident/📜️journal/main`, started automatically after login) materializes `semio-repo-watch` into `.cache/b/t/m/repo-watch` while the tools image `.cache/b/t` is NOT mounted, i.e. into the bare mountpoint. Stopping the service and deleting the folder does not help: it restarts within seconds and writes it again.
Code: `📚️library/🏗️build/🏡️resident/🔭️watch/🏗️provision/🔌️tools/🟨️.mjs` `ensureResidentWatcherV1` writes to `join(root, TOOL_REGISTRY_V1.partitions.tools, "repo-watch")` without acquiring the tools image.
Fix (stage via the gate, law first): the journal must hold the tools-image receiver (mount + lease) before materializing the watcher, or install the watcher into its own receiver; add a law "an inactive image root is never written".
Workaround that worked at 22:55: stop the journal service and its `semio-repo-watch`, `rm -r .cache/b/t/m/repo-watch`, then mount the tools image by hand: `hdiutil attach -nobrowse -noautoopen -mountpoint "$PWD/.cache/b/t/m" .cache/b/t/fixed.dmg` (contents intact; the hand mount adds `noowners`, which the build accepted).

## Correctness fixes staged (mirror-verified, not live), priority order

1. stale-g `staged/owner-child`: 716 wrapped recipes (190 projects, incl. print, ui-styling, course, presentation) key only the owner-command driver, never the child script → stale hits today. Re-infers ~3,754 owners: land ALONE with a product build.
2. pathmod-g `staged/rust-mounts`: inline-`mod` macros make the Rust source parser refuse → crate-dir fallback misses mounted files (9 crates fall back, 2,809 have one in their closure).
3. red-g `inferfix`: command-module dependency edges lost since 04:12. Land ALONE.
4. keys5-g `schema-index-key` (+ live fix of framework:test-source-text already applied), keys4-g `precision-rules`.
5. witness2-g batch (git ignore sources incl. core.excludesFile, remote-mount journal refusal, symlink vouch, root-gone identity, exact `ps` argv). AFTER journal-fix/vouch/probes.

## Performance batches staged (not live)

quietfc-g `staged` (whole quietfc + upgrade ceiling 12288 MB + measured p90 bound + real quiet cap + soft-stop grants; NOT run on a mirror yet), graphlat `plan-once2` (replaces plan-once) + `reuse-once` (double inference/setup: 97.6 → 58.1 CPU s), boot-g `graph-memo`, rustfast-g `rustfast` (Rust ingress: planning digest + admission memo + foreign hash memo), gram3 `index-chunks` + `churn`, gram4 `shape-keys`, seal2-g `seal2` (before the sealed flip), disk-g 5 batches, red-g 8 batches, fdleak-g `guard`, obsrows2-g, sweep2-g, bench3-g `observer`, docs3-g 22 doc batches (land each WITH its code batch). Landing order and pre-check verdicts for G2/G3: `pregate-1026g.md`; later groups were not pre-checked.

## Measured once (targets not met)

| Target | Measured | Budget |
|---|---|---|
| Course hot no-op | 60-83 s | < 10 s |
| Course one-line edit | 506 s (graph 307 s) | < 60 s |
| Rust (print) one-line edit | 2,637 s | < 2 min |
| Peer edits / 3 agents on 16 GB | never measured (harnesses `peer-g-tools/`, `agents3-g-tools/` ready) | 0 re-executions / no swap storm |

## Products

Builds verified green at 23:13 (course:build, repo-dashboard-rs:build, print-rs:test). `course:test` is still RED on the fresh generation 580 (re-run 23:14, 15 s): `Unknown build input project: @teaching/proctor (task @teaching/architecture-course:test:prototype)`. So it is not a stale peer generation. proctor's `📋️project.json` is valid. The course names proctor only through an input row `{ input: "cargoToolchain", projects: ["@teaching/proctor"] }` and `@teaching/proctor:build`/`:dev` task references. The slice refresh of the course closure (39 projects) does not include proctor, and the build-input resolver looks the project up in the slice. Fix: closure/slice expansion must include every project named by an input row (with a law that fails today). This is priority 1 after the journal/tools-image blocker. Dashboard journeys 33/35 (PTY timing under load). Open official runs: `repo-tickets-rs:test`, `repo-graphql-go:test`, course e2e (`e2e-f-tools/final.sh`). Commands: top of `result-integ-g.md`.

## Open user decisions

- Delete `🧰️framework/🤖️generated/process` (193 MB, unread; disk-g `procstore` refuses until gone).
- Delete the stray module-directory `.cache` roots (16.2 GiB; commands in `result-disk-g.md`).
- The fixture tickets the Go tests wrote into the real ticket tree.

## Process lessons (also in memory)

- Checkpoint the fleet at ~93 % weekly, not 97 %.
- A full 20-lane fleet starves the gate: verdict builds waited 25-70 min in admission.
- `bun test` without `./` is a filter that exhausts host file descriptors.
