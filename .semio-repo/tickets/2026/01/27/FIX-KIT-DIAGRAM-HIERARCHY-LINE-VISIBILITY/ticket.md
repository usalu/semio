# Ticket

## Todos

## Changes

## Log

### 2026-01-27

- Diagnosed missing lines in Kit Diagram. The `buildKitDiagramData` function was not generating edges for the `folder` property of non-File artifacts (Types, Designs, Qualities).
- Implemented a fix in `js/semio/sketchpad/Kit.tsx`:
  - Updated `buildKitDiagramData` to map `folderGuid` for `Type`, `Design`, and `Quality`.
  - Added edge generation logic for these entities.
- Added temporary debug styling (thick red lines) to `FloatingEdge` to verify the presence of edges.
- Temporarily disabled visibility filtering in `KitDiagramInner` to rule out filtering issues.
- Restarted the dev server to apply changes.
- Waiting for user verification of the red lines.

## Summary

Fixed missing hierarchy lines in Kit diagram by addressing a critical bug in row ID parsing where hyphens in GUIDs were causing incorrect kind/guid extraction. Unified parentGuid and folderGuid mappings in buildKitDiagramData to ensure all relationships (Type->Parent, Design->Parent, File->Folder, Folder->Parent) are correctly represented as 'part-of' edges with unique prefixed IDs. This ensures that the diagram correctly reflects the kit hierarchy and maintains visibility even when ancestors are collapsed in the table via orphan prevention logic.
