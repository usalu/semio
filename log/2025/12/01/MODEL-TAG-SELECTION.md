---
date: "2025-12-01T16:18:54.361Z"
slug: MODEL-TAG-SELECTION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Add model tag selection in footer for design and type apps
model: claude-sonnet-4.5
---

# Previously

The design and type apps were rendering placeholder geometry (boxes) instead of actual 3D models. The model loading infrastructure existed (`TypeMesh` and `PieceMesh` components) but there was no UI for selecting model tags to filter which model variant is displayed.

# Plan

1. Add validation rule for 3D file extensions in semio.ts
2. Add footer component for tag selection in Type app
3. Add footer component for tag selection in Design app
4. Update documentation

# Changes

## semio.ts

- Added `SUPPORTED_3D_EXTENSIONS` constant with all common 3D file formats supported by Three.js loaders (gltf, glb, fbx, obj, dae, stl, etc.)
- Added `isSupportedModelExtension(filename)` helper function
- Added `validateModelFile(filename)` function that returns validation result with optional warning for unsupported extensions

## Type.tsx - TypeAppFooter

- Updated `TypeAppFooter` component to show all tag names from the type's models
- Tags are clickable to toggle selection
- Selected tags appear with foreground color and font-medium, unselected with muted-foreground
- Uses existing `useTypeAppSelectedModelTags`, `addModelTag`, `removeModelTag` hooks/commands
- Tag selection triggers model filtering via Jaccard index (existing `selectBestModel` logic)

## Design.tsx - DesignAppFooter

- Updated `DesignAppFooter` component to show all tag names from all types used in the design
- Collects unique tags across all types that have pieces in the current design
- Added new commands: `setModelTagsForType`, `addModelTagForAllTypes`, `removeModelTagForAllTypes`
- Clicking a tag toggles it for all types that have models with that tag
- Uses existing `useDesignAppSelectedModelTags` hook (returns `Record<Guid, string[]>` mapping type guids to selected tag guids)
- Tag selection affects model display in scene via existing `PieceMesh` component logic
