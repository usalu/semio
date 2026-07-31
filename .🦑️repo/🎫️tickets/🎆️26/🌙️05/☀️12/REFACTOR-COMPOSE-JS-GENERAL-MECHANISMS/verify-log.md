# Verification — directive pass (2026-05-12)

## Phase B next slice (2026-05-12) — `compose/js` only

### Done

- **`*Entity` renames**: no `FileEntity` / `*Entity` class names remained in `compose/js`; strong handles are already `File`, `Folder`, `Layer`, `Group`, `Stat`, `Prop`, etc.
- **`Design` id-list-stable pieces + connections**: **`readPieces`**, **`subscribePieces`**, **`readConnections`**, **`subscribeConnections`** (internal `__stablePieces` / `__stableConnections` via `__readIdListStable`). React `useDesignPieces` can switch to `design.readPieces()` in a follow-up (this slice does **not** edit `compose/react/index.tsx` per directive).
- **VCS / collections (types-only, no new worker surface)**: **`Graph`** — `readAlternatives`, `readCheckpoints`, `subscribeAlternatives`, `subscribeCheckpoints`; **`Session`** — `readAlternativeIds`, `readAlternatives`, `subscribeAlternatives`; **`Checkpoint`** — `readChangeIds`, `readEditIds`, `readChanges`, `readEdits`, `subscribeChanges`, `subscribeEdits`; helper **`__parseStrongEntityArrayIds`** for non-relay `[StrongEntity!]` lists (e.g. checkpoint `changes`).
- **Embedded vitest**: extended **`compose-js Kit facade (strict)`** with assertions that the new `read*` / `subscribe*` methods exist on `Design` / `Graph` / `Session` / `Checkpoint`.

### Commands

| Command                                                                                         | Exit                                                                                               |
| ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `bunx tsc --noEmit` in `compose/js`                                                             | **0**                                                                                              |
| `$env:COMPOSE_JS_RUN_EMBEDDED_TESTS='1'; bunx vitest run index.ts` in `compose/js` (PowerShell) | **0** (11 tests)                                                                                   |
| `set COMPOSE_JS_RUN_EMBEDDED_TESTS=1&& bunx vitest run index.ts` (cmd-style)                    | **1** — env not propagated to Node on this runner (`No test suite found`); use PowerShell `$env:…` |

### Files

- `compose/js/index.ts`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)
- `compose/js/kit-store.worker.ts` — **unchanged** (regions already present; no further edits this slice)

---

## Phase C continuation (2026-05-12) — react + sketchpad

### Files

- `compose/react/index.tsx` — Kit list hooks use **`Kit#readDesigns` / `readTypes` / `readAuthors` / `readQualities` / `readTags` / `readConcepts`** (`useKitDesigns`, `useKitTypes`, …); **`useDesigns` / `useTypes` / `usePieces`** bundles for sketchpad; **`useDesignPieces` / `useDesignConnections`**; entity reads renamed to **`usePieceContextRead` / `useTypeContextRead` / `useQualityContextRead`**; dropped stale **`DESIGN_*` / `KIT_*_SPECS`** re-exports that no longer exist on `@semio-tech/compose-js`.
- `compose/sketchpad/index.tsx` — **`KitTabContextProvider`** (was kit tab “scope” wrapper); route **`contextProvider`**; **`RouteParamContextShell`**; **`SketchpadInstance` / `SketchpadInstanceContext` / `SketchpadInstanceProvider` / `useSketchpadInstance`**; **`DesignAppShell*` / `QualityAppShell*` / `TypeAppShell*`**; fixed **`TypeAppShellProvider`** (was commented out); **`LayoutContextHost` / `ToolbarContextHost`**; **`useXStateFieldWithContext`**; imports **`use*ContextRead`** + list hooks from `@semio-tech/compose-react`.

### Commands

| Command                                    | Exit  |
| ------------------------------------------ | ----- |
| `bunx tsc --noEmit` in `compose/react`     | **0** |
| `bunx tsc --noEmit` in `compose/sketchpad` | **0** |
| `bunx vitest run` in `compose/react`       | **0** |

### Notes

- `getDefaultSketchpadInstanceId` replaces `getDefaultSketchpadScopeId` (embedded vitest string checks updated).

---

