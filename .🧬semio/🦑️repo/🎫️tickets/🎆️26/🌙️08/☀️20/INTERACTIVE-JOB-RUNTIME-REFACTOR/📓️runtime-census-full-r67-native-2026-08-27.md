# Full Runtime R67: Baseline Fixture Regression

Canonical exhaustive full runtime suite, without exclusions, completed exit 1. Raw: `🧪️member-runtime-census-full-r67-native-2026-08-27.txt`.

```text
FAIL surface_ownership_resident_reservation_uses_one_shared_aggregate_ledger
Summary [4.488s] 80/115 tests run: 79 passed, 1 failed, 0 skipped
warning: 35/115 tests were not run due to test failure
```

The old dynamic-permit fixture captured its baseline before the first runtime fixed-domain registration, then expected final dynamic release to remove that permanent static domain too. The corrected fixture explicitly registers fixed backing before measuring dynamic admission/return. Its exact dynamic byte/slot assertions remain unchanged. The companion contended-return fixture received the same setup correction. Dedicated R65 independently asserts the static registration delta, no double charge, and retention after all queue owners close.

This is not a full-suite pass. The full suite will be rerun; no assertion or resident ceiling was relaxed. Original inline census now passed in R66, while broader dynamic census and live PatchTracker ownership remain open.
