---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Reverted all workbench-to-main-container migration changes by restoring 16 files to their pre-migration state (commit 5381e7e37). Workbench is back as a left toggle panel (PanelKind.WORKBENCH) in shared.ts, with workbench panels in Design, Quality, Docs, Type, Kit apps. TypeScript compiles with 0 errors. 6/7 e2e tests pass (Design drag test is pre-existing flaky).
## Changes

## Log

## Todos

## Plan
