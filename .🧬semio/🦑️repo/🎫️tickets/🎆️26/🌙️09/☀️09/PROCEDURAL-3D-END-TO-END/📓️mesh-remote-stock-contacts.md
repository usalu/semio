# Remote stock contacts on Mesh cutter supports

The joined longitudinal Mesh cursor now handles closed interior cutter profiles whose supporting planes pass through a remote reflex vertex or coincide with a remote stock edge. It admits either contact only when another cutter halfspace strictly excludes both ends from the active cut. An isolated reflex tangent leaves the support's crossing order intact. For a coincident stock edge, the cursor walks the ordered stock boundary events and checks stock membership for each interval. A collinear or uncertain interval can proceed only when a separate cutter support excludes it. This preserves the original stock wall ownership while keeping the active cutter walls, cap openings and all cyclic vertex links certified.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area | Difference genus |
| --- | ---: | ---: | ---: | ---: | ---: |
| Hexagonal pocket with a remote reflex tangent | 0.875 | 6.207106781186548 | 13.125 | 49.179242736186126 | 0 |
| Triangular through-opening with a remote coincident stock edge | 0.25 | 2.960404813240945 | 13.75 | 48.43254076824052 | 1 |

The initial native witness reproduced the reflex-contact refusal. Nine focused native unit tests and the dedicated longitudinal integration test passed. The linked authored Mesh test rebuilt the actual clipping, document/tessellation, reconstruction and façade crates and passed. It verifies exact stock and cutter ownership, named and raw UVs, and affine vertex attributes after closed reimport. Manifold independently matched area, volume, components and genus for Difference and both Intersect orders. The portable suite passed 19 tests with 5,637 assertions.

The existing public geometry corpus now runs as the clipping package's registered longitudinal integration target. The packaging test recognizes a package's implicit Cargo dependency in its own integration targets while continuing to check production dependencies. The clipping library source closure is 3,693 active lines under the existing 4,000-line limit.

Contacts on the active cut perimeter still require their own certificate.
