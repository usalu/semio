---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Implemented connection details panel tree structure: converted Gap/Shift/Rise from Stepper to Slider, renamed Connector to Port labels, renamed U/V to X Offset/Y Offset in Diagram, updated both en/de translations, updated e2e test assertions. 0 TS errors, 13/13 unit tests pass.

## Changes

- `compose/js/sketchpad/Design.tsx`: SingleConnectionInfo renamed connectingConnectorId → connectingPortId, connectedConnectorId → connectedPortId; SingleConnectionFields converted gap/shift/rise from Stepper to Slider with labels; renamed u/v steppers to x/y; same changes in ConnectionsSectionForm bulk editing section
- `compose/js/sketchpad/locales/en.json`: connectingConnectorId → connectingPortId ("Connecting Port"), connectedConnectorId → connectedPortId ("Connected Port"), u → "X Offset", v → "Y Offset"
- `compose/js/sketchpad/locales/de.json`: connectingConnectorId → connectingPortId ("Verbindender Anschluss"), connectedConnectorId → connectedPortId ("Verbundener Anschluss"), u → "X-Versatz", v → "Y-Versatz"
- `compose/js/sketchpad.test.ts`: Updated e2e test assertions from connectingConnectorId → connectingPortId, connectedConnectorId → connectedPortId, u → x, v → y

## Log

- Read existing SingleConnectionInfo and SingleConnectionFields components
- Read en.json and de.json for current i18n labels
- Applied Gap/Shift/Rise Stepper→Slider conversion with label+div wrapper pattern matching orientation sliders
- Renamed connector→port in connecting/connected sections
- Renamed u/v→x/y in diagram section
- Applied same changes to bulk editing section (ConnectionsSectionForm)
- Updated en.json and de.json translations
- Updated e2e test assertions
- Verified 0 TypeScript errors, 13/13 unit tests pass

## Todos

- [x] Read existing connection details code
- [x] Convert Gap/Shift/Rise from Stepper to Slider
- [x] Rename Connector to Port in i18n labels
- [x] Rename U/V to X Offset/Y Offset
- [x] Apply changes to bulk editing section
- [x] Update en.json translations
- [x] Update de.json translations
- [x] Update e2e test assertions
- [x] Verify TypeScript compilation (0 errors)
- [x] Verify unit tests pass (13/13)

## Plan

1. Convert Gap/Shift/Rise controls from Stepper to Slider with label pattern matching orientation sliders
2. Rename "Connector" to "Port" in connecting/connected sections
3. Rename "U"/"V" to "X Offset"/"Y Offset" in diagram section
4. Apply same changes to bulk editing section
5. Update i18n translations in both languages
6. Update test assertions
7. Verify builds and tests pass
