# Terra All-App Reactivity Inventory

Date: 2026-08-26  
Scope: read-only source and metadata census while Flow packets are active. No Cargo, Nx, Wasm, browser, Git mutation, or production edit was run.

## Subsequent checkpoint

The launch-registration findings in this read-only census are superseded by `📓️codex-all-app-discovery-and-launch-gate-implementation-2026-08-26.md`. The canonical `.vscode/🧩️launch.seed.jsonc`, generated `.vscode/launch.json`, and registry generator now contain and validate the focused app gate, root interactivity gate, tool-job gate, dependency ratchet, and zero-target dependency gate. The latest focused discovery run is GREEN at 32 descriptors, 101 descriptor app contexts, 68 launch-only products including 11 Compose surfaces, 169 total surfaces, 248 development launches, 0 failures, and 25 self-tests. All source/runtime/action/dependency deficits below remain live unless a later packet explicitly supersedes them.

## Decisive result — RED

The all-app acceptance contract is not presently demonstrated. The production-entry census finds **32 plugin descriptors**, plus framework/product launch surfaces. `📜️script.ts` contains substantial static hostile-mutation verifiers for a selected set of vertical slices, but that is not all-app scheduler/reachability proof. The master checkpoint still records `verify interactivity tool-jobs` **RED** with **0 / 884 admitted**; this scout did not rerun that active-source gate. `.vscode/launch.json` has **185** `🛠️dev` configurations but no registered `verify interactivity` or `verify interactivity tool-jobs` configuration. Thus neither the final serialized launch matrix nor the required native/Wasm/timing/browser evidence exists.

This is a census result, not a claim that every unproved row is broken: `missing` means no qualifying evidence was located in the permitted static scan.

## Authoritative discovery method

The surface set is metadata-derived, not a manual allowlist. Commands executed (read-only):

```sh
rg --files '✏️s/🔌️plugins' -g '🔣️descriptor.json' | sort
bun -e 'const fs=require("fs"),cp=require("child_process"); const p=cp.execFileSync("rg",["--files","✏️s/🔌️plugins","-g","🔣️descriptor.json"],{encoding:"utf8"}).trim().split("\\n").filter(Boolean).sort(); for(const x of p) console.log(x); console.log("COUNT="+p.length)'
bun -e 'const s=require("fs").readFileSync(".vscode/launch.json","utf8"); const n=[...s.matchAll(/"name"\\s*:\\s*"([^"]+)"/g)].map(x=>x[1]); console.log(n.filter(x=>x.includes("🛠️dev")).join("\\n")); console.log("COUNT="+n.filter(x=>x.includes("🛠️dev")).length)'
rg -n 'interactivity|tool-jobs|toolJob' '📜️script.ts'
rg -n 'interactivity|tool-jobs' .vscode/launch.json
rg -n 'compile_and_solve|FillBuilder|Engine::run|transact\\(|reject_whole_buffer_artifact_envelope_ingress' '✏️s/🔌️plugins' '🧰️framework' -g '*.rs'
```

`🔣️descriptor.json` is the primary plugin catalog. `project.json`/`package.json` and `launch.json` provide activation evidence. The repository has catalog-query source, but no generated playground catalog was found by this static scan; its generated/runtime membership is therefore an uncertainty, not an exclusion.

## Evidence legend and universal deficits

`S` = selected static verifier/fixture evidence; `M` = remaining monolithic/raw/whole-buffer or synchronous route found; `?` = evidence missing; `R` = runtime-only final proof pending. `S` does **not** mean final green.

Every row below lacks final proof for the complete contract columns unless explicitly stated: fixed capacities/preflight; fuel/deadline; progress/checkpoint/preview; cancel, retained ACK and exact handback; stale/ABA/lost-handle/retry/idempotent-close; 8 ms timing; language-neutral empty/single/max/max+1 fixture; owned test-only third-party oracle; native/Wasm byte-identical ledger; real production reachability; EN+DE accessible status without a default language; and desktop then mobile/tablet validation. No mobile/tablet production launch registration was discovered. React/WGPU Wasm/native triples are desktop-target evidence only, not device acceptance.

## Plugin descriptor matrix (all 32)

