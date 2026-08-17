# Terra UI Mobile Panel Layout Inline Acceptance

## Scope

The former standalone mobile panel implementation and its exclusive Storybook story were removed. Its behavior now lives in Layout's private `LayoutMobilePanel` region. `LayoutMobilePanelProps` is the `LayoutProps.mobilePanel` contract and is exported with `Layout` by the React index. Panel, PanelTabBar, and ElementId documentation now describe Layout's private mobile panel rather than linking to the removed component.

## Final Closure

| Path | SHA-256 / state |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📱️MobilePanel/🟦️component.tsx` | absent |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📱️MobilePanel/🧪️story.tsx` | absent |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️component.tsx` | `3f00d039bb23b303172be9367b6eb53373806977613990bb9369948b3004586a` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️component.tsx` | `504b1eb08472bb2437b6eb45987b23b6d2111cf355fccf57269c42db82b11455` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️PanelTabBar/🟦️component.tsx` | `b522d8a6cdc026d020562fc8c5a2a0e2ec8c1a204036457c8f1c18922d722cb3` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️component.tsx` | `308951b26486abda4a67e5adda3273ac8eff260e924b3f57a0728ed110cfc38d` |
| React index registrar change | `64eb6dcf68e5c20a02409cedf789a96010f040d4144793b7de069f982795a10f` |
| Storybook smoke-spec registrar change | `ac6541e23bf754205e81c3fec1f3ff7cf800b9a176a47d10a1b200e4dc42d4ab` |

The final stale scan found zero exact `MobilePanel` or `MobilePanelProps` symbols and zero direct `📱️MobilePanel/🟦️component.tsx` paths in `🧰️framework` and `.storybook`. It found the required private `LayoutMobilePanel`, public `LayoutMobilePanelProps`, mobile render branch, and React-index type export.

## Diff Integrity

Scoped ordinary and cached `git diff --check` commands completed with exit code 0 and no output. The ordinary scoped diff contains only the two removed mobile-panel artifacts, four owned source updates, and the registrar-owned React-index and Storybook smoke-spec updates. The scoped cached diff is empty.

## Required Nx Validation

| Command | Exit | Result |
| --- | ---: | --- |
| `bun nx run @semio-tech/ui-react:lint` | 0 | Passed. |
| `bun nx run @semio-tech/ui-react:typecheck` | 1 | Failed on pre-existing broad workspace type errors, including missing framework plugin types, statechart event-shape mismatches, styling readonly assignments, and existing React-index type failures. No repair was made outside this packet. |
| `bun nx run @semio-tech/ui-react:test-quick` | 1 | Failed: 1 test file, 10 failed tests, 513 passed tests, and 2 unhandled errors. The reported failures are in existing `VirtualFileSystem` rendering tests; the unhandled errors are `closest` and `Node.contains` failures in existing pointer-event paths. |
| `bun nx run @semio-tech/ui-react:build` | 1 | Failed because Vite/Rollup cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/🌳OntologyTree.stories.tsx`. |

Each target was run once. The failing broad gates were recorded and left unrepaired as required by the packet.
