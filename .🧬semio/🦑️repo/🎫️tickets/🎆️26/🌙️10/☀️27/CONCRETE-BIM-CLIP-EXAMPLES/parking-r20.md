# Physical Parking Validation R20

## Result

No verified three-car fit was found for the unchanged assumed4.6×1.75×1.65m vehicles under the retained75mm boundary/column-surface and25mm inter-car margins. The result is bounded-search inconclusive, not a proof of global infeasibility and not a contradiction of the primary historical three-car capacity. No building, column, garage or vehicle geometry was changed for parking.

The earlier three poses were invalid after the actual structural register replaced the old grid: they collide with columns7,12 and21. They now form an explicit negative regression. The best bounded candidate is also explicitly rejected: boundary distance0.057817m versus0.075m; column21 surface distance0.069587m versus0.075m; and pair clearance approximately0m versus0.025m. No invalid three-car pose is represented as a passing demonstration.

A separate two-car positive control is verified at centres(13.79,9.5) and(12,14), both90degrees, with the same full-size vehicles. Minimum boundary distance is0.225m, column-surface distance0.117m, physical wall/filling distance0.150m and car-pair gap0.040m. This establishes two valid static poses, not a maximum garage capacity.

## Physical Contract

The portable fixture is model/🧪️tests/🚗️parking.json version2. It distinguishes target.status=unverified-after-bounded-search, documented historical_capacity=3, verified_control with two poses, and two rejected_demonstrations. The retired three-bay portal reserves were removed after the real unified sliding portal was authored. Actual hall0.8m and chauffeur0.6m pedestrian reserves remain, bound to native clear portal endpoints. The unified closed panel trains and curved glazing remain physical obstacles.

All35 column sections intersecting the full vehicle height are independently reconstructed, including cellar-labelled14 and transfer supports whose physical elevations intersect the vehicle. The language-neutral oracle contains1049 relevant-height physical parts, including actual garage panels and glass. A source-input fingerprint binds the vehicle scenario, relevant physical source objects, types/materials and datums. New hoist/post/shaft products require fresh oracle review and geometric validation; do not merely refresh the hash. The two-car control overlaps the future hoist area and is not claimed to remain valid after that implementation.

## Search and Validation

Retained inputs and replay code: parking-r20-input.json and parking-r20-solver.py. Numerical search used approximately94.5seconds across bounded Nelder-Mead, differential evolution and signed-separating-axis SLSQP refinements. Each chunk has an explicit time cap and checks a cancel_requested input flag. The poor initial global GEOS union preprocessing was cancelled and replaced by indexed individual constraints. Search observations are outputs under generated/parking-r20-*.json, not source inputs.

Independent JavaScript polygon clipping/manual distances and Python Shapely/NumPy transforms agree on positive and negative verdicts and numeric clearances. All five Python tests pass, including source binding, native physical-oracle conformance, two-car control, rejection of both three-car demonstrations and cross-implementation metric parity. The JS validateParking export also passes. Its result now exposes target, historical_capacity, verified_fleet_size, verified and rejected; no legacy top-level three-car success is retained.

This contract provides static body-envelope proof only. It does not establish ingress/egress trajectories, open vehicle doors, operational lifting-jib clearance or a surveyed historical vehicle model. The local16–30mm signed-margin deficits are of the same order as the inferred plan registration uncertainty; no historical conclusion follows from them.
