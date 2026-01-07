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
