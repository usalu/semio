# P10k Owned Runtime and Platform Imports

## Scope

- Added `@semio-tech/ui-react/runtime`, an owned React runtime boundary with structural UI contracts, hook wrappers, an error-boundary factory, and root mounting.
- Migrated the Entwerfen mit Bestand demonstrator away from direct `react` and `react-dom` imports.
- Migrated the standalone terrain story from direct R3F `Canvas` to the existing `ThreeCanvas` UI boundary.
- Replaced the Unified Gumball story's direct Three.js type import with its owned `UnifiedGumballProps` contract.
- Rewired the checked-in JSPI fixture to its checked-in local Preview 2 shim and host shim instead of an undeclared npm package. The four resolved relative files exist and `node --check` passes.
- Corrected JavaScript parity ownership so package `engines` capabilities, such as the VS Code extension-host `vscode` module, are recognized as platform-provided rather than missing npm dependencies.

## Gates

- `bun nx run @semio-tech/ui-react:typecheck`: pass.
- Demonstrator `build`: intentionally interrupted before Vite validation because the project unexpectedly entered a procedural Cargo build while the Puzzle P4 agent owned the exclusive Cargo window. This gate remains pending for the next permitted Cargo window.
- `node --check .../out-jspi-explicit/jcoprobe.js`: pass.
- Relative shim existence checks: pass.
- `bun ./📜️script.ts verify dependencies`: pass at 180 identities, 58 below the frozen baseline, zero additions.
- `bun ./📜️script.ts verify dependencies parity js`: improved from 15 findings at packet start to 6 findings. The command remains red by design until the BRep and repo-server ownership packet closes the final six.

## Ownership Notes

- The runtime boundary exposes only workspace-owned structural contracts; React and React DOM remain implementation details of the UI package.
- No dependency row or parity allowlist was added.
- The engine-capability rule is manifest-derived for every package and does not special-case a package or source path.
