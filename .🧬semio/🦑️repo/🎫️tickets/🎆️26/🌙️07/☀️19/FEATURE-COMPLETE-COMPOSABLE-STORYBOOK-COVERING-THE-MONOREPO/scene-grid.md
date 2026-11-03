# Two-Sided Scene Floor Grid

The actual Drei Grid defaults to Three.BackSide. Its shader projects plane vertices onto XZ, so this intentionally one-sided material disappears when viewed below the floor.

Scene now explicitly supplies `side={THREE.DoubleSide}` without changing grid visibility, colors, fading or camera controls. Added a real `BelowFloor` Scene story with a camera below the grid.

Language-neutral camera/visibility vectors: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎬️scene-grid/🔣️.json`. The focused test parses Scene's actual Grid prop and independently checks above/below intersections with Three's real plane geometry/material/raycaster. Before the fix: one passed, two failed (explicit surface contract and below-floor visibility). After: three passed.

Browser/WebGL acceptance is recorded in the final visual repair report; this focused oracle is not itself a pixel-rendering claim.
