@capability-incremental-evaluation @oracle-full-evaluation-fold
Feature: Incremental evaluation equals full evaluation
  Executed by `⚡️incremental-evaluation/🧪️tests/🔬️unit/🦀️.rs`. The independent oracle is the full evaluation: the ordered fold of every check unit over the document.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Random mutation sequences: Given a document with 12 members, 4 sections and a global factor When 60 random mutations are applied (field edits, reference changes, inserts, removals), with some runs cancelled halfway Then after each settled run the session report equals the full evaluation report
  - Minimal re-run: Given a settled session When one section capacity changes Then exactly the units whose subjects use that section are re-run and the others are served from the memo
  - Latest wins: Given a run in progress When a newer edit supersedes it Then the run stops at the next unit boundary, finished units are kept and the remaining units stay stale
  - Declared reads: Given a unit and its read set When every leaf outside the read set is perturbed Then the unit output does not change
  - A paint never waits for slow units: Given a family whose slow unit takes longer than the paint budget When the editor paints with the responsive live report Then the partial report and settled false are returned within the budget and the progress names finished and total units And the next paint after the worker finished returns the full evaluation
  - Cancelling keeps finished units: Given a slow unit running on the session worker When the checks window cancel action supersedes the run Then finished units stay in the memo and the others are re-planned on the next paint
