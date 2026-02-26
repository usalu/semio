---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Restructured the Design app piece details panel to display piece information and parent connection as two separate collapsible TreeItem groups. When a piece is selected, the details panel now shows:
- **Piece** (collapsible): ID, Name, Type/Design, Variant, Description, Scale, Color, Center, Plane, Attributes
- **Parent Connection** (collapsible): Connecting/Connected info, Plane (Translation/Orientation), Diagram offsets

Added i18n keys for `pieceInfo`, `properties`, `multipleTitle` under `section.piece` and `section.connection` in both en.json and de.json locale files.

## Changes

- `semio/js/sketchpad/Design.tsx`: Wrapped piece fields inside `<TreeItem id="...pieceInfo" defaultOpen={true}>` collapsible group. Parent connection remains at sibling level as `<TreeItem id="...parentConnection">`.
- `semio/js/sketchpad/locales/en.json`: Added `pieceInfo`, `properties`, `multipleTitle` i18n keys for piece section; `properties`, `multipleTitle` for connection section.
- `semio/js/sketchpad/locales/de.json`: Same i18n keys with German translations (Bauteil, Bauteile, Verbindung, Verbindungen).

## Log

1. Analyzed current Design.tsx Details section structure (lines 4149-6024)
2. Studied old Design.Details.tsx.old reference implementation
3. Studied TreeItem/TreeSection component behavior in elements.tsx
4. Studied i18n system (useLabel resolves dot-separated keys)
5. Added i18n keys to en.json and de.json
6. Restructured PiecesSectionForm return JSX to wrap piece fields in collapsible TreeItem
7. Verified zero TypeScript errors
8. Verified dev server compiles cleanly

## Todos

- [x] Analyze current Design.tsx details section
- [x] Study old build reference
- [x] Study TreeItem/i18n components
- [x] Add i18n keys for piece/connection sections
- [x] Restructure PiecesSectionForm JSX with TreeItem groups
- [x] Verify compilation
- [x] Close ticket

## Plan

Wrap piece-specific fields in a collapsible `<TreeItem id="...pieceInfo">` group and keep parent connection as a sibling-level `<TreeItem id="...parentConnection">` group within the PiecesSectionForm component.
