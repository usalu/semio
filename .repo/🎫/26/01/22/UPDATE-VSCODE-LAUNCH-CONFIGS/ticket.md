# Ticket

## Todos

# Plan: Update VSCode Launch Configs

## Analysis

### Current State

The launch.json was updated to only include valid targets. However, some packages are missing `preflight` scripts that should be added for consistency.

### Available Targets by Project (Current)

| Project             | dev | test | build | preflight   | publish | other                                                            |
| ------------------- | --- | ---- | ----- | ----------- | ------- | ---------------------------------------------------------------- |
| repo/go             | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| repo/server         | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| repo (vscode)       | ✓   | ✓    | ✓     | ✓           | -       | publish:vsix                                                     |
| compose/go          | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| compose/rs          | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| compose/py          | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| compose/engine      | ✓   | ✓    | ✓     | ✓           | -       | -                                                                |
| compose/js          | ✓   | ✓    | ✓     | ✓           | -       | dev:storybook, dev:sketchpad, test:unit, test:e2e, test:coverage |
| compose/docs        | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| compose/play        | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| compose/desktop     | ✓   | -    | ✓     | ✓           | ✓       | -                                                                |
| compose/net         | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| compose/grasshopper | -   | ✓    | ✓     | ✓           | -       | -                                                                |
| assets              | -   | -    | -     | ✓           | -       | -                                                                |
| compose/logo        | ✓   | -    | ✓     | **MISSING** | -       | animate                                                          |
| compose/icons       | -   | -    | ✓     | **MISSING** | -       | -                                                                |
| compose/yak         | -   | -    | ✓     | **MISSING** | ✓       | -                                                                |

## Scripts to Add

1. **compose/logo** - Add `"preflight": "tsc --noEmit"` (uses TypeScript via tsx)
2. **compose/icons** - Add `"preflight": "echo \"No preflight checks configured for icons\""` (placeholder, no TypeScript)
3. **compose/yak** - Add `"preflight": "tsc --noEmit"` (uses TypeScript via tsx)

## Launch configs to add

After adding preflight scripts, add corresponding launch configs for:

- compose/logo preflight
- compose/icons preflight
- compose/yak preflight

## Files to Update

- `assets/logo/package.json` - Add preflight script
- `assets/icons/package.json` - Add preflight script
- `yak/package.json` - Add preflight script
- `.vscode/launch.json` - Add preflight launch configs

## Changes

## Log

## Summary
