# Single-Guard Publication And Telemetry Tail TDD

## Staged Boundary

Current boundary: actual [R3](./📓️watchdog-tail-r3-compile-red-2026-08-28.md) failed with four missing-method diagnostics and zero executed tests; parent then approved the exact trace implementation. Actual [R4](./📓️watchdog-tail-r4-native-green-2026-08-28.md) passed3 native tests/20 skipped, including all eight vectors and held real telemetry locks. Trace source is mounted and released; old Watchdog layout/finish remain unchanged. No UI/WGPU production adoption or full-callback publication proof. Actual canonical [sourceR2](./📓️watchdog-tail-source-r2-eight-vectors-2026-08-28.md) separately passed all eight neutral vectors/five hostiles.

## Exact Window

One original Watchdog must first check admission immediately before fixed publication and then produce a terminal verdict after publication and optional telemetry. The current finish reads the end clock before report()/telemetry, so it does not measure that tail. A successful admission checkpoint is not a full-window success.

Proposed phases: original start → admission reading → prevalidated fixed receiver writes if admitted → optional nonblocking telemetry using its interim sample → final clock reading/terminal verdict → fixed scalar result storage. No telemetry, user callback, waker, allocation or live generic Drop runs after that final reading. The last fixed scalar return itself is not an empirically measured machine instruction tail; no exact wall-time claim is made without the mounted outer callback gate.

If terminal elapsed reaches8000 or clock authority fails after the writes, the mounted caller must return **CommittedFault**, retain the exact published owners and quarantine them, not return ordinary Accepted, retry that candidate, claim a rollback, or claim the old source is still unconsumed. If admission already fails, no receiver writes occur. Global telemetry remains optional and cannot decide either outcome. In particular an overrun that occurs during telemetry may appear only in the terminal owned verdict; the earlier percentile/violation sample is explicitly interim, not the final authority.

## Selected Affine API

The staged native tests select the consuming checkpoint shape: Watchdog::admission_checkpoint(self) -> WatchdogAdmission, where the new private-field wrapper owns the same original guard and its observed reading; WatchdogAdmission::finish_after_telemetry(self) consumes that owner. No second start, caller-supplied verdict, or public inner replacement exists. This avoids adding tracking fields to every existing Watchdog/fixture while allowing the new terminal path to reject backward readings relative to the checkpoint, not merely relative to start. Immutable .verdict() output is diagnostic data and cannot construct the wrapper.

The eight current vectors include an interim backward reading followed by a numerically healthy final reading and a missing admission reading followed by healthy later readings. Both remain faulted in actual nativeR4. Actual sourceR2 independently passes the eight-vector oracle.

Superseded history: the initial proposal used a borrowed checkpoint and six vectors, before sticky intermediate authority was added. SourceR1 is preserved as that exact historical six-vector run; neither its borrowed API nor the old refinement-pending statement describes current staged source.

## Authored Native Scope

The three tests executed in nativeR4: eight start/admission/interim/terminal vectors; a previous same-operation/generation success alongside a later8000us terminal fault; and actual held site+violation mutexes while the terminal fault returns without waiting. The clock seam checks that the real optional site sample exists before the terminal reading, rather than merely counting four clock calls. These are guard/telemetry tests, not actual WGPU queue publication or candidate ownership tests. The existing31 root missing-API diagnostics and original five queue semantic REDs remain separate.
