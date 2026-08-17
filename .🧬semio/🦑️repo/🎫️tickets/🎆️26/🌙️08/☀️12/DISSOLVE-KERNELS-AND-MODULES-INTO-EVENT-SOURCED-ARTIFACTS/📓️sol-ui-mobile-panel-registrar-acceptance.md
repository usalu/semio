# UI Mobile Panel Registrar Acceptance

## Accepted Source State

The one-consumer implementation moved into Layout before registrar work. Final source SHA-256 values observed at the handoff:

| Path | SHA-256 |
|---|---|
| Layout | `3f00d039bb23b303172be9367b6eb53373806977613990bb9369948b3004586a` |
| Panel documentation referrer | `504b1eb08472bb2437b6eb45987b23b6d2111cf355fccf57269c42db82b11455` |
| PanelTabBar documentation referrer | `b522d8a6cdc026d020562fc8c5a2a0e2ec8c1a204036457c8f1c18922d722cb3` |
| ElementId documentation referrer | `308951b26486abda4a67e5adda3273ac8eff260e924b3f57a0728ed110cfc38d` |

The former MobilePanel component and story are absent. Scoped ordinary and cached source diff checks pass.

## Registrar Change

- React index pre-edit SHA-256: `fa8dbb145f3c31af948dc7f18bc51a931cc7cb981fcdac3bd26086e273b99f0b`.
- Removed the standalone MobilePanel import/re-export region.
- Added `LayoutMobilePanelProps` to the existing Layout import/export region.
- Reworded one owner-local dock-context docstring to reference Layout's private mobile panel without a removed link.
- Removed `MobilePanel` from the Storybook smoke-spec inventory comment.

## Evidence

- React index post-edit SHA-256: `64eb6dcf68e5c20a02409cedf789a96010f040d4144793b7de069f982795a10f`.
- Storybook smoke spec post-edit SHA-256: `ac6541e23bf754205e81c3fec1f3ff7cf800b9a176a47d10a1b200e4dc42d4ab`.
- Active stale scan for the old direct path, `MobilePanelProps`, standalone `MobilePanel`, and `<MobilePanel>`: zero matches.
- New private `LayoutMobilePanel` and public Layout-owned `LayoutMobilePanelProps` exist, and the latter is exported only with Layout.
- Scoped ordinary and cached diff checks for both registrar paths: pass.

Final Nx validation and consolidated acceptance remain with the resumed Terra lease.
