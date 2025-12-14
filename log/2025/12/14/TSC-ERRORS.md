---
date: '2025-12-14T18:08:53.458Z'
slug: TSC-ERRORS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix all TypeScript errors from tsc
model: claude-opus-4.5
---
# Previously

- The repo-wide TypeScript check (`npx tsx hooks/typescript.ts`) failed due to an empty root `tsconfig.json`, plus downstream type errors in `@semio/js` and `@semio/desktop`.
- TypeScript did not include `.storybook` files by default (dot-directory), so story typing drift was not caught by the repo-wide check.

# Plan

- Define a real root `tsconfig.json` as the canonical project for `hooks/typescript.ts`.
- Ensure `.storybook` TypeScript files are part of the repo-wide check.
- Fix all remaining TypeScript errors until `reports/typescript.json` is clean.
- Update dev docs to reflect the canonical TypeScript configuration.

# Changes

- Added a real root `tsconfig.json` for `hooks/typescript.ts` (strict + bundler resolution) with explicit `.storybook` inclusion and exclusions for temp/log/report folders.
- Fixed Sketchpad typing issues by aligning selection shapes, importing missing app plugin types, and guarding optional command invocations.
- Fixed the design diagram helper-line state to be writable via React state.
- Updated the desktop renderer to pass the Sketchpad instance identifier via `id`.
- Added `@electron/fuses` typings and dependency wiring for the desktop forge config.
- Updated Storybook stories to match current element APIs and satisfy typed `StoryObj` requirements.
