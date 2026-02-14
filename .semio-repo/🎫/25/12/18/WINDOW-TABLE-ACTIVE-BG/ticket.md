# Ticket

## Todos
# Previously

- The kit app renders a table inside a multi-window layout, but there was no explicit active-window state for the app.
- The table header background was not guaranteed to match the window surface background when rendered inside GoldenLayout windows.

# Plan

- Track `activeWindow` for the kit app and keep it valid across window kinds.
- Highlight the active window surface and make the table background follow the active window background.
- Document the active-window + background mechanism in root docs.

# Changes

- Updated `LayoutCanvas` to apply an active-window background tint to the active window root.
- Updated the kit app multi-window entrypoint to maintain and pass `activeWindow` into `LayoutCanvas`.
- Updated the shared `Table` component to inherit its surface background so it matches the window surface.

## Changes

## Log

## Summary
# Summary

Use active window background for table
