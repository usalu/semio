# Result lay14: OS renderer dependency-cruiser re-measure and fixes

Executor `lay14`. Scope: only the OS renderer (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` and its public-entry mirror `🧰️framework/📦️packages/🟦️typescript/🛍️products/💻️os/🔨️modules/📺️renderer`). No `package.json`/`bun.lock` edit, nothing under the frozen storage/revision paths, no cargo, no modifying git command, no sub-agent. `C` = this ticket folder, tools in `C/lay14-tools/`. The coordinator's rule on the CLI closure (relative imports in root `📜️script.ts`, `🏗️build`, `⚡️caching`, `🔧️tools`, bootstrap) was respected: no renderer file is in that closure and no repo-lib barrel was touched.

## 1. Measurement

Command (scoped, about 3 min per run): `node --max-old-space-size=8192 node_modules/dependency-cruiser/bin/dependency-cruise.mjs <os renderer> <os renderer public-entry mirror> --include-only '^🧰️framework/' --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json`, wrapped in `perl -e 'alarm 590; exec @ARGV' --`. Violations are counted where the source is inside the two renderer roots (`lay14-tools/agg.ts`). Same method before and after. `no-cross-package-relative` is reported once per package alias (`framework` and `framework-rs` share a source owner), so unique edges are about half.

| rule | before (b0, after lay9/lay13) | after (b3) |
|---|---:|---:|
| no-cross-package-relative | 381 | 99 |
| renderer-hosts-only-ui | 164 | 49 |
| framework-no-products | 11 | 11 |
| no-circular | 4 | 0 |
| total | 560 | 159 |

## 2. Changes

### 2.1 Deep imports to existing public entries (rewrites)
`lay14-tools/rfix.ts` (lay9's type-checker rewriter) with three corrections, all found by the dry runs:
- The "no entry inside the importer's own package" guard compared against the owner of the whole OS tree, so the react target (its own package `framework-renderer-react`) could never use `@semio-tech/framework-os` entries. It now compares the importer's deepest owner (`ownerOf(from)`).
- Host files may use the declared UI-contract entries (section 2.2).
- Dynamic `const { a, b } = await import(spec)` is resolved by its destructured names (it was a namespace use, hence unfixable), and a JSON document is matched through a glue wrapper that re-exports exactly that file (`export { default } from "<file>.json" with { type: "json" }`); the import attribute is removed when the specifier moves to the wrapper.
- Applied: 116 + 128 + 86 import specifiers in 84 files (`lay14-tools/rfix-changed.txt`): react target scripts (`scene-and-world-hosts`, `hub-connection-and-spaces`, `board-ink-and-feed-hosts`, `interpreter-and-shell`) and `NodeGraph/**` to `@semio-tech/framework-os/modules/renderer/engine/elements/**`, `UiDocumentStore` tests to `@semio-tech/framework/{packs,value/resident,actor/shard-client}`, host tests to the fixture JSON wrappers of `@semio-tech/framework`, `@semio-tech/framework/{kernel/return/content/input,actor/wire-turn,actor/shard-client/*,...}` and others. Every new specifier resolves (`Bun.resolveSync`, 0 unresolved).

### 2.2 Policy: renderer-hosts-only-ui allows the declared UI contract only (coordinator ruling)
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`: new `RENDERER_HOST_UI_CONTRACT` (`@semio-tech/framework-os`, subpath prefix `./modules/renderer/engine/elements/`). `RENDERER_HOST_UI_CONTRACT_TARGETS` is derived from the live workspace manifest (`exportTargetPatterns` over the declared export targets of exactly those subpaths) and appended to `RENDERER_HOST_ALLOWED_RESOLVED_PATHS`. No directory is allowed wholesale: the OS root barrel, store, wgpu glue, media, oracles, other OS entries stay forbidden. Test: `🧪️tests/🚫️resolved-boundary-classification` got a synthetic `@semio-tech/framework-os` package and 3 new portable cases (declared entry allowed; undeclared OS internal, declared entry outside the contract prefix and the wgpu entry forbidden), checked by our regex evaluation and by dependency-cruiser's own `validateDependency`.

### 2.3 Cycles: 4 to 0
`PluginRuntime`: `typed-result` and `turn-settle` and `turn-dispatch` formed a cycle through `yieldPluginUiContinuation` (a one-line wrapper of `hostContinuations.yieldContinuation()` that lived in `turn-settle`). New leaf `🧱️elements/🔌️PluginRuntime/⏩️continuation-yield/🟦️.ts` owns it (docstring moved with it); importers (`typed-result`, `operation-completion`, `handle-ui-intake`, `handle-turn-drive`, the `PluginRuntime` root, `turn-settle`) re-pointed with `lay14-tools/movesym.ts` (lay9's AST move tool). The unused `hostContinuations` import in `turn-settle` was removed.

### 2.4 Ticket tools (inputs, kept)
`C/lay14-tools/`: `agg.ts`, `list.ts`, `rules.ts`, `countrule.ts` (cruise report readers), `rfix.ts`, `pkgs.ts`, `movesym.ts`, `plan-yield.json`, `spec.ts`, `runvt.py`, `rfix-changed.txt`, `exports-needed.json`. Generated output (`🗑️generated/lay14-run`, `lay14-art`) was deleted.

## 3. What is left (159) and why

Spec: `C/lay14-tools/exports-needed.json` (lay6/lay9 format; `plannedIn` marks rows that lay9/lay13 already propose, so the manifest batch can deduplicate: 15 rows are already in `lay13-tools/exports-needed.json` with their manifest key still missing, 11 are new). 18 `@semio-tech/framework` rows (50 host edges: actor `lifetime`, `lifetime/patch`, `page`, root `Lane`/`TurnScheduler`, `wire-turn` diagnostics and `turn-result`, value `resident`, `diagnostic/controlled`, `process/build`, the three renderer oracles, `infinite/world/r3f` with 17 edges, plugin `window-kits/media`), `framework-actor` 4, `framework-canvas-ts` 1, `node-graph-rs` 1, `renderer-wgpu` 1, `ui-react` 1.
- **wgpu target (27 edges, `wgpuBlocked` in the spec)**: a bare specifier in its browser closure needs the schema-owned `profile.workspaceImports` binding first; not touched.
- **`framework-no-products` 11**: glue files of `@semio-tech/framework` under `📦️packages/🟦️typescript/🛍️products/💻️os/**` re-export renderer sources (`media/channel`, `media/lifecycle`, `wgpu/accessibility-mirror`, the shading oracle, 6 fixture wrappers). Unchanged by this work (lay13 lists them as misplaced glue). The `media` module straddles the host (`MediaTransportHost`, `PluginRuntime` channel adapter) and the wgpu target and depends on a host element (`SegmentedDownload`) and on plugin window-kits; it needs a neutral home or a UI port, not another export.
- **`infinite/world/r3f` (17 host edges)**: `World3dHost` reaches into a react-three package under `framework/modules/infinite`; needs the glue entry (spec) or a ui port.
- **Shared fixtures** consumed by Rust `include_str!` and by host tests were not moved (no cargo to prove it).

## 4. Verification

- `bun test ./…/🚫️resolved-boundary-classification/🟦️.ts` (with `SEMIO_TEST_ARTIFACT_DIR`): 5 pass, 272 expects (includes `assertRuleSetValid` and the new cases). `bun test ./…/🚀️bootstrap/🧪️tests/🟦️.ts`: 19 pass.
- `bun test ./…/🧪️tests/🔁️acyclic-import-graph/🟦️.ts` (lay9's cycle law, own SCC scan vs dependency-cruiser on the real renderer tree): 2 pass.
- `NODE_OPTIONS=--max-old-space-size=8192 node node_modules/typescript/bin/tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json`: 38 errors before the later rewrites, 40 after; the delta is a peer file (`process/native-build/storage/fixed`); the only error inside a file I changed (`🧪️tests/🔁️refresh-coalescing` 165,13) is also in the earlier run. The rest are the known missing dependencies (`@dnd-kit`, `react-resizable-panels`, jsdom/pngjs types, `katex`, flow-core).
- Renderer vitest through the react config with lay9's alias stubs (`lay14-tools/runvt.py`, stubs copied next to the config for the run and deleted): filters `UiDocumentStore pinch-gesture tree-windows canvas-presence input-contract`: 8 files, 70 tests pass. Filters `NodeGraph PluginRuntime Interpreter ShellHelpers MediaTransport ShellSync SpaceBrowser typed-operation window-action extension-invocation`: 26 of 30 files, 490 of 496 tests pass. Failures: 3 Playwright (browser binary missing, environment) and `Interpreter › retained surface host across refreshes`: `Unknown component: node-graph` because the vitest config aliases the whole `framework-os-node-graph-rs` package tree to `wasmEngineStub` (config line 278), so `NodeGraphHost` is `undefined` in vitest; the import resolves to a function under bun. Not caused by these edits (confirmed by `[DEBUG]` probes, removed), but not proven against a baseline run (no stash/worktree allowed).
- Not run: wgpu vitest config, the whole renderer suite, `bun repo` targets, cargo.

## 5. Open
1. Manifest batch: apply `lay14-tools/exports-needed.json` (deduplicate with lay13 through `plannedIn`), then re-run `bun C/lay14-tools/rfix.ts <fresh cruise json> apply <out>` (needs a cruise of the renderer roots as above).
2. Decide the `media` module home and the `infinite/world/r3f` contract (ui port) to clear the last 28 host edges and the 11 glue breaches.
3. wgpu profile bindings (27 edges).
4. Vitest alias for `framework-os-node-graph-rs` makes the `container-node-ids` retention law fail; the test needs the real host or the alias an escape for that single package.
