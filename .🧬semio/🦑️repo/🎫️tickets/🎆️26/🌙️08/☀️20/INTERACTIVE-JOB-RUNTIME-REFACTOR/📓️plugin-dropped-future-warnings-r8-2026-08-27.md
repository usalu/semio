# Plugin R8 Dropped-Future Warnings

Read-only scope: the five requested main-source warnings in the actual R8 compiler log. No warning suppression, await insertion, production macro change, or test execution was performed in this packet. Mutation owns its separate surface fixture line195 await join.

## Checkpoint/Restore Fixture

The main-source restartable-task test calls `test_support::run_until_idle(8)` and `cancel_instance_registry_requests(instance)` without awaiting either. Both functions are async. The first only executes the real test executor when polled; the second only enters its request-registry cancellation cursor when polled. Their returned futures capture scalar inputs and are discarded immediately, so the warning is not itself proof of a dropped large buffer. The important ownership defect is skipped work: the task is not parked by the claimed pre-checkpoint step, and the old instance's pending request registry is not explicitly drained by the claimed cleanup step.

The following `task_count_for_instance(...).await` checks task records, not exact request-registry emptiness, so it cannot prove the skipped request cleanup occurred. The necessary fixture repair is to await those original operations and assert their actual expected state/results. The cancellation test helper itself loops its cursor to completion; it is cold test harness work, not a bounded production close proof. No production close API or exact-key authority should be inferred from merely adding the await.

## Derived Subset Registration Fixture and Macro Expansion

The two direct `register_subset()` calls in `subset_macro_derived_register_is_idempotent` return futures that are never polled. Consequently the async `register` body's Once-guarded registry mutation does not execute. The later awaited direct validator call tests the validator implementation, not that it was registered once or is discoverable through the registry.

The warning attributed to the `subset!` invocation comes from its generated `conformance::subset_macro_derived_validator_registers`: both `register()` and `<Validator as SubsetValidator>::validate(&payload)` are unawaited there. The ignored validator future only borrows the local empty Text payload; its absence of execution, not a completed validator Result, is what must be repaired. An ignored Future is not a successful registration or validation receipt. The production `register` implementation already awaits its effects indirectly through the existing synchronous Once bridge when its future is actually polled; no new bridge or silent `let _` suppression is warranted.

The intended scoped follow-up is a real registration/conformance regression that first observes the absent registry entry, awaits the actual registration, looks up that exact validator through the registry, and awaits its result. Awaiting twice must preserve the real idempotent behavior. That test should expose the current omission before the macro and direct fixture call sites are corrected. These are queued tests/repairs, not new pass claims.

Source evidence: R8 log entries at main35920/35928 and37505/37513/37514; `reactor::test_support` async helpers around2407/2458; main derived `subset!` macro `conformance` body around37475. Line numbers are from the captured R8 source and may move. The raw R8 log is retained alongside this report.
