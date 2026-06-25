---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Fix incorrect VSCode launch.json and tasks.json configs.

## Changes

- `.vscode/launch.json`
- `.vscode/tasks.json`

## Log

### Analysis

nx projects and their actual roots:

- `@semio-tech/semio-logo` → `assets/logo` (configs say `compose/logo` — wrong)
- `@semio-tech/semio-icons` → `assets/icons` (configs say `compose/icons` — wrong)
- `@semio-tech/compose-net` → `compose/net/Compose` (configs say `compose/net` — wrong)
- `@semio-tech/compose-sketchpad` → `compose/sketchpad` (missing from configs)
- `compose/grasshopper` → does NOT exist in nx (remove)
- `compose/yak` → does NOT exist in nx (remove)
- `repo/vscode dev` task uses `"type": "npm", "path": "js/vscode"` — path is wrong (old path), should use shell `npx nx dev repo`

## Todos

- [x] Fix launch.json: wrong nx refs (logo, icons, net), remove grasshopper/yak, add sketchpad
- [x] Fix tasks.json: wrong nx refs, fix repo/vscode dev task, remove grasshopper/yak, add sketchpad

## Plan

1. Fix `repo/vscode dev` task: change from npm type with `js/vscode` path to shell `npx nx dev repo`
2. Fix nx paths: `compose/logo` → `assets/logo`, `compose/icons` → `assets/icons`, `compose/net` → use `@semio-tech/compose-net`
3. Remove non-existent: `compose/grasshopper`, `compose/yak`
4. Add missing: `compose/sketchpad`
