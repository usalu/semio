# 🧭 Result: Wrong-Direction Relocation (SUMMARY Follow-up 3)

Coordinator: Opus 5.5 side session, started by the monolith-split fleet session, 2026-10-15 14:25–20:45. There were five Opus executors (slices A–E), and the rules are in `📓️RELOCATION-BRIEF.md`. Each slice has a detailed report:

| Slice | Target | Report |
| --- | --- | --- |
| A | framework manifest `🧳️product-vocabulary` | `RESULT-RELOCATE-MANIFEST-VOCABULARY.md` |
| B | tiled-map `🌍️gis-policy` | `RESULT-RELOCATE-TILED-MAP-GIS.md` |
| C | hub inference `🌍️gis-map-binding` | `RESULT-RELOCATE-HUB-GIS-BINDING.md` |
| D | puzzle 2d naming in the os infinite board | `RESULT-RELOCATE-BOARD-PUZZLE2D.md` |
| E | plugin SDK tool-run/time-travel/`ui_wgpu`, `os_vcs` in `GuestRuntime` | `RESULT-RELOCATE-PLUGIN-SDK.md` |

## 0. Verdict

All five targets have moved, each through a neutral registration point, and every caller, TS twin, fixture and source-text reader was updated in the same pass. No compatibility re-export was left.

Rust was **not compiled**. The sanctioned `bun repo cargo` launcher refused every touched crate, first with "Finite original maintenance authority required" and later at the source-boundary gate (os-kernel, replication). All Rust evidence is therefore static:
- the patched full-resolution differential;
- definition and privacy scans, each with a seeded regression;
- slice B's stand-in type-check probe;
- rustfmt.

TS was verified with scoped `tsc`, bun and vitest.

The whole-tree dependency-cruiser count moved from 15,439 to 15,768, but the rise comes from concurrent wave-4 carving and peer work. Attributed to this relocation, the change is net 0 (section 3). It leaves two new edge instances of a pre-existing violation class (barrel re-exports of nested-owner TS twins), offset by removed edges.

The repo direction lints (`lint-rust-source-direction`, `lint-cargo-dependency-direction`, `lint-framework-neutral-product-direction`, `lint-dependency-direction`) could not run. The `bun repo` runner failed on peer infra ("Repository target execution contract refused", "Original selected owner registry changed", and a 900 s timeout). Static direction scans took their place (section 3).

## 1. What moved where

