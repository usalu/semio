# Ticket

## Todos
# Previously

Create actions in the Kit app are triggered via filter-strip `Toggle` action buttons.

# Plan

Trace UI `actionId` → `onActionClick` → kit command wiring, then make create actions immediately visible by switching the kind filter and selection to the created artifact.

# Changes

Updated Kit app create actions for ports/tags (and aligned concepts/folders) to set the current kind filter and selection to the newly created entity, and moved default names to i18n-backed labels.

## Changes

## Log

## Summary
# Summary

Fix create port/tag buttons doing nothing
