# Ungroup Editing

Added Ungroup to the selection command schema, action form and English/German inspector. The operation promotes immediate children into their group's original sibling position, composes each child transform with its former group transform, carries hidden state to promoted children, preserves child identity/properties/locks, and removes the empty container. It selects the released children. Multiple selected groups can belong to different parents; nested selected groups are processed deepest first. The complete mutation set is planned before publication.

Rust plans the existing transform, visibility, reorder and delete mutations. The TypeScript implementation independently produces the resulting tree and selection. Eleven shared fixtures cover reflection/shear, nested selection, hidden state, locked children/ancestors, empty groups, separate parents, mixed/absent targets and numeric overflow. Three.js checks every leaf's world transform at three noncollinear points before and after ungrouping. Inputs stay unchanged on both success and refusal.

Native tests exercise real mutation application, localized inspector controls, promoted-child selection and one undo/redo history entry. They are authored but unverified while the native run remains pending.

## Compositing Finding

`flatten_drawing_document_with_transformation` currently traverses groups without propagating group opacity or blend mode. SVG export consumes these flattened scene nodes, so it shares this limitation. Correct isolated group compositing requires scene/renderer/export work; multiplying child opacity is not equivalent for overlapping children.

Ungroup therefore refuses selected groups with non-unit opacity or non-normal blend mode. This avoids claiming those settings can be preserved by a tree-only edit. Supporting the complete intended group behavior remains part of the active goal. Group attributes do not currently inherit into child paint. Ungroup retains each child's authored paint unchanged.

Planning still clones the document and walks it synchronously. Resumable, cancellable large-document structural editing remains unfinished. No browser behavior has been verified for this addition.

## Evidence

- `tests-ungroup-red.txt`: missing implementation reproduced, exit 1.
- `tests-ungroup-current.txt`: **190 tests / 19,230 assertions / 26 files passed**, exit 0, plus the 48 field-patch cases and 36-command publication audit. This included the initial eight fixtures.
- Final extended run 72547 (`tests-ungroup-final.txt`) passed **190 tests / 19,265 assertions / 26 files**, exit 0, including all eleven ungroup fixtures and both audits.
- Native 88159 (`tests-native-svg-gradients.txt`) and component 93519 (`build-draw-inspector-rows.txt`) remain live. Their eventual source coverage must be confirmed; they began before these edits.

## Changed Sources

Relative to the Draw any-subset editor:

- `🎮️commands/🎛️edit-selection/🧩️ungroup/🦀️.rs`, `🟦️.ts`, and `🧫️fixtures/🔣️.json`
- Selection command schema, handler and Rust/TypeScript tests
- Editor terminology, action form options, inspector and inspector tests
- Registered editor history/selection test

The ticket and full editing goal remain active and incomplete.
