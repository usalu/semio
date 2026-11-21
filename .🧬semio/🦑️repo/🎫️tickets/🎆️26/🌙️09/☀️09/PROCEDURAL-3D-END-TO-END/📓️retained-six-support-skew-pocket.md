# Retained six-support skew pocket

Retained BRep Cut now runs the authored planar arrangement for all six active cutter supports. A doubly tilted cubical tool opens at the top stock face. The native result has one closed shell with 22 vertices, 35 edges, and 15 faces. All six authored tool planes survive as inward walls with 11 reciprocal wall joins. Its measured area is 100.04968239525003 and its measured volume is 63.093324992027725.

The new shared fixture reproduced UnsupportedOwner in the compact native source suite before the change. The complete suite passed 45 native tests afterward. The registered Manifold and Three oracle passed 20 tests with 253 assertions. The exact Cut fixture checks shell incidence, authored supports, measures, classification, trim agreement and the original Flow publication path. Native planar publication now also returns a typed geometry refusal if a supplied ring exceeds its fixed facet capacity.

The reflected stock case producing three disconnected remnants remains a separate compound publication issue. Independent Manifold measures its three material volumes as 11.453815966043472, 4.254279290254225, and 0.036275250915147286.
