# Harness and Styling Lane

## Implemented

- Added silhouette content-plane flow rules using the renderer-provided top/bottom clearance variables. Matching padding and negative block margins keep ordinary document clearance and intrinsic sizing stable; edgeless and dead-line scroll content consume the full clipped plane.
- Added no-backdrop capability, reduced-transparency, and forced-colors fallbacks. Only independently painted glass regions become opaque/system-colored; silhouette gaps remain transparent and border paths use system ink.
- Added a deterministic bilingual Mode fixture with coordinate stripes, text, multiple tab chips, controls, and a high-contrast floor. Added a WGPU host fixture using the same floor.
- Corrected both Storybook OS hosts to destructure the declared `plugin` prop and replaced stale pre-taxonomy imports in the adapter and OS Playwright suite.
- Extended the existing UI and OS Playwright specs with the new stories plus reduced-transparency, forced-colors, and WGPU adapter coverage.
- Changed the root Storybook runner from one hard-coded puzzle spec to the complete `.storybook/playwright.config.ts` suite.
- Added `workspace:test-storybook`, routed `package.json#test:storybook` through Nx, registered the gate in the launch seed/generated launch file, and added it to the existing Playwright workflow.

## Verification

- `bun nx run @semio-tech/ui-styling:test`: passed, 27 tests.
- `bunx playwright test --list --config .storybook/playwright.config.ts`: passed, 169 tests in 10 files; proves the configured suite is discoverable after repairing the stale registry import.
- `bun nx show project workspace`: passed; `test-storybook` resolves to `bun ./📜️script.ts test storybook` with cache disabled and the configured port.
- `bun nx run @semio-tech/plugin-registry:generate`: passed; regenerated `.vscode/launch.json` from the seed.
- `bun nx run workspace:build-storybook`: blocked before these stories by the unrelated unresolved `@semio-tech/coda-desktop/renderer` import in `.storybook/stories/ui/✅ValidationTree.stories.tsx`.
- UI-scoped Storybook build: blocked by the same unrelated import in `.storybook/stories/ui/🌳OntologyTree.stories.tsx`.
- `bun nx run @semio-tech/plugin-registry:check`: launch output was not reported stale, but the command failed on pre-existing concurrent plugin taxonomy violations.
- Narrow standalone `tsc`: not a valid isolated gate for this workspace and failed on current cross-workspace errors, including concurrent duplicate React silhouette exports; the new Mode story's required args and labels were nevertheless updated to the current branded API.

The ticket remains open for coordinator integration and end-to-end runtime screenshots.
