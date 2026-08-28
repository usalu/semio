# Resident and Handback Runtime Regression — R41

Canonical command:

```text
bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib -- --skip surface_ownership_inline_fields_do_not_allocate_a_second_owner --skip surface_ownership_existing_component_refuses_before_cloning_unadmitted_payload --skip surface_ownership_existing_component_retains_comparison_and_copy_between_turns'
```

Actual output:

```text
[DEBUG] surface-ownership-oracle checks=33
Starting 101 tests across 1 binary (3 tests skipped)
Summary [0.665s] 101 tests run: 101 passed, 3 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Exit 0; raw `🧪️member-runtime-resident-handback-regression-r41-native-2026-08-27.txt`. The three named exclusions are still real open REDs: inline resident double-accounting, and original R30/R31 existing-component admission/retained comparison. This is an explicitly scoped regression, not a full runtime pass.

This snapshot includes shared permit factoring, pending-return maintenance, exact root/output credit regression, typed handback entry faults, held-registry laws, and the subsequent preflight plus typed pending-patch error-forwarding hardening. It does not certify old nested whole-tree retirement, blocking ordinary Drop handback/admission, or the not-yet-joined reactor error mapping.

Scheduling correction: this command was dispatched while the preceding ActorBytePage tool session had not yet reported completion. The immediate follow-up found the Actor command completed and no remaining matching Cargo process; no claim is made about their precise internal overlap. Both actual outputs are retained. Subsequent dispatch must wait for an explicit exit, not merely a short poll.
