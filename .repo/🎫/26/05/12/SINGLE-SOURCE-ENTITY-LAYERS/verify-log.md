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

1. **Rust weak geom**: Finish true **one-type-per-weak-geom** collapse for `Vector`, `Point`, `Coordinate`, `Offset`, `Plane`, `Location`, `Attribute` (still split Copy wire vs `*Node`); align `entity_family!` / SDL naming with merged shapes.
2. **Rust bundle / serde_json**: Remove remaining `KitStoreBundle` / snapshot DTO paths per plan; confine `serde_json` to GraphQL decode + `DevBackbone` I/O only.
3. **Rust `Event::canonical_touched_paths`**: Extend for every `Event` variant and future ops (today only a handful of variants exist; subscription gating still coarse vs plan).
4. **Rust `rust-sub-fieldgate` / macros / SDL**: Per plan YAML (`rust-sub-fieldgate`, `rust-macros`, `rust-sdl-roundtrip`, `rust-vcs-canonical`, `rust-change-algebra-canonical`).
5. **JS**: Weak entities as `class` + caches; `*Entity` renames; VCS + change-algebra classes; `Entity` + `defineField` / `defineOperations`; purge `KIT_*`; private Json wire (`js-purge-json`, `js-drop-fieldspecs`, …).
6. **React**: `*Scope` → `*Context` on all exports; field hooks + owned collections; bridges region; no `useSyncExternalStore` (grep-clean + vitest negatives per plan).
7. **Sketchpad**: Migrate off `useKitRuntimeSafe`, `useKitScope`, `KitScope` / `KitScopeProvider` to the renamed React API once (6) lands.
8. **Verify**: Run `kit_store_golden_ops_via__op_json_match_fingerprint` where golden files are checked out; add `depcruise:layers` if/when scripted; broader `cargo test -p semio` when fixtures available.
