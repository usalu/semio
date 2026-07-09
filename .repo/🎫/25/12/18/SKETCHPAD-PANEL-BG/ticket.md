# Ticket

## Todos

# Previously

- Panel containers were rendered without enforcing panel level context and background.
- With a global base level provider, panels could inherit base background instead of `bg-panel`.

# Plan

- Ensure the shared `Panel` component runs under `LevelProvider level="panel"`.
- Ensure panel base containers paint `bg-panel` (for default `showBackground=true`).
- Document the invariant in root docs.

# Changes

- Wrapped `Panel` in `LevelProvider level="panel"` and applied `bg-panel` to the panel container.
- Documented panel level enforcement in `README.md` and `AGENTS.md`.

## Changes

## Log

## Summary

# Summary

Ensure panels use panel background and panel level context
