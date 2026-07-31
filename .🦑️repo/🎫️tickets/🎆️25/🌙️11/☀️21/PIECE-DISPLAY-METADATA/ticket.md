# Ticket

## Todos

# Piece Display Metadata

## Problem

For displaying purposes, every piece needs:

- **Center** (Coord) for diagram positioning
- **Plane** for scene 3D positioning

Both come from the flattened design which depends deeply on the kit structure.

## Current State

- `usePiecesMetadata()` exists in App.tsx and returns a Map with plane, center, and document info
- This metadata comes from `design.getFlatPiecesMetadata()` which calculates positions during flattening
- The data is already available but not exposed as individual hooks

## Implementation Status

### ✅️ Completed

#### 1. Hooks added to DesignStore (App.tsx) - ALREADY IMPLEMENTED

- ✅️ `usePieceCenter(pieceId?: Guid): Coord | undefined` - Get diagram center for a piece
- ✅️ `usePiecePlane(pieceId?: Guid): Plane | undefined` - Get scene plane for a piece
- These use the existing `usePiecesMetadata()` internally
- Located at lines 4958-4971 in App.tsx

#### 2. Hooks added to DesignAppStore (design/App.tsx) - ALREADY IMPLEMENTED

- ✅️ `useDesignAppPieceCenter(id?: DesignAppId, pieceId?: Guid): Coord | undefined`
- ✅️ `useDesignAppPiecePlane(id?: DesignAppId, pieceId?: Guid): Plane | undefined`
- These bridge to the DesignStore hooks with the design app context
- Located at lines 1935-1950 in design/App.tsx

#### 3. Scene component - ALREADY USING HOOKS

- ✅️ `ModelPiece` component uses `useFlatPiecePlane()` for piece positioning (line 5901)
- ✅️ Plane data flows through metadata to the scene correctly

#### 4. Diagram component - ALREADY USING METADATA

- ✅️ `designToNodesAndEdges` function uses metadata for centers (line 4603)
- ✅️ Creates `centerMap` from `flattenedDesign.pieces` which contains calculated centers
- ✅️ Uses centers for both piece nodes and design nodes

## Verification

All necessary hooks and components are already implemented and using the flattened design metadata:

### Data Flow (Verified)

```
Kit → Design → getFlatPiecesMetadata() → usePiecesMetadata() → individual hooks
                                                                    ↓
                                                  Scene (via useFlatPiecePlane)
                                                  Diagram (via metadata param)
```

### Scene Component Chain

```
DesignAppScene → ModelDesign → ModelPiece → useFlatPiecePlane()
```

### Diagram Component Chain

```
DesignDiagram → designToNodesAndEdges(metadata) → centerMap → node positions
```

## Conclusion

The implementation is complete. All pieces are displayed using calculated centers and planes from the flattened design metadata. No changes are needed.

## Changes

## Log

## Summary
