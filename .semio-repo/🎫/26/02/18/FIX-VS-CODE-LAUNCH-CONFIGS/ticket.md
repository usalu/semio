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
- `@semio/logo` → `semio/assets/logo` (configs say `semio/logo` — wrong)
- `@semio/icons` → `semio/assets/icons` (configs say `semio/icons` — wrong)
- `@semio/net` → `semio/net/Semio` (configs say `semio/net` — wrong)
- `@semio/sketchpad` → `semio/sketchpad` (missing from configs)
- `semio/grasshopper` → does NOT exist in nx (remove)
- `semio/yak` → does NOT exist in nx (remove)
- `semio-repo/vscode dev` task uses `"type": "npm", "path": "js/vscode"` — path is wrong (old path), should use shell `npx nx dev semio-repo`

## Todos

- [x] Fix launch.json: wrong nx refs (logo, icons, net), remove grasshopper/yak, add sketchpad
- [x] Fix tasks.json: wrong nx refs, fix semio-repo/vscode dev task, remove grasshopper/yak, add sketchpad

## Plan

1. Fix `semio-repo/vscode dev` task: change from npm type with `js/vscode` path to shell `npx nx dev semio-repo`
2. Fix nx paths: `semio/logo` → `semio/assets/logo`, `semio/icons` → `semio/assets/icons`, `semio/net` → use `@semio/net`
3. Remove non-existent: `semio/grasshopper`, `semio/yak`
4. Add missing: `semio/sketchpad`