| Target | Owner decision (evidence in the slice report) | Registration point | Final location |
| --- | --- | --- | --- |
| History-edit ids, definitions and finalize dialog | Framework time-travel feature: the session reducer `semio-framework-time-travel`, plus the TS kernel/replication twins | `ReservedContribution`, listed in `FRAMEWORK_RESERVED_CONTRIBUTIONS` and consumed by the plugin builder and the mcp catalog | `⏪️time-travel/🕹️actions` = `semio_framework::history_edit` |
| Tool-run re-export, `app_declares_tool_run`, `tool_run_action_definitions` | Framework tool-run module | `ReservedContribution` (window actions) | `⏯️tool-run/🕹️actions` = `semio_framework::tool_run`. Callers use `semio_framework_tool_run` directly. |
| Gumball audience rule | os interaction hosts; only the plugin builder enforces it | `ReservedContribution.fixed_audience` | `🔌️plugin/🏗️builder/🧩️app/🖐️gumball-audience` |
| `Introduction*`, `Tutorial*`, tutorial engine | Framework UI feature (introduction overlay, TS tutorial engine) | `AppDefinition` typed fields, following the precedent of `interactions` | new framework module `🎓️onboarding` (crate `semio-framework-onboarding`) |
| `ActionKind`, `InteractiveJobClassification`, `AppRole`, `PackageRole` | Framework contracts the manifest's own types need. They are not product vocabulary. Docs were neutralised. | n/a | manifest `🕹️actions`, `🧮️arg-schema`, `🧾️plugin-manifest` crates |
| `OsMediaCapability`, `MediaClass`, `MediaForm` | Media wire vocabulary | n/a (opening is follow-up 3) | `🛂️manifest/🎞️media` (crate `semio-framework-manifest-media`) |
| tiled-map `🌍️gis-policy` and six further GIS leaks | GIS plugin. The viewer needs it too, so it lives in `🗺️tiles`, not the editor. | `MapPolicy` on `MapExtension`. Serialisable `MapPolicyDescriptor` sent inside the map descriptor, installed by `MapHost::sync_map_json`. Policy-declared layer toggles, overlay layers and style presets. | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🗺️tiles/🌍️gis-policy`; framework `🎛️map-policy` is neutral. GIS cartography tokens were removed from `ui_styling`. |
| hub `🌍️gis-map-binding` | GIS plugin's hub composition (`semio-hub-gis`) | New port crate `semio-hub-inference-binding` (`🌎️hub/💡️inference/🔗️binding`); the committer is generic over the port; registration happens in the hub bootstrap | `🌎️hub/🧩️compositions/🌍️gis/💡️inference`. `semio-hub` dropped the direct GIS plugin and stdio-semio `[dependencies]`. |
| puzzle 2d naming in `♾️infinite` (0 matches remain) | Puzzle 2d and 5d plugins; the LOD table is board vocabulary and was renamed `BOARD_LODS` | `BoardNaming` (static) plus the `BoardPlacementNaming` wire form on `Board2dScene.placement_naming`, installed by the wgpu `sync_board_engine` and the React `Board2dHost` | `P/⚙️engine/🎲️board-host` `PUZZLE_2D_BOARD_NAMING`. The os renderer's `puzzle_board_*`, `PUZZLE2D_*` and `Puzzle2d*` names are neutral. |
| plugin SDK tool-run/time-travel in the generic instance | Same crate and layer, but the instance hard-coded both domains | `InstanceExtension`: a tuple list registered at the plugin root, with static dispatch | `plugin/⏯️tool-run/🧵️instance`, `plugin/⏪️time-travel/🧵️instance` |
| plugin SDK `ui_wgpu` | The types are defined in renderer-neutral crates | n/a | imports come from `semio-framework-ui-layout`, `-ui-declaration` and `-ui-scene`; `ui_wgpu` is gone from both plugin `Cargo.toml` files; the crate root uses explicit `pub use` lists (88 names in use) |
| `os_vcs` in `GuestRuntime` | `semio-framework-os-vcs` owns the types (os-kernel only re-exported them) | n/a | `GuestRuntime` names the owning crate; the `os_vcs`, `opening_config` and `config_io` re-exports were removed and importers re-pointed |

Slice follow-ups the coordinator required in the same pass:
- **D:** the desktop wgpu lane minted `board.brush.<n>` while the other lanes minted `puzzle2d.brush.<n>`. This regression is now fixed through the scene contract, and the os renderer names are neutral.
- **E:** combined A+E baseline differential, and removal of `ui_wgpu`.
- **B:** the six remaining GIS leaks.

## 2. Coordinator fix: cross-owner TS imports from slice A

The whole-tree cruiser attribution (section 3) found 10 new `no-cross-package-relative-*` edges from slice A's TS twins. The rule admits a cross-owner import only when it resolves to an authored `package.json` export target. Fixed by `relo-coord-tools/cruiser_fix.py`, with pristine copies in `🗑️generated/relo-coord-baseline/`:

- The 10 generated `ToolRun*`/`JobKindId` wire aliases went back to the manifest TS twin `🛂️manifest/🟦️.ts`. They are manifest projection types. `⏯️tool-run/🕹️actions/🟦️.ts` now holds only the `TOOL_RUN_*` re-export from its own module and `appDeclaresToolRun`, whose `run` is now `unknown` because it only tests presence.
- The manifest TS twin re-exports the `🗺️window-layout` types. The kernel and the os config schema import `WindowLayout` from the manifest twin again, as they did at HEAD, and the redundant barrel line was removed.
- The history-edit TS test imports `DIALOG_CHOICE_ARG` from the `@semio-tech/framework` export target.

Verification:
- Scoped `tsc --noEmit` over the 7 touched files and their closure: 0 errors.
- Seeded probe: misusing `ToolRunDefinition` and `WindowLayout` through the barrel gives exactly 2 TS2322 errors.
- `bun test` in place: history-edit 1/1 pass, tool-run 3/3 pass. A repo-root multi-file invocation crashes Bun 1.3.14 with a segfault in Bun itself.
- Targeted cruiser on the 7 files: the slice-A sources went from 45 (before) to 47, offset by 4 removed edges elsewhere, as shown in section 3.

## 3. Direction verification

**dependency-cruiser, whole tree** (`🧰️framework ✏️s 🌎️hub ♻️mit-bestand`, policy config, `--output-type err`):
- Before (14:50): 15,439 violations, 16,170 modules (`🗑️generated/relo-coord-cruiser-before.txt`).
- After (20:00, before the section 2 fix): 15,768 violations, 16,366 modules.
- The multiset delta (`relo-coord-tools/cruiser_delta.py`, `relo-coord-cruiser-delta.log`) is dominated by peer and wave-4 rules: `no-cross-package-relative-@semio-tech/process` +116, `repo-lib` +63, schema-validator +58, canvas +16 and others. These come from the carves of surface, facade, os-infinite, pack, os-mcp, os-flow, db and stdio-contract, and from the GIS dev variant tests.

Attributed to this relocation:
- Moved edges replaced their originals 1:1, for a net of 0. Examples:
  - the engine-contract test now reads the onboarding fixture;
  - onboarding local-interaction now imports the replication wire;
  - the ui owned-locale test now imports onboarding.
- Slice A's 10 new edges were fixed in section 2.
- What remains is net 0, made up of 4 added and 4 removed edge instances:
  - **Added (4):**
    - the framework barrel re-exports `⏪️time-travel/🕹️actions/🟦️.ts` and `⏯️tool-run/🕹️actions/🟦️.ts`. This is the same class as 15 pre-existing barrel lines (kernel, schema, async ×4, sqlite-snapshot ×2, io-schema, viewport ×2, machine, value ×2), and it belongs to the SDK-barrel follow-up (SUMMARY item 5).
    - the moved tool-run TS test validates against the manifest JSON schema (counted under 2 rules).
  - **Removed (4):**
    - the old manifest tool-run test's 2 edges into tool-run;
    - manifest → `⏯️tool-run/🟦️.ts`;
    - manifest → kernel.
- Spot check of a slice-D file: the os renderer `react/🟦️.tsx` has 73 violations before, after and now. Apparent pairs for it in the delta are byte-mangled emoji in the `err` output.

**Static direction scan** (`relo-coord-tools/direction_scan.py`, `relo-coord-direction-scan.log`). Layers: framework < os < plugins < top-level products.

| Check | Result |
| --- | --- |
| Framework manifest names product vocabulary | 0 |
| Framework surface (tiled-map) names GIS | 0. The 7 hits are terrain docs (`🏔️terrain`, `🎨️paint`) outside this target, see follow-up 9. |
| os infinite names puzzle 2d | 0 |
| plugin host names `os_vcs`/`ui_wgpu` | 0 |
| `ui_wgpu` in plugin / plugin-host `Cargo.toml` | none |
| GIS plugin crates in hub `[dependencies]` | none |
| plugin instance names tool-run/time-travel | 4 intra-crate lines (`crate::tool_jobs::time_travel` in dispatch/ingress, `TOOL_RUN_TRACE_DELTA_BYTES`, one doc). This is state the domain contracts still own, see follow-up 6. |
| hub inference names GIS | test code (dev-dependencies) and the catalog's `gis_map_binding` vocabulary over the new port, see follow-up 5 |
| Upward Cargo `path =` | 1, pre-existing at HEAD: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services` → `🌎️hub/🔐️auth/🔌️client` |
| Upward `#[path]`/`include*` | 18, all pre-existing at HEAD and outside the relocation: framework value/pack/replication/ui/job tests → os store and plugin fixtures |

