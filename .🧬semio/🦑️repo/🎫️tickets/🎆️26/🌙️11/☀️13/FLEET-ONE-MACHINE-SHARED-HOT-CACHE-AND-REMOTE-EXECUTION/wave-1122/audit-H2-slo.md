# 🎯 Audit H2 (slo): Objectives of the Build System, Wave 1122

Read-only audit, 2026-11-22. No source, state or tree file was changed. No bench, build or test was run. No sub-agent was used.

## Sources read

- Objective definitions: `B/📏️incremental/🏎️bench/🎯️slo/🔣️.json` (115 objectives, 42 probes, 11 units). Source `B` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`.
- Scenario lists: `repo incremental list` (22 scenarios), `repo concurrency list` (5 workloads), `repo conformance list` (21 scenarios with nx, bazel and semio claims). `repo bench list` returned 68 kB; its data was parsed, and the objective set matches the SLO file.
- History folder and stored reports: `bench run --history=<folder>` takes the folder from the user, and the code names the files `benchHistoryNameV1` (`YYYYMMDDTHHMMSSZ` plus host class). No default history folder exists in the build library, the caches (`~/Library/Caches/semio/repo`) or the wave folders. Searches of the ticket tree (`☀️13`, `☀️14`, `☀️15`, `☀️16`, `☀️20`) found no stored bench report.
- Live fleet figures: `brief.md` (measured 00:38) and `wave-1121/census-low-resource-thresholds.md`.
- Compact table of the 115 objectives: `W/🗑️generated/H2/slo-table.txt` (deleted at the end of this lane; the full table is in section 4).

## 1. Result in one view

- Objectives: 115. Blocking: 46. Advisory: 69.
- With goal or gate: 66. Record-only (no goal, no gate): 49, guarded only by the baseline comparison.
- Latest measured value: none exists for any objective. No stored report was found, so `bench compare` could not run: it needs two reports of one host class. Every objective is therefore **unproven**.
- Failing: none is proven failing by a stored measurement. Failing in the live fleet, outside any objective: 0 of 13 workers Ready; 1000 action-key divergences in the remote cache; remote execution 1 completed, 1 failed, 3 offloaded, six machines refused with `no-headroom`; coordinator swap at 20.0 of 21.5 GB (brief, 00:38).

## 2. Coverage of the four themes asked for

| Theme | Objectives that measure it | Status |
|---|---|---|
| Low-resource behaviour | `mem-noop-run-max`, `mem-edit-ts-max`, `product-*-rss-max` (blocking on noop and peer), `planning-two-rss-max`, `planning-warm-rss-max` (blocking, 2 GiB goal / 3 GiB gate), `planning-two-oom-max`, `planning-warm-oom-max` (blocking, 0 OOM kills), `planning-warm-swap-max` (advisory), `planning-warm-store-max` (blocking) | Defined, never measured. No objective for a 16 GB machine under load (queueing, offload when busy). The census of thresholds says a hard stop has no override path; no objective checks the wait time behind that stop. GAP. |
| Remote cache hit rate | None. `cache-second-run-*-min` (blocking, ratio 1) measure the local second run only. The conformance scenario `remote-cache` (nx native, bazel adapter, semio native) is a behaviour check, not a rate. | GAP. The live hit rate (peers tier 0 hits / 704 misses / 20 errors) is not an objective. |
| Remote execution | None. No probe, no objective, no conformance scenario offloads work to another machine. | GAP. Live state: 1 completed, 1 failed, 6 refusals with no-headroom while idle machines exist. |
| "Nothing computed twice" | `product-*-noop-executed-max` (blocking, 0 re-executed actions), `product-*-peer-unrelated-executed-max` and `product-*-peer-scheduling-executed-max` (blocking, 0), `cache-second-run-*-min` (blocking, ratio 1), `planning-leaf-edit-closures-max` (advisory). Conformance: `cache-restore`, `early-cutoff`, `failure-not-cached`, `single-flight` (adapter in nx, native in bazel). Incremental: 22 scenarios (`fw-ts-comment` early cutoff, `fw-ts-unrelated`, `activation-unrelated`, `typegen-unrelated`, `framework-generate-unrelated`). | Defined, unmeasured. Not covered: the same action key with a different output sha across machines (live: 1000 divergences). No objective or conformance scenario checks cross-machine output identity. GAP. |

## 3. Findings

1. No objective is measured. The brief's wave goal needs a stored baseline, which does not exist. The first step of the wave must be one `bench run --history=<folder>` on the coordinator, in a second run, to have two comparable reports.
2. Three gaps against the wave goal: remote cache hit rate, remote execution, and cross-machine output identity. The objectives file must gain them before the wave can be judged.
3. Several blocking objectives are tight enough to fail under fleet load: `noop-run-resident-p95` and `noop-test-resident-p95` (goal 300 ms = gate 300 ms, p95 needs 20 samples), `edit-*-resident-start-p95` (500 ms goal = gate). The census note says a missed gate on a saturated host stays unproven, so these will read as unproven, not failed, on the live fleet.
4. The `cli-help-p50` objective is blocking with goal 100 and gate 250. It is the only blocking objective without a stored value and not tied to the build path.
5. `product-course-edit-green-p50` and `product-print-edit-green-p50` are blocking at 60 s; `product-dashboard-edit-green-p50` at 120 s. These are the only end-to-end "edit to green" blocking gates.
6. Enforcement: `blocking` means `bench compare` reports it over budget. The wiring to CI or to the fleet was not found by a grep of `🛂️acceptance`; the tool description of `bench compare` is the only source for this.
7. The `⚡️caching` folder (`C`) and the remote cache objectives in `B` were not read in depth; this audit covers the SLO definitions, the scenario lists and the live numbers given in the brief.

## 4. Objectives (all 115), status unproven unless noted

Columns: id | probe | metric.statistic | direction | goal | gate | enforcement | tools | latest value. Latest value for every row: none stored (no bench report exists). Blocking objectives are marked `!`.

| id | probe | metric.stat | dir | goal | gate | enforcement | tools | latest value |
|---|---|---|---|---|---|---|---|---|
| !cli-help-p50 | cli-help | wallMs.p50 | lower | 100 | 250 | blocking | neutral | unproven (none stored) |
| cli-help-p95 | cli-help | wallMs.p95 | lower | 200 | 400 | advisory | neutral | unproven (none stored) |
| cli-help-p99 | cli-help | wallMs.p99 | lower | - | 1000 | advisory | neutral | unproven (none stored) |
| noop-run-oneshot-p50 | noop-run-oneshot | wallMs.p50 | lower | 300 | 1000 | advisory | nx,bazel | unproven (none stored) |
| noop-test-oneshot-p50 | noop-test-oneshot | wallMs.p50 | lower | 300 | 1000 | advisory | nx,bazel | unproven (none stored) |
| !noop-run-resident-p95 | noop-run-resident | wallMs.p95 | lower | 300 | 300 | blocking | nx,bazel | unproven (none stored) |
| !noop-test-resident-p95 | noop-test-resident | wallMs.p95 | lower | 300 | 300 | blocking | nx,bazel | unproven (none stored) |
| edit-ts-oneshot-start-p50 | edit-ts-oneshot | startMs.p50 | lower | 500 | 2000 | advisory | nx,bazel | unproven (none stored) |
| edit-ts-oneshot-green-p50 | edit-ts-oneshot | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !edit-ts-resident-start-p95 | edit-ts-resident | startMs.p95 | lower | 500 | 500 | blocking | nx,bazel | unproven (none stored) |
| edit-ts-resident-green-p50 | edit-ts-resident | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| edit-rust-oneshot-start-p50 | edit-rust-oneshot | startMs.p50 | lower | 500 | 2000 | advisory | nx,bazel | unproven (none stored) |
| edit-rust-oneshot-green-p50 | edit-rust-oneshot | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !edit-rust-resident-start-p95 | edit-rust-resident | startMs.p95 | lower | 500 | 500 | blocking | nx,bazel | unproven (none stored) |
| edit-rust-resident-green-p50 | edit-rust-resident | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| edit-go-oneshot-start-p50 | edit-go-oneshot | startMs.p50 | lower | 500 | 2000 | advisory | nx,bazel | unproven (none stored) |
| edit-go-oneshot-green-p50 | edit-go-oneshot | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !edit-go-resident-start-p95 | edit-go-resident | startMs.p95 | lower | 500 | 500 | blocking | nx,bazel | unproven (none stored) |
| edit-go-resident-green-p50 | edit-go-resident | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| edit-python-oneshot-start-p50 | edit-python-oneshot | startMs.p50 | lower | 500 | 2000 | advisory | nx,bazel | unproven (none stored) |
| edit-python-oneshot-green-p50 | edit-python-oneshot | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !edit-python-resident-start-p95 | edit-python-resident | startMs.p95 | lower | 500 | 500 | blocking | nx,bazel | unproven (none stored) |
| edit-python-resident-green-p50 | edit-python-resident | greenMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| graph-scope-warm-p50 | graph-scope-warm | wallMs.p50 | lower | 300 | 2000 | advisory | nx,bazel | unproven (none stored) |
| graph-scope-cold-max | graph-scope-cold | wallMs.max | lower | - | 300000 | advisory | nx,bazel | unproven (none stored) |
| !cache-second-run-build-min | cache-second-run-build | hitRatio.min | higher | 1 | 1 | blocking | nx,bazel | unproven (none stored) |
| !cache-second-run-test-min | cache-second-run-test | hitRatio.min | higher | 1 | 1 | blocking | nx,bazel | unproven (none stored) |
| mem-noop-run-max | mem-noop-run | peakRssBytes.max | lower | 536870912 | 1073741824 | advisory | nx,bazel | unproven (none stored) |
| mem-edit-ts-max | mem-edit-ts | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| concurrency-disjoint-wait-max | concurrency-disjoint | maxUnrelatedWaitMs.max | lower | 1000 | 5000 | advisory | nx,bazel | unproven (none stored) |
| concurrency-disjoint-makespan-max | concurrency-disjoint | makespanMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| concurrency-warm-fanout-makespan-max | concurrency-warm-fanout | makespanMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| concurrency-warm-fanout-wait-max | concurrency-warm-fanout | maxUnrelatedWaitMs.max | lower | 1000 | 5000 | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-cold-wall-max | product-dashboard-cold | wallMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-cold-rss-max | product-dashboard-cold | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-dashboard-noop-wall-p50 | product-dashboard-noop | wallMs.p50 | lower | 10000 | 10000 | blocking | nx,bazel | unproven (none stored) |
| !product-dashboard-noop-rss-max | product-dashboard-noop | peakRssBytes.max | lower | 1610612736 | 1610612736 | blocking | nx,bazel | unproven (none stored) |
| !product-dashboard-noop-executed-max | product-dashboard-noop | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| product-dashboard-noop-queue-p50 | product-dashboard-noop | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-noop-graph-p50 | product-dashboard-noop | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-noop-execute-p50 | product-dashboard-noop | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-dashboard-edit-green-p50 | product-dashboard-edit | greenMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-dashboard-edit-executed-max | product-dashboard-edit | executedActions.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-edit-rss-max | product-dashboard-edit | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-edit-queue-p50 | product-dashboard-edit | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-edit-graph-p50 | product-dashboard-edit | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-dashboard-edit-execute-p50 | product-dashboard-edit | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-unrelated-executed-max | product-dashboard-peer-unrelated | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-unrelated-graph-p50 | product-dashboard-peer-unrelated | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-dashboard-peer-unrelated-inferred-max | product-dashboard-peer-unrelated | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-unrelated-rss-max | product-dashboard-peer-unrelated | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-scheduling-executed-max | product-dashboard-peer-scheduling | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-scheduling-graph-p50 | product-dashboard-peer-scheduling | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-dashboard-peer-scheduling-inferred-max | product-dashboard-peer-scheduling | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-dashboard-peer-scheduling-rss-max | product-dashboard-peer-scheduling | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| product-course-cold-wall-max | product-course-cold | wallMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-cold-rss-max | product-course-cold | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-course-noop-wall-p50 | product-course-noop | wallMs.p50 | lower | 10000 | 10000 | blocking | nx,bazel | unproven (none stored) |
| !product-course-noop-rss-max | product-course-noop | peakRssBytes.max | lower | 1610612736 | 1610612736 | blocking | nx,bazel | unproven (none stored) |
| !product-course-noop-executed-max | product-course-noop | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| product-course-noop-queue-p50 | product-course-noop | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-noop-graph-p50 | product-course-noop | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-noop-execute-p50 | product-course-noop | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-course-edit-green-p50 | product-course-edit | greenMs.p50 | lower | 60000 | 60000 | blocking | nx,bazel | unproven (none stored) |
| product-course-edit-executed-max | product-course-edit | executedActions.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-edit-rss-max | product-course-edit | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-edit-queue-p50 | product-course-edit | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-edit-graph-p50 | product-course-edit | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-course-edit-execute-p50 | product-course-edit | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-course-peer-unrelated-executed-max | product-course-peer-unrelated | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-course-peer-unrelated-graph-p50 | product-course-peer-unrelated | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-course-peer-unrelated-inferred-max | product-course-peer-unrelated | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-course-peer-unrelated-rss-max | product-course-peer-unrelated | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| !product-course-peer-scheduling-executed-max | product-course-peer-scheduling | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-course-peer-scheduling-graph-p50 | product-course-peer-scheduling | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-course-peer-scheduling-inferred-max | product-course-peer-scheduling | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-course-peer-scheduling-rss-max | product-course-peer-scheduling | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| product-print-cold-wall-max | product-print-cold | wallMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-cold-rss-max | product-print-cold | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-print-noop-wall-p50 | product-print-noop | wallMs.p50 | lower | 10000 | 10000 | blocking | nx,bazel | unproven (none stored) |
| !product-print-noop-rss-max | product-print-noop | peakRssBytes.max | lower | 1610612736 | 1610612736 | blocking | nx,bazel | unproven (none stored) |
| !product-print-noop-executed-max | product-print-noop | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| product-print-noop-queue-p50 | product-print-noop | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-noop-graph-p50 | product-print-noop | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-noop-execute-p50 | product-print-noop | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-print-edit-green-p50 | product-print-edit | greenMs.p50 | lower | 60000 | 60000 | blocking | nx,bazel | unproven (none stored) |
| product-print-edit-executed-max | product-print-edit | executedActions.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-edit-rss-max | product-print-edit | peakRssBytes.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-edit-queue-p50 | product-print-edit | queueMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-edit-graph-p50 | product-print-edit | graphMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-print-edit-execute-p50 | product-print-edit | executeMs.p50 | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-print-peer-unrelated-executed-max | product-print-peer-unrelated | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-print-peer-unrelated-graph-p50 | product-print-peer-unrelated | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-print-peer-unrelated-inferred-max | product-print-peer-unrelated | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-print-peer-unrelated-rss-max | product-print-peer-unrelated | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| !product-print-peer-scheduling-executed-max | product-print-peer-scheduling | executedActions.max | lower | 0 | 0 | blocking | nx,bazel | unproven (none stored) |
| !product-print-peer-scheduling-graph-p50 | product-print-peer-scheduling | graphMs.p50 | lower | 120000 | 120000 | blocking | nx,bazel | unproven (none stored) |
| product-print-peer-scheduling-inferred-max | product-print-peer-scheduling | inferredProjects.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| !product-print-peer-scheduling-rss-max | product-print-peer-scheduling | peakRssBytes.max | lower | 2147483648 | 2147483648 | blocking | nx,bazel | unproven (none stored) |
| product-agents-makespan-max | product-agents | makespanMs.max | lower | - | - | advisory | nx,bazel | unproven (none stored) |
| product-agents-wait-max | product-agents | maxUnrelatedWaitMs.max | lower | 1000 | 5000 | advisory | nx,bazel | unproven (none stored) |
| product-agents-speedup-min | product-agents | speedup.min | higher | - | - | advisory | nx,bazel | unproven (none stored) |
| planning-cold-wall-max | planning-cold | wallMs.max | lower | - | 120000 | advisory | nx | unproven (none stored) |
| planning-cold-inferred-max | planning-cold | inferredProjects.max | lower | - | 500 | advisory | nx | unproven (none stored) |
| !planning-warm-wall-p50 | planning-warm | wallMs.p50 | lower | 2000 | 10000 | blocking | nx | unproven (none stored) |
| !planning-leaf-edit-wall-p50 | planning-leaf-edit | wallMs.p50 | lower | 3000 | 10000 | blocking | nx | unproven (none stored) |
| planning-leaf-edit-closures-max | planning-leaf-edit | closuresVerified.max | lower | - | 200 | advisory | nx | unproven (none stored) |
| planning-manifest-edit-wall-p50 | planning-manifest-edit | wallMs.p50 | lower | 10000 | 60000 | advisory | nx | unproven (none stored) |
| planning-two-wall-max | planning-two | wallMs.max | lower | - | 30000 | advisory | nx | unproven (none stored) |
| !planning-two-rss-max | planning-two | peakRssBytes.max | lower | 2147483648 | 3221225472 | blocking | nx | unproven (none stored) |
| !planning-warm-rss-max | planning-warm | peakRssBytes.max | lower | 2147483648 | 3221225472 | blocking | nx | unproven (none stored) |
| !planning-two-oom-max | planning-two | oomKills.max | lower | 0 | 0 | blocking | nx | unproven (none stored) |
| !planning-warm-oom-max | planning-warm | oomKills.max | lower | 0 | 0 | blocking | nx | unproven (none stored) |
| planning-warm-swap-max | planning-warm | swapBytes.max | lower | 0 | 2147483648 | advisory | nx | unproven (none stored) |
| !planning-warm-store-max | planning-warm | storeBytes.max | lower | 21474836480 | 53687091200 | blocking | nx | unproven (none stored) |

## 5. Proposed definition of done for wave 1122 (objectives that must pass)

Each item must be measured with a stored report of the coordinator host class (one `bench run --history=…`, then `bench compare`), and the value must be recorded in the wave result.

1. `cache-second-run-build-min` and `cache-second-run-test-min`: hit ratio 1 (blocking).
2. `product-dashboard-noop-executed-max`, `product-course-noop-executed-max`, `product-print-noop-executed-max`: 0 re-executed actions (blocking).
3. All `*-peer-unrelated-executed-max` and `*-peer-scheduling-executed-max` (six): 0 (blocking). These are the "nothing computed twice" proofs for peer edits.
4. `product-dashboard-noop-wall-p50`, `product-course-noop-wall-p50`, `product-print-noop-wall-p50`: 10 s gate (blocking), and the three `*-noop-rss-max` at 1.5 GiB (blocking) for the 16 GB class.
5. `product-dashboard-edit-green-p50` (120 s), `product-course-edit-green-p50` and `product-print-edit-green-p50` (60 s) (blocking).
6. `*-peer-*-rss-max` at 2 GiB (blocking), and `*-peer-*-graph-p50` at 120 s (blocking).
7. `planning-two-oom-max`, `planning-warm-oom-max` = 0 (blocking); `planning-two-rss-max`, `planning-warm-rss-max` at 2 GiB goal / 3 GiB gate (blocking); `planning-warm-store-max` (blocking).
8. `concurrency-disjoint-wait-max` and `concurrency-warm-fanout-wait-max` at 1 s goal (advisory today; promote to blocking for this wave).
9. Conformance: all 21 scenarios must pass on the semio backend (native in the list today) and on nx and bazel where they are native; the adapter claims for nx and bazel (for example `single-flight` on nx, `progress` on nx and bazel) must be proven or removed from the claim.
10. NEW objectives required before the wave can pass (not in the file today): remote cache hit rate on a fresh checkout of a second machine (goal at least 0.9); zero cross-machine output divergences for one action key (goal 0); remote execution: offload accepted for an idle peer while the local host is above its budget (goal: offloaded task completes); a worker refusing with `no-headroom` while an idle peer exists must be a failure; a remote part down must fall back to local within the timeout (no hang).
11. Every worker in the fleet (13) reaches Ready on its platform (cross-check with `WORKERS-READY-WITH-LOW-RESOURCES` ticket).

Not verified by this audit: all measured values, the enforcement wiring in CI, the remote cache internals and remote execution internals.