## Phase C pass (2026-05-12) — react + sketchpad

### Files

- `compose/react/index.tsx` — `ActiveKitTabContext` / `KitWasmMountProvider` / `useKitWasmHost` / `useKitOptional` / entity context row + `PieceUnderActiveDesignProvider` + `ConnectionUnderActiveDesignProvider` / id-stable entity list hooks / legacy `use*ScopedRead` shims / sketchpad kit types / vitest banned-pattern scan (excludes self-region) + robust `index.tsx` path for vitest on Windows.
- `compose/sketchpad/index.tsx` — `*Scope*` → `*Context*` / `ActiveKitTab` / `KitWasmMountProvider` / `useKitWasmHost` call sites; `getKitSnapshot` / `useKitCommandsById` aligned to `kitTabId`; fixed accidental `usePieceScope`→substring corruption in `usePieceScopedRead` names.

### Commands

| Command                                    | Exit                                                                                             |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| `bunx tsc --noEmit` in `compose/react`     | **0**                                                                                            |
| `bunx tsc --noEmit` in `compose/sketchpad` | **0** (inherits js `include`; does not typecheck monolithic `index.tsx` unless explicitly added) |
| `bunx vitest run` in `compose/react`       | **0**                                                                                            |

### Notes

- Sketchpad still imports many symbols not yet restored on `@semio-tech/compose-react`; full `index.tsx` strict check remains for a follow-up when the barrel is complete.

---

## Constraint applied (user)

- **No** `KitRuntime` / embed-host umbrella in React: context holds **`Kit` only**; `useKit()` returns **`Kit`**. Materialization read point stays in provider state and is applied via **`Kit#setReadPoint`** (not exposed as a synthetic runtime object).
- **JS**: VCS navigation uses **entity classes** aligned with the plan: `Graph`, `Session`, `TheKit`, `Checkpoint`, `Alternative`, `Change`, `Edit`, `Conflict`, abstract **`Operation`**, plus **`Kit#wip` / `#authoritative` / `#session` / `#conflict`**.
- **React**: **`useWipGraph`**, **`useAuthoritativeGraph`**, **`useSession`** (no shim). Optional **`GraphContextProvider` + `useGraph()`** when a subtree must bind `GraphRootKind` explicitly.
- **Algorithms story**: `FindReplaceableTypesInDesigns` now imports **`Kit` from `../../index`** (algorithms façade `Kit.ensure`) — not `@semio-tech/compose-react` — removed **`Kit as KitRuntime`** pattern.

## Commands (this pass)

- `bunx tsc --noEmit` in `compose/js` — **exit 0**
- `bunx tsc --noEmit` in `compose/react` — **exit 0**
- `bunx tsc --noEmit` in `compose/sketchpad` — **exit 0** (note: sketchpad `tsconfig` inherits `include` from `compose/js`; giant `index.tsx` may be outside default program — treat as smoke only until sketchpad tsconfig includes app entry)

## Remaining plan (for follow-up)

- Rust: geom weak single-struct collapse; serde_json confinement; exhaustive `Event::canonical_touched_paths`.
- JS: weak **classes** (`Position`, …), full operation roster, `EntityRef`, purge `KIT_*` specs, private Json surface.
- React: full field-hook inventory; vitest negative-greps vs plan list; migrate **sketchpad** off legacy `useKitRuntimeSafe` / `useKitScope` imports (still present in source).
- `npm run depcruise:layers`; `cargo check` / `cargo test` matrix with isolated `--target-dir`.

---

## Continuation pass (2026-05-12) — Rust `PositionNode` SSOT

### Code change (partial `rust-weak-collapse`)

- `geom::entity::PositionNode` no longer stores a duplicate `RwLock<geom::Position>`; live state is **only** `center` + `plane` child nodes. `snapshot_value()` / `compute_hash()` assemble from those locks. Piece drag JSON apply updates `center` directly (same effective behavior as the old data+center sync).

### Commands (this pass)

