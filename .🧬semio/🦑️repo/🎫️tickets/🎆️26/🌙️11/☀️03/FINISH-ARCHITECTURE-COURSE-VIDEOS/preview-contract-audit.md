# Preview Join And Verification Audit

Read-only review, 2026-11-07.

## Concrete wrapper defect

The ticket join-en.ps1 invokes course_preview.py with only en; course_preview.py requires --work-root. If this wrapper reaches that command, argparse exits with an error before joining. The wrapper also waits for a historic BIM-specific log and writes to a generated folder without the trash emoji; it is Windows-path-specific. These findings apply to that wrapper only; no claim is made that the active root workflow uses it. Invoke the Python joiner with an explicit ticket generated work root instead.

## Join and verification coverage

The current course_preview.py checks each manifest entry against master_fresh and SHA-256 before and after normalization, then publishes the combined file. verify-preview.py checks lesson order, preview digest, chapter titles/durations, video and audio format, and invokes ffmpeg xerror full decode, as well as sampling three frames per chapter. Sample sheets need human review and cannot prove continuous visual correctness. The verifier compares the record lesson list but does not independently compare each recorded source hash to the current master; the joiner performs that check before publishing, so the two together cover the intended path. There is no evidence that this distinction caused a current failure.
