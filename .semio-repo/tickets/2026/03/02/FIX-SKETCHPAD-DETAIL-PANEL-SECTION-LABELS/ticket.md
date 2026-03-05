---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Added missing locale labels for the design, kit, and type detail sections so the right panel shows readable headings instead of raw ids, and extended the existing sketchpad Playwright coverage to assert those headings. The targeted Design run confirmed the design and kit labels are visible but still failed later in an unrelated existing connection gap stepper assertion.
## Changes
- Added locale labels for `semio.sketchpad.app.design.properties` and `semio.sketchpad.app.kit.properties`.
- Extended the existing sketchpad Playwright coverage to assert the rendered section headers.
- Ran the targeted Design Playwright flow with the sketchpad dev server.
- Added locale labels for `semio.sketchpad.app.type.properties` and `semio.sketchpad.app.type.connector.properties`.
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
- Add `properties` labels under the existing `semio.sketchpad.app.design` and `semio.sketchpad.app.kit` locale trees.
- Assert those labels are visible in the design detail panel test, since that panel renders both sections.
- Validate with the targeted Playwright run.
