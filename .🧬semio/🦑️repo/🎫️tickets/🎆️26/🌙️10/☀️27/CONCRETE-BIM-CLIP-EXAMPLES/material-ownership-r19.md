# Wall Material Ownership R19

Layered wall material originally intersected registered concrete columns and beam bodies, and a typed jamb could overlap an adjacent wall. The wall body now subtracts other explicitly owned concrete/filling solids by horizontal polygon difference in elevation bands. This creates no fictitious architectural portal or IFC opening. Source openings retain their meaning; cut material is owned by existing structural or filling objects.

The implementation includes a reusable standard-library prism subtraction helper. It preserves wall layer/material and other part metadata and uses the same occupied parts for projections, quantities and STEP export. The wall output reports solid_volume and joint_displacement_volume, avoiding a misleading net-area-times-envelope-depth quantity. Both values derive from disjoint material partitions. Gross/envelope quantities retain their explicitly different meanings.

Content-sensitive caches retain8 cutter contexts and256 wall partitions. Inputs include actual structural, opening, host, type and material geometry; changed input content invalidates the result. Returned wall parts are isolated copies. No external runtime geometry dependency was introduced.

Two test-first contracts pass: a portable three-owner case has independent SciPy/GEOS union-integrated volume2.044m³, preserves input data/metadata and has no occupied intersection with cutters; the actual W-E source wall formerly intersected registered column solids by0.28152501098964433m³, and its new material parts have less than1e-9m³ overlap with those columns. The focused batch passed in3.15seconds on the Python WASM worker. This is a material geometry proof, not a native frame-rendering claim.

Pending integration gates: verify full current wall/filling model, preserve the ownership contract while adding sourced vertical facade zones, refresh analysis links and rerun clip callbacks against the final source. Native rendering remains unavailable through the current repository launcher.


## Precision and Regional Wall Integration

The physical material subtraction now requests twelve-decimal clipping with a 1e-12 tolerance. Connected coplanar pieces are recombined only when their reconstructed outer loops preserve area and contain no holes; larger partitions remain exact disjoint pieces. This prevents nine-decimal rounding gaps at column interfaces and avoids unbounded boundary reconstruction. Source ownership metadata remains on every fragment. The portable ownership fixture passes all three checks against GEOS: actual W-E column intersections, independent union volume, and equivalent connected wall polygons. The current 374-element building geometry completes in 5.27 seconds in the CPython WASM verifier and produces 2,215 physical wall parts. This is geometry verification, not native rendered-frame verification.
