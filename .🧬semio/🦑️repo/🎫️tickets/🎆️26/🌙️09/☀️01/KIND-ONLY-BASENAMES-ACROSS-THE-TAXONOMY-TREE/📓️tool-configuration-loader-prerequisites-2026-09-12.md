# Tool Configuration Loader Prerequisites

This is coordinator preparation for the queued tooling lane. No product configuration, editor setting, source owner, or catalog exception was changed. Authored positive controls are retained under `📋️tool-config-controls`; disposable native results are under `🗑️generated/coordinator/tool-config-controls`.

## Current Exact Population

The focused visible-source census contains **50 configuration files, 1,967 newline segments and zero installed TypeScript syntax diagnostics**: 43 Vitest, two Tailwind, one PostCSS, one package ESLint TypeScript, one root ESLint MJS, one dependency-cruiser CJS and one VS Code test MJS. The earlier 52-path review also included two Cargo build entries, now covered by a separate execution packet. This is not all executable metadata. The authored styling preset `styling/💨️tailwind/🎨️tailwind.config.ts` is an additional named implementation owner reached by both exact Tailwind shims. Those two shims are pure reexports; syntax parsing is not semantic ownership or runtime acceptance.

## Executed Native Controls

- **Vite 7.3.6**: installed `loadConfigFromFile` successfully loaded both `🧪️vitest/🟦️.ts` and `🧪️vitest/_.ts` inside the emoji ticket tree with its default loader, preserving each distinct test name and reporting one dependency. Both controls passed. The current root schema-oracle comment that an emoji filename is unresolvable by its esbuild config pass is not supported by this installed loader. Production configs were not moved or executed; the executor must preserve their roots, aliases, include lists and native Vitest behavior.
- **ESLint 10.8.0**: anonymous `🔎️lint/🟨️.mjs`, selected through `overrideConfigFile`, produced exactly one expected `no-debugger` error of severity 2 for `debugger;`. No production lint sweep or cache mutation was requested.
- **@vscode/test-cli 0.0.10**: actual `tryLoadConfigFile` loaded anonymous `🧩️extension-test/🟨️.mjs`, retained its one exact labelled test and resolved the relative development path correctly. This was configuration execution, not an extension-host launch or installation. Its installed CLI explicitly provides `--config`; the loader resolves Mocha requirements and extension paths relative to the selected configuration directory.

All controls ran with Bun on this macOS host and actual installed tool code. They establish configurable loader support here; they do not establish Windows/Linux application runtime or all editor integrations. Temporary command logs use `[DEBUG]`.

## Actual Editor Consumers

Installed **Vitest Explorer 1.50.8** declares `vitest.configSearchPatternInclude`, `vitest.configSearchPatternExclude`, `vitest.rootConfig` and `vitest.workspaceConfig`. The workspace instead writes `vitest.configSearchPattern` at `.vscode/settings.json:111`; that key is absent from this installed extension's declared configuration and inspected read path. Its bundled implementation reads `configSearchPatternInclude`, passes it directly to `vscode.workspace.findFiles`, and carries selected paths into configuration metadata. This supports selecting a semantic-owner glob through the actual setting. Retain per-project discovery and the deliberate root no-tests config; do not create a root test aggregator. VS Code UI discovery was not driven here.

Installed **Tailwind CSS IntelliSense 0.16.0** declares `tailwindCSS.experimental.configFile` as a string or config-path-to-source-glob map. This is an explicit editor selector. Current production exports and v4 CSS/PostCSS consumers still need complete tracing before removal.

Installed **VS Code ESLint 3.0.34** exposes `eslint.options` to the ESLint API. Installed ESLint validates and forwards `overrideConfigFile`; the native positive proves that API accepts the anonymous file. Editor working-directory resolution must be rebased in the actual workspace configuration. Editor integration alone does not justify blanket fixed-name admission.

## Production Consumer Closure Required

The actual library `runVitest` defaults to `vitest.config.ts`, and `vitestRunArguments` forwards it after `--config`. The normalization reference tokenizer and its `run-vitest-config-argument-tokens` fixture explicitly parse that default declaration; update the real owner and source-as-data consumer together. Root schema-oracle constant, direct package calls, source fixtures, package exports, Nx inputs and launch seed/derived values must refer to selected neutral configuration owners. Coverage intentionally uses Node and non-coverage uses the caller runtime; preserve that distinction.

Tailwind's two package reexports lead to the styling preset above; `styling/🌓️theme/🟦️.ts` is another direct consumer. Both UI package export maps expose Tailwind config and React also exposes PostCSS config. Preserve actual API ownership instead of adding a compatibility chain. React PostCSS owns a local interface and plugin map. Installed postcss-load-config 6.0.1 exposes search-place options, but inspect the actual production invocation and Vite options before choosing selection. Installed Tailwind is 4.3.3; do not infer a v3 `tailwindcss --config` CLI contract from catalog verification text.

Root dependency-cruiser and ESLint files contain actual policy computation, including runtime filesystem discovery and related rule semantics; they are not literal registration-only configuration. Their explicit CLI selection is visible in root lint/verify routes. Extract actual domains and close both policy consumers. Catalog claims of unconfigurability must follow exact native contracts rather than default basename convention.

Use `📓️tooling-hooks-followup-packet-2026-09-12.md` for execution scope. This report supplies prerequisites and does not claim those remaining moves or the broader goal are complete.

## Exact Reviewed Files

| Source | Lines |
| --- | ---: |
| .dependency-cruiser.cjs | 466 |
| eslint.config.mjs | 129 |
| vitest.config.ts | 42 |
| ♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts | 39 |
| ♻️mit-bestand/🧺️demonstrator/vitest.config.ts | 15 |
| ✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/vitest.config.ts | 17 |
| ✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts | 42 |
| ✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/vitest.config.ts | 13 |
| ✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/vitest.config.ts | 56 |
| ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| ✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| ✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| ✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| ✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/vitest.config.ts | 15 |
| ✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/vitest.config.ts | 16 |
| ✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/vitest.config.ts | 15 |
| ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/vitest.config.ts | 17 |
| 🌎️hub/📦️packages/🟦️typescript/vitest.config.ts | 25 |
| 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts | 33 |
| 🧰️framework/📦️packages/🟦️typescript/vitest.config.ts | 29 |
| 🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/vitest.config.ts | 25 |
| 🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/vitest.config.ts | 29 |
| 🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts | 26 |
| 🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts | 43 |
| 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/vitest.config.ts | 42 |
| 🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/vitest.config.ts | 31 |
| 🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/tailwind.config.ts | 6 |
| 🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/vitest.config.ts | 26 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/eslint.config.ts | 24 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/postcss.config.ts | 31 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tailwind.config.ts | 6 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts | 50 |
| 🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/vitest.config.ts | 33 |
| 🧰️framework/🔨️modules/🧬️schema/vitest.config.ts | 23 |
| 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/vitest.config.ts | 30 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts | 53 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/vitest.config.ts | 26 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts | 35 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts | 104 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/vitest.config.ts | 20 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/vitest.config.ts | 19 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts | 19 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/vitest.config.ts | 25 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/vitest.config.ts | 28 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts | 51 |
| 🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/vitest.config.ts | 27 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/.vscode-test.mjs | 18 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/vitest.config.ts | 21 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/vitest.config.ts | 22 |
