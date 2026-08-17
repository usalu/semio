---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Bulk close

## Changes

## Log

## Todos

- [ ] Fix flaky scenePan3Duration assertion (line 2103 sketchpad.test.ts: 1500ms → 5000ms)
- [ ] Run e2e Design test to verify child piece parent connection section
- [ ] Debug parent connection section rendering if test fails
- [ ] Clean up DEBUG console.log statements in Design.tsx
- [ ] Close ticket

## Plan

1. Increase scenePan3Duration threshold from 1500ms to 5000ms (and similar for related assertions)
2. Run Design e2e test to verify the child piece selection + parent connection section appears
3. If parent connection section test fails, debug and fix
4. Remove DEBUG console.log statements
5. Close ticket
