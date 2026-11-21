# WGPU Map Tile Recovery and Touch

## Tile recovery

The browser and native WGPU transports now retain a raster or vector map tile's admitted request through the existing two retry waits of 200 and 800 ms. Both transports use the same eligibility rule. The browser returns partial pages and byte credits before streaming a fresh attempt under the same request token. Native transport retains the existing fixed allowance and retires partial pages before reuse. Rejections and exhausted retries still reach the map's terminal miss handling.

## Touch

A single touch tracks the original local position. At six pixels, the retained MapHost starts panning from that origin and applies the full travel. The pan remains latched. A tap picks at release. A successful pan uses the last release coordinates and a cancelled pan retains the last valid sample. Either publishes one camera action. A second contact transfers ownership to pinch and the final pinch contact settles once. Surface retirement clears both the map gesture and its contacts.

## Verification

- Vitest reproduced the original missing map-tile transport capability and passed all 64 browser frame transport tests after the fix.
- The adjacent worker step budget suite passed 18 of 19 tests with the pinned Playwright Chromium. The remaining presenter assertion expects the frame condition to begin with the presenter test. The source already includes the icon preview condition at HEAD.
- Added Rust coverage for exact shared-owner reuse and for tap selection, normal pan, cancelled pan, pinch handoff, and final camera settlement. Pinned rustc parsed the modified Rust files. The scoped Cargo test stopped with E0433 before running WGPU tests. World terrain parsing referenced the unlinked \`semio_framework_pack_json\` crate twice at \`♾️infinite/🌍️world/🧱️scene/🦀️.rs:152\`. A concurrent change added the direct dependency and updated its \`Cargo.lock\` entry. The dependency path resolves to the expected package. The one-job rerun started with the repaired graph and was cancelled while still compiling dependencies to free memory for the GIS browser runtime build. The scoped Rust tests remain unexecuted.
