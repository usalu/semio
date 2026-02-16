---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT
---

# Ticket

## Summary

Reordered both `.vscode/launch.json` and `.vscode/tasks.json` from package-based grouping to task-kind-based grouping. Renamed generic debuggers. Unified vscode package into build.

## Changes

- `.vscode/launch.json`: Reordered sections to BUILD → PUBLISH:TEST → PUBLISH → DEV. Renamed `Go` → `Go dev`, `Python` → `Python dev`. Removed separate `semio-repo/vscode package` entry (unified into build).
- `.vscode/tasks.json`: Reordered sections to BUILD → TEST → PREFLIGHT → ANALYZE & FIX → PUBLISH:TEST → PUBLISH → UPDATE → DEV → MCP. Removed separate `semio-repo/vscode package` task.

## Log

- Read current launch.json and tasks.json structure
- Rewrote launch.json grouped by task kind instead of package name
- Rewrote tasks.json grouped by task kind instead of package name
- Renamed generic language debuggers to "Go dev" and "Python dev"
- Merged semio-repo/vscode package into build section

## Todos

- [x] Reorder launch.json by task kind
- [x] Rename generic debuggers to Go dev, Python dev
- [x] Unify semio-repo/vscode package into build
- [x] Reorder tasks.json consistently

## Plan

1. Read current launch.json and tasks.json
2. Rewrite launch.json grouped by task kind: BUILD, PUBLISH:TEST, PUBLISH, DEV
3. Rewrite tasks.json grouped by task kind: BUILD, TEST, PREFLIGHT, ANALYZE & FIX, PUBLISH:TEST, PUBLISH, UPDATE, DEV, MCP
4. Rename Go/Python generic debuggers to "Go dev"/"Python dev"
5. Remove separate semio-repo/vscode package entry, keep only build
