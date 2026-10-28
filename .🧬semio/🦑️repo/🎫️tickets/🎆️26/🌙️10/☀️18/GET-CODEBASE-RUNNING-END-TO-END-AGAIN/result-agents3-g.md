# result-agents3-g (wave 1026g): three concurrent agents on a declared 16 GB machine

Tools `C/agents3-g-tools/` (`agents3.py run|summary|restore`, `measure_lib.py` = bench2-g's measure.py with a guarded main and a `BENCH_ENV` passthrough, `phases.json`, `products.json`); output `C/🗑️generated/agents3-g/<tag>/` (`timeline.jsonl` 2 s samples, `results.json`, per-attempt logs, `edits.jsonl`; sweepable, the numbers are copied here).

## Status (21:58, CHECKPOINT: weekly usage 97 %)
- NO NUMBER YET for the 3-agent scenario. Harness ready and smoke-tested (fake root, edits restored byte-exact, sampler, summary). Nothing is running: waiter stopped (own pid), no edit ON; the three leaf files are byte-identical to their originals (course `📚️course/🟦️.ts` 2057 B, print `🦀️.rs` 8571 B, peer `📜️script.ts` 5202 B).
- Staged: nothing (no product defect fixed by me; the harness lives in `C/agents3-g-tools/`).
- Exact next command (when quietfc has been RE-LANDED with a fixed upgrade and no gate product build / rustedit-g print-rs run executes): `env -u SEMIO_SCRIPT_STALL_MS python3 -I C/chain-tools/detach.py C/🗑️generated/agents3-g/t5.log python3 -I C/agents3-g-tools/agents3.py run --tag t5 --wait` (waits itself for: no bench2-g process, no print-rs run, no `g*-course.log` written in 3 min, last quietfc line LANDED, `leases status` WAITING < 4 twice; then declares 16384 MB + 8 cores). Afterwards `python3 -I C/agents3-g-tools/agents3.py summary --dir C/🗑️generated/agents3-g/t5` prints the tables (queue/run/wall/phases/actions, peak footprint/RSS per tree, Bazel servers, swap/compressor/swapouts, free %, starvation, exit 75). Verify afterwards with `git diff --stat` that the three files are unchanged.

## Attempts
- t1 (label pre-quietfc, private capacity directory, 20:53-20:55): stopped after ~2 min on the coordinator's gate-priority rule. In that window all three agents sat before execution (bun repo parents ~90 MiB RSS / ~30 MiB footprint each, Bazel server 348 MiB), host load 34-42, free memory 70-1190 MiB, compressor ~0.6 GB, swap used 3.3 GB of 4 GB: the host was ALREADY under swap pressure before the scenario's builds started.
- t2 (21:54): started by a waiter bug (stale LANDED line read after gate-g's 21:50 revert), killed after ~1 min, files restored (dir `t2-aborted`, unused). Waiter rewritten (`quietfc_landed`: only the LAST gate line about quietfc counts).

## Findings so far
1. (post-quietfc, gate-g g5-course.log 21:22-21:50) the quiet follow-up works (hot course admitted at 2048 MB at once, "forecast quiet because the earlier run finished 485 s ago"), but as soon as work had to run the upgrade asked to grow 2048 -> 28672 MB (full request 35840 MB clamped to total - floor 4096 MB) and waited 25 min ("-120 MB spare above the 4096 MB floor; 1 running, 2 ahead"); a course build measures 4.4-6.3 GiB tree (bench2-g). The estimate is ~5x too high and the wait needs the whole machine = starvation by construction (gate-g reverted quietfc 21:50). On a declared 16 GB machine the clamp would be 16384 - floor 2048 = 14336 MB, i.e. a third of the machine for ONE course build, so 3 agents could never run side by side. Acceptance sub-criteria (no swap storm, no starvation) cannot be judged until the request is sized by the measured peak, not 200 % of the highest phase.
2. Reading the quietfc fix: `estimates.quietReferenceMegabytes/quietMaximumMegabytes` cap only the quiet reservation, the upgrade target is still the busy-class size; the follow-up rule must also bound the upgrade target by the observed peak of the class (tree footprint, not RSS).

## Verdicts per acceptance sub-criterion (3 agents on 16 GB)
- no swap storm: NOT MEASURED (host at 3.3 GB swap used with no scenario running).
- no starvation: NOT MEASURED; gate-g's g5 run is the only data point and it starved 25 min.
- fair queue + progress visible to waiters: capacity rows present (`[capacity] ... waiting Ns ...`) in EN; not judged for fairness.
- exit 75 / refusals: none seen.
