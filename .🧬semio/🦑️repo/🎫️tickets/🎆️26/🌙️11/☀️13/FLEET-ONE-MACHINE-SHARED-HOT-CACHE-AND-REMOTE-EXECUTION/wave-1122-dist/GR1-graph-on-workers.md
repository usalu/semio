# GR1: why the build graph blocks workers (read-only audit, 2026-11-23 about 15:00 Mac clock)

No file was edited. No build, read or refresh was started. Only read-only commands were run (`repo fleet status`, file reads, `ps`, `grep`). Graph code paths are relative to `G` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/`.

## Verdict

1. A command never needs the whole graph. It needs a selected owner closure (607 owners for the Ready `workspace` stage, 949 admitted for the layout target, about 2,300 by the final closure). The whole graph (5,097 stale of 5,222 owners, generation 261) is needed only to seal a generation, and the seal has not completed since 2026-11-20.
2. Workers infer their own selection from zero because the only machine that may serve the shared graph is the 32 GB coordinator (`serveTotalMegabytes` 24,000), and a worker can adopt a published head only when it is already Ready. Ready needs the `workspace` stage, which needs the graph. This is a circular dependency, and no head is published ("Generation: none published").
3. The coordinator cannot seal in a reasonable time. Its single whole read is at 98 of 128 wave-1 owners for a long time, the owner tail is cold Rust and cargo-plugin owners at 15 to 35 min each under load 50 to 60, and the stale verifier marks whole closure rows (`closure observations`) that cannot name the moved file.
4. Ranked fixes: (i) coordinator seals and workers fetch the sealed selected slice by content key, after relaxing the self-Ready precondition of head adoption (largest effect, medium risk); (iii) derive the Rust closure from Cargo.toml path dependencies only for `repo cargo` and the semio-tech build (already implemented as `nativeDependencyRoots`, verified against an independent script in E33 round 2); (ii) per-file closure rows, so one crate edit re-checks one row, not the whole closure; (iv) memory-bounded streaming, which fixes OOM but not the single-thread time.

## 1. What a command needs from the graph

| command | graph need | blocks until finished |
|---|---|---|
| (a) `repo run <project>:<target>` | `selectBuildTasksV1` plus `closeBuildOwnerAdmissionV1` (`G/🚪️admission/🟦️.ts`) admit owners by the closure of the seeds: selection, task, caret, dependency, file and input edges. Only this selected set is inferred, in the process that runs the command (`G/📥️load/🟦️.ts`, `G/📥️load/🤝️selected-exchange/🟦️.ts`). | Yes. The command waits for the selected read; a clean dry run was 765 s, of which the slice was 160 s (E63). |
| (b) `repo cargo` | Before cargo starts it captures the tool's own source closure (`source.capture`, 136 s with a stale memo) and spends 286 s in the "generation scope" stage (C2). Cargo itself took 213 s in `metadata --no-deps` under load. Whether the generation scope calls the selected graph read was not traced line by line. | Yes, in the generation scope stage. |
| (c) Ready `workspace` stage | `repo run-many norm-din4108,norm-din4108-rs,repo-mcp --targets=build` (W1-ready). It runs a selected read of 607 owners (`ready.json` tail on m02 and m07), then the cargo units. One hour timeout. | Yes. On m01 the read waits in host capacity: 6,553 MB requested, minimum 2,048 MB, 763 MB above the 2,048 MB floor, queued 267 s. |
| (d) semio-tech play build | The layout closure is selected, not whole: 943 `file` edges are the `Cargo.toml` of the native dependency closure (E33 round 2: 861 crates for the layout component, 450 for node-graph, 717 for renderer-wgpu, union about 995 crates). The play release runs from `/Users/ueli/.sinc/play` through the same selected read. | Yes, the selected read blocks it. The play release is not in the graph cost path itself. |

The whole-graph refresh (`refreshing 5097 projects (generation 261)`, reason `refresh`) is different: it is one detached runner (`read-record.json`, `G/../serve`), started by the sidecar's `mayRefresh` (one claim per platform per 10 minutes, `G/../../../../../🌐️remote-execution/🛰️sidecar/🕸️graph/🟦️.ts:89`). While it holds graph custody exclusive, selected reads queue behind it (E33: custody held 6,800 s by `show projects`).

Owner counts: whole graph 5,192 to 5,222 owners (phases `owners 112/5222`); coordinator wave 1 = 128 owners (`owners 98 of 128`); workspace stage = 607; layout target = 949 admitted (trace), about 2,300 by the final closure (E33 round 2).

## 2. Cost model (measured)

Per owner, warm: probe 5 ms, path-dependency crate 121 ms, full re-check 1.5 s (E40). Cold: first TypeScript owner 15.7 s, then 1.1 s (E1 round 7); cold Rust owner 22 to 70 s CPU (E42); lean/cargo-plugin owner 52 to 58 s CPU and 150 to 280 s wall in isolation (E80), but 1,700 to 2,105 s wall in a loaded read (E80, E54: 15 to 25 min each).

What is inferred per owner (`G/📥️inference/`):
- TypeScript: import-closure walk and `typescriptText`/`analysis` rows (`📦️project-nodes`, `🧬️source-analyses`).
- Rust: the native path-dependency closure, `exists`/`field` rows per crate `Cargo.toml` (`🦀️native-manifests`, `nativeDependencyRoots`). A native closure can hold 300 to 861 crates.
- Every row is re-verified on every lookup: m02 reports 1,041,152 verified memo rows and 26,743 completed owner checks in 8,765 s; observation row verification alone was 14.5 percent of the isolated profile (E80).
- Per owner: `readdir`, `stat`, `closure observations` (one whole row), `global` (354 registry rows per owner, E62).

Memory: the whole-read heavy child peaked at 9.7, 11.8 and 10.8 GB in three OOMs (13:28, 14:18, 15:45). The cause was the `stream-patch` append that kept every row object (E54, 15:56), fixed by log-backed append; not yet proven by a completed seal. Current coordinator read: heavy child 2.15 to 2.2 GB against a 3,072 MB grant, 0 OOM lines in the current `read.log`. Plan budget (`G/🧠️memory/🪜️plan-stage/🟨️.mjs`, `PLAN_STAGE_POLICY_V1`): parent 3,600 MB, farm child typical 900, cap 1,400, alone 4,200 MB; availability floor 4,096 MB; minimum 2,048; maximum 16,384. A project needs about 1,609 MB (E45). Worker peaks in `ready.json`: m01 9,392 MB, m02 5,492 MB, m07 10,662 MB.

Why 16 GB machines thrash: a 16 GB Mac has about 2.8 GB above its floor under load 50 to 60 on 10 cores (`repo fleet status`: 14.2 of 32.7 GB free on the coordinator, but workers refuse with "no headroom (memory)"). The plan then runs "inferring N projects in this process (memory)" with no farm, so one JavaScript thread time-slices 4 to 8 owners at about 40 percent CPU (E45 sample), the heap sits at its grant, and each cargo owner finishes after more than 14 min or is killed by the 25 min stall rule (`inferStallMilliseconds` 1,500,000 ms). The Rust cap is 2 owners at a time (`kindLimits language:rust 2`, E45 round 2). The current read logs "headroom of 577 MB is below one project (902 MB), inferring one project at a time".

## 3. What is shared, and why workers still infer

Shared today:
- Owner records per owner in `.cache/b/g/m/inference/selections` (per-owner keys, E1 round 3). A record is taken only when every row still verifies (`G/📥️inference/📦️project-nodes/♻️taken-memo`, E80/E81).
- Fleet fetch of owner records: on the coordinator read 40690 there are 87 present, 0 fetched from the fleet, 11 inferred here, and 30 wanted from the fleet. Earlier reads fetched 15 to 39 records from the fleet (E45, E54), so the path works when the coordinator serves.
- Remote exchange `G/📥️load/🌐️remote-exchange` and selected-exchange `G/📥️load/🤝️selected-exchange/🟦️.ts` (`lookSelectedGenerationV1` line 109, `acceptSelectedOwnersV1` line 130).
- Cache server `main/ac` and `main/nx` (`.cache/b/p/m/fleet/cache-server/v1`): a sample of 31 `ac` files and 50 `nx` files contained no `project-inference` kind. I could not confirm that such entries exist; this is not verified.

Why workers still infer for hours:
1. Only machines with `totalMegabytes >= serveTotalMegabytes` (24,000) serve wanted records. `mayServe` (`G/../../…/🕸️graph/🟦️.ts:87`) and `graphServerHeadroomV1` (line 42 to 43) return zero headroom below that. Only the coordinator qualifies, and it is busy running its own whole read. Its wanted queue (30 owners) is served by the same process that is stuck at 98 of 128. Fleet status: "Graph: no builder; 0 computer(s), 0 waiting" and "Generation: none published".
2. Head adoption (`G/../../…/📥️adoption/🟦️.ts`, lines 188 to 198) requires: host total at least 16,384 and below 24,000, pressure normal, free at least floor plus 6,144, no waiters, a live head, and `readyForHeadAdoptionV1` (lines 95 to 115), which needs the worker's own `ready` state with `stage === null`. A worker reaches Ready only after its `workspace` stage, which is the read it wants to skip. No head is published, so nothing can be adopted. This is the circularity.
3. Stale records. The eight `[graph-stale]` events in the current read all have kind `whole`, path `closure observations`, why `closure-probe`, and `current = "-"` (not askable). So an edit anywhere in a Rust closure invalidates the whole owner, and the verifier cannot say which file moved. The owners named are `norm-energy-plugin`, `norm-eurocode-materials-plugin`, `norm-eurocode-plugin`, `block-2d-rs`, `block-catalog-rs`, `norm-eurocode-structural-plugin`, `semio-hub-norm-eurocode-materials-component`, `abi-rs`.
4. `workspace is stale` churn: 32 lines in the current log, 26 of them from `units/sealed/<hash>/…` (sealed copies of runtime sources, which enter the workspace roster when a new sealed copy appears; E81 round 2 found the same).
5. Peers' landings no longer restart a running slice (E63 round 2, `finishOnLoadedImplementationV1`). Landings still move rows, which is the stale-record path above.
6. Build-owner registry (`📋️build-owners.json`, 27 edits a day, E55): E62 turned the registry reads into one row per question, so one edit moves one row (not the main cause now).
7. Per-machine keys: the platform key is shared by the macOS workers (`macos-arm64-312cc5b2932fbb31`), so the records can be shared. Rust unit keys still differ on `CARGO_ENCODED_RUSTFLAGS` (`-Zthreads=0` repeated) between m02 and m07, so compiled units are not shared (fleet finding). This is a separate defect, in the unit cache.

## 4. Live state (read-only)

Coordinator (`g1-server`):
- Read `read-record.json`: runner 40687, read 40690, reason `refresh`, started 1795437598845 ms, last beat 4,878 s later. `serve-state.json`: reading `refresh`.
- Phases at last checkpoint (`read-checkpoint.json`): closures 5,101/5,101, membership 127,831, facts 841,640, decisions 3,937/3,937, owners 112/5,222, inference 98/128, shared closures 49/49, ledger 98/128.
- `read.log`: `owners 98 of 128 done: 87 present, 0 fetched from the fleet, 11 inferred here (6 to 8 running), 30 wanted from the fleet, 2 stale in the fleet`. The count has sat at 98 for six consecutive log lines. Heavy child 2.15 to 2.2 GB of 3,072 MB (`graph: the process held … collected and shrunk`).
- `repo fleet status`: "Graph: no builder; 0 computer(s), 0 waiting"; run 34126 `show plan`, 81 min, 3,072/2,613 MB; "Generation: none published"; host 14,239 of 32,768 MB free; load 59.34; swap 1,775 MB.
- `current.json`: generation 260, manifest null, reason `closure-missing`, written 2026-11-20 03:59 (not sealed since).

Workers (`/Users/Shared/semio-store/v1/origins/<origin>/ready.json`; there are no separate graph progress files per origin, progress is in `tail` and `slice-memory.json`):

| origin | state | stage, attempt | reason / refusal | peak MB | slice tail (latest) | Ready lost (age) |
|---|---|---|---|---|---|---|
| g1-m01 | running | workspace 5/5, attempt 5 | stalled 10 min, restarted | 9,392 | 271 s queued for host capacity (6,553 MB requested, minimum 2,048) | 21.2 h |
| g1-m02 | running | workspace 5/5, attempt 4 | adopted, not restarted | 5,492 | slice 8,765 s; 86,795 source observations, 26,743 owner checks, 1,041,152 memo rows, 607 owners admitted | 11.1 h |
| g1-m07 | running | workspace 5/5, attempt 2 | stalled 10 min | 10,662 | slice 869 s; 216,031 observations, 3,847 owner checks, 931,264 memo rows, 607 owners | 5.4 d |
| g1-m12 | waiting | workspace 5/5, attempt 46, exit 1 | disk-collecting: needs 4,096 MB, 451 MB free | 6,163 | closure-read TypeError (W1 finding c) | 30.2 h |

`slice-memory.json` of m07 at 15:01: phase "checking the file inventory (216,031 observations, 3,910 owner checks, 931,264 memo rows, 607 owners admitted)", RSS 2,474 MB, heap 1,274 MB, guard 1,981 MB, no child processes. Fleet status also shows m06 and m11 Ready, m03 in tools, m04 in dependencies (disk short, 3.5 GiB free), m05 Windows idle behind a lease, m08 and m10 and m13 in workspace, m09 with no record.

## 5. Options, ranked

| # | option | effect | risk | what changes |
|---|---|---|---|---|
| i | Coordinator seals; workers fetch the sealed selected slice by content key and never infer when a head or record matches | Removes the 2 to 3 h worker slice reads (m02, m07). Workspace stage becomes fetch plus cargo | Coordinator is a single point; seal must work first (it does not yet). Trust: the workers' synced tree is identical, so keys bind | Relax `readyForHeadAdoptionV1` (`📥️adoption/🟦️.ts:95-115`) so adoption does not need the worker's own Ready; lower head adoption threshold on 16 GB (`📥️adoption/🟦️.ts:188-198`); make `lookSelectedGenerationV1` fetch-first (`📥️load/🤝️selected-exchange/🟦️.ts:109`) with a bounded wait then infer; `G/🪪️graph-state/🙋️wanted/🛠️server` serve path |
| ii | Per-file closure rows instead of one `closure observations` row | One crate edit re-checks one row and names the file in `[graph-stale]`. Cargo owners are not dropped by a whole-closure move | More rows per owner (300 to 860 per Rust owner); memo size | Split `closure` rows in `G/📥️inference/🧭️script-inputs/🟨️.mjs` (around line 1870 `closure-probe`) and `G/📥️inference/📦️project-nodes`; keep the stamp memo (E35) |
| iii | `repo cargo` and the semio-tech build derive their closure from Cargo.toml path dependencies alone (no Nx-style project graph, no TS walk) | Removes the TS owner work from Rust-only commands; `nativeDependencyRoots` matched an independent script to within one crate (E33 round 2) | Misses dynamic edges (E33: `file` edges and generated outputs must stay); cargo metadata is 213 s under load if used directly | `G/🦀️rust/🟦️.ts` (`selectedCargoBuildOwnerV1`, `selectedRustAggregateTargetV1`), `G/📥️inference/🦀️native-manifests/🟨️.mjs`, `G/🧭️selection/📥️closure/🟦️.ts` |
| iv | Memory-bounded streaming: owner records verified in a child, cap per process by measured peak, never one heap for a whole wave | Removes OOM and thrash (the 10.8 GB peak is already gone after the stream-patch fix). Does not fix the single-thread cold cost | Low; more processes compete on a loaded host | `G/🧠️memory/🪜️plan-stage/🟨️.mjs` (`planFarmWorkersV1` line 93, `PLAN_STAGE_POLICY_V1`), `G/🌊️stream-patch` |
| v | Lower `serveTotalMegabytes` to 16,000 so 16 GB machines serve each other (E45 proposal) | Adds serving capacity beside the coordinator; a second source of wanted records | A 16 GB machine has about 2.8 GB spare under load and would OOM under a whole read | `G/../../…/🕸️graph/🟦️.ts` policy and the schema under `🪪️graph-state/🙋️wanted` |

Recommended order: (i) with its head-adoption fix, because it removes the circularity and the workers' 2 to 3 h reads; then (ii) so the coordinator's own stale-record churn stops re-inferring cargo owners for every landing; (iii) for `repo cargo` and the semio-tech build, which do not need the TypeScript owners.

## Not verified

- Whether `repo cargo`'s 286 s "generation scope" calls the selected graph read directly (not traced).
- Whether the play release's selection is whole or selected at the coordinator (only the layout selection was measured).
- The existence of `project-inference` entries on the cache server (0 hits in the 31 `ac` and 50 `nx` files sampled).
- That the 24 GB clamp is the only reason no worker serves; the fleet had no serve attempt logged from a 16 GB machine.
- The post-fix seal (log-backed append, 15:52): no completed seal since.
- The exact owners that move under `units/sealed` churn (only the stale-line count was taken).

No files were changed. No lock was taken.
