# Independent Garage Parking Validation

The final source-informed garage polygon is [[12,6],[16,3.5],[16,15.5],[11.7,18.1],[10.9,18.1],[10.9,10],[12,10]]. Its coordinates are an authored teaching approximation of the CMN visitor diagram, not a historical survey. The model car remains 4.6 by 1.75 metres. The ground column diameter remains 0.30 m. No vehicle or column was reduced.

## Portable Contract and Independent Oracle

The permanent fixture is model/🧪️tests/🚗️parking.json and the independent test is model/🧪️tests/🚗️parking.mjs, exported validateParking(). The test reads the live JSON model, checks the exact room outline, source car dimensions, column positions and radii, and all five portal endpoints. It computes oriented car rectangles using trigonometry and uses the third-party polygon-clipping test oracle to independently check containment, footprint area, non-overlap and reserved local door access. A separate separating-axis calculation agrees with the clipping oracle for every car pair. Polygon-clipping is an existing test dependency; no product runtime dependency was introduced.

The three static parked poses, measured counterclockwise from positive x, are centre (14.069,7.280), angle114.677°; centre (12.647,11.748), angle111.039°; centre (13.525,14.477), angle119.006°. Their minimum pair gap is0.029763m. Minimum distance to a boundary wall axis is0.104623m; subtracting the0.075m wall half-thickness leaves0.029623m minimum physical wall-face clearance. Minimum column surface clearance is0.105116m. These are tight physical parked-car clearances, not a parking comfort standard.

All three cars avoid local rectangular access reserves extending0.8m into the garage at the hall portal,0.6m at the diagonal chauffeur portal and0.3m at each garage portal. These reserves establish unobstructed immediate threshold zones only. They do not establish a continuous pedestrian route, a door swing envelope or a vehicle maneuver.

The source car wall_clearance0.25 and min_side_margin0.5 remain arrival/drive-through assumptions, not imposed as parking comfort thresholds by this static fixture. A sampled search using those values found no three-car configuration; that is a bounded search failure, not an impossibility proof. The earlier horizontal chauffeur boundary and x15.15 enclosure also failed the sampled three-car search. This motivated checking the actual diagram topology rather than shrinking vehicles.

## Validation Result and Limits

The Node API executed validateParking() successfully against the live revised166-element source. All three cars have clipped area8.05m²; all pair overlaps and local access-reserve overlaps are zero. The test asserts at least0.025m car separation and at least0.075m column-surface clearance. Permanent scripts and fixtures are retained; no output files were generated.

No drivable-ingress proof has been produced. A valid static fit cannot certify that a4.6m car with the assumed5.5m turning radius can enter or leave each door in the presence of the other parked cars. That requires a separately specified steering/axle model and swept-path validation; the current turning-radius scalar alone does not define the vehicle kinematics.
