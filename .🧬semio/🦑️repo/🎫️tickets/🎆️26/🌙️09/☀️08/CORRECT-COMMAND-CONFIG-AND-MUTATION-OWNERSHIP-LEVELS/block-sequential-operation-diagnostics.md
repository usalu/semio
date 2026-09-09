# Block Sequential Operation Diagnostics

The execution agent's direct runtime harness completed one brush hover, then observed a second dispatched operation remain pending across thousands of complete maintenance cycles. That observation is not yet attributed to a scheduler defect. The shared `has_pending_typed_operations` predicate excludes retained cancellation/instance-operation cleanup, but maintenance continues to drive those owners while another operation exists, so the predicate alone does not explain a permanent second-operation stall.

The cancellation maintenance cursor advances on an empty slot because `cleanup_finished_slot` returns `Some(false)`; only lock contention returns `None`. The result-page method's integer argument is the receiving app instance, not an operation ID, so using receiver 1 in this bound fixture is correct.

A temporary `VcsArtifactApp::debug_typed_operation_state()` diagnostic exposes each operation's lifecycle stage, worker-session and publication ownership, result-page state, cancellation status, and shared retained-owner counts. It must be removed after the actual stalled owner is identified. The Block regression must preserve sequential hover, hover, leave, placement. Batching was diagnostic only. Leaving/canceling the brush must clear the addressed preview; placement does not acquire an invented automatic select-tool action.
