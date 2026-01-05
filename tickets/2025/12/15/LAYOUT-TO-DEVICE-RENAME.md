---
slug: LAYOUT-TO-DEVICE-RENAME
prompt: Rename layout in useLayout to device and useDevice, etc. All types enums etc. Not the Layout component.
summary: Rename layout to device in hooks, types, and enums
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-16T17:06:07.957Z
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: Rename layout in useLayout to device and useDevice, etc. All types enums etc. Not the Layout component.
    model: claude-opus-4-5
    date:
      started: 2025-12-15T13:32:41.372Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
  - prompt: FINISH Rename layout in useLayout to device and useDevice, etc. All types enums etc. Not the Layout component.
    model: claude-opus-4-5
    date:
      started: 2025-12-15T14:00:17.150Z
    commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
    bundles:
      "@semio":
        files:
          AGENTS.md:
            sections:
              AGENTS:
                lines:
                  added: 111
                  removed: 141
          README.md:
            sections:
              README:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Design.tsx:
            sections:
              Design:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Home.tsx:
            sections:
              Home:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Kit.tsx:
            sections:
              Kit:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Quality.tsx:
            sections:
              Quality:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Sketchpad.tsx:
            sections:
              Sketchpad:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/Type.tsx:
            sections:
              Type:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/locales/de.json:
            sections:
              De:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/locales/en.json:
            sections:
              En:
                lines:
                  added: 111
                  removed: 141
          js/js/sketchpad/shared.ts:
            sections:
              Shared:
                lines:
                  added: 111
                  removed: 141
          log/prompts.md:
            sections:
              Prompts:
                lines:
                  added: 111
                  removed: 141
          log/tickets/2025/12/15/LAYOUT-TO-DEVICE-RENAME.md:
            sections:
              LAYOUT TO DEVICE RENAME:
                lines:
                  added: 111
                  removed: 141
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
