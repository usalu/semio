# Native130 Full Renderer Census

The executed census ran1359 tests:1306 passed,53 failed, zero skipped (29.006s; Nx1m22s). This report organizes observed failures for repair; no failing assertion is waived. Complete command output and structured exact assertions are retained under `🗑️generated/astra-runtime/renderer-native130-full/`.

| Area | Observed failure | Current work |
| --- | --- | --- |
| Component close | An external close wait returns before unrelated input/frame work. | Sol repaired admission fencing for both live FrameBuild and Presenter, then permits sibling work alongside an admitted close. Native131 is running. |
| Local asset cancellation | Publication boundary no longer fails, but fixture cleanup drops a World asset claim before terminal handback. | Sol repaired the fixture to drive exact component Asset cleanup before whole-World retirement. Native131 is running. |
| Input candidate retirement | All five new seal/cancel/close carrier laws pass. | Fresh browser boot also survives the original orphaned-candidate fault. |
| Map rebase | The exact captured Map owner now retires after a same-host sibling rebase. | Full census passes this regression. |
| HTTP head cancellation | Withheld response headers no longer block the sibling request in the regression. | Full census passes this regression; per-actor abandoned-future credit ownership is a separate services regression. |
| EngineCanvas and NodeGraph | Direct scene paint fixtures fail; Map gesture fixtures do not establish active drag. | Root and Terra audit retained host admission and presented input protocol before changing fixtures. Production phase4 requires a mounted scene owner before binding an engine token. |
| Slot budget | Measured EngineSurfaceRegistry entry is76216 bytes; committed budget is75824. | Audit the concrete392-byte increase and memory impact before updating the budget. |
| Shell and Interpreter | Failures include candidate seal refusal, absent hit targets, pointer/modal/focus routing, and keyboard/clipboard dispatch. | Sol began repairing the shared world fixture to paint every visible document in one candidate before one seal and ACK. Remaining failures require fresh triage. |

The browser smoke now exposes a distinct stale-before-submit presenter fault after the Driver heading interaction. This runtime failure has priority over broad fixture repair. Evidence and chronology are in `📓️astra-checkpoint19-runtime.md`.

## Fixture Repairs In Progress

The two retained Map lifecycle tests now inspect and cancel by the actual returned host ID, preserve public wire IDs, and drive exact CPU component cleanup while their document retirement waits. A shared test-only OsHost bridge completion helper verifies the exact engine slot is empty before returning the external close witness. The fixed-slot fixture records the measured76216-byte EngineSurfaceRegistry entry with capacity256 unchanged: the measured392-byte delta costs100352 bytes across the table. Parser checks passed; native GREEN is pending. The six older direct scene paint helpers still need to enter through the Interpreter's retained-document admission seam.

Native131 ran83 selected tests:82 passed, one local asset terminal-cleanup failure (11.470s;1276 filtered tests; Nx3m39s). Component-close scheduling and the candidate retirement laws pass. Local cleanup requires a focused trace of the already-built binary; no broad native GREEN is claimed.

Native132 full census ran1360 tests:1309 passed,51 failed, zero skipped (31.402s; Nx4m20s). The fixed-slot measurement passes. The shared Shell fixture no longer fails seal refusal; its affected cases now reach their window activation assertions and still fail. The two Map lifecycle fixtures reach host-specific assertions but require the presented-candidate seal/ACK sequence used by the passing sibling Map law; that sequence is now source-applied to those two fixtures, GREEN pending. Their explicit close bridge and terminal slot assertions remain intact.

The Native132 local cancellation backtrace identifies a production lost wake: when `RendererIoHandle::poll` finds its exact slot temporarily checked out, it can return Pending without retaining or waking the task's waker. The unowned task drops its live asset owner. Sol is adding a deterministic checked-out-slot regression and preserving the task through a self-wake before returning Pending. No deadline extension is planned.

Native133 now runs the retained-owner integration selection plus the deterministic checked-out I/O wake law, the two corrected presented Map lifecycle fixtures, and the slot-size receipt. Its result is pending. The Map fixture's CPU cleanup now removes only its own staged surface rather than draining the shared staging table.
