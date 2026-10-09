@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the floor, wall and ceiling finish areas of every room from its walls, openings and the slab above and audit them with shapely
  `s.bim.model@1` stores, per space, only the materials that finish its floor, its walls and its ceiling. `🎨️finishes` derives the areas those materials
  cover: the floor is the net floor area of the room (the outline without its islands and without the columns); the walls are the perimeter of the room
  (the outline plus the boundary of every island) times its clear height, minus every valid opening of the storey whose host has a face on that
  boundary, once per face (a door between two rooms is subtracted from both rooms, a window in an outer wall from the one room behind it), clipped
  to the height between floor and ceiling; the ceiling is the part of the net floor under the hung ceiling of the room (the lowest authored ceiling of the storey over its point)
  over the cosine of that ceiling slope, plus the rest of the net floor over the cosine of the slope of the slab above that closes the room (the soffit). A room
  that is not enclosed has no finish rows. The oracle is `🐍️.py` in this directory. It reproduces the whole table from the committed snapshot: `shapely` 2
  gives the rooms through the sibling oracle `../🏠️infer-bim-1-spaces`, tests for every opening on which side of its host the room lies with
  `Polygon.boundary.distance` of the middle of each face, measures the part of the room under its hung ceiling with `Polygon.intersection` and `difference` of the columns, and restates the closed forms from the authored records alone. The committed expectation is
  written by that file, never by hand.

  @id-finishes-rooms
  @level-quick
  @mode-differential
  Scenario: Two rooms around a partition with a door, windows, a column, an island wall, a sloped slab above and two hung ceilings resolve to their finish areas
    Given the committed rooms model shared://💡️inferences/🎨️finishes/🏡️rooms/📸️snapshot/🔣️.json
    When 🎨️finishes is inferred for it
    Then every finished space's areas equal the table shared://💡️inferences/🎨️finishes/🏡️rooms/💡️inference/🎨️finishes/🔣️.json

  @id-finishes-one-room
  @level-quick
  @mode-differential
  Scenario: A closed room with two windows and a door resolves to the room-facing area of its wall solids
    Given the committed one-room model shared://💡️inferences/🎨️finishes/🏠️one-room/📸️snapshot/🔣️.json
    When 🎨️finishes is inferred for it
    Then every finished space's areas equal the table shared://💡️inferences/🎨️finishes/🏠️one-room/💡️inference/🎨️finishes/🔣️.json
