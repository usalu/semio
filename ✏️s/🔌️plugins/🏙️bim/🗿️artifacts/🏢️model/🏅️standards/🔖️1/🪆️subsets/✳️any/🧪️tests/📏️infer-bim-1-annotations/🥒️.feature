@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer what every dimension, tag, note and leader of a storey shows and audit the geometry with shapely
  `s.bim.model@1` stores the anchors of a dimension (a wall face, a wall axis, a wall end, an opening centre, a grid line, a column centre or a free point), its
  measuring angle, its offset, its style and an optional lock, the element and the category of a tag, a text note's position and text, and a leader's anchor,
  offset and text. It stores no printed number. `🪧️annotation-layout` derives, per annotated storey, the distance every dimension prints (the difference of the
  positions of its anchors along its measuring direction, each segment of a chain and the total), the text every tag reads from its element (name, type, number
  or size, in the unit and precision of its style) and the findings: anchors that have no geometry, anchors that never cross the measuring line, zero segments,
  locks that are not kept, styles that are missing and tags that print nothing. A lock is checked and never moves anything. The oracle is `🐍️.py` in this
  directory. It reproduces the whole table from the committed snapshot: `shapely` 2 offsets every wall axis into its two faces (`offset_curve`), places the
  centre of an opening (`LineString.interpolate`) and must measure the clear distance of two parallel faces (`distance`) exactly as the dimension prints it;
  it also audits the metamorphic laws that lengthening a wall by a distance lengthens its end-to-end dimension by that distance and that moving a column does
  not change what its tag prints. The committed expectation is written by that file, never by hand.

  @id-annotations-room
  @level-quick
  @mode-differential
  Scenario: Dimensions, tags, a note and a leader around a walled room resolve to their printed distances, texts and findings
    Given the committed room model shared://💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json
    When 🪧️annotation-layout is inferred for it
    Then every dimension's distances, every tag's text and the findings equal the table shared://💡️inferences/🪧️annotation-layout/🏠️room/💡️inference/📏️annotations/🔣️.json