| Command                                                                                               | Exit                                                                                  |
| ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `cargo check -p compose --target-dir target-ssel4`                                                    | **0**                                                                                 |
| `cargo check -p compose --target wasm32-unknown-unknown --target-dir target-ssel4`                    | **0**                                                                                 |
| `cargo test -p compose --target-dir target-ssel4 schema_matches_target_graphql_file`                  | **0**                                                                                 |
| `cargo test -p compose --target-dir target-ssel4 no_deep_clone_on_traversal`                          | **0**                                                                                 |
| `cargo test -p compose --target-dir target-ssel4 kit_store_golden_ops_via__op_json_match_fingerprint` | **101** (fixture `kit-store.golden.operations` not found on this runner — `NotFound`) |
| `bunx tsc --noEmit` in `compose/js`                                                                   | **0**                                                                                 |
| `bunx tsc --noEmit` in `compose/react`                                                                | **0**                                                                                 |
| `bunx tsc --noEmit` in `compose/sketchpad`                                                            | **0**                                                                                 |
| `npm run depcruise:layers`                                                                            | **n/a** — script not present in workspace `package.json`                              |

### remaining-work (plan checklist — ticket **closed** after Phase A2 append; reopen for next slice)

1. **Rust weak geom**: ~~Finish true **one-type-per-weak-geom** collapse for `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Location`~~ **(partial)** — live `geom::entity::{…}` + wire `*Input` split landed; **`Attribute`** still meta-only / not collapsed here; macro/SDL naming may still need alignment passes.
2. **Rust bundle / serde_json**: ~~Remove remaining `KitStoreBundle` / snapshot DTO paths per plan; confine `serde_json` to GraphQL decode + `DevBackbone` I/O only.~~ **(partial)** — legacy **`KitStoreBundleFile` / `GraphSnapshotDto` / …** renamed to **`DevBackboneBundleDoc` / `DevBackboneGraphHead` / …**; **`initialKit` projection** JSON build/hydrate moved into `kit_backbone`; **`CanonicalKitDiff.types` / `designs`** are typed envelopes (**Phase A3**); inner **`modified[].diff`**, **`files` / `folders` / `authors`**, tag/concept/quality **`added`**, tests, WASM bootstrap, **`stored_ops`** JSON — still `Value` where not yet typed.
3. **Rust `Event::canonical_touched_paths`**: ~~Extend~~ **(partial)** — added `…:designs` paths for piece events; still coarse for future op variants / full field tree vs plan.
4. **Rust `rust-sub-fieldgate` / macros / SDL**: Per plan YAML (`rust-sub-fieldgate`, `rust-macros`, `rust-sdl-roundtrip`, `rust-vcs-canonical`, `rust-change-algebra-canonical`).
5. **JS**: Weak entities as `class` + caches; `*Entity` renames; VCS + change-algebra classes; `Entity` + `defineField` / `defineOperations`; purge `KIT_*`; private Json wire (`js-purge-json`, `js-drop-fieldspecs`, …).
6. **React**: `*Scope` → `*Context` on all exports; field hooks + owned collections; bridges region; no `useSyncExternalStore` (grep-clean + vitest negatives per plan).
7. **Sketchpad**: Migrate off `useKitRuntimeSafe`, `useKitScope`, `KitScope` / `KitScopeProvider` to the renamed React API once (6) lands.
8. **Verify**: Run `kit_store_golden_ops_via__op_json_match_fingerprint` where golden files are checked out; add `depcruise:layers` if/when scripted; broader `cargo test -p compose` when fixtures available.

---

## Phase A slice (2026-05-12) — Rust weak-geom + touches

### Weak geom (single kind per SDL weak entity in `geom::entity`)

- Wire-only inputs: `VectorInput`, `PointInput`, `CoordinateInput`, `OffsetInput`, `PlaneInput`, `PositionInput` (+ existing `LocationInput`).
- Canonical live nodes renamed from `*Node` to `Coordinate`, `Vector`, `Point`, `Plane`, `Offset`, `Position`, `Location`, `Place` under `geom::entity`; `Position::snapshot_input` replaces the old Copy `Position` snapshot.
- `operation::Input::FixedPiece` / `CreateFixedPieceInput` / kit diff paths / GraphQL resolvers use `PositionInput`; drag paths use `OffsetInput`.
- `DesignSlot` (was `DesignHandle`) for kit-graph slot indirection only.
- Removed unused SDL enum `KitGraphWorkspace` (not in `target.schema.graphql`).
- `Event::canonical_touched_paths`: piece-affecting events also list `…:designs` paths for subscription gating.

