# PDF17 X/VT Source Neutral Demand Readiness

Held exact owning-file pair: `📥️inputs/pdf17-x-vt-source-metadata-demand-held-pair.json`. No mount, Source/Native gate, or passing claim.

The existing registered PDF metadata test file is extended with strict AJV validation of both hand-authored contracts, exact official input length and independent node:crypto digest, typed output-intent BLOB projection/readback, independent Bun SQLite full profile byte equality, integrity/FK checks, and literal raw reference dictionaries. VT explicitly covers DPartRootNode/Parent/Start. The nested OutputIntents array member is checked by full dictionary reconstruction; the direct SQL dictionary-entry join deliberately does not pretend an array node is a reference value.

Actual handwritten SQL64 confirms the seven output-intent columns: id/subtype/condition_identifier/condition/registry_name/info/profile. Optional metadata stays null. The capsule uses the existing projection/readers and existing built-in test dependencies; no new runtime library is introduced.

This is a metadata/graph slice, not a full Source Snapshot/public registry/native-envelope law. The Native held laws independently require complete typed owner equality and real public Binary/Text routes; those cannot be inferred from this Source slice. Register/mount both closed fixture/schema pairs before this test. Any shared-file guard must be rechecked after concurrent test work.
