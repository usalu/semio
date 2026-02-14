# Ticket

## Todos
- [x] Reopen ticket for 5-row Selection subtool request.
- [x] Add three more Design Selection subtools (total five).
- [x] Keep Selection dropdown list one-column stacked rows.
- [x] Add subtool i18n labels.
- [x] Update README.md and AGENTS.md docs.

## Changes
- Updated `js/semio/sketchpad/Design.tsx` to register 5 Selection subtools in Design toolbar group:
  - `select`
  - `hand`
  - `additive`
  - `subtractive`
  - `intersect`
- Added three new toolbar sections for `additive`, `subtractive`, and `intersect` subtools with unique ids, icons, and cleanup removals.
- Updated `js/semio/sketchpad/locales/en.json` toolbar subtool labels for `hand`, `additive`, `subtractive`, and `intersect`.
- Updated docs in `README.md` and `AGENTS.md` to specify five one-column stacked Selection subtools.

## Log
- Reopened ticket with prompt: "add five selection subtools in selection dropdown as one column five rows".
- Implemented by Design toolbar group section expansion; dropdown list row count follows unique `subToolId` entries.

## Summary

Added five Design selection subtools so the Selection dropdown renders as a one-column five-row list (Select, Hand, Additive, Subtractive, Intersect), with updated labels and docs.
