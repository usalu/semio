# P10o Owned Diagram Implementations and Renderer Export

## Outcome

- Removed the two requested unique JavaScript dependency identities: `@types/d3-force` and `@types/dagre`.
- Confined the remaining `d3-force` and `dagre` runtime imports to the Diagram leaf implementation.
- Replaced external graph/simulation type exports with workspace-owned structural contracts for directed positions, force nodes, force links, and the simulation handle used by interaction code.
- Removed the raw D3 force functions, D3 simulation types, and Dagre namespace from the public `@semio-tech/ui-react` barrel. No repository consumer used those external re-exports.
- Added focused runtime coverage for left-to-right directed layout and deterministic force settling through the owned contracts.
- Restored the missing `renderUiControl` Interpreter export used by `ShellHelpers`, backed by an owned declarative-control shape and a focused action-dispatch test.
- Confirmed the production demonstrator build now passes the former missing-export point and completes all 1,944 transformed modules.

## Dependency Identity Census

- Packet removals: exactly `js:@types/d3-force` and `js:@types/dagre`.
- Current canonical total: **169** identities = **63 Rust + 106 JavaScript**.
- Frozen baseline: 238 identities; repository total removals: 69; additions: 0.
- `🔒️dependencies.json` intentionally retains both names as historical freeze-baseline evidence; the live package manifests and `bun.lock` contain neither identity.
- `bun install --ignore-scripts` passed, checked 2,023 installs across 2,073 packages, and saved the shared lockfile.

The shared lock refresh also materialized manifest removals already made by concurrent packets. Those unrelated lock deletions are preserved but are not attributed to this two-identity cohort.

## Exact Validation

- UI Diagram focused test: `owned diagram implementations` — 2/2 passed, 536 skipped.
- Renderer focused test: `owned declarative controls` — 1/1 passed, 436 skipped.
- `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` — passed after both type packages were absent from the lockfile.
- `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` — 538/538 passed.
- `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` — passed.
- `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` — 437/437 passed.
- `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache` — passed.
- `SKIP_PLUGIN_BUILD=1 SKIP_ENGINE_BUILD=1 bun nx run @semio-tech/mit-bestand-demonstrator:build --skip-nx-cache` — passed; 1,944 modules transformed and Vite completed in 11.52 seconds without invoking Cargo.
- `bun ./📜️script.ts verify dependencies` — passed at 169 identities, 69 removed from baseline, zero additions.
- `bun ./📜️script.ts verify dependencies parity js` — passed with 83 manifests, 290 external rows, 141 evidenced, 149 unowned, and 0 undeclared imports.
- `bun ./📜️script.ts verify dependencies list js | jq 'length'` — 106.
- `bun ./📜️script.ts verify dependencies list rust | jq 'length'` — 63.
- `git diff --check` — passed.

## Scope and Generated Output

- No Cargo command ran.
- No dependency row, allowlist, suppression, compatibility import, or vendored implementation was added.
- The demonstrator build's required registry refresh produced no registry diff.
- The same build regenerated `.vscode/launch.json`, leaving a 33-line deletion that reflects the current shared manifest state. It is preserved as generated concurrent-state output and is not attributed to the Diagram or renderer source changes in this packet.
