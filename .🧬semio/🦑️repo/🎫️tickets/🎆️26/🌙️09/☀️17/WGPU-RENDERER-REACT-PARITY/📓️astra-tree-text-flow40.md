# Tree Text Reading Order 40

The live React Projection pane contains canonical labels `1-Point`, `2-Point`, and `3-Point`. Its rendered Tree label buttons inherit `direction: rtl` from the pane, and their text spans have no independent reading-direction policy. The paired browser review showed `Point-1` in React and `1-Point` in WGPU. The current WGPU target paints these Latin labels in source order; the newer shared text subsystem also separates Unicode paragraph direction from physical layout.

A neutral English/German fixture and actual Chromium glyph-position oracle are staged under the Tree element. The test mounts the real TreeItem, loads the production stylesheet, and checks twelve label/direction combinations. It requires the label container to retain the panel's layout direction while its visible text preserves the authored Latin order.

The fail-first run completed: 1 passed, 1 failed, Nx exit 1. Chromium measured `Point-1` for the `rtl:en:1-Point` case, reproducing the live discrepancy. The stylesheet now applies Unicode plaintext paragraph resolution to the Tree label and its immediate content, preserving the outer RTL placement. The follow-up completed with 2/2 tests passing, Nx exit 0, 27.72 seconds. All twelve actual browser text-order and inherited-direction comparisons passed. The live app has been reloaded for a physical follow-up.

The live React app was deliberately reloaded and Projection was opened. Its buttons still report RTL layout, while the label and text span report `unicode-bidi: plaintext`. The screenshot visibly reads `1-Point`, `2-Point`, and `3-Point` in the authored order. The console has no errors and records the fresh `[DEBUG] react-world-frame` publications for both visible windows.

This packet covers the observed bilingual chrome labels only. It does not establish full Unicode shaping parity for the current WGPU per-codepoint text painter.
