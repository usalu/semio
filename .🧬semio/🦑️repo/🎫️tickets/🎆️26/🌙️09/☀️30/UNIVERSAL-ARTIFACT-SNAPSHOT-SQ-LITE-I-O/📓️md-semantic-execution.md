# CommonMark Semantic SQLite Execution

The individually handwritten27-table schema covers the complete canonical model: seven block and nine inline variants, explicit ordered ownership for documents, quotes, list items, blocks, emphasis, strong and links. The retained language-neutral fixture covers every variant, fullu8/u32 fields, custom schema identity, absent/empty optional strings and empty collections. Native and TypeScript providers traverse1024-level block and inline trees iteratively. SQL contains semantic entities and relationships; typed I/O does not invoke native encoding or decoding.

## Test-Driven Repairs

The actual native test-first gate failed15 missing-provider trait/method errors. The initial TypeScript stub failed all four SQLite laws. A fifth schema/parser law then exposed an undefined parseMdBlock in the existing canonical parser. The authored block/inline parsers now traverse iteratively, reject cycles and validate the complete schema. Heading JSON bounds were corrected to nativeu8 and list starts tou32. The public package initially reported TS2769 on an inferred JSON fixture kind:string in the test assertion; the expected fixture assertion now uses the canonical model type after independent Ajv and owned-parser validation.

## Final Verification

Fresh registered @semio-tech/framework-rs:test-snapshot-sqlite-md passed5/5native laws (nextest46969d62-2dd2-4b4b-b166-2e2c6bbac2ee), followed by5/5TypeScript laws and32 assertions. All three prerequisites ran; the entire gate took8.8s with cache skipped. Coverage includes independent Bun SQLite integrity/FKs, SQL domain joins and edits, malformed subtype/ownership/ordinal/width refusal, resource/cancellation bounds, actual registered typed I/O preserving otherwise non-wire-representable state, and1024-level SQL/tree traversal.

Fresh @semio-tech/stdio-md:test passed in4.4s with cache skipped: seven built outputs, seven public exports, independent emitted-declaration consumer compilation, owned-suite typecheck and the same5/5TypeScript laws with32 assertions. Exact literal TypeScript DDL equals the authored SQL source byte-for-byte.
