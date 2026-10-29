# Clipping Precision Correction R79

The exact native canopy reproduction in slab-invalid-geometry-r79.md showed adjacent source triangles sharing only an edge. Rounding each intermediate half-plane result against the original unrounded boundaries created a self-crossing quadrilateral; this was not real positive material.

Only analysis._clip changed. Half-plane operations retain input precision throughout; requested coordinate rounding and exact ring cleanup happen once at publication. The digits=None path retains its prior behavior. No area threshold, invalid-ring skip, geometry-specific exception or production dependency was added.

The checked-in language-neutral partitions.json case covers both operand orders, boundary winding/cyclic order and digits9/12/None. Eight recorded RED subtest failures become a GREEN method. A positive2e-18m² facet remains valid and matches independent GEOS within1e-30m². These permutations do not claim every independent subject-winding variation.

Actual all-slab validity scan:10slabs,all six occupied/full/shared structural/finish views,zero invalid or nonpositive polygons in5.138s. Canopy/weather material now has51valid ear parts instead of57including false rounded fragments. Actual four-method SlabPartitionContract passes in3.926s with original conservation and pair-intersection tolerances. Evidence: generated/clip-topology-red-r79.json,clip-topology-green-r79.json,slab-validity-r79.json,slab-full-r79.json. Inputs retained in slab-validity-r79.py,slab-full-r79.py and checked-in partitions data/tests.

Producer before SHA256 d8f3c7f5620ce384d15486c076bbab2795c66af43f84ef02e886a0599a52ea6d;after be55d6f11f43d2372edf86c04293d2e03e6bdc11a6b73326fdbb20a6c2ddb91e. Physical source during these proofs1bad321155b30fcaed4d85f6ff15e0ea5102d93e71c0b367693d19cccf72bb34. Later source/schema/export changes require their own scope qualification. Independent review is final-geometry-fix-audit-r80.md.
