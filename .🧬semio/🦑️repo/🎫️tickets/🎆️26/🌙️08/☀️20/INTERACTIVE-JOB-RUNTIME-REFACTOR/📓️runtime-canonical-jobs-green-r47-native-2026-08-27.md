# Canonical Live Jobs GREEN R47

Command: `bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='--lib surface_canonical_document_ -- --nocapture'`.

Actual exit0, **2 passed / 104 skipped / 0.067s**.

```text
[DEBUG] canonical-reconcilers actual-surfaces=9 exact-root-readers=9 roots-after-owner-close=9 typed-reader-close=true
[DEBUG] canonical-reader-replacement grant=1 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=64 original-root-unchanged=true original-credit-retained=true typed-terminal=true
[DEBUG] canonical-reader-replacement grant=4096 original-root-unchanged=true original-credit-retained=true typed-terminal=true
Summary [0.067s] 2 tests run: 2 passed, 104 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

These use actual SurfaceReconcileJob admission/drive/take_ready and actual final-reader close. Nine live reconcilers coexist, each reader aliases its original canonical root without another reservation/tree; old readers retain their immutable bytes and resident permit while the replacement job completes.

Scope remains limited: the complete resident-footprint census and static metadata baseline are not yet complete; the inline census RED remains; transaction output-owner collection and full regressions remain work in progress. No hard callback-latency, Process fit, or consumed Wasm claim.

Raw: `🧪️member-runtime-canonical-jobs-r47-native-2026-08-27.txt`.
