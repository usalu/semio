# Runtime Output Pool R64

Canonical exhaustive selector `surface_output_pool_` completed with exit 0 in the existing shared native target; coverage disabled. Raw output: `🧪️member-runtime-output-pool-r64-native-2026-08-27.txt`.

```text
[DEBUG] output-pool held-mutex-drop-waits=false exact-return-drained=true
[DEBUG] output-pool reuse-before-drain=false exact-epoch=2 explicit-close-no-second-return=true
[DEBUG] output-pool busy-refusal-exact=true zero-grant-mutates=false static-bytes=125088
[DEBUG] output-pool fifo=2 exact-rejected-pointer=true paired-credit=true close-grants=1,64,4096
[DEBUG] output-pool preproducer=64 extra=false entry-limit=64 independent-payload-quota=false
Summary [0.069s] 5 tests run: 5 passed, 109 skipped
```

The actual native shared backing is 125,088 bytes, counted from the registry mutex and fixed atomic handback arrays. Fixed backing still awaits same-ledger registration; there is no resident-bound claim yet. The held-lock test drops the exact reservation and queue on another worker, checks the original entry remains retained, then drains and proves both slots and return bits empty. The saturation test requires an explicit return drain before same-slot next-epoch reuse and rejects the old epoch. Explicit close followed by Drop does not enqueue another return.

These five laws do not cover live transaction production, full physical census, or callback timing. Original census RED remains unchanged.
