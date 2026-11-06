# Draw Guest Lifecycle Review

Root independently read the current own neutral scenario schema/fixture, Ajv oracle and native law, plus actual Wasmtime dispatch, actor exports and lifecycle ownership sources. This is a source review; no compiler or Wasm execution pass is inferred.

## Current Contracts

Wasmtime `execute_turn` accepts mutable events. Its actual `call_poll` errors map through `failure_from_wit` into `TurnFault::ActorCall` with original input/spent/work. Legacy `TurnFault::Guest` can arise from command/cold-pair staging before poll, without an actual raw retained receipt. Neither staging page applies to an InstanceOpen lifecycle event. The framework retry helper recognizes only that old Guest shape. A missing receipt must fail rather than mint a replacement or retry renewed authority.

Actor `close_wit` checks held outputs/jobs and closes its OriginalActorEntry; it does not dispatch app InstanceClose. Shared lifetime ownership retains the app owner through explicit close and receipt acknowledgements. Draw has no own InstanceClose event arm; the shared reactor implements it. The patched verifier now captures the actual opening lifetime, sends close request2 for that same lifetime, preserves actual close generation, acknowledges receipts and requires Retired/idle before actor-context close. This is structurally appropriate and still needs real execution.

## Findings Sent To Execution

- After posting raw actual spent/work, receipt validation must enforce the actual Budget.work_fuel and retained_policy. Original input conservation alone allows spending the separately withheld native parent-retirement floor.
- Every returned ActorCall raw receipt must be posted before semantic failure, including non-retry failures. The legacy Guest retry guard cannot produce an ActorCall receipt; the old nested branch was unreachable.
- Consumed event vectors and actual TurnResult owners require controlled paid retirement in the same initiating Source. Execution added that path without renewing work or retained grant.
- Actor-context close must also withhold the native parent Source retirement floor; only final parent retirement may spend it.
- Current Draw Cargo dependency key is semio-plugin, while the own identity fixture call still used semio_framework_plugin. The own source must use the actual current crate name, with no compatibility alias.

The version2 neutral policy bounds native monotonic deadline, semantic fuel, separate conservative VM execution allowance, call count and retained dimensions. The initial --light assertion run is diagnostic only. Execution reports a later proper1CPU2048MB admitted oracle exit0. Root independently read guest-lifecycle-oracle.txt: it records ordinary capacity waiting, actual1job2048MB admission and the positive schema-oracle completion line. The terminal exit0 is recorded by the execution owner for completed tool chunk095a46. This proves the neutral contract oracle only. Actual current component production, compiler/guest lifecycle and all descriptor/registry/browser gates remain mandatory.

Execution confirmed the identified own receipt floors, raw ActorCall failure receiving and current dependency-name issues fixed. The actual patched close lifecycle uses the captured lifetime and unchanged close generation through Retired acknowledgment, and returned event/result owners are controlled-retired under the same caller context. Fresh canonical guest29814 was normally admitted after56seconds; no compiler/lifecycle execution pass is inferred from that admission. Source is frozen pending real gate findings.

## Owning Test-Level Reachability

Root checked current GuestInstanceCheckScript rather than assuming its default level: it calls resolveTestLevel(segments,quick) before passing the lib/guest_instance_tests filter to the model-custody Cargo runner. The authoritative helper explicitly raises SEMIO_TEST_LEVEL to at least quick in process.env; that side effect applies even though this specialized script ignores its return object. Thus the native law's mod quick is structurally reachable rather than excluded by fundamental default. No source repair was needed. This does not substitute for an actual named positive execution and nonzero test count.
