# Ticket

## Todos

# Plan: Update VSCode Launch Configs

## Analysis

### Current State

The launch.json was updated to only include valid targets. However, some packages are missing `preflight` scripts that should be added for consistency.

### Available Targets by Project (Current)

| Project             | dev | test | build | preflight   | publish | other                                                            |
| ------------------- | --- | ---- | ----- | ----------- | ------- | ---------------------------------------------------------------- |
| semio-repo/go       | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio-repo/server   | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio-repo (vscode) | ✓   | ✓    | ✓     | ✓           | -       | publish:vsix                                                     |
| semio/go            | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/rs            | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/py            | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/engine        | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/js            | ✓   | ✓    | ✓     | ✓           | -       | dev:storybook, dev:sketchpad, test:unit, test:e2e, test:coverage |
| semio/docs          | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| semio/play          | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| semio/desktop       | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| semio/net           | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/grasshopper   | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| semio/assets        | -   | -    | -     | ✓           | -       | -                                                                |
| semio/logo          | ✓   | -    | ✓     | **MISSING** | -       | animate                                                          |
| semio/icons         | -   | -    | ✓     | **MISSING** | -       | -                                                                |
| semio/yak           | -   | -    | ✓     | **MISSING** | ✓       | -                                                                |

## Scripts to Add

1. **semio/logo** - Add `"preflight": "tsc --noEmit"` (uses TypeScript via tsx)
2. **semio/icons** - Add `"preflight": "echo \"No preflight checks configured for icons\""` (placeholder, no TypeScript)
3. **semio/yak** - Add `"preflight": "tsc --noEmit"` (uses TypeScript via tsx)

## Launch configs to add

After adding preflight scripts, add corresponding launch configs for:

- semio/logo preflight
- semio/icons preflight
- semio/yak preflight

## Files to Update

- `assets/logo/package.json` - Add preflight script
- `assets/icons/package.json` - Add preflight script
- `yak/package.json` - Add preflight script
- `.vscode/launch.json` - Add preflight launch configs

## Changes

## Log

## Summary
