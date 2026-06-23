---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Bulk close
## Changes
- Added locale labels for `compose.sketchpad.app.design.properties` and `compose.sketchpad.app.kit.properties`.
- Extended the existing sketchpad Playwright coverage to assert the rendered section headers.
- Ran the targeted Design Playwright flow with the sketchpad dev server.
- Added locale labels for `compose.sketchpad.app.type.properties` and `compose.sketchpad.app.type.connector.properties`.
- Extended the existing sketchpad Playwright coverage to assert the Type detail panel section headers.

## Log
- Located the issue in `PanelSectionWrapper`, which falls back to the raw section id when a translation key is missing.
- Confirmed the design and kit detail sections register ids without matching locale labels.
- Verified in the Design Playwright run that the detail panel now renders `Design Properties` and `Kit Properties`.
- The full `Design` test still fails later in an unrelated existing check for the connection `gap` stepper group.
- Confirmed the Type app uses the same fallback for the type and connector detail section ids.

## Todos
- Update locale entries in `en.json` and `de.json`.
- Update the existing sketchpad test to verify the section headers.
- Run the relevant sketchpad Playwright coverage.
- Note the unrelated pre-existing test failure in the ticket summary.
- Run the targeted Type Playwright coverage.

## Plan
- Add `properties` labels under the existing `compose.sketchpad.app.design` and `compose.sketchpad.app.kit` locale trees.
- Assert those labels are visible in the design detail panel test, since that panel renders both sections.
- Validate with the targeted Playwright run.
