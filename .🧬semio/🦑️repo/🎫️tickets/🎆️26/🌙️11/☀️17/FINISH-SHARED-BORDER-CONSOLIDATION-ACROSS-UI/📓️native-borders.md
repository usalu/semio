# Native Border Verification

The wgpu event router applies the resolved hover state to the innermost interactive control and any enclosing overview cards that opt into hover. A change to a card’s hover state invalidates its parent card stack, which emits the card’s outline. The stack gives each outline its own state and color and submits coincident edges to the shared resolver.

The native presence painter, retained tree rows, and reference tree rows use one effect painter. Introduction and celebration draw at the theme hairline while their accent animates. The terminal renderer has no overview card stack treatment or corresponding variable border thickness.

In a shared cap/footer band, the painter uses the current pointer position and tests it against the same stepped polygons it draws. The upper footer, lower title, and gap each produce their own hover result. The router invalidates the stack when the pointer moves through a cap or footer band, including moves that keep the same hit leaf. Both retained frame entry points supply the pointer in the drawing coordinate system. The painter uses that point only for stacks in the routed hover path.

Action routing continues to use accepted node rectangles. Card chip widths come from the live atlas, including fallback glyphs, so the visual hit reuses the measured outline already available in paint.

## Checks

The wgpu event laws cover card and control hover ownership, repaint invalidation on moves within one target, and clearing the pointer on exit. The paint laws check moving between the interlocked chips and the gap, preserving outline positions and stroke, keeping obscured stacks at rest, and holding animated borders at a fixed width throughout the cycle. An independent `polygon-clipping` check used the shared interlock vector and confirmed ownership of the lower title, gap, and upper footer.

The repository Cargo command could not select the companion suites from the root workspace. The repository target runner was waiting on the project graph. A native Cargo harness used the original companion modules, their dependencies, and fixtures, and passed all 92 event laws, 73 paint laws, and 27 light-renderer laws. The stack tests select vertically aligned cards from the shared interlock fixture, which also includes horizontal neighbors for the Web compositor. The `semio-ui-wgpu-engine` build passed with both retained frame entry points.
