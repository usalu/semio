# Authored sampling at a stationary concave stock cap

The Mesh Intersect and Difference routes for two longitudinal supports crossing inside the stock now also cover one or two stationary transverse caps. The clipping cursor uses the stock prism's certified cap normal, and the generated point on a stationary cap carries nonnegative weights from up to three vertices on that cap.

Authored projection checks that the weighted corner lies inside the clipped stock face's source profile, that its contributing vertices belong to that face, and that the weights reconstruct the output position. It uses those weights for raw corner UVs and the named numeric channels. Existing affine-chart checks refuse a stationary cap whose authored UVs cannot be transferred by this method. Edge samples, exact rings, face owners and cutter charts keep their existing paths.

| Neutral fixture | Intersect volume | Intersect area | Difference volume | Difference area |
| --- | ---: | ---: | ---: | ---: |
| One stationary cap | 1 | 7 | 13 | 46.47213595499958 |
| Two stationary caps | 2 | 10 | 12 | 42.47213595499958 |
| L profile with two stationary caps | 1.125 | 7.125 | | |

The stationary cap cases each produce one closed genus-zero shell. The standalone L-profile case replaced a previous refusal that is now a certified intersection. Independent Manifold checks agree on exact measurements, components and genus in both Intersect operand orders and for the two Difference fixtures.

All 10 focused Rust clipping tests and all 19 portable tests with 5,429 assertions passed. The linked authored test rebuilt the clipping, reconstruction and façade sources. It verifies both Intersect operand orders and Difference, raw and named UVs, material ownership, the authored vertex-channel values after closed reimport, and the typed refusal of a deliberately nonaffine UV chart.
