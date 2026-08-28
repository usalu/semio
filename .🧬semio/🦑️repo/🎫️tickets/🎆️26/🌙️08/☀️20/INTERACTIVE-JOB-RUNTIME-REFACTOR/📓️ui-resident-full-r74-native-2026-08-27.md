# Full UI Contract R74

Canonical `SEMIO_COVERAGE=0 ... bun x nx run @semio-tech/ui-contract-rs:test --skip-nx-cache --args='exhaustive --lib -- --nocapture'` completed with exit 0. The existing shared native target and exhaustive profile were retained.

```text
Summary [6.794s] 159 tests run: 159 passed, 0 skipped
NX Successfully ran target test for project @semio-tech/ui-contract-rs
```

Full raw output: `🧪️member-ui-resident-full-r74-native-2026-08-27.txt`.

This covers the static-baseline correction, same-ledger primitive, actual canonical reader-pressure regression, and all current UI contract native tests. It is not a runtime/Process/browser/Wasm timing result. Runtime's output-pool registration and dynamic census remain separate gates.
