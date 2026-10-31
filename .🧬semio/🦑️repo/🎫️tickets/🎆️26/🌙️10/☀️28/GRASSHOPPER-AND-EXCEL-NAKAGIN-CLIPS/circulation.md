# Physical Circulation Audit

Actual convex support faces: 1084; reachable from ground: 1084; missing capsule landings: 0/140.

A walking edge requires actual upward convex faces, maximum riser 0.170001 m, no XY gap and at least 0.6 m shared support width. Corner contacts and narrower edges are rejected. The stdlib convex support graph is checked against the independent NumPy clipping/NetworkX oracle.

## Required Targets

| Target | Required | Reached |
|---|---:|---:|
| Capsule landings | 140 | 140 |
| Lift approaches, including B1 | 26 | 26 |
| Bridge approaches | 6 | 6 |
| Bridge decks | 3 | 3 |
| Exterior threshold seeds | 2 | 2 |
| Office thresholds | 2 | 2 |
| Native L-return decks | 3 | 3 |
| Roof return landings | 2 | 2 |

## Historical Failure Record

Before phase-route reconstruction, the graph had 819 supports, only 72 reachable and none of 140 capsule landings reachable. The first-turn shared width was 0.55 m, below the 0.6 m contract, and the first A west pair had a 0.84 m gap. The later graph reached all targets but mesh headroom exposed two low roof arms and then one B water-tank obstruction; these were corrected in geometry, not suppressed by the tests.

## First Turning Contacts

~~~json
[]
~~~

## Capsule Landing Contacts

~~~json
[]
~~~

## Authoritative Semantic Fingerprints

~~~json
{
  "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "907f513a3801c4d5e8695b6779a51a89d77c58cf141a81db5134b39c0f433091",
  "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "270e985286ab0bad4ba28891a9353c65f72ffb96ffa19c20df42d3ccee324712",
  "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "e50dc411966e2e2e962352c4c51140e4f7600bdf0b44773ebfe7af70e71c9a75",
  "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "e2b8fa628969c0f7498361139b34085c871333e99c07b2aa9ee7167301bf2ff7"
}
~~~

## Body Footprint Coverage

Checked oriented paths: 618; unsupported or oracle-disagreeing paths: 0.

## Walking Clearance

All-part mesh clearance conflicts: 0. Actual stair-tread centre paths checked: 558.
~~~json
[]
~~~

## Coverage Limits

The graph starts at the two explicit exterior ground thresholds. Passage width, normal-riser limits and clear heights are authored assumptions, not surveyed proof of the historical building. Flat supports use continuous 0.6 m body strips, including oriented polygonal turns; complete footprints are independently proven covered by the union of actual supports. Every actual stair tread is checked along its complete run with a continuous 2 mm centreline tube at 2.1 m headroom. AABBs provide only a broad phase. Actual closed-mesh faces are ear-clipped and checked by triangle/prism SAT plus ray containment; independent NumPy clipping/solid-angle fixtures validate the narrow phase. Every physical part and conservative nominal capsule envelope is considered. Door landings use the authored 1.95 m height, interior connectors 2.1 m; a 0.0001 m surface tolerance handles rounded authored elevations, and only geometrically adjacent normal-riser transitions are exempted. The separate full doorway-volume and structural-bearing regressions complement this route proof. This does not establish clearance along every possible path over a support, between the tested centre strips, through operable lift doors, or for a surveyed/code-compliant reconstruction.
