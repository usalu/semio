# Fix: shared editor bounded contract

## Problem
Family editor `bounded_first_step_tool_proofs!` blocks restated `ToolExecutionContract::bounded_first_step(...)` with divergent `max_step_micros` (mostly `15_000`, some `7_500`/`7_999`) while `crate::app_surface::norm_bounded_contract()` uses `7_999`.

## Macro
`bounded_first_step_tool_proofs!` takes `contract: $contract:expr` — arbitrary expressions are accepted.

## Fix
Every family editor proof now uses:

```rust
contract: crate::app_surface::norm_bounded_contract(),
```

vdi3805 also:
- routes `execution_contract` through `norm_bounded_contract()`
- drops the stale 7500/15000 doc comment

## Verification
See `🗑️generated/fix-editor-contract/`.

## Results

### Files changed (15 editors)
All `…/✏️editor/🦀️.rs` under the fifteen family artifacts now use `contract: crate::app_surface::norm_bounded_contract()`.
vdi3805 additionally uses that shared fn in `execution_contract` and dropped the stale 7500/15000 doc.

### Runners
- `cargo check --tests -p semio-s-artifact-norm-en1999`: **compile fail** (186 errors; concurrent `En1999Snapshot` breakage — not contract).
- `cargo check --tests -p semio-s-artifact-norm-en1993`: **ok**
- `bun nx run @semio-tech/norm-en1993-rs:test` (fail-fast): early stop; full `cargo nextest … --no-fail-fast`: **80 executed / 76 passed / 4 failed**
  - Failures are empty-report / headline assertions — **no** bounded-contract proof failures.
- `bun nx run @semio-tech/norm-en1999-rs:test`: **no tests executed** (same compile fail).

Logs: `🗑️generated/fix-editor-contract/`.
