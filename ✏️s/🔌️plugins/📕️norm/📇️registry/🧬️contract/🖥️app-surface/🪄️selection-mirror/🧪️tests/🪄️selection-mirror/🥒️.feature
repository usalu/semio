@capability-norm-live-view
Feature: The live view never freezes a paint and reports evaluation progress
  A paint waits at most the paint budget for slow units; the checks window shows the progress and offers cancellation.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - A settled evaluation has no pending progress: Given a settled live report Then the checks view has no pending progress
  - Slow units show finished over total and never more than the total: Given an evaluation with 3 of 5 units finished and slow units still running Then the checks view shows 3 / 5 pending and a cancel action
  - The next paint picks up what finished: Given a worker that finished after the previous paint When the editor paints again Then the report is the full evaluation and no progress is shown
