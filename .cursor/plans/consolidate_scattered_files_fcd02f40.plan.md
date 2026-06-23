---
name: Consolidate Scattered Files
overview: Fold remaining scattered source/module files into their package's single god-file (index.ts(x) / lib.rs) using regions, strictly limited to module-internal splits so no bundle dependency or runtime behaviour changes.
todos:
  - id: reopen
    content: Reopen ticket 2026/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES via ticket_reopen
    status: completed
  - id: ts-geometry
    content: Fold geometry/brep/js contracts.ts, kernel.ts, mesh.ts into index.ts regions; delete files; fix imports
    status: completed
  - id: ts-repolib
    content: Fold repo/lib/js/src commit/micro-commit/uloc-metrics into index.ts; delete shim files; repoint index.test.ts
    status: completed
  - id: ts-vscode
    content: Fold repo/client/vscode codegen/*.ts into extension.ts regions; update internal import
    status: completed
  - id: rust-graph
    content: Inline-fold mathematical/graph siblings (geometry, scene_json, types, board_host, fixture_layout) into respective lib.rs
    status: completed
  - id: rust-vello
    content: Inline-fold infinite/cavas/vello icon_codec.rs (preserve include!) and theme.rs into lib.rs
    status: completed
  - id: py-test
    content: Embed compose/client/lib/py store_test.py into main.py; delete store_test.py
    status: completed
  - id: verify
    content: Run nx test/build, cargo check/test, pytest for affected packages; confirm no runtime/bundle change; close ticket
    status: completed
isProject: false
---

# Consolidate Scattered Files

Pure refactor. Fold scattered files into each package's single god-file using `#region` sections. HARD CONSTRAINT (per dev): no change to bundle dependencies and no change to runtime behaviour. The classifier below was validated by reading every candidate's imports, `package.json` exports, bundler/Cargo configs.

Work inside the existing open ticket `2026/05/30/CONSOLIDATE-MONOREPO-INTO-SINGLE-FILES` (goal `🎯aioptimizedrepo🎯singlefilerepo`) via `ticket_reopen` — do NOT open a new one.

## Decision rule

```mermaid
flowchart TD
  F[Scattered file] --> Q{Distinct bundler/runtime entry?}
  Q -->|"worker / vite html / electron / vscode entry / pkg.json subpath export / build.rs / .inc.rs / bin-only crate / go _test.go / .NET test project"| Keep[Keep split - merging changes bundle or runtime]
  Q -->|"only relative-imported within its own package"| Merge[Fold into index.ts(x) / lib.rs region]
```



## Workstream 1 - TypeScript source folds

- `geometry/brep/js`: fold [contracts.ts](geometry/brep/js/contracts.ts), [kernel.ts](geometry/brep/js/kernel.ts), [mesh.ts](geometry/brep/js/mesh.ts) into [index.ts](geometry/brep/js/index.ts) as `//#region`; delete the 3 files; rewrite the `./contracts`/`./kernel`/`./mesh` relative imports. `index.ts` already runs `import.meta.vitest`; vitest `include` stays `index.ts`. No package `exports` map, so no boundary change.
- `repo/lib/js/src`: fold [commit.ts](repo/lib/js/src/commit.ts), [micro-commit.ts](repo/lib/js/src/micro-commit.ts), [uloc-metrics.ts](repo/lib/js/src/uloc-metrics.ts) into [index.ts](repo/lib/js/src/index.ts) as regions; delete the unused re-export shims [bundle-script.ts](repo/lib/js/src/bundle-script.ts) and [dependency-boundary.ts](repo/lib/js/src/dependency-boundary.ts); repoint [index.test.ts](repo/lib/js/src/index.test.ts) imports to `./index`. Keep `index.test.ts` split (separate `bun test` entry).
- `repo/client/vscode`: fold [codegen/gql.ts](repo/client/vscode/codegen/gql.ts), [codegen/graphql.ts](repo/client/vscode/codegen/graphql.ts), [codegen/fragment-masking.ts](repo/client/vscode/codegen/fragment-masking.ts), [codegen/index.ts](repo/client/vscode/codegen/index.ts) into [extension.ts](repo/client/vscode/extension.ts) regions (hand-maintained, not generated); update internal `./codegen/gql` import. Keep `extension.ts` + `extension.test.ts` (Vite lib + test entries).

## Workstream 2 - Rust module folds (inline `pub mod`)

Convert `pub mod X;` + sibling file into inline `pub mod X { ... }` `// #region` inside each crate root, then delete the sibling `.rs`:

- [mathematical/graph/lib.rs](mathematical/graph/lib.rs) <- `geometry.rs`, `scene_json.rs`
- [mathematical/graph/port/directed/lib.rs](mathematical/graph/port/directed/lib.rs) <- `types.rs`, `scene_json.rs`
- [mathematical/graph/port/directed/normal/lib.rs](mathematical/graph/port/directed/normal/lib.rs) <- `board_host.rs` (~5.6k lines - large fold, do carefully)
- [mathematical/graph/normal/undirected/lib.rs](mathematical/graph/normal/undirected/lib.rs) <- `fixture_layout.rs`
- [infinite/cavas/vello/lib.rs](infinite/cavas/vello/lib.rs) <- `icon_codec.rs` (preserve its `include!(concat!(env!("OUT_DIR"), ...))`), `theme.rs`

## Workstream 3 - Python test embed

- `compose/client/lib/py`: embed [store_test.py](compose/client/lib/py/store_test.py) into [main.py](compose/client/lib/py/main.py) (already the pytest `python_files` host with embedded tests); update relative imports; delete `store_test.py`.

## Explicitly excluded (merging would change bundle/runtime)

- Web workers: `compose/client/lib/js/kit-store.worker.ts`, `puzzle/3d/react/precompute.worker.ts`
- Entry points: `compose/client/lib/sketchpad/js/boot.tsx` (Vite HTML + `./boot` export), `repo/client/vscode/extension.ts`/`extension.test.ts`, all Electron `preload.ts`/`renderer.tsx`, Next.js `app/**/route.tsx`
- `package.json` subpath exports: `ui/styling/js/resolve.ts`, generated `ui/styling/js/tokens.generated.ts`; and `ui/styling` Vite tooling files (`vite-elements-assets.ts`, `playground-*.ts`) stay split to avoid pulling node-only build code into the browser entry
- Rust `build.rs` and shared `*.inc.rs` fragments; bin-only crates already single-file (`compose/server/hub/bin.rs`, `compose/client/bin/store/bin.rs`)
- Go `*_test.go` (runner requires separate file) and .NET `*.Tests` projects (separate assemblies)

## Verification

Run per affected package via the existing runner (no config changes): `nx` test/build for `@semio-tech/geometry-brep-js`, `@semio-tech/repo-lib`, the `vscode` extension build; `cargo check`/`cargo test` for the touched crates; `uv run pytest` for the py lib. Confirm region structure compiles and tests pass before closing the ticket with `ticket_close` listing all touched files.