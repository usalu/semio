# Diagram Piece Must Render In Scene With Correct Type Model

## Goal

Fix and validate Sketchpad Design behavior so newly added diagram pieces render in scene with the correct type model.

## Plan

1. Analyze piece creation flow (addPiece command, drag-drop, plus-add-piece)
2. Analyze scene rendering pipeline (PieceMesh, ModelPiece, model resolution)
3. Identify bugs preventing piece from showing in scene with correct model
4. Fix Design.tsx as needed
5. Update sketchpad.test.ts with assertions for scene model resolution
6. Run tests and verify

## Context

- Pieces are added via `compose.designApp.addPiece` command with `{ guid, type: { guid }, center, plane }`
- Scene renders pieces through `ModelPiece` → `PieceMesh` → `LoadedPieceMesh`
- PieceMesh resolves type, finds model via `selectBestModel`, then loads file blob URL
- Piece needs `plane` for scene positioning (used in `ModelPiece` via `planeToMatrix`)

## Analysis

### Piece Creation Paths

1. **Drag-drop from workbench**: `handleDragEnd` creates piece with `{ guid, type: { guid }, center, plane }`
2. **Plus-add-piece button**: onClick creates piece with `{ guid: guid(), type: { guid: type.guid }, center: { u: 0, v: 0 }, plane }`

Both paths correctly provide `center` and `plane` data.

### Scene Rendering Pipeline

1. `ModelPiece` gets piece via `usePiece()`, derives plane from `piece.plane || flatPlane`
2. `PieceMesh` resolves type, finds best model, gets file URL
3. `LoadedPieceMesh` renders 3D mesh (GLTF, FBX, OBJ)
4. `pieceMatrix = planeToMatrix(plane)` positions the mesh

### Model Resolution

- `selectBestModel(type.models, selectedTagGuids)` picks the right model
- Model's file guid is resolved to a blob URL via `kitStore.getFileBlobUrl`

## Changes

## Summary

## Status

- [x] Analyzed piece creation flow
- [x] Analyzed scene rendering pipeline
- [ ] Identify and fix bugs
- [ ] Update tests
- [ ] Verify tests pass
