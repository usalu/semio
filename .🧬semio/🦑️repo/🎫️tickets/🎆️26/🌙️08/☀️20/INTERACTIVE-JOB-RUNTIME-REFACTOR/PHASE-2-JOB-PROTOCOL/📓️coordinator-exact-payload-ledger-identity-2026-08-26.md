# Exact Payload Ledger Identity

## Scope

This checkpoint covers only the identity invariant between a resumable job step and its retained payload ledger. It does not claim the wider Phase 2 or runtime acceptance gates are complete.

## Implementation

`StepContext::with_payload_ledger` now requires both parts of the retained operation identity to match exactly:

- the ledger operation identifier equals the step operation identifier;
- the ledger generation equals the step generation.

This prevents retained payload pages from being attached to another operation or to a stale generation of the same operation identifier.

## Regression Law

`payload_ledger_identity_must_match_the_exact_step_context` exercises both hostile cases independently and catches the expected assertion failures:

- operation `90000` paired with ledger operation `90001`;
- operation generation `6` paired with ledger generation `7`.

## Verification

Command:

```text
CARGO_TARGET_DIR=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-0-SCHEDULER-FOUNDATION/🧪️target-p0-current CARGO_INCREMENTAL=0 cargo test --locked -p semio-framework-job payload_ledger_identity_must_match_the_exact_step_context -- --nocapture
```

Observed result on 2026-08-26:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out
```

The output also contained the two expected, caught assertion messages for the operation and generation mismatches.
