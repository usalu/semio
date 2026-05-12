# Verification — directive pass (2026-05-12)

## Constraint applied (user)

- **No** `KitRuntime` / embed-host umbrella in React: context holds **`Kit` only**; `useKit()` returns **`Kit`**. Materialization read point stays in provider state and is applied via **`Kit#setReadPoint`** (not exposed as a synthetic runtime object).
- **JS**: VCS navigation uses **entity classes** aligned with the plan: `Graph`, `Session`, `TheKit`, `Checkpoint`, `Alternative`, `Change`, `Edit`, `Conflict`, abstract **`Operation`**, plus **`Kit#wip` / `#authoritative` / `#session` / `#conflict`**.
- **React**: **`useWipGraph`**, **`useAuthoritativeGraph`**, **`useSession`** (no shim). Optional **`GraphContextProvider` + `useGraph()`** when a subtree must bind `GraphRootKind` explicitly.
- **Algorithms story**: `FindReplaceableTypesInDesigns` now imports **`Kit` from `../../index`** (algorithms façade `Kit.ensure`) — not `@semio/react` — removed **`Kit as KitRuntime`** pattern.

## Commands (this pass)

- `bunx tsc --noEmit` in `semio/js` — **exit 0**
- `bunx tsc --noEmit` in `semio/react` — **exit 0**
- `bunx tsc --noEmit` in `semio/sketchpad` — **exit 0** (note: sketchpad `tsconfig` inherits `include` from `semio/js`; giant `index.tsx` may be outside default program — treat as smoke only until sketchpad tsconfig includes app entry)

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

| Command | Exit |
|--------|------|
| `cargo check -p semio --target-dir target-ssel4` | **0** |
| `cargo check -p semio --target wasm32-unknown-unknown --target-dir target-ssel4` | **0** |
| `cargo test -p semio --target-dir target-ssel4 schema_matches_target_graphql_file` | **0** |
| `cargo test -p semio --target-dir target-ssel4 no_deep_clone_on_traversal` | **0** |
| `cargo test -p semio --target-dir target-ssel4 kit_store_golden_ops_via__op_json_match_fingerprint` | **101** (fixture `kit-store.golden.operations` not found on this runner — `NotFound`) |
| `bunx tsc --noEmit` in `semio/js` | **0** |
| `bunx tsc --noEmit` in `semio/react` | **0** |
| `bunx tsc --noEmit` in `semio/sketchpad` | **0** |
| `npm run depcruise:layers` | **n/a** — script not present in workspace `package.json` |

### remaining-work (plan checklist not satisfied — ticket **open**)

1. **Rust weak geom**: ~~Finish true **one-type-per-weak-geom** collapse for `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Location`~~ **(partial)** — live `geom::entity::{…}` + wire `*Input` split landed; **`Attribute`** still meta-only / not collapsed here; macro/SDL naming may still need alignment passes.
2. **Rust bundle / serde_json**: Remove remaining `KitStoreBundle` / snapshot DTO paths per plan; confine `serde_json` to GraphQL decode + `DevBackbone` I/O only.
3. **Rust `Event::canonical_touched_paths`**: ~~Extend~~ **(partial)** — added `…:designs` paths for piece events; still coarse for future op variants / full field tree vs plan.
4. **Rust `rust-sub-fieldgate` / macros / SDL**: Per plan YAML (`rust-sub-fieldgate`, `rust-macros`, `rust-sdl-roundtrip`, `rust-vcs-canonical`, `rust-change-algebra-canonical`).
5. **JS**: Weak entities as `class` + caches; `*Entity` renames; VCS + change-algebra classes; `Entity` + `defineField` / `defineOperations`; purge `KIT_*`; private Json wire (`js-purge-json`, `js-drop-fieldspecs`, …).
6. **React**: `*Scope` → `*Context` on all exports; field hooks + owned collections; bridges region; no `useSyncExternalStore` (grep-clean + vitest negatives per plan).
7. **Sketchpad**: Migrate off `useKitRuntimeSafe`, `useKitScope`, `KitScope` / `KitScopeProvider` to the renamed React API once (6) lands.
8. **Verify**: Run `kit_store_golden_ops_via__op_json_match_fingerprint` where golden files are checked out; add `depcruise:layers` if/when scripted; broader `cargo test -p semio` when fixtures available.

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

