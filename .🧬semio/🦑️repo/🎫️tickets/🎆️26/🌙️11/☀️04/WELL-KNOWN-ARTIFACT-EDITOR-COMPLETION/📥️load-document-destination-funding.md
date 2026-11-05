# LoadDocument Destination Funding

Current read-only source finding: a funded copy of a committed Pack into LoadDocument does not make destination typed decoding bounded.

Plugin 🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs:243 builds RetainedPersistedDocumentHydration::from_pack during HydrateParent, after retained SPR history decoding. It passes the exact app document owner catalog, Pack, decoded history, expected artifact identity/dialect, schema, and current operation/generation. Later owner.step(cx, maximum_items:1) drives hydration.

Store 🧾️document/📜️history/💧️hydration/🚶️hydration-step/📦️pack-scan-and-fold/🦀️.rs:
- step_scan_pack chunks hashing using available fuel and OWNED_SCHEMA_DECODE_PAGE_BYTES.
- step_decode_pack directly calls P::decode_pack on the entire retained Pack, assigns the typed result, changes phase, then consumes one fuel.
- There is no NativeDecodeControl or original receiver admission in that decode function. The chunked scan is not typed parsing.
- The route uses P:ArtifactPack + ArtifactStoreSnapshot + StoreState, not OriginalArtifactPackReceiving. Missing owned DSL bindings do not produce an Unsupported fallback here; they are bypassed by the cold ArtifactPack call.

A separate member decoder at store/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🦀️.rs:14 likewise implements MemberSnapshotDecoder via P::decode_pack. It is not evidence of bounded native receiving.

No TypedSnapshotSpec or RegistryCodec symbol was found in current searched framework sources. Actual registry is DocumentCodecRegistry/ArtifactCodec in store/📦️codec/🦀️.rs. Naming those nonexistent types would conceal the concrete cold hydration route.

## SPR identity

Plugin 🧵️instance/🏪️stores/📥️staging/🧵️instance-root/🦀️.rs:23 stamp_load_document_identity stamps the replacement SPR with the current store artifact ID, A::DOCUMENT_SCHEMA, dialect, and owner. This permits example replacement to retain current document identity. It invokes stamp_document_spr_identity synchronously through resolve_ready. It is not permission to pass empty SPR bytes: begin_document_archive_load rejects empty parent Pack or SPR at plugin-app-impl/🗃️document-load:40, and hydration Begin requires composition identity.

An actual empty history encoded by empty_document_spr is distinct from an empty Vec. Its identity stamp and any required allocations also need genuine funded custody in a complete retained producer/consumer design.

No build, native test, browser run, or source edit was performed. Static Pack chunk-copy can remove redundant producer parsing, but destination step_decode_pack remains a concrete funding gap until replaced by a real bounded receiver.
