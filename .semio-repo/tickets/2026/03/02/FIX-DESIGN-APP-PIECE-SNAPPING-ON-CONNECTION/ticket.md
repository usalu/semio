---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed piece snapping by using the computed flat plane from flattenDesign for connected child pieces in the Scene window. Root cause was ModelPiece always preferring stored piece.plane over the computed flatPlane. Added useIsConnectedPiece check so root pieces keep their user-placed position while connected child pieces use the snap-computed plane. All 14 unit tests pass.
## Changes

## Log

## Todos

## Plan
