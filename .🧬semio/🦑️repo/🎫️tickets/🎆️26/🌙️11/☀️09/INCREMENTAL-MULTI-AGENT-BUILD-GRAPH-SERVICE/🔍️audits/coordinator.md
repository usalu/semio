# 🧭 Coordinator Checkpoint (2026-11-09)

## End-to-end dry run through the host service
- `repo run repo-task-scope:test --dry-run`: passed the sealed loader (door.script, ~140 s), then refused with `Every owner needs edges of the published environment: semio-framework-pack-json-output-receiving` (root `🎒️pack/🔤️json/📥️decode/🧳️receiving/🫴️unique/📦️packages/🦀️rust`).
- Project name is unique in the repo; the stored generation 325 carries the old Cargo-crate owner name while the live registry uses `@semio-tech/framework-pack-json-output-receiving-rs` (`project.json` added after generation 325). Root cause reproduced with a real-shape fixture (`🕰️generations/🧪️tests/🏷️renamed-owner`): the old-named owner was neither dirty nor returned and kept stale edges. Fixed in the merge (`🕰️generations/🟨️.mjs`): a previous untagged owner whose root a returned project holds under another name is dropped. Not yet confirmed live.

## Blocked on the user
- The running host service still runs old code. `repo service host stop` was denied for agents by the permission policy ("interfering with other agents' workloads"). A restart is needed to load the cut-over, the new diagnostics, and to measure warm/edit dry-run times.
- Earlier pending: deletion of unreferenced old resident code and Go `🛰️resident*` packages (list in `client-cutover.md`), and stopping fleet-owned build processes.

## Landed (light-tested, not yet live)
- Closure settler on `createWorkspaceGraphV1` (shared per workspace, cap 16), grant bound in `📜️script.ts` via `useCapacityGrantV1`; cutoff 146, load 85, generation-protocol 47, resident host 13, revision 24, generations 30 pass.
- Derived DAG (round 6): shared-manifest edit 0.13–0.19 s, project edit 35–48 ms, no-op 35 ms, cold 22.7–30.4 s.

## In progress
- Done: journal dirty set feeds `settle({dirty})` on the workspace graph (cutoff 147 pass); a clean set does 0 re-verifications.
- integration: new node kinds `project-config` / `project-set` so project inference re-assembles only touched projects; planner `stale` port (action keys from the graph, recorded per successful task); fixture + oracle + bench case. Until this lands, warm/edit wins reach only the closure settler.
- rss-profile: planner peak ≤ 1.2 GB.
- Open defect: ledger law test fails on `🗄️cas/🧬️schema/🔣️.json:149` (`bazel/actions/cas`).

## Update: project assembly and task state (integration, eleventh request)
- New derived kinds `project-config`, `project-set`, `task-key` (schema + Python oracle, 13 pass).
- Planner `stale` port: task keys from the workspace graph; records after successful real runs (host CAS + pointer under `.cache/b/g/m/task-keys`); all-up-to-date plans skip the build tool (task-state 2, resident host 14, plan 6 pass).
- Bench scenario `task-keys-edit` added, not yet run (needs service restart).
- Open: inference still reads the whole roster; next is a shared root-filter function, an offline real-repo roster equivalence script, then wiring inference to `project-set`/`project-config`.
- rss-profile round 2: peak 1.7–1.9 GB (target ≤ 1.2 GB); round 3 = in-flight cap, shared SourceFile registry, fixing observation-blocks suite.

## Update: roster (integration, twelfth request)
- Shared pure root-filter rules `🕸️graph/🧭️roster` used by inference and `project-set`; real-repo equivalence 0 differences (118,274 files, 4,898 roots, 4,751 named).
- Removed `discoverNativeBuildOwnersV1` (77.6 s cold → 0.58 s candidate step).
- Roster read stays in inference (0.2–0.4 s, memoized parse) — accepted.
- Resident command/provision/service suites green. Next: full BUILD test sweep to green.

## Update: live service path (rss-profile rounds 6–8, integration 14–17)
- Fixed: host service ran inside the launching job's cgroup (7 GB in-job kills); now own per-service cgroup `semio-service-graph-<hash>`, heap target 1024 MB, limit 4096 MB, stripped client env.
- Fixed: registry module refused as uncaptured import; JSON modules parsed as JS in the capture loader.
- Slice loads run on a worker pool (max 2, cancellable, heartbeat); status answers < 1 s under a synchronous load (test).
- Critical gap: the live `loadGenerationGraphV1` still runs full "Verifying graph facts" (319,836 facts, ~10 min) and closure verification (2005 closures, stalled) at 3.0–3.3 GB. The derived DAG and journal do not reach it yet. Assigned to integration as top priority. Live dry run `dry-3` was still reading the slice after 540 s.

## 2026-11-09 slice memory guard was disabled

- `⌨️command/🌐️host/🟦️.ts` read `HOST_POLICY_V1.sliceSafetyMarginMegabytes`, but the schema declared it in `Policy.const`. `Number(undefined)` is NaN, so the guard `current+margin>limit` never fired. That is why the live service hit its 4 GiB cgroup OOM instead of refusing with `memory`.
- Fixed: the key moved to `HostPolicy.const` in `🏡️resident/🧬️schema/🔣️.json`. `createSlicePoolV1` now throws "Finite memory margin and budget required" on a non-finite or negative margin.
- Test: `🧪️tests/🌐️host` "the host policy declares a finite safety margin…". Suites: host 27, service 37, protocol 22, cancel 25, graph-hooks 3; all pass.
