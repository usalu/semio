# Retained Closest UV on a Rational Plane Patch

Surface Closest UV and Closest Point now admit a clamped, positive-weight, planar bilinear NURBS patch. The solver first projects onto the finite convex quadrilateral, then solves the rational bilinear parameter inverse on the original knot domains. It verifies the recovered point before returning the parameters and the native surface sample. Folded and nonplanar footprints are rejected.

The shared fixture uses a tilted, skew quadrilateral, unequal weights, and nonunit knot domains. Its three targets select an interior point, a point on the authored edge, and an authored corner. The production inverse compiled with the actual BRep vector crate and passed five compact tests, including refusal for nonplanar/folded input and zero weights. Three.js NURBS evaluation and independent triangle projection agreed on all three cases, passing nine assertions. The registered BRep analysis and retained Flow test now exercise the same values, including the u, v, point, and distance output channels. Their complete linked run remains to be verified.

## Actual linked analysis

The current source replay linked against the managed BRep dependency signatures and passed all 18 analysis tests, including rational bilinear nearest and the exact planar solid tests. The later Section-region replay passed all 19.
