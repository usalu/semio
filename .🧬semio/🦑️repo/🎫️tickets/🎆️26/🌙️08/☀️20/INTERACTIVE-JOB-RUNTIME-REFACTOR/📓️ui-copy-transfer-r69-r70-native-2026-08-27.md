# Component Allocation and Independent Root Transfers — R69/R70

Schema-first R69 compiled the new split-allocation/root-return test against absent methods and failed with six E0599 references to the three intended APIs. No test executed.

The implementation exposes allocation-only `reserve_next` and separately granted completed source/candidate returns. The allocation step passes zero payload work to the exact retained field cursor. It does not round up work credit. Refusal leaves the exact roots and backing unchanged; the shared initialized-byte/cancellation paths remain in use.

R70 actual `retained_component_copy_` native run: **6 passed, 151 skipped**, 0.199s; session 34105 exited 0. It includes the new exact transfer test, both byte-owner tests, all native component variants, large-list cancellation, and real 4096-byte work-grant surface copying.

```text
[DEBUG] component-copy-real-grant inline=3096 work-max=4096 complete=true
Summary [0.199s] 6 tests run: 6 passed, 151 skipped
```

Canonical route: `@semio-tech/ui-contract-rs:test --args='--lib retained_component_copy_ -- --nocapture'`, unchanged master target/environment. Raw: `🧪️member-ui-copy-transfer-red-r69-native-2026-08-27.txt`, `🧪️member-ui-copy-transfer-green-r70-native-2026-08-27.txt`.

This is shared copy ownership evidence only. Runtime near-grant parent completion, complete resident accounting and transaction output ownership remain separate acceptance gates.
