# Ticket

## Todos

- [x] Update section tree item icons
- [x] Update definition tree item icons
- [x] Update ticket documentation

## Changes

### Updated Tree Item Icons for Better Visual Distinction

**File:** `js/vscode/extension.ts`

1. **Section Item Icons** - Changed to `symbol-folder`:
   - `ContributorSectionItem` (line 2506): Changed from `symbol-class` to `symbol-folder`
   - `SectionItem` (line 2748): Changed from `symbol-namespace` to `symbol-folder`

2. **Definition Item Icons** - Changed to `symbol-variable`:
   - `ContributorDefinitionItem` (line 2515): Changed from `symbol-method` to `symbol-variable`
   - `SectionDefinitionItem` (line 2760): Changed from `symbol-function` to `symbol-variable`

### New Icon Mapping

| Tree Item Type  | Old Icon                            | New Icon          | Visual Distinction         |
| --------------- | ----------------------------------- | ----------------- | -------------------------- |
| **Sections**    | `symbol-class` / `symbol-namespace` | `symbol-folder`   | Folder-like container icon |
| **Definitions** | `symbol-method` / `symbol-function` | `symbol-variable` | Variable/definition icon   |

### Rationale

- **Sections** now use `symbol-folder` to represent them as containers that hold definitions and child sections
- **Definitions** now use `symbol-variable` to clearly distinguish them as individual code elements within sections
- The new icons provide better visual document and make it easier to distinguish between structural sections and actual code definitions

## Log

2025-01-29: Updated section and definition tree item icons to be more visually distinct. Sections now use folder icons while definitions use variable icons.

## Summary

Successfully updated tree item icons to provide better visual distinction between sections and definitions. Section items now use symbol-folder icons to represent their container nature, while definition items use symbol-variable icons to clearly identify them as individual code elements. This improves the visual document and makes the tree view easier to navigate.
