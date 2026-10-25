# Wave 1025f brief (10-25 19:15 CET, weekly + 5-h usage reset to 0 %; 5-h window resets 00:00 CET)

User (10-25 19:05): get the codebase completely running again. Focus ALL resources on three products running end to end:
1. the dashboard (`@semio-tech/repo-dashboard-rs`, binary `semio`, `repo dashboard`),
2. the architecture course (`@teaching/architecture-course`),
3. print (`@semio-tech/print`, `@semio-tech/print-rs`, `@semio-tech/presentation(-react)`, the bericht documents, `praesentation-projektetage`).
Other agents (Claude, Codex, Cursor, Antigravity) work on the same files at the same time: do not stop for them, keep going on your own task. Plugins / play / demonstrator are OUT of scope unless your item is shared infrastructure those three need.

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

Start state: `C/HANDOVER-1025.md` (incl. "Update 14:15" and the 14:35 RISK), `C/coordination.md` tail (from "## 15:55 (10/24"), then the `C/result-*.md` files named in your lane. ALL rules of `C/brief-wave-1023b.md` (incl. addenda), the 07:30/08:45 addenda of `C/brief-wave-1023c.md` and the "Rules added this wave" of `C/brief-wave-1024d.md` hold unchanged. Your result file is `C/result-<prefix>.md`, tools in `C/<prefix>-tools/`, output in `C/🗑️generated/<prefix>/`.

## Facts at 19:10
- No process of the previous fleet is running (all dead at the 15:40 usage cut; load 16 from peers, swap 1.2 GB, disk 62 GiB free). Every "queued"/"running" pid in a result file is dead: relaunch with a new run id.
- Several large edits were NEVER compiled or run together on one tree: purity-d moved 24 repo Rust crates (169 moves, `C/result-purity-d.md`), emoji-d rewrote docstrings in ~2,079 files (only syntax-checked), ext-d replaced i18next with an owned engine (`🖱️ui/🌐️i18n`), crel-d made course locales lazy, expc-d/expd-d/knot-d rewrote imports to explicit facades. Every GREEN in `C/result-fin-d.md` predates some of them. Re-verify what your lane owns on the CURRENT tree first; fix breakage at the source.
- The previous wave starved its own builds (20 agents, 2-h capacity queues). This wave runs ≤ 10 build lanes; Haiku audits never build.

## Rules added this wave
- Disk: check `df -h /` before any heavy launch; below 30 GiB free do not launch, write it at the top of your result file. Remove private/ad-hoc target or copy dirs you create as soon as you no longer need them. Never create another full repo copy (`~/.sinc` already holds a ~60 GB image).
- One heavy process per agent, `nice -n 10`, launched headless via `env -u SEMIO_SCRIPT_STALL_MS python3 C/chain-tools/detach.py <log> <cmd…>`; wait in the foreground with bounded loops (≤ 9 min per tool call).
- Peer churn (ENOENT on a renamed path, "outdated implementation", half-edited file of someone else): wait 2-5 min and relaunch the same target; do not "fix" someone else's half-finished edit unless it stays broken > 30 min (then fix at the source, minimal, and note it).
- Ports: course e2e 6061/6063/8791/8793 (e2e-f only), projektetage 6050 (print-f only), dashboard smoke (dash-f picks a free one, writes it in its result file).
- Result file every ≤ 20 min, newest first; `FOR <prefix>:` lines at the top for cross-lane handoffs. Final message ≤ 8 lines. No sub-agents, no ticket tools, no modifying git commands.

## Lanes (Sonnet 5.5 high executors; Haiku 5.5 high read-only audits)
| prefix | product | item | continue from |
|---|---|---|---|
| dash-f | dashboard | P0: official Bazel `repo-dashboard-rs:build` + every dashboard test target (test-unit, windows, process-deadline, dispatch, ci-source, language, source-freshness, journeys, authority, `:test`) on the current tree after purity-d's crate moves; fix at the source; then launcher smoke EN+DE (`repo run @semio-tech/repo-dashboard-rs:run` headless, daemon status, 2 finite tasks, cancel, stop) | result-fin-d, result-purity-d, result-dflake-d, result-dash3-d, result-term-d |
| crate-f | dashboard closure | `cargo check --tests` over the 48 repo crates (`C/purity-d-tools/cargo-repo.sh`) then Bazel `:test` of all repo-*-rs (fin-d r1-r3 batches); fix moved-crate breakage at the source | result-purity-d, result-fin-d, result-rtest-d |
| course-f | course | non-e2e closure on Bazel on the current tree: course typecheck/test/check/build, proctor, course-react, pets(-react), ui-react (owned i18n engine), ui-rs, ui-styling (color removal), framework(-rs, -actor-rs), crel2-d u3/u4; fix reds | result-fin-d (course rows / chain.tsv), result-crel2-d, result-ext-d, result-cui-d |
| e2e-f | course | course e2e via `C/e2e-orch-tools/run.sh course`: DEV topology full 76/76 (proves the shortage-reload fix in `🎭️e2e/🚶️learner`), then REHEARSAL topology; pets `200% text` en longest-name specs at 320/375 px in rehearsal (`last seen: 0`) + parachute 6140 vs 6000 stage-ms: root cause, fix, focused 5× then full | result-crel2-d, report-e2e-orch-e2e-1025-crel2-d-b1, result-cpets-d, creload-d-tools |
| print-f | print | official Bazel: print-rs:test, print check/build/test, presentation(-react):test, bericht set, projektetage build/test + headless dev smoke (6050); fix reds | result-fin-d (p1-p5), result-print-d, result-pres-d, result-expc-d |
| nx-f | all 3 | default route (Nx) 13/13: everything in default-d's table + `print:test` (nx4-d plan: cold → probe → cache-hit proof) + dashboard `:test` on Nx; fix Nx infra faults | result-default-d, result-nx3-d, result-nx4-d |
| ext-f | all 3 | literal-external truth gate: re-run `C/ext-d-tools/truth.ts`; resolve the remaining oracle conflicts (`image`, `serde_json`) at the source; locks stay in sync (manifest + lock together) | result-ext-d |
| cache-f | infra | caching module: the 85 red / 4 timeout test files of `C/table-cache-d.md` green through the official `test-<suite>` targets | result-cache-d, table-cache-d |
| knot-f | infra | official `repo-lib:lint-dependency-direction` verdict (Nx and Bazel), the 4 remaining `no-circular` + course 5 edges; bound-d's open items (`repo-lib:test-package-body-policy` re-run, discovery-boundaries facade layout) | result-knot-d, result-bound-d, result-expc-d |
| gen-f | course infra | warm proof: cold → warm 0 re-executions for course build after gen-d's precision inputs (actor-typegen, framework-generate, ui-contract-generate); unblock cad `Cargo.lock` wasm-bindgen 0.2.126 vs pin 0.2.129 via `bun repo cargo … update --offline` if still drifted | result-gen-d, result-cinp-d |
| status-f (Haiku) | all 3 | `C/status-1025f.md`: per product/target last verdict + whether it predates the never-compiled edits (compare run start vs file mtimes); refresh every ~40 min from the lanes' result files | — |
| hyg-f (Haiku) | all 3 | `C/audit-hygiene-1025f.md`: `[DEBUG]` lines in the three closures with file:line + owner, leftover processes, cache/disk fill, stray files outside ticket folders | — |
| docs-f (Haiku) | all 3 | `C/audit-run-instructions-1025f.md`: how a human dev runs each product end to end (`repo dashboard`, course dev/rehearsal, print builds) per README/docs vs the current `📋️project.json` targets; list stale instructions with file:line | — |
