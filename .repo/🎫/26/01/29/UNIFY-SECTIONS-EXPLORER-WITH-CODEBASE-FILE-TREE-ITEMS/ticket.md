# Ticket

## Todos

## Changes

## Log

## Summary

Unified the Sections explorer view with the Codebase file tree items. The Sections view now shows both sections AND definitions (just like expanding a file in the Codebase sidebar), sorted by line number. Key changes:

1. Added `SectionDefinitionItem` class with symbol-function icon, matching `CodebaseDefinitionItem` behavior
2. Refactored `SectionsProvider` to fetch both sections and definitions for the active file
3. Root level shows sections and file-level definitions (those not inside any section), sorted by line
4. Expanding a section shows child sections and direct definitions (excluding those in child sections), sorted by line
5. Collapsible state now based on whether a section has child sections OR definitions (matching Codebase logic)
6. Updated `SectionsDragAndDropController` to work with the new `SectionTreeItem` union type
7. Icons match Codebase: symbol-namespace for sections, symbol-function for definitions
8. Context menus correctly scoped: rename/create-child/remove only on sections (viewItem == section), not definitions
