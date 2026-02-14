# Ticket

## Todos
- [x] Register five placeholder sub-tools in Kit Selection toolbar dropdown.
- [x] Keep deterministic toolbar ordering and cleanup removals for all new sub-tool sections.
- [x] Update developer documentation in `README.md` and `AGENTS.md`.

## Changes
- Added `kitToolbarSelectionSubTools` in `js/semio/sketchpad/Kit.tsx` with five Selection sub-tool entries: `select`, `hand`, `additive`, `subtractive`, `intersect`.
- Replaced single Selection toolbar registration with iterative registrations across the five sub-tool section ids.
- Updated toolbar cleanup to remove all five Selection sub-tool section ids.
- Documented Kit Selection dropdown placeholder sub-tools in `README.md` (`# 📦 Bundles`, Sketchpad toolbar tooltree).
- Documented requirement and implementation references in `AGENTS.md` (`# Software Requirements Specification` and `# Codebase`).

## Log
- Opened ticket `KIT-SELECTION-PLACEHOLDER-SUBTOOLS`.
- Created `plan.md` with implementation steps.
- Implemented Kit Selection dropdown placeholder sub-tool registration.
- Updated root developer docs.
- Verified patch diff for `Kit.tsx`, `README.md`, and `AGENTS.md`.

## Summary

Implemented five placeholder Kit Selection dropdown sub-tools and documented the behavior in README and AGENTS.
