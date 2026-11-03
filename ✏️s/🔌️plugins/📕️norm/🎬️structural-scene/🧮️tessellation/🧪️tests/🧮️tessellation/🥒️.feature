@capability-structural-scene-tessellation
@oracle-python-shapely-numpy-trimesh
@comparison-closed-form-v1
Feature: Tessellate structural geometry into watertight, analytically exact meshes
  Section outlines and extrusions are pure functions of geometry, tolerance and level; area, centroid,
  second moments, volume, watertightness and bounds must agree with the independent Python geometry oracle.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Section <profile> matches the oracle: Given a parametric section "<profile>" with parameters "<params>" When its outline is tessellated at level "lod2" and extruded 3 m along X Then the oracle reports area, centroid and second moments within 1e-8 of the rectangle decomposition And the oracle reports the mesh volume within 2e-5 of area times length And the oracle reports the mesh as watertight with bounds equal to its vertices Examples (8 rows: ishape-h-0-24-b-0-24-tw-0-01-tf-0-017, rect-h-0-5-b-0-3, rhs-h-0-2-b-0-1-t-0-01, channel-h-0-2-b-0-075-tw-0-0085-tf-0-0115, tee-h-0-15-b-0-15-tw-0-01-tf-0-012, angle-h-0-1-b-0-1-t-0-01, cee-h-0-2-b-0-075-t-0-008-c-0-02, zed-h-0-2-b-0-075-t-0-008-c-0-02)
  - A circular hollow section follows the chord tolerance: Given a parametric section "chs" with parameters "d=0.2,t=0.01" When its outline is tessellated at level "lod3" with chord tolerance 1e-6 Then the oracle reports the polygon area equal to the regular n-gon formula And the oracle reports the polygon area within the n-gon bound of the circle area
  - Root fillets add the closed-form area: Given a parametric section "ishape" with parameters "h=0.24,b=0.24,tw=0.01,tf=0.017,r=0.021" When its outline is tessellated at level "lod3" Then the oracle reports the area within 0.2% of the sharp area plus four times (1 - pi/4) r squared
  - Levels are bounded and deterministic: Given a member with an I-shape section When it is tessellated twice at each level Then both results are identical And level "lod0" is a 12-triangle box impostor with the volume of the bounding box And the triangle count does not decrease from "lod1" to "lod3"
  - Solids, shells and fasteners have analytical volumes: Given prisms, boxes, wedges, strata, footings, layered walls with openings, cylindrical shells and rebar When each is tessellated Then each volume equals the closed form for its faceted construction within 1e-6
