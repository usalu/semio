# Retained Row Action Admission

Root read-only review after the generic ButtonView repair found an uncovered sibling path in the Interpreter. TableRowActionButton accepts `onRun: () => void` and dispatchRowAction discards the onIntent completion promise. A repeated row action therefore has no per-action pending admission or busy status. The tree-row action mapper calls the same dispatcher. Unlike ButtonView, dispatchRowAction also constructs its binding from the rendered record/target/action rather than rereading the current store record at activation.

The existing 97 editable-control laws and preview-35 Add-row witness prove the generic button path only. They do not establish repeated Remove/duplicate row-action safety in tables or trees. Required follow-up: one target/action/owner-scoped pending admission path, current-record validation at dispatch, exact owner retirement, observable busy/disabled accessibility, and mounted tests for completion, refusal, rejection, replacement, and a store publication before React commit. The target binding must be rebuilt from the current row action and target, not copied from a stale callback.

This audit is not a claim that table or tree row action admission is fixed. No code was changed in this read-only cut.

## Implemented Admission Contract — 2026-10-04

Table and Tree row actions now share one weak registry for the exact local document owner and `UiDocumentStore`. Its pending identity is the canonical, object-key-order-independent encoding of the complete current `RowTarget` and `RowAction`. A pending action disables only that identity; another target or action under the same document owner remains available. Owner retirement clears the registry synchronously, and replacing either the owner or Store creates a separate lifetime.

Activation rereads the row by node id and authored key from the current Store, then rereads the action at its authored index and rebuilds the binding from the current target. A publication that lands after the browser event closure was rendered but before React commits therefore dispatches the new target/action. A removed row, replaced authored key, disabled action, retired owner, or duplicate pending action dispatches nothing.

Both row projections expose the same admission state. Table buttons and Tree row buttons become natively disabled and publish `aria-busy=true` while their exact promise is pending. Applied, refused, and rejected settlements all release the admission. Existing authored-disabled Table actions remain focusable through `aria-disabled`, preserving the row-action accessibility contract.

The language-neutral fixture and strict 2020-12 schema are `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/{🧫️fixtures,🧬️schema}/🎬️row-action-admission/🔣️.json`. Ajv is the independent schema oracle; Testing Library drives the actual Table and Tree buttons.

## Validation Receipts

- `🗑️generated/row-action-admission-red-1.log`: schema oracle passed; all five mounted behavioral witnesses failed against the old void/stale dispatch path.
- `🗑️generated/row-action-admission-green-3.log`: registered zero-touch row-action gate passed 6/6, covering duplicate admission, Table/Tree busy accessibility, applied/refused/rejected recovery, publication before React commit, and exact target/Store/owner replacement.
- `🗑️generated/row-action-interpreter-regression-1.log`: 187/188 passed and exposed one regression in authored-disabled Table focusability; this run is superseded.
- `🗑️generated/row-action-interpreter-regression-2.log`: the complete Interpreter inline suite passed 188/188 after restoring focusability.
- `🗑️generated/row-action-admission-typecheck-2.log`: aggregate React renderer typecheck passed against the final source.

The focused executable is registered as `@semio-tech/framework-renderer-react:row-action-admission-check` in the project, task router, launch seed, and generated launch file.

## Authority follow-up — 2026-10-04

A Store publication may disable a row or replace/reorder its action slot before React commits the new DOM. Dispatch now requires the current record to remain enabled and to carry the exact authored target plus verb at the same slot. The pending key intentionally excludes icon, label, placement, disabled state, and reason so cosmetic publication does not create a second command identity.

Fresh receipts:

- `🗑️generated/row-action-admission-green-4.log`: 8/8 mounted DOM/schema laws green, including disabled current rows, replaced/reordered slots, and cosmetic publication.
- `🗑️generated/row-action-admission-typecheck-3.log`: aggregate renderer TypeScript check green.
