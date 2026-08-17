# Terra UI Chrome Umbrella Dissolution Acceptance

## Scope

- Deleted `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️Chrome/🟦️component.tsx`.
- Distributed its geometry contract to `🪟️WindowSilhouette`, accessible hint to `💡️ChromeControlHint`, and scroll/measurement contract to `🚧️WindowContentDeadLine`.
- Made the single `LoadingRow` consumer private to Skeletons and removed both obsolete public Storybook showcases.
- Retargeted direct element imports without altering accepted Toggle or PanelTabBar content beyond their imports.

## Final Hashes

| Path | SHA-256 |
| --- | --- |
| `🪟️WindowSilhouette/🟦️component.tsx` | `638808b85c409025183611afc2433924f44b8d660ed68f92690ba28e2c5bae36` |
| `💡️ChromeControlHint/🟦️component.tsx` | `351fb5e3d294d33b4aefa91c7bb2dc6da79cb4ab4e7d39b2a54b9a38b09fcd63` |
| `🚧️WindowContentDeadLine/🟦️component.tsx` | `952fac05fe91a4aa8239d1021b8e53e5da78b72e5057a0cd6187718203762c20` |
| Shared React barrel | `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401` |
| Storybook central spec | `6ffb3b9befd21d25ebaacda3db95a6174d49a9fa16144de5582c4b9992e50eed` |

## Source Acceptance

- `🎛️Chrome/🟦️component.tsx` is absent.
- Active TypeScript/TSX stale scan has zero old Chrome-path references.
- Active TypeScript/TSX stale scan has zero public `LoadingRow`, `LoadingRowProps`, or standalone `LoadingRow` exports.
- Specific direct dependency directions are present:
  - Scrollable and Window import `🚧️WindowContentDeadLine`.
  - Toggle, PanelTabBar, and DragHandle import `💡️ChromeControlHint`.
  - The shared barrel imports and re-exports `🪟️WindowSilhouette`, `💡️ChromeControlHint`, and `🚧️WindowContentDeadLine` from their specific source paths.
- Scoped tracked and untracked `git diff --check` checks are clean.

## UI React Gates

Each Nx gate below ran once after the registrar handoff. A preliminary shell stopped before invoking Nx because zsh reserves `status`; it did not run any gate.

| Gate | Result | Evidence |
| --- | --- | --- |
| `@semio-tech/ui-react:lint` | Pass | Nx completed successfully. |
| `@semio-tech/ui-react:typecheck` | Fail | Existing workspace type failures include unresolved framework plugin/machine symbols and `📦️index.tsx` assertion/schema failures outside the split ownership paths. |
| `@semio-tech/ui-react:test-quick` | Fail | 510 of 520 tests passed; 10 failures and 2 unhandled errors occur in gumball, icon CSS, CanvasPickMenu, Shell, tree, and VirtualFileSystem coverage outside this split. |
| `@semio-tech/ui-react:build` | Fail | Storybook cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/🌳OntologyTree.stories.tsx`. |

The source split, stale scans, direct-consumer checks, registrar hashes, and scoped diff checks are accepted. The aggregate typecheck, quick-test, and Storybook-build gates remain blocked by the listed external failures.
