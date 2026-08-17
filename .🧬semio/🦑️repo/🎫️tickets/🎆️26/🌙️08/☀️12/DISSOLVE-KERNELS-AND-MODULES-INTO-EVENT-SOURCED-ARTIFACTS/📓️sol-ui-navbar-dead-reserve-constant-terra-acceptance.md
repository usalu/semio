# UI Navbar Dead Reserve Constant Acceptance

## Lease Outcome

The zero-consumer `shellNavbarTrailingEndReserveCss` constant is removed from Navbar and from the registrar-owned React barrel. Navbar’s measured-reserve behavior, layout, items, APIs with production consumers, stories, products, generated output, locks, and manifests were not changed.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️component.tsx | 85f2e37dd539498d05e193673a0eb7388d67e68083c327b618429a18fdba9099 |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | b4b1622b05d3bdbf50e7ef5f1edfd4cda00e35963a1b07c28efba2ca37cfd9c5 |

The accepted pre-edit hashes were Navbar `2918372bf6dcee1d211db0a0082db4f7cd596db2c06ba6fb91d641d426ce024e` and protected barrel `537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d`.

## Static Acceptance

- Navbar removed only the dead trailing-reserve constant and its attached documentation; the surrounding fill helpers and all navbar behavior remain intact.
- The registrar removed the corresponding mechanical barrel import/export. The active UI scan has zero `shellNavbarTrailingEndReserveCss` references.
- Scoped ordinary and cached `git diff --check` commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with `--skip-nx-cache`.

| Command | Outcome |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Passed; only the existing `NO_COLOR`/`FORCE_COLOR` warning was emitted. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. The failures are the established Scene camera mock, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event cases. |

## Blockers

No Navbar dead-reserve source or registrar blocker remains. The non-passing quick-test target is blocked by the established unrelated failures and jsdom errors.
