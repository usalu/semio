@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the placement of every component and the run of every MEP element and audit them with numpy and shapely
  `s.bim.model@1` stores a component as an instance of a parametric family: its storey, family, plan position, height above the storey, rotation, mirror flag, optional host wall and optional terminal system,
  with its parameter overrides as separate rows. A routed MEP element stores its system, its section (duct, pipe or tray) and a polyline whose `z` is the height above its storey. `🪑️components` and `🌀️mep`
  derive everything else: the family under the overrides, the position and turn in the building frame (a hosted component clings to the face of its wall on the side where its position lies and faces away from
  the wall), the footprint, the bounds, the volume and the connector; the path, section, length, closed-form volume and surface of every run; the clashing pairs of different systems; the quantities per element and
  per category, system and size; and the findings (missing family, profile family, missing host, faulty overrides, components outside their storey or in a wall, degenerate runs, clashes, unconnected terminals).
  The oracle is `🐍️.py` in this directory. It is a second, independently written implementation: the family under the overrides comes from the independent interpreter of the families oracle, the placement from
  numpy (projection on the axis, the sign of the cross product, mirror then rotation), the footprints, wall overlaps and distances beyond the walls from shapely 2 (GEOS), the segment distances from the minimum over
  the interior solution and the boundary edges. It also proves the closed forms against GEOS (the polygon area of a footprint, the flat buffer of a straight duct) and the metamorphic laws that mirroring a hosted component keeps its
  footprint area, that moving it along its wall moves it by exactly that distance and that raising a storey does not move a component on the ground storey. The committed expectation is written by that file, never by hand.

  @id-components-room
  @level-quick
  @mode-differential
  Scenario: A furnished room with hosted fixtures, overrides, terminals and crossing runs resolves to its components, runs, quantities and findings
    Given the committed room model shared://💡️inferences/🪑️components/🏠️room/📸️snapshot/🔣️.json
    When 🪑️components and 🌀️mep are inferred for it
    Then every component, run, quantity and finding equals the table shared://💡️inferences/🪑️components/🏠️room/💡️inference/🪑️components/🔣️.json
