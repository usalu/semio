---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Migrated old build detail panel UI sizing into new build. Added showLabel to piece Type and Variant Comboboxes. Added textarea and combobox button sizing overrides to rightSidePanelElementSizingClassName for consistent compact panel layout.
## Changes
- Design.tsx: Added `showLabel` to Type Combobox (id=semio.sketchpad.app.design.piece.type)
- Design.tsx: Added `showLabel` to Variant Combobox (id=semio.sketchpad.app.type.variant)
- Sketchpad.tsx: Added textarea sizing overrides to rightSidePanelElementSizingClassName
- Sketchpad.tsx: Added combobox button (role=combobox) sizing overrides to rightSidePanelElementSizingClassName

## Log
- Systematically compared Design.Details.tsx.old (1352 lines) against new Design.tsx details section (lines 4149-6200)
- Reviewed rightSidePanelElementSizingClassName overrides (52+ rules covering side-panel, tree, property, input, stepper, slider, button)
- Reviewed elements.tsx Label, Stepper (always wraps Label), Slider (conditional showLabel), Combobox (conditional showLabel), Textarea, Button
- Found Stepper always wraps in Label (no gap)
- Found Slider already uses showLabel (no gap)
- Found Input already uses showLabel (no gap)
- Found Type/Variant Comboboxes missing showLabel (gap)
- Found textarea and combobox button missing sizing overrides (gap)
- Verified tsc --noEmit: only pre-existing FC type compat errors, no new errors
- Verified dev server compiles cleanly

## Todos
- [x] Read old file and new details code
- [x] Read rightSidePanelElementSizingClassName overrides
- [x] Read elements.tsx key components (Label, Stepper, Slider, Combobox, Textarea)
- [x] Identify concrete gaps between old and new builds
- [x] Implement missing showLabel on Type/Variant Comboboxes
- [x] Add textarea/combobox sizing overrides
- [x] Verify compilation
- [x] Verify dev server

## Plan
1. Deep comparison of old Design.Details.tsx.old vs new Design.tsx details section
2. Check all component implementations for showLabel/label support
3. Add missing showLabel props where old build had labels
4. Add missing sizing overrides to rightSidePanelElementSizingClassName
5. Verify compilation and dev server
