# Pure Fixed VHD Codec Review

Read-only new-files-only review; no codec tests, image allocation, imports/routes or kernel/runtime operations executed in this lane.

Pure encoder has no IO/clock/entropy and uses explicit UUID/timestamp. Fixed sentinel,512byte footer offsets, big-endian fields, checksum and bounded CHS geometry are coherent with the stated fixed-format contract. Existing MiB filesystem/physical envelope includes footer within at least1MiB reserve; no capacity widening is introduced. Fresh output arrays do not share caller mutation.

Test uses independent SQL construction of all512 bytes and Ajv closed corpus/request, plus actual first-party parser as additional check. SQLite is an independent encoding implementation, not an independent VHD reader/kernel interoperability proof. Real Windows mounting and physical allocation remain unqualified.

Concrete DATA stability issue reported: encoder validates unknown value then aliases/repeatedly reads it. Accessor/proxy fields can change after validation, so output may use unchecked values. Capture one privately owned primitive request snapshot and validate that snapshot; retain private schema/protocol snapshot against mutable shared JSON module data. Add changing-accessor actual-function counterlaw. No Source edit performed by audit.

## Stability Correction Reread

Current encoder captures own descriptors once, refuses accessors and symbols before getters, validates owned primitive DATA and encodes only that frozen snapshot. Private schema/protocol are detached from imported mutable JSON. Inherited fields are not authority; extra/nonprimitive fields refuse through the closed schema. Fresh output remains independent. Changing-accessor getter0 and external-schema mutation scenarios exercise the actual function. Narrow Source review is clean; requested separate new semantic RED evidence and final strict remain executor-owned. No production imports/routes/runtime or independent VHD interoperability qualification is inferred.

Executor confirms separate actual accessor regression replay RED: prior unchecked alias line temporarily replayed only in the new unimported helper, focused law observed3 getter reads versus0 and returned1; exact snapshot restored. Final full two tests232expects471ms returned0 and final strict17031 returned0; held four hashes match final strict Source. Replay scope is explicit and not conflated with initial stub RED. This strengthens TDD evidence without proving VHD/kernel interoperability.
