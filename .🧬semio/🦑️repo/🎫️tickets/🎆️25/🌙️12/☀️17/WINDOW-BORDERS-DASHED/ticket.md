# Ticket

## Todos

# Previously

GoldenLayout window borders were clipped on the outer edges (bottom/right) and window borders were not visually distinct enough for the intended “window boundary” semantics.

# Plan

Enforce border-box sizing for GoldenLayout items, switch window borders to dashed style for both Canvas and GoldenLayout windows, and document the border + spacing mechanism.

# Changes

GoldenLayout items now use border-box sizing and stacks render a dashed border so borders remain visible on all sides. Canvas windows use a dashed `border-window` border while GoldenLayout windows use `kind=\"layout\"` to avoid nested borders. Developer documentation updated to reflect dashed window borders.

## Changes

## Log

## Summary

# Summary

Sketchpad windows have continuous dashed borders
