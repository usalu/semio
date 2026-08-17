# UI Element Identity and Dead Transaction Acceptance

## Lease Outcome

`ElementProps` is now a UI-owned element-identity module. The nine leased elements consume it directly, the React barrel has the registrar-owned type import/export, and the prior identity source has no forwarder. The unused transaction context and its inert lifecycle paths are gone; ActionGroup retains only its explicit callback ownership.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/🆔️element-identity/🟦️component.ts | 4488af2baa0a4507ea74966c42693f43605941307db4d9962d9e978adf05096f |
| Removed | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐹️ElementProps/🟦️component.tsx | pre-removal: 68687315c6a2438862014df75a6671fef4ce810544b378eb4c73198be8983091 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx | 1978fe34c166fceba6be70cd478d65b66a5e85e3301347cde77e077b7287e9b4 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🟦️component.tsx | 8354681c8e9e735636ae9d56c6b5bda865904df59c5c25d1c6409258f588035a |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📄️Textarea/🟦️component.tsx | 6cd9610c92532aeeee189aa237e6770399a200d7c35394d6d82c211e12c27a06 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🟦️component.tsx | d296191c37e24cfcee96ae8dd99fbd64bff19efecba5d4972e6a6d90a1f681e9 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️component.tsx | 9e2ffd0faf02d3646dbd9f696361ad60dbf42ea7aabdb37eb0d0e7e7e3a4d226 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/☑️Select/🟦️component.tsx | 74a319ed0427947f9191e4bdc8fe7400b414633ca642a9a44a4abf84b4f44198 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️component.tsx | 9aa60f8260c3b51830000dc14caa9e0d99268090d02ee694ebd5a94fa71e88a4 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️HistoryTable/🟦️component.tsx | 3ccc276424bc23b711200d14fb50200f893f7256c4c5a5564907497d08018b37 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx | 837c514ed223178c9327ad097185f70537c0dda0e6d33f727fbe83b3f84ab40e |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx | 5bd32b0c107de82c8a663b50bd860d7f87c2e24c7013ce94364ad93f924c3fdb |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | a9a764971875336ed637b8be0ec1dae23150dfce09985ddf7cd5d69cafc774f6 |

## Static Acceptance

- The new module exports exactly the repository-owned `ElementProps` contract with `readonly id: string`.
- Ten active direct imports resolve to the new module: the registrar barrel plus HistoryTable, Ring, Toggle, Textarea, Stepper, Slider, Tree, Select, and Input. The nine element imports are direct module imports rather than barrel imports.
- The active UI scan is zero for the legacy ElementProps path, `TransactionProvider`, `TransactionContext`, `useTransaction`, and `ElementBaseProps`.
- Ring, Textarea, Stepper, Slider, Select, and Input have zero `transaction?.start`, `transaction?.finalize`, or `transaction?.abort` calls.
- ActionDropdown preserves the explicit `startTransaction?.()` and `finalizeTransaction?.()` callback calls and has no transaction-context fallback.
- The old ElementProps source file was removed without a forwarder. Its legacy directory contains no entries.
- The registrar-owned barrel now imports and exports `ElementProps` only as a type from the new module. Its SHA-256 is the final handshake value above.
- Scoped ordinary and cached `git diff --check` commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with `--skip-nx-cache`.

| Command | Outcome |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Passed; Bun emitted only the existing `NO_COLOR`/`FORCE_COLOR` warning. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | Failed on broad existing framework and UI diagnostics, including plugin/event contracts, generated-manifest symbols, styling readonly writes, translations, XYFlow, Table, Icons, Tree, and product typing. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. The failures are the known Scene camera mock, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event cases. |
| `bun nx run @semio-tech/ui-react:build --skip-nx-cache` | Failed independently because Storybook cannot resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/🌳OntologyTree.stories.tsx`. |

## Blockers

No ElementIdentity source or registrar blocker remains. The non-passing gates are blocked by the existing workspace type diagnostics, the established quick-test/jsdom failures, and the unrelated unresolved Storybook Coda renderer dependency.
