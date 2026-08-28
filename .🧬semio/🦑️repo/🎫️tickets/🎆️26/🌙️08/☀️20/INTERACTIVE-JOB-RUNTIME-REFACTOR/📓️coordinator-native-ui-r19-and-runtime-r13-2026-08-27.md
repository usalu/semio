# UI Allocation, Cancellation, And Remaining Physical Ownership

## Reviewed Actual Native Output

On 2026-08-27 the coordinator read the retained native log footers and cancellation DEBUG evidence:

| Gate | Actual result |
| --- | --- |
| UI contract R19 | 122 passed, zero skipped, 5.591 s |
| Runtime paged allocation/cancellation R12 | 2 passed, 87 skipped, 0.060 s |
| Runtime regression R13 | 88 passed, one explicitly excluded intentional inline-census RED, 1.945 s |

R12 reports cancellation after pending/directory/page/placement with 0 / 27,672 / 33,992 / 33,992 bytes retained before close. All four cases terminate with zero allocation during close. This verifies exact ownership and allocation phasing for this path; it is not a full resident meter or Process workshop fit proof.

Logs: `🧪️member-ui-full-green-r19-native-2026-08-27.txt`, `🧪️surface-patch-allocation-green-r12-native-2026-08-27.txt`, `🧪️surface-runtime-regression-r13-native-2026-08-27.txt`.

## Shared Backing Decision

The current `UiFixedList` stores `Option<Vec<T>>`, reserves all N slots in one allocation, and its cold `Clone` copies every initialized element. `UiNodeBindings` is `UiFixedList<ActionBinding, 32>`; the measured 2,072-byte element yields 66,304 bytes, above the unchanged 32,768-byte work grant. `FreshRecordClone` calling a field-level credited clone therefore does not establish bounded copying. Other UI component lists and maps share this prerequisite.

The publication executor owns the common fixed-cap paged backing/admission region and its typed retirement join, with retained runtime comparison/clone/census adoption. The tutorial executor continues the separate OrderedMap interaction root/cursor. This is one shared collection design, not a second binding-only representation. All logical capacities and ordered wire shapes stay intact. Directory/page reservation, placement, exact rejection, physical byte counters, and empty-page release must have separately tested ownership. No existing logical limit or time/byte budget is raised.

The 32-binding runtime regression has been authored by the executor but has not yet executed at this checkpoint. The UI wasm32 compilation gate is running; compilation success is not yet claimed. Producer pre-materialization admission, complete current/candidate/patch/cursor/retirement accounting, callback timing and fresh app behavior remain open.

## Static Verification Drift

The retained canonical static run after the Puzzle microsecond source repair now fails at `render-fault-wire-erased`: its string mutation made no source change. This is later than the earlier root R8 Puzzle failure, but still precedes a completed findings census. Root source review also confirms pending-operation and fixed-list verifier strings name the superseded Option/Box layout. They must be retargeted to the actual ownership boundary, preserving hostile rejection, after the storage source is coherent. An old list of 19 findings is not a current completed run.

