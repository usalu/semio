# 1107 audit aud-activation: activation chain at HEAD, closure, freeze, single-lane canary

Read-only static audit, 2026-11-07 about 03:35 CET. No repo run, no cargo, no edits outside this folder. Probes: file reads, `jq`/python `-I` over the generated registry, `ps`, `df`, `uptime`, and the read-only `prefreeze.sh` output that the actinfra lane wrote at 03:30 (`🗑️generated/1105b/actinfra-prefreeze-1107.txt`). I did not re-run `prefreeze.sh` (git grep over the whole tree took more than 2 minutes at load 60).

## 1. Verdict

1. Most act22 facts still hold (targets, script paths, serve host, union path, test-e2e chain). Stale: preflight 1 (wit now GREEN for compile), preflight 2 (single-component proof GREEN on 11-05 via the direct cargo route), preflight 6 (slots now 0/0 as the freeze needs), and the lock/sync gate (wave 1107 now needs `lock-N` with N >= 100 plus `lock-ok-1107` and `sync-done-1107`; present: lock-38, sync-6, no wave markers).
2. Freeze restarts are narrower than act22 says. The verification restart is triggered by edits to the repo tool's own loaded implementation (exit 75 contract), not by app sources. Edits to `✏️s/**` sources invalidate Bazel actions, not the verification. The dominant cost is the stale dependency generation receipt (about 45 min per run, up to 6 restarts observed).
3. Component closure: 4 root plugin ids (`demonstrator`, `procedural`, `energy`, `fem`) give 32 components (11 plugins + 21 extensions). Lanes: `generator` (30 components), `energy` (1), `fem3d` (1). The lane set is already reduced to these 3 by design, so no two lanes share a component.
4. Single-lane canary: `energy` can be activated alone without disturbing the union, because it shares no component with the other lanes and the activation package never removes sibling directories. The memory note about single-lane kills describes overlapping closures (generator vs generation3d, or the play product's lanes). It does not apply to today's disjoint partition. The canary cannot be a browser canary: the 6029 union needs all 3 lanes, and no per-variant serve target was located. Its only proof is compile + stage (about 5-45 min) plus the native energy smoke, which is RED on a known fault.
5. Shortest freeze: (a) coordinator makes a fresh dependency receipt (`dependencies sync` then `lock`, new N >= 100), (b) freeze repo-implementation files only, (c) run the energy compile through the W- priority slot before the freeze. Full activation stays hours (6-9 h at 2 jobs, 12 h+ at 1 job per act22; load 60 today makes this worse; UNVERIFIED). Two tool conflicts with the brief must be fixed first: `act.sh` sets `SEMIO_LEASE_PRIORITY` (forbidden) and bypasses `slot.sh` (required).

## 2. Act22 audit points at HEAD

| act22 point | Status now | Evidence |
|---|---|---|
| Targets activate-prototype, activate-lanes-prototype, dev, serve, serve-e2e, test-e2e | HOLDS | `♻️mit-bestand/🧺️demonstrator/📋️project.json`. activate-prototype deps: prepare-prototype (13) + activate-lanes-prototype (3 lanes: energy, fem3d, generator). serve/dev depend on activate-prototype. test-e2e -> serve-e2e -> prepare-e2e -> activate-prototype + workspace:deps-browsers. |
| Act.sh command (bazel run activate-prototype, env) | HOLDS, but conflicts with brief | `act.sh` sets `SEMIO_LEASE_PRIORITY=interactive` (brief: never set it) and calls `repo` directly, not via `slot.sh` (brief: every repo run through slot). |
| Serve host 127.0.0.1 unless DEVCONTAINER=true | HOLDS | `📜️script.ts` ServeScript `host:` expression. |
| serve-e2e binds port 0 | HOLDS | `ServeTestScript` `port: 0`. |
| Union receipt dir `dist/♻️activation/prototype` | HOLDS (absent today: no activation has run since cache reset) | `🧩️runtime/♻️activation/🟦️.ts` `DEMONSTRATOR_UNION_RECEIPT_DIRECTORY`. |
| Lane receipts `.cache/b/p/m/browser/prototype/runtime/react/<lane>/activation/🔣️receipt.json` (serve.sh requires generator, energy, fem3d) | HOLDS, all absent today | `ls` at 03:35. |
| Preflight 1: wit lane green | STALE: now GREEN for compile (wit-11..14), Rust reactor-context laws and browser runtime UNVERIFIED | `1105b-result-wit.md` header. The act22 RED reason (store-replay / store-space-history foreign errors) is not re-checked here. |
| Preflight 2: single-component proof | STALE: GREEN on 11-05 through the direct route (`repo cargo ... build --manifest-path ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.toml --target wasm32-wasip2 -p semio-s-plugin-flow-list-component`, then stages). The `repo run <component>:component` route was NOT proven (killed after 2.8 h). | `1105b-result-actinfra.md` header and "Single-component proof" section. |
| Preflight 3: build-dir prefix <= 55 | UNVERIFIED now (not in prefreeze.sh) | act22 said prefix 53 statically. |
| Preflight 4: lock/sync converged | STALE: wave-stamped gate, see prefreeze.sh `WAVE=1107`, `LOCKMIN=100`. Newest lock-38.rc=0 (11-06 22:13) and sync-6 are not valid for wave 1107; Cargo.toml newer than lock-38 (wfc engine/job and oracle crates). | `actinfra-prefreeze-1107.txt` lines `lock-run`, `sync-run`, `cargo-vs-lock`. |
| Preflight 5: disk >= 100 GiB | HOLDS but thin: 106 GiB free at 03:30 (was 165 in act22, 207 in actinfra) | `df -g`. `.cache/b/r/m` own mount 47 GiB free (>= 20). |
| Preflight 6: slots paused | DONE: slots.k=0, slots.wextra=0. Still 6 normal waiters queued, 0 held. | `1105b-tools/slots.k`, `slots.wextra`, prefreeze `slots` line. |
| Preflight 7: no cargo in flight | OPEN: load average 60 at 03:30; 9 repo/cargo/bun processes from peers in this ticket and a `repo mcp` server. | `ps`, `uptime`. |
| Preflight 8: browser harness | HOLDS (static): `run.sh --check` 8/8; browser verdict UNVERIFIED until a serve exists. | `1105b-result-e2e.md`. |
| Act14-act21 failure fixes | HOLD (not re-checked beyond the act22 table). The act21 path fix is in Cargo.lock; `os/🎚️config/🏭️bridge/Cargo.lock` was flagged open in act22. | act22 section 3. |
| Channel pin | HOLDS: pin=25 Rust, TS and browser-actor all 25 (prefreeze PASS). The 70 descriptor JSON files at 24 are describe outputs (INFO only, refreshed by describe write mode). | prefreeze `channel-pin-code`. |
| `[DEBUG]` lines | NEW BLOCKER: 49 lines in 23 files (fem `native-tests.rs`, puzzle editor and pane-smoke files). Must be removed before freeze (AGENTS.md). | prefreeze `no-debug-lines`. |
| ext-ok, wasm-debug0 (prototype-wasm debug=0 patch) | UNVERIFIED: the 03:30 output stops after channel-pin-code. Note the schema has 56 `"debug": 0` lines, so the patch may be applied; prefreeze tail must be re-run. | `actinfra-prefreeze-1107.txt` is cut. |
| Freeze file `🗑️generated/1105b/freeze` | ABSENT (freeze not called) | `ls`. |
| Serve pre-check markers (lock-ok, sync-done, admission-ok, storage-ok) | Present for the old gate names; the wave gate now needs `lock-ok-1107`, `sync-done-1107` | `ls 🗑️generated/1105b`. |
| Panes smoked by `panes.sh` | GAP: covers cad (koordinator), process3d (bearbeiten), energy, fem3d (statik), gis (verfolgen). No smoke for aggregator (puzzle3d), aussuchen (sourcing), generator (procedural/demonstrator). | `1105b-tools/panes.sh`. |

## 3. Component closure (current registry and pane rows)

Sources: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json` (variant -> pluginId), `🔌️plugins.json` (dependsOn, consumes, role, directoryName), `🔨️modules/🧩️runtime/🧫️cases.json`, `🔣️.json` (panes), and `1105b-tools/actinfra/ids.json` (earlier closure run).

Pane rows (`🔣️.json` `panes`) to runtime variant to pluginId:

| Pane (brief) | Pane variant | Runtime variant | pluginId | Lane |
|---|---|---|---|---|
| generator | generator | generation3d | procedural (runtime) / demonstrator (variant) | generator (primary) |
| koordinator | koordinator | koordinator | demonstrator | generator |
| aggregator | aggregator | aggregator | demonstrator | generator |
| aussuchen | aussuchen | aussuchen | demonstrator | generator |
| bearbeiten | bearbeiten | bearbeiten | demonstrator | generator |
| verfolgen | verfolgen | verfolgen | demonstrator | generator |
| energie | energy | energy | energy | energy (own lane) |
| statik | fem3d | fem3d | fem | fem3d (own lane) |

Derived sets:
- Root plugin ids: demonstrator, procedural, energy, fem (4).
- Closure: 32 components = 11 plugins (cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing) + 21 extensions (4 cad, 9 flow, 1 playbook-module-procedural, 4 process, 3 sourcing). The 32 match `ids.json` and `cases.json` exactly. Reached by dependsOn alone: 19 components. The other 13 (cad/process/sourcing/playbook extensions, playbook) come through capability `consumes` links (demonstrator consumes `sourcing.module`, `process.machines`, `cad.computer`, `flow.extension`). I did not read the runtimeComponentClosure rule itself; the count is taken from the earlier run.
- Lanes (`demonstratorActivationLanes()`, which skips any runtime variant whose plugin is already covered): generator (demonstrator closure: 30 components), energy (1: `energy`, no deps, no consumes, no contributes), fem3d (1: `fem`, same). 30 + 1 + 1 = 32. Lane `generation3d` (procedural) is inside generator's closure and is excluded on purpose (code comment).
- Expected completed-components count: 32 in the union receipt. Bazel: about 77 actions (32 components + describe-rs + wasm/support), as act22 said.
- Wasm engines shared by all lanes: `surface-rs`, `editor-rs`, `os-node-graph-rs`, `puzzle-plugin` (prepare-prototype deps). Whether the energy or fem lane pulls them is UNVERIFIED (the framework-generated `prepare-energy-react-prototype` target is not visible statically).

## 4. Freeze: what really restarts graph verification

- Repo contract (`🧰️framework/.../🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🧪️tests/🔁️implementation-edit/🟦️.ts`): a peer that edits the loaded reader (repo implementation code) while a command runs ends the first process with exit 75; the replacement finishes. Bounded: "(1 of 6)" restarts. So the restart trigger is an implementation edit, not an application-source edit.
- Observed trigger (1105b-result-actinfra.md line 52): a peer edited `⚡️caching/…/dependencies/generation/🟦️.ts` (implementation) during the 47-min verification.
- Observed slowdown (act22, actinfra): stale dependency generation receipt, because the installer input `nx@23.2.0.patch` changed after the last `dependencies lock/sync`. Every `repo run` then re-verifies (about 45 min) until a fresh receipt exists. The receipt is fixed by the coordinator's `dependencies sync` then `lock`.
- Activation source edits (`✏️s/**`, demonstrator sources): they change input hashes, so Bazel re-runs the affected actions. The claim "restarts on ANY source edit" is not supported by the code or the observed logs. Treat it as UNVERIFIED for app sources.
- Practical freeze: freeze repo-implementation files (`🧰️framework/**` build, caching, process, repo-lib modules) and the wasm/profile configuration; app edits can continue as patches but should not touch an action's inputs during its run.

## 5. Single-lane canary (energy)

Safe to activate alone (static proof):
- Lane receipt is per lane (`developmentRuntimeRoot(workspace, lane, prototype, react)/activation`). Its only shared write is `builtPluginModulesRootIn(...)/energy` (the `🔌️plugin-modules` root). The activation package has no sibling-directory removal (the only `rmSync` calls are lease files and temp files).
- Merge (`mergeDemonstratorActivationReceipts`) throws only on (a) a sha disagreement for one pluginId across lanes, (b) a non-prototype or wrong-variant receipt, (c) a missing or extra id against the 32-id union. Energy shares no id with generator or fem3d, so an energy-only re-activation cannot cause (a). It can cause (c) only while the union is read before all 3 lanes exist, which is the expected pre-state.
- Memory note `project-play-lane-merge-and-wasm-build-mutex` (19 days old): its kill mechanism is overlap between lanes. It is correct for lanes that overlap (generation3d vs generator, both containing procedural and 10 more plugins, which is why the code excludes generation3d). It does not apply to energy or fem3d today. Its wasm-mutex advice still applies to any wasm32 cargo that runs alongside others.

Not possible yet:
- A browser canary for energy alone. The 6029 demonstrator union needs all 3 lanes (serve.sh and readDemonstratorActivation both require them). The energy playground (react port 6106 in `🚀️playgrounds.json`) would need a per-variant serve target; I did not locate its name (UNVERIFIED). Treat energy browser evidence as unavailable until the union serves.
- Canary evidence available: compile (`semio-hub-energy-component`, hub.sh already built it GREEN in W-energie-h2) and stage/describe (stages.ts). `stages.ts` hard-codes the base name `semio_s_plugin_flow_list_component`, so it must be parametrized for energy (`semio_hub_energy_component`) before use (ticket tool edit, not done).
- Native pane smoke (`panes.sh` W-pane-energy) is RED on a known fault (`initializer-cancelled`, 1105b-result-energie.md), so a green canary is not expected from that smoke.
- Slot policy: with slots.k=0 and wextra=0 even W- labels wait. For the canary use `slots.k=0`, `slots.wextra=1` (one priority slot for W- labels only), then restore 0/0 before the freeze.

## 6. Command plan with expected durations

Durations are from prior logs where cited; others are UNVERIFIED estimates and will be longer at load 60.

Phase 0, gates (coordinator; no activation yet)
1. Fix conflicts: remove `SEMIO_LEASE_PRIORITY` from `act.sh`, and route it through `slot.sh` (label `W-act22-activate`), or accept a manual slot gate. Owner: coordinator (act.sh is a ticket tool).
2. Remove the 49 `[DEBUG] ` lines (fem native-tests, puzzle editor files). Owner: lanes that own them (AGENTS.md rule).
3. `dependencies sync` then `dependencies lock` (coordinator only) to produce `lock-N` and `sync-N` with N >= 100 and the `lock-ok-1107` and `sync-done-1107` markers. Duration: UNVERIFIED (prior lock/sync runs are in `🗑️generated/1105b/lock-*.txt`).
4. `bash C/1105b-tools/prefreeze.sh` (read-only, about 2-5 min under load) until ALL PASS, including the tail (ext-ok, wasm-debug0).

Phase 1, energy canary (optional, before the freeze; about 30-60 min total)
5. `slots.k=0`, `slots.wextra=1` (coordinator writes these files).
6. Compile through the priority slot (hub manifest, wasm, same flags as `hub.sh`):
   `C/1105b-tools/slot.sh W-energy-canary env SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- build --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 --keep-going -p semio-hub-energy-component > C/🗑️generated/1105b/W-energy-canary.txt 2>&1`
   Duration: warm 2-10 min (hub check was GREEN earlier); cold 15-40 min (flow-list cold build 41m44s, 427 MB wasm).
7. Stage and describe the energy wasm with a parametrized `stages.sh` (base name changed to `semio_hub_energy_component`, output in `C/🗑️generated/1105b/energy-canary/`). Duration: about 2 min (flow-list stages about 75 s to 2 min).
8. Optional lane receipt (through the graph, so it pays graph verification): `C/1105b-tools/slot.sh W-energy-act env SEMIO_LANGUAGE=en repo --tool=bazel run @semio-tech/framework-os-dev:activate-energy-react-prototype > C/🗑️generated/1105b/W-energy-act.txt 2>&1`. Duration: graph verification 5-25 min if the receipt is fresh; 45 min per restart if stale (observed up to 2.8 h). Then energy's own Bazel action(s) and any wasm engine prepares (UNVERIFIED).
9. Restore `slots.k=0`, `slots.wextra=0`. Check the energy canary logs (rc 0 and stages output).

Phase 2, freeze window and full activation (coordinator)
10. `printf 1 > C/🗑️generated/1105b/freeze` and freeze repo-implementation edits (app edits as patches only, no edits to a running action's inputs).
11. Confirm `slots/` has no `s*` directory and an empty queue, `slots.k=0` and `slots.wextra=0`, disk >= 100 GiB, no `[DEBUG]` lines.
12. `nohup C/1105b-tools/act.sh act22 >/dev/null 2>&1 &` (after conflict fix 1, so it runs through slot). Monitor with `bash C/1105b-tools/actinfra/act-monitor.sh act22 60` (only reads logs).
    Expected: capacity door 0-30 min; graph verification 5-25 min if fresh (2.8 h if stale); Bazel about 77 actions, describe-rs about 24 min, components 13-60 min each (flow-list 41m44s). Total 6-9 h at 2 jobs, 12 h+ at 1 job (act22); worse at load 60.
13. Success marker: `act22.status` ends with `exit 0` and `dist/♻️activation/prototype` exists.

Phase 3, serve and browser (after Phase 2 exit 0)
14. `C/1105b-tools/serve.sh start` (or the act22 nohup command). Expected: minutes; ready line "Demonstrator ready" and GET / 200 (cold landing can exceed 15 s).
15. `bash C/1105b-tools/e2eready/run.sh http://127.0.0.1:6029/` for the 8-pane browser verdict (UNVERIFIED until it runs; the e2e lane estimated 30-60 min for test-e2e).
16. `C/1105b-tools/slot.sh act22-test-e2e repo run @semio-tech/mit-bestand-demonstrator:test-e2e` (dependsOn serve-e2e, prepare-e2e, activate-prototype). Expected 30-60 min plus any re-verification.

## 7. Tool and policy conflicts found

- `act.sh` sets `SEMIO_LEASE_PRIORITY=interactive`; the brief forbids it. Fix before the freeze.
- `act.sh` runs `repo` directly, bypassing `slot.sh`; the brief requires every repo run through the slot semaphore. act22 step 0 relies on a manual gate.
- `prefreeze.sh` has changed since act22 (03:28): wave-stamped (`WAVE=1107`, `LOCKMIN=100`, `lock-ok-1107`, `sync-done-1107`). The act22 document still describes the older checks.
- The 03:30 prefreeze output is incomplete (no ext-ok, wasm-debug0 or INFO lines), so those two checks are UNVERIFIED.
- The act22 text "restarts on ANY source edit" is not supported (section 4).
- `serve.sh` checks the three lane receipts and that the union directory exists before starting; `script.ts` serve also calls `readDemonstratorActivation`, which rebuilds and checks the union, so a stale or incomplete union fails there with the named-target message.

## 8. Not verified (static only)

- Whether the energy or fem lane pulls the shared wasm engines (framework-generated prepare targets not visible).
- Energy-only serve target for the browser (no name located).
- Whether Bazel action outputs from a direct `repo cargo build` are cache hits for the later activation (would remove most component time). Not proven.
- Current status of ext-ok and wasm-debug0 (prefreeze tail missing), and build-dir prefix (no current check).
- All durations are estimates unless cited from a log.