| Metadata surface | Launch evidence | Expensive-interaction disposition | Static evidence / exact blocker |
|---|---|---|---|
| writer | React, WGPU Wasm/native | S + R | named retained envelope caller verifier; runtime/all-contract proof pending |
| mathematical | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| procedural | 2D/3D React, WGPU Wasm/native | S + M + R | P3 static packet; `compile_and_solve` WFC remains master-plan monolith; P3 global raw-caller census is 8, not required 9 |
| flow | React, WGPU Wasm/native | ? | source packet active; no final static/runtime claim |
| gis | 2D/3D React, WGPU Wasm/native | S + R | GIS Map named retained envelope verifier; 3D and final runtime matrix pending |
| vcs | play React, WGPU Wasm/native | S + R | history/store structural static verifier; real VCS app maintenance→authority→atomic swap runtime proof pending |
| animate | animateplay React, WGPU Wasm/native | S + R | universal verifier includes Animate retained-session mutation checks; app matrix pending |
| shooting | React, WGPU Wasm/native | M + ? | raw envelope ingress caller appears in current census; no qualifying vertical acceptance found |
| demonstrator | Mitbestand demonstrator only | ? | descriptor, but no own registered app triplet located |
| sequence | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| fem | 2D/3D React, WGPU Wasm/native | S + M + R | named FEM/static coverage, but master plan retains FEM engine full-run/solver migration |
| architect | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| process | 3D React, WGPU Wasm/native | S + R | Phase-8 paged-ingress/host static coverage; no all-operation runtime proof |
| lowpoly | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| reasoning | Wires React, WGPU Wasm/native | ? | no named all-app job verifier located |
| forms | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| layout | React, WGPU Wasm/native | S + R | retained layout/cold-relay static mutation verifier; timing/device/i18n pending |
| cad | React, WGPU Wasm/native | M + ? | whole-buffer envelope caller found; four extensions are separate descriptor rows |
| cad/aec-building-structure | cad extension launch not registered separately | ? | descriptor discovered; activation/expensive path requires classification |
| cad/aec-building | cad extension launch not registered separately | ? | descriptor discovered; activation/expensive path requires classification |
| cad/spatial-shape | cad extension launch not registered separately | ? | descriptor discovered; activation/expensive path requires classification |
| cad/aec-building-energy | cad extension launch not registered separately | ? | descriptor discovered; activation/expensive path requires classification |
| norm | no direct launch found | ? | descriptor only; product reachability missing |
| imperative | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| remodel | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| energy | no direct plugin launch found | M + R | master plan identifies `Engine::run` full-run loop; P7c completion/runtime evidence absent |
| dag | React, WGPU Wasm/native | M + ? | raw envelope ingress caller appears in current census |
| draw | React, WGPU Wasm/native | S + R | named retained envelope caller verifier; final matrix pending |
| raster | React, WGPU Wasm/native | S + R | named retained envelope caller verifier; final matrix pending |
| note | React, WGPU Wasm/native | ? | no named all-app job verifier located |
| space | no direct launch found | ? | descriptor only; product reachability missing |
| sourcing | React, WGPU Wasm/native | ? | no named all-app job verifier located |

The 32nd descriptor count includes the CAD root plus its four extensions; the table has 32 rows. `block` and `puzzle` are production launch surfaces but lack top-level descriptor files, so are in the launch-derived matrix below rather than silently excluded.

## Product/framework and launch-derived matrix

| Surface discovered from launch/workspace metadata | Disposition | Evidence / blocker |
|---|---|---|
| Puzzle 2D, 3D, 5D and concrete/capsule variants | S + M + R | P3 FillBuilder has resumable static work; master plan still calls 2D/WFC work unfinished; raw P3 Wasm caller found in current census |
| Block 2D, 3D, 5D | ? | launch triples exist; no corresponding all-app static acceptance located |
| Trinity Jack and Rewrite | S + R | named retained envelope caller verifiers; runtime matrix pending |
| Playbook | ? | launch triples exist; no named all-app acceptance located |
| Framework UI/OS/Shell/renderer (`s`, OS hub/MCP/run, dashboard) | M + R | master plan identifies synchronous `UiRuntime::transact`, synchronous host callbacks, and unfinished P5 transaction/renderer work |
| Repo MCP/client | ? | registered developer products; paged ingress static code covers framework routes, not product end-to-end proof |
| Coda | ? | desktop/MCP launch entries only; no qualifying acceptance evidence located |
| Compose (desktop, hub, engine/MCP, sketchpad/query/GQL/3dm) | ? — scope conflict | launch metadata makes it a production surface; master plan says `./compose` out of scope, whereas acceptance contract permits no exemption |
| Mitbestand and Projektetage | ? | launch entries only; map owned apps/plugins before final gate |
| Print, Storybook, typecheck, scale/size fixtures | dev/test-only | excluded from production acceptance unless a product manifest/entry point promotes them; they do not satisfy app evidence |

