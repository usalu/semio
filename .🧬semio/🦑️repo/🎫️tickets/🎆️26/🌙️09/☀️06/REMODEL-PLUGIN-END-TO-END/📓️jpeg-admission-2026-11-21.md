# Still-image JPEG admission

A valid baseline JPEG can contain seemingly valid frame markers inside an APP segment. The old Remodel decoder searched all header bytes for marker patterns and could refuse such an image or charge dimensions from that metadata instead of the actual frame. The bounded JPEG constructor now uses the same parsed frame record as the decoder, checks its pixel count before component plane allocation, and preserves the existing compressed and decoded size limits.

The neutral 8 × 8 JPEG fixture includes both progressive and oversized baseline marker patterns inside an APP2 payload. The independent libjpeg-turbo `djpeg -grayscale` decoder returned an 8 × 8 raster with every grayscale sample equal to 128. The Rust law compares those pixels and checks refusal when the limit is one pixel below the actual extent.

The initial Cargo run compiled the decoder and exposed stale test fixtures for the compressed rope and JPEG snapshot types. Both fixtures now use the current types. The complete image library suite passed all 35 tests when linked with the pinned Rust compiler against the warmed Cargo outputs. The redundant queued replay was canceled after this result. An earlier full native artifact build observed old JPEG metadata when compiling the updated Remodel caller, and the incremental artifact retry is queued on the existing native tree.

Working-resolution downscaling and media beyond the bounded decoder's limit are separate requirements that remain open.
