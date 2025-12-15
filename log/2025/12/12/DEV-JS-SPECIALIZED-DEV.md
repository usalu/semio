---
date:
  created: '2025-12-12T22:37:27.485Z'
  updated: '2025-12-12T22:37:27.485Z'
slug: DEV-JS-SPECIALIZED-DEV
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Split @semio/js dev scripts
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

# `@semio/js` runs both Vite and Storybook from a single `dev` entrypoint (`js/js/dev.ts`).

# Root and VS Code configs lacked dedicated entrypoints for running just one of these when working in parallel.

# Plan

# Add dedicated package-level scripts for Vite-only and Storybook-only runs.

# Add root shortcuts and VS Code tasks/launch configs for each specialization.

# Document the naming and wiring in dev docs.

# Changes

# Added `dev:storybook` and `dev:sketchpad` to `js/js/package.json` and refactored `js/js/dev.ts` to reuse them.

# Added root shortcuts `dev:storybook` and `dev:sketchpad` and matching `.vscode` tasks/launch configs.

# Standardized VS Code naming to hierarchical task/launch names (`dev js js storybook`, `dev js js sketchpad`) while keeping `dev:<...>` root shortcuts for CLI usage.
