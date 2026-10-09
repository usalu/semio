@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the room of every space from the walls around its seed and audit the arrangement with shapely
  `s.bim.model@1` stores a space's storey, number, name, usage and a boundary that is either an explicit loop or a
  `Bounded` seed point. `🏠️spaces` derives the room: the bounded face of the storey's wall arrangement (the union of the
  join-trimmed wall footprints, so a partition that butts into an outer wall closes two rooms, a free-standing wall
  becomes an island) that contains the seed, its area, perimeter, island count, net floor area without the columns, the
  clear height under the thickest slab of the storey above that covers it and under the lowest ceiling of its own storey that hangs over it (a hung ceiling
  only ever lowers the clear height; its holes let the room reach the slab), the volume and the walls that bound it. A seed
  in a wall or in a face open to the outside is reported as such. The oracle is `🐍️.py` in this directory. It reproduces
  the whole table from the committed snapshot: `shapely` 2 unites the footprints of the sibling oracle
  `../🧱️infer-bim-1-wall-joins`, `polygonize`s the union boundary into faces, keeps the faces that are not wall material,
  measures areas, perimeters and shared edges, cuts the columns out with `difference` and tests slab coverage with
  `contains`; explicit outlines take closed-form circular segments that GEOS audits on a sampled polygon. It also audits
  that every room is a valid polygon disjoint from the walls and that moving the partition by `delta` moves exactly
  `delta` times the clear width of area between the two rooms, and that lowering a ceiling that governs a room by `delta` lowers exactly that room's clear
  height by `delta`. The committed expectation is written by that file, never by hand.

  @id-spaces-rooms
  @level-quick
  @mode-differential
  Scenario: Rooms around a partition, an island wall, columns and slabs resolve to their areas, clear heights and bounding walls
    Given the committed rooms model shared://💡️inferences/🛋️spaces/🏡️rooms/📸️snapshot/🔣️.json
    When 🏠️spaces is inferred for it
    Then every space's room equals the table shared://💡️inferences/🛋️spaces/🏡️rooms/💡️inference/🏠️spaces/🔣️.json

  @id-spaces-hung-edit
  @level-quick
  @mode-differential
  Scenario: Editing the ceilings of the rooms model lowers, replaces and removes the ceiling that governs the clear height of the rooms below
    Given the committed edited rooms model shared://💡️inferences/🛋️spaces/🔲️hung-edit/📸️snapshot/🔣️.json
    When 🏠️spaces is inferred for it
    Then every space's room equals the table shared://💡️inferences/🛋️spaces/🔲️hung-edit/💡️inference/🏠️spaces/🔣️.json