### Not done in this slice (still for follow-up / other workers)

- ~~`KitStoreBundleFile` / snapshot DTO stack and broad `serde_json::Value` confinement to DevBackbone + GraphQL decode only~~ **(partial, Phase A2)** — types renamed to `DevBackbone*`; `initialKit` projection JSON isolated in `kit_backbone`; full `Value` purge still open.
- Full macro-driven weak-entity registration churn beyond re-pointing `entity_full_family!` at renamed `geom::entity` kinds.

### Commands (`--target-dir c:\\git\\compose\\target-phaseA`)

| Command                                                                                                                     | Exit  |
| --------------------------------------------------------------------------------------------------------------------------- | ----- |
| `cargo check -p compose --target-dir c:\git\compose\target-phaseA`                                                          | **0** |
| `cargo check -p compose --target wasm32-unknown-unknown --target-dir c:\git\compose\target-phaseA`                          | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA schema_matches_target_graphql_file`                        | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA no_deep_clone_on_traversal`                                | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA kit_store_bundle_serialize_hydrate_round_trip_via_graphql` | **0** |

### Files touched

- `compose/rs/lib.rs` only (no `compose/graphql/*.graphql` edits — schema guard still green).

### Conflicts avoided

- Did **not** edit `compose/js/index.ts` or `compose/react/index.tsx` (per parent directive).
- Ticket `ticket_reopen` returned “already open”; no duplicate ticket.

---

## Phase B — JS (`compose/js/index.ts`, `compose/js/kit-store.worker.ts`) — 2026-05-12

### Done (sub-pass)

- **Weak geometry + wire**: `Position`, `Coordinate`, `Plane`, `Point`, `Vector`, `Offset`, `Place`, `Location`, `Camera` as **classes** with parent/role; `PositionInput` / `OffsetInput` + `formatPositionInput` / `formatOffsetInput` kept as **plain mutation structs** (file-local helpers under `🪶️WeakGeometry`); removed duplicate tail `🪶️WeakEntities` interfaces.
- **Artifacts**: `Attribute` and `Benchmark` as **classes** (`Attribute.owner`, `Benchmark.quality`); `ConnectionSide` as **class** + `export type Side = ConnectionSide`; `parseAttributeConnectionUnder(ownerEntity, json)`.
- **Caches**: `Kit` factories + `wip`/`authoritative`/`session`; `Design` piece/connection/layer/group; `Type` port/connector/representation; `Graph` checkpoint/alternative + `TheKit`; `Session` alternative; `Checkpoint` edit/change; id-list-stable **`readDesigns` / `readTypes` / `readAuthors` / `readQualities` / `readTags` / `readConcepts`** via `__readIdListStable`.
- **VCS + change algebra**: `__kitGraphqlEnvelope` for navigators; removed public **`Kit#runGraphql`** and legacy subscription **`operationSucceeded`** fan-out; **`backboneSyncNow`**, **`backboneStatus`** (typed snapshot); extended **`🧮️ChangeAlgebra`** (`Diff`, `Modification`, `Modifications`, `Input`, `ChangeLedgerEvent`, diff/mod/input/operation variant shells incl. **`RenamedKit`**).
- **`EntityRef`**: `export type EntityRef = Entity`.
- **Worker**: `kit-store.worker.ts` split into **`//#region`** blocks (header, handle, wire, onmessage).

### Commands

| Command                             | Exit  |
| ----------------------------------- | ----- |
| `bunx tsc --noEmit` in `compose/js` | **0** |

### Files touched

- `compose/js/index.ts`
- `compose/js/kit-store.worker.ts`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

### Note

- **`KIT_*_SPECS` / `defineFields` / `defineOperations`** kept **exported** for parallel **`compose/react`** re-exports (not edited here). Full purge waits React pass.
- **`// @ts-nocheck`** retained: embedded vitest block still references legacy `Graph.*` kit-store types.

---

## Phase A2 slice (2026-05-12) — Rust dev backbone doc rename + initialKit projection SSOT

### Done

- **DTO rename (plan `rust-bundle-fold` partial)**: `KitStoreBundleFile` → **`DevBackboneBundleDoc`**, `GraphSnapshotDto` → **`DevBackboneGraphHead`**, `TheKitVersionDto` → **`DevBackboneTheKitHead`**, `AlternativeVersionDto` → **`DevBackboneAltHead`** (wire shape unchanged; serde keys unchanged).
- **`serde_json::Value` for `initialKit` projection**: moved off **`Kit`** / **`Design`** into **`//#region 🔖️ dev_backbone_initial_kit_projection`** inside `kit_backbone`: `initial_kit_projection_value`, `hydrate_kit_from_initial_projection_value`, `hydrate_design_pieces_from_snapshot_value`, `graph_new_overlay_from_initial_projection_json`.
- **`Kit`**: removed `kit_full_snapshot_value` / `hydrate_from_kit_full_snapshot_json`; `deep_clone` delegates to backbone helpers.
- **`Design`**: removed `hydrate_pieces_from_snapshot_json` (logic lives in `kit_backbone`).
- **`Graph`**: `new_overlay_from_kit_json` → **`new_overlay_from_initial_kit_projection_json`** (delegates to `kit_backbone::graph_new_overlay_from_initial_projection_json`).
- **`ParentRuntime`**: `spawn_wip_overlay_from_kit_dto` → **`spawn_wip_overlay_from_initial_kit_projection_json`**; WASM bootstrap updated.
- **GraphQL schemas**: unchanged (`schema_matches_target_graphql_file` green); no `compose/js` / `compose/react` edits.

### Not done (still broad `serde_json::Value` outside backbone + gql decode)

- **`operation::CanonicalKitDiff`**, **`Kit::apply_*_diff_json`**, golden/test JSON fixtures, **`stored_ops_from_golden_ops_json`**, WASM bootstrap `bootstrap_runtime_from_json_value`, etc. — still use `Value`; full confinement waits a larger pass.

### Commands (`--target-dir c:\git\compose\target-phaseA2`)

| Command                                                                                               | Exit  |
| ----------------------------------------------------------------------------------------------------- | ----- |
| `cargo check -p compose --target-dir c:\git\compose\target-phaseA2`                                   | **0** |
| `cargo check -p compose --target wasm32-unknown-unknown --target-dir c:\git\compose\target-phaseA2`   | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA2 schema_matches_target_graphql_file` | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA2 kit_store_bundle`                   | **0** |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA2 no_deep_clone_on_traversal`         | **0** |

### Files touched

- `compose/rs/lib.rs`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

---

## Phase A3 slice (2026-05-12) — Rust `rust-bundle-fold` kit diff envelope + piece drag wire

### Done

- **`CanonicalKitDiff.types` / `.designs`**: replaced top-level `Option<serde_json::Value>` with **`TypesCollectionDiff`** / **`DesignsCollectionDiff`** (`removed` / `modified` / `added` rows); **`TypeModifiedWireRow` / `DesignModifiedWireRow`** carry `diff: serde_json::Value` so **`metabolism.kit.diff.compose.json` round-trip** stays byte-stable.
- **`Kit::apply_diff`**: **`apply_types_collection_diff`** / **`apply_designs_collection_diff`** (no outer `Value::as_object` walk); **`KitOperation::to_diff`** builds typed envelopes for entity description/icon/image on kinds + piece lifecycle ops.
- **Weak geom / drag**: **`PieceDiffWire`** deserializes `fixPiece`, **`OffsetInput`** `drag`, **`PositionInput`** `pose`; drag deltas in `to_diff` use **`serde_json::to_value(offset)`** from `OffsetInput`.
- **`//#region 🔖️canonical_kit_types_designs_wire`** wraps the new wire structs.

### Still open (same ticket themes / follow-up)

- **`types.modified[].diff`** / **`designs.modified[].diff`** inner trees, **`files` / `folders` / `authors`**, tag/concept/quality **`added` rows**, **`stored_ops` / WASM bootstrap** `Value`, **`DevBackboneBundleDoc`** internals — still `serde_json::Value` where not touched here.

### Commands (`--target-dir c:\git\compose\target-phaseA3`)

| Command                                                                                                                  | Exit            |
| ------------------------------------------------------------------------------------------------------------------------ | --------------- |
| `cargo check -p compose --target-dir c:\git\compose\target-phaseA3`                                                      | **0**           |
| `cargo check -p compose --target wasm32-unknown-unknown --target-dir c:\git\compose\target-phaseA3`                      | **0**           |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA3 schema_matches_target_graphql_file`                    | **0**           |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA3 canonical_kit_diff_metabolism_fixture_json_round_trip` | **0**           |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA3 no_deep_clone_on_traversal`                            | **0**           |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseA3 kit_store_bundle`                                      | **0** (6 tests) |

### Files touched

- `compose/rs/lib.rs`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

---

## Phase D — verification matrix (2026-05-12)

### Done

- **Golden ops fixture**: tests pointed at non-existent `kit-store.golden.operations.compose.json`; repo ships **`assets/compose/kit-store.golden.ops.compose.json`** with key **`ops`**. Added **`kit_backbone::golden_ops_records_ref`** (accepts `operations` or `ops`); all golden readers use **`kit-store.golden.ops.compose.json`** + helper / `stored_ops_from_golden_ops_json`.
- **`KitOperation::to_diff` JSON**: fixed **`DragPieceInDesign`** / **`FixPieceInDesign`** `serde_json::json!` closing (`})` vs `),`) and **`}}]`** → **`}]`** so `cargo check` parses.
- **`compose/algorithms` tsc**: `openKit(JSON.stringify(__toBootstrap(kit)))`; dropped removed **`KitBootstrapJson`** in favor of **`JsonObject`**.
- **`compose/ui` tsc** (algorithms pulls ui): local **`Attribute` / `Coordinate` / `Plane` / `ComposeVector`** as **`JsonObject`** aliases; **`VectorValue`** explicit `{x,y,z}`; **`toSceneVector`** accepts **`JsonValue`** with numeric coercion.
- **Nx `workspace:depcruise`**: **`project.json`** switched **`bunx dependency-cruiser@16`** → **`npx --yes dependency-cruiser@16`** (Bun could not resolve `dependency-cruiser@16` script on this runner).

### Commands (`--target-dir c:\git\compose\target-phaseD` unless noted)

| Command                                                                                                                         | Exit                                                                                           |
| ------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `cargo check -p compose --target-dir c:\git\compose\target-phaseD`                                                              | **0**                                                                                          |
| `cargo check -p compose --target wasm32-unknown-unknown --target-dir c:\git\compose\target-phaseD`                              | **0**                                                                                          |
| `cargo test -p compose --target-dir c:\git\compose\target-phaseD`                                                               | **0** (36 passed, 1 ignored)                                                                   |
| `bunx tsc --noEmit` in `compose/js`                                                                                             | **0**                                                                                          |
| `bunx tsc --noEmit` in `compose/react`                                                                                          | **0**                                                                                          |
| `bunx tsc --noEmit` in `compose/sketchpad`                                                                                      | **0**                                                                                          |
| `bunx tsc --noEmit` in `compose/algorithms`                                                                                     | **0**                                                                                          |
| `bun nx run workspace:depcruise`                                                                                                | **0** (after `project.json` fix; was **1** with `bunx dependency-cruiser@16` script-not-found) |
| `npx --yes dependency-cruiser@16 compose/js compose/react compose/sketchpad --config .dependency-cruiser.cjs --output-type err` | **0** (same graph as Nx target)                                                                |
| `bunx vitest run` in `compose/react`                                                                                            | **0**                                                                                          |

### Files touched

- `compose/rs/lib.rs`
- `compose/ui/index.tsx`
- `compose/algorithms/index.ts`
- `project.json`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

---

## Wire-type audit (2026-05-12) — no `*Wire` / `*WireRow` names; piece diff without extra struct

### Removed / renamed (policy: align with `target.schema.graphql` input names + existing row patterns; no ad-hoc `*Wire`)

| Before                                             | After / action                                                                                                                                                                  |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `operation::TypeModifiedWireRow`                   | **`TypeModifiedRow`** (same shape as `TagModifiedRow`; `diff` remains `serde_json::Value`)                                                                                      |
| `operation::DesignModifiedWireRow`                 | **`DesignModifiedRow`**                                                                                                                                                         |
| `operation::PieceDiffWire`                         | **removed** — `Kit::apply_design_piece_modified_json` reads `fixPiece` / `drag` / `pose` from `serde_json::Value` and deserializes **`OffsetInput`** / **`PositionInput`** only |
| `WireReq` (WASM GraphQL JSON envelope)             | **`GraphqlExecuteJson`**                                                                                                                                                        |
| `request_from_wire`                                | **`graphql_execute_request_from_str`**                                                                                                                                          |
| `//#region 🔖️canonical_kit_types_designs_wire`     | **`//#region 🔖️canonical_kit_types_designs_mod`**                                                                                                                               |
| `compose/ui` **`KitPortWire`**                     | **`KitPortPlain`** (still `JsonObject` extension; naming matches `KitKindPlain`)                                                                                                |
| Docstrings / tests mentioning “wire” for transport | Rephrased to JSON / SDL-aligned wording in **`compose/js/index.ts`**, **`compose/react/index.tsx`**, **`compose/ui/index.tsx`**                                                 |

### Commands (`--target-dir` under ticket folder)

| Command                                                                                                             | Exit                         |
| ------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| `cargo test -p compose --target-dir c:/git/compose/.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/target-wire-audit` | **0** (36 passed, 1 ignored) |
| `bunx tsc --noEmit -p compose/js/tsconfig.json`                                                                     | **0**                        |
| `bunx tsc --noEmit -p compose/react/tsconfig.json`                                                                  | **0**                        |
| `bunx tsc --noEmit -p compose/sketchpad/tsconfig.json`                                                              | **0**                        |
| `bunx tsc --noEmit -p compose/algorithms/tsconfig.json`                                                             | **0**                        |
| `bunx tsc --noEmit -p compose/ui/tsconfig.json`                                                                     | **0**                        |
| `bun nx run workspace:depcruise`                                                                                    | **0**                        |

### Files touched

- `compose/rs/lib.rs`
- `compose/js/index.ts`
- `compose/react/index.tsx`
- `compose/ui/index.tsx`
- `compose/algorithms/index.ts`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

---

## Parallel plan pass + wire-surface tighten (2026-05-12)

### Parallel agents (read-only)

- **Rust**: `geom::entity::*` + `*Input` wire split; no `PositionNode`; bundle DTOs absent from product code (`DevBackbone*` in use).
- **JS**: regions `🌐️Transport` / `🧬️Entity` / `🪶️WeakGeometry`; weak geometry already `class`; `KIT_*_FIELD_SPECS` absent.
- **React**: no `useSyncExternalStore` calls; `KitContext` naming; gaps noted for backbone hooks and full K-hook roster.

### Done this slice

- **`compose/js`**: `JsonValue` / `JsonObject` are **file-local** wire aliases (no longer exported) per plan `js-purge-json`.
- **`compose/algorithms`**: local **`GqlWireObject`**; dropped `JsonObject` import from `@semio-tech/compose-js`.
- **`compose/ui`**: local **`PlainJsonObject` / `PlainJsonValue`**; removed `@semio-tech/compose-js` type import; **`toSceneVector`** accepts `unknown` for piece plane origins.
- **`compose/react`**: **`useAttachBackbone`**, **`useDetachBackbone`**, **`useBackboneSyncNow`**, **`useBackboneStatus`** (`🪝️BackboneOps` under kit hooks), wired to existing `Kit` GraphQL methods.

### Commands

| Command                                                                            | Exit                                                                                         |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `bunx tsc --noEmit -p compose/js/tsconfig.json`                                    | **0**                                                                                        |
| `bunx tsc --noEmit -p compose/react/tsconfig.json`                                 | **0**                                                                                        |
| `bunx tsc --noEmit -p compose/ui/tsconfig.json`                                    | **0**                                                                                        |
| `bunx tsc --noEmit -p compose/algorithms/tsconfig.json`                            | **0**                                                                                        |
| `bunx tsc --noEmit -p compose/sketchpad/tsconfig.json`                             | **0**                                                                                        |
| `$env:COMPOSE_JS_RUN_EMBEDDED_TESTS='1'; bunx vitest run index.ts` in `compose/js` | **0** (11 tests)                                                                             |
| `npx nx run workspace:depcruise`                                                   | **0**                                                                                        |
| `cargo test -p compose`                                                            | **not completed** — blocked on global Cargo artifact directory file lock in this environment |

### Files

- `compose/js/index.ts`
- `compose/algorithms/index.ts`
- `compose/ui/index.tsx`
- `compose/react/index.tsx`
- `.repo/🎫️/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)
