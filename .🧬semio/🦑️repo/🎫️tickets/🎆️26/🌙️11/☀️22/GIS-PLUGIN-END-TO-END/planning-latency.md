# Planning latency of `repo run-many` (6 GIS projects, `--targets=test`)

Source: resident log `host-d2224d988b81.log` (87k lines, 04:38 to 10:29), read by grep/sed ranges only. No code was changed and the service was not touched.

## Where the time goes

All 583 "still working" lines say `last phase started: source.served`, so the wait is inside one graph request.

| Observation | Value |
|---|---|
| Closure of the 6 selected GIS owners (`[graph-closure]`) | 913 owners at pass 0, 994 at pass 4 (of 995). Kinds: `selection` 6, `file` 988. |
| Who admits them | `@semio-tech/gis-gismap-rs` admits 704 owners through `file` edges, `gis-gismap-js` 96, `gis-gisterrain-rs` 81, `framework-os-node-graph-rs` 68, `gis-plugin` 25. |
| Why | Each Rust package `script.ts` imports `runArtifactRustPackageMain` (`framework/modules/process/packages/typescript/testing/cargo`). Its command-source import closure touches files of about 700 owners. `closeBuildOwnerAdmissionV1` (graph/admission) admits the owner of every read file (`admit(dependency,"file")`) and observes each one, so a narrow selection plans nearly the whole workspace. |
| One full owner pass | 995 owners, 4 at a time, about 10 to 17 min. Memory pressure: 98 "headroom below one project, inferring one at a time" lines, 379 heap collections, 4.8 GB heap, 14.5M live objects. |
| Full owner passes restarted from `0 of 995` | 4 (log lines 8067, 16915, 29818, 51714). Three started with 0 present / 0 fetched: every record stale because planner source changed. |
| Settle rounds (`[graph-settle]`) | One unbroken chain rounds 0 to 4 took t=4370 s to 10251 s (about 1.6 h); each round re-walks the closure (913 then 994 pair, about 700 to 1000 s). |
| Restart messages | 2x "implementation authority changed ... restart" (graph/load ts, taxonomy.json), 2x "Stale source facts that a repeated inference cannot refresh" (graph/load/ts). |
| Per-owner retries | Only `s-spatial-kernel-semio-session-rs`: 3 sequences x 3 rounds (250/500/750 ms). Negligible. |
| Outcome | 09:15:35 `request-failed ... BUILD_OWNER_FAILED` (owner `repo`, `process.testing.route-table-invalid`). |

## Root cause

A product of three bounded loops, each costing a whole-workspace pass:

1. Whole-workspace closure for a 6-owner selection (file-edge admission through a shared script import).
2. Planner source is edited constantly by other agents: 77 non-test files under `build/` changed in the last 5 h, about a dozen in the last hour (graph/load, generations, inference, source closure, resident, taxonomy). Each edit moves the implementation stamp, invalidates owner records (up to 400+ stale in the fleet) and causes a restart or settle round.
3. Replica-unsettled repair loop in `graph/load/<ts>` (catch branch near line 285): a failed owner inference is only thrown when `current.digest===basis.digest` (tree inventory unchanged) or `repairRounds` (4) is used up. With 210k observed files and 50 writers the digest practically never holds, so even a `stable` needed failure (reproduces on unchanged inputs) first seen at t=13550 s was only thrown at t=16552 s after 3 more full closure walks (about 50 min wasted).

Bounds exist (`repairRounds`=4, reexec progress reset in `format-stamps/reexec`, per-owner 3 rounds) but they multiply, and the restart count resets whenever a run made progress, so under constant planner edits the plan does not converge.

## Fix decision

No code change made. Every candidate changes convergence semantics of shared planner files that a dozen agents are editing right now, and verification needs hours per plan:

- graph/load catch branch: throw immediately on a `stable` owner failure (`isOwnerFailureV1(error)&&error.stable`) whose rows hold, instead of requiring an unchanged tree digest (removes the 3 wasted closure walks).
- `closeBuildOwnerAdmissionV1`: admit owners reached only by `file` edges as ownership-only (no full observation) so a narrow selection does not infer about 700 foreign owners.
- Do not let implementation stamp moves in files outside the requested closure's load path invalidate records, or pin the implementation per request.
- Operationally: avoid editing `build/` while a plan runs.

## Repository sources currently making the planner fail

1. `repo`: `process.testing.route-table-invalid`, thrown by `readProcessTestTableV1` in `framework/modules/process/testing/suites/<ts>` when `suites/<json>` (mtime 09:43) does not satisfy `suites/schema/<json>` (mtime 09:38). Both were mid-edit by another agent; recheck once settled.
2. `@semio-tech/s-gis-services-native`: "Canonical native dependency owner required" for `os/modules/renderer/engine/elements/HubFirstRun/targets/wgpu/packages/rust`: that directory has only a stub 122-byte project.json and a Cargo.toml (07:44 to 08:01, no sources). Another agent is still creating this owner.
3. `@semio-tech/s-spatial-kernel-semio-session-rs`: "Generated task cannot consume its own output" for target `wasm`: `session/tests/browser/<ts>` imports generated `packages/rust/bindings/semio_session.js`, which `wasm` itself produces and whose inputs cover it. project.json (06:27) and script.ts (06:21) were edited by another agent. The test importer must be a `browser-test` input only.
4. Earlier, now past: unresolved workspace imports `@semio-tech/stdio-mp3` and `@semio-tech/writer-writer`, missing `cad_geometry.js` binding, `route-undeclared:test-build-task-view`.

## Files changed

None (analysis only, this report).
