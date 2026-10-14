# Settlement Guard Binding Review

Read-only plan/current-source review, 2026-10-13. Lower scope Native83624 genuine5/5 reported by parent, not executed here. Scope Source5165 live. At inspection TypedFileSettlement still stored RefCell<&mut Control>; binding not yet landed.

## Concrete Dereference Correction

With proposed `RefCell<SqliteSnapshotReceivingControl<'a,'b>>`, a mutable borrow is `RefMut<Guard>`. First dereference produces Guard; second uses Guard::DerefMut and produces actual SqliteSnapshotControl. Existing `let control=&mut **borrowed` remains correct. A literal triple dereference attempts to dereference Control itself, which has no Deref implementation, and will not compile. This applies to four IO binders, History validator macro and new Kernel slack-law SQL borrow. Normal typed function-argument Deref coercion should handle `&mut self.control.borrow_mut()` in settle.

## Lifetime And Reentrancy

Constructor reads original remaining, then stores original `control.receiving_scope()` loan. Parent requirement remains true until settlement drops after scoped native receive, outcome handling and residual refused settlement. A nested History settlement borrows the same actual Control with previous=true; inner Drop restores true, outer Drop eventually restores original mode. Original cumulative ledger is still shared and no fresh controller/allocation is introduced.

Body binders borrow settlement RefCell while performing producer work. Body observer returns true while post_body=false and must not borrow that RefCell. Binder drops Port/RefMut before finish_body; finish_body settles, drops its temporary borrow, then sets post_body=true. Subsequent owner close/publication observer can safely settle and checkpoint. Nested History post-body events use its inner settlement while the outer settlement remains body-mode; outer observer does not recursively borrow. After inner receive returns and inner guard drops, outer body still holds its original RefMut and required-mode loan coherently.

Error shortcuts before finish_body drop lexical body borrow during unwinding; outer refused then settles through remaining guard. A finish_body settlement failure leaves post_body=false and original frame retained. Do not move guard out or drop settlement before scoped owner returns. No other concrete lifetime/reentrant defect found in existing sequence.

Kernel negative Probe now records denied literal pointer before returning error and compares after owner promotion, both residual settlement calls and funded cleanup. Earlier missing-pointer gap is resolved in source. No new runtime qualification claimed.
