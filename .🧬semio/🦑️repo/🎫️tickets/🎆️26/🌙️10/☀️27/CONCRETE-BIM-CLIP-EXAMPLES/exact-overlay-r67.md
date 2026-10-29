# Exact Polygon-Set Area Reference R67

Root approved the principled area-only kernel after both GEOS and polygon-clipping exposed concrete floating-topology failures on unchanged coordinates. The final Python implementation uses only standard-library Fraction.from_float, Counter, json and math. No subprocess, injected backend, Pyodide/js import, geometry repair, snapping, tolerance adjustment or topology fallback remains. The existing Node polygon-clipping implementation remains an independent cross-check only on cases where it works.

## Exact Construction

Coordinates are interpreted as their exact binary64 rationals. Every vertex x and every exact segment-intersection x partitions the domain. Original floating endpoint AABBs only prune provably separated segment pairs; all retained predicates/intersections use Fraction arithmetic. Parallel/collinear overlaps change only at already included endpoints. Between successive critical x-values, each polygon's ring intersections have fixed order. Even-odd pairing handles holes independent of ring orientation. Intervals are unioned per side, then exact symmetric-difference length is integrated at the stripe midpoint; its linearity makes midpoint integration exact. Each positive exact stripe area converts to float only after multiplication, and math.fsum forms the final nonnegative result. A multiset shortcut applies only to literally identical binary64 geometry, using float.hex coordinate identities. It never accepts near equality.

## TDD and Independent Checks

Portable JSON cases cover same-side overlap, missing/extra material, displacement, cross-cell/reversed holes, tiny positive area, empty sets, proper interior crossings, concavity, endpoint contact, the captured column2/4 counterexamples and the104-polygon CW frame group. Small cases compare with GEOS; ordinary cases and the104 union compare with the existing independent Node library. Low's independent review passed12 cases, the annulus (.07762043377033379 vsGEOS .07762043377033376), and104union (.45947224758806315 vsNode .4594722475880584), with identityXOR0. See fraction-overlay-audit-r67.md.

The real actual339-case batch completes14.4820101s, all339 below the unchanged1e-8 limit. Maximum XOR area8.870865977772452e-13. The portable contract passes7.1741554s. Request/response AJV2020 schemas pass for all339 actual cases. The per-case progress and complete results remain generated/fraction-overlay-progress-r67.json and fraction-overlay-result-r67.json. The Node independent batch is fraction-overlay-node-independent-r67.json. No full feature-tail pass is claimed by these kernel-only results.

## File and Identity Scope

Final test interface: model/tests/🚗️overlay.py, 🚗️overlay.mjs, 🚗️overlay.schema.json and 🚗️overlays.json; parking.py retains its exact NumPy/SciPy/GEOS height projection and unchanged per-group1e-8 assertion. Only its set-area call changes. Ticket/python-validation.mjs removes all failed host/RPC injection. The approved two-line P restoration separately returns the exact9d506aab…4c18 builtin-tuple source, preserving all other lines. Source JSON remainsc53.

Ten original passed canonical IDs and their AST/helper evidence are retained in feature-prefix-r62.json. The first resumed tail's48 completed IDs contain46 passes and two preserved errors; its frozen-input abort evidence is retained uniquely. Any further resume must explicitly retain only still-valid passed IDs and rerun changed-kernel/P affected tests. No ordinal skip or duplicate full structural replay is authorized.

The restored actual P return contract was freshly imported and passed separately in0.033s with builtin tuple result. The benchmark worker was then disposed. Root opened a final metadata/STEP-span patch window; no full tail is running during those edits. The forthcoming final tail will reuse only the ten explicitly proven heavy prefix IDs, rerun the46 prior cheap passes and affected kernel/P tests, and record the bibliographic-only source transition separately from the originalc53 numeric proof.
