# Inspector List Action Audit

Read-only source evidence. No test, browser or build launched.

Shared base: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/`.

## Rendering Is Not Capability-Gated

`🌲️outliner/🦀️.rs` render_collection appends Add item whenever its lazy section is open. Its arguments contain only path and index=items.len(). entity_row unconditionally appends Remove with row.path and index0. Reorder separately checks state.reorderable; this does not gate Add/Remove.

`🪟️visual-windows/🧾️properties/🦀️.rs` scalar_list does the same for non-entity arrays: every row has Remove, and an open section has Add. Neither builder asks EDIT_RULES, ID_REMOVALS or a factory/default. Empty arrays are classified as entity collections because is_entity_collection uses all(Object), vacuously true.

Object maps are rendered as entries_section, not as collections. Thus root geometry/curves maps do not receive map-entry Add/Remove from this generic inspector. Their contained arrays still receive the generic list controls when opened.

## Add Has No Typed Row Factory

`🏭️command-and-tool-job-macros/🦀️.rs` maps insertItem missing value to InsertItem.value_json=None. `⌨️command-args-and-dispatch/🦀️.rs` dispatch_insert_item converts absent JSON to value=None and calls handle_insert_item. The latter forwards NormEdit::InsertItem directly.

Both ISO and VDI concrete `✏️editor/🎮️commands/➕insert-item/🦀️.rs` handlers pass that payload directly to shared dispatch and native resolve_edit. No factory/default injection occurs there.

`✏️edit-rules/🦀️.rs` NormEditRules::payload first requires an exact insert rule, then requires value Some or returns insertItem '<path>' needs a value. Therefore even supported catalogue Add buttons lack their required typed row. Opening VDI catalog.products or ISO catalogue.products and pressing the authored Add action yields that refusal by source flow. Unsupported paths instead refuse no kind inserts into '<template>'.

This is separate from commands/tools supplying an explicit typed value, which can legitimately resolve through an insert rule.

## Representative Domain Coverage

Domain base: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<family>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`.

| Family/path | Insert route | Remove route | Generic inspector consequence |
| --- | --- | --- | --- |
| ISO catalogue.products | introduce-product, requires product payload | retire-product via ID_REMOVALS | Remove supported; Add shown without required payload |
| ISO dictionary.subjects | introduce-subject, requires subject | retire-subject via ID_REMOVALS | Same distinction |
| ISO catalogue.productClasses/propertyDefinitions/productSeries/productIndexes/productGroups | corresponding introduce kind | corresponding retire kind via ID_REMOVALS | Same distinction |
| ISO selection.constraints | add-selection-constraint, requires constraint | indexed remove-selection-constraint | Remove supported; Add lacks payload |
| VDI catalog.products | add-product, requires product | remove-product via ID_REMOVALS | Remove supported for canonical article/id identity; Add lacks payload |
| VDI catalog.products[].records | no insert rule | no removal rule | Generic Add/Remove actions refuse |
| VDI catalog.products[].configuration.functionRefs | no insert rule | no removal rule | Generic list actions refuse despite enclosing configuration set rule |
| VDI geometry.<key>.connections | no inspector insert rule | no inspector removal rule | Generic array Add/Remove refuse; geometry map itself has no map-entry controls |
| VDI curves.<key>.points | no inspector insert rule | no inspector removal rule | Same unsupported nested-array controls |

VDI has native add/remove-geometry-connection mutations, but the inspected EDIT_RULES roster does not map generic inspector list paths to them. Their mere existence does not establish this UI route.

ISO geometry is also an object/catalogue hierarchy: inspect each array's precise rule rather than infer capability from the presence of a geometry map. The inspected ISO insert roster is exactly the eight collection paths in the table; remove roster is selection.constraints plus the seven ID_REMOVALS paths. Other rendered arrays have no generic insert/remove mapping.

## Capability Boundary

Scalar setter admission is insufficient to decide list operations. catalogue.products has no whole-list setter but has legitimate domain insertion/removal rules. Conversely configuration.functionRefs has an enclosing configuration setter after descendant support, but current InsertItem/RemoveItem routing still requires its own structural rules.

Any later capability interface must distinguish insert, remove and move independently, include ID_REMOVALS, and distinguish a supported insert command from availability of a concrete typed default/factory for an argument-free Add button. This audit proposes no production edit.
