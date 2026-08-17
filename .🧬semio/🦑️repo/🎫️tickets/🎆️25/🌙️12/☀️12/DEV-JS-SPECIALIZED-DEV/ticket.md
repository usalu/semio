# Ticket

## Todos

# Previously

# `compose/js` runs both Vite and Storybook from a single `dev` entrypoint (`js/compose/dev.ts`).

# Root and VS Code configs lacked dedicated entrypoints for running just one of these when working in parallel.

# Plan

# Add dedicated package-level scripts for Vite-only and Storybook-only runs.

# Add root shortcuts and VS Code tasks/launch configs for each specialization.

# Document the naming and wiring in dev docs.

# Changes

# Added `dev:storybook` and `dev:sketchpad` to `js/compose/package.json` and refactored `js/compose/dev.ts` to reuse them.

# Added root shortcuts `dev:storybook` and `dev:sketchpad` and matching `.vscode` tasks/launch configs.

# Standardized VS Code naming to hierarchical task/launch names (`dev js js storybook`, `dev js js sketchpad`) while keeping `dev:<...>` root shortcuts for CLI usage.

## Changes

## Log

## Summary

# Summary

Split compose/js dev scripts
