@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the run of every ramp from the storey levels and audit its flights and landings with shapely
  `s.bim.model@1` stores a ramp's centre-line path (vertices with bulges), width, the lengths of its flat landings at the
  foot, at the head and on every corner, the slope limit and the top constraint. `🛝️ramp-runs` derives the rise from the
  storey levels, the arc length of the path, the landings (overlapping landings merge and are clipped to the path), the
  sloped flights between them, the slope `rise / run length` and the code flags (a rise needs a sloped run; the slope must
  stay within the limit). The oracle is `🐍️.py` in this directory. It is a second, independently written implementation of
  those rules (arc length from the bulge, corners from tangent angles, merged intervals), and `shapely` 2 audits the
  geometry: the GEOS length of the path must measure the table's length, the flights and landings cut from it must tile it
  exactly, the mitred strip must cover width times length, and the heights must be continuous across every piece. It also
  proves the parametric law: raising a storey by `delta` changes the rise of a ramp that follows a storey top by exactly
  `delta` times the difference of its target being above and its own storey being above the raised one, and leaves the
  free ramps alone. The committed expectation is written by that file, never by hand.

  @id-ramp-runs-ramps
  @level-quick
  @mode-differential
  Scenario: Straight, bent, zigzag, curved, descending, flat and degenerate ramps on constrained and free rises resolve to their runs
    Given the committed ramps model shared://💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json
    When 🛝️ramp-runs is inferred for it
    Then every ramp's run equals the table shared://💡️inferences/🛝️ramp-runs/🏞️ramps/💡️inference/🛝️ramp-runs/🔣️.json
