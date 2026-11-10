# Bench lane: planning measurement harness and baseline

## What exists

- `repo bench list|run|compare|acceptance` (impl: `🏗️build/📏️incremental/🏎️bench/`) measures SLO probes in a disposable copy and acceptance scenarios (noop/peer/edit) of `repo run`. It has no phase timings, closure counts, host RSS/swap/OOM or store size, and every probe pays a full `repo run`.
- Reused from it: `acceptanceLineV1/MutateV1/RestoreV1` (byte-exact one-line edit and revert), the same schema-first layout (schema JSON, scenario JSON, TS module, TS tests + Python oracle).
- New sibling module `🏗️build/📏️incremental/🏎️bench/🧭️planning/` (standalone runner and log parser) and, wired into `repo bench` itself, a probe kind `planning` (see "Wired into repo bench").

## Files

- `🧭️planning/🧬️schema/🔣️.json` scenarios + sample + report schema (draft-07).
- `🧭️planning/🎯️scenarios/🔣️.json` scenarios (a)-(g), phase markers, counters, store homes (data, not code).
- `🧭️planning/🟦️.ts` observe (lines -> phases/counters), median, OOM count, markdown render EN/DE.
- `🧭️planning/🏃️runner/🟦️.ts` runner: `bun run <runner> --out=<dir> --scenario=<ids|all> [--gate=<build-slot.sh>]`; samples host RSS (sum of repo/bun/cargo/rustc/bazel/nx processes via `ps`), `/proc/meminfo` available + swap, `journalctl -k` OOM kills, `du` of stores; restores edits byte-exact; cancellable via SIGINT.
- `🧭️planning/🧪️tests/` fixtures from real recorded logs + independent Python oracle.

## Scenarios

| id | what | cmd |
|---|---|---|
| cold-plan | (a) cold graph plan of `@semio-tech/norm-en1990-rs:build` | `repo run <t> --dry-run` |
| warm-noop | (b) warm no-op re-plan (1 prime + 3 runs) | same |
| leaf-edit | (c) one comment line appended to a leaf `.rs` of en1990 | same |
| shared-manifest-edit | (d) one comment line in `🧰️framework/Cargo.toml` | same |
| two-planners | (e) en1990 + din16798 planned concurrently (needs `SEMIO_FLEET_SLOTS=2` through the gate) | same x2 |
| host-memory | (f) repo-process RSS sum, MemAvailable, swap, OOM | sampler |
| store-disk | (g) `du` of `.🧬semio/🦑️repo/⚡️cache`, `.cache/b` | du |

Phases come from log markers in order: facts (start to first `refreshing`/`stale facts`), verification (to inference start), inference (worker/host asks), planning (from `generation N published`/`is current`/`sealed by a peer` to exit; includes the `derived results asked again` closure re-verification). Closures verified/reused/changed come from `N derived results asked again after M stale facts: R unchanged, C changed`.

## Verification

`repo --service=off tools exec --light bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🧭️planning/🧪️tests/🟦️.ts` : 6 pass, 0 fail, 41 expects (schema accept/reject with Ajv, phase+counter parity with Python oracle, report validation, EN/DE render, median, OOM). Runner smoke: `--scenario=host-memory,store-disk` ran.

## Baseline (2026-11-09, loaded host: 12 cores, load ~9, 15.8 GB, swap 7.6/8.9 GB in use)

Host (f)/(g), measured now: repo-related process RSS sum 3.6 GB (one 3.0 GB bun planner/daemon), MemAvailable 6.0 GB, swap used 7.6 GB, store+cache 77 GB on disk.
OOM kills in the last 2 days (journalctl -k): 3 x bun (anon-rss up to 10.8 GB each) + 1 python cgroup kill; the bun kills were at 07:01-07:03 Nov 8.

Historical plans (parsed from NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated logs, 170 graph publishes):

| metric | value |
|---|---|
| projects refreshed per publish: wall s | min 32, median 156, p90 386, max 1596 |
| example 42 projects after 18 stale facts | 722 s (rt-main-2) |
| example 1631 refreshed after 13 stale facts | 1117 s (rt-main-3); 8 worker restarts |
| closure re-verification | typical 1-65 asked, all unchanged; wide edit: 1052 asked, 1051 reused/1 changed; 2975 stale facts: 34 asked, 33 changed |
| worker restart RSS | n=114, median 1236 MB, max 2482 MB (budget 2048 MB/worker) |
| peer graph-build queue wait | n=41, median 95 s, max 801 s |
| waiting-for-memory lines | 2229 |
| fact index | 664k-675k facts over 4933 projects, 59-61 MB expanded |
| sample stale causes | `norm/Cargo.toml` file fact stales the norm projects; framework routing `.ts` analysis fact stales cad-js; policy.json stales everything |

