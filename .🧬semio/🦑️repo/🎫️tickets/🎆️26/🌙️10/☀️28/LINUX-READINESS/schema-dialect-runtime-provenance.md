# Remaining Dialect Runtime Provenance

Read-only Source references; no evaluation, Native access, cache inspection, fixture invention, or Source edits.

## Accepted Serialized DATA

Trinity graph/io/🧪️tests/🧫️fixtures/📦️archive.json contains complete serialized positive values at /cases/0/json and subsequent cases. Its 🧪️tests/🟦️.ts line10 encodes the authored bytes, checks exact equality with this text, JSON-parses it, and explicitly Ajv-validates the resulting object. An explicit closed JSON-text decoding witness can consume this authentic carrier; a plain JSON pointer alone yields a string rather than the schema object.

Build graph/generations/🧷️pin/🧪️tests/🔣️.json contains a complete serialized Pin at /accepted/0/text. Its tests/🟦️.ts lines39–40 invoke the real reader without rejection and compare independent Python acceptance. The encoding tests at31–35 additionally construct full pins through encodeGenerationPinV1 from authored fixture fields, preserving coverage encoding semantics. Do not equate the encoder input object with the wire Pin object.

## Existing Construction Proofs

Build graph/generations/🔐️custody/🧪️tests/🟦️.ts lines26–27 declares the complete Claim with version,24-character token,ticket,owners,structural,state,queuedMs, explicitly Ajv-validates it and requires the real reader to return the same claim. This is a direct authored positive literal; it need not be invented from the schema.

Build graph/generations/🧪️tests/🟦️.ts lines322–340 seed ringFiles(3), refresh the existing createWorld fixture and obtain complete Pointer and Manifest objects. The valid mutation case compares real-reader acceptance with Ajv for both; lines337–338 read the referenced p0 Fragment and explicitly assert Ajv and reader acceptance. The Manifest owns the complete OwnerRow at manifest.owners.p0. Reuse the existing fixture constructor only through its normal authenticated test route; do not evaluate copied Source snippets or inspect raw cache data to obtain these values.

Test/storage/🟦️.ts emits complete Run metadata under its original test-store publisher: version, task identity, and basename(root). The schema is not shown unsatisfiable by the missing checked-in JSON corpus. No independent existing explicit positive witness was established in this review for Run or build authority.json, so those remain excluded.

## Verified SQLite Schema Drift

OS plugin/🧪️tests/🧫️fixtures/🪶️sqlite/🔣️.json has dialect,encoding,payload,limits,native. The current sqlite JSON schema is closed and declares only the first four. Its sibling 🦀️.rs SnapshotInput at76–82 requires native:SnapshotGrant, and the actual refusal/native tests deserialize the five-field neutral request. Consequently the authentic current request fails this schema because the schema omits a required runtime authority axis. This is real schema/runtime divergence; dropping native to force a witness would hide it. Keep this candidate excluded until the owning schema is corrected and independently checked.

Energy metadata provenance remains generated/schema-dialect-energy-witnesses.json. No impossibility claim was established for any remaining target.

## Three Current SHA-Bound Carriers

Generated/schema-root-authored-carriers.json records current exact schema and fixture SHA values plus pointers for Energy birth metadata, Trinity archive serialized JSON, and accepted Pin serialized JSON. Energy supplies a direct object at x-semio-energy-row-birth. Trinity and Pin require one explicit JSON-text decoding step after pointer resolution; decoded keysets match their existing authored wire carriers. The referenced tests already establish their provenance, but no dual-validator proof was executed by this review. Target Pin is definitions/Pin, not merely its unconstrained module root. The output contains references only, no copied Source payload.
