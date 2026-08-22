# P10b Compose JS Source-Proven Dependency Cleanup

## Scope

This cohort is independent of the plugin facade manifests cleaned in P10a and of the concurrently edited Rust/runtime products. It changes only `compose/client/lib/js` packaging/test routing plus the Bun lockfile.

The package's executable and generated/runtime source roots were inspected as a unit:

- `index.ts` — public GraphQL/WASM transport and embedded Vitest suite.
- `kit-store.worker.ts` — bundled worker runtime entrypoint.
- `vite.config.ts` — test loader/configuration entrypoint.
- `📜️script.ts` — permanent Nx test router.
- `tsconfig.json` and `📋️project.json` — compiler and Nx command configuration.

The initial static census excluded `package.json`, `bun.lock`, and `node_modules`. Searching those roots for `rxjs`, `zod`, `cross-env`, `fast-check`, and `@vitest/coverage-v8` found no runtime, dynamic-import, test, config, code-generation, or script reference. The only `zod` match was stale prose in the local `AGENTS.md`; it is not an executable input and was intentionally not edited.

## Removed Manifest Rows

| Row | Section | Source evidence | Result |
| --- | --- | --- | --- |
| `rxjs` | dependencies | no import, require, dynamic import, or config/script use | removed |
| `zod` | dependencies | no import, require, dynamic import, or config/script use | removed |
| `cross-env` | devDependencies | no command/config use | removed |
| `fast-check` | devDependencies | no test/source use | removed |
| `@vitest/coverage-v8` | devDependencies | Vitest config declares coverage selection only; no provider/plugin import or command | removed |

Retained declarations were not inferred stale merely because they are tooling. `typescript`, `vitest`, `@types/node`, `@types/react`, and `@types/react-dom` remain wired through `tsconfig.json`, `vite.config.ts`, or the permanent test router. `@semio-tech/assets` also remains, because it is a workspace row rather than an external dependency and this bounded pass did not change its configured path aliases.

`bun install --lockfile-only` synchronized the removal. Its lock resolution removed the now-unreferenced `fast-check`, `pure-rand`, and `rxjs` package records. A simultaneous workspace edit added an unrelated `@semio-tech/framework-os` row elsewhere in `bun.lock`; that row was preserved and is not attributed to this cohort.

## Test Router Repair

The permanent Compose JS router named `⚙️vite.config.ts`, while the actual configuration entrypoint is `vite.config.ts`. The exact Nx test target therefore could not load its configuration before this correction. The router now names the real file; no command interface changed.

## Verification

- `bun install --lockfile-only` — exit 0; lockfile saved.
- `bun nx run @semio-tech/compose-js:build --skip-nx-cache` — exit 0; `bunx tsc --noEmit` passed.
- `bun nx run @semio-tech/compose-js:test-quick --skip-nx-cache` — exit 0; 1 file / 7 tests passed after the router repair.
- `rg -n '\\[DEBUG\\]' compose/client/lib/js/package.json compose/client/lib/js/📜️script.ts` — no matches.
- `git diff --check` — exit 0; no whitespace errors.

The first exact `test-quick` invocation failed before test discovery solely because the router referenced the non-existent emoji-prefixed config file. That failure is resolved by the one-line source-name correction above; the succeeding Nx invocation is the acceptance result.

