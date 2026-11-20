# Mounted PDF Natural Import Audit

Read-only source review of the mounted 1.7 decoder and `NaturalFileImportJob`; no build was run. Files may change concurrently.

## Findings

1. **Grant exhaustion can be reported as a stalled import.** The controlled decoder returns unchanged progress when items, depth or copy grant is insufficient. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs:255` rejects unchanged progress unless the context is yielding or cancelled. A grant too small for a native allocation or release also produces an immediate media error on the first work unit. Exercise a real mounted job with depleted typed grants and ensure the decoder signals a valid yield or a demand instead of returning successful zero progress.

2. **Valid large files requiring the reference path are rejected.** The controlled decoder returns a media error on `ReferenceRequired` or a deferred semantic lift. The synchronous `decode_pdf_with_password` instead falls back to `read_pdf_reference` when the retained byte graph cannot handle the structure. For example `🔨️modules/🔗️xref/📥️retained-graph/🦀️.rs:763` explicitly requests the reference path for inherited xref trailers. A large incremental-update PDF can therefore fail solely because it crosses the work-unit threshold. Add a mounted regression for this structure and implement bounded handling before claiming feature parity.

## Ownership and limits checked

The admission function moves the original byte vector into the controlled owner; the job moves a completed snapshot to `pending_snapshot` before validating progress. Publication moves graph objects and the prepared trailer into that snapshot while the source remains in the decoder for incremental retirement. The native close path also retires partially built snapshots and the remaining graph. These paths require cancellation and completion tests with actual job grants; this review alone does not establish accounting correctness.

The contract `max_decoded_items` is passed as the graph maximum object count, and `max_output_bytes` as its maximum stream bytes; the job separately checks reported decoded items and owned output bytes. Fixtures must use bounds high enough for all graph objects and live retained output, not merely page count or source file size. The decoder reports retained output only at completion, so intermediate limit and high-water behavior deserves explicit coverage.
