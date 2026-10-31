# Internal Schema Reference Census

Read-only current catalog document membership: 3987 documents, 11007 internal reference strings, 0 missing pointer targets and 0 unreadable/unbounded documents. This reproduces the existing harness whole-JSON reference observation, including literal/example objects; it does not presume those are schema evaluation nodes. No Source was changed or native producer invoked. Raw metadata: 🗑️generated/schema-internal-reference-census.json.



## Reference Repair Evidence

The original census at2026-10-29T20:54:44Z observed3987catalog documents,11008internal refs and exactly one missing pointer: BCF2.1markup SetParts definitions.part.properties.data -> #/definitions/octets. Read failures0. The current scope already depends on the byte payload semantic: its snapshot private octets helper exactly equals existing public framework.value.bytes/Octets.

Actual independent Ajv compile failed0passed/1failed/2assertions before the edit. Replaced only that single ref string with https://json.schemas.assets.semio-tech.com/framework/value/bytes/schema.json#/$defs/Octets, preserving the existing aggregate discriminator and payload structure. No private helper was duplicated, no other scope private definitions were addressed, and no format annotation/guard was relaxed. Current prepared owned validator and Ajv agree on11neutral tagged-payload cases. Including actual current catalog dependency resolution, green2tests26assertions0failures.

Fresh canonical generate49454 completed0 with3630scopes/11494diagnostics and derives the genuine framework.value.bytes dependency. Follow-up census at2026-10-29T21:00:16Z observed3987documents/11007internal refs/0missing targets/0readerrors. It verifies current catalogued internal pointers, not full schema readiness or literal-schema-node semantics. Docs14344 remains separately pending. Test inputs and report history remain in the ticket.
