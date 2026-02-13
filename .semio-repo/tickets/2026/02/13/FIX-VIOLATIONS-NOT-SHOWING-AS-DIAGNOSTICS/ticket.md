---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Fixed violations not showing as VS Code diagnostics. Two root causes fixed: (1) Data path mismatch — CLI returns {"analyze": {"violations": [...]}} but code checked result.data.violations instead of result.data.analyze.violations. (2) No event handlers to trigger file analysis on open/save/change — added onDidOpenTextDocument, updated onDidSaveTextDocument, and added debounced onDidChangeTextDocument (1.5s).
## Changes

- `semio-repo/vscode/extension.ts`: Fixed `analyzeFile()` to unwrap the `analyze` envelope from CLI JSON output (`result.data.analyze.violations` instead of `result.data.violations`). Also cleans up `fileViolationsMap` when violations are empty.
- `semio-repo/vscode/extension.ts`: Added `onDidOpenTextDocument` handler to analyze files when opened.
- `semio-repo/vscode/extension.ts`: Updated `onDidSaveTextDocument` handler to also trigger `analyzeFile` and `validateKitDocument` on save.
- `semio-repo/vscode/extension.ts`: Added debounced `onDidChangeTextDocument` handler (1.5s debounce) to provide live diagnostics while editing.

## Log

- Ran `./semio-repo/cli/cli --json analyze semio` to see actual CLI output format: `{"analyze":{"metrics":{...},"violations":[...]}}`
- Identified data path mismatch: code expected `result.data.violations` but data is at `result.data.analyze.violations`
- Identified missing event handlers for file open/save/change
- Applied both fixes
- Build succeeds (vite build + tsc --noEmit)

## Todos

- [x] Diagnose the data path mismatch in analyzeFile
- [x] Fix analyzeFile to correctly extract violations from CLI output
- [x] Add event handlers for file open/save/change to trigger analysis
- [x] Build and verify diagnostics work

## Plan

### Root Cause Analysis

1. **Data path mismatch**: CLI `analyze` outputs `{"analyze": {"violations": [...]}}`.
   - `extractRepoResult` wraps this as `{data: {analyze: {violations: [...]}}}`
   - `runRepoCommandJson` returns `{data: {analyze: {...}}, output: {...}}`
   - `analyzeFile` checks `result?.data?.violations` which is `undefined` — violations are at `result.data.analyze.violations`

2. **Missing event handlers**: `analyzeFile` is only called once at activation via `setTimeout`. No handlers for:
   - `onDidOpenTextDocument`
   - `onDidSaveTextDocument` (existing handler only refreshes tree)
   - `onDidChangeTextDocument` (with debounce)

### Fix Plan

1. Fix `analyzeFile` to unwrap the `analyze` envelope from CLI output
2. Add `onDidOpenTextDocument`, `onDidSaveTextDocument`, and debounced `onDidChangeTextDocument` handlers
3. Build and verify
