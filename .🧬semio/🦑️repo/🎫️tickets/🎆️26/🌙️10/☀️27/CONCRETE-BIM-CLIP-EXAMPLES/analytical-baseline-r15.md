# Analytical Baseline Contracts R15

Only BimAnalysisContract and BimTributaryContract in the main clip Python test module are owned in this phase. Class replacements are applied to fresh file contents so concurrent edits to other test classes and ModelCase helpers remain intact. No captured manifest counts are used by these analytical classes.

## Independent Oracles

The structural translation tests independently intersect actual regional prisms with GEOS grid cells, compare shell occupancy per region, validate local mid-surface Z and depth, preserve source profile sections and compare analytic circle/ellipse areas with NumPy or polygon areas with GEOS. Support membership uses the shared independently authored physical_supports helper, including actual regional soffits and column birth elevations. Counts and mapping rules are checked against the actual derived objects. Coincident XY slab nodes at different Z must expose complete assumed eccentricity links.

Tributary tests use SciPy HalfspaceIntersection to reconstruct each nearest-support cell independently, then clip it to the actual occupied slab with GEOS. Exact cell geometries and disjoint coverage are checked against exported tributary pieces. Independent material/finish patches from ModelCase.load_patches are intersected with these cells to integrate individual reactions and zone forces. SciPy cKDTree and NumPy sampling provide a second area/reaction oracle. Source material ownership distinguishes solid beam strips from composite field regions. Aggregate means are not applied uniformly to individual cells.

## Regression Found and Corrected

The first independent current-source pass caught a real material propagation bug: field regions explicitly named their nominal assembly material MAT-HOLLOW-SLAB. Passing that identifier as the solid-material override caused the nominal14kN/m³ reference density to replace sourced rib/compression/hollow-unit fractions. The current source produced3.137772661kN/m² self-weight, versus the independently integrated3.3604374425kN/m². These values describe the observed source revision and are not captured test expectations.

The correction in analysis._local_breakdown identifies the authored field material from the slab's direct assembly material or its structural material layer. The source author also corrected the first slab's stale nominal MAT-RC assembly reference to MAT-HOLLOW-SLAB; solid RC remains explicitly assigned to member-owned regions. That native assembly identifier uses the composite geometry calculation. A different explicit regional material still uses its solid density and full depth, preserving RC beam-strip ownership. The baseline floor/material reaction contracts provide the failing-before regression.

## Source Declaration Findings

The first pass also found source translation declarations still claiming every member is solid RC25kN/m³ and that actual meshed floor voids are discarded. These source authoring issues were reported and corrected by the source author. Tests retain assertions that propagated physical geometry and declared losses agree. The source author regenerated analysis links after the material correction.

## Validation Status

Initial current-source analysis setup completed in16.7seconds. Geometry, profile, support, source/revision, mapping and point-reaction propagation checks passed in the first selected pass; material and declaration failures above were retained. The complete initial19-test run confirmed exact SciPy/GEOS cell geometry but exposed finite-grid aliasing at the former.025m sampling pitch: a3.626m² cell differed by.054m². The independent sampler now uses.0125m pitch while retaining the original1% area/1.2% force tolerances. Exact geometric and reaction assertions remain unchanged. The final complete run passed19/19 tests in237.125seconds after product, source material and declaration corrections. Both classes' loaded source dictionaries were compared against the frozen source after the run: each had zero differing top-level keys, and the frozen source contained340elements. Exact GEOS/SciPy geometry, area and force tolerances were retained; finer numerical sampling passed the original percentage thresholds.

## Files

- Main clip 🧪️tests/🐍️.py: only BimAnalysisContract and BimTributaryContract.
- Model 🧱️model/🧮️analysis/🐍️.py: one legitimate _local_breakdown material classification correction.
