# Exact Intersection of Rotated Shared-Axis Boxes

The retained Rotate and Translate jobs preserve the authored corner, orthogonal unit directions, and lengths of a rectangular solid. The existing axis-aligned primitive route remains available for cardinal placements. For other orthogonal placements, the same funded planar solid publisher produces the rotated box and retains its material certificate for Intersect.

If either box fully contains the other, Intersect reissues the exact contained certificate. Otherwise, the new route accepts a shared principal axis, including antiparallel or non-world axes. It intersects the signed cap intervals and clips the rectangular footprint by the other box's four side planes. Each output face lies on an authored support plane. The footprint contains at most eight corners and the existing prism publisher emits planar faces and native edge identities. Separation and cap or edge contact give the existing empty-solid result. Positive geometry that cannot be resolved at kernel tolerance is refused. Partial overlap between skew boxes with no shared principal axis is outside this certificate.

The language-neutral fixture covers an eight-sided overlap, containment, a cap-only contact, a side tangency, and separation. Manifold-3D independently matched volume, area, bounds, and genus and confirmed zero contact volume. Its two tests passed with 21 assertions.

The disk-light Rust build passed all 22 tests, including intersection after the authored Rotate and Translate steps, exact support normals and same-parameter pcurves, reciprocal edge uses, mass measures, reversed operands, a common oblique world axis, antiparallel caps, and refusal for skew partial overlap. The registered Flow fixture exercises construction and measured output through the actual operator IDs, one-item publication, cancellation and retirement, labels, and empty results. The linked suite was left to the managed Cargo lane.

Nightly Rust syntax, scoped whitespace, and path lengths passed. Source closures contain 3,999 active Engine lines, 3,998 Native Registry, and 3,921 Solid Transaction.
