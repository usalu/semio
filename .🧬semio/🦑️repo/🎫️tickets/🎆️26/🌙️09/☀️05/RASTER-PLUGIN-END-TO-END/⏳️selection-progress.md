# Selection Progress Repair

Updated the neutral cancellation fixture before implementation: All/Invert first progress is 0.25; the first half-completed combination grant is 0.625. The combination mounted test now consumes its fixture value rather than a hardcoded 0.75.

Selection publication receives an explicit starting phase. Shape/flood retain 0→0.5 preparation, combination uses 0.5→0.75, and span encoding uses 0.75→1. All/Invert editing uses 0→0.5 and span encoding 0.5→1. This removes the previous reset to zero at span encoding.

Added two mounted regressions (fixture actions All and Invert) recording the progress element's actual published value attributes. They require intermediate progress in both phases and a bounded monotonic sequence until the selection command is submitted. Existing cancellation tests cover the updated first grants.

Both changed TSX files passed Bun syntax scans. No typecheck or test execution was possible under the existing descriptor-3 source-admission failure; no passing-test claim is made.
