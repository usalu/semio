# Logical Materialization Contract

## Superseding acceptance rule

The artifact schema, snapshot, diff, and mutations must not retain an imported file image, physical byte ranges, lexical byte records, or any equivalent byte-replay side channel. Deserialization consumes native bytes into a lossless logical format model. Serialization materializes native bytes from that model through the ordinary writer.

Permitted binary values are only genuine artifact content, such as PDF stream values after their declared filter semantics, MP4 media samples, embedded presentation media, and typed DWG entity values. Encoded pages, compressed ZIP members, encrypted headers, padding, token tapes, original whitespace bytes, complete unsupported sample-entry boxes, raw unknown-box bodies, raw XML/STEP/PDF syntax strings, and whole unknown file regions are not permitted merely to reproduce the input.

## Required flow laws

1. Native bytes deserialize into the logical snapshot.
2. Snapshot pack and DSL codecs preserve the logical snapshot.
3. Empty diff, no mutation, effective mutation, inverse, and absorbed diff preserve or intentionally update the logical snapshot.
4. Native serialization uses only the resulting logical snapshot.
5. Native output is compared byte-for-byte with the original fixture.
6. Tests must fail if a source/physical replay field is reintroduced.
7. Acceptance compares every route with the imported fixture itself; comparing with a first canonical export is forbidden.

## Information-theoretic constraint

Several standards admit more than one valid byte encoding for the same logical artifact. Examples already demonstrated by the fixtures and specifications include DWG LZ match selection and page padding, STEP numeric spelling and comments, XML quote/whitespace choices, PDF object ordering/number spelling/xref representation, ZIP compression streams/timestamps/extras, and ISO-BMFF box layout. A byte-free logical model can reproduce an arbitrary imported byte image only when either:

- every byte-affecting distinction is a modeled standard value rather than discarded representation state; or
- the imported file already equals this implementation's deterministic canonical serialization.

The implementation must therefore expand the typed logical model wherever a distinction is genuinely part of the standard model, use deterministic canonical materializers for representation-only choices, and report the first irreducible mismatch honestly. Hardcoded fixture output and hidden byte retention are forbidden.

## Rejected implementations

- `ArtifactSource { bytes, semantic_blake3 }`
- `PdfPhysicalLayout` token bytes
- `XmlLexicalDocument`/generic token tapes
- `Part21PhysicalFile` token tapes
- `PptxPhysicalState`/ZIP physical replay state
- `DwgPhysicalSnapshot` encoded fragments
- `Mp4PhysicalFile` box-fragment replay state
