# Curved Path Preparation Retirement

The previous goal turn made verified progress on active trace ownership. Its last native handle 14105 was polled directly and finished exit 0: all eleven trace cases passed, including already-cancelled state. The trace report and acceptance record were updated from confirmed-live to terminal evidence.

## Ownership Contract

PathFlattenJob now supports a consuming retirement transfer in Rust and TypeScript. Valid completed contours move to the caller unchanged; their container identity is preserved. Unfinished, failed and cancelled outputs are never returned as completed geometry. Private partial contours remain owned by the retirement cursor. Each grant releases one flat source or subdivision buffer, one private contour point-buffer, or the empty contour container. The temporary current-contour reference detaches during transfer. Terminal retirement retains no kernel owner or closure, and grants report real structural operations independently of preparation work.

A shared first-party 2D retirement controller now supplies grant validation, progress counters and terminal idempotence for trace and flatten retirement. The repeated controllers were removed from the two TypeScript kernels. Both native wrappers use the same counter, while keeping their domain-specific cleanup steps. Public progress types stay first-party aliases; the existing 2D root exports the new domain retirement types. Nx TypeScript cache inputs include the new controller directory; existing executable commands and launch entries cover verification.

These counts are structural work, not allocator byte credit or an arbitrary hard real-time guarantee. Completed output belongs to the receiving caller and must be retained or retired by its eventual owner. Existing eager cancel paths still exist until the real parent/job registry uses the new transfer protocol. The scheduled editor is not yet activated.

## Neutral Evidence

Seven authored cases cover fresh, preparing, cubic subdivision, successful completed geometry, invalid-coordinate failure, prior cancellation, and several simultaneously retained private contours. Every advance additionally proves the number of retired private contours does not exceed the grant. Tests run grants 1, 7 and 4096, compare work to an independently JSON-parsed contour inventory, validate TypeScript progress with AJV, verify literal completed square coordinates and exact container identity, keep input documents intact, reject invalid grants and duplicate transfer, protect a transferred kernel from subsequent eager cancel, and require an empty final owner. Native numbers are compared through the JSON parser's numeric values rather than treating JSON integer/float representation as different geometry.

## Verification

- Red TypeScript `flatten-retirement-red.log`, handle 84366: exit 1; 141 passed and one failed specifically because `intoRetirement` did not yet exist.
- First implementation TypeScript `flatten-retirement-ts.log`, handle 2608: exit 1 at strict checking because the new shared-controller import traversed one directory too far. Corrected the relative source path.
- Corrected full 2D TypeScript `flatten-retirement-ts-fixed.log`, handle 50064: exit 0, **142 passed / zero failed**, strict production checks included.
- Final multi-contour gate `flatten-retirement-ts-final.log`, handle 84071: exit 0, **142 passed / zero failed**, all seven files and strict production checking.
- Full 2D native `flatten-retirement-native.log`, handle 58283: exit 0, **51 passed / zero failed or skipped**. The shared Cargo lock wait was preserved until the same process progressed to compilation and tests.
- Full Draw TypeScript `flatten-retirement-draw-ts.log`, handle 52547: exit 0, **561 passed / zero failed or skipped**, existing inspected PDF oracle inputs enabled, plus strict and independent/publication/scheduler checks.
- Native cleanup diagnostics `flatten-retirement-native-runtime.log`, handle 51832: exit 0, **two passed / 49 intentionally filtered**. Actual `[DEBUG]` output verifies all seven flatten cases and all eleven trace cases against the shared controller.
- Full Draw native integration `flatten-retirement-draw-native.log`, handle 94709: exit 0, **518 passed / zero failed or skipped**, 3m5 total. All current curve-retirement validation handles are terminal.
- Scoped `git diff --check`: exit 0.

Logs are in the ticket generated directory. The overall goal and ticket stay active; repo MCP lifecycle tools remain unavailable.

## Files

Under `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten`: `🦀️.rs`, `🟦️.ts`, `🧬️schema/🔣️.json`, new `🧫️fixtures/🧹️retirement/🔣️.json`, and existing `🧪️tests/🦀️.rs` and `🧪️tests/🟦️.ts`. Shared controller files are `◻️2d/🧹️retire/🦀️.rs` and `🟦️.ts`. Registration and refactoring touch the 2D Rust package root, TypeScript package project cache inputs, TypeScript root exports, and both trace production sources.

## Remaining Goal

Filled-region and path Boolean jobs, document preparation/trace/Boolean wrappers, byte admission and actual mounted canvas production/consumption still need integration. Picking, selection frames, conversions, typography/outlines, complete IO, browser journeys and multiuser acceptance remain open.

## Next Boolean Ownership Evidence

Direct source inspection confirms that native BooleanJob retains nested input operands, edges whose parameter sets each need retirement, grid/outgoing map entries, nested raw index rings, and point-owning Ring elements. The remaining index/tree/query/winding/atomic/output vectors and heap items are flat native POD owners. TypeScript additionally retains splitValues iterator aliases, a ringHeap of actual Ring objects, and an emitting Ring reference; native instead stores RingItem indices and an emitting index. This difference must be accounted for when retiring internal references, rather than assuming the TypeScript heap is a flat POD owner. Both input types retain caller source contours; TypeScript's constructor only copies the input shell and must not mutate caller-visible operand arrays.

PathBooleanJob must compose the flatten retirement already verified here with filled-region retirement, then retire its remaining source operands, local contours, world contours and prepared operands. Completed output needs an explicit move to its caller; cancellation/failure candidates must stay private. Byte-capacity admission and real Draw mounted hooks remain mandatory before activating scheduled canvas consumption.
