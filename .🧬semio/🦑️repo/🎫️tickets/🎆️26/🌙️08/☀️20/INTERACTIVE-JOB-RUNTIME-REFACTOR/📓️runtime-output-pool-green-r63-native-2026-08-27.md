# Runtime Output Pool R63: Two Native Laws Pass

Canonical `SEMIO_COVERAGE=0 ... bun x nx run @semio-tech/ui-runtime-rs:test --skip-nx-cache --args='exhaustive --lib surface_output_pool_ -- --nocapture'` completed with exit 0 in the sole existing native target.

```text
[DEBUG] output-pool fifo=2 exact-rejected-pointer=true paired-credit=true close-grants=1,64,4096
[DEBUG] output-pool preproducer=64 extra=false entry-limit=64 independent-payload-quota=false
Summary [0.030s] 2 tests run: 2 passed, 109 skipped
NX Successfully ran target test for project @semio-tech/ui-runtime-rs
```

Full output: `🧪️member-runtime-output-pool-green-r63-native-2026-08-27.txt`.

The implementation has one static 64-entry Ready payload pool and 64 small FIFO headers. Reservation checks precede producer invocation in the law. Exact queue/entry epoch and generation bind insertion. Zero-grant and occupied-target refusal preserve the original owner and payload pointer. FIFO extraction transfers the paired Ready owner. In-place typed retirement honors 1/64/4096 byte grants; empty entry/header release is separate.

Scope remains primitive-only. Production transaction adoption, static backing admission into the existing resident ledger, contention/ABA/cancellation laws, and full resident census are not credited by these two tests. The original inline census RED is unchanged. No heap payload array per transaction, budget increase, cleanup, or git mutation occurred.
