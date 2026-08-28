# Canonical Runtime Grant and Layout Gates — R52/R53

R52 actual **3 passed, 104 skipped**, 0.181s: the real nine-reconciler/captured-reader law, old-reader replacement under 1/64/4096 retirement grants, and new near-grant completion law. The latter finishes comparison with a full 4096-byte child turn, leaves both roots structurally retained, releases the last old-root read under a separate 4096-byte retirement turn, and returns each completed copy root separately.

```text
[DEBUG] parent-child-grants compare-final=4096 lease-close=4096 comparison-owner=2256 source-return=3096 candidate-physical=6416 separate-turns=true
[DEBUG] canonical-reconcilers actual-surfaces=9 exact-root-readers=9 roots-after-owner-close=9 typed-reader-close=true
```

Candidate placement's 6416-byte inline physical ownership uses the existing 32768-byte physical grant; it is not called 4096-byte work. Comparison initialization is 2256 bytes, no heap and no duplicated incoming component. Each advance reacquires only an exact canonical `try_read` guard. Incoming and document lease remain outside that guard/callback.

R50 had actually failed the unchanged layout bounds: reconciler 760, cursor 53712, retained owner 70152; 0 passed, 1 failed, 105 skipped, 0.094s. R53 now actually **1 passed, 106 skipped**, 0.021s:

```text
[DEBUG] canonical-owner-layout reconciler=760 cursor=48552 retained=64992
```

Thus the cursor remains below the unchanged 48KiB limit and retained state below unchanged 64KiB. No Box or enlarged limit was used. R53 precedes the new four-frontier unwind fixture; that fixture awaits R54.

Canonical commands use `SEMIO_COVERAGE=0` with `@semio-tech/ui-runtime-rs:test --args='exhaustive --lib surface_canonical_document_ -- --nocapture'` and the exact layout selector. Sessions 77501 and 27194 exited 0. Raw: `🧪️member-runtime-canonical-grants-r52-native-2026-08-27.txt`, `🧪️member-runtime-canonical-layout-r53-native-2026-08-27.txt`; historical R50 raw retained unchanged.

Remaining acceptance gates include the original inline-footprint RED, complete simultaneous resident ownership, transaction paired-output authority, full native regression and fresh Process workshop. These scoped passes do not settle them.
