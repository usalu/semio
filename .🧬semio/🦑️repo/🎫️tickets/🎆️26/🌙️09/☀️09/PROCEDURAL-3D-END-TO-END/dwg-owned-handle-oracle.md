# DWG owned handle independent oracle

Read-only source inspection; no build or edits to production/cache.

The independent acadrust 0.5.5 source in the local Cargo registry exposes io::dwg::dwg_stream_readers::bit_reader::DwgBitReader. Its public new(Vec<u8>, DwgVersion, DxfVersion) is at bit_reader.rs:51 and read_handle_relative(base) / read_handle_reference at :624–687. The nearby handle_reader.rs::read_handles is a different Handles-section delta-map reader and must not be used as an oracle for object-reference encoding.

A compact byte-aligned fixture with base 0x100 and owned_count 3 is [0x21,0x42,0x60,0xa1,0x03,0xc1,0x02]: the first absolute soft-ownership handle is 0x42, +1 resolves 0x101, +3 resolves 0x103, and the final -2 is SEQEND 0xfe. Repeatedly call the acadrust read_handle_relative with the same original base and compare each production read_object_handle and new retained owner cursor; check the terminal handle is not published as a vertex. Include 0x80 (-1) and larger big-endian offsets as extra valid cases. Drive zero/sub-demand/tiny grants, interruption between header/payload bytes and before SEQEND, and source/cancel/close cases against the production owner.

Restrict this independent equality comparison to valid nonzero references without arithmetic overflow: acadrust uses wrapping add/sub and clamps byte count, while production fields/🦀️.rs:255–269 uses checked arithmetic and treats zero as None. Test malformed/null/overflow independently against the production cold behavior rather than treating acadrust as the rejection oracle. Distinguish the object reference stream from section handle-map CRC/delta fixtures.

The local acadrust source is present but the sampled surviving compiler lane has no acadrust rlib or completed selected DWG production crates. Compiling the full bit_reader file also requires its crate modules, encoding_rs and types; no standalone linked test was run. Existing acadrust-generated full drawings are in DWG export/🧪️tests/🔮️acadrust-oracle/🦀️.rs:153–201 and should be used as the end-to-end proof when owned compiler capacity returns.
