# UI Flow Direction Context Acceptance

## Lease Outcome

The qualified logical-direction context now belongs to the UI `flow-direction-context` module. The seven element consumers import it directly, the registrar mechanically redirected the React barrel, and neither legacy source directory remains. The context retains its LTR/down default and nested partial-override behavior without exposing React-derived provider props.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🧭️flow-direction-context/🟦️component.tsx | a84af143796b57028794b503423be3fa2254d4d4df47c58be97a1d355ccb32e2 |
| Removed | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️Flow/🟦️component.tsx | pre-removal: 163e978a3635d7b0fd2654187b7782fa9c5f5637e3ba128d7ecc426cdd4953d3 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️component.tsx | 0ca2c5f79fe9ed8a8efa6d73a699c20326224f8fcab79d2466fcede332cba8be |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️PanelTabBar/🟦️component.tsx | 8320685ca88cc33d891732241d7a890e691ab546278885cd6778f9d50c429583 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️component.tsx | 3b8ca2e68825778adf714fedaef8661e38c194ce8ceb4ce03a425a51f12580d8 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx | 853972c04839c1cf285031a47fc4e183006e51042441f15d989960f89c3459a4 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️component.tsx | 92f29a27b6e7c62cc2083aba5600751945964e174cd63549bb8ca7f0161beade |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️component.tsx | 6d021f2d6b045d192c44c8f94c4b97b18d61c3749c7b1a503c0a81668d2b699e |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx | ad15221e658ab8ffd5050fdd9a1e6791549d1df0d654c03bff7034e9d805c6b0 |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d |

## Static Acceptance

- Public `FlowInline`, `FlowBlock`, and `Flow` remain repository-owned contracts. `FlowProvider` uses a private named props interface with `unknown` children; its sole `React.ReactNode` use is a private adapter-boundary cast, and the public module API has no React- or external-derived type.
- The default remains `{ inline: "ltr", block: "down" }`. Nested providers retain the exact `inline ?? parent.inline` and `block ?? parent.block` merge semantics.
- Exactly eight active module imports exist: the registrar barrel plus the seven direct production consumers Popover, PanelTabBar, ContextMenu, Select, Panel, Dialog, and Tree.
- The active UI scan has zero legacy Flow source references. Both `🧭️Flow` and the previously verified-empty `🐹️ElementProps` directories were individually verified empty, then removed with `rmdir`; no other directory was removed.
- The registered quick-test target executed the FlowProvider default/nested-partial-override test, and that named test was not among its reported failures.
- Scoped ordinary and cached `git diff --check` commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with `--skip-nx-cache`.

| Command | Outcome |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Passed; only the existing `NO_COLOR`/`FORCE_COLOR` warning was emitted. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | Failed on broad existing framework and UI diagnostics, including plugin/event contracts, generated manifests, styling readonly writes, translations, XYFlow, Table, Icons, Tree, and product typing. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. The failures are the established Scene camera mock, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event cases. |
| `bun nx run @semio-tech/ui-react:build --skip-nx-cache` | Failed independently because Storybook cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/🌳OntologyTree.stories.tsx`. |

## Blockers

No Flow direction-context source or registrar blocker remains. The non-passing gates are blocked by the existing workspace type diagnostics, established quick-test/jsdom failures, and the unrelated unresolved Storybook Coda renderer dependency.