Notable: even `repo show project` waited for capacity (needs 6.9 GB; floor 1.98 GB) because the host was above the swap ceiling; a read-only query is therefore not cheap under load.

Dry-run baselines (a)-(e) under the gate: cold-plan queued behind the slot owner (`framework-plugin:check`, `norm-artifact-contract-rs:test`); see the "Pending" section below for its result if it completed.

## Success-metric targets to compare against

no-op re-plan after a one-file edit: seconds (today median publish 156 s); planner/daemon peak RSS <= 3 GB (today a 3.0 GB planner plus up to 10.8 GB OOM-killed bun); zero OOM kills (today 3 in 2 days); store capped (today 77 GB).

## Follow-ups

- Windows: RSS/swap/journal sampling returns null there (cross-platform degrade), store size needs a non-`du` walk.

## Wired into repo bench

- Schema (`🏎️bench/🧬️schema/🔣️.json`): kind `planning`; metrics `closuresVerified`, `closuresReused`, `staleFacts`, `swapBytes`, `availableBytes`, `oomKills`, `storeBytes` (peakRssBytes now also a plain planning metric = sum of all repo-related processes); probe field `peer` (second concurrent planner); observation pattern `graphClosures`.
- SLO table (`🏎️bench/🎯️slo/🔣️.json`): units `norm-leaf`, `framework-manifest`; probes `planning-cold|warm|leaf-edit|manifest-edit|two`; 13 objectives. Blocking: warm and leaf-edit re-plan p50 gate 10 s (goal 2-3 s), repo-process RSS <= 3 GiB (goal 2 GiB), OOM kills 0, store+cache <= 50 GiB (goal 20). Advisory: cold plan wall <= 120 s and re-inferred projects <= 500 (bounded by closure), closures verified after a leaf edit <= 200, manifest-edit re-plan <= 60 s, two planners <= 30 s, swap <= 2 GiB. `bench_compare` flags regressions through the per-objective tolerances.
- Code: `🏎️bench/🟦️.ts` (kinds, validation, observer closures, formatting, selector), `🏎️bench/🏃️runner/🟦️.ts` (planning dry-run args, concurrent peer, resource port using `🧭️planning/🏃️runner`), operations catalog + help fixtures (EN+DE `--slo` text lists `planning`).
- Selecting: `repo bench run --slo=planning/leaf-edit` (also `planning/warm|cold|manifest-edit|two`, `kind:planning`, `probe:planning-<n>`); `bench_run`/`bench_compare` take the same `--slo` and report files.
- Tests: `🏎️bench/🧪️tests/🟦️.ts` 45 pass (new: planning table admitted, selector, objective gates, closure observation); `🧭️planning/🧪️tests/🟦️.ts` 6 pass. `operations` tests 15/15 pass after the fixes below; `repo bench list` now runs live and prints the 13 planning objectives.

## Live crash root causes

- The ENOENT in the bootstrap authorizer was the capture reading a source tree that other lanes were moving at that moment: first `🕸️graph/📥️load/🟦️.ts` (just split out of `🕸️graph/🟦️.ts`) kept dynamic imports `./📥️inference/...` and `../../../🧪️test/...` that no longer resolve from the new folder (fixed to `../📥️inference/...` and `../../../../🧪️test/...`); later transient missing folders (`📏️headroom/🧾️closures/🗂️members`, `build/⚡️caching`) appeared mid-move and were gone on retry.
- Operations tests: help snapshot regenerated from the renderer with `TICKET/🧰️scripts/regen-help-snapshot.ts` (adds `cache grow` and the scheduler rows already in the catalogue); the spawned-CLI test now passes `SEMIO_LANGUAGE=en` explicitly (no default language added).
- Open, not mine: MCP build-surface oracle fails because the scheduler lane's `scheduler status|cancel|run` rows are neither exposed nor excluded.
- `bench run`/`compare` end to end: a disposable-copy run would cold-build the whole graph (269k tracked files) on a host with swap nearly full, so it is not run; instead the new test runs the planning probes through the real probe runner with scripted ports (dry-run args, concurrent peer, byte-exact edit restore, closure and resource metrics), and the existing stand-in-CLI tests cover run, baseline and compare over the full table.

## Cold-plan baseline (queued dry-run through the gate)

exit 1, wall 146 ms, phases {'facts': 146, 'verification': 0, 'inference': 0, 'planning': 0}, closures {'verified': None, 'reused': None, 'changed': None}, peak repo RSS 9895 MB, swap 8948 MB, min available 889 MB, OOM kills 48
