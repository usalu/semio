# Original Convex Hull

The retained Convex Hull node builds an exact native planar solid from four to sixteen input points. Each point and each supporting-plane candidate advances through the caller's grants. The builder uses the kernel's exact-sign orientation predicates, merges coplanar support into convex polygonal faces, and discards duplicate, collinear and interior samples. The boundary must have Euler characteristic two and opposite face uses of each edge before the funded solid publisher can assign live topology and picking labels. Coplanar, collapsed or out-of-range input returns an explicit refusal.

The original Flow fixture covers a translated tetrahedron, a box with coplanar and interior samples, a scaled octahedron and coplanar rejection. It checks polygon ring sizes, opposite uses, stable preview references, world bounds, exact Volume and cancellation during the support-plane scan.

The standalone Rust harness compiled the implemented hull builder with the actual kernel orientation predicates. It verified outward volume and exact topology for the three positive cases and refused the coplanar result. The independent Manifold oracle passed all four cases and 20 assertions. Rust syntax parsing passed across nine touched modules. The full native Flow fixture remains unexecuted because the host has insufficient space for Cargo admission.
