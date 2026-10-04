# Details Collection Tree Shape Repair

## Problem

The Stdio Details producer placed a windowed `treeSection` below a `treeItem`. The retained React interpreter only admits `treeSection` below the tree root and discovers nested rows from direct `treeItem` children. Collection entries therefore existed in the native node census but were absent from the mounted Details DOM.

## Contract

- A collection is one windowed `treeItem`.
- Its materialized entries are direct `treeItem` descendants.
- Collection controls remain ordinary direct controls and do not count as logical rows.
- Host-provided closed state materializes no entry rows; host-provided open state materializes only the requested window.
- Object entries retain their open state so scalar fields remain reachable.
- Existing scalar inputs keep their semantic draft target and `setSnapshotValue` commit binding.
- The retained `BuiltChildren` carrier is not serialized. It intentionally requires retained page transport. The cross-language fixture is an exact semantic projection asserted from the native tree.

## Implementation

- `detail_node` now calls the shared `tree_window_indexed_item` helper for collections.
- The neutral `collection-tree-shape` fixture carries the source snapshot, schema, expected values, exact native keys, open state, window extent, entry hierarchy, accessible input labels, values, action, and draft-target witness.
- The Rust law renders both explicit closed and open host requests. It checks bounded materialization, direct descendant shape, controls, edit inputs, draft targets, actions, and exact fixture projection.
- The React law constructs a retained document from the Rust-verified projection and mounts it through `UiDocumentStore` and `UiNodeView`. It verifies real tree roles/open semantics, both editable existing values, blur commit intent, fold removal, and reopen restoration.
- The renderer suite and external fixture are registered in the renderer test configuration and Nx inputs. Focused native and renderer commands are registered in both launch seed and generated launch configuration.

## Validation

- `🗑️generated/details-tree-descendants-native-6.log`: final focused native shape and independent JSON projection law passed 1/1 with 92 skipped.
- `🗑️generated/details-tree-first-paint-native-1.log`: the existing repeated first-paint and bounded-retirement law passed 1/1 with 92 skipped after its folded collection was paired with an explicit closed host request.
- `🗑️generated/details-tree-contract-native-full-2.log`: the complete contract crate passed 93/93.
- `🗑️generated/details-tree-renderer-dom-7.log`: the final fixture mounted through the real retained renderer; 1 file and 1 test passed. The DOM exposes expandable collection/object rows, commits an edited existing value, folds, and reopens.
- `🗑️generated/details-tree-renderer-typecheck-3.log`: aggregate renderer TypeScript typecheck passed after the retained input-outcome boundary changes.
- `🗑️generated/details-tree-contract-native-full-1.log`: the first full gate exposed the older unhosted first-paint assumption. It ran 37/93 before fail-fast: 36 passed and one Details law failed. The explicit closed-window request above replaced that implicit assumption, and the final full gate passed.

## Scope

No renderer interpreter, input-draft lane, TIFF, PNG, BMP, Office, or OPC production code was changed in this repair.
