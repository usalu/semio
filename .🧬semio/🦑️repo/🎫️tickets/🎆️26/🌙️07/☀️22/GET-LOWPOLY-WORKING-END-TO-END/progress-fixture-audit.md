# Lowpoly Progress Fixture Audit

Read-only source audit, 2026-11-04. No builds or browser checks.

## Fixture

Create ArtifactOperationProgress with operation_id:0x12, generation:0x34, label:LocalizedLabel::native, completed_units:7, cancelling:false. Bind borrowed slice with ArtifactView::new(&snapshot,&history).with_operations(&operations), render Model and UV through ArtifactEditor::render, then project_and_retire_fixture_tree. Repeat cancelling:true and an empty slice. Empty models terminal presentation removal, not runtime cancellation.

Assert exact keys framework.operation.0000000000000012.status/progress/cancel; cancel binding trigger activate, action scope Lowpoly controller, name cancelTypedOperation, version 1, args operationId 0000000000000012 and generation 0000000000000034. Cancelling disables button. Test both explicit locales. Partial scope must list semantic Model/UV window body IDs only.

## Authorities

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🔭️artifact-view/🦀️.rs:140 operations; :144 with_operations.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🛬️lifted/👁️view/🦀️.rs:7 ArtifactOperationProgress fields.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏳️operation-progress/🦀️.rs:66 controls; :95 cancelling disable; :98 exact identity; :114 framework.operations root.
- Same operation-progress/🧪️tests/🦀️.rs covers neutral identity/targets/captions and exact dirty scope; no rendered fixture found in narrow owner/framework/peer search.
- ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️app/🦀️.rs:93-94 Partial scope; :393-398 wrapper composition precedent.
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🧪️tests/🔬️unit/🦀️.rs:55 Lowpoly tree projection precedent.

## JSON Oracle

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🌳️fixture-tree-projection-and-scenes/🦀️.rs:44 emits {key,component,bindings,disabled,accessibility,children} at each node. Recursively locate by top-level key through children; no root wrapper. Disabled is top-level.

🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:17 and :68 define action/binding fields. 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🦀️.rs:87 serializes UiValue Map/Text as plain JSON objects/strings. Independent serde_json comparisons can assert whole args/action objects.

Actual scope API is operation_progress_scope() -> UiDirtyScope, not OperationProgressScope. Runtime terminal filtering and authority remain cancellation-audit.md evidence.
