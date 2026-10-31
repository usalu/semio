# Long Wall Luminaire

The original Domus interior photograph shows an elongated lamp above the entry-side ventilation grille, near the ceiling and beside the upper shelving. Its precise dimensions and concealed internal fittings are not measured by that photograph. The source registry therefore labels the authored housing, diffuser and tube dimensions as assumptions.

The first saved device was 0.90 m long but centred at Z2.05 m, below the grille's Z2.16–2.22 m span. A new portable regression failed on that source-relative ordering, and separately found missing evidence metadata on the tube.

The lamp is now centred over the actual grille's longitudinal span. Its inferred housing occupies Z2.22–2.29 m below the usable 2.30 m ceiling. Its backplate contacts the real media-wall face; the top, bottom, end caps, diffuser and tube have eleven explicit real-material contact links. The tube retains its actual end-cap contacts rather than floating inside a generic solid box. Every part is marked assumed/Domus.

Two tests pass in 4.295 seconds across all six physical families. NumPy checks source-relative position and ceiling clearance; independent surface-distance/segment-triangle checks verify the actual contacts; Rhino/OpenNURBS confirms material closedness. Portable cases are in the model's cases/💡️light domain. No production runtime library was added.

Source: [Domus original drawings and interior photographs](https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html).

## Independent Saved-Source Review

The audit independently read _long_wall_luminaire and the portable light cases/tests. It derives the actual handed cabinet face, centres its length on the current grille span, and places a finite housing/diffuser/tube above that grille under the usable ceiling. All seven named solids carry assumed/Domus metadata; the tube is not omitted from provenance. The eleven links check real surface/segment-triangle contact, not only AABB overlap, and Rhino closure is asserted. The reference-only call is inside capsule_assembly full detail after actual family/handed interior placement, so it follows all six physical families while study fitouts remain separate. Root's fresh two-test result is recorded above; no redundant render or final gallery acceptance was performed here.
