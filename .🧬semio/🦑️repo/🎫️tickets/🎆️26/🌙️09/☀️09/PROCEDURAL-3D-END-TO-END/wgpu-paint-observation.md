# WGPU Paint Observation

The world renderer maintains per-instance draw hover flags in world host sync-and-overlay Rust lines 416–426, but the worker mesh-stats export reports producer selection state rather than those flags. No existing browser export of the flags was found.

The ticket WGPU journey now clips actual browser-composited screenshots to the mounted procedural-preview rectangle, before graph hover, during the exact shell@solid port hover, and after leave. The PNG pixel oracle requires at least 20 pixels with an RGB delta of at least 24 on both transitions, overlap of at least half the changed pixels, and restoration within a bounded drift. The exact graph/guest-state checks remain prerequisites. It saves the three PNGs and records counts in bidirectional-hover.json. The oracle is a reversible preview-region pixel witness, not a per-instance draw-ID proof: other simultaneous changes within the crop can contribute. A threshold failure should be inspected with the saved frames and may indicate rendering or test environment instability.

The pngjs encode/decode synthetic oracle passed reversible, unchanged, one-way and drift cases; all four combined watcher and pixel tests passed, the modified probe passed node syntax check, and Bun resolved the PNG decoder. No mounted session, build or watcher restart was performed, so real WGPU paint remains unverified.
