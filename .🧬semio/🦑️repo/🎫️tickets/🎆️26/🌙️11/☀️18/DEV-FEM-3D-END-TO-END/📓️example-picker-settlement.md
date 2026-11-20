# Example picker settlement

## Reproduction

The React shell displayed Concrete Forest after FEM 3D refused the manual example input. Automatic boot wrote the brand default before dispatch and had the same failure mode. Guest history replay also set the label before its input settled. The generic input funnel remembered explicit IDs before dispatch, allowing later redo to reuse a refused choice.

## Implementation

Manual, boot, and replay selection publish through the existing input funnel after an applied `setActiveExample` result. The funnel verifies that the captured program and loaded plugin are still live. It records the latest applied input sequence per program and updates the visible example only for the focused program. Out-of-order completions cannot replace a newer applied selection.

An unlocked picker starts at the existing localized placeholder. Boot still resolves the configured example from the offered list and publishes the selection on success. An empty default input displays the resolved boot ID after success while keeping the latest successful explicit ID for redo. History patches derive their selection from the focused program's successful dispatches. The program lifecycle retires its remembered selection with its history state.

## Verification

- The browser build activated at 03:18 left the picker at its placeholder on both the boot refusal and manual Concrete Forest refusal. The manual command produced input #2 and a refusal alert.
- The language-neutral settlement fixture covers refusal, supersession, out-of-order completion, successful selection, default selection, and background selection. Bun matched all six cases and the undo/redo assertions. An independent SQLite evaluation matched all six fixture outcomes.
- The focused React shell option locks and initial example integration suites passed all 46 tests.
- The standalone React package TypeScript check exposed a stale optimistic branch in the generic dispatch, which was removed. The rerun reported missing shared generated shell exports and no diagnostics in the picker paths. The renderer-scoped Git whitespace check passed.