Every slice also ran its own scans: no upward Cargo or source edge was introduced by A, B, C, D or E.

## 4. Rust differential summary (details in the slice reports)

- **A:** mcp, mcp catalog and os-renderer-wgpu are identical in native, test and wasm32. The plugin crate shows only unresolved external crates. The facade run was mid-carve and cannot be attributed; the carver's per-crate oracle supersedes it.
- **B:** model, host, gismap, ui-scene and ui-styling crates show no new first-party diagnostics in native, test or wasip2. The os renderer was checked native only; its test configuration did not finish in 600 s. The stand-in type-check probe gives 0 errors on native and wasip2, and its seeds are caught.
- **C:** the hub lib in four configurations, the `os-hub` bin, `semio-hub-gis` including wasip2, and the port crate: no new in-crate errors. One real regression (a renamed WAL parameter in test hooks) was caught and fixed.
- **D:**
  - os-infinite, ui-scene-plane, puzzle-2d and puzzle-5d (6 configurations each): only unresolved externals, attributes or imports.
  - os-renderer-wgpu (native, `--test`, wasm32): identical.
- **E:** after the combined A+E baseline, plugin (7 configurations) and plugin-host (2): 0 new internal keys, except 3 `cannot find attribute serde` keys from slice A's moved gumball test, which come from an unresolved external derive.

