# UI Orb Ring Inline Acceptance

## Lease Outcome

Orb is no longer a standalone UI component. Ring owns the private RingMarker contract and implementation, while the React registrar removed Orb and OrbProps from the protected barrel.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🟦️component.tsx | 9e12b40a944ceb231bea40af1fe939104901da0881accb29f5ecc220c239901b |
| Removed | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔮️Orb/🟦️component.tsx | ef40a085ec84797203ca379615c5d042888325511bfc289f7e31bf182c2f475f before removal |
| Removed | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔮️Orb/🧪️story.tsx | a6df96142c0e9057a05b4e3ed713730381566474320b6db16e8d2188ea54ca32 before removal |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | bdacd77b4c05441d97f044fb928d36300989652d60da3cca7ee473d1809a1f87 |

The empty legacy Orb directory was checked for dependency and cache files, found empty, and removed.

## Static Acceptance

- Active UI scan found zero Orb-path imports, public Orb or OrbProps contracts, barrel exports, or standalone story references.
- RingMarker remains private. The exact circle data attributes, geometry calculation, selected radius, drag transition, all class literals, pointer-events style, and disabled pointer-handler suppression are present in Ring.
- The scoped ordinary and cached git diff --check commands completed cleanly.

## Nx Validation

Each required target used the uncached Nx form with --skip-nx-cache.

| Command | Outcome |
| --- | --- |
| bun nx run @semio-tech/ui-react:lint --skip-nx-cache | Passed. |
| bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache | Failed on broad existing workspace and UI type errors. No Ring or Orb diagnostic was emitted. |
| bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache | Ran 520 tests: 510 passed, 10 existing failures, and 2 existing jsdom unhandled errors. No failure references Ring or Orb. |
| bun nx run @semio-tech/ui-react:build --skip-nx-cache | Failed independently because Storybook cannot resolve @semio-tech/coda-desktop/renderer from .storybook/stories/ui/✅ValidationTree.stories.tsx. |

## Blockers

No Orb/Ring source-split blocker remains. Validation is blocked by unrelated pre-existing workspace and UI type errors, the existing quick-test failures in Scene, icon CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and jsdom event handling, plus the unresolved Storybook Coda renderer dependency.
