# Follow-up: remove 3-level Fill nesting

## Cause
`buildToolTree` emitted two empty-label sections (activate + options). With `sections.length > 1`, `PanelTreeUnitsPane` enables `sortableSections`, which adds drag handles and disables TreeSection's headerless pass-through — empty sections rendered as folders. The activate row also used `label: "Fill"` + Toggle control, which TreeItem renders as a nested property/folder row.

## Fix
1. Single headerless section; `sortableSections: false`.
2. When active: only measure controls (count, distributions, edit-volumes toggle) — no nested Fill row.
3. Selecting the `tool.<id>` footer tab activates that tool so measures show immediately.
4. Inactive tool tab still shows a flat unlabeled Toggle to activate.
