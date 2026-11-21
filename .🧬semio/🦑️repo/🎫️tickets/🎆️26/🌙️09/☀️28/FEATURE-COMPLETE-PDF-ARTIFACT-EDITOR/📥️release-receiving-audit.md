# Release Receiving Audit

- The native decoder terminal transfer at `🚪️io/📥️import/🔁️controlled/🦀️.rs` charges `Phase::Publish` at most one `PdfSnapshot` body per work unit. `publish_snapshot` also takes a `PreparedPdfFile`, takes the snapshot, transfers the graph objects and trailer, and returns the snapshot after the receipt was consumed. Quote and pay for all inline transfers or split them into paid phases; validate through the mounted receiving contract.
- `controlled-font-release.py::apply_reviewed` preflights all paths but writes sequentially. A later failed preimage assertion leaves earlier files applied, and the check/write interval can race a shared writer. Coordinate release and verify the entire roster afterwards.
- The staged inspector progress control and embedded-font role implementation were inspected; the mounted cancellation and product receiving gates remain pending. The source-only probes do not prove either.

## Receiving Grant Follow-up

The portable physical contract (`🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs`) bounds copied payload bytes separately from capacity and release. Jack’s `🚪️io/💾️binary/📸️snapshot/🫴️receiving/🦀️.rs::take_into` charges one snapshot body for direct candidate transfer. For PDF’s two distinct owned bodies, the logical-body quote is `max(COPY_PER_WORK, size_of::<PreparedPdfFile>() + size_of::<PdfSnapshot>())`; do not count the transferred graph and trailer again as independent bodies. This is an inferred logical quote, not a proven exact bound on compiler-generated physical copies. Natural-file `adopt_genesis` uses twice the sum of three option-slot sizes for its staged transfer and restoration.

Test the real controlled receiving context with a copy grant one byte below and exactly at the chosen quote: the first must yield without moving either owner, the second must complete and report the charged receipt. Check the target-specific sizes and actual maximum grant on every supported target; source inspection alone does not establish that the quote fits the mounted 32 KiB limit.
