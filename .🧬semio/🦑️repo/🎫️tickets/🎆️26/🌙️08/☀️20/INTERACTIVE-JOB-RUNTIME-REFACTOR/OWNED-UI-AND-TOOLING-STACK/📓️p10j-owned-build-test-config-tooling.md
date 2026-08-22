# P10J Owned Build And Test Configuration Tooling

//#region 🎯️Scope

This packet owns the remaining external build, test, and lint implementation imports in the
requested six configuration/source consumers without adding dependency rows, allowlists, scanner
exceptions, or exported third-party types:

- `.storybook/main.ts`
- `.storybook/scopes.ts`
- root `🧪️vitest.config.ts`
- UI styling `🟦️vite-elements-assets.ts`
- UI React `🟦️eslint.config.ts`
- infinite-world R3F `🧪️vitest.config.ts`

The existing `@semio-tech/ui-react/test` subpath and `🧪️render.ts` were preserved.

//#endregion 🎯️Scope

//#region 🧩️OwnedBoundaries

Three repository-owned boundaries now isolate temporary implementations behind the deepest
manifest that already declares each implementation:

1. UI React `🟦️build-tooling.ts` owns Tailwind Vite, React Vite, and Vitest config
   implementations. It exports only repository-owned structural configuration, plugin, middleware,
   and test-project contracts.
2. Root `.storybook/🟦️lint-tooling.ts` owns Storybook ESLint, globals, and
   typescript-eslint implementations. It exports only `OwnedLintConfig` records.
3. OS dev `🟦️config-tooling.ts` owns Vite config loading and returns the repository-owned
   `OwnedTestProjectConfig` contract.

The consumer behavior remains unchanged: Tailwind and React plugins retain their ordering, root
Vitest discovery still loads each child config and backfills its root, Storybook retains its
duplicate-plugin guards, and UI React retains the same flat ESLint composition.

Six unused MDX/rehype/remark implementation imports were also deleted from UI styling; they had no
runtime use and therefore required no adapter.

//#endregion 🧩️OwnedBoundaries

//#region 📊️Parity

The packet-start live JS parity census was:

- manifests: **83**
- external rows: **304**
- evidenced rows: **142**
- unowned rows: **162**
- undeclared imports: **34**
- requested-path undeclared findings visible to the scanner: **13**

The requested cohort contained **18 external implementation import sites**: thirteen visible
undeclared findings plus five configuration imports that the current scanner did not attribute to
these consumers. All eighteen now route through repository-owned boundaries or were proven unused
and deleted.

The final live census is:

- manifests: **83**
- external rows: **303**
- evidenced rows: **144**
- unowned rows: **159**
- undeclared imports: **11**
- requested-path undeclared findings: **0**

Other agents changed the shared repository census during this packet, so the global **34 -> 11**
undeclared delta and **304 -> 303** row delta are not attributed solely to this cohort. The exact
bounded result is **13 -> 0 scanner-visible requested-path findings** and **18 -> 0 requested external
implementation import sites**. No manifest changed in this packet, so `bun install` was not needed.

The eleven residual undeclared imports are outside this packet and remain explicit follow-up work:

- three `brepjs` / `brepjs-opencascade` imports in the spatial-kernel BREP implementation;
- three generated `@bytecodealliance/preview2-shim` imports in the jcoprobe fixture;
- two VS Code host imports in the VS Code extension package;
- three server-library imports: `pg`, `next`, and `pg-boss`.

Subsequent packet P10K owned the six live B-Rep/server findings while concurrent scanner/source
repairs resolved the other five; the canonical JS parity gate is now clean at zero undeclared
imports. See `📓️p10k-owned-brep-and-repo-server-implementations.md`.

//#endregion 📊️Parity

//#region ✅️Validation

- `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`: **PASS**.
- `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`: **PASS**.
- `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`: **PASS**, 533 tests.
- `bun nx run @semio-tech/ui-styling:test-quick --skip-nx-cache`: **PASS**, 30 tests.
- `bun nx run @semio-tech/infinite-world-r3f-pkg:test-quick --skip-nx-cache`: **PASS**, 100 tests.
- Bun import smoke for `.storybook/main.ts`: **PASS**.
- Bun import smoke for root `🧪️vitest.config.ts`: **PASS**.
- `bun ./📜️script.ts verify dependencies`: **PASS**, baseline 238, current 180, removed 58,
  additions 0.
- dependency lists: **63 Rust + 117 JavaScript = 180**.
- JS parity target filter: **PASS**, requested-path findings 0.
- full JS parity gate: expected repository residual **FAIL**, exactly 11 out-of-scope undeclared
  imports listed above.

An attempted `workspace:test-quick` was stopped with exit 130 as soon as it unexpectedly recursed
into `compose/graphql:build` and started Cargo; it is not counted as a validation gate. No further
workspace-wide test was run, preserving the serialized Cargo lock window.

//#endregion ✅️Validation
