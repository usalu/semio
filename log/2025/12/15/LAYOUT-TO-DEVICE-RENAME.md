---
slug: LAYOUT-TO-DEVICE-RENAME
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: 'Rename layout to device in hooks, types, and enums'
model: claude-opus-4.5
input:
  - prompt: >-
      Rename layout in useLayout to device and useDevice, etc. All types enums
      etc. Not the Layout component.
    date: '2025-12-15T13:32:41.372Z'
  - prompt: >-
      FINISH Rename layout in useLayout to device and useDevice, etc. All types
      enums etc. Not the Layout component.
    date: '2025-12-15T14:00:17.150Z'
commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
files:
  updated:
    - AGENTS.md
    - README.md
    - js/js/sketchpad/Design.tsx
    - js/js/sketchpad/Home.tsx
    - js/js/sketchpad/Kit.tsx
    - js/js/sketchpad/Quality.tsx
    - js/js/sketchpad/Sketchpad.tsx
    - js/js/sketchpad/Type.tsx
    - js/js/sketchpad/locales/de.json
    - js/js/sketchpad/locales/en.json
    - js/js/sketchpad/shared.ts
    - log/2025/12/15/LAYOUT-TO-DEVICE-RENAME.md
    - log/prompts.md
lines:
  added: 1447
  removed: 1836
---
# Previously

- Sketchpad exposed the global interaction mode as `layout` (`useLayout()`, `SET_LAYOUT`, `settings.layout.*` i18n IDs).
- The rename to `device` was started, but app settings UIs, locale trees, and docs still referenced `layout`.

# Plan

- Replace remaining `useLayout` call sites with `useDevice` and rename related local variables.
- Migrate settings IDs from `*.settings.layout*` to `*.settings.device*` and update `en.json`/`de.json` accordingly.
- Update developer docs to reflect the new naming and reserved meaning of `layout` (window layouts + `Layout` component).
- Update log metadata (affected files + stats).

# Changes
- Renamed the global UI setting from `layout` to `device` across Sketchpad hooks, state/events, and app settings UIs.
- Migrated i18n IDs and locale trees from `settings.layout` to `settings.device` (including Home app settings).
- Updated developer docs to reference `Device`/`useDevice`/`SET_DEVICE` and clarify `layout` naming usage.
