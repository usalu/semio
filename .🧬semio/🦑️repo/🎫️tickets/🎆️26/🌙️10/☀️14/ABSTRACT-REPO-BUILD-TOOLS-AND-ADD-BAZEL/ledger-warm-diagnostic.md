# Ledger Warm-Image Diagnostic

The combined graph suite `final-graph-gate-projection.log` failed only the distinct-dataset law's warm computation count: expected one, observed two. That alone does not identify optional denial, source freshness, or a production bug.

The unchanged real custody test was run through the existing authentic `qualify/📜️script.ts --ledger-gate`, with opt-in `SEMIO_LEDGER_DIAGNOSTIC=1`. Only test diagnostics were added immediately after cold inference and before the original warm assertion: optional cache leaf existence/bytes, actual input SHA/stamps, full captured fact validity and first stale fact, actual filesystem free bytes, and existing observer statistics. Original assertions, Original 120-second gate and existing per-law 60-second controls remain. No reset, cache deletion outside the test's existing owned cleanup, capacity change or publication bypass was introduced.

The focused run terminated zero: **7 tests / 185 assertions / 8.00 s**, `🗑️generated/ledger-warm-diagnostic.log`. Both real shared-image dataset leaves were published (1,813 and 1,814 bytes); their three captured facts were current, no first stale fact existed, input bytes/stamps were stable, and available physical bytes were approximately 1.848 GB (above the unchanged 1 GiB optional floor). Both warm calls reused the images with exactly one computation. Dataset exclusion and retirement assertions also passed.

This isolated green does not establish the cause of the prior combined failure, and no claim of universal optional persistence follows. The opt-in snapshot can identify absent publication or stale captured authority if the combined suite reproduces it. No production algorithm or expectation was weakened.

Test Source SHA-256: `657036ac2ec55bf73d14932c6956b23f6d44ab40e0bc778131366c7b9a1293c5`.


The subsequent authentic combined gate with both ledger and lease fault diagnostics passed **97 tests / 2,186 assertions / 25.19 s**, session 77906 (`final-graph-gate-lease-diagnostic.log`). Both images published at 1,813/1,814 bytes and remained source-current; no SQLITE_FULL reproduced. This retains the original expectations while leaving both earlier causes unknown.
