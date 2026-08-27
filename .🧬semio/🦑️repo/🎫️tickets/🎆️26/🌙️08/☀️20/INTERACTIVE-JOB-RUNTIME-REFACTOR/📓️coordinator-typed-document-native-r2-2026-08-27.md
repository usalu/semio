# Typed UI Document Native Review

## Actual Results

The coordinator read the actual targeted native output:6passed,102skipped,.134s in `🧪️member-ui-document-typed-green-r2-native-2026-08-27.txt`. It then read the full current UI output:108passed,0skipped,2.742s in `🧪️member-ui-full-green-r6-native-2026-08-27.txt`. The earlier exact document-descendant law really failed before this correction.

The coordinator read all6test bodies, the typed cursor and its document-slot integration. Coverage includes all18Component and11UiPatchOp variants, exact payload bytes,512-byte Unicode text and32768-byte Surface data under1/64/4096-byte grants, no-progress zero grants, unchanged aliased readers, final-alias claims that global maintenance cannot steal, cancellation handback and close-step mutex contention. Type-associated schema depth computes8for UiSnapshot and UiPatch, under the16-slot fixed traversal path; arbitrary UiValue nesting uses its own arena-linked cursor, not that depth bound.

## Ownership Boundary

Typed records stay in their existing claimed document slot while individual fields retire. Only the final-alias owner may mutate the record; a retiring document rejects header/page reads. A detached credited page owns its own nested aliases and remains unchanged. Lists pop emptied elements and release empty backing separately. Slot terminal waits for exact value descendants and typed storage; it cannot become terminal after merely queuing them globally.

## Remaining Work

The cancellation/exception Drop handback still enters the blocking document-arena helper. The focused try_lock close law does not certify Drop. The owning lane is mounting a held-arena Drop RED and designing fixed-slot atomic handback obligations, preserving exact owners without allocation, dropping them, or a best-effort try_lock discard. The corresponding old UiValue handback remains in scope.

Typed patch/resource adoption into every parent, outer runtime close timing, per-instance renderer/host/ACK ownership and all-app native/Wasm/browser gates remain open. The original cold close R6 values8519/19965microseconds are not superseded by these small-crate tests.

