# Retained IFC Event Admission

## Implemented source boundary

`IFC/🔎️details/admission.rs` now walks the original owned `SnapshotEditEvent` payload under its initiating `StepContext`. It counts actual DSL nodes, rejects nonfinite numbers, enforces depth 128 and nodes 65,536, and compares duplicate object keys in chunks of at most 64 bytes. Its traversal metadata is inline. It then drives the existing native Store canonical encoder over the borrowed original event, counts the exact event JSON bytes against 16,777,216, and closes that encoder before the reducer may move the payload. It does not allocate an encoded event body.

The editing trait adds an optional retained extent reservation and a separate path-only admission method. Generic editors retain their original cold extent behavior. IFC4 and IFC4x3 reserve their declared work extent and execute actual admission in the retained worker. IFC4's semantic inspector admission is preserved. A reservation is not acceptance: no captured source, pending mutations or publication appears before original admission succeeds. Accepted extents are cached after payload consumption. Missing original mutable event ownership is refused explicitly.

Existing cold admission remains available with unchanged byte/value checks. The shared event derives the existing native canonical-tree interface; no runtime third-party dependency was added.

## Tests and independent evidence

Schema-first fixture: `IFC/🔎️details/🧪️tests/admission.json`. Six real events have independently computed UTF-8 wire lengths 81, 80, 48, 87, 71 and 45. Refusals cover duplicate keys, NaN, excessive nodes/depth, and a string whose complete event exceeds the wire limit. `admission-oracle.ts` passes using Ajv2020 against the existing event JSON Schema and ECMAScript UTF-8 JSON encoding. This is fixture/oracle evidence, not native cursor execution.

Native source tests exercise original retained caller grants, exact cold/native encoder equivalence, cancellation at every admission frontier, owner retirement after cancellation/refusal, and the real IFC worker's admission-before-capture boundary. The real worker test checks every successful admission prefix with a canceled original request and verifies the captured graph is untouched.

`preimage-oracle.py` also passes: 11 progressive batches, 29 immediate inverse rows, exact real bits restored, and five independent IfcOpenShell scenarios. This remains independent evidence for the progressive inverse fixtures, not execution of the Rust overlay.

Rustfmt parsed the new admission files, worker changes and the narrow editing-factory changes. Authentic Rust compilation remains blocked before type checking by the repository compiler ingress requiring its launcher and enforcing Store. No compiler bypass, native heap trace, scheduler execution or native cancellation pass is claimed.

## Remaining ownership boundaries

The event is already typed when this worker receives it. Generic action-to-event conversion and wire-to-native command receiving must be audited independently; this change does not claim to make their earlier decoding incremental. Path shape checks still use bounded/synchronous existing pointer helpers and reference normalization; their temporary allocation accounting is not yet closed. Whole entity/constituent reduction still contains synchronous owned reconstruction and declaration population. A finite tick around those operations is not progressive ownership. Native canonical-tree depth is separately constrained by Store's physical cursor capacity and is not represented here as a promise that every 128-level DSL value publishes.

The progressive overlay keeps original snapshot owners and immutable pending payloads alive through inverse encoding and controlled close. Whole argument/header replacement borrows the original before-value without cloning it. Nested reconstruction of a large old body retains the explicit contiguous-grant denial regression. These boundaries are described in `host-progressive-preimages.md` and must remain visible until their real retained paths execute.
