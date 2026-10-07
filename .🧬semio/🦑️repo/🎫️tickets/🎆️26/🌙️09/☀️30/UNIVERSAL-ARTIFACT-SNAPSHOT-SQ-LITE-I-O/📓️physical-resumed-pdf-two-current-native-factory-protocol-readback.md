# PDF Native Factory Hash Authority Readback

PDF's two executable native factories use ArtifactCodec::of for their actual typed snapshot/mutation owners. The actual current store constructor derives pack_schema_hash from P::record_spec through os_pack::schema_hash (store11935), not from SHA256 of protocol text. Therefore the recorded protocol SHA256 comparisons are informational and do not establish stale metadata; no binding repair is warranted from that comparison.

The exact existing PDF runtime law sqlite_snapshot_pdf_native_factories_publish_exact_structural_hashes compares both factory schema identities and actual structural hashes to metadata. The original whole-PDF replay includes that law. Current protocol identities, byte lengths, and definition bindings are retained in physical-resumed-pdf-two-current-native-factory-protocol-readback.json, with this authority clarification. No factory or runtime qualification is claimed from readback.
