# Ticket

## Todos

## Changes

## Log

## Summary

Fixed three issues with tree item click navigation in the VS Code extension:

1. **Codebase tree section items** (`navigateToSection`): Previously only opened the file without jumping to the line. Now passes `section.range.start` (line number) as argument and navigates to the correct position.

2. **Codebase tree definition items** (`navigateToDefinition`): Same issue - only opened file. Also had a regex mismatch (`§` vs `#` separator). Now passes `definition.range.start` as argument and navigates correctly.

3. **Section tree items** (`sectionOpen`): Added `revealRange` with `InCenter` reveal type to ensure the editor scrolls to show the target line.

All three navigation commands now open the file, set the cursor position, and scroll to center the target line in the editor.
