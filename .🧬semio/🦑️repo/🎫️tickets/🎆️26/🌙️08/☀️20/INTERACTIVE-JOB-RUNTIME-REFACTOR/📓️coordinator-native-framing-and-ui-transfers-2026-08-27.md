# Coordinator Native Framing and UI Transfer Review — 2026-08-27

## Executed Evidence

The sole native executor ran these gates. The coordinator read their reports and actual test-result/debug lines, not merely source assertions. These are not additional coordinator Cargo runs.

| Gate | Actual result | Scope |
| --- | --- | --- |
| Common Kernel framing R2 | 4 passed, 253 skipped, 0.047 s | Two fixed-header/split-reader laws and actual existing Invocation/Presence dialects |
| Shared component comparison R68 | 3 passed, 153 skipped, 0.039 s | Eight-byte checked frames, all 18 native variants, hostile values, cancellation/contention |
| Shared component copy R70 | 6 passed, 151 skipped, 0.199 s | Allocation separated from child work and each completed root return; initialized-byte cancellation |
| Runtime canonical grants R52 | 3 passed, 104 skipped, 0.181 s | Nine reconcilers/readers, old-reader credit, exact comparison/read-close/copy completion boundaries |
| Runtime layout R53 | 1 passed, 106 skipped, 0.021 s | Actual unchanged cursor and retained-owner limits |

Raw logs are [Kernel R2](🧪️member-kernel-return-framing-green-r2-native-2026-08-27.txt), [comparison R68](🧪️member-ui-compare-frame-green-r68-native-2026-08-27.txt), [copy R70](🧪️member-ui-copy-transfer-green-r70-native-2026-08-27.txt), [runtime R52](🧪️member-runtime-canonical-grants-r52-native-2026-08-27.txt), and [layout R53](🧪️member-runtime-canonical-layout-r53-native-2026-08-27.txt). Their original artifacts remain in place.

## Source Review

The compact comparison frame retains the complete 256-frame logical depth. Page indices are checked against the actual 256-slot domain before conversion; the only sentinel is 65535. The checked two-operand text position bound is 1024. Actual frame size is eight bytes and comparison cursor size is 2208 bytes. This is not a depth reduction, Box allocation, or raised work limit.

The current existing-component runtime path structurally retains the incoming component in its record and the captured canonical document lease outside the bounded read callback. Each comparison advance reacquires only a nonwaiting exact document read. Comparison completion, read retirement, wrapper release, source return, and candidate placement occur on separate turns. R52 logs actual 4096-byte final comparison, separate 4096-byte read close, 3096-byte source return, and 6416-byte physical candidate placement. The last figure uses the existing physical 32768-byte grant and is not relabeled 4096-byte comparison work.

Actual R53 sizes are reconciler 760, cursor 48552, retained owner 64992 bytes, below the unchanged 48 KiB and 64 KiB limits. This resolves that exact R50 layout failure, not the separate complete resident census.

## Remaining Sibling Boundaries

Coordinator source inspection still finds fresh-component completion calling the child and then transferring two component roots in the same turn. Fresh-binding completion also transfers and clears both roots after the child result. These are assigned to the same native runtime owner for exact frontier, grant, and unwind tests; the repaired existing-component path does not certify these sibling paths.

The separate owned UiDocumentComponentCompare API still exposes an ungranted take_completed and clears its completed lease wrapper on the child's completion turn. Its callers and accounting remain separate from the new runtime borrowed-comparison path. No broad terminal or boundedness claim is made for that API.

The original inline physical census, full simultaneous resident ownership, paired transaction output admission, unwind coverage, complete native regression, strict callback timing, and fresh Process workshop remain open. Kernel header/dialect tests do not prove a retained whole return encoder, input ACK ownership, WIT poll cutover, or live guest execution. No cleanup or generated publication occurred.

## Orchestration Correction

A coordinator attempt to open coordinator-react-full-r22 used a guessed filename that did not exist. The actual retained report is coordinator-renderer-react-full-r22 and was located and read immediately afterward. This is not evidence loss.