## 5. Residual risks and follow-ups

1. **Real compile.** Run `check` and `test` (native, `--tests`, and the wasm targets each crate builds for) once the launcher and gate admit them, for:
   - framework: onboarding, time-travel-actions, tool-run-actions, `manifest-*`, surface tiled-map host/model/mvt, ui-scene(-plane), ui-styling;
   - os: plugin, plugin-host, os-run, os-mcp(-catalog), os-renderer-wgpu, os-infinite;
   - hub and plugins: `semio-hub`, `semio-hub-gis`, `semio-hub-inference-binding`, gismap, puzzle-2d/5d, stdio-pdf, and the touched plugin test crates.
   
   Typeck-phase privacy, trait bounds (the generic hub committer, `Send` through the `InstanceExtension` tuple) and serde derives are unproven.
2. **Lock refreshes:** the root `Cargo.lock`, `📸️remodeling`, `🔋️model`, `🌎️hub` and gismap (`update --workspace --offline`).
3. **Open `MediaForm` as a registered id.** 9 of its 16 variants are plugin topics. Also retire `OsMediaCapability` onto `MediaForm`, and rename the `InteractiveJobClassification` variants (`Migrated`, `BatchOnlyPendingRewrite`). Plans are in report A §6. This needs the compile window, because there are about 200 match sites and roughly 140 to 300 files per change.
4. The mcp catalog publishes `startIntroduction` but not `startTutorial`/`recordTutorial` (report A §6.4).
5. **Hub GIS vocabulary:** about 650 `GisMap*` identifiers, the `GIS_*` constants, the catalog checks and the bootstrap routes. There is a five-step plan in report C. Peers are migrating APIs in that code at the same time.
6. **Plugin SDK:** instance state still names the two domains (ledger fields, receipt/presence methods, render overlays, history panel, close-ladder rungs). W5 (`include!` composition in the instance) is not done.
7. os renderer `🗣️Interpreter/🟦️.tsx` `classifierKind === "puzzle2d"` needs a plugin-declared classifier registration point.
8. ui-scene defaults (`"colored"`, the `/osm/` and `/vt/` tile templates) and palette slot names (`positionFill`, `routeStroke`) still carry cartography vocabulary.
9. Framework surface terrain docs name the GIS plugin. The terrain owner is a wave-4 SURFACE scope item.
10. Barrel re-exports of nested-owner TS twins: 17 lines, 2 of them new here. These belong to the SDK-barrel work packages 7 and 8.
11. Out-of-repo plugins that relied on other `ui_wgpu` names through the plugin SDK glob must import them from the owning crates.

## 6. Files

- **Coordinator (this session):** `📓️RELOCATION-BRIEF.md`, this report, `relo-coord-tools/{direction_scan.py,cruiser_delta.py,cruiser_fix.py}`, and the logs `🗑️generated/relo-coord-*`.
- **Source edits by the coordinator:** `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`, `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts`, `🧰️framework/📦️packages/🟦️typescript/🟦️.ts`, `🧰️framework/🔨️modules/⏯️tool-run/🕹️actions/🟦️.ts`, `🧰️framework/🔨️modules/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts`.
- **Slice files:** listed in each `RESULT-RELOCATE-*.md`, with tools in `relo-{manifest,tiledmap,hubgis,board,pluginsdk}-tools/`.
- No modifying git command, no worktree, no AGENTS.md edit, and no ticket opened or closed. Both repo MCP ticket tools failed for this fleet earlier, so the work lives in this subfolder.
