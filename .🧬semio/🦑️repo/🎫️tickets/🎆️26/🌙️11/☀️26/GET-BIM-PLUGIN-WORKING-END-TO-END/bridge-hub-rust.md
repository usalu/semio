# Bridge and hub Rust (bridge-hub agent) - interim

## Findings
- Bare `repo cargo ... -- check` inside the bridge dir fails with "Unknown current repository Cargo package: semio-bim-mutation-bridge": the bridge is an own workspace without `workspace.metadata.semio.repository`, so the repo's member inventory lists it with `packages: []` and name lookup by cwd fails. The framework's own bridge runner (`runRepositoryMutationBridgeV1`) always passes `--manifest-path <abs>`, which works; that is the canonical invocation and was used. Same shape as the block bridge (not a bim-specific defect); no code change made.
- Native `cargo` is refused by the repo's compiler ingress guard, so repo cargo is the only path.
- `📜️script.ts list-mutations` cannot be run standalone (needs a parent script invocation); it is run via the framework.
- run3 (cold build) was SIGTERMed by the host warden under memory pressure and hit the catalog-entities admission bug (see admission.md); rerun as run4 after the admission fix.

## Status
- run4: `repo cargo --store=shared --configuration=prototype -- run --quiet --offline --manifest-path <bridge>/Cargo.toml --bin semio-bim-mutation-bridge -- list-mutations s.bim.model 1 any` in progress. Hub targets not yet run.
- Update: after ~86 min the cold shared-store build for run4 was still compiling with 0 compile errors, throttled by host memory pressure (capacity waits, warden guard). Log: `🗑️generated/bridge-hub-run4.log` ends with an `exit N` line when done. Hub targets (`repo run @semio-tech/bim-plugin:test`, `@semio-tech/stdio-bim-plugin:test`) are NOT yet run: they need the same shared build plus the BIM guest component (`📜️script.ts test` calls `requireBimGuest`).
- Open: list-mutations answer unconfirmed; hub check/tests and committed-descriptor test not run; no files edited.

## run4 result: FAILED (exit 1) - framework compile error, not bridge code
`error[E0599]: no method named next_close_depth_demand found for struct ReplayMessageAccumulator` at `semio-os-store-replay` `🛑️cancel/🦀️.rs:59` (under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/...`; the only similar method is `next_close_work_demand` in `📨️messages/✂️clamp/🔁️settlement/🦀️.rs:33`). `next_close_depth_demand` is defined only in the flow host crates, so someone's in-progress edit in the store/replay area is half-landed. It is in the bridge's dependency path via semio-os-kernel. Out of my scope: needs the owner of the store/replay (admission/model agent) to land the matching definition. Log: `🗑️generated/bridge-hub-run4.log`, line ~55357. Rerun run4 after that (units built so far are cached). Hub targets still unrun.
- run8: bridge answers list-mutations s.bim.model 1 any: PASS (     216 mutation rows)

## Update (after coordinator message)
- Bridge: PASS. `list-mutations s.bim.model 1 any` (run8) prints a `semio.repository-test.runtime-inventory/v2` document with the bim model mutation rows (set-element-workset, create-workset, create-support, ...), `producedBy: semio-bim-mutation-bridge`.
- Peer half-edits met and waited out (no edits by me): replay `next_close_depth_demand`, plugin instance-stores crate dep, job-context-step re-exports, store-engine `owned_roots_witness` ([DEBUG] scaffolding), a transiently rewritten `🕰️generations/🟨️.mjs`. Each cost a full re-run; all resolved by the peers.
- `repo run @semio-tech/stdio-bim-plugin:test` (stdio5): graph + 5 dependency tasks OK; the test task's `cargo test --no-run -p semio-hub-stdio-bim` exited 101 with no rendered diagnostic in the log. To see the real diagnostic I started the same compile via `repo cargo ... -- test --no-run --offline -p semio-hub-stdio-bim` in the stdio bim dir (log `🗑️generated/bridge-hub-stdio-cargo.log`, ends with `exit N`); it was still compiling after ~38 min (memory warden/capacity waits).
- NOT run: `@semio-tech/bim-plugin:test` (also needs the BIM guest component via `requireBimGuest`) and the committed-descriptor test (part of that target).
- Precise blocker: host throughput (10+ concurrent grants, warden at guard) plus continuous peer churn in framework crates; no bim-specific failure has been seen yet. Next step: read `bridge-hub-stdio-cargo.log` result, then run `repo run @semio-tech/bim-plugin:test`.

## Final blocker
The stdio compile (`bridge-hub-stdio-cargo.log`) was cancelled by the host memory warden: "cargo pid 64843 (1501 MB): the host was critically short of memory. Nothing is retried." The earlier exit 101 of `repo run @semio-tech/stdio-bim-plugin:test` is most likely the same cause (no compiler diagnostic emitted). No bim-code error observed. Needs a calm host (other agents' builds finished), then rerun: `repo run @semio-tech/stdio-bim-plugin:test`, then `repo run @semio-tech/bim-plugin:test` (includes committed-descriptor test). Not run: hub bim-plugin tests and committed-descriptor test.
- Fix: added semio-framework-value (workspace) to stdio bim hub crate Cargo.toml + 🌎️hub/Cargo.lock entry; dyn_enum_close! macro expansion needs it (siblings trinity/raster/flow already have it).
- stdio compile reached link step; failed with 'No space left on device' (disk dipped under pressure from shared builds), not code. Retrying.

## Final status (this pass)
- Bridge list-mutations: PASS.
- Env finding: a stale dependency generation receipt (BUN_ENABLE_CRASH_REPORTING in the harness env vs. peers' env) made `repo run` die with "Actual declared requested Source scope required"; `repo dependencies sync --all` and running with `env -u BUN_ENABLE_CRASH_REPORTING` fixed it.
- Real fix: stdio bim hub crate lacked `semio-framework-value` (dyn_enum_close! expansion, E0433) -> added to its Cargo.toml and `🌎️hub/Cargo.lock`. The crate now compiles through the link step.
- stdio target (run 12, 13, direct cargo3): link / rmeta writes fail with `No space left on device (os error 28)`; free disk swings between ~22 and ~45 GB with spikes to zero during concurrent shared builds (largest cache dirs under `.cache/b/r/m/u/l/`: pa7844a 17G, ped16c6 13G, p803402 8.6G, q90be4b 6.4G; repo-managed, not mine to delete). Blocker: disk exhaustion under concurrent fleet builds, not code.
- Not run: `@semio-tech/bim-plugin:test` + committed-descriptor test (same shared build + guest component). Needs: reclaim cache dirs (repo cache gc by owner) or serialise fleet builds, then rerun `env -u BUN_ENABLE_CRASH_REPORTING repo run @semio-tech/stdio-bim-plugin:test` then `... bim-plugin:test`.
