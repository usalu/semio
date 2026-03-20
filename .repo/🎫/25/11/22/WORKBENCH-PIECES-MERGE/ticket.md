# Ticket

## Todos
# Merge Design App Workbench Sections

Goal: Merge the design app workbench sections into a single pieces section with separate types and designs sub-items while keeping parent-child hierarchies intact.

Plan:

1. Review the existing workbench section setup in js/semio/sketchpad/apps/design/App.tsx to understand how types and designs are rendered and interacted with (hover, navigation, creation).
2. Refactor the workbench registration to expose one pieces section containing both trees, ensuring the tree builders preserve parent-child relationships and the section actions reflect the new structure.
3. Update any related identifiers or translations necessary for the new pieces section and verify the resulting hierarchy and interactions align with the requirement.

Implementation:

- Merged the workbench into a single pieces section that nests the type and design trees, keeps drag/hover and creation flows, and reuses the parent-child recursion for both hierarchies.
- Registered the new pieces section alongside the window library and removed the separate type/design workbench entries.
- Added en/de translations for the pieces label so the new section resolves through i18n.

## Changes

## Log

## Summary
# Summary

Merge design app workbench sections into single pieces section
