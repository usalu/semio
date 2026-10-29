# Villa Savoye Axon Reconstruction R4

The axonometric renderer now subtracts the union of every door, window and unfilled opening from closed wall solids. The portable co-located cases cover multiple cuts, solid partitions and overlapping cuts; their areas are checked independently with NumPy polygon areas. Explicit ground walls replace synthesized room-outline enclosures. All authored slabs, columns at every level, helical stair treads, guards, details and furniture are projected from shared model geometry, preserving element identity, discipline selection and exploded offsets. Isolated wall animation supports several cuts and nullable fillings.

The source uses plan coordinates, with approximate true north (-1,-1); the drawing documentation no longer calls positive y north. Dimensions remain source-informed teaching assumptions. The landscape frame is an authored standard wall with an unfilled cut. Curved screen paths additionally support cumulative-distance cuts.

Validation pending: repo --help currently exits with malformed runtime registry restart field, preventing repository-approved test and render execution. The portable test module is axon/🧪️tests.py.
