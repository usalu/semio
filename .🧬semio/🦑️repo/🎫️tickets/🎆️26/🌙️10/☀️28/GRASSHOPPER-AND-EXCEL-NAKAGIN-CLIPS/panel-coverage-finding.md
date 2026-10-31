# Source Panel Coverage Finding

Read-only independent confirmation of the auditor's physical front-panel slit. The exported actual shell-front face does not cover the point (1.049, 4.1, 2.249) m, although it is 1 mm inside the intended flat rectangle bounds X ±1.05, Z 0.25…2.25 and far outside the window aperture. The exported outer loop connects (1.05, 1.68492424) to (1.0, 2.25), clipping the upper-right rectangular corner. Corresponding gaps occur at the other corners.

This is a baseline physical coverage omission, independent of culling and AA. Each clipped plate remains individually closed/manifold, so existing per-part closedness/positive-volume and vertex-containment checks do not detect missing intended material. The radial rectangle intersections need exact rectangle-corner directions, with matching aperture-loop breakpoints to preserve a manifold annulus. The sharp-casing circular loft has the same outer-loop strategy and should receive the same targeted regression.

Root subsequently authorized the production correction. Conformal cap coverage is now saved and passes the dedicated three-test gate, the complete 135 physical suite and same-model 52 structural checks. The initial read-only finding is retained here as red evidence.
