# Ticket

## Todos

- [x] Identify source of `appCommands` runtime ReferenceError in Design app.
- [x] Patch design panel hotkey wiring to use available command hook output.
- [x] Update root developer docs (`README.md`, `AGENTS.md`) with panel hotkey command-binding requirement.
- [x] Attempt targeted verification for toolbar/design app path.

## Changes

- Updated `js/semio/sketchpad/Design.tsx` hotkey panel-toggle callback to invoke `togglePanel` from `useDesignAppTogglePanel()`.
- Updated `README.md` under `### Sketchpad` / `#### Sketchpad toolbar tooltree` with the direct Design panel-hotkey action binding requirement.
- Updated `AGENTS.md`:
  - `# Software Requirements Specification` → `## UI/UX` → `#### Design Editor` with panel-hotkey command-container constraint.
  - `# Codebase` with `#### Design app panel hotkeys` implementation note for `js/semio/sketchpad/Design.tsx`.

## Log

- Reproduced issue context from reported stack trace (`Design.tsx:8011`).
- Found stale `appCommands` identifier referenced inside `togglePanelHotkey` callback but not declared in `App` scope.
- Confirmed `useDesignAppTogglePanel()` already provides `togglePanel` in the same component scope.
- Rewired callback to call `togglePanel?.(panelKey)` and updated `useCallback` dependency list.
- Attempted targeted Playwright run via:
  - `pnpm -C js/semio test:e2e playwright/sketchpad/toolbar.spec.ts`
  - `pnpm -C js/semio exec playwright test playwright/sketchpad/toolbar.spec.ts`
- Verification is currently blocked because `playwright` executable is unavailable in this environment.

## Summary

Fixed Design app panel hotkeys to use useDesignAppTogglePanel output instead of undefined appCommands and documented the command binding requirements in README.md and AGENTS.md.
