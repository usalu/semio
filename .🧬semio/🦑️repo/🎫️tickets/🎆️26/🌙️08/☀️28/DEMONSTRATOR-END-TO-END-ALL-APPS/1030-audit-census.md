# 1030 Audit Census: demonstrator component closure evidence matrix

STATUS: 32 closure components (11 plugins, 21 extensions) + 5 framework/site wasm targets. As of 2026-10-30 ~06:30: 21 extensions GREEN, 4 framework wasm GREEN, 8 hub plugins RED, 3 hub plugins UNKNOWN, 0 unowned-but-broken. Read-only, no source edits, no cargo/build runs, no sub-agents. Lanes `energy` (1030 `energy-chk25`) and `resid` (1030 `resid-chk1`) were still at `build:source-admission` (06:22), so no compile result from this wave exists yet.

## 1. Method and inputs

- Closure: `1030-tools/census/closure.ts` reads `[package.metadata.semio]` (`component-kind`, `depends-on`, `extends`, `consumes`, `contributes`, `host`, `playground`) from the 71 manifests in `🗑️generated/1030/manifests.txt`. It applies the same algorithm as `runtimeComponentClosure` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs`). Roots come from the pane variants and runtimeVariants of `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json` mapped through the playground tables: `demonstrator`, `procedural`, `energy`, `fem`.
- Lane roots (matching `demonstratorActivationLanes` in `♻️activation/🟦️.ts`): `generator`=demonstrator (30 nodes), `generation3d`=procedural (13, covered by demonstrator, so no lane of its own), `energy`=energy (1), `fem3d`=fem (1). Union = 32 nodes. Full table: `🗑️generated/1030/closure-run1.txt`.
- Evidence: 1029 compile logs (`🗑️generated/1029/*.txt` with `.rc`), 1029 matrix (`1029-audit-matrix.md` section 2), and the 1030 folder. Pattern scan `🗑️generated/1030/census-compile-hits.txt` (`rg -F` over 1029 for `Checking|Compiling <pkg> v` and `could not compile \`<pkg>\``), summarised by `1030-tools/census/latest.ts` into `census-latest-components.txt` and `census-latest-crates.txt`.
- Reachability: `1030-tools/census/blockers-chk26.ts` walks `[dependencies]` path edges (over-approximation, dev-deps excluded) and checks against the newest failing set `🗑️generated/1030/birth-chk26-failing.txt` (12 crates from `1029/birthresult-chk26.txt`, 10-30 02:02, rc=1). Output: `🗑️generated/1030/blockers-chk26.txt`.
- Status rule: GREEN = latest own-batch run rc=0 and no reachable crate has a failure newer than that run. RED = latest run rc!=0, or a reachable crate fails in the newest run. UNKNOWN = no pass for the component itself and nothing failing in the newest run.

## 2. Component table

Lane key: gen = procedural lane (demonstrator, procedural), resid = stdio/gis/flow/cad/process/sourcing extensions, fem, energy, runtime = framework wasm, site = demonstrator site. Entries "none" are unowned, see section 4.

| Component | Kind | Cargo package | Lane | Latest evidence (log, mtime, rc) | Status | Notes |
|---|---|---|---|---|---|---|
| demonstrator | plugin | `semio-hub-demonstrator-component` (bundle `semio-hub-demonstrator`) | gen | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26 reaches `semio-s-artifact-cad-cad-transient`, `semio-s-artifact-demonstrator-playground-editor`, `semio-s-artifact-procedural-3d-geometry` |
| procedural | plugin | `semio-hub-procedural-component` | gen | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `procedural-2d-io-text`, `procedural-3d-geometry`. `procedural-2d-editor-config` and `-presence` last failed 10-29 01:22 (`1029/matrix-hub.txt`), no later pass |
| cad | plugin | `semio-hub-cad-component` | resid | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `semio-s-artifact-cad-cad-transient` |
| fem | plugin | `semio-hub-fem-component` | fem | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `fem-2d-io`, `fem-3d-editor-interaction`, `fem-3d-editor-visual-job`. `semio-s-fem-node`/`-material` passed 10-29 15:00 (`hubfix-fem50`) |
| energy | plugin | `semio-hub-energy-component` (lib `semio-hub-energy`) | energy | `1029/energy-chk21.txt` 10-30 00:06 rc=1 | RED | `error[E0275]` overflow in `semio-hub-energy` (`MaybeDangling<…NativeReceiver…>`). Same error in chk19/20. Not in chk26 list, so not verified passing either. 1030 `energy-chk25` pending |
| flow | plugin | `semio-hub-flow-component` | resid | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `semio-s-artifact-flow-flow-inferences` |
| gis | plugin | `semio-hub-gis-component` | resid | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `gisterrain-editor`, `stdio-pptx-strict`, `stdio-tiff-mutation-regressions` |
| playbook | plugin | `semio-hub-playbook-component` | none (see 4) | `1029/birthresult-chk26.txt` 10-30 02:02 rc=1 | RED | chk26: `semio-s-artifact-playbook-playbook-inferences` |
| process | plugin | `semio-hub-process-component` | resid | `1029/matrix-hub.txt` 10-29 02:22 rc=1 (hub batch) | UNKNOWN | No failing crate in chk26. Reachable process crates passed 10-30 01:25 (`pubtoken-b3-p3d`). No own pass |
| puzzle | plugin | `semio-hub-puzzle-component` (lib `semio-hub-puzzle`, wasm `puzzle-plugin:wasm`) | none (see 4) | `1029/matrix-hub.txt` 10-29 02:22 rc=1 (hub batch) | UNKNOWN | No failing crate in chk26. No own pass |
| sourcing | plugin | `semio-hub-sourcing-component` | resid | `1029/matrix-hub.txt` 10-29 02:22 rc=1 (hub batch) | UNKNOWN | No failing crate in chk26. No own pass |
| cad-extension-aec-building | extension | `semio-s-plugin-cad-aec-building-component` | resid | `1029/extfix-cad-aec-building-final.txt` 10-29 21:53 rc=0 | GREEN | |
| cad-extension-aec-building-energy | extension | `semio-s-plugin-cad-aec-energy-component` | resid | `1029/extfix-cad-aec-energy-final.txt` 10-29 21:55 rc=0 | GREEN | |
| cad-extension-aec-building-structure | extension | `semio-s-plugin-cad-aec-structure-component` | resid | `1029/extfix-cad-aec-structure-final.txt` 10-29 10:45 rc=0 | GREEN | |
| cad-extension-spatial-shape | extension | `semio-s-plugin-cad-spatial-shape-component` | resid | `1029/extfix-cad-spatial-shape-final.txt` 10-29 11:40 rc=0 | GREEN | Earlier 05:58 failures (guest-contract, registration, 2d-text) superseded by framework-r2 and dispatch-chk9 |
| flow-extension-bim | extension | `semio-s-plugin-flow-bim-component` | resid | `1029/matrix-flow-bim-r2.txt` 10-29 05:06 rc=0 | GREEN | |
| flow-extension-brep | extension | `semio-s-plugin-flow-brep-component` | resid | `1029/extfix-flow-brep-4.txt` 10-29 09:25 rc=0 | GREEN | |
| flow-extension-dictionary | extension | `semio-s-plugin-flow-dictionary-component` | resid | `1029/extfix-flow-dictionary-final.txt` 10-29 12:57 rc=0 | GREEN | |
| flow-extension-draw | extension | `semio-s-plugin-flow-draw-component` | resid | `1029/extfix-flow-draw-final.txt` 10-29 13:28 rc=0 | GREEN | |
| flow-extension-list | extension | `semio-s-plugin-flow-list-component` | resid | `1029/extfix-flow-list-final.txt` 10-29 13:55 rc=0 | GREEN | |
| flow-extension-logic | extension | `semio-s-plugin-flow-logic-component` | resid | `1029/extfix-flow-logic-final.txt` 10-29 15:33 rc=0 | GREEN | |
| flow-extension-math | extension | `semio-s-plugin-flow-math-component` | resid | `1029/matrix-flow-math.txt` 10-29 03:24 rc=0 | GREEN | |
| flow-extension-primitive | extension | `semio-s-plugin-flow-primitive-component` | resid | `1029/extfix-flow-primitive-final.txt` 10-29 17:35 rc=0 | GREEN | |
| flow-extension-text | extension | `semio-s-plugin-flow-text-component` | resid | `1029/matrix-flow-text.txt` 10-29 03:30 rc=0 | GREEN | |
| playbook-module-procedural | extension | `semio-s-plugin-playbook-procedural-component` | none (see 4) | `1029/extfix-playbook-procedural-final.txt` 10-29 19:16 rc=0 | GREEN | `semio-s-plugin-flow-brep-mesh` failed 06:02 and passed in final |
| process-extension-concrete | extension | `semio-s-plugin-process-concrete-component` | resid | `1029/extfix-process-concrete-final.txt` 10-29 20:32 rc=0 | GREEN | |
| process-extension-metal | extension | `semio-s-plugin-process-metal-component` | resid | `1029/extfix-process-metal-final.txt` 10-29 21:06 rc=0 | GREEN | |
| process-extension-robotic | extension | `semio-s-plugin-process-robotic-component` | resid | `1029/extfix-process-robotic-final.txt` 10-29 21:18 rc=0 | GREEN | |
| process-extension-wood | extension | `semio-s-plugin-process-wood-component` | resid | `1029/extfix-process-wood-final.txt` 10-29 21:39 rc=0 | GREEN | |
| sourcing-module-beams | extension | `semio-s-plugin-sourcing-beams-component` | resid | `1029/extfix-sourcing-beams-final.txt` 10-29 21:47 rc=0 | GREEN | |
| sourcing-module-slabs | extension | `semio-s-plugin-sourcing-slabs-component` | resid | `1029/extfix-sourcing-slabs-final.txt` 10-29 21:49 rc=0 | GREEN | |
| sourcing-module-windows | extension | `semio-s-plugin-sourcing-windows-component` | resid | `1029/extfix-sourcing-windows-final.txt` 10-29 21:51 rc=0 | GREEN | |
| (framework) surface | wasm | `semio-framework-surface-component` | runtime | `1029/matrix-framework-r2.txt` 10-29 06:09 rc=0 | GREEN | `framework-surface-rs:wasm` |
| (framework) editor | wasm | `semio-framework-editor-component` | runtime | `1029/matrix-framework-r2.txt` 10-29 06:09 rc=0 | GREEN | `framework-editor-rs:wasm` |
| (framework) os-node-graph | wasm | `semio-framework-os-node-graph-component` | runtime | `1029/matrix-root-nodegraph-r2.txt` 10-29 06:17 rc=0 | GREEN | `framework-os-node-graph-rs:wasm` |
| (framework) os-flow | wasm | `semio-framework-os-flow-component` | runtime | `1029/matrix-root-flow.txt` 10-29 04:40 rc=0 | GREEN | Reaches `instance-typed-ops`, `time-travel-turns`, which passed in `1029/dispatch-chk9.txt` 10-30 00:42 rc=0 |
| (site) os-flow browser package | wasm | `semio-framework-os-flow-browser-package:wasm` (project `🎁️browser-package`) | site | as os-flow | GREEN (by os-flow) | Owner `demonstrator:site` |

Totals: 21 extensions GREEN, 4 framework wasm GREEN, 8 hub plugins RED (demonstrator, procedural, cad, energy, fem, flow, gis, playbook), 3 hub plugins UNKNOWN (process, puzzle, sourcing). The `semio-hub-demonstrator` bundle crate is RED through demonstrator.

## 3. Lane view

| Lane | Roots | Closure | Status |
|---|---|---|---|
| gen (generator) | demonstrator, procedural | 30 nodes (+ procedural via generation3d, no own lane) | RED (chk26, procedural and demonstrator) |
| energy | energy | 1 | RED (energy-chk21, E0275) |
| fem3d | fem | 1 | RED (chk26: 3 fem crates) |
| resid | cad, flow, process, sourcing, gis extensions | included in gen closure | mixed: extensions GREEN; hub cad, flow, gis RED, process and sourcing UNKNOWN |
| runtime | framework wasm targets | 4 | GREEN |

## 4. Unowned components

No lane in the brief owns these, so they need an owner decision:

1. `playbook` hub plugin (`semio-hub-playbook-component`, RED) and `playbook-module-procedural` extension (GREEN). Both are in the demonstrator closure (`playbook` via `playbook-module-procedural` and `consumes`). Not in gen, resid, fem or energy.
2. `puzzle` hub plugin (`semio-hub-puzzle-component`, lib `semio-hub-puzzle`, wasm target `puzzle-plugin:wasm`, UNKNOWN). It is a `depends-on` of `demonstrator`, and the `aggregator` pane app (`s.puzzle.puzzle3d`) runs on it, but no lane owns it. The brief's runtime lane names only the framework plugin-instance and decl-app.
3. Framework wasm targets `surface`, `editor`, `os-node-graph` and `os-flow`. They are GREEN, but the runtime lane does not name them, so they are unowned by definition.
4. Site: `semio-framework-os-flow-browser-package:wasm` and `semio-framework-canvas:stage-runtime-assets` (`site` owner `demonstrator:site`). Not in the closure, owned by the site script. `canvas` has no compile evidence in 1029 or 1030.
5. `semio-framework-plugin-host-test-component` has no `deployment-directory`, so it is not in the closure. It is excluded, not owned.

Not unowned: `semio-hub-demonstrator` (bundle) belongs to gen.

## 5. Caveats

- The closure is computed from Cargo manifests with the registry algorithm, not from the generated catalog (`@semio-tech/s-dev-catalog-ts/generated/plugins`, not present on disk). If the generator differs, the table changes.
- The blocker walk over-approximates (path deps only, no cfg). It can flag a component that does not really need the crate.
- Evidence from 1029 mixes snapshots. Extension GREEN rows are the latest `extfix-*-final` rc=0 runs, which do not re-check hub-level crates.
- Hub plugins (process, puzzle, sourcing) have no direct pass since the 1029 hub batch (rc=1, 02:22). Their status is UNKNOWN, not GREEN.
- 1030 `energy-chk25` and `resid-chk1` were still in source admission at 06:22. The energy lane has no 1030 compile evidence yet.

## 6. Files

- Closure table: `🗑️generated/1030/closure-run1.txt` (script `1030-tools/census/closure.ts`, input `🗑️generated/1030/manifests.txt`)
- Blocker walk: `🗑️generated/1030/blockers-chk26.txt` (script `1030-tools/census/blockers-chk26.ts`, input `🗑️generated/1030/component-manifests.txt`, `🗑️generated/1030/birth-chk26-failing.txt`)
- Pattern scan and latest per package: `🗑️generated/1030/census-compile-hits.txt`, `census-latest-components.txt`, `census-latest-crates.txt` (script `1030-tools/census/latest.ts`)
- Earlier walk with the 1029 failing set (superseded, do not use): `🗑️generated/1030/blockers-run1.txt`
- Newest compile evidence: `🗑️generated/1029/birthresult-chk26.txt`, `🗑️generated/1029/energy-chk21.txt`, `🗑️generated/1029/dispatch-chk9.txt`, `🗑️generated/1029/hubfix-fem50.txt`
