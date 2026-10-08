@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Take off the quantities of walls, slabs, columns, beams and spaces and audit the areas with shapely
  `s.bim.model@1` stores authored parameters only. `🧮️quantities` derives the quantity take-off from the inferred layouts
  and the types: per wall the centreline length, thickness, height, gross and net side area, the opening area clipped to the
  wall, the join-trimmed footprint, the gross and net volume, the per-layer areas and volumes and the mass; per slab the
  gross and net area (holes), the sloped surface, the vertical thickness and the layers; per column and beam the profile
  area and perimeter, the length and the volume; per space the area and the volume; and the totals per kind, per type and per
  material for every storey, every building and the project. The oracle is `🐍️.py` in this directory. It reproduces the
  table from the committed snapshot and measures with `shapely` 2: wall lengths and opening areas as GEOS lengths and
  `intersection`s with the wall development, each layer area as the GEOS `intersection` of the join-trimmed footprint of the
  sibling oracle `../🧱️infer-bim-1-wall-joins` with the strip between its layer interface curves, slab areas as GEOS polygons
  with holes, profile areas and beam lengths as GEOS polygons and lines, and the space rows from the sibling oracle
  `../🏠️infer-bim-1-spaces`. The totals are re-summed independently, and the parametric law is proved: raising a storey by
  `delta` adds `delta` times the length to the gross side area of every wall that follows its top and `delta` to every such
  column. The committed expectation is written by that file, never by hand.

  @id-quantities-building
  @level-quick
  @mode-differential
  Scenario: Walls with openings, mitered and curved walls, holed and round slabs, columns, a beam and rooms resolve to their quantities and totals
    Given the committed building model shared://💡️inferences/🧮️quantities/🏗️building/📸️snapshot/🔣️.json
    When 🧮️quantities is inferred for it
    Then every element's quantities and every total equal the table shared://💡️inferences/🧮️quantities/🏗️building/💡️inference/🧮️quantities/🔣️.json
