# Ticket

## Todos
# Previously

# Plan

- Introduce `fix` and `analyze` as first-class root commands.
- Make `preflight` run `fix` then `analyze` and keep husky pre-commit on `preflight`.
- Make `test → preflight → test`, `build → test → build`, `prepublish/publish → build → <target>`.
- Add a consistent skip mechanism and Nx arg passthrough for scoping.
- Update hooks, Nx defaults, and VS Code tasks/launch to match.

# Changes

- Added `preflight.ts` as the orchestrator for the new pipeline and wired root scripts to it.
- Split the pipeline into `fix` (formatters/autofix) and `analyze` (non-mutating checks + reports).
- Added `--skip=...` and `--nx ...` passthrough to keep the pipeline composable and scoping-friendly.
- Updated VS Code tasks/launch entries and aligned ESLint hook to accept forwarded Nx args.

## Changes

## Log

## Summary
# Summary

Integrate analyze/fix CI pipeline
