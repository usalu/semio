# Native Operation Child Forwarding

## Source Inspection

The lower `OwnedUiWireOperationCursor` and `OwnedUiWirePatchCursor` still contain forced-pending joins despite the repaired instance/surface wrappers. This inspection is not an executed new test or a live-path regression claim. The existing implementation is held for the coordinator's combined snapshot.

`OwnedUiWireOperationCursor.advance` turns a child decoder rejection into a thrown exception and then a fresh 128-byte result, discarding the child's actual counts. A blocked decoder is forced to pending. `#closeInput` and the operation-retirement branch force all child kinds to pending and unlink on the returned complete flag instead of a later terminal-owner step.

The patch stream similarly catches decoder/apply rejection into a replacement 128-byte result. Decoder completion also takes the result and starts close in the same step. Input close and operation retirement force pending. The surface child join preserves its kind, but still needs count validation and separate terminal-reference release. These joins require precise child forwarding, not a global generator rewrite.

## Staged Repair Laws

The uncollected `native-child` neutral fixture declares seven real private child boundaries, blocked/rejected results, a full-grant terminal result, over-grant rejection, zero-grant immutability and separate 64/128-byte wrapper steps. Tests will intercept actual class children through private-owner test control, preserve the original real owner, restore the method, and complete explicit close. A failure must neither lose a root nor turn into progress. Node assertion/Ajv will check the neutral expectations, and existing native operation byte/semantic oracle tests will run as regression.

After the source hold is released, the planned repair forwards child fault/refusal and raw work counts, validates at most one item and the actual byte grant, and keeps the owner until its terminal state is observed in a later wrapper transition. Result capture, receipt issue and reference removal become explicit phases where needed. A thrown child remains owned with a surfaced diagnostic; no claim is made about unobservable work inside a throwing test double.

## Actual TDD and Repair

R1 failed both selected tests at the actual blocked-to-pending assertions: decoder forwarding and payload retirement. It discovered 642 tests (two failed, 640 skipped), 5.93 seconds. The repair preserves child kind/counts, validates one-item/byte grants, and keeps a terminal child reference for a separate 64-byte release. Stream result capture and input receipt issue each have a separate 128-byte phase. Decoder and surface rejections no longer lose their counts through a replacement error result.

R2 passed all eight decode combinations (two layers, two refusal kinds, zero/over-grant counts). Its close fixture still attempted to intercept an already-terminal scalar retirement. R3 narrowed that case to a real retained field payload and added terminal full-grant and throwing-close laws. The stream setup was then corrected to stop at the explicit result-capture phase: the operation cursor has already retired its own input before this phase, so waiting for another child close call was not a reachable boundary. Neither correction weakened the production terminal check.

R3's new law reached the terminal-4096 assertions successfully, then failed because an actual private decoder close exception escaped. Public operation/stream close now retains its owner, records the exception and returns an explicit rejected result; later close remains usable. This records no invented byte count for work hidden inside the throwing child.

R4 passed all three selected tests, 640 skipped, 643 discovered, 7.31 seconds (243 ms execution). The complete logs are `🧪️renderer-owned-native-child-red-r1-2026-08-27.txt` and `🧪️renderer-owned-native-child-r{2,3,4}-2026-08-27.txt`. Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedNativeChild'`.

Strict R1 had seven existing tutorial errors plus three new fixture typing errors (assertion-call annotations and callback `this`). Exact owned types repaired those. Strict R2 has zero owned errors, but is still RED with fifteen diagnostics: the seven tutorial joins plus eight peer-owned in-progress Shard return-source declarations. Its entire output is `🧪️renderer-owned-native-child-strict-r2-2026-08-27.txt`; it must not be described as a strict pass or only-seven snapshot.

The broader `OwnedWireOperation` R5 regression passed all seven tests, 636 skipped, 643 discovered, 12.82 seconds (10.27 seconds test execution), including all eleven tags, whole input-prefix cancellation, native buffer ownership and late-cancel publication ACK. Full output is `🧪️renderer-owned-native-child-regression-r5-2026-08-27.txt`. R6 `OwnedInstance` passed thirteen tests (630 skipped, 643 discovered, 8.99 seconds), and `OwnedIntake` passed five (638 skipped, 643 discovered, 6.85 seconds); both complete outputs are retained in `🧪️renderer-owned-native-child-Owned{Instance,Intake}-r6-2026-08-27.txt`. Lower typed payload/index/generator implementations still have their own forwarding obligations; this packet establishes the inspected native-operation wrapper boundary, not an all-layer or live transport certificate.
