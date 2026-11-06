# IO retirement codec review

The receiving frame retains the decoded chain nodes, partial error strings and constructed retirement owner across a refusal. Decode scans the original child chain iteratively and checks increasing original depth before allocating its fixed node vector; reconstruction attaches the original nodes from leaf to root.

At initial review node birth charged native ownership before work admission; the owner moved work admission earlier. A separate allocator-failure concern remains: OriginalFailureStorage::admit may fail after native.charge succeeds, with no physical node or receipt, so its ledger/reservation contract needs explicit reconciliation. The chain owners remain in the frame.

Encoding retirement_value recursively calls itself per child while passing usize::MAX to scoped_depth. A sufficiently deep valid chain can exhaust native stack without a finite depth limit. A bounded iterative encoder or an actually enforced finite original depth limit is required to preserve all owners.

Source-only review; no build or runtime verification.

## Revised source

The encoder now preflights depth iteratively and builds the chain under a guarded root; node admission uses original quote/post with actual receipt, addressing the previously identified recursion and premature charge paths. FailureFrame retains partial nodes and decoded errors for paid retirement.

Remaining source concern: object producer evaluates an entire fallible array before inserting any entry into its guarded object. If a later optional_error refuses after an earlier field allocated a DslValue, the earlier temporary is dropped outside the root/fields guard. The outer top-level object uses the same pattern. Incrementally attach each successful value to guarded ownership before the next fallible operation. Runtime validation remains pending.

## Field builder follow-up

Current builder inserts each completed field into its guarded object before starting the next fallible field. push_encoding_controlled also guards the incoming value during key admission. The previously identified inter-field temporary loss is addressed in source. No runtime verification was performed.
