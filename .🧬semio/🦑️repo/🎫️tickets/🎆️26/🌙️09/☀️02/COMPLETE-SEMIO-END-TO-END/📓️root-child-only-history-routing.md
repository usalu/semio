# Child-Only Composite History Routing

## Verdict

RED. A child-only `Emit` is applied and stamped as one real composition group, but framework Undo/Redo discovers groups exclusively from the parent store's tail. Because a child-only group applies no parent operation, it leaves no parent tail group. The next Undo can therefore ignore the newest child command and undo an older parent edit or collapse to a no-op.

This report records a source-backed defect and the intended repair boundary. No runtime result is claimed yet.

## Evidence

- `dispatch_emit_group` passes an empty `parent_ops` vector for a child-only `Emit` and records the returned child edit ids in `CommandLogEntry.child_edit_ids`.
- `TransactionCoordinator::dispatch_relation_group` only creates/stamps a member edit for a non-empty operation slice.
- `commit_framework_history_route` currently selects a composition group from `self.store.tail_group_id()` for Undo and `self.store.redo_tail()` for Redo. It does not inspect live child tails or the command row that produced `child_edit_ids`.
- The first typed Flow `addWidget` command now emits exactly one `SemioFlowMutation::InsertNode` and no parent mutation, so it exercises this missing route directly.

## Required Semantics

History is one app-level order across parent-only and composite commands:

1. A successful parent-only document command contributes one parent route.
2. A successful child-only or mixed parent/child command contributes one group route carrying its shared invocation id and exact member edit ids.
3. Undo consumes the latest still-applied app route, irrespective of whether the parent participated.
4. Redo consumes the most recently undone app route, not the route with the greatest original edit timestamp or command sequence.
5. A successful new document command clears the app-level redo routes, matching each affected VCS store.
6. Foreign child tails not named by an app command row are never selected as local history.
7. Group execution remains delegated to `CompositionCoordinator::undo_group`/`redo_group`; the repair changes route selection, not member history behavior.

Parent-tail fallback remains necessary for loaded or ingested parent history that predates the runtime command log. There is no compatibility mode for child groups: a public persisted child-group history requires a future schema-backed app-history cursor rather than guessing from unrelated child tails.

## Test-Driven Packet

The language-neutral corpus must model a mixed sequence such as parent A, child-only B, mixed C, parent D and require Undo order D/C/B/A plus Redo order A/B/C/D. It must reject duplicate route ids, a group whose member edit set is empty, a group/member id substitution, and selection of a foreign child tail.

Native framework laws must use real parent and child stores and prove:

- child-only Undo restores the exact child snapshot while the parent snapshot and parent edit tail remain unchanged;
- Redo reapplies that child snapshot;
- a newer child-only group wins over an older parent edit;
- repeated Undo followed by repeated Redo preserves app command order across parent-only, child-only and mixed routes;
- a new edit after Undo clears the app-level redo route;
- a foreign child tail is skipped and cannot redirect local Undo.

The Flow exact law must prove its inserted `note_2`/`note_3` nodes undo and redo through the public framework history action while `FlowSnapshot.content` stays byte-identical.

## Nonclaims

- This repair does not make initial child publication atomic.
- It does not persist a cross-member history cursor across process restart.
- It does not convert the remaining Flow parent `FlowDiff` operations.
- It does not establish public member reopening or client execution of a trusted plugin.
