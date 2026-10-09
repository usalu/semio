@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer what every zone and every area scheme adds up over the rooms of its spaces and audit it with shapely
  `s.bim.model@1` stores a zone (a name, a category and an occupancy density in persons per square metre), the zone each space belongs to and the area
  schemes (a measure, the usages and the zones they count). `🏘️zones` derives the totals: a zone adds up the gross area, the net floor area, the volume,
  the occupancy (density times net floor area) and the three finish areas of its resolved members; an area scheme counts a space when its usage list is empty
  or names the usage of the space and its zone list is empty or names the zone of the space, and adds up the gross area or the net floor area of the
  counted rooms with their volume and occupancy. A space whose room is not enclosed is counted but adds no area. The oracle is `🐍️.py` in this directory.
  It reproduces the whole table from the committed snapshot: `shapely` 2 gives the rooms and the finish areas through the sibling oracles
  `../🏠️infer-bim-1-spaces` and `../🎨️infer-bim-1-finishes`, the sums are exactly rounded `math.fsum`, and it audits that a scheme which counts exactly the
  spaces of a zone adds up what the zone adds up. The committed expectation is written by that file, never by hand.

  @id-zones-zoning
  @level-quick
  @mode-differential
  Scenario: Two zones and four schemes over a house with an open space resolve to their areas, volumes and occupancy
    Given the committed zoning model shared://💡️inferences/🏘️zones/🏡️zoning/📸️snapshot/🔣️.json
    When 🏘️zones is inferred for it
    Then every zone's and scheme's totals equal the table shared://💡️inferences/🏘️zones/🏡️zoning/💡️inference/🏘️zones/🔣️.json
