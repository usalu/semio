---
goal: R26-02/UPDATED-SKETCHPAD
---

# Ticket

## Summary

Unified toolbar button/toggle spacing by: (1) changing ToggleGroupItem from gap-0+ml-single to gap-single for icon-text gap consistency with ButtonGroupItem, (2) upgrading horizontal padding from p-single to py-single px-double for both ToggleGroupItem and ButtonGroupItem when text is present, giving buttons enough breathing room. No new TS errors introduced.
## Changes

- `semio/js/sketchpad/elements.tsx`: Unified spacing mechanism between ToggleGroupItem and ButtonGroupItem

## Log

- Analyzed ToggleGroupItem: uses `gap-0` + `ml-single` on text span
- Analyzed ButtonGroupItem: uses `gap-single` + no margin on text span
- Both produce ~0.2rem gap but via different mechanisms
- Buttons with text use only `p-single` (0.2rem) horizontal padding making them too narrow
- Fix: unify both to use `gap-single` on parent, add `px-double` for items with text

## Todos

- [x] Analyze root cause
- [ ] Fix ToggleGroupItem: change `gap-0` to `gap-single`, remove `ml-single` from text span
- [ ] Fix ButtonGroupItem: ensure consistent padding for text items
- [ ] Unify both base variants to use same padding strategy for text items
- [ ] Run tests

## Plan

Root cause: `ToggleGroupItem` and `ButtonGroupItem` use different spacing mechanisms for icon+text:
- ToggleGroupItem: `gap-0` on parent + `ml-single` on text span
- ButtonGroupItem: `gap-single` on parent + no margin on text span
Both items use `p-single` (0.2rem) horizontal padding when text is present, making buttons too narrow.

Fix:
1. Unify ToggleGroupItem to use `gap-single` like ButtonGroupItem
2. Remove `ml-single` from ToggleGroupItem text span
3. Use `px-double` horizontal padding for items with text (both ToggleGroupItem and ButtonGroupItem)
