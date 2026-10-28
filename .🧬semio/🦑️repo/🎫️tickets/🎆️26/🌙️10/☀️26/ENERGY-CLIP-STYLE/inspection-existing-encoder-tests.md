# Existing Encoder And Transaction Validation

The normalization/count-guard production files stayed frozen during this follow-up. No permanent test contract or source implementation was changed.

## Terminal Results

- Existing contract and media runner: 13 tests in 287.640 seconds. All four native media cases passed: silent 12 frames, resampled 8 frames, edited-source 16 frames, and actual-UI audible 49 frames.
- That initial runner retained five setup errors across two contract methods because the runner had not created its generated output root before those early TemporaryDirectory calls. The normal authoring entry already creates the root. This was a ticket-runner setup defect, not a product or test-contract failure.
- The two affected unchanged contract methods were rerun after the runner-only mkdir correction: 2 tests passed in 0.841 seconds. These include all four presentation/content/source/cursor change subcases and the failed-certificate atomicity case.
- Existing collection media-port cases passed: 3 tests in 1.593 seconds, covering unique repeated measurement logs in one work directory, cancellation before resolving ports, and cancellation during rendering preserving the previous movie/certificate and reclaiming the candidate.

All 13 unique affected presentation contract/media tests therefore pass across the initial and targeted rerun, plus the three existing media-port/cancellation tests (16 unique cases). The initial failed runner result is retained accurately in generated/ie/compat/existing-result.json; no failed whole-run result is relabeled GREEN.

## Preserved Coverage

The native tests validate decoded source frame ordering/full frame edges, exact expected finite clocks, source resampling choices, edited MP4 packet-versus-decoded-frame behavior, camera seams, contextual boundary identity, finite silence, audible 440Hz body placement/amplitude and audio padding. Contract cases retain explicit locales, current catalog/source provenance, one ordered collection publication transaction and atomic/current-input protection.

No expensive camera/browser-control suite or overview test was repeated. Those are separately green. The new mixed-mode encoder regression and the independent actual EB07 proof remain documented in inspection-encoder-clock.md and inspection-encoder-normalization-proof.md.

## Retained Input And Evidence

Input: inspection-existing-encoder-tests.py. The runner now creates its configured generated root before loading cases.

Generated scope: generated/ie/compat, including the original terminal diagnostic and native-evidence audible movie/frames. The parent retains required evidence before final generated cleanup.
