# Puzzle 3D window Add Object button

## Symptom

Each Puzzle 3D viewport window (Top / Perspective split) showed an **Add Object…** quick-action chip (`shell-menu.action.openAddObjectDialog`) overlaid inside the window chrome.

## Cause

`main::engagement` published one `WindowEngagementOption` pointing at `openAddObjectDialog`. The framework `Window` component renders `engagement.options` as `data-slot="window-engagement-quick-actions"` on every window instance, so both split panes duplicated the control.

## Intended UX

- **Catalogue**: drag object kinds into the viewport (`addObjectKind`).
- **Shell menu / palette**: `openAddObjectDialog` (declared action, `in_palette`).
- **Not** inside viewport window chrome.

## Fix

Removed `engagement.options` from `main::engagement` and updated `engagement_exposes_no_utility_switch_options`.
