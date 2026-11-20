# WGPU SVG Preview Transport Recovery

The same pane request can make four additional GLB attempts after a temporary transport or request-capacity fault. Retry delays are 250, 500, 1,000 and 2,000 milliseconds after each failed attempt. The host frame scheduler wakes at the earliest active retry deadline. The waiting pane keeps its raster key and displays its rendering status. Closing or replacing it removes the old retry deadline, and each GLB attempt reserves its own token even when another pane requests the same URL.

The shared renderer handoff carries the transport’s retryability classification. Decoder rejection and permanent transport refusal remain terminal. The retry budget resets when the mesh arrives successfully or the pane request changes. Export jobs still report an asset failure through their existing job outcome.

## Verification

- The focused Rust probe compiled and ran on the macOS native target and on `wasm32-unknown-unknown` using the actual pane and icon asset modules. The probe substitutes the raster and renderer I/O boundaries and covers deadline parking, exhaustion, cancellation, and terminal transport and decoder rejection.
- The native probe produced cumulative attempt offsets of 0, 250, 750, 1,750 and 3,750 milliseconds. The neutral retry fixture and an independent RxJS virtual timer produced the same offsets.
- The icon-host test closure measures 4,000 native active lines and 3,789 browser target active lines. The asset-authority test closure measures 1,032 and 974, respectively.
- The repository test covers real asset reservation, owner return, retirement, dormant retry scheduling and cancellation. The asset-authority regression checks that same-URL sibling loads receive distinct tokens and stale rejection cannot reach the replacement. The full Cargo and browser dev runs are tracked in the parent ticket.

The retained validation inputs are [the Rust probe](svg-retry-probe.rs) and [the RxJS comparison](svg-retry-oracle.cjs). Compile the probe by passing its path to `repo tools exec --light rustup -- run nightly-2026-07-20 rustc --edition=2021`. Use the ticket’s generated directory for the executable and `--target=wasm32-unknown-unknown` for the WebAssembly variant. 
