# SVG Ellipse Export Parity

The actual Three SVGRenderer emits a centered view box (`-width/2 -height/2 width height`). React's ellipse export helper used `width/2,height/2` as the clip center, putting the mask in the wrong coordinate space. An opaque scene background is a style on the outer SVG and was also outside the clipped group. Empty background-only SVG output can serialize as a self-closing root, which the helper previously returned unchanged.

Added a neutral fixture for Three-centered, positive-origin, shifted/scaled, and implicit view boxes. The oracle renders actual Three background output, calls the production React helper, and rasterizes the resulting SVG with Sharp. The original helper failed the suite. The repaired helper passed both tests (Nx exit 0, 8.7 seconds), including center RGB/alpha, four transparent corners, exact clip coordinates, and idempotence.

The helper now derives the ellipse from the declared view box, accepts a self-closing SVG, and moves the background fill inside the clip. Other styles and content remain. The upcoming native SVG serializer should produce the same geometry and alpha contract. No WGPU SVG export runtime exists yet.