- `KitStoreBundleFile` / snapshot DTO stack and broad `serde_json::Value` confinement to DevBackbone + GraphQL decode only (unchanged struct names; still many `serde_json` call sites).
- Full macro-driven weak-entity registration churn beyond re-pointing `entity_full_family!` at renamed `geom::entity` kinds.

### Commands (`--target-dir c:\\git\\semio\\target-phaseA`)

| Command | Exit |
|--------|------|
| `cargo check -p semio --target-dir c:\git\semio\target-phaseA` | **0** |
| `cargo check -p semio --target wasm32-unknown-unknown --target-dir c:\git\semio\target-phaseA` | **0** |
| `cargo test -p semio --target-dir c:\git\semio\target-phaseA schema_matches_target_graphql_file` | **0** |
| `cargo test -p semio --target-dir c:\git\semio\target-phaseA no_deep_clone_on_traversal` | **0** |
| `cargo test -p semio --target-dir c:\git\semio\target-phaseA kit_store_bundle_serialize_hydrate_round_trip_via_graphql` | **0** |

### Files touched

- `semio/rs/lib.rs` only (no `semio/graphql/*.graphql` edits — schema guard still green).

### Conflicts avoided

- Did **not** edit `semio/js/index.ts` or `semio/react/index.tsx` (per parent directive).
- Ticket `ticket_reopen` returned “already open”; no duplicate ticket.

---

## Phase B — JS (`semio/js/index.ts`, `semio/js/kit-store.worker.ts`) — 2026-05-12

### Done (sub-pass)

- **Weak geometry + wire**: `Position`, `Coordinate`, `Plane`, `Point`, `Vector`, `Offset`, `Place`, `Location`, `Camera` as **classes** with parent/role; `PositionInput` / `OffsetInput` + `formatPositionInput` / `formatOffsetInput` kept as **plain mutation structs** (file-local helpers under `🪶WeakGeometry`); removed duplicate tail `🪶WeakEntities` interfaces.
- **Artifacts**: `Attribute` and `Benchmark` as **classes** (`Attribute.owner`, `Benchmark.quality`); `ConnectionSide` as **class** + `export type Side = ConnectionSide`; `parseAttributeConnectionUnder(ownerEntity, json)`.
- **Caches**: `Kit` factories + `wip`/`authoritative`/`session`; `Design` piece/connection/layer/group; `Type` port/connector/representation; `Graph` checkpoint/alternative + `TheKit`; `Session` alternative; `Checkpoint` edit/change; id-list-stable **`readDesigns` / `readTypes` / `readAuthors` / `readQualities` / `readTags` / `readConcepts`** via `__readIdListStable`.
- **VCS + change algebra**: `__kitGraphqlEnvelope` for navigators; removed public **`Kit#runGraphql`** and legacy subscription **`operationSucceeded`** fan-out; **`backboneSyncNow`**, **`backboneStatus`** (typed snapshot); extended **`🧮ChangeAlgebra`** (`Diff`, `Modification`, `Modifications`, `Input`, `ChangeLedgerEvent`, diff/mod/input/operation variant shells incl. **`RenamedKit`**).
- **`EntityRef`**: `export type EntityRef = Entity`.
- **Worker**: `kit-store.worker.ts` split into **`//#region`** blocks (header, handle, wire, onmessage).

### Commands

| Command | Exit |
|--------|------|
| `bunx tsc --noEmit` in `semio/js` | **0** |

### Files touched

- `semio/js/index.ts`
- `semio/js/kit-store.worker.ts`
- `.repo/🎫/26/05/12/SINGLE-SOURCE-ENTITY-LAYERS/verify-log.md` (this append)

### Note

- **`KIT_*_SPECS` / `defineFields` / `defineOperations`** kept **exported** for parallel **`semio/react`** re-exports (not edited here). Full purge waits React pass.
- **`// @ts-nocheck`** retained: embedded vitest block still references legacy `Graph.*` kit-store types.
