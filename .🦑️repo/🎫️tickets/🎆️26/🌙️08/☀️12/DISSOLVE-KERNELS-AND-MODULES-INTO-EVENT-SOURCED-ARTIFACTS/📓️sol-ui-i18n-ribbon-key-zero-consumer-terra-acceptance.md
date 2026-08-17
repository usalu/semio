# UI I18n Ribbon Key Zero-Consumer Type Acceptance

## Lease Outcome

The production-zero-consumer `UiRibbonParentKey` type and its attached documentation are removed. All remaining I18n contracts and runtime values are unchanged; the registrar removed the mechanical type export and updated the inline test to use the existing `UiTranslationKey` contract.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx | ac87da2a670949bf7755af2ef85c0bebc2dcbeb3b655784f1e3336a5ce4152d2 |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 4e916cf18ad6c1a44961405f6adddb20b0a7383e3283af306f5c756e016ca52d |

The accepted pre-edit hashes were I18n `15122bb408ca449538f80a6d973bae23db4f85e92942336ea241edd7ea099891` and protected barrel `48d2d0da1eacb16553b4c9924b45b34ddc7dc70e33af4b9ac115513768a66076`.

## Static Acceptance

- The exact active UI scan for `UiRibbonParentKey` is zero.
- The independent public `AssertUiRibbonParentKeysCovered` contract remains defined and registered, preserving compile-time ribbon-category coverage.
- The registrar removed the mechanical type import/export and changed the ribbon test-only assertion cast to `UiTranslationKey`.
- No translations, bundles, products, manifests, generated output, or locks were edited.
- Scoped ordinary and cached `git diff --check` commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with `--skip-nx-cache`.

| Command | Outcome |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Passed; only the existing `NO_COLOR`/`FORCE_COLOR` warning was emitted. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. The failures are the established Scene camera mock, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event cases. |

## Blockers

No I18n Ribbon-key source or registrar blocker remains. The non-passing quick-test target is blocked by the established unrelated failures and jsdom errors.
