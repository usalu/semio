# Enhance Section Tree Unfoldability and Sorting

## Plan

- [x] Analyze `CodebaseProvider` in `js/vscode/extension.ts`.
- [x] Implement line-based sorting for sections and definitions.
- [x] Implement proper section document traversal to avoid duplicate definitions.
- [x] Ensure sections with definitions are collapsible.
- [x] Sort items by appearance (start line).

## Changes

- Modified `js/vscode/extension.ts`:
    - Refactored `CodebaseProvider` class.
    - Added `sortItemsByLine` and `getItemStartLine` helpers.
    - Updated `buildFileChildren` to sort items.
    - Updated `buildSectionChildren` to filter definitions belonging to child sections and sort items.
    - Ensured `hasChildren` flag correctly reflects existence of child definitions.

## Summary

Refactored the `CodebaseProvider` in the VS Code extension to correctly sort sections and definitions by their appearance in the source code (using start lines).
Also improved the document logic to ensuring definitions are nested under their specific sections and not duplicated in parent sections, and that sections containing definitions are collapsible.


Refactored CodebaseProvider to sort sections/definitions by line and nesting.
