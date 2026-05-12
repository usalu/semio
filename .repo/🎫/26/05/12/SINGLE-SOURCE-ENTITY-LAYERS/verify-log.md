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
