# Playbook Generation First-party Codec Prerequisite

The actual Forms13 owning compiler reported four E0277 errors at FormGeneration's values field and Serde derive. The live declaration is FormGeneration, not PlaybookMutationPayload. PlaybookValues remains the actual first-party OrderedMap<DslValue>; no external serialization traits or map bridge were added.

The three generation model types retain their existing ToValue/FromValue derives and authored value tags/defaults. Their stale Serde derives/attributes and GenerationPlayRoot's Serde forwarding were removed. The existing first-party JSON codec is the actual declared consumer boundary.

Nine actual Rust paths were paired. Flow's generation producer now returns ValueError on malformed declared JSON. A genuinely uninitialized window whose generation text is empty still explicitly constructs its authored default state; parsing failures never enter that branch. The decoded state is guarded and explicitly retired through the existing GenerationPlayRoot retirement path. The editor preserves the typed cause into its existing Fault terminal; three UI assembly terminals deliberately extract the message into their declared PluginAssemblyError. Generation encoding uses the first-party codec. The existing generation and Flow fixtures retain their exact wire assertions, alias sharing and byte-bounded retirement obligations; Serde JSON remains only the independent test output oracle.

Parser result: 9/9 exit 0. No Cargo or owning assertion execution was performed in this lane. This does not prove numerical ownership, cancellation, full cleanup allocation, or the first-party codec's compiled behavior in these consumers. In particular, the existing retirement path may allocate its shared root metadata; no allocation-free cleanup claim is made.

Paired parser roster: 🗑️generated/playbook-generation-firstparty-codec-paired-parsers.json.


Current owning follow-up found one newly concurrent orphan Serde field attribute. It and its two unreferenced Serde map helpers were removed after verifying the actual whole Rust symbol census had no other consumers. Fresh one-file parser exit 0; the first-party typed generation authorities and remaining top-level Playbook Serde models are preserved.
