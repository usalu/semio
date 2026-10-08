@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the run of every stair from the storey levels and audit its flights and landings with shapely
  `s.bim.model@1` stores a stair's start, direction, width, flight kind (straight, L-turn, U-turn, spiral), top constraint
  and the comfort limits `max_riser` and `min_tread`. `🪜️stair-runs` derives the rise from the storey levels, the equal
  riser height from the riser limit (the fewest risers within it), the tread from Blondel's rule `2R + T` and the minimum
  tread, the flights with their riser ranges and walking lines, the landings, the winder of a spiral, the run length and the
  code flags. The oracle is `🐍️.py` in this directory. It is a second, independently written implementation of those rules
  on exactly rounded sums, and `shapely` 2 audits the geometry: every flight is a flat-capped strip around its walking
  line, every landing a rotated box; the walking lines must measure the run length, the strips must be disjoint, a landing
  must touch the two flights it joins, and a winder must cover the annulus sector it claims. It also proves the parametric
  law: raising a storey by `delta` raises the rise of exactly the stairs that follow its top by `delta` and leaves the free
  ones alone. The committed expectation is written by that file, never by hand.

  @id-stair-runs-flights
  @level-quick
  @mode-differential
  Scenario: Straight, quarter-turn, half-turn and spiral stairs on constrained, free and degenerate rises resolve to their runs
    Given the committed flights model shared://💡️inferences/🪜️stair-runs/🪜️flights/📸️snapshot/🔣️.json
    When 🪜️stair-runs is inferred for it
    Then every stair's run equals the table shared://💡️inferences/🪜️stair-runs/🪜️flights/💡️inference/🪜️stair-runs/🔣️.json
