# PDF natural-file decoder staging audit

Attempt 17 (driver `wiwHwRI`) reached `semio-s-artifact-stdio-pdf-mut-family-11` in the genuine Note guest build. Its captured sources produced two E0599 errors for `NaturalFileDecodeStep::Yield` and one E0277 for the unconstrained retirement helper. The stage log recorded a concurrent edit to the controlled importer during its retry and ended with stage exit 75.

The current shared sources contain both corrections:

- The plugin declares `NaturalFileDecodeStep::Yield` at line 25 of `📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs`. The host import job maps it directly to `JobOutcomeBorrow::admit_yield` at line 336 of `📜️declaration/📥️natural-file/🦀️.rs`. It separately validates bounded, monotonically increasing progress and exact final input consumption.
- The PDF importer bounds `retirement_birth<T>` by `semio_framework_value::RetireOwned` at line 357 of `🚪️io/📥️import/🔁️controlled/🦀️.rs`. Its two callers supply `PdfSnapshot` and `PdfPage`, which both derive `RetireOwned`.

## Budget and guest target

The PDF 1.7 editor registers both controlled-decoder constructors. They select the controlled decoder for sources larger than `max_work_units_per_step`. Mounted admission checks the host copy, capacity, item, and depth grants before moving the source. Each controlled turn checks cancellation and yield, respects the contract work limit, and uses one-unit cursor grants. The allocation grant is the lesser of remaining capacity and remaining copy bytes after reserving the step's copy charge. The cursor receives the remaining release grant. The decoder charges each receipt back to the host for fuel, copied data, allocated backing, and released backing. An allocation or release refusal with no work yields on the first occurrence and reports a media error if the same demand and grant recur. Controlled reference-only PDF structures return a media error.

The active PDF mutation crate declares its plugin and value dependencies as general dependencies. The PDF importer includes the controlled module without a target-specific condition. No additional guest target compile gate surfaced in this source review. The existing plugin fixture `natural_file_controlled_decoder_crosses_turns_and_retires_its_owner` covers turnover and owner retirement. Attempt 18 supplies the fresh guest compile check. This audit made no source changes and started no competing Rust build or test.
