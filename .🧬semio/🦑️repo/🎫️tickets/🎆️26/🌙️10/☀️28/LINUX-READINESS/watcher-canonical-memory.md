# Canonical Watcher Memory and Progress Handoff

The retained canonical watcher job is advancing, not demonstrated stalled. No cancellation, replacement job, budget increase, memory-floor change or guard bypass was performed.

Exact custody: original PID 1649144, captured kernel child 1832522, active graph child 1832975. Root argv remains repo run repo:test-nx-output-observation --parallel=1. Full samples are generated/watcher-canonical-sample1.json and sample2.json.

| UTC | Project Progress | Active Child RSS | HWM | Swap | Host Load 1/5/15 |
| --- | --- | --- | --- | --- | --- |
| 23:15:48.907 | 269/515, one project running | 2,543,128 KiB | 2,563,172 KiB | 301,616 KiB | 22.22 / 45.06 / 30.26 |
| 23:16:58.424 | 407/515 | 2,786,940 KiB | 2,919,944 KiB | 326,696 KiB | 11.08 / 36.85 / 28.48 |

Over 69.52 seconds, the active child consumed 5870 user and 1655 system CPU ticks, advanced 138 projects, added 29,480 log bytes, read about 150 MB through character IO and about 107 MB physically. It remained runnable; parents waited in epoll. This is genuine work. Prior graph verification reached 781/851, then the job entered a further inference/planning phase; no terminal target result is inferred.

The local guard repeatedly reported roughly 2388–2460 MB in use against its declared 1792 MB admission threshold, with only the already running project admitted. The guard pauses additional project admission; it has not produced a terminal refusal in these samples. RSS continued increasing while that project/graph work progressed. Do not change the budget to label this healthy.

smaps_rollup at sample2 attributes 2,753,340 KiB to private anonymous memory and only 3,580 KiB file PSS, with 326,696 KiB swap. Thus the large resident set is primarily anonymous/heap storage rather than file mappings. It is not yet an identified JavaScript object leak.

Read-only Source audit identifies concrete retention paths for further causal profiling:
- script-inputs createScriptInputCache retains strong files and closures Maps for a whole graph construction.
- boundedScriptInputsV1 stores observed import results per physical path, required-Source mode, provider designation, whole resource-set digest and router selectors. Distinct target resource sets can retain separate records for a shared physical module. Closure results retain compiled-name sets and observed evidence until graph scope ends.
- TypeScript program reuse, when installed by its deriver owner, has strong programs/parsed Maps keyed by configuration. Its end method prunes unused file keys only for configurations used by the question, and does not remove all older configuration/program entries. Whether that reuse owner is active in this exact child still needs confirmation; it is a candidate, not proven heap attribution.

No heap snapshot, signal, debugger attachment or Source modification was used. A later fix should use owned diagnostics and a neutral causal retention test before changing cache policy; retain Original-bound observation evidence and correct invalidation. The current progressing operation remains under original custody.

The isolated actual-composition attempt watcher-actual-composition separately exited 1 at 23:13:47.830 during fresh private rustup sealing with Original Script deadline refused, before compilation or service startup. No claim of composition-ready success is made. The supported genuine birth has a 290000 ms process maximum and 120000 ms child test timeout; the handcrafted private invocation requested 180000 ms. No budget was enlarged after that refusal. Earlier real d428 cold 18/245 and full public watcher 48/477 results remain distinct valid receipts.

Current d428 full service file exited 0 at 23:03:00.791, 35 tests/756 assertions, with its authored optional service-process crash scenario skipped because the independent first-party NATIVE/faketool fixture was absent. Native watcher parent-kill/orphan coverage did run. Root requires legitimate zero-touch native fixture preparation and full authored crash execution next, after current checkpoint, without manual environment authority or a fake tools owner.

## Recovered Terminal Receipt and New Boot

The durable watcher-physical-integration.outcome.json was recovered after reboot: status1/null, PID1649144, ended2026-10-29T23:19:00.073Z. Matching log ends with graph-plan SIGKILL at463/515 projects and memory3755–3806MB against1792MB, then parent repository:command1. This is a real preboot terminal failure before target tests; SIGKILL origin is unproven. Old resident PID1764191 has no receipt, so its reboot-lost outcome remains unknown. New boot11220615 was revalidated23:58:17Z; both old PIDs absent. No manual signals, lease pruning or duplicate restoration was performed. See mcp-staging-review.md for independent current startup review.
