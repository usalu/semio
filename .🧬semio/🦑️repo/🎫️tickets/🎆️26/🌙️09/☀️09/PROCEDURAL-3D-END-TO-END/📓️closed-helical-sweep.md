# Retained closed circular Helical Sweep

The registered `brep.sweep.helical` operator now publishes the exact analytic ring torus for a centered circular face, zero pitch and one signed full turn. The recipe uses the authored sweep axis and origin, requested path radius, and original circular trim radius. The existing torus publisher owns all native topology, rollback and preview. A sweep radius at or within tolerance of the profile radius is refused together with zero or malformed axes.

The cold sweep evaluates the helix spine at 24 stations per turn, transports rotation-minimizing frames, and rules corresponding profile sections between adjacent stations. Those pieces approximate a nonzero-pitch sweep. The current BRep curve and surface types have no exact helix and screw surface representation. The retained path explicitly refuses nonzero pitch, partial or repeated turns, and profiles outside the centered circular certificate. The existing synchronous station sweep still handles its own broader contract.

## Verification

- The neutral solid fixture defines a translated tilted axis, positive and negative full turns, exact area and volume, and unsupported inputs.
- The independent Three.js solid suite passed all five tests. The torus witness checks section-radius placement in an arbitrary-axis frame and compares triangulated area and volume to the neutral values.
- The isolated Rust source harness compiled the actual torus recipe with the native vector operations. Both tests passed and checked exact signed-turn geometry, analytic measures and refusal inputs.
- The linked `original-brep-solids` regression calls Circle, Face From Wire and the original Helical Sweep operator under one-item grants. It checks rollback after native topology changes, refusal without publication, source survival, exact torus geometry and volume, and funded component-labelled preview. Linked execution is pending the shared native build.
- Rust AST and scoped diff whitespace checks passed. After the forwarders and before subsequent shared work the Engine owner measured 3,991 active source lines and 236,691 active bytes. The native registry measured 3,808 lines and 206,002 bytes.
