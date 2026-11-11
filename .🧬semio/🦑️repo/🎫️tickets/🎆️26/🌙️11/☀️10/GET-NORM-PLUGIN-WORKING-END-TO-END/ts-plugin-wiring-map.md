# 🟦 Norm TS / Plugin Wiring Map

- Registration: workspace entries in `✏️s/package.json` (norm-js, 15 artifact TS packages, dev harness), same as note/layout.
- Plugin registry `🧰️framework/.../📇️registry/🤖️generated/🔌️plugins.json` is `[]` → nothing discovered at runtime (affects all plugins).
- Bridge spawns Rust binary `semio-norm-mutation-bridge`.
- Targets: `@semio-tech/norm-js:test|test-wire-twins`, `@semio-tech/norm-<code>:build|check|test`, `@semio-tech/s-norm-composition-tests:verify-norm-document-contract|verify-norm-results-window-ownership`.
- All 46 relative imports resolve; framework deps exist.
- Suspicious:
  1. Empty plugin registry.
  2. `✏️s/🔌️plugins/🔒️policy-allowlist.json` lists nonexistent `🎛️apps/...` paths for iso16757, vdi3805, din4108, din16798, en1990.
  3. No `dist/`; artifact `exports`/`types` point to `dist/🟦️.js` → consumers need `build` first.
  4. Dev harness imports `framework/🔨️modules/🏃️process` vs norm-js/bridge `framework/📦️packages/🟦️typescript/🏃️process` (inconsistent).
  5. UI deps (ui-react, ui-styling, infinite-world-r3f, framework-3d-js) unused in norm TS; UI surface lives only in Rust hub composition `🌎️hub/🧩️compositions/📕️norm`.
