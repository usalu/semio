# Icon PNG Export Bytes

The neutral PNG export fixture now declares opaque, half-alpha, quarter-alpha, one-alpha, and transparent framebuffer pixels. The oracle creates actual Chromium WebGL framebuffer bytes with premultiplied alpha, publishes the Canvas PNG, and decodes it with Sharp. Both schema validation and exact output comparison passed (2/2, Nx exit 0, 7.0 seconds).

The WGPU Icon export encoder now normalizes at most 1,024 pixels per step and then advances the existing first-party PNG encoder. It retains a real phase/progress witness, rejects invalid dimensions and byte lengths, discards cancelled candidates, and transfers complete PNG bytes once. Native laws cover the same pixels, bounded progress, cancellation before normalization/during normalization/during encoding, and early or duplicate publication. These Rust laws have not run yet; Native37 has been launched with them included.

The icon export effect is still not connected. The remaining work is the owned asset/scene packet operation, offscreen presentation/readback, SVG serialization, ordered item delivery, cancellation/progress UI, and app runtime verification. This is an encoder checkpoint, not a completed export feature.

## Native Gate Admission

Native37 stopped before compilation because the new internal pixels dependency was incorrectly declared as a workspace dependency. It is path-only in this repository. Corrected the manifest to its direct internal path and launched Native37b; its compilation and test outcome are still pending. No native test was claimed from Native37.
