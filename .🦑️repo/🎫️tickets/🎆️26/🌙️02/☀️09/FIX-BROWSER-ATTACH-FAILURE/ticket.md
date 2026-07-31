# Ticket

## Todos

- [x] Identify root cause of VS Code browser attach failure.
- [x] Replace browser-attach launch profiles with attach-free dev launch profiles.
- [x] Keep one-click URL open from Run and Debug.
- [x] Update root docs (`README.md`, `AGENTS.md`) with attach-free launch requirements.
- [x] Validate launch target and URL patterns.

## Changes

- Converted app browser-launch configs in `.vscode/launch.json` to attach-free Node launches:
  - `@semio-tech/compose-js dev`
  - `@semio-tech/compose-js dev:storybook`
  - `@semio-tech/compose-js dev:sketchpad`
  - `@semio-tech/compose-sketchpad-docs dev`
  - `@semio-tech/compose-sketchpad-play dev`
- Added `serverReadyAction` with `openExternally` for those profiles so URLs open automatically once dev servers are ready.
- Updated sketchpad launch URL pattern to `http://localhost:5173` to match `dev:sketchpad`.
- Updated README bundle docs to describe attach-free launch behavior.
- Updated AGENTS SRS and Codebase docs to require Node + `serverReadyAction` launch behavior for remote/devcontainer workflows.
- Added ticket plans in `plan_1.md`, `plan_2.md`, and `plan_3.md`.

## Log

- Reopened ticket after repeated `Unable to attach browser` reports.
- Removed browser debugger launch dependency from affected dev profiles.
- Replaced with Node launches and server-ready URL opening.
- Aligned sketchpad port to `5173` in launch URL pattern.
- Updated root docs to codify the mechanism.

## Summary

Switched app dev launch profiles to attach-free node launches with serverReadyAction openExternally to eliminate Unable to attach browser failures in remote/devcontainer sessions.
