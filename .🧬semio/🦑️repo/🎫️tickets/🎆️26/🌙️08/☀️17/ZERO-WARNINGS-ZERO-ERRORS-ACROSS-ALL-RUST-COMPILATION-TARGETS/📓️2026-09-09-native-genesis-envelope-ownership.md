# Native Genesis Envelope Ownership

Both hash-bound native GIS and VCS codec laws reproduced a panic in native_artifact_genesis_for_editor when its temporary ArtifactEnvelope shell reached Drop with owners attached. The current print_document_pack API takes ArtifactEnvelopeOwners, and the native codec tests already explicitly consume parsed shells into that owner record. Consume the newly created shell into the same owner record before mutation, validation and serialization. This native batch path never installs the envelope into an app store or holds snapshot leases; ordinary owned data remains alive across serialization and is released on either return branch. No owner is leaked, no Drop guard is disabled, and all identity/history checks remain intact. The existing literal JSON/serde codec laws are the failing-first regression evidence; a fresh compiled rerun remains necessary.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
