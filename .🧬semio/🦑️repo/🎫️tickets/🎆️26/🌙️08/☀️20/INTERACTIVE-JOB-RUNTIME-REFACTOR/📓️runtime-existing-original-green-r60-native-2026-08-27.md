# Original Existing-Component Acceptance R60

Both original R30/R31 laws now pass unchanged after the separate-return scheduling correction: **2 passed, 107 skipped**, 0.052s; session 11536 exited 0.

```text
[DEBUG] existing-component-refusal rejected=true allocation-before-admission=0 source-unchanged=true
[DEBUG] existing-component-copy turns=42 allocation-ledger=32768 old-unchanged=true
```

The new exact copy performs 42 retained turns and accounts the actual 32768-byte candidate backing. The original zero-credit refusal still leaves the incoming source unchanged and allocates no candidate backing.

Canonical `SEMIO_COVERAGE=0 @semio-tech/ui-runtime-rs:test --args='exhaustive --lib surface_ownership_existing_component_ -- --nocapture'`, unchanged target/environment. Raw `🧪️member-runtime-existing-original-r60-native-2026-08-27.txt`.

The third original inline census RED remains open. Full resident overlap and transaction output ownership are not inferred from this two-law result.
