# Original two-profile Loft

The retained Loft node accepts exactly two live planar faces. The circular route requires equal radii, matching seam frames and a coaxial displacement, then publishes the exact cylinder. The polygon route quotes and checks corresponding directed line trims one pair per turn, then validates the first section and publishes its exact translated prism. The translation must be transverse and match both endpoints of every pair at kernel tolerance. Two sections remain degree one with Smooth enabled, matching the cold Loft math. Additional sections, holes, mismatched trims, change in radius and twist remain outside this retained route.

The original Flow fixture constructs a reversed world-Y circular loft with Smooth enabled and an oblique translated triangular loft. It verifies the `solid` output, opposite coedge uses, exact volume and funded preview bounds. It also exercises a mismatched polygon, cancellation and sparse numeric profile keys.

An isolated Rust run compiled the actual matching function and checked three translated boundary pairs and three mismatches. The independent Manifold runs reproduced both solids and passed all 17 assertions. Rust syntax parsing passed across nine touched modules. The full native Flow fixture remains pending repository Cargo admission.
