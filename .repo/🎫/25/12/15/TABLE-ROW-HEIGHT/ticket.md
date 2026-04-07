# Ticket

## Todos
# Previously

# Sketchpad tables target `h-medium` rows but table body `td` used `p-single`, so rows expanded beyond the fixed height whenever a cell contained an `h-medium` control (Toggle/Input/etc).

# Plan

- Align the `Table` primitive so row height is enforced by the row, not by additive cell padding.
- Keep horizontal padding (`px-single`) and vertically center all cell content.
- Update dev docs to capture the table sizing mechanism.

# Changes

- Updated `Table` and `TableSkeleton` body cells to use `px-single py-0` and wrap cell content in a vertically centered `h-full` flex container.

## Changes

## Log

## Summary
# Summary

Normalize Sketchpad table row heights
