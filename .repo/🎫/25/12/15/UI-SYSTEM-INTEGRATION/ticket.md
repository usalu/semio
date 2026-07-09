# Ticket

## Todos

# Previously

# The UI “horizontal container” components existed in multiple competing forms inside `js/compose/sketchpad/elements.tsx`, causing type mismatches (and effectively preventing consistent usage).

# TypeScript errors surfaced across Sketchpad and Storybook due to incompatible `Band`/`Navbar` props and missing `UiContext` fields used by the UI state machine.

# Plan

# 1. Run TypeScript hook and use the JSON report as the source of truth for failures.

# 2. Consolidate `Band`/`Navbar`/`Strip` into a single API with fixed system heights and minimal props.

# 3. Update all usages in Sketchpad and Storybook to the new APIs.

# 4. Fix remaining TypeScript errors in the UI state machine and tooling.

# 5. Update dev docs for the new UI sizing + container APIs and add required post-change checks.

# Changes

# - Unified `Band`, introduced `Strip`, and refactored `Navbar` as a non-scrollable band-style container.

# - Updated Sketchpad usage to the new `Navbar.items` + `Band.items` contracts.

# - Restored type correctness for the UI state machine by adding missing `UiContext` app maps and initializing them in machine context.

# - Fixed `scripts/log.ts` sorting to use `frontmatter.date.updated` instead of the `date` object.

# - Ran TypeScript hook until clean.

## Changes

## Log

## Summary

# Summary

Integrate UI system with fixed heights, bands/strips, and action text
