# P10a Live JavaScript Dependency Census

## Scope

This checkpoint records the Phase 10 dependency surface on 2026-08-21 while the Phase 9 runtime packets continue. `compose/` is excluded by the master plan and by the dependency-freeze implementation.

## Reproducible Commands

```text
bun ./📜️script.ts verify dependencies list js
bun ./📜️script.ts verify dependencies list rust
```

## Current Boundary

- JavaScript: 134 unique third-party names.
- Rust: 76 unique third-party names.
- Combined freeze boundary: 210 names, down from the 238-name baseline.
- This is a progress census, not the Phase 10 exit gate.

## Highest-Fanout JavaScript Rows

| Dependency | Kind | Direct manifests |
|---|---|---:|
| `typescript` | tooling | 60 |
| `vitest` | tooling | 54 |
| `react` | runtime/tooling | 39 |
| `react-dom` | runtime/tooling | 39 |
| `@types/react` | tooling | 39 |
| `@types/react-dom` | tooling | 38 |
| `vite` | tooling | 37 |
| `@vitejs/plugin-react` | tooling | 36 |
| `@tailwindcss/vite` | tooling | 35 |
| `@react-three/fiber` | runtime | 33 |
| `three` | runtime | 33 |
| `@react-three/drei` | runtime | 32 |
| `@types/three` | tooling | 32 |
| `chevrotain` | runtime/tooling | 31 |
| `xstate` | runtime | 31 |
| `brepjs` | runtime | 30 |
| `brepjs-opencascade` | runtime | 30 |

## Manifest-Parity Finding

The high fanout is partly a manifest-quality problem, not evidence that every package imports every listed library. For example, `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/package.json` still describes itself as the CAD plugin and declares the generic React/R3F/Three/BREP/Chevrotain/XState stack. Its package entry point instead reexports generated Puzzle schema/codec facets, while the React-specific target is a separate nested package. This repeated shape must be audited source-first before any product replacement claims are made.

## Ordered Phase 10 Packets

1. Run a package-by-package manifest/source parity audit and delete only proven-unused direct rows, validating each affected Nx project.
2. Separate Rust/WASM schema facades from React renderer targets so product packages do not inherit UI dependencies they never import.
3. Migrate actual React/R3F/Three consumers product by product onto the worker-built owned UI/render contracts, with fixture parity before deleting each old target.
4. Replace the shared test/build/doc toolchain only after its callers have a stable owned command contract in the mandated `📜️script.ts` routers.
5. Keep the dependency-freeze ratchet green after every packet; zero names is the only Phase 10 exit condition.
