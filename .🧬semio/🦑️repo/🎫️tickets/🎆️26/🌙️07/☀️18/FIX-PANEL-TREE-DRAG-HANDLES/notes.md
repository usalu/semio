# Fix Panel Tree Drag Handles

## Root cause
`PanelTreeUnitsPane` showed a unit header whenever `dock && anchor` (PanelDockProvider), even for unlabeled `singleTreeLeaf` units — producing a lonely top-right grip with no section/item handles.

## Fix
1. Unit header only when `unit.label || unit.icon || units.length > 1`.
2. `TreeSection` renders `DragHandle` when reorderable; handle-only initiation.
3. `Tree` enables `sortableSections` by default for 2+ sections; native MIME reorder + `mergeTreeSectionOrder` so plugin re-renders keep user order.
4. Sortable `TreeItem` always gets a drag handle.
5. Declarative / panel trees pass `sortableSections`.

## Tests
`bun ./📜️script.ts test --testNamePattern="PanelTreeUnitsPane|Tree section reorder|mergeTreeSectionOrder|sortable TreeItem always|Tree's controlled|Panel wires bottom|renders sortable drag"` — 8 passed.
