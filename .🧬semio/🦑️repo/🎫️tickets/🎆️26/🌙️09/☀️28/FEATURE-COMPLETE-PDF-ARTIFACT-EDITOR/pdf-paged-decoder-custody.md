# Paged PDF Decoder Custody

The proposed PDF decoder takes the exact sealed `NaturalFilePagedSource` work owner and reads it through the shared byte-source interface. Contiguous input still uses the same native decode path. A single PDF constant sets the paged selection and decoder admission extent to 16 MiB. The unrelated raw-wire command allowance stays with the factory that owns that wire. The staged conformance case uses a one-byte raw-wire allowance and an actual 963-byte two-page PDF in paged custody, then compares its decoded snapshot with contiguous input. A denied first grant leaves the paged source untouched. The normal job publication and physical source retirement remain paid.

The prepared reader already processed the original 9,437,877-byte classic PDF, the resource stream and compressed xref fixtures from segmented storage. The decoder source test linked the exact staged borrowing/retirement adapter to the repository Value library and compared all selected bytes in 4 KiB reads. It exposed that the existing generic PagedBytes retirement accounts each octet as its own item. At the mounted 64-item grant this requires more than 147,000 payload turns for the fixture alone. The new octet retirement stage keeps the generic list contract and charges one paid item per bounded physical leaf operation for PagedBytes. The existing page release continues to retire actual backing through its original effect. The focused native law used the production paged list and retired the exact fixture in 2,305 leaf operations and 4,766 backing releases, returning 10,405,992 physical bytes. Its zero-item grant left the source untouched and its generic list test retained one element per paid item.

Mounted publication also requires the separate paged reserved-job contract. The generic import-media value of 8 MiB cannot retain the fixture's 9,437,184-byte uncompressed image. The new purpose-bound factory uses 32 MiB output and passed its focused Rust, Ajv and independent SQLite checks. The visible editor still hands the completed result to the host archive, which has a separate durable format limit.

- [PDF decoder custody and conformance stage](📜️pdf-paged-decoder-stage.ts)
- [Prepared PDF graph verification](segmented-pdf-preparation.md)
- [Existing decoder source close law](📜️pdf-paged-source-law.rs)
- [Octet retirement stage](📜️paged-octet-retirement-stage.ts)
- [Paid octet page-close law](📜️paged-octet-retirement-law.rs)
- [Shared SDK publication handoff](natural-pdf-sdk-publication.md)