## Cross-check: root verifier and launch registration

`📜️script.ts` implements a broad static `verify interactivity` audit and a source/hostile-mutation `verify interactivity tool-jobs` census. Named vertical checks include Present, Writer, Jack, Trinity Rewrite, GIS Map, Raster, Draw, history/store, peer roots, paged ingress, Puzzle routes, Layout/cold relay, FEM and renderer/Animate fixtures. This is valuable **partial static coverage**, not a discovery-driven registration of all 32 descriptors plus launch-only products.

`.vscode/launch.json` has no `interactivity`/`tool-jobs` gate command. Consequently a developer cannot start the required root verification from the mandated launch surface, and the final matrix cannot demonstrate that every production app starts via a registered command. Add the two root Bun commands in the existing `4_gate` ordering only after active source packets are quiescent.

The fresh P3 audit independently found `reject_whole_buffer_artifact_envelope_ingress` in Store plus **seven** peer callers (Shooting, FEM2D, FEM3D, CAD, Puzzle5D, Puzzle3D, DAG): eight files total. The P8 raw-census acceptance requirement is guard plus eight peers (= nine files), and Procedural3D is absent. This is a global census drift blocker even where individual static fixture probes are green.

## Non-overlapping execution packets and priority

| Priority | Packet boundary | Deliverable and ownership boundary |
|---|---|---|
| P0 | Gate registration and discovery ledger only | Owner: coordinator/launch-gate packet. Generate descriptor+launch ledger and add launch registrations; no plugin/source migration overlap. |
| P1 | Framework UI transaction/host/renderer | Existing P5 owner: eliminate `UiRuntime::transact`/host synchronous path before app claims. |
| P2 | Scheduler/admission/protocol | Existing P2a1/P1q owners: make pool, retained protocol, cancellation and handle lifecycle common; no per-app wrappers. |
| P3 | Raw ingress census closure | Existing Phase-8 ingress owners: restore guard+eight-peer contract and remove whole-buffer callers, slicing per plugin (Shooting, CAD, DAG, Puzzle/FEM) to avoid collisions. |
| P4 | Solver verticals | Existing P6/P7 owners: FEM, Energy `Engine::run`, WFC/Puzzle2D. Keep their math directories disjoint. |
| P5 | Descriptor groups without named static proof | Separate file-disjoint owners: authoring (mathematical/architect/lowpoly), workflow (flow/imperative/sequence/reasoning/forms), content (note/remodel/sourcing/space/norm), and CAD extensions. Each first supplies a schema/ledger and reachability map. |
| P6 | Serialized runtime acceptance | After source quiescence: every discovered row gets native+Wasm/replay/timing/browser evidence, oracle comparison, EN/DE accessibility, and device matrix. No source ownership. |

## Uncertainties and closure conditions

1. A descriptor may represent an extension rather than an independently launchable app; that changes the launch row, not its need for a parent production reachability proof.
2. Generated playground/catalog membership was not materialized on disk. Re-run its discovery after source quiescence and reconcile it against this ledger.
3. The checkpoint's `0 / 884` is reported evidence, not re-executed in this packet because Cargo/Nx/Wasm/browser and concurrent source work are prohibited.
4. No runtime test was run here. No oracle, 8 ms, production reachability, EN/DE accessibility, or mobile/tablet claim is green on the strength of this report.
5. Resolve the Compose scope contradiction explicitly; the all-app contract otherwise blocks closure.

Closure remains RED until all metadata-discovered rows are classified, the launch gate exists, raw/monolithic denials are zero, and the contract's serialized runtime matrix is green for each production surface.
