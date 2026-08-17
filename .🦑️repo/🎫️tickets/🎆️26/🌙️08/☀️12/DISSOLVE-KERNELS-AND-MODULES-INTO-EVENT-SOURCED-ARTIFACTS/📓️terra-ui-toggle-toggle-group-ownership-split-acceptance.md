# Terra UI Toggle Toggle Group Ownership Split Acceptance

## Scope

- Lease packet: `📓️sol-ui-toggle-toggle-group-ownership-split-terra-packet.md`
- Changed sources:
  - `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx`
  - `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
  - `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx`
- The coordinator updated the shared React registrar after the source checkpoint; this lease did not modify it.

## Delivered Ownership

- `Toggle` now owns its contracts, `addIconSize`, and executable wrapper behavior.
- `Toggle` depends on public `ToggleGroup`; `ToggleGroup` has no dependency on `Toggle` or Toggle contracts.
- `ToggleGroup` owns its context, root/item implementation, and private `toggleVariants` styling helper.
- `Tree` imports `Toggle` directly from the specific Toggle component.
- The shared registrar preserves public `Toggle`, `ToggleProps`, `ToggleItem`, `ToggleGroup`, and `ToggleGroupItem` exports without exporting `toggleVariants`.

## Source Integrity

The packet's three precondition source hashes matched before the move. Final SHA-256 values are:

| File | SHA-256 |
| --- | --- |
| `🎚️Toggle/🟦️component.tsx` | `15a3aa7bb4f7bdddd74fe148b0df40327c8f5441df9d83a5509e02e5e6da50cb` |
| `🎛️ToggleGroup/🟦️component.tsx` | `355ee2db3f2df4916d2ab4288c44f83024eae6fa672f59f369453cabc8fccb08` |
| `🪵️Tree/🟦️component.tsx` | `7a405fba59feb98f9122329bd32531d9bc1a861920e374b037ee58d11f8ea731` |
| shared React registrar | `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81` |

## Static Verification

- No direct `import { Toggle }` from `🎛️ToggleGroup/🟦️component.tsx` remains.
- `Toggle` imports `ToggleGroup` exactly once; `ToggleGroup` has no Toggle contract, executable Toggle, or specific Toggle-component reference.
- `toggleVariants` appears only as an unexported `const` in `ToggleGroup`; the Toggle source and React registrar have no `toggleVariants` symbol.
- Tree has exactly the direct specific Toggle import.
- `git diff --check` for the three leased source files completed without whitespace errors.

## Required UI React Gates

Each gate ran once through Nx with Bun.

| Gate | Result | Evidence |
| --- | --- | --- |
| `bun nx run @semio-tech/ui-react:lint` | Pass | Nx reported successful completion. |
| `bun nx run @semio-tech/ui-react:typecheck` | Fail | Current workspace-wide diagnostics include missing framework/plugin contract types and unrelated UI diagnostics; no diagnostic was emitted for either Toggle source file. |
| `bun nx run @semio-tech/ui-react:test-quick` | Fail | 510 tests passed; 10 failed with 2 jsdom unhandled errors. No failed test names the relocated Toggle wrapper. |
| `bun nx run @semio-tech/ui-react:build` | Fail | Storybook/Vite cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/✅ValidationTree.stories.tsx`. |

No unrelated repair was applied after those observed gate results.
