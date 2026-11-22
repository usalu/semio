# Retained guided straight Pipe

A guided Pipe now retains the cold operator's placement law for a planar line face, one open line path and one open line guide. The two guide endpoints must project to the same nonzero ray in the plane perpendicular to the directed path. Every closest point on that straight guide therefore gives the same in-plane orientation at every path station. The retained planner uses that ray as the frame's x-axis, keeps the frame origin on the path and maps the profile into the first station. The prism draft transforms each admitted source corner and preserves the existing bounded validation and publication sequence.

The neutral fixture starts with a 2 × 4 rectangle at the world origin, a path starting at (10, 20, 30) and a guide along +Z displaced in +X. The output has six faces, bounds (10, 18, 30) to (14, 20, 33), and volume 24. Changes to guide direction produce different geometry even though the path is the same. The retained Flow test checks the funded preview, exact volume, opposing edge uses, and refusal of coincident and direction-changing guides without a live result.

- Three.js independently computed the orientation from the closest guide point and Manifold independently measured the hull. All three portable cases and 24 assertions passed.
- The linked cold production WASI Pipe passed the same neutral guided fixture and reproduced its exact envelope and volume.
- Source preflight passed on WASI, macOS, Windows and Linux, with Engine at 4,000 active lines and Session within its byte limit.
- The previous broad test unit failed source admission at 272,067 bytes against a 262,144-byte limit. The straight sweep, unguided Pipe and guided Pipe regressions now share the dedicated `original-straight-sweeps` integration target. Its standalone source closure passed admission on the four targets at 4 files and 35,805 bytes. The managed native test exited 101 after automatic cleanup and retry. The compiler image still had 5.6 GiB free of its 96 GiB budget, with other running lanes holding the space. The funded retained witness requires a current compiled registry artifact. [Linked witness input](./🐍️linked-guided-pipe.py)

This route covers fixed, nonzero guide direction. Curved or multi-edge path and guide wires, varying or degenerate guide frames, and conic or holed guided profiles require other retained construction paths.
